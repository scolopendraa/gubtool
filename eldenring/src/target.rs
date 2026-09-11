pub use crate::obj_refs::target;
use {
    crate::{
        offsets::{code_cave::CaveAddr, lock_tgt_man_imp, module_offsets::Hook},
        phase_transition,
        pointer_cache::ResolvedPtr,
        resources::ASM,
    },
    assemble::patch::DWORD,
    gubtool_core::{
        address::{Address, ChainReadExt, POINTER},
        attached::version,
        game_version::EldenRingVersion::{self, *},
        sys::{install_hook, read, sys_error::SysResult, write, write_bytes},
    },
    shared::{
        act_array::ActArray,
        command::ValueCommand,
        toggle_command,
        unit_command,
        value_command,
        value_command_set,
    },
};

pub fn unlock() -> SysResult {
    ResolvedPtr::LockTgtMan
        .get()
        .add_offset(lock_tgt_man_imp::IS_LOCKED)
        .write::<u8>(0x0)
}

const TARGET_HOOK_BYTES_ORIGINAL: [u8; 7] = [0x48, 0x8b, 0x8f, 0x88, 0x00, 0x00, 0x00];
toggle_command!(SaveTargetHook {
    is: {
        read::<[u8; 7]>(Hook::LockedTargetPointer)
            .map(|val| val != TARGET_HOOK_BYTES_ORIGINAL)
    }

    set(state): {
        if state {
            let mut fun = ASM.get_function("save_target_hook");
            fun.patch::<POINTER>("saved_pointer_loc", CaveAddr::SavedTargetPointer);
            fun.patch_rel32("hook_loc", CaveAddr::SaveTargetHook, Hook::LockedTargetPointer.add(7), 4);
            install_hook(&fun.bytes, CaveAddr::SaveTargetHook, Hook::LockedTargetPointer, 7)?;
        } else {
            write_bytes(Hook::LockedTargetPointer, &TARGET_HOOK_BYTES_ORIGINAL)?;
        }
        Ok(())
    }
});

value_command!(Health, i32 (cli_name = "target-health") {
    get: {
        target()?.get_current_hp()
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

unit_command!(NextPhase (cli_name = "target-next-phase") {
    phase_transition::target_next_phase()
});

value_command_set!(RepeatAction, u8 (cli_name = "target-repeat-action") {
    set(val): {
        Ok(target()?.repeat_act(val)?)
    }
});

value_command_set!(ForceActSequence, ActArray (cli_name = "target-force-act-sequence") {
    set(val): {
        let orig_instr_off = {
            match version::<EldenRingVersion>() {
                Some(v) if v <= Version1_6_0 => 0xe9b1,
                _ => 0xe9c1,
            }
        };

        let location = CaveAddr::ForceActSequenceHook;
        let npc_think_param_id = target()?.npc_think_param_id()?;

        let mut fun = ASM.get_function("force_act_sequence_hook");

        fun.patch::<POINTER>("should_run_flag", CaveAddr::ActSeqeunceShouldRun);
        fun.patch::<DWORD>("npc_think_param_id", npc_think_param_id);
        fun.patch::<POINTER>("current_idx", CaveAddr::CurrentActIdx);
        fun.patch::<POINTER>("act_array", CaveAddr::ActArray);
        fun.patch::<DWORD>("orig_instr_off", orig_instr_off);
        fun.patch_rel32("hook_loc", location, Hook::GetForceActIdx.add(7), 4);

        write_bytes(CaveAddr::ActArray, &val.as_dword_le_bytes())?;
        write::<i32>(CaveAddr::CurrentActIdx, 0x0)?;
        write::<u8>(CaveAddr::ActSeqeunceShouldRun, 0x1)?;
        Ok(install_hook(&fun.bytes, location, Hook::GetForceActIdx, 7)?)
    }
});

unit_command!(ResetPosition (cli_name = "target-reset-position") {
    target()?.reset_position()
});

toggle_command!(NoDamage (cli_name = "target-no-damage") {
    is: {
        target()?.is_no_damage()
    }

    set(state): {
        Ok(target()?.set_no_damage(state)?)
    }
});

const TARGET_STAGGER_HOOK_BYTES_ORIGINAL: [u8; 8] =
    [0x48, 0x8b, 0x41, 0x08, 0x83, 0x48, 0x2c, 0x08];
toggle_command!(NoStagger (cli_name = "target-no-stagger") {
    is: {
        read::<[u8; 8]>(Hook::TargetNoStagger)
            .map(|val| val != TARGET_STAGGER_HOOK_BYTES_ORIGINAL)
    }

    set(state): {
        if state {
            let mut fun = ASM.get_function("target_stagger_hook");
            fun.patch::<POINTER>("target_ptr_loc", CaveAddr::SavedTargetPointer);
            fun.patch_rel32("hook_loc", CaveAddr::TargetStaggerHook, Hook::TargetNoStagger.add(8), 4);
            install_hook(&fun.bytes, CaveAddr::TargetStaggerHook, Hook::TargetNoStagger, 8)?;
        } else {
            write_bytes(Hook::TargetNoStagger, &TARGET_STAGGER_HOOK_BYTES_ORIGINAL)?;
        }
        Ok(())
    }
});

toggle_command!(DisableAi (cli_name = "target-disable-ai") {
    is: {
        target()?.is_disable_ai()
    }

    set(state): {
        Ok(target()?.set_disable_ai(state)?)
    }
});

toggle_command!(RepeatLastAction (cli_name = "target-repeat-last-action") {
    is: {
        target()?.is_repeat_act()
    }

    set(state): {
        Ok(target()?.set_repeat_last_act(state)?)
    }
});
