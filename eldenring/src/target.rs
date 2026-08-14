pub use crate::phase_transition::target_next_phase as next_phase;
use {
    crate::{
        chr_ins::{ChrIns, ResolvedChrPtr},
        mem::*,
        offsets::{code_cave::CaveAddress, module_offsets::Hook},
        phase_transition,
        resources::ASM,
    },
    gubtool_core::{
        address::Address,
        attached::version,
        game_version::EldenRingVersion::*,
        slice_ops::*,
        sys::sys_error::{PointerType, ProcResult, ProcessError},
    },
    shared::{
        act_array::ActArray,
        command::{ToggleCommand, UnitCommand, ValueCommand},
        declare_command,
    },
    std::sync::{LazyLock, Mutex, MutexGuard},
};

static TARGET: LazyLock<Mutex<Target>> = LazyLock::new(|| Mutex::new(Target::new()));

pub fn target() -> MutexGuard<'static, Target> {
    TARGET.lock().unwrap()
}

#[derive(Debug)]
pub struct Target {
    chr_ins: Option<ChrIns>,
}

impl Target {
    fn new() -> Self {
        let mut target = Self {
            chr_ins: None,
        };
        target.update();
        target
    }

    pub fn update(&mut self) {
        match read::<u64>(CaveAddress::SavedTargetPointer) {
            Ok(new_target) => {
                let same_target = self
                    .chr_ins
                    .as_ref()
                    .and_then(|chr| chr.resolved_pointers.get(&ResolvedChrPtr::ChrIns))
                    .is_some_and(|ptr| *ptr == new_target);

                if !same_target {
                    if new_target != 0 {
                        self.chr_ins = Some(ChrIns::new(new_target))
                    } else {
                        self.chr_ins = None;
                    }
                }
            }
            Err(_) => {
                self.chr_ins = None;
            }
        }
    }

    pub fn chr_ins(&mut self) -> ProcResult<&mut ChrIns> {
        self.chr_ins
            .as_mut()
            .ok_or(ProcessError::null_pointer(PointerType::Target))
    }

    pub fn set(&mut self, chr_ins: ChrIns) {
        self.chr_ins = Some(chr_ins)
    }

    pub fn pointers(&self) -> Vec<(String, u64)> {
        self.chr_ins
            .as_ref()
            .map(|c| c.pointers())
            .unwrap_or_default()
    }
}

declare_command!(
    SaveTargetHook,
    Health,
    HealthPercentage => "Health %",
    Kill,
    NextPhase,
    RepeatAction,
    ForceActSequence,
    ResetPosition,
    NoDamage,
    NoStagger,
    DisableAi,
    RepeatLastAction,
);

const TARGET_HOOK_BYTES_ORIGINAL: [u8; 7] = [0x48, 0x8b, 0x8f, 0x88, 0x00, 0x00, 0x00];
impl ToggleCommand for SaveTargetHook {
    fn is(&self) -> ProcResult<bool> {
        read::<[u8; 7]>(Hook::LockedTargetPointer).map(|val| val != TARGET_HOOK_BYTES_ORIGINAL)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        if state {
            let mut fun = ASM.get_function("save_target_hook");
            let mut asm = fun.take_bytes();

            write_addr_to_slice(
                &mut asm,
                fun.reloc("saved_pointer_loc"),
                CaveAddress::SavedTargetPointer,
            )?;
            write_rel_i32(
                &mut asm,
                CaveAddress::SaveTargetHook,
                fun.reloc("hook_loc"),
                Hook::LockedTargetPointer.add_offset(7),
                4,
            )?;
            install_hook(&asm, CaveAddress::SaveTargetHook, Hook::LockedTargetPointer, 7)?;
        } else {
            write_bytes(Hook::LockedTargetPointer, &TARGET_HOOK_BYTES_ORIGINAL)?;
        }
        Ok(())
    }
}

impl ValueCommand<i32> for Health {
    fn get(&self) -> ProcResult<i32> {
        target().chr_ins()?.get_current_hp()
    }
    fn set(&self, val: i32) -> anyhow::Result<()> {
        target().chr_ins()?.set_hp(val)?;
        Ok(())
    }
}

impl ValueCommand<f32> for HealthPercentage {
    fn get(&self) -> ProcResult<f32> {
        target().chr_ins()?.get_hp_pct()
    }
    fn set(&self, val: f32) -> anyhow::Result<()> {
        target().chr_ins()?.set_hp_pct(val)
    }
}

impl UnitCommand for Kill {
    fn execute(&self) -> anyhow::Result<()> {
        target().chr_ins()?.set_hp(0)?;
        Ok(())
    }
}

impl UnitCommand for NextPhase {
    fn execute(&self) -> anyhow::Result<()> {
        phase_transition::target_next_phase()
    }
}

