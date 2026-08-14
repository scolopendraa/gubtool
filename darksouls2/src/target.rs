use {
    crate::{
        chr_ctrl::{ChrCtrl, ResolvedChrPtr},
        mem::*,
        offsets::{
            code_cave::CaveAddress,
            module_offsets::{Function, Hook},
        },
        resources::asm_function,
    },
    gubtool_core::{
        address::Address,
        attached::is_32,
        slice_ops::*,
        sys::sys_error::{PointerType, ProcResult, ProcessError},
    },
    shared::{
        command::{ToggleCommand, UnitCommand, ValueCommand},
        declare_command,
    },
    std::{
        collections::HashMap,
        sync::{LazyLock, Mutex, MutexGuard},
    },
};

static TARGET: LazyLock<Mutex<Target>> = LazyLock::new(|| Mutex::new(Target::new()));

pub fn target() -> MutexGuard<'static, Target> {
    TARGET.lock().unwrap()
}

#[derive(Debug)]
pub struct Target {
    chr_ctrl: Option<ChrCtrl>,
}

impl Target {
    fn new() -> Self {
        let mut target = Self {
            chr_ctrl: None,
        };
        target.update();
        target
    }

    pub fn update(&mut self) {
        match read_address(CaveAddress::SavedTargetPointer) {
            Ok(new_target) => {
                let same_target = self
                    .chr_ctrl
                    .as_ref()
                    .and_then(|chr| chr.resolved_pointers.get(&ResolvedChrPtr::ChrCtrl))
                    .is_some_and(|ptr| *ptr == new_target);

                if !same_target {
                    if new_target != 0 {
                        self.chr_ctrl = Some(ChrCtrl::new(new_target));
                    } else {
                        self.chr_ctrl = None;
                    }
                }
            }
            Err(_) => {
                self.chr_ctrl = None;
            }
        }

        if let Some(chr_ctrl) = &mut self.chr_ctrl
            && !chr_ctrl.is_valid_chr().unwrap_or(false)
        {
            self.clear();
        }
    }

    pub fn clear(&mut self) {
        let _ = write::<u64>(CaveAddress::SavedTargetPointer, 0x0);
        self.chr_ctrl = None
    }

    pub fn chr_ctrl(&mut self) -> ProcResult<&mut ChrCtrl> {
        self.chr_ctrl
            .as_mut()
            .ok_or(ProcessError::null_pointer(PointerType::Target))
    }

    pub fn set(&mut self, chr_ctrl: ChrCtrl) {
        self.chr_ctrl = Some(chr_ctrl)
    }

    pub fn pointers(&self) -> Vec<(String, u64)> {
        self.chr_ctrl
            .as_ref()
            .map(|c| c.pointers())
            .unwrap_or_default()
    }
}

static ACT_LOGGER: LazyLock<Mutex<ActLogger>> = LazyLock::new(|| Mutex::new(ActLogger::default()));

pub fn act_logger() -> MutexGuard<'static, ActLogger> {
    ACT_LOGGER.lock().unwrap()
}

#[derive(Default, Debug)]
pub struct ActLogger {
    map:      HashMap<u64, i32>,
    read_idx: i32,
}

impl ActLogger {
    pub fn update(&mut self) -> ProcResult {
        let buffer = read::<[u8; 0x50]>(CaveAddress::SavedActBuffer)?;
        let (write_idx, buffer) = buffer.split_at(4);
        let write_idx = read_from_slice::<i32>(write_idx, 0)?;

        let num_to_read = ((write_idx - self.read_idx).rem_euclid(6)) as usize;

        for i in 0..num_to_read {
            let idx = (self.read_idx + i as i32) % 6;
            let read_offset = idx * 12;

            let chr_ai_pointer = read_from_slice::<u64>(buffer, read_offset as u64)?;
            let act_id = read_from_slice::<i32>(buffer, (read_offset + 8) as u64)?;

            let _ = self.map.insert(chr_ai_pointer, act_id);
        }

        self.read_idx = write_idx;
        Ok(())
    }

