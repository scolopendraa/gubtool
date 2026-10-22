pub mod chr_ins;
pub mod code_cave;
pub mod game_data_man;
pub mod module_offsets;
pub mod world_chr_man;

pub mod field_area {
    pub const WORLD_INFO_OWNER: i32 = 0x10;

    pub mod world_info_owner_offsets {
        pub const AREA_COUNT: i32 = 0x28;
        pub const AREA_ARRAY_BASE: i32 = 0x30;
    }
}

pub mod chr_dbg_flags {
    #[repr(i32)]
    pub enum ChrDbgOffset {
        PlayerNoDeath       = 0x0,
        OneShot             = 0x2,
        InfiniteConsumables = 0x3,
        InfiniteStamina     = 0x4,
        InfiniteFp          = 0x5,
        InfiniteArrows      = 0x6,
        Hidden              = 0x8,
        Silent              = 0x9,
        AllNoDeath          = 0xa,
        AllNoDamage         = 0xb,
        AllNoHit            = 0xc,
        AllNoAttack         = 0xd,
        AllNoMove           = 0xe,
        AllDisableAi        = 0xf,
    }
}

pub mod game_man {
    use gubtool_core::{
        attached::version,
        game_version::EldenRingVersion::{self, *},
    };

    pub const QUITOUT: i32 = 0x10;

    pub fn stored_time() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version2_0_1 => 0x18,
            _ => 0x20,
        }
    }

    pub fn start_new_game() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version2_0_1 => 0xb4d,
            _ => 0xb7d,
        }
    }
}

pub mod damage_manager {
    use gubtool_core::{
        attached::version,
        game_version::EldenRingVersion::{self, *},
    };

    pub fn hitbox_view_a() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version2_7_1 => 0xa0,
            _ => 0xa8,
        }
    }

    pub fn hitbox_view_b() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version2_7_1 => 0xa1,
            _ => 0xa9,
        }
    }
}

pub mod cs_flipper_imp {
    use gubtool_core::{
        attached::version,
        game_version::EldenRingVersion::{self, *},
    };

    pub fn game_speed() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version2_0_1 => 0x2d4,
            _ => 0x2cc,
        }
    }
}

pub mod cs_dlc_imp {
    pub const BYTE_FLAGS: i32 = 0x10;
    pub mod flags {
        pub const DLC_CHECK: i32 = 0x1;
    }
}

pub mod dl_user_input_manager_impl {
    pub const STEAM_INPUT: i32 = 0x88b;
}

pub mod menu_man {
    use gubtool_core::{
        attached::version,
        game_version::EldenRingVersion::{self, *},
    };

    pub const FLAG_ARRAY: i32 = 0x90;
    pub const IS_LOADED: i32 = 0x94;

    pub fn is_fading() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version1_3_2 => 0x8e,
            _ => 0x96,
        }
    }

    pub mod fade_bit_flags {
        pub const IS_FADE_SCREEN: u8 = 0b00000010;
    }
}

pub mod map_dbg_flags {
    pub const SHOW_ALL_MAPS: i32 = 0x0;
    pub const SHOW_ALL_GRACES: i32 = 0x1;
}

pub mod lock_tgt_man_imp {
    pub const IS_LOCKED: i32 = 0x2831;
}

pub mod lua_event_system {
    pub const LUA_EVENT_PROXY: i32 = 0x8;
}
