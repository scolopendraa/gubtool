use crate::core::attach::{Version, module_handle, version};

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x3C56BE0,
        Version::ER1_2_1 => 0x3C56C00,
        Version::ER1_2_2 => 0x3C56C20,
        Version::ER1_2_3 => 0x3C59C40,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x3C6B7B0,
        Version::ER1_4_0 |
        Version::ER1_4_1 => 0x3C0E8B8,
        Version::ER1_5_0 => 0x3C26660,
        Version::ER1_6_0 => 0x3C37880,
        Version::ER1_7_0 => 0x3C52240,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x3CE0940,
        Version::ER1_9_0 |
        Version::ER1_9_1 |
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x3CE3D80,
        Version::ER2_2_0 => 0x3D6CFC0,
        Version::ER2_2_3 |
        Version::ER2_3_0 => 0x3D6CFE0,
        Version::ER2_4_0 |
        Version::ER2_5_0 |
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x3D6CFC0,
        _ => 0x0,
    }
}

pub const SHOW_ALL_MAPS: u64 = 0x0;
pub const SHOW_ALL_GRACES: u64 = 0x1;
