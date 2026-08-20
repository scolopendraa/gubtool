use {
    crate::offsets::module_offsets::structs::{
        BasePointers,
        Data,
        ExternalFunctionPointers,
        Functions,
        Hooks,
        ModuleOffsets,
        Patches,
    },
    gubtool_core::{
        aob_scanner::{ScanStrategy, scan_error::ScanError},
        attached::{self, AddressSize},
        parallel_scan,
        pe::PeParser,
    },
    std::path::PathBuf,
};

macro_rules! patterns {
    ($strategy:expr) => {{
        parallel_scan!(
            $strategy,
            {
                game_manager_imp: GAME_MANAGER_IMP,
                katana_main_app: KATANA_MAIN_APP,

                give_souls: GIVE_SOULS,
                warp: WARP,
                item_spawn: ITEM_SPAWN,
                build_item_dialogue: BUILD_ITEM_DIALOGUE,
                show_item_dialogue: SHOW_ITEM_DIALOGUE,
                current_item_quantity_check: CURRENT_ITEM_QUANTITY_CHECK,
                set_event: SET_EVENT,
                map_entity_from_map_id_and_obj_id: MAP_ENTITY_FROM_MAP_ID_AND_OBJ_ID,
                get_state_act_component: GET_STATE_ACT_COMPONENT,
                make_sound: MAKE_SOUND,
                bonfire_rest: BONFIRE_REST,
                bonfire_unlock: BONFIRE_UNLOCK,
                open_menu: OPEN_MENU,
                menu_chr_state: MENU_CHR_STATE,
                level_up: LEVEL_UP,
                level_lookup: LEVEL_LOOKUP,
                chr_set_action: CHR_SET_ACTION,
                ez_state_external_event_ctor: EZ_STATE_EXTERNAL_EVENT_CTOR,
                ez_state_execute_event: EZ_STATE_EXECUTE_EVENT,

                set_shared_flag: SET_SHARED_FLAG,
                locked_target_pointer: LOCKED_TARGET_POINTER,
                credits_skip: CREDITS_SKIP,
                faster_menu: FASTER_MENU,
                event_log: EVENT_LOG,
                player_no_damage: PLAYER_NO_DAMAGE,
                infinite_poise: INFINITE_POISE,

                infinite_stamina: INFINITE_STAMINA,
                infinite_consumables: INFINITE_CONSUMABLES,
                infinite_durability: INFINITE_DURABILITY,
                infinite_casts: INFINITE_CASTS,
                no_soul_gain: NO_SOUL_GAIN,
                no_hollowing: NO_HOLLOWING,
                no_soul_loss: NO_SOUL_LOSS,
                player_hidden: PLAYER_HIDDEN,
                player_silent: PLAYER_SILENT,
                skip_logos: SKIP_LOGOS,
                menu_transition: MENU_TRANSITION,
                no_roll: NO_ROLL,
                no_backstep: NO_BACKSTEP,

                map_id: MAP_ID,

                kernel32_create_thread: KERNEL32_CREATE_THREAD,
                kernel32_close_handle: KERNEL32_CLOSE_HANDLE,
                kernel32_sleep: KERNEL32_SLEEP,
                kernel32_load_library_w: KERNEL32_LOAD_LIBRARY_W,
            }
        );

    let base_ptrs = BasePointers {
        game_manager_imp,
        katana_main_app,
    };
    let functions = Functions {
        give_souls,
        warp,
        item_spawn,
        build_item_dialogue,
        show_item_dialogue,
        current_item_quantity_check,
        set_event,
        map_entity_from_map_id_and_obj_id,
        get_state_act_component,
        make_sound,
        bonfire_rest,
        bonfire_unlock,
        open_menu,
        menu_chr_state,
        level_up,
        level_lookup,
        chr_set_action,
        ez_state_external_event_ctor,
        ez_state_execute_event,
    };
    let hooks = Hooks {
        set_shared_flag,
        locked_target_pointer,
        credits_skip,
        faster_menu,
        event_log,
        player_no_damage,
        infinite_poise,
    };
    let patches = Patches {
        infinite_stamina,
        infinite_consumables,
        infinite_durability,
        infinite_casts,
        no_soul_gain,
        no_hollowing,
        no_soul_loss,
        player_hidden,
        player_silent,
        skip_logos,
        menu_transition,
        no_roll,
        no_backstep,
    };
    let data = Data {
        map_id,
    };
    let external_fn_ptrs = ExternalFunctionPointers {
        kernel32_create_thread,
        kernel32_close_handle,
        kernel32_sleep,
        kernel32_load_library_w,
    };

    let offsets = ModuleOffsets {
        base_ptrs, functions, hooks, patches, data, external_fn_ptrs
    };

    offsets
    }};
}

fn scan(strategy: ScanStrategy, address_size: AddressSize) -> Result<ModuleOffsets, ScanError> {
    match address_size {
        AddressSize::Bits32 => {
            use crate::resources::vanilla_patterns::*;
            Ok(patterns!(strategy))
        }

        AddressSize::Bits64 => {
            use crate::resources::scholar_patterns::*;
            Ok(patterns!(strategy))
        }
    }
}

pub fn scan_mem() -> Result<ModuleOffsets, ScanError> {
    let strategy = ScanStrategy::Mem;
    scan(strategy, attached::address_size().unwrap())
}

pub fn scan_mem_exhaustive() -> Result<ModuleOffsets, ScanError> {
    let strategy = ScanStrategy::MemExhaustive;
    scan(strategy, attached::address_size().unwrap())
}

pub fn scan_disk<T>(path: T) -> Result<ModuleOffsets, ScanError>
where T: Into<PathBuf> {
    let path = path.into();
    let pe_image = PeParser::new(&path)?;
    let address_size = pe_image.address_size()?;
    let strategy = ScanStrategy::Disk(&path);
    scan(strategy, address_size)
}

pub fn scan_disk_exhaustive<T>(path: T) -> Result<ModuleOffsets, ScanError>
where T: Into<PathBuf> {
    let path = path.into();
    let pe_image = PeParser::new(&path)?;
    let address_size = pe_image.address_size()?;
    let strategy = ScanStrategy::DiskExhaustive(&path);
    scan(strategy, address_size)
}