    pub fn get(&self, chr_ai: u64) -> Option<i32> {
        self.map.get(&chr_ai).copied()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

declare_command!(
    SaveTargetHook,
    ActHook,
    Health,
    HealthPercentage => "Health %",
    Kill,
    RepeatAction,
    RepeatLastAction,
);

const SAVE_TARGET_ORIGINAL_VANILLA: [u8; 6] = [0x89, 0xb7, 0xb8, 0x00, 0x00, 0x00];
const SAVE_TARGET_ORIGINAL_SCHOLAR: [u8; 7] = [0x48, 0x89, 0xbb, 0xc0, 0x00, 0x00, 0x00];
impl ToggleCommand for SaveTargetHook {
    fn is(&self) -> ProcResult<bool> {
        if is_32() {
            read::<[u8; 6]>(Hook::LockedTargetPointer)
                .map(|val| val != SAVE_TARGET_ORIGINAL_VANILLA)
        } else {
            read::<[u8; 7]>(Hook::LockedTargetPointer)
                .map(|val| val != SAVE_TARGET_ORIGINAL_SCHOLAR)
        }
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        if state {
            let orig_instr_len = if is_32() {
                6
            } else {
                7
            };

            let mut fun = asm_function("save_target_hook");
            let mut asm = fun.take_bytes();

            write_addr_to_slice(
                &mut asm,
                fun.reloc("saved_ptr_loc"),
                CaveAddress::SavedTargetPointer,
            )?;
            write_rel_i32(
                &mut asm,
                CaveAddress::SaveTargetHook,
                fun.reloc("hook_loc"),
                Hook::LockedTargetPointer.add_offset(orig_instr_len),
                4,
            )?;
            install_hook(
                &asm,
                CaveAddress::SaveTargetHook,
                Hook::LockedTargetPointer,
                orig_instr_len,
            )?;
        } else {
            let bytes: &[u8] = match is_32() {
                true => &SAVE_TARGET_ORIGINAL_VANILLA,
                false => &SAVE_TARGET_ORIGINAL_SCHOLAR,
            };
            write_bytes(Hook::LockedTargetPointer, bytes)?;
        }
        Ok(())
    }
}

const SET_ACT_ORIGINAL_VANILLA: [u8; 7] = [0x55, 0x8b, 0xec, 0x8b, 0x45, 0x08, 0x83];
const SET_ACT_ORIGINAL_SCHOLAR: [u8; 7] = [0x83, 0x89, 0x50, 0x03, 0x00, 0x00, 0x01];
impl ToggleCommand for ActHook {
    fn is(&self) -> ProcResult<bool> {
        if is_32() {
            read::<[u8; 7]>(Function::ChrSetAction).map(|val| val != SET_ACT_ORIGINAL_VANILLA)
        } else {
            read::<[u8; 7]>(Function::ChrSetAction).map(|val| val != SET_ACT_ORIGINAL_SCHOLAR)
        }
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        if state {
            let mut fun = asm_function("target_action_hook");
            let mut asm = fun.take_bytes();

            write_addr_to_slice(&mut asm, fun.reloc("force_act_flag"), CaveAddress::ForceActFlag)?;
            write_addr_to_slice(
                &mut asm,
                fun.reloc("repeating_chr_ai"),
                CaveAddress::ForceActChrAi,
            )?;
            write_addr_to_slice(&mut asm, fun.reloc("force_act_id"), CaveAddress::ForceActId)?;
            write_addr_to_slice(&mut asm, fun.reloc("buffer"), CaveAddress::SavedActBuffer)?;

            install_hook(&asm, CaveAddress::TargetActHook, Function::ChrSetAction, 7)?;
        } else {
            let bytes: &[u8] = match is_32() {
                true => &SET_ACT_ORIGINAL_VANILLA,
                false => &SET_ACT_ORIGINAL_SCHOLAR,
            };
            write_bytes(Function::ChrSetAction, bytes)?;
        }
        Ok(())
    }
}

impl ValueCommand<i32> for Health {
    fn get(&self) -> ProcResult<i32> {
        target().chr_ctrl()?.get_hp()
    }
    fn set(&self, val: i32) -> anyhow::Result<()> {
        target().chr_ctrl()?.set_hp(val)?;
        Ok(())
    }
}

impl ValueCommand<f32> for HealthPercentage {
    fn get(&self) -> ProcResult<f32> {
        target().chr_ctrl()?.get_hp_pct()
    }
    fn set(&self, val: f32) -> anyhow::Result<()> {
        target().chr_ctrl()?.set_hp_pct(val)
    }
}

impl UnitCommand for Kill {
    fn execute(&self) -> anyhow::Result<()> {
        target().chr_ctrl()?.set_hp(0)?;
        Ok(())
    }
}

impl ValueCommand<i32> for RepeatAction {
    fn set(&self, val: i32) -> anyhow::Result<()> {
        target().chr_ctrl()?.repeat_action(val)?;
        Ok(())
    }
    fn can_get(&self) -> bool {
        false
    }
    fn get(&self) -> ProcResult<i32> {
        unreachable!("no getter for RepeatAction")
    }
}

impl ToggleCommand for RepeatLastAction {
    fn is(&self) -> ProcResult<bool> {
        target().chr_ctrl()?.is_action_repeating()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        target().chr_ctrl()?.repeat_last_action(state)?;
        Ok(())
    }
}
