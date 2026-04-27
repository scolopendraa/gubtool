use crate::core::attach::{Version, module_handle, version};

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x3C53B88,
        Version::ER1_2_1 => 0x3C53BA8,
        Version::ER1_2_2 => 0x3C53BC8,
        Version::ER1_2_3 => 0x3C56BE8,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x3C68758,
        Version::ER1_4_0 |
        Version::ER1_4_1 => 0x3C0BA08,
        Version::ER1_5_0 => 0x3C237B8,
        Version::ER1_6_0 => 0x3C349D8,
        Version::ER1_7_0 => 0x3C4F398,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x3CDD2C8,
        Version::ER1_9_0 |
        Version::ER1_9_1 |
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x3CE0708,
        Version::ER2_2_0 => 0x3D69918,
        Version::ER2_2_3 |
        Version::ER2_3_0 => 0x3D69938,
        Version::ER2_4_0 |
        Version::ER2_5_0 |
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x3D69918,
        _ => 0x0,
    }
}

pub fn start_new_game() -> u64 {
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
        Version::ER1_6_0 |
        Version::ER1_7_0 |
        Version::ER1_8_0 |
        Version::ER1_8_1 |
        Version::ER1_9_0 |
        Version::ER1_9_1 |
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0xB4D,
        _ => 0xB7D,
    }
}
