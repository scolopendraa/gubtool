pub mod scan;
mod structs;
mod versions;

use {
    crate::offsets::module_offsets::structs::module_offsets,
    gubtool_core::{address::Address, attached::module_base, impl_address_patch},
};

#[derive(Clone, Copy)]
pub enum BasePointer {
    WorldChrMan,
    FieldArea,
    GameMan,
    GameDataMan,
    MenuMan,
    CsEmkSystem,
    VirtualMemFlag,
    DamageManager,
    WorldAreaTimeImpl,
    MapItemManImpl,
    DlUserInputManagerImpl,
    CsFlipperImp,
    CsDlcImp,
    LockTgtManImp,
}

#[derive(Clone, Copy)]
pub enum Function {
    GraceWarp,
    BlockWarp,
    GetPlayerItemQuantityById,
    ItemSpawn,
    GiveRunes,
    GetEvent,
    SetEvent,
    SetSpeffect,
    RemoveSpeffect,
    GetChrInsByEntityId,
    EmevdSwitch,
    EmkEventInsCtor,
    ExternalEventTempCtor,
    ExecuteTalkCommand,
    AddCoolTime,
}

#[derive(Clone, Copy)]
pub enum Hook {
    SaveTarget,
    TargetStagger,
    PlayerNoGrab,
    PlayerInfinitePoise,
    WarpCoordWrite,
    WarpAngleWrite,
    GetForceActIdx,
    SetRequestedAction,
    NoTimePassOnDeath,
}

#[derive(Clone, Copy)]
pub enum Patch {
    NoLogo,
    FpsCap,
    MuteMusic,
    PauseWorld,
    TorrentDisabledUnderworld,
    WhistleDisabled,
    OpenMap,
    CloseMap,
    CanFastTravel,
    NoRuneLossOnDeath,
}

#[derive(Clone, Copy)]
pub enum Data {
    ChrDbgFlags,
    MapDbgFlags,
}

#[derive(Clone, Copy)]
pub enum ExternalFunctionPointer {
    Kernel32CreateThread,
    Kernel32CloseHandle,
    Kernel32LoadLibraryW,
}

impl Address for BasePointer {
    fn addr(&self) -> u64 {
        let f = &module_offsets().base_ptrs;
        let offset = match self {
            Self::WorldChrMan => f.world_chr_man,
            Self::FieldArea => f.field_area,
            Self::GameMan => f.game_man,
            Self::GameDataMan => f.game_data_man,
            Self::MenuMan => f.menu_man,
            Self::WorldAreaTimeImpl => f.world_area_time_impl,
            Self::CsEmkSystem => f.cs_emk_system,
            Self::VirtualMemFlag => f.virtual_mem_flag,
            Self::DamageManager => f.damage_manager,
            Self::MapItemManImpl => f.map_item_man_impl,
            Self::DlUserInputManagerImpl => f.dl_user_input_manager_impl,
            Self::CsFlipperImp => f.cs_flipper_imp,
            Self::CsDlcImp => f.cs_dlc_imp,
            Self::LockTgtManImp => f.lock_tgt_man_imp,
        };
        module_base() + offset
    }
}

impl Address for Function {
    fn addr(&self) -> u64 {
        let f = &module_offsets().functions;
        let offset = match self {
            Self::GraceWarp => f.grace_warp,
            Self::BlockWarp => f.block_warp,
            Self::GetPlayerItemQuantityById => f.get_player_item_quantity_by_id,
            Self::ItemSpawn => f.item_spawn,
            Self::GiveRunes => f.give_runes,
            Self::GetEvent => f.get_event,
            Self::SetEvent => f.set_event,
            Self::SetSpeffect => f.set_speffect,
            Self::RemoveSpeffect => f.remove_speffect,
            Self::GetChrInsByEntityId => f.get_chr_ins_by_entity_id,
            Self::EmevdSwitch => f.emevd_switch,
            Self::EmkEventInsCtor => f.emk_event_ins_ctor,
            Self::ExternalEventTempCtor => f.external_event_temp_ctor,
            Self::ExecuteTalkCommand => f.execute_talk_command,
            Self::AddCoolTime => f.add_cool_time,
        };
        module_base() + offset
    }
}

impl Address for Hook {
    fn addr(&self) -> u64 {
        let f = &module_offsets().hooks;
        let offset = match self {
            Self::SaveTarget => f.locked_target_pointer,
            Self::TargetStagger => f.target_no_stagger,
            Self::PlayerNoGrab => f.player_no_grab,
            Self::PlayerInfinitePoise => f.player_infinite_poise,
            Self::WarpCoordWrite => f.warp_coord_write,
            Self::WarpAngleWrite => f.warp_angle_write,
            Self::GetForceActIdx => f.get_force_act_idx,
            Self::SetRequestedAction => f.set_requested_action,
            Self::NoTimePassOnDeath => f.no_time_pass_on_death,
        };
        module_base() + offset
    }
}

impl Address for Patch {
    fn addr(&self) -> u64 {
        let f = &module_offsets().patches;
        let offset = match self {
            Self::NoLogo => f.no_logo,
            Self::FpsCap => f.fps_cap,
            Self::MuteMusic => f.mute_music,
            Self::PauseWorld => f.pause_world,
            Self::TorrentDisabledUnderworld => f.torrent_disabled_underworld,
            Self::WhistleDisabled => f.whistle_disabled,
            Self::OpenMap => f.open_map,
            Self::CloseMap => f.close_map,
            Self::CanFastTravel => f.can_fast_travel,
            Self::NoRuneLossOnDeath => f.no_rune_loss_on_death,
        };
        module_base() + offset
    }
}

impl Address for Data {
    fn addr(&self) -> u64 {
        let f = &module_offsets().data;
        let offset = match self {
            Self::ChrDbgFlags => f.chr_dbg_flags,
            Self::MapDbgFlags => f.map_dbg_flags,
        };
        module_base() + offset
    }
}

impl Address for ExternalFunctionPointer {
    fn addr(&self) -> u64 {
        let f = &module_offsets().external_fn_ptrs;
        let offset = match self {
            Self::Kernel32CreateThread => f.kernel32_create_thread,
            Self::Kernel32CloseHandle => f.kernel32_close_handle,
            Self::Kernel32LoadLibraryW => f.kernel32_load_library_w,
        };
        module_base() + offset
    }
}

impl_address_patch!(BasePointer, Function, Hook, Patch, Data, ExternalFunctionPointer);
