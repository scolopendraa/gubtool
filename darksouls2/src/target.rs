use {
    crate::{
        chr_ctrl::{ChrCtrl, ResolvedChrPtr},
        enemy,
        mem::*,
        offsets::{code_cave::CaveAddr, module_offsets::Hook},
        resources::asm_function,
    },
    gubtool_core::{
        address::{Address, POINTER},
        attached::is_32,
        sys::sys_error::{PointerType, SysError, SysResult},
    },
    shared::{
        command::{ToggleCommand, UnitCommand, ValueCommand},
        toggle_command,
        unit_command,
        value_command,
        value_command_set,
    },
    std::sync::{LazyLock, Mutex, MutexGuard},
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
        match read_address(CaveAddr::SavedTargetPointer) {
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
        let _ = write::<u64>(CaveAddr::SavedTargetPointer, 0x0);
        self.chr_ctrl = None
    }

    pub fn chr_ctrl(&mut self) -> SysResult<&mut ChrCtrl> {
        self.chr_ctrl
            .as_mut()
            .ok_or(SysError::null_pointer(PointerType::Target))
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

const SAVE_TARGET_ORIGINAL_VANILLA: [u8; 6] = [0x89, 0xb7, 0xb8, 0x00, 0x00, 0x00];
const SAVE_TARGET_ORIGINAL_SCHOLAR: [u8; 7] = [0x48, 0x89, 0xbb, 0xc0, 0x00, 0x00, 0x00];
toggle_command!(SaveTargetHook {
    is: {
        if is_32() {
            read::<[u8; 6]>(Hook::LockedTargetPointer)
                .map(|val| val != SAVE_TARGET_ORIGINAL_VANILLA)
        } else {
            read::<[u8; 7]>(Hook::LockedTargetPointer)
                .map(|val| val != SAVE_TARGET_ORIGINAL_SCHOLAR)
        }
    }

    set(state): {
        if state {
            let orig_instr_len = if is_32() { 6 } else { 7 };

            let mut fun = asm_function("save_target_hook");

            fun.patch::<POINTER>("saved_ptr_loc", CaveAddr::SavedTargetPointer);
            fun.patch_rel32(
                "hook_loc",
                CaveAddr::SaveTargetHook,
                Hook::LockedTargetPointer.add(orig_instr_len),
                4,
            );
            install_hook(
                &fun.bytes,
                CaveAddr::SaveTargetHook,
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
});

value_command!(Health, i32 (cli_name = "target-health") {
    get: {
        target().chr_ctrl()?.get_hp()
    }
    set(val): {
        Ok(target().chr_ctrl()?.set_hp(val)?)
    }
});

value_command!(HealthPercentage, f32 (display = "Health %") (cli_name = "target-health-percentage") {
    get: {
        target().chr_ctrl()?.get_hp_pct()
    }

    set(val): {
        target().chr_ctrl()?.set_hp_pct(val)
    }
});

unit_command!(Kill (cli_name = "target-kill") {
    Ok(target().chr_ctrl()?.set_hp(0)?)
});

value_command_set!(RepeatAction, i32 (cli_name = "target-repeat-action") {
    set(val): {
        Ok(target().chr_ctrl()?.repeat_action(val)?)
    }
});

toggle_command!(RepeatLastAction (cli_name = "target-repeat-last-action") {
    is: {
        target().chr_ctrl()?.is_action_repeating()
    }

    set(state): {
        Ok(target().chr_ctrl()?.repeat_last_action(state)?)
    }
});

toggle_command!(DisableAi (cli_name = "target-disable-ai") {
    is: {
        target().chr_ctrl()?.is_ai_disabled()
    }

    set(state): {
        Ok(target().chr_ctrl()?.set_disable_ai(state)?)
    }
});

toggle_command!(DisableAiExceptTarget {
    is: {
        read::<u8>(CaveAddr::DisableAllExceptTarget).map(|val| val != 0)
    }

    set(state): {
        let byte = match state {
            true => {
                enemy::install_disable_ai_hook()?;
                1
            }
            false => 0,
        };
        Ok(write::<u8>(CaveAddr::DisableAllExceptTarget, byte)?)
    }
});
