use {
    crate::offsets::module_offsets::versions,
    gubtool_core::{attached::version, game_version::DarkSouls3Version::*},
};

#[derive(Debug)]
pub struct ModuleOffsets {
    pub kernel32_create_thread:       u64,
    pub kernel32_close_handle:        u64,
    pub kernel32_load_library_w:      u64,
    pub world_obj_man_impl:           u64,
    pub world_chr_man_imp:            u64,
    pub game_man:                     u64,
    pub game_data_man:                u64,
    pub menu_man:                     u64,
    pub map_item_man:                 u64,
    pub event_flag_man:               u64,
    pub world_ai_manager:             u64,
    pub debug_flags:                  u64,
    pub set_event_flag:               u64,
    pub cs_dlc:                       u64,
    pub bonfire_warp:                 u64,
    pub warp_coords:                  u64,
    pub warp_angle:                   u64,
    pub disable_logos:                u64,
    pub game_speed:                   u64,
    pub sprj_flipper_imp:             u64,
    pub get_item_quantity:            u64,
    pub item_spawn:                   u64,
    pub camera_reset:                 u64,
    pub camera_rotation:              u64,
    pub ez_state_external_event_ctor: u64,
    pub execute_talk_command:         u64,
    pub full_shop:                    u64,
    pub locked_target_pointer:        u64,
}

#[inline(always)]
pub(super) fn module_offsets() -> &'static ModuleOffsets {
    match version() {
        Some(Version1_1_1) => &versions::OFFSETS_1_1_1,
        Some(Version1_3_1) => &versions::OFFSETS_1_3_1,
        Some(Version1_3_2) => &versions::OFFSETS_1_3_2,
        Some(Version1_4_1) => &versions::OFFSETS_1_4_1,
        Some(Version1_4_2) => &versions::OFFSETS_1_4_2,
        Some(Version1_4_3) => &versions::OFFSETS_1_4_3,
        Some(Version1_5_0) => &versions::OFFSETS_1_5_0,
        Some(Version1_5_1) => &versions::OFFSETS_1_5_1,
        Some(Version1_6_0) => &versions::OFFSETS_1_6_0,
        Some(Version1_7_0) => &versions::OFFSETS_1_7_0,
        Some(Version1_8_0) => &versions::OFFSETS_1_8_0,
        Some(Version1_9_0) => &versions::OFFSETS_1_9_0,
        Some(Version1_10_0) => &versions::OFFSETS_1_10_0,
        Some(Version1_11_0) => &versions::OFFSETS_1_11_0,
        Some(Version1_12_0) => &versions::OFFSETS_1_12_0,
        Some(Version1_13_0) => &versions::OFFSETS_1_13_0,
        Some(Version1_14_0) => &versions::OFFSETS_1_14_0,
        Some(Version1_15_0) => &versions::OFFSETS_1_15_0,
        Some(Version1_15_1) => &versions::OFFSETS_1_15_1,
        Some(Version1_15_2) => &versions::OFFSETS_1_15_2,
        _ => &versions::OFFSETS_1_15_2,
    }
}
