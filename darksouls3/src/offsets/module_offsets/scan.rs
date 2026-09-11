use {
    crate::{offsets::module_offsets::r#struct::ModuleOffsets, resources::scan_patterns::*},
    gubtool_core::{
        aob_scanner::{ScanStrategy, scan_error::ScanError},
        parallel_scan,
    },
    std::path::PathBuf,
};

fn scan(strategy: ScanStrategy) -> Result<ModuleOffsets, ScanError> {
    parallel_scan!(
        strategy,
        {
            kernel32_create_thread: KERNEL32_CREATE_THREAD,
            kernel32_close_handle: KERNEL32_CLOSE_HANDLE,
            kernel32_load_library_w: KERNEL32_LOAD_LIBRARY_W,
            world_obj_man_impl: WORLD_OBJ_MAN_IMPL,
            world_chr_man_imp: WORLD_CHR_MAN_IMP,
            game_man: GAME_MAN,
            game_data_man: GAME_DATA_MAN,
            menu_man: MENU_MAN,
            map_item_man: MAP_ITEM_MAN,
            event_flag_man: EVENT_FLAG_MAN,
            world_ai_manager: WORLD_AI_MANAGER,
            debug_flags: DEBUG_FLAGS,
            set_event_flag: SET_EVENT_FLAG,
            cs_dlc: CS_DLC,
            bonfire_warp: BONFIRE_WARP,
            warp_coords: WARP_COORDS,
            warp_angle: WARP_ANGLE,
            disable_logos: DISABLE_LOGOS,
            game_speed: GAME_SPEED,
            sprj_flipper_imp: SPRJ_FLIPPER_IMP,
            get_item_quantity: GET_ITEM_QUANTITY,
            item_spawn: ITEM_SPAWN,
            camera_reset: CAMERA_RESET,
            camera_rotation: CAMERA_ROTATION,
            ez_state_external_event_ctor: EZ_STATE_EXTERNAL_EVENT_CTOR,
            execute_talk_command: EXECUTE_TALK_COMMAND,
            full_shop: FULL_SHOP,
            locked_target_pointer: LOCKED_TARGET_POINTER,
        }
    );

    let offsets = ModuleOffsets {
        kernel32_create_thread,
        kernel32_close_handle,
        kernel32_load_library_w,
        world_obj_man_impl,
        world_chr_man_imp,
        game_man,
        game_data_man,
        menu_man,
        map_item_man,
        event_flag_man,
        world_ai_manager,
        debug_flags,
        set_event_flag,
        cs_dlc,
        bonfire_warp,
        warp_coords,
        warp_angle,
        disable_logos,
        game_speed,
        sprj_flipper_imp,
        get_item_quantity,
        item_spawn,
        camera_reset,
        camera_rotation,
        ez_state_external_event_ctor,
        execute_talk_command,
        full_shop,
        locked_target_pointer,
    };

    Ok(offsets)
}

pub fn scan_mem() -> Result<ModuleOffsets, ScanError> {
    let strategy = ScanStrategy::Mem;
    scan(strategy)
}

pub fn scan_mem_exhaustive() -> Result<ModuleOffsets, ScanError> {
    let strategy = ScanStrategy::MemExhaustive;
    scan(strategy)
}

pub fn scan_disk<T>(path: T) -> Result<ModuleOffsets, ScanError>
where T: Into<PathBuf> {
    let strategy = ScanStrategy::Disk(&path.into());
    scan(strategy)
}

pub fn scan_disk_exhaustive<T>(path: T) -> Result<ModuleOffsets, ScanError>
where T: Into<PathBuf> {
    let strategy = ScanStrategy::DiskExhaustive(&path.into());
    scan(strategy)
}
