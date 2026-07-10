use crate::{offsets::module_offsets::versions, resources::scan_patterns};
use anyhow::Result;
use gubtool_core::{aob_scanner, attached::version, game_version::EldenRingVersion::*};

#[derive(Debug)]
pub struct ModuleOffsets {
    pub base_ptrs: BasePointers,
    pub functions: Functions,
    pub hooks: Hooks,
    pub patches: Patches,
    pub data: Data,
    pub external_fn_ptrs: ExternalFunctionPointers,
}

#[derive(Debug)]
pub struct BasePointers {
    pub world_chr_man: u64,
    pub field_area: u64,
    pub game_man: u64,
    pub game_data_man: u64,
    pub menu_man: u64,
    pub cs_emk_system: u64,
    pub virtual_mem_flag: u64,
    pub damage_manager: u64,
    pub map_item_man_impl: u64,
    pub dl_user_input_manager_impl: u64,
    pub cs_flipper_imp: u64,
    pub cs_dlc_imp: u64,
    pub cs_trophy: u64,
}

#[derive(Debug)]
pub struct Functions {
    pub grace_warp: u64,
    pub block_warp: u64,
    pub get_player_item_quantity_by_id: u64,
    pub item_spawn: u64,
    pub give_runes: u64,
    pub get_event: u64,
    pub set_event: u64,
    pub set_speffect: u64,
    pub remove_speffect: u64,
    pub get_chr_ins_by_entity_id: u64,
    pub emevd_switch: u64,
    pub emk_event_ins_ctor: u64,
    pub external_event_temp_ctor: u64,
    pub execute_talk_command: u64,
}

#[derive(Debug)]
pub struct Hooks {
    pub locked_target_pointer: u64,
    pub target_no_stagger: u64,
    pub player_no_grab: u64,
    pub player_infinite_poise: u64,
    pub warp_coord_write: u64,
    pub warp_angle_write: u64,
    pub get_force_act_idx: u64,
    pub set_requested_action: u64,
}

#[derive(Debug)]
pub struct Patches {
    pub no_logo: u64,
    pub fps_cap: u64,
    pub mute_music: u64,
    pub pause_world: u64,
    pub torrent_disabled_underworld: u64,
    pub whistle_disabled: u64,
    pub open_map: u64,
    pub close_map: u64,
    pub can_fast_travel: u64,
    pub no_rune_loss_on_death: u64,
}

#[derive(Debug)]
pub struct Data {
    pub chr_dbg_flags: u64,
    pub map_dbg_flags: u64,
}

#[derive(Debug)]
pub struct ExternalFunctionPointers {
    pub kernel32_create_thread: u64,
    pub kernel32_close_handle: u64,
}

pub(super) fn module_offsets() -> &'static ModuleOffsets {
    match version() {
        Some(Version1_2_0) => &versions::OFFSETS_1_2_0,
        Some(Version1_2_1) => &versions::OFFSETS_1_2_1,
        Some(Version1_2_2) => &versions::OFFSETS_1_2_2,
        Some(Version1_2_3) => &versions::OFFSETS_1_2_3,
        Some(Version1_3_0) => &versions::OFFSETS_1_3_0,
        Some(Version1_3_1) => &versions::OFFSETS_1_3_1,
        Some(Version1_3_2) => &versions::OFFSETS_1_3_2,
        Some(Version1_4_0) => &versions::OFFSETS_1_4_0,
        Some(Version1_4_1) => &versions::OFFSETS_1_4_1,
        Some(Version1_5_0) => &versions::OFFSETS_1_5_0,
        Some(Version1_6_0) => &versions::OFFSETS_1_6_0,
        Some(Version1_7_0) => &versions::OFFSETS_1_7_0,
        Some(Version1_8_0) => &versions::OFFSETS_1_8_0,
        Some(Version1_8_1) => &versions::OFFSETS_1_8_1,
        Some(Version1_9_0) => &versions::OFFSETS_1_9_0,
        Some(Version1_9_1) => &versions::OFFSETS_1_9_1,
        Some(Version2_0_0) => &versions::OFFSETS_2_0_0,
        Some(Version2_0_1) => &versions::OFFSETS_2_0_1,
        Some(Version2_2_0) => &versions::OFFSETS_2_2_0,
        Some(Version2_2_3) => &versions::OFFSETS_2_2_3,
        Some(Version2_3_0) => &versions::OFFSETS_2_3_0,
        Some(Version2_4_0) => &versions::OFFSETS_2_4_0,
        Some(Version2_5_0) => &versions::OFFSETS_2_5_0,
        Some(Version2_6_0) => &versions::OFFSETS_2_6_0,
        Some(Version2_6_1) => &versions::OFFSETS_2_6_1,
        Some(Version2_6_2) => &versions::OFFSETS_2_6_2,
        _ => &versions::OFFSETS_2_6_2,
    }
}

