use crate::core::attach::{Version, module_handle, version};

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x4473138,
        Version::ER1_2_1 => 0x4472D58,
        Version::ER1_2_2 => 0x4472D78,
        Version::ER1_2_3 => 0x4475D98,
        Version::ER1_3_0 => 0x4487918,
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x4487908,
        Version::ER1_4_0 |
        Version::ER1_4_1 => 0x442AB08,
        Version::ER1_5_0 => 0x4442C18,
        Version::ER1_6_0 => 0x4453E98,
        Version::ER1_7_0 => 0x446E858,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x44FD2C8,
        Version::ER1_9_0 |
        Version::ER1_9_1 |
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x4500708,
        Version::ER2_2_0 => 0x4589AD8,
        Version::ER2_2_3 |
        Version::ER2_3_0 => 0x4589AF8,
        Version::ER2_4_0 |
        Version::ER2_5_0 |
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x4589AD8,
        _ => 0x0,
    }
}

pub fn game_speed() -> u64 {
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
        Version::ER2_0_1 => 0x2D4,
        _ => 0x2CC,
    }
}