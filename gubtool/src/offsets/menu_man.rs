use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn base() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x3C55B30,
            Version::V1_2_1 => 0x3C55B50,
            Version::V1_2_2 => 0x3C55B70,
            Version::V1_2_3 => 0x3C58B90,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x3C6A700,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x3C0D9D0,
            Version::V1_5_0 => 0x3C25780,
            Version::V1_6_0 => 0x3C369A0,
            Version::V1_7_0 => 0x3C51360,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x3CDF140,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x3CE2580,
            Version::V2_2_0 => 0x3D6B7B0,
            Version::V2_2_3 |
            Version::V2_3_0 => 0x3D6B7D0,
            Version::V2_4_0 |
            Version::V2_5_0 |
            Version::V2_6_0 |
            Version::V2_6_1 => 0x3D6B7B0,
            Version::Invalid => 0x0,
        }
    }
}

pub const FLAG_ARRAY: u64 = 0x90;
pub const IS_LOADED: u64 = 0x94;

pub fn is_fading() -> u64 {
    match unsafe { ATTACHED_PROCESS.version } {
        Version::V1_2_0 |
        Version::V1_2_1 |
        Version::V1_2_2 |
        Version::V1_2_3 |
        Version::V1_3_0 |
        Version::V1_3_1 |
        Version::V1_3_2 => 0x8E,
        _ => 0x96,
    }
}

pub mod fade_bit_flags {
    pub const IS_FADE_SCREEN: u8 = 0b00000010;
}
