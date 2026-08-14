use {
    crate::{
        event,
        game_state::{self, StateFlag},
        mem::*,
        offsets::{
            ChainReadExt,
            code_cave::CaveAddress,
            cs_flipper_imp,
            damage_manager,
            dl_user_input_manager_impl,
            game_data_man,
            game_man,
            map_dbg_flags,
            module_offsets::{Data, Hook, Patch},
        },
        pointer_cache::ResolvedPtr,
        resources::ASM,
        utils::player_loaded_check,
    },
    gubtool_core::{address::Address, slice_ops::*, sys::sys_error::ProcResult},
    shared::{
        command::{ToggleCommand, UnitCommand, ValueCommand},
        declare_command,
    },
};

declare_command!(
    Quitout,
    ClearCount,
    TriggerNewGameCycle,
    FpsCap,
    DisableLogos,
    FreezeWorld,
    MuteMusic,
    DrawHitboxes,
    ShowAllGraces,
    ShowAllMaps,
    StutterFix,
    GameSpeed,
    MapInCombat,
    TravelInDungeons,
    DisableAreaWelcomeMessage,
    DisableRoll,
    DisableJump,
    DisableBackstep,
);

impl UnitCommand for Quitout {
    fn execute(&self) -> anyhow::Result<()> {
        player_loaded_check()?;
        ResolvedPtr::GameMan
            .get()
            .add_offset(game_man::QUITOUT)
            .write::<u8>(0x1)?;
        Ok(())
    }
}

const NG_EVENT_IDS: [u32; 8] = [50, 51, 52, 53, 54, 55, 56, 57];
impl ValueCommand<i32> for ClearCount {
    fn get(&self) -> ProcResult<i32> {
        ResolvedPtr::GameDataMan
            .get()
            .add_offset(game_data_man::NEW_GAME)
            .read::<i32>()
    }
    fn set(&self, val: i32) -> anyhow::Result<()> {
        player_loaded_check()?;
        ResolvedPtr::GameDataMan
            .get()
            .add_offset(game_data_man::NEW_GAME)
            .write::<i32>(val)?;

        let current_ng = ClearCount.get()?.clamp(0, 7);
        NG_EVENT_IDS
            .iter()
            .enumerate()
            .try_for_each(|(i, &id)| event::set_event(id, i == current_ng as usize))
    }
}

impl UnitCommand for TriggerNewGameCycle {
    fn execute(&self) -> anyhow::Result<()> {
        player_loaded_check()?;
        ResolvedPtr::GameMan
            .get()
            .add_offset(game_man::start_new_game())
            .write::<u8>(0x1)?;
        Ok(())
    }
}

impl ValueCommand<f32> for FpsCap {
    fn get(&self) -> ProcResult<f32> {
        read::<f32>(Patch::FpsCap.add_offset(0x3)).map(|val| (1.0_f32 / val).round())
    }
    fn set(&self, val: f32) -> anyhow::Result<()> {
        write::<f32>(Patch::FpsCap.add_offset(0x3), 1.0_f32 / val)?;
        Ok(())
    }
}

impl ToggleCommand for DisableLogos {
    fn is(&self) -> ProcResult<bool> {
        read::<[u8; 2]>(Patch::NoLogo).map(|val| val != [0x74, 0x53])
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        match state {
            false => write_bytes(Patch::NoLogo, &[0x74, 0x53])?,
            true => write_bytes(Patch::NoLogo, &[0x90, 0x90])?,
        }
        Ok(())
    }
}

impl ToggleCommand for FreezeWorld {
    fn is(&self) -> ProcResult<bool> {
        read::<[u8; 2]>(Patch::PauseWorld).map(|val| val != [0x0f, 0x84])
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        match state {
            false => write_bytes(Patch::PauseWorld, &[0x0f, 0x84])?,
            true => write_bytes(Patch::PauseWorld, &[0x0f, 0x85])?,
        }
        Ok(())
    }
}

impl ToggleCommand for MuteMusic {
    fn is(&self) -> ProcResult<bool> {
        read::<[u8; 4]>(Patch::MuteMusic).map(|val| val != [0x0f, 0xb6, 0x48, 0x04])
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        match state {
            false => write_bytes(Patch::MuteMusic, &[0x0f, 0xb6, 0x48, 0x04])?,
            true => write_bytes(Patch::MuteMusic, &[0x31, 0xc9, 0x90, 0x90])?,
        }
        Ok(())
    }
}

impl ToggleCommand for DrawHitboxes {
    fn is(&self) -> ProcResult<bool> {
        Ok(game_state::is_flag(StateFlag::Hitboxes))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::Hitboxes, state)?;
        let _ = self.set_in_game(state);
        Ok(())
    }
}
impl DrawHitboxes {
    pub fn get_in_game(&self) -> ProcResult<bool> {
        ResolvedPtr::DamageManager
            .get()
            .add_offset(damage_manager::HITBOXVIEW_A)
            .read::<u8>()
            .map(|val| val != 0x0)
    }
    pub fn set_in_game(&self, state: bool) -> ProcResult {
        ResolvedPtr::DamageManager
            .get()
            .add_offset(damage_manager::HITBOXVIEW_A)
            .write::<i64>(state as i64)
    }
}

impl ToggleCommand for ShowAllGraces {
    fn is(&self) -> ProcResult<bool> {
        read::<u8>(Data::MapDbgFlags.add_offset(map_dbg_flags::SHOW_ALL_GRACES))
            .map(|val| val != 0x0)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        write::<u8>(Data::MapDbgFlags.add_offset(map_dbg_flags::SHOW_ALL_GRACES), state as u8)?;
        Ok(())
    }
}

