use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn base() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x3C526E8,
            Version::V1_2_1 => 0x3C52708,
            Version::V1_2_2 => 0x3C52728,
            Version::V1_2_3 => 0x3C55748,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x3C672A8,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x3C0A538,
            Version::V1_5_0 => 0x3C222E8,
            Version::V1_6_0 => 0x3C33508,
            Version::V1_7_0 => 0x3C4DEC8,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x3CDBDF8,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x3CDF238,
            Version::V2_2_0 => 0x3D68448,
            Version::V2_2_3 |
            Version::V2_3_0 => 0x3D68468,
            Version::V2_4_0 |
            Version::V2_5_0 |
            Version::V2_6_0 |
            Version::V2_6_1 => 0x3D68448,
            Version::Invalid => 0x0,
        }
    }
}

pub const BLOCK_SIZE: u64 = 0x1C;
pub const STRIDE: u64 = 0x20;
pub const EVENT_TREE_BASE: u64 = 0x28;
pub const EVENT_TREE_ROOT: u64 = 0x38;

pub mod node_offsets {
    pub const LEFT_CHILD: u64 = 0x0;
    pub const RIGHT_CHILD: u64 = 0x10;
    pub const IS_LEAF: u64 = 0x19;
    pub const BLOCK_INDEX: u64 = 0x20;
    pub const TYPE: u64 = 0x28;
    pub const DATA_INDEX: u64 = 0x30;
}
