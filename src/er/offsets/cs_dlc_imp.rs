use crate::core::attach::{Version, module_handle, version};

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x3C705D8,
        Version::ER1_2_1 => 0x3C701F8,
        Version::ER1_2_2 => 0x3C70218,
        Version::ER1_2_3 => 0x3C73238,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x3C84DB8,
        Version::ER1_4_0 |
        Version::ER1_4_1 => 0x3C27F58,
        Version::ER1_5_0 => 0x3C3FD58,
        Version::ER1_6_0 => 0x3C50FB8,
        Version::ER1_7_0 => 0x3C6B988,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x3CFA3F8,
        Version::ER1_9_0 |
        Version::ER1_9_1 |
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x3CFD838,
        Version::ER2_2_0 => 0x3D86BD8,
        Version::ER2_2_3 |
        Version::ER2_3_0 => 0x3D86BF8,
        Version::ER2_4_0 |
        Version::ER2_5_0 |
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x3D86BD8,
        _ => 0x0,
    }
}
pub const BYTE_FLAGS: u64 = 0x10;

pub mod flags {
    pub const DLC_CHECK: u64 = 0x1;
}
