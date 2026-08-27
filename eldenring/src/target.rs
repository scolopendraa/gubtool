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
        game_version::{EldenRingVersion, EldenRingVersion::*},
        sys::sys_error::{PointerType, SysError, SysResult},
    },
    shared::{
        act_array::ActArray,
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

const TARGET_HOOK_BYTES_ORIGINAL: [u8; 7] = [0x48, 0x8b, 0x8f, 0x88, 0x00, 0x00, 0x00];
toggle_command!(SaveTargetHook {
    is: {
        read::<[u8; 7]>(Hook::SaveTarget)
            .map(|val| val != TARGET_HOOK_BYTES_ORIGINAL)
    }

    set(state): {
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
});

value_command!(Health, i32 (cli_name = "target-health") {
    get: {
        target().chr_ins()?.get_current_hp()
    }

    set(val): {
        Ok(target().chr_ins()?.set_hp(val)?)
    }

});

value_command!(HealthPercentage, f32 (display = "Health %") (cli_name = "target-health-percentage") {
    get: {
        target().chr_ins()?.get_hp_pct()
    }

    set(val): {
        target().chr_ins()?.set_hp_pct(val)
    }
});

unit_command!(Kill (cli_name = "target-kill") {
    Ok(target().chr_ins()?.set_hp(0)?)
});

unit_command!(NextPhase (cli_name = "target-next-phase") {
    phase_transition::target_next_phase()
});

value_command_set!(RepeatAction, u8 (cli_name = "target-repeat-action") {
    set(val): {
        Ok(target().chr_ins()?.repeat_act(val)?)
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
        let npc_think_param_id = target().chr_ins()?.npc_think_param_id()?;

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
    target().chr_ins()?.reset_position()
});

toggle_command!(NoDamage (cli_name = "target-no-damage") {
    is: {
        target().chr_ins()?.is_no_damage()
    }

    set(state): {
        Ok(target().chr_ins()?.set_no_damage(state)?)
    }
});

const TARGET_STAGGER_HOOK_BYTES_ORIGINAL: [u8; 8] =
    [0x48, 0x8b, 0x41, 0x08, 0x83, 0x48, 0x2c, 0x08];
toggle_command!(NoStagger (cli_name = "target-no-stagger") {
    is: {
        read::<[u8; 8]>(Hook::TargetStagger)
            .map(|val| val != TARGET_STAGGER_HOOK_BYTES_ORIGINAL)
    }

    set(state): {
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
});

toggle_command!(DisableAi (cli_name = "target-disable-ai") {
    is: {
        target().chr_ins()?.is_disable_ai()
    }

    set(state): {
        Ok(target().chr_ins()?.set_disable_ai(state)?)
    }
});

toggle_command!(RepeatLastAction (cli_name = "target-repeat-last-action") {
    is: {
        target().chr_ins()?.is_repeat_act()
    }

    set(state): {
        Ok(target().chr_ins()?.set_repeat_last_act(state)?)
    }
});
