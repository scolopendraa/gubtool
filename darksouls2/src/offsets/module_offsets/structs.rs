use {
    crate::offsets::module_offsets::versions,
    gubtool_core::{attached::version, game_version::DarkSouls2Version::*},
};

#[derive(Debug)]
pub struct ModuleOffsets {
    pub base_ptrs:        BasePointers,
    pub functions:        Functions,
    pub hooks:            Hooks,
    pub patches:          Patches,
    pub data:             Data,
    pub external_fn_ptrs: ExternalFunctionPointers,
}

#[derive(Debug)]
pub struct BasePointers {
    pub game_manager_imp: u64,
    pub katana_main_app:  u64,
}

#[derive(Debug)]
pub struct Functions {
    pub give_souls:                        u64,
    pub warp:                              u64,
    pub item_spawn:                        u64,
    pub build_item_dialogue:               u64,
    pub show_item_dialogue:                u64,
    pub current_item_quantity_check:       u64,
    pub set_event:                         u64,
    pub map_entity_from_map_id_and_obj_id: u64,
    pub get_state_act_component:           u64,
    pub make_sound:                        u64,
    pub bonfire_rest:                      u64,
    pub bonfire_unlock:                    u64,
    pub open_menu:                         u64,
    pub menu_chr_state:                    u64,
    pub level_up:                          u64,
    pub level_lookup:                      u64,
    pub chr_set_action:                    u64,
    pub ez_state_external_event_ctor:      u64,
    pub ez_state_execute_event:            u64,
}

#[derive(Debug)]
pub struct Hooks {
    pub set_shared_flag:       u64,
    pub locked_target_pointer: u64,
    pub credits_skip:          u64,
    pub faster_menu:           u64,
    pub event_log:             u64,
    pub player_no_damage:      u64,
    pub infinite_poise:        u64,
}

#[derive(Debug)]
pub struct Patches {
    pub infinite_stamina:     u64,
    pub infinite_consumables: u64,
    pub infinite_durability:  u64,
    pub infinite_casts:       u64,
    pub no_soul_gain:         u64,
    pub no_hollowing:         u64,
    pub no_soul_loss:         u64,
    pub player_hidden:        u64,
    pub player_silent:        u64,
    pub skip_logos:           u64,
    pub menu_transition:      u64,
    pub no_roll:              u64,
    pub no_backstep:          u64,
}

#[derive(Debug)]
pub struct Data {
    pub map_id: u64,
}

#[derive(Debug)]
pub struct ExternalFunctionPointers {
    pub kernel32_create_thread:  u64,
    pub kernel32_close_handle:   u64,
    pub kernel32_sleep:          u64,
    pub kernel32_load_library_w: u64,
}

#[inline(always)]
pub(super) fn module_offsets() -> &'static ModuleOffsets {
    match version() {
        Some(Vanilla1_0_10) => &versions::VANILLA_1_0_10,
        Some(Vanilla1_0_11) => &versions::VANILLA_1_0_11,
        Some(Vanilla1_0_12) => &versions::VANILLA_1_0_12,
        Some(Scholar1_0_1) => &versions::SCHOLAR_1_0_1,
        Some(Scholar1_0_2) => &versions::SCHOLAR_1_0_2,
        Some(Scholar1_0_3) => &versions::SCHOLAR_1_0_3,
        _ => &versions::SCHOLAR_1_0_3,
    }
}
