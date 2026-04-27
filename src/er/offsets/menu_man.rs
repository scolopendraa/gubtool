use crate::core::attach::{Version, module_handle, version};

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x3C55B30,
        Version::ER1_2_1 => 0x3C55B50,
        Version::ER1_2_2 => 0x3C55B70,
        Version::ER1_2_3 => 0x3C58B90,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x3C6A700,
        Version::ER1_4_0 |
        Version::ER1_4_1 => 0x3C0D9D0,
        Version::ER1_5_0 => 0x3C25780,
        Version::ER1_6_0 => 0x3C369A0,
        Version::ER1_7_0 => 0x3C51360,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x3CDF140,
        Version::ER1_9_0 |
        Version::ER1_9_1 |
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x3CE2580,
        Version::ER2_2_0 => 0x3D6B7B0,
        Version::ER2_2_3 |
        Version::ER2_3_0 => 0x3D6B7D0,
        Version::ER2_4_0 |
        Version::ER2_5_0 |
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x3D6B7B0,
        _ => 0x0,
    }
}

pub const FLAG_ARRAY: u64 = 0x90;
pub const IS_LOADED: u64 = 0x94;

pub fn is_fading() -> u64 {
    match version() {
        Version::ER1_2_0 |
        Version::ER1_2_1 |
        Version::ER1_2_2 |
        Version::ER1_2_3 |
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x8E,
        _ => 0x96,
    }
}

pub mod fade_bit_flags {
    pub const IS_FADE_SCREEN: u8 = 0b00000010;
}
