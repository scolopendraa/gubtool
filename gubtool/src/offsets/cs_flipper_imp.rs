use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn base() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x4473138,
            Version::V1_2_1 => 0x4472D58,
            Version::V1_2_2 => 0x4472D78,
            Version::V1_2_3 => 0x4475D98,
            Version::V1_3_0 => 0x4487918,
            Version::V1_3_1 |
            Version::V1_3_2 => 0x4487908,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x442AB08,
            Version::V1_5_0 => 0x4442C18,
            Version::V1_6_0 => 0x4453E98,
            Version::V1_7_0 => 0x446E858,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x44FD2C8,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x4500708,
            Version::V2_2_0 => 0x4589AD8,
            Version::V2_2_3 |
            Version::V2_3_0 => 0x4589AF8,
            Version::V2_4_0 |
            Version::V2_5_0 |
            Version::V2_6_0 |
            Version::V2_6_1 => 0x4589AD8,
            Version::Invalid => 0x0,
        }
    }
}

pub fn game_speed() -> u64 {
    unsafe {
        match ATTACHED_PROCESS.version {
            Version::V1_2_0 |
            Version::V1_2_1 |
            Version::V1_2_2 |
            Version::V1_2_3 |
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 |
            Version::V1_4_0 |
            Version::V1_4_1 |
            Version::V1_5_0 |
            Version::V1_6_0 |
            Version::V1_7_0 |
            Version::V1_8_0 |
            Version::V1_8_1 |
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x2D4,
            _ => 0x2CC,
        }
    }
}