impl ToggleCommand for ShowAllMaps {
    fn is(&self) -> ProcResult<bool> {
        read::<u8>(Data::MapDbgFlags.add_offset(map_dbg_flags::SHOW_ALL_MAPS)).map(|val| val != 0x0)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        write::<u8>(Data::MapDbgFlags.add_offset(map_dbg_flags::SHOW_ALL_MAPS), state as u8)?;
        Ok(())
    }
}

impl ToggleCommand for StutterFix {
    fn is(&self) -> ProcResult<bool> {
        Ok(game_state::is_flag(StateFlag::StutterFix))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::StutterFix, state)?;
        let _ = self.set_in_game(state);
        Ok(())
    }
}
impl StutterFix {
    pub fn get_in_game(&self) -> ProcResult<bool> {
        ResolvedPtr::DlUserInputManagerImpl
            .get()
            .add_offset(dl_user_input_manager_impl::STEAM_INPUT)
            .read::<u8>()
            .map(|val| val != 0x0)
    }
    pub fn set_in_game(&self, state: bool) -> ProcResult {
        ResolvedPtr::DlUserInputManagerImpl
            .get()
            .add_offset(dl_user_input_manager_impl::STEAM_INPUT)
            .write::<u8>(state as u8)
    }
}

impl ValueCommand<f32> for GameSpeed {
    fn get(&self) -> ProcResult<f32> {
        ResolvedPtr::CsFlipperImp
            .get()
            .add_offset(cs_flipper_imp::game_speed())
            .read::<f32>()
    }
    fn set(&self, val: f32) -> anyhow::Result<()> {
        ResolvedPtr::CsFlipperImp
            .get()
            .add_offset(cs_flipper_imp::game_speed())
            .write::<f32>(val)?;
        Ok(())
    }
}

impl ToggleCommand for MapInCombat {
    fn is(&self) -> ProcResult<bool> {
        read::<u8>(Patch::OpenMap).map(|val| val != 0x74)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        match state {
            true => {
                write::<u8>(Patch::OpenMap, 0xeb)?;
                write_bytes(Patch::CloseMap, &[0x90; 3])?
            }
            false => {
                write::<u8>(Patch::OpenMap, 0x74)?;
                write_bytes(Patch::CloseMap, &[0xff, 0x50, 0x60])?
            }
        }
        Ok(())
    }
}

impl ToggleCommand for TravelInDungeons {
    fn is(&self) -> ProcResult<bool> {
        read::<[u8; 5]>(Patch::CanFastTravel).map(|val| val != [0x84, 0xc0, 0x0f, 0x94, 0xc0])
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        match state {
            true => write_bytes(Patch::CanFastTravel, &[0xb0, 0x01, 0x90, 0x90, 0x90])?,
            false => write_bytes(Patch::CanFastTravel, &[0x84, 0xc0, 0x0f, 0x94, 0xc0])?,
        }
        Ok(())
    }
}

impl ToggleCommand for DisableAreaWelcomeMessage {
    fn is(&self) -> ProcResult<bool> {
        Ok(game_state::is_flag(StateFlag::TitleCards))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::TitleCards, state)?;
        Ok(())
    }
}

macro_rules! impl_control_toggle {
    ($struct_name:ident, $control_flag:path) => {
        impl ToggleCommand for $struct_name {
            fn is(&self) -> ProcResult<bool> {
                is_control_disabled($control_flag)
            }
            fn set(&self, state: bool) -> anyhow::Result<()> {
                set_control($control_flag, state)
            }
        }
    };
}

impl_control_toggle!(DisableRoll, ControlFlag::Roll);
impl_control_toggle!(DisableJump, ControlFlag::Jump);
impl_control_toggle!(DisableBackstep, ControlFlag::Backstep);

fn install_action_hook() -> ProcResult {
    let location = CaveAddress::ActionHook;

    let mut fun = ASM.get_function("action_hook");
    let mut asm = fun.take_bytes();

    write_rel_i32(&mut asm, location, fun.reloc("roll_flag"), CaveAddress::DisableRollFlag, 5)?;
    write_rel_i32(&mut asm, location, fun.reloc("jump_flag"), CaveAddress::DisableJumpFlag, 5)?;
    write_rel_i32(
        &mut asm,
        location,
        fun.reloc("backstep_flag"),
        CaveAddress::DisableBackstepFlag,
        5,
    )?;

    install_hook(&asm, location, Hook::SetRequestedAction, 5)
}

#[derive(Clone, Copy)]
enum ControlFlag {
    Roll,
    Jump,
    Backstep,
}

impl Address for ControlFlag {
    fn addr(&self) -> u64 {
        match self {
            Self::Roll => CaveAddress::DisableRollFlag.addr(),
            Self::Jump => CaveAddress::DisableJumpFlag.addr(),
            Self::Backstep => CaveAddress::DisableBackstepFlag.addr(),
        }
    }
}

fn is_control_disabled(flag: ControlFlag) -> ProcResult<bool> {
    read::<u8>(flag).map(|val| val != 0x0)
}

fn set_control(flag: ControlFlag, state: bool) -> anyhow::Result<()> {
    if state {
        install_action_hook()?;
    }
    write::<u8>(flag, state as u8)?;
    Ok(())
}
