use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn base() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x3C56BE0,
            Version::V1_2_1 => 0x3C56C00,
            Version::V1_2_2 => 0x3C56C20,
            Version::V1_2_3 => 0x3C59C40,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x3C6B7B0,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x3C0E8B8,
            Version::V1_5_0 => 0x3C26660,
            Version::V1_6_0 => 0x3C37880,
            Version::V1_7_0 => 0x3C52240,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x3CE0940,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x3CE3D80,
            Version::V2_2_0 => 0x3D6CFC0,
            Version::V2_2_3 |
            Version::V2_3_0 => 0x3D6CFE0,
            Version::V2_4_0 |
            Version::V2_5_0 |
            Version::V2_6_0 |
            Version::V2_6_1 => 0x3D6CFC0,
            Version::Invalid => 0x0,
        }
    }
}

pub const SHOW_ALL_MAPS: u64 = 0x0;
pub const SHOW_ALL_GRACES: u64 = 0x1;
