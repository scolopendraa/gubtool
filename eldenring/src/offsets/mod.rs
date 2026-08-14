pub mod chr_ins;
pub mod code_cave;
pub mod game_data_man;
pub mod module_offsets;
pub mod world_chr_man;

use {
    crate::mem::{is_bit_set, read, set_bit, write},
    gubtool_core::sys::sys_error::ProcResult,
    pelite::Pod,
};

pub trait ChainReadExt {
    fn read_offset(self, offset: u64) -> ProcResult<u64>;
    fn add_offset(self, offset: u64) -> ProcResult<u64>;
    fn read<T: Pod>(self) -> ProcResult<T>;
    fn write<T: Pod>(self, val: T) -> ProcResult;
    fn is_bit_set(self, mask: u8) -> ProcResult<bool>;
    fn set_bit(self, mask: u8, state: bool) -> ProcResult;
}

impl ChainReadExt for ProcResult<u64> {
    #[track_caller]
    fn read_offset(self, offset: u64) -> ProcResult<u64> {
        let base = self?;
        read::<u64>(base.saturating_add(offset))
    }
    fn add_offset(self, offset: u64) -> ProcResult<u64> {
        let base = self?;
        Ok(base.saturating_add(offset))
    }
    #[track_caller]
    fn read<T: Pod>(self) -> ProcResult<T> {
        let addr = self?;
        read::<T>(addr)
    }
    #[track_caller]
    fn write<T: Pod>(self, val: T) -> ProcResult {
        let addr = self?;
        write::<T>(addr, val)
    }
    #[track_caller]
    fn is_bit_set(self, mask: u8) -> ProcResult<bool> {
        let addr = self?;
        is_bit_set(addr, mask)
    }
    #[track_caller]
    fn set_bit(self, mask: u8, state: bool) -> ProcResult {
        let addr = self?;
        set_bit(addr, mask, state)
    }
}

pub mod field_area {
    pub const WORLD_INFO_OWNER: u64 = 0x10;

    pub mod world_info_owner_offsets {
        pub const AREA_COUNT: u64 = 0x28;
        pub const AREA_ARRAY_BASE: u64 = 0x30;
    }
}

pub mod chr_dbg_flags {
    #[repr(u64)]
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
    use gubtool_core::{attached::version, game_version::EldenRingVersion::*};

    pub const QUITOUT: u64 = 0x10;

    pub fn start_new_game() -> u64 {
        match version() {
            Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) | Some(Version1_2_3)
            | Some(Version1_3_0) | Some(Version1_3_1) | Some(Version1_3_2) | Some(Version1_4_0)
            | Some(Version1_4_1) | Some(Version1_5_0) | Some(Version1_6_0) | Some(Version1_7_0)
            | Some(Version1_8_0) | Some(Version1_8_1) | Some(Version1_9_0) | Some(Version1_9_1)
            | Some(Version2_0_0) | Some(Version2_0_1) => 0xb4d,
            _ => 0xb7d,
        }
    }
}

pub mod damage_manager {
    pub const HITBOXVIEW_A: u64 = 0xa0;
    pub const HITBOXVIEW_B: u64 = 0xa1;
}

pub mod cs_flipper_imp {
    use gubtool_core::{attached::version, game_version::EldenRingVersion::*};

    pub fn game_speed() -> u64 {
        match version() {
            Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) | Some(Version1_2_3)
            | Some(Version1_3_0) | Some(Version1_3_1) | Some(Version1_3_2) | Some(Version1_4_0)
            | Some(Version1_4_1) | Some(Version1_5_0) | Some(Version1_6_0) | Some(Version1_7_0)
            | Some(Version1_8_0) | Some(Version1_8_1) | Some(Version1_9_0) | Some(Version1_9_1)
            | Some(Version2_0_0) | Some(Version2_0_1) => 0x2d4,
            _ => 0x2cc,
        }
    }
}

pub mod cs_dlc_imp {
    pub const BYTE_FLAGS: u64 = 0x10;
    pub mod flags {
        pub const DLC_CHECK: u64 = 0x1;
    }
}

pub mod dl_user_input_manager_impl {
    pub const STEAM_INPUT: u64 = 0x88b;
}

pub mod menu_man {
    use gubtool_core::{attached::version, game_version::EldenRingVersion::*};

    pub const FLAG_ARRAY: u64 = 0x90;
    pub const IS_LOADED: u64 = 0x94;

    pub fn is_fading() -> u64 {
        match version() {
            Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) | Some(Version1_2_3)
            | Some(Version1_3_0) | Some(Version1_3_1) | Some(Version1_3_2) => 0x8e,
            _ => 0x96,
        }
    }

    pub mod fade_bit_flags {
        pub const IS_FADE_SCREEN: u8 = 0b00000010;
    }
}

pub mod map_dbg_flags {
    pub const SHOW_ALL_MAPS: u64 = 0x0;
    pub const SHOW_ALL_GRACES: u64 = 0x1;
}
