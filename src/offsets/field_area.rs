use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn base() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x3C53470,
            Version::V1_2_1 => 0x3C53490,
            Version::V1_2_2 => 0x3C534B0,
            Version::V1_2_3 => 0x3C564D0,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x3C68040,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x3C0B2C0,
            Version::V1_5_0 => 0x3C23070,
            Version::V1_6_0 => 0x3C34298,
            Version::V1_7_0 => 0x3C4EC50,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x3CDCB80,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x3CDFFC0,
            Version::V2_2_0 => 0x3D691D8,
            Version::V2_2_3 |
            Version::V2_3_0 => 0x3D691F8,
            Version::V2_4_0 |
            Version::V2_5_0 |
            Version::V2_6_0 |
            Version::V2_6_1 => 0x3D691D8,
            Version::Invalid => 0x0,
        }
    }
}

pub const WORLD_INFO_OWNER: u64 = 0x10;

pub mod world_info_owner_offsets {
    pub const AREA_COUNT: u64 = 0x28;
    pub const AREA_ARRAY_BASE: u64 = 0x30;
}
