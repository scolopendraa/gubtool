use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn base() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x3C53B88,
            Version::V1_2_1 => 0x3C53BA8,
            Version::V1_2_2 => 0x3C53BC8,
            Version::V1_2_3 => 0x3C56BE8,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x3C68758,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x3C0BA08,
            Version::V1_5_0 => 0x3C237B8,
            Version::V1_6_0 => 0x3C349D8,
            Version::V1_7_0 => 0x3C4F398,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x3CDD2C8,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x3CE0708,
            Version::V2_2_0 => 0x3D69918,
            Version::V2_2_3 |
            Version::V2_3_0 => 0x3D69938,
            Version::V2_4_0 |
            Version::V2_5_0 |
            Version::V2_6_0 |
            Version::V2_6_1 => 0x3D69918,
            Version::Invalid => 0x0,
        }
    }
}

pub fn start_new_game() -> u64 {
    match unsafe { ATTACHED_PROCESS.version } {
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
        Version::V2_0_1 => 0xB4D,
        _ => 0xB7D,
    }
}