impl ValueCommand<u8> for RepeatAction {
    fn set(&self, val: u8) -> anyhow::Result<()> {
        target().chr_ins()?.repeat_act(val)?;
        Ok(())
    }
    fn can_get(&self) -> bool {
        false
    }
    fn get(&self) -> ProcResult<u8> {
        unreachable!("no getter for RepeatAction")
    }
}

fn force_act_orig_instr_off() -> i32 {
    match version() {
        Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) | Some(Version1_2_3)
        | Some(Version1_3_0) | Some(Version1_3_1) | Some(Version1_3_2) | Some(Version1_4_0)
        | Some(Version1_4_1) | Some(Version1_5_0) | Some(Version1_6_0) => 0xe9b1,
        _ => 0xe9c1,
    }
}
impl ValueCommand<ActArray> for ForceActSequence {
    fn set(&self, mut val: ActArray) -> anyhow::Result<()> {
        let location = CaveAddress::ForceActSequenceHook;
        let npc_think_param_id = target().chr_ins()?.npc_think_param_id()?;

        let mut fun = ASM.get_function("force_act_sequence_hook");
        let mut asm = fun.take_bytes();

        write_addr_to_slice(
            &mut asm,
            fun.reloc("should_run_flag"),
            CaveAddress::ActSeqeunceShouldRun,
        )?;
        write_to_slice::<i32>(&mut asm, fun.reloc("npc_think_param_id"), npc_think_param_id)?;
        write_addr_to_slice(&mut asm, fun.reloc("current_idx"), CaveAddress::CurrentActIdx)?;
        write_addr_to_slice(&mut asm, fun.reloc("act_array"), CaveAddress::ActArray)?;
        write_to_slice::<i32>(&mut asm, fun.reloc("orig_instr_off"), force_act_orig_instr_off())?;
        write_rel_i32(
            &mut asm,
            location,
            fun.reloc("hook_loc"),
            Hook::GetForceActIdx.add_offset(7),
            4,
        )?;

        val.zero_fill();
        write_bytes(CaveAddress::ActArray, &val.as_qword_le_bytes())?;
        write::<i32>(CaveAddress::CurrentActIdx, 0x0)?;
        write::<u8>(CaveAddress::ActSeqeunceShouldRun, 0x1)?;
        install_hook(&asm, location, Hook::GetForceActIdx, 7)?;
        Ok(())
    }
    fn can_get(&self) -> bool {
        false
    }
    fn get(&self) -> ProcResult<ActArray> {
        unreachable!("no getter for ForceActSequence")
    }
}

impl UnitCommand for ResetPosition {
    fn execute(&self) -> anyhow::Result<()> {
        target().chr_ins()?.reset_position()
    }
}

impl ToggleCommand for NoDamage {
    fn is(&self) -> ProcResult<bool> {
        target().chr_ins()?.is_no_damage()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        target().chr_ins()?.set_no_damage(state)?;
        Ok(())
    }
}

const TARGET_STAGGER_HOOK_BYTES_ORIGINAL: [u8; 8] =
    [0x48, 0x8b, 0x41, 0x08, 0x83, 0x48, 0x2c, 0x08];
impl ToggleCommand for NoStagger {
    fn is(&self) -> ProcResult<bool> {
        read::<[u8; 8]>(Hook::TargetNoStagger).map(|val| val != TARGET_STAGGER_HOOK_BYTES_ORIGINAL)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        if state {
            let mut fun = ASM.get_function("target_stagger_hook");
            let mut asm = fun.take_bytes();

            write_addr_to_slice(
                &mut asm,
                fun.reloc("target_ptr_loc"),
                CaveAddress::SavedTargetPointer,
            )?;
            write_rel_i32(
                &mut asm,
                CaveAddress::TargetNoStaggerHook,
                fun.reloc("hook_loc"),
                Hook::TargetNoStagger.add_offset(8),
                4,
            )?;
            install_hook(&asm, CaveAddress::TargetNoStaggerHook, Hook::TargetNoStagger, 8)?;
        } else {
            write_bytes(Hook::TargetNoStagger, &TARGET_STAGGER_HOOK_BYTES_ORIGINAL)?;
        }
        Ok(())
    }
}

impl ToggleCommand for DisableAi {
    fn is(&self) -> ProcResult<bool> {
        target().chr_ins()?.is_disable_ai()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        target().chr_ins()?.set_disable_ai(state)?;
        Ok(())
    }
}

impl ToggleCommand for RepeatLastAction {
    fn is(&self) -> ProcResult<bool> {
        target().chr_ins()?.is_repeat_act()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        target().chr_ins()?.set_repeat_last_act(state)?;
        Ok(())
    }
}
