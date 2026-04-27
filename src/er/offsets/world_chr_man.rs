use crate::core::attach::{Version, module_handle, version};

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x3C50268,
        Version::ER1_2_1 => 0x3C50288,
        Version::ER1_2_2 => 0x3C502A8,
        Version::ER1_2_3 => 0x3C532C8,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x3C64E38,
        Version::ER1_4_0 |
        Version::ER1_4_1 => 0x3C080E8,
        Version::ER1_5_0 => 0x3C1FE98,
        Version::ER1_6_0 => 0x3C310B8,
        Version::ER1_7_0 => 0x3C4BA78,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x3CD9998,
        Version::ER1_9_0 |
        Version::ER1_9_1 |
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x3CDCDD8,
        Version::ER2_2_0 => 0x3D65F88,
        Version::ER2_2_3 |
        Version::ER2_3_0 => 0x3D65FA8,
        Version::ER2_4_0 |
        Version::ER2_5_0 |
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x3D65F88,
        _ => 0x0,
    }
}

pub fn chr_set_pool() -> u64 {
    match version() {
        Version::ER1_2_0 |
        Version::ER1_2_1 |
        Version::ER1_2_2 |
        Version::ER1_2_3 |
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 |
        Version::ER1_4_0 |
        Version::ER1_4_1 |
        Version::ER1_5_0 |
        Version::ER1_6_0 => 0x18038,
        _ => 0x1DED8,
    }
}

pub mod chr_set_offsets {
    pub const CHR_SET_ENTRIES: u64 = 0x18;
}

pub fn player_ins() -> u64 {
    match version() {
        Version::ER1_2_0 |
        Version::ER1_2_1 |
        Version::ER1_2_2 |
        Version::ER1_2_3 |
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 |
        Version::ER1_4_0 |
        Version::ER1_4_1 |
        Version::ER1_5_0 |
        Version::ER1_6_0 => 0x18468,
        _ => 0x1E508,
    }
}

pub mod player_ins_offsets {
use crate::core::attach::{Version, version};

    pub const HANDLE: u64 = 0x8;

    pub fn current_block_id() -> u64 {
        match version() {
            Version::ER1_2_0 |
            Version::ER1_2_1 |
            Version::ER1_2_2 |
            Version::ER1_2_3 |
            Version::ER1_3_0 |
            Version::ER1_3_1 |
            Version::ER1_3_2 => 0x6C8,
            Version::ER1_4_0 |
            Version::ER1_4_1 |
            Version::ER1_5_0 |
            Version::ER1_6_0 |
            Version::ER1_7_0 => 0x6C0,
            _ => 0x6D0,
        }
    }
    pub fn current_map_coords() -> u64 {
        match version() {
            Version::ER1_2_0 |
            Version::ER1_2_1 |
            Version::ER1_2_2 |
            Version::ER1_2_3 |
            Version::ER1_3_0 |
            Version::ER1_3_1 |
            Version::ER1_3_2 => 0x6B8,
            Version::ER1_4_0 |
            Version::ER1_4_1 |
            Version::ER1_5_0 |
            Version::ER1_6_0 |
            Version::ER1_7_0 => 0x6B0,
            _ => 0x6C0,
        }
    }
    pub fn current_map_angle() -> u64 {
        match version() {
            Version::ER1_2_0 |
            Version::ER1_2_1 |
            Version::ER1_2_2 |
            Version::ER1_2_3 |
            Version::ER1_3_0 |
            Version::ER1_3_1 |
            Version::ER1_3_2 => 0x6C4,
            Version::ER1_4_0 |
            Version::ER1_4_1 |
            Version::ER1_5_0 |
            Version::ER1_6_0 |
            Version::ER1_7_0 => 0x6BC,
            _ => 0x6CC,
        }
    }
}
