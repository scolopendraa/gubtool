use crate::core::attach::{Version, module_handle, version};

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x3C53470,
        Version::ER1_2_1 => 0x3C53490,
        Version::ER1_2_2 => 0x3C534B0,
        Version::ER1_2_3 => 0x3C564D0,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x3C68040,
        Version::ER1_4_0 |
        Version::ER1_4_1 => 0x3C0B2C0,
        Version::ER1_5_0 => 0x3C23070,
        Version::ER1_6_0 => 0x3C34298,
        Version::ER1_7_0 => 0x3C4EC50,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x3CDCB80,
        Version::ER1_9_0 |
        Version::ER1_9_1 |
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x3CDFFC0,
        Version::ER2_2_0 => 0x3D691D8,
        Version::ER2_2_3 |
        Version::ER2_3_0 => 0x3D691F8,
        Version::ER2_4_0 |
        Version::ER2_5_0 |
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x3D691D8,
        _ => 0x0,
    }
}

pub const WORLD_INFO_OWNER: u64 = 0x10;

pub mod world_info_owner_offsets {
    pub const AREA_COUNT: u64 = 0x28;
    pub const AREA_ARRAY_BASE: u64 = 0x30;
}