pub fn scan() -> Result<ModuleOffsets> {
    Ok(ModuleOffsets {
        base_ptrs: BasePointers {
            world_chr_man: aob_scanner::scan(scan_patterns::WORLD_CHR_MAN)?,
            field_area: aob_scanner::scan(scan_patterns::FIELD_AREA)?,
            game_man: aob_scanner::scan(scan_patterns::GAME_MAN)?,
            game_data_man: aob_scanner::scan(scan_patterns::GAME_DATA_MAN)?,
            menu_man: aob_scanner::scan(scan_patterns::MENU_MAN)?,
            cs_emk_system: aob_scanner::scan(scan_patterns::CS_EMK_SYSTEM)?,
            virtual_mem_flag: aob_scanner::scan(scan_patterns::VIRTUAL_MEMORY_FLAG)?,
            damage_manager: aob_scanner::scan(scan_patterns::DAMAGE_MANAGER)?,
            map_item_man_impl: aob_scanner::scan(scan_patterns::MAP_ITEM_MAN_IMPL)?,
            dl_user_input_manager_impl: aob_scanner::scan(scan_patterns::DL_USER_INPUT_MANAGER_IMPL)?,
            cs_flipper_imp: aob_scanner::scan(scan_patterns::CS_FLIPPER_IMP)?,
            cs_dlc_imp: aob_scanner::scan(scan_patterns::CS_DLC_IMP)?,
            cs_trophy: 0, // Resolved via version-specific offsets in module_offsets()
        },
        functions: Functions {
            grace_warp: aob_scanner::scan(scan_patterns::GRACE_WARP)?,
            block_warp: aob_scanner::scan(scan_patterns::BLOCK_WARP)?,
            get_player_item_quantity_by_id: aob_scanner::scan(scan_patterns::GET_PLAYER_ITEM_QUANTITY_BY_ID)?,
            item_spawn: aob_scanner::scan(scan_patterns::ITEM_SPAWN)?,
            give_runes: aob_scanner::scan(scan_patterns::GIVE_RUNES)?,
            get_event: aob_scanner::scan(scan_patterns::GET_EVENT)?,
            set_event: aob_scanner::scan(scan_patterns::SET_EVENT)?,
            set_speffect: aob_scanner::scan(scan_patterns::SET_SPEFFECT)?,
            remove_speffect: aob_scanner::scan(scan_patterns::REMOVE_SPEFFECT)?,
            get_chr_ins_by_entity_id: aob_scanner::scan(scan_patterns::GET_CHR_INS_BY_ENTITY_ID)?,
            emevd_switch: aob_scanner::scan(scan_patterns::EMEVD_SWITCH)?,
            emk_event_ins_ctor: aob_scanner::scan(scan_patterns::EMK_EVENT_INS_CTOR)?,
            external_event_temp_ctor: aob_scanner::scan(scan_patterns::EXTERNAL_EVENT_TEMP_CTOR)?,
            execute_talk_command: aob_scanner::scan(scan_patterns::EXECUTE_TALK_COMMAND)?,
        },
        hooks: Hooks {
            locked_target_pointer: aob_scanner::scan(scan_patterns::LOCKED_TARGET_POINTER)?,
            target_no_stagger: aob_scanner::scan(scan_patterns::TARGET_NO_STAGGER)?,
            player_no_grab: aob_scanner::scan(scan_patterns::PLAYER_NO_GRAB)?,
            player_infinite_poise: aob_scanner::scan(scan_patterns::PLAYER_INFINITE_POISE)?,
            warp_coord_write: aob_scanner::scan(scan_patterns::WARP_COORD_WRITE)?,
            warp_angle_write: aob_scanner::scan(scan_patterns::WARP_ANGLE_WRITE)?,
            get_force_act_idx: aob_scanner::scan(scan_patterns::GET_FORCE_ACT_IDX)?,
            set_requested_action: aob_scanner::scan(scan_patterns::SET_REQUESTED_ACTION)?,
        },
        patches: Patches {
            no_logo: aob_scanner::scan(scan_patterns::NO_LOGO)?,
            fps_cap: aob_scanner::scan(scan_patterns::FPS_CAP)?,
            mute_music: aob_scanner::scan(scan_patterns::MUTE_MUSIC)?,
            pause_world: aob_scanner::scan(scan_patterns::PAUSE_WORLD)?,
            torrent_disabled_underworld: aob_scanner::scan(scan_patterns::TORRENT_DISABLED_UNDERWOLD)?,
            whistle_disabled: aob_scanner::scan(scan_patterns::WHISTLE_DISABLED)?,
            open_map: aob_scanner::scan(scan_patterns::OPEN_MAP)?,
            close_map: aob_scanner::scan(scan_patterns::CLOSE_MAP)?,
            can_fast_travel: aob_scanner::scan(scan_patterns::CAN_FAST_TRAVEL)?,
            no_rune_loss_on_death: 0,
        },
        data: Data {
            chr_dbg_flags: aob_scanner::scan(scan_patterns::CHR_DBG_FLAGS)?,
            map_dbg_flags: aob_scanner::scan(scan_patterns::MAP_DBG_FLAGS)?,
        },
        external_fn_ptrs: ExternalFunctionPointers {
            kernel32_create_thread: aob_scanner::scan(scan_patterns::KERNEL32_CREATE_THREAD)?,
            kernel32_close_handle: aob_scanner::scan(scan_patterns::KERNEL32_CLOSE_HANDLE)?,
        },
    })
}