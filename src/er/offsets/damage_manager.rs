use crate::core::attach::{Version, module_handle, version};

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x3C50658,
        Version::ER1_2_1 => 0x3C50678,
        Version::ER1_2_2 => 0x3C50698,
        Version::ER1_2_3 => 0x3C536B8,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x3C65228,
        Version::ER1_4_0 |
        Version::ER1_4_1 => 0x3C084B8,
        Version::ER1_5_0 => 0x3C20268,
        Version::ER1_6_0 => 0x3C31488,
        Version::ER1_7_0 => 0x3C4BE48,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x3CD9D68,
        Version::ER1_9_0 |
        Version::ER1_9_1 |
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x3CDD1A8,
        Version::ER2_2_0 => 0x3D66378,
        Version::ER2_2_3 |
        Version::ER2_3_0 => 0x3D66398,
        Version::ER2_4_0 |
        Version::ER2_5_0 |
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x3D66378,
        _ => 0x0,
    }
}

pub const HITBOXVIEW_A: u64 = 0xA0;
pub const HITBOXVIEW_B: u64 = 0xA1;
