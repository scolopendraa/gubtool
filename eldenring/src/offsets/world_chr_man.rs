use gubtool_core::{
    attached::version,
    game_version::EldenRingVersion::{self, *},
};

pub fn chr_set_pool() -> i32 {
    match version::<EldenRingVersion>() {
        Some(v) if v <= Version1_6_0 => 0x18038,
        _ => 0x1ded8,
    }
}

pub mod chr_set_offsets {
    pub const CHR_SET_ENTRIES: i32 = 0x18;
}

pub fn player_ins() -> i32 {
    match version::<EldenRingVersion>() {
        Some(v) if v <= Version1_6_0 => 0x18468,
        _ => 0x1e508,
    }
}

pub mod player_ins_offsets {
    use gubtool_core::{
        attached::version,
        game_version::EldenRingVersion::{self, *},
    };

    pub fn current_block_id() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version1_3_2 => 0x6c8,
            Some(v) if v <= Version1_7_0 => 0x6c0,
            _ => 0x6d0,
        }
    }
    pub fn current_map_coords() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version1_3_2 => 0x6b8,
            Some(v) if v <= Version1_7_0 => 0x6b0,
            _ => 0x6c0,
        }
    }
    pub fn current_map_angle() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version1_3_2 => 0x6c4,
            Some(v) if v <= Version1_7_0 => 0x6bc,
            _ => 0x6cc,
        }
    }
}
