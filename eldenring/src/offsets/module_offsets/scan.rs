use {
    crate::{
        offsets::module_offsets::structs::{
            BasePointers,
            Data,
            ExternalFunctionPointers,
            Functions,
            Hooks,
            ModuleOffsets,
            Patches,
        },
        resources::scan_patterns::*,
    },
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
            world_chr_man: WORLD_CHR_MAN,
            field_area: FIELD_AREA,
            game_man: GAME_MAN,
            game_data_man: GAME_DATA_MAN,
            menu_man: MENU_MAN,
            cs_emk_system: CS_EMK_SYSTEM,
            virtual_mem_flag: VIRTUAL_MEMORY_FLAG,
            damage_manager: DAMAGE_MANAGER,
            map_item_man_impl: MAP_ITEM_MAN_IMPL,
            dl_user_input_manager_impl: DL_USER_INPUT_MANAGER_IMPL,
            cs_flipper_imp: CS_FLIPPER_IMP,
            cs_dlc_imp: CS_DLC_IMP,

            grace_warp: GRACE_WARP,
            block_warp: BLOCK_WARP,
            get_player_item_quantity_by_id: GET_PLAYER_ITEM_QUANTITY_BY_ID,
            item_spawn: ITEM_SPAWN,
            give_runes: GIVE_RUNES,
            get_event: GET_EVENT,
            set_event: SET_EVENT,
            set_speffect: SET_SPEFFECT,
            remove_speffect: REMOVE_SPEFFECT,
            get_chr_ins_by_entity_id: GET_CHR_INS_BY_ENTITY_ID,
            emevd_switch: EMEVD_SWITCH,
            emk_event_ins_ctor: EMK_EVENT_INS_CTOR,
            external_event_temp_ctor: EXTERNAL_EVENT_TEMP_CTOR,
            execute_talk_command: EXECUTE_TALK_COMMAND,

            locked_target_pointer: LOCKED_TARGET_POINTER,
            target_no_stagger: TARGET_NO_STAGGER,
            player_no_grab: PLAYER_NO_GRAB,
            player_infinite_poise: PLAYER_INFINITE_POISE,
            warp_coord_write: WARP_COORD_WRITE,
            warp_angle_write: WARP_ANGLE_WRITE,
            get_force_act_idx: GET_FORCE_ACT_IDX,
            set_requested_action: SET_REQUESTED_ACTION,

            no_logo: NO_LOGO,
            fps_cap: FPS_CAP,
            mute_music: MUTE_MUSIC,
            pause_world: PAUSE_WORLD,
            torrent_disabled_underworld: TORRENT_DISABLED_UNDERWOLD,
            whistle_disabled: WHISTLE_DISABLED,
            open_map: OPEN_MAP,
            close_map: CLOSE_MAP,
            can_fast_travel: CAN_FAST_TRAVEL,

            chr_dbg_flags: CHR_DBG_FLAGS,
            map_dbg_flags: MAP_DBG_FLAGS,

            kernel32_create_thread: KERNEL32_CREATE_THREAD,
            kernel32_close_handle: KERNEL32_CLOSE_HANDLE,
            kernel32_load_library_w: KERNEL32_LOAD_LIBRARY_W,
        }
    );

    let base_ptrs = BasePointers {
        world_chr_man,
        field_area,
        game_man,
        game_data_man,
        menu_man,
        cs_emk_system,
        virtual_mem_flag,
        damage_manager,
        map_item_man_impl,
        dl_user_input_manager_impl,
        cs_flipper_imp,
        cs_dlc_imp,
    };
    let functions = Functions {
        grace_warp,
        block_warp,
        get_player_item_quantity_by_id,
        item_spawn,
        give_runes,
        get_event,
        set_event,
        set_speffect,
        remove_speffect,
        get_chr_ins_by_entity_id,
        emevd_switch,
        emk_event_ins_ctor,
        external_event_temp_ctor,
        execute_talk_command,
    };
    let hooks = Hooks {
        locked_target_pointer,
        target_no_stagger,
        player_no_grab,
        player_infinite_poise,
        warp_coord_write,
        warp_angle_write,
        get_force_act_idx,
        set_requested_action,
    };
    let patches = Patches {
        no_logo,
        fps_cap,
        mute_music,
        pause_world,
        torrent_disabled_underworld,
        whistle_disabled,
        open_map,
        close_map,
        can_fast_travel,
    };
    let data = Data {
        chr_dbg_flags,
        map_dbg_flags,
    };
    let external_fn_ptrs = ExternalFunctionPointers {
        kernel32_create_thread,
        kernel32_close_handle,
        kernel32_load_library_w,
    };

    let offsets = ModuleOffsets {
        base_ptrs,
        functions,
        hooks,
        patches,
        data,
        external_fn_ptrs,
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
