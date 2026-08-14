use {
    crate::{
        game_state::{self, StateFlag},
        mem::*,
        offsets::{
            ChainReadExt,
            code_cave::CaveAddress,
            game_manager_imp::{self, game_data_manager_offsets::clearcount_ptr_offsets},
            module_offsets::{Data, Hook, Patch},
        },
        pointer_cache::ResolvedPtr,
        resources::asm_function,
        utils::player_loaded_check,
    },
    gubtool_core::{address::Address, attached::is_32, slice_ops::*, sys::sys_error::ProcResult},
    shared::{
        command::{ToggleCommand, UnitCommand, ValueCommand},
        declare_command,
    },
};

declare_command!(Quitout, NewGame, FastQuitout, SkipCredits, DisableRoll, DisableBackstep,);

impl UnitCommand for Quitout {
    fn execute(&self) -> anyhow::Result<()> {
        player_loaded_check()?;
        ResolvedPtr::GameManagerImp
            .get()
            .add_offset(game_manager_imp::QUITOUT)
            .write::<u8>(0x6)?;
        Ok(())
    }
}

impl ValueCommand<u8> for NewGame {
    fn get(&self) -> ProcResult<u8> {
        ResolvedPtr::ClearCountPtr
            .get()
            .add_offset(clearcount_ptr_offsets::CLEARCOUNT)
            .read::<u8>()
    }
    fn set(&self, val: u8) -> anyhow::Result<()> {
        player_loaded_check()?;
        ResolvedPtr::ClearCountPtr
            .get()
            .add_offset(clearcount_ptr_offsets::CLEARCOUNT)
            .write::<u8>(val)?;
        Ok(())
    }
}

const VANILLA_MENU_PATCH_ORIGINAL: [u8; 2] = [0x0f, 0x85];
const SCHOLAR_MENU_PATCH_ORIGINAL: [u8; 2] = [0x75, 0xea];
impl ToggleCommand for FastQuitout {
    fn is(&self) -> ProcResult<bool> {
        Ok(game_state::is_flag(StateFlag::FastQuitout))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::FastQuitout, state)?;
        Ok(())
    }
}

impl FastQuitout {
    pub(crate) fn is_hook(&self) -> ProcResult<bool> {
        let patch_orig = if is_32() {
            VANILLA_MENU_PATCH_ORIGINAL
        } else {
            SCHOLAR_MENU_PATCH_ORIGINAL
        };
        read::<[u8; 2]>(Patch::MenuTransition).map(|val| val != patch_orig)
    }
    pub(crate) fn set_hook(&self, state: bool) -> ProcResult {
        if state {
            let orig_instr_len = if is_32() {
                5
            } else {
                8
            };
            let patch_bytes = if is_32() {
                [0x0f, 0x84]
            } else {
                [0x74, 0xea]
            };
            write_bytes(Patch::MenuTransition, &patch_bytes)?;

            let mut fun = asm_function("faster_menu");
            let mut asm = fun.take_bytes();
            write_rel_i32(
                &mut asm,
                CaveAddress::FasterMenuHook,
                fun.reloc("hook_loc"),
                Hook::FasterMenu.add_offset(orig_instr_len),
                4,
            )?;
            install_hook(&asm, CaveAddress::FasterMenuHook, Hook::FasterMenu, orig_instr_len)
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
impl ToggleCommand for SkipCredits {
    fn is(&self) -> ProcResult<bool> {
        if is_32() {
            read::<[u8; 6]>(Hook::CreditsSkip)
                .map(|val| val != [0x81, 0xec, 0xfc, 0x01, 0x00, 0x00])
        } else {
            read::<[u8; 7]>(Hook::CreditsSkip)
                .map(|val| val != [0x48, 0x81, 0xec, 0x20, 0x02, 0x00, 0x00])
        }
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        if state {
            let orig_instr_len = if is_32() {
                6
            } else {
                7
            };
            let modify_once = CaveAddress::CreditsModifyOnceFlag;
            let mut fun = asm_function("credits_skip");
            let mut asm = fun.take_bytes();

            write_addr_to_slice(&mut asm, fun.reloc("modify_once_flag"), modify_once)?;
            write_rel_i32(
                &mut asm,
                CaveAddress::CreditsSkipHook,
                fun.reloc("hook_loc"),
                Hook::CreditsSkip.add_offset(orig_instr_len),
                4,
            )?;
            write::<u8>(modify_once, 0x0)?;
            install_hook(&asm, CaveAddress::CreditsSkipHook, Hook::CreditsSkip, orig_instr_len)?;
        } else {
            let bytes: &[u8] = if is_32() {
                &VANILLA_CREDITS_ORIGINAL
            } else {
                &SCHOLAR_CREDITS_ORIGINAL
            };
            write_bytes(Hook::CreditsSkip, bytes)?;
        }
        Ok(())
    }
}

const DISABLE_ROLL_ORIGINAL: [u8; 2] = [0xb0, 0x01];
impl ToggleCommand for DisableRoll {
    fn is(&self) -> ProcResult<bool> {
        read::<[u8; 2]>(Patch::NoRoll).map(|val| val != DISABLE_ROLL_ORIGINAL)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        let bytes = if state {
            [0x30, 0xc0]
        } else {
            DISABLE_ROLL_ORIGINAL
        };
        write_bytes(Patch::NoRoll, &bytes)?;
        Ok(())
    }
}

const DISABLE_BACKSTEP_ORIGINAL: [u8; 3] = [0x0f, 0x95, 0xc0];
impl ToggleCommand for DisableBackstep {
    fn is(&self) -> ProcResult<bool> {
        read::<[u8; 3]>(Patch::NoBackstep).map(|val| val != DISABLE_BACKSTEP_ORIGINAL)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        let bytes = if state {
            [0x30, 0xc0, 0x90]
        } else {
            DISABLE_BACKSTEP_ORIGINAL
        };
        write_bytes(Patch::NoBackstep, &bytes)?;
        Ok(())
    }
}

pub fn get_area_id() -> ProcResult<u32> {
    read::<u32>(Data::MapId)
}
