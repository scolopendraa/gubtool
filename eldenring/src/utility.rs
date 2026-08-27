use {
    crate::{
        event,
        game_state::{self, StateFlag},
        mem::*,
        offsets::{
            ChainReadExt,
            code_cave::CaveAddr,
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
    gubtool_core::{
        address::{Address, POINTER},
        sys::sys_error::SysResult,
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
    Ok(ResolvedPtr::GameMan
        .get()
        .add_offset(game_man::QUITOUT)
        .write::<u8>(0x1)?)
});

const NG_EVENT_IDS: [u32; 8] = [50, 51, 52, 53, 54, 55, 56, 57];
value_command!(ClearCount, i32 {
    get: {
        ResolvedPtr::GameDataMan
            .get()
            .add_offset(game_data_man::NEW_GAME)
            .read::<i32>()
    }

    set(val): {
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
});

unit_command!(TriggerNewGame {
    player_loaded_check()?;
    Ok(ResolvedPtr::GameMan
        .get()
        .add_offset(game_man::start_new_game())
        .write::<u8>(0x1)?)
});

value_command!(FpsCap, f32 {
    get: {
        read::<f32>(Patch::FpsCap.add(0x3))
            .map(|val| (1.0_f32 / val).round())
    }

    set(val): {
        Ok(write::<f32>(Patch::FpsCap.add(0x3), 1.0_f32 / val)?)
    }
});

toggle_command!(DisableLogos {
    is: {
        read::<[u8; 2]>(Patch::NoLogo)
            .map(|val| val != [0x74, 0x53])
    }

    set(state): {
        match state {
            false => write_bytes(Patch::NoLogo, &[0x74, 0x53])?,
            true => write_bytes(Patch::NoLogo, &[0x90, 0x90])?,
        }
        Ok(())
    }
});

toggle_command!(FreezeWorld {
    is: {
        read::<[u8; 2]>(Patch::PauseWorld)
            .map(|val| val != [0x0f, 0x84])
    }

    set(state): {
        match state {
            false => write_bytes(Patch::PauseWorld, &[0x0f, 0x84])?,
            true => write_bytes(Patch::PauseWorld, &[0x0f, 0x85])?,
        }
        Ok(())
    }
});

toggle_command!(MuteMusic {
    is: {
        read::<[u8; 4]>(Patch::MuteMusic)
            .map(|val| val != [0x0f, 0xb6, 0x48, 0x04])
    }

    set(state): {
        match state {
            false => write_bytes(Patch::MuteMusic, &[0x0f, 0xb6, 0x48, 0x04])?,
            true => write_bytes(Patch::MuteMusic, &[0x31, 0xc9, 0x90, 0x90])?,
        }
        Ok(())
    }
});

toggle_command!(DrawHitboxes {
    is: {
        Ok(game_state::is_flag(StateFlag::Hitboxes))
    }

    set(state): {
        game_state::set_flag(StateFlag::Hitboxes, state)?;
        let _ = DrawHitboxes.set_in_game(state);
        Ok(())
    }
});

impl DrawHitboxes {
    pub fn get_in_game(&self) -> SysResult<bool> {
        ResolvedPtr::DamageManager
            .get()
            .add_offset(damage_manager::HITBOXVIEW_A)
            .read::<u8>()
            .map(|val| val != 0x0)
    }
    pub fn set_in_game(&self, state: bool) -> SysResult {
        ResolvedPtr::DamageManager
            .get()
            .add_offset(damage_manager::HITBOXVIEW_A)
            .write::<i64>(state as i64)
    }
}

toggle_command!(ShowAllGraces {
    is: {
        read::<u8>(Data::MapDbgFlags.add(map_dbg_flags::SHOW_ALL_GRACES))
            .map(|val| val != 0x0)
    }

    set(state): {
        Ok(write::<u8>(Data::MapDbgFlags.add(map_dbg_flags::SHOW_ALL_GRACES), state as u8)?)
    }
});

toggle_command!(ShowAllMaps {
    is: {
        read::<u8>(Data::MapDbgFlags.add(map_dbg_flags::SHOW_ALL_MAPS))
            .map(|val| val != 0x0)
    }

    set(state): {
        Ok(write::<u8>(Data::MapDbgFlags.add(map_dbg_flags::SHOW_ALL_MAPS), state as u8)?)
    }
});

toggle_command!(StutterFix {
    is: {
        Ok(game_state::is_flag(StateFlag::StutterFix))
    }

    set(state): {
        game_state::set_flag(StateFlag::StutterFix, state)?;
        let _ = StutterFix.set_in_game(state);
        Ok(())
    }
});

impl StutterFix {
    pub fn get_in_game(&self) -> SysResult<bool> {
        ResolvedPtr::DlUserInputManagerImpl
            .get()
            .add_offset(dl_user_input_manager_impl::STEAM_INPUT)
            .read::<u8>()
            .map(|val| val != 0x0)
    }
    pub fn set_in_game(&self, state: bool) -> SysResult {
        ResolvedPtr::DlUserInputManagerImpl
            .get()
            .add_offset(dl_user_input_manager_impl::STEAM_INPUT)
            .write::<u8>(state as u8)
    }
}

value_command!(GameSpeed, f32 {
    get: {
        ResolvedPtr::CsFlipperImp
            .get()
            .add_offset(cs_flipper_imp::game_speed())
            .read::<f32>()
    }

    set(val): {
        Ok(ResolvedPtr::CsFlipperImp
            .get()
            .add_offset(cs_flipper_imp::game_speed())
            .write::<f32>(val)?)
    }
});

toggle_command!(MapInCombat {
    is: {
        read::<u8>(Patch::OpenMap).map(|val| val != 0x74)
    }

    set(state): {
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
});

toggle_command!(TravelInDungeons {
    is: {
        read::<[u8; 5]>(Patch::CanFastTravel)
            .map(|val| val != [0x84, 0xc0, 0x0f, 0x94, 0xc0])
    }

    set(state): {
        match state {
            true => write_bytes(Patch::CanFastTravel, &[0xb0, 0x01, 0x90, 0x90, 0x90])?,
            false => write_bytes(Patch::CanFastTravel, &[0x84, 0xc0, 0x0f, 0x94, 0xc0])?,
        }
        Ok(())
    }
});

const AREA_MESSAGE_BYTES: [u8; 3] = [0x48, 0x8b, 0xcb];
toggle_command!(DisableAreaWelcomeMessage {
    is: {
        read::<[u8; 3]>(Patch::DisableAreaWelcomeMessage)
            .map(|val| val != AREA_MESSAGE_BYTES)
    }

    set(state): {
        let bytes = if state { [0xeb, 0x6, 0x90] } else { AREA_MESSAGE_BYTES };
        Ok(write::<[u8; 3]>(Patch::DisableAreaWelcomeMessage, bytes)?)
    }
});

macro_rules! impl_control_toggle {
    ($struct_name:ident, $control_flag:path) => {
    toggle_command!($struct_name {
            is: {
                is_control_disabled($control_flag)
            }

            set(state): {
                set_control($control_flag, state)
            }
        });
    };
}

impl_control_toggle!(DisableRoll, ControlFlag::Roll);
impl_control_toggle!(DisableJump, ControlFlag::Jump);
impl_control_toggle!(DisableBackstep, ControlFlag::Backstep);

fn install_action_hook() -> SysResult {
    let location = CaveAddr::ActionHook;

    let mut fun = ASM.get_function("action_hook");

    fun.patch::<POINTER>("roll_flag", CaveAddr::DisableRollFlag);
    fun.patch::<POINTER>("jump_flag", CaveAddr::DisableJumpFlag);
    fun.patch::<POINTER>("backstep_flag", CaveAddr::DisableBackstepFlag);

    install_hook(&fun.bytes, location, Hook::SetRequestedAction, 5)
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
            Self::Roll => CaveAddr::DisableRollFlag.addr(),
            Self::Jump => CaveAddr::DisableJumpFlag.addr(),
            Self::Backstep => CaveAddr::DisableBackstepFlag.addr(),
        }
    }
}

fn is_control_disabled(flag: ControlFlag) -> SysResult<bool> {
    read::<u8>(flag).map(|val| val != 0x0)
}

fn set_control(flag: ControlFlag, state: bool) -> anyhow::Result<()> {
    if state {
        install_action_hook()?;
    }
    Ok(write::<u8>(flag, state as u8)?)
}
