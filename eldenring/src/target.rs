pub use crate::phase_transition::target_next_phase as next_phase;
use {
    crate::{
        chr_ins::{ChrIns, ResolvedChrPtr},
        mem::*,
        offsets::{ChainReadExt, code_cave::CaveAddr, lock_tgt_man_imp, module_offsets::Hook},
        phase_transition,
        pointer_cache::ResolvedPtr,
        resources::ASM,
    },
    assemble::patch::DWORD,
    gubtool_core::{
        address::{Address, POINTER},
        attached::version,
        game_version::EldenRingVersion::*,
        sys::sys_error::{PointerType, SysError, SysResult},
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
        match read::<u64>(CaveAddr::SavedTargetPointer) {
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

    pub fn chr_ins(&mut self) -> SysResult<&mut ChrIns> {
        self.chr_ins
            .as_mut()
            .ok_or(SysError::null_pointer(PointerType::Target))
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

pub fn unlock() -> SysResult {
    ResolvedPtr::LockTgtMan
        .get()
        .add_offset(lock_tgt_man_imp::IS_LOCKED)
        .write::<u8>(0x0)
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
    fn is(&self) -> SysResult<bool> {
        read::<[u8; 7]>(Hook::SaveTarget).map(|val| val != TARGET_HOOK_BYTES_ORIGINAL)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        if state {
            let mut fun = ASM.get_function("save_target_hook");

            fun.patch::<POINTER>("saved_pointer_loc", CaveAddr::SavedTargetPointer);
            fun.patch_rel32("hook_loc", CaveAddr::SaveTargetHook, Hook::SaveTarget.add(7), 4);
            install_hook(&fun.bytes, CaveAddr::SaveTargetHook, Hook::SaveTarget, 7)?;
        } else {
            write_bytes(Hook::SaveTarget, &TARGET_HOOK_BYTES_ORIGINAL)?;
        }
        Ok(())
    }
}

impl ValueCommand<i32> for Health {
    fn get(&self) -> SysResult<i32> {
        target().chr_ins()?.get_current_hp()
    }
    fn set(&self, val: i32) -> anyhow::Result<()> {
        target().chr_ins()?.set_hp(val)?;
        Ok(())
    }
}

impl ValueCommand<f32> for HealthPercentage {
    fn get(&self) -> SysResult<f32> {
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
    fn get(&self) -> SysResult<u8> {
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
        let location = CaveAddr::ForceActSequenceHook;
        let npc_think_param_id = target().chr_ins()?.npc_think_param_id()?;

        let mut fun = ASM.get_function("force_act_sequence_hook");

        fun.patch::<POINTER>("should_run_flag", CaveAddr::ActSeqeunceShouldRun);
        fun.patch::<DWORD>("npc_think_param_id", npc_think_param_id);
        fun.patch::<POINTER>("current_idx", CaveAddr::CurrentActIdx);
        fun.patch::<POINTER>("act_array", CaveAddr::ActArray);
        fun.patch::<DWORD>("orig_instr_off", force_act_orig_instr_off());
        fun.patch_rel32("hook_loc", location, Hook::GetForceActIdx.add(7), 4);

        val.zero_fill();
        write_bytes(CaveAddr::ActArray, &val.as_qword_le_bytes())?;
        write::<i32>(CaveAddr::CurrentActIdx, 0x0)?;
        write::<u8>(CaveAddr::ActSeqeunceShouldRun, 0x1)?;
        install_hook(&fun.bytes, location, Hook::GetForceActIdx, 7)?;
        Ok(())
    }
    fn can_get(&self) -> bool {
        false
    }
    fn get(&self) -> SysResult<ActArray> {
        unreachable!("no getter for ForceActSequence")
    }
}

impl UnitCommand for ResetPosition {
    fn execute(&self) -> anyhow::Result<()> {
        target().chr_ins()?.reset_position()
    }
}

impl ToggleCommand for NoDamage {
    fn is(&self) -> SysResult<bool> {
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
    fn is(&self) -> SysResult<bool> {
        read::<[u8; 8]>(Hook::TargetStagger).map(|val| val != TARGET_STAGGER_HOOK_BYTES_ORIGINAL)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        if state {
            let mut fun = ASM.get_function("target_stagger_hook");
            fun.patch::<POINTER>("target_ptr_loc", CaveAddr::SavedTargetPointer);
            fun.patch_rel32("hook_loc", CaveAddr::TargetStaggerHook, Hook::TargetStagger.add(8), 4);
            install_hook(&fun.bytes, CaveAddr::TargetStaggerHook, Hook::TargetStagger, 8)?;
        } else {
            write_bytes(Hook::TargetStagger, &TARGET_STAGGER_HOOK_BYTES_ORIGINAL)?;
        }
        Ok(())
    }
}

impl ToggleCommand for DisableAi {
    fn is(&self) -> SysResult<bool> {
        target().chr_ins()?.is_disable_ai()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        target().chr_ins()?.set_disable_ai(state)?;
        Ok(())
    }
}

impl ToggleCommand for RepeatLastAction {
    fn is(&self) -> SysResult<bool> {
        target().chr_ins()?.is_repeat_act()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        target().chr_ins()?.set_repeat_last_act(state)?;
        Ok(())
    }
}
