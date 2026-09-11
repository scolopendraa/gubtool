pub use crate::obj_refs::target;
use {
    crate::{
        enemy,
        offsets::{code_cave::CaveAddr, module_offsets::Hook},
        resources::asm_function,
    },
    gubtool_core::{
        address::{Address, POINTER},
        attached::is_32,
        sys::{install_hook, read, sys_error::SysResult, write, write_bytes},
    },
    shared::{
        command::ValueCommand,
        toggle_command,
        unit_command,
        value_command,
        value_command_set,
    },
};

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
                Hook::LockedTargetPointer.ptr() + orig_instr_len,
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
        target()?.get_hp()
    }
    set(val): {
        Ok(target()?.set_hp(val)?)
    }
});

value_command!(HealthPercentage, f32 (display = "Health %") (cli_name = "target-health-percentage") {
    get: {
        target()?.get_hp_pct()
    }

    set(val): {
        target()?.set_hp_pct(val)
    }
});

unit_command!(Kill (cli_name = "target-kill") {
    Ok(target()?.set_hp(0)?)
});

value_command_set!(RepeatAction, i32 (cli_name = "target-repeat-action") {
    set(val): {
        Ok(target()?.repeat_action(val)?)
    }
});

toggle_command!(RepeatLastAction (cli_name = "target-repeat-last-action") {
    is: {
        target()?.is_action_repeating()
    }

    set(state): {
        Ok(target()?.repeat_last_action(state)?)
    }
});

toggle_command!(DisableAi (cli_name = "target-disable-ai") {
    is: {
        target()?.is_ai_disabled()
    }

    set(state): {
        Ok(target()?.set_disable_ai(state)?)
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
