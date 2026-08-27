use {
    crate::{
        game_state::{self, StateFlag},
        mem::*,
        offsets::{
            ChainReadExt,
            code_cave::CaveAddr,
            game_manager_imp::{self, game_data_manager_offsets::clearcount_ptr_offsets},
            module_offsets::{Data, Function, Hook, Patch},
        },
        pointer_cache::ResolvedPtr,
        resources::asm_function,
        utils::player_loaded_check,
    },
    gubtool_core::{
        address::{Address, POINTER},
        attached::is_32,
        sys::{ipc::X86CallingConvention, sys_error::SysResult},
    },
    shared::{
        command::{ToggleCommand, UnitCommand, ValueCommand},
        toggle_command,
        unit_command,
        value_command,
    },
};

unit_command!(Quitout {
    player_loaded_check()?;
    Ok(ResolvedPtr::GameManagerImp
        .get()
        .add_offset(game_manager_imp::QUITOUT)
        .write::<u8>(0x6)?)
});

value_command!(NewGameCycle, u8 {
    get: {
        ResolvedPtr::ClearCountPtr
            .get()
            .add_offset(clearcount_ptr_offsets::CLEARCOUNT)
            .read::<u8>()
    }

    set(val): {
        player_loaded_check()?;
        Ok(ResolvedPtr::ClearCountPtr
            .get()
            .add_offset(clearcount_ptr_offsets::CLEARCOUNT)
            .write::<u8>(val)?)
    }
});

unit_command!(TriggerNewGame {
    run_game_function(Function::TriggerNewGame, &[], X86CallingConvention::__cdecl)
});

const VANILLA_MENU_PATCH_ORIGINAL: [u8; 2] = [0x0f, 0x85];
const SCHOLAR_MENU_PATCH_ORIGINAL: [u8; 2] = [0x75, 0xea];
toggle_command!(FastQuitout {
    is: {
        Ok(game_state::is_flag(StateFlag::FastQuitout))
    }

    set(state): {
        Ok(game_state::set_flag(StateFlag::FastQuitout, state)?)
    }
});

pub fn enable_skip_logos() -> SysResult {
    write::<u8>(Patch::SkipLogos, 0x1)
}

impl FastQuitout {
    pub(crate) fn is_hook(&self) -> SysResult<bool> {
        let patch_orig =
            if is_32() { VANILLA_MENU_PATCH_ORIGINAL } else { SCHOLAR_MENU_PATCH_ORIGINAL };
        read::<[u8; 2]>(Patch::MenuTransition).map(|val| val != patch_orig)
    }
    pub(crate) fn set_hook(&self, state: bool) -> SysResult {
        if state {
            let orig_instr_len = if is_32() { 5 } else { 8 };
            let patch_bytes = if is_32() { [0x0f, 0x84] } else { [0x74, 0xea] };
            write_bytes(Patch::MenuTransition, &patch_bytes)?;

            let mut fun = asm_function("faster_menu");
            fun.patch_rel32(
                "hook_loc",
                CaveAddr::FasterMenuHook,
                Hook::FasterMenu.add(orig_instr_len),
                4,
            );
            install_hook(&fun.bytes, CaveAddr::FasterMenuHook, Hook::FasterMenu, orig_instr_len)
        } else {
            let patch_orig = if is_32() {
                &VANILLA_MENU_PATCH_ORIGINAL
            } else {
                &SCHOLAR_MENU_PATCH_ORIGINAL
            };
            let hook_orig: &[u8] = if is_32() {
                &[0x33, 0xc5, 0x89, 0x45, 0xfc]
            } else {
                &[0x48, 0x89, 0x84, 0x24, 0x50, 0x01, 0x00, 0x0]
            };
            write_bytes(Patch::MenuTransition, patch_orig)?;
            write_bytes(Hook::FasterMenu, hook_orig)
        }
    }
}

const VANILLA_CREDITS_ORIGINAL: [u8; 6] = [0x81, 0xec, 0xfc, 0x01, 0x00, 0x00];
const SCHOLAR_CREDITS_ORIGINAL: [u8; 7] = [0x48, 0x81, 0xec, 0x20, 0x02, 0x00, 0x00];
toggle_command!(SkipCredits {
    is: {
        if is_32() {
            read::<[u8; 6]>(Hook::CreditsSkip)
                .map(|val| val != [0x81, 0xec, 0xfc, 0x01, 0x00, 0x00])
        } else {
            read::<[u8; 7]>(Hook::CreditsSkip)
                .map(|val| val != [0x48, 0x81, 0xec, 0x20, 0x02, 0x00, 0x00])
        }
    }

    set(state): {
        if state {
            let orig_instr_len = if is_32() { 6 } else { 7 };
            let modify_once = CaveAddr::CreditsModifyOnceFlag;
            let mut fun = asm_function("credits_skip");

            fun.patch::<POINTER>("modify_once_flag", modify_once);
            fun.patch_rel32(
                "hook_loc",
                CaveAddr::CreditsSkipHook,
                Hook::CreditsSkip.add(orig_instr_len),
                4,
            );

            write::<u8>(modify_once, 0x0)?;
            install_hook(&fun.bytes, CaveAddr::CreditsSkipHook, Hook::CreditsSkip, orig_instr_len)?;
        } else {
            let bytes: &[u8] =
                if is_32() { &VANILLA_CREDITS_ORIGINAL } else { &SCHOLAR_CREDITS_ORIGINAL };
            write_bytes(Hook::CreditsSkip, bytes)?;
        }
        Ok(())
    }
});

const DISABLE_ROLL_ORIGINAL: [u8; 2] = [0xb0, 0x01];
toggle_command!(DisableRoll {
    is: {
        read::<[u8; 2]>(Patch::NoRoll)
            .map(|val| val != DISABLE_ROLL_ORIGINAL)
    }

    set(state): {
        let bytes = if state { [0x30, 0xc0] } else { DISABLE_ROLL_ORIGINAL };
        Ok(write_bytes(Patch::NoRoll, &bytes)?)
    }
});

const DISABLE_BACKSTEP_ORIGINAL: [u8; 3] = [0x0f, 0x95, 0xc0];
toggle_command!(DisableBackstep {
    is: {
        read::<[u8; 3]>(Patch::NoBackstep)
            .map(|val| val != DISABLE_BACKSTEP_ORIGINAL)
    }

    set(state): {
        let bytes = if state { [0x30, 0xc0, 0x90] } else { DISABLE_BACKSTEP_ORIGINAL };
        Ok(write_bytes(Patch::NoBackstep, &bytes)?)
    }
});

pub fn get_area_id() -> SysResult<u32> {
    read::<u32>(Data::MapId)
}

pub enum ElanaSummon {
    Pigs,
    Skeletons,
    Velstadt,
}
