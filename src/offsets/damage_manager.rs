use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn base() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x3C50658,
            Version::V1_2_1 => 0x3C50678,
            Version::V1_2_2 => 0x3C50698,
            Version::V1_2_3 => 0x3C536B8,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x3C65228,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x3C084B8,
            Version::V1_5_0 => 0x3C20268,
            Version::V1_6_0 => 0x3C31488,
            Version::V1_7_0 => 0x3C4BE48,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x3CD9D68,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x3CDD1A8,
            Version::V2_2_0 => 0x3D66378,
            Version::V2_2_3 |
            Version::V2_3_0 => 0x3D66398,
            Version::V2_4_0 |
            Version::V2_5_0 |
            Version::V2_6_0 |
            Version::V2_6_1 => 0x3D66378,
            Version::Invalid => 0x0,
        }
    }
}

pub const HITBOXVIEW_A: u64 = 0xA0;
pub const HITBOXVIEW_B: u64 = 0xA1;
