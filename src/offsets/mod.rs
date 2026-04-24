pub mod chr_dbg_flags;
pub mod chr_ins;
pub mod code_cave;
pub mod cs_dlc_imp;
pub mod cs_flipper_imp;
pub mod damage_manager;
pub mod field_area;
pub mod functions;
pub mod game_data_man;
pub mod game_man;
pub mod hooks;
pub mod map_debug_flags;
pub mod menu_man;
pub mod patches;
pub mod virtual_memory_flag;
pub mod world_chr_man;

use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn kernel32_create_thread() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x48CC70C,
            Version::V1_2_1 => 0x48CC6EC,
            Version::V1_2_2 => 0x48CC71C,
            Version::V1_2_3 => 0x48CF71C,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x48E271C,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x487E734,
            Version::V1_5_0 => 0x4897734,
            Version::V1_6_0 => 0x48AA734,
            Version::V1_7_0 => 0x48C5734,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x495C734,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x495F6C4,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x4C0C6BC,
            Version::V2_3_0 |
            Version::V2_4_0 |
            Version::V2_5_0 |
            Version::V2_6_0 |
            Version::V2_6_1 => 0x4C0C714,
            Version::Invalid => 0x0,
        }
    }
}

pub mod map_item_impl {
    use crate::core::attach::{Version, ATTACHED_PROCESS};
    pub fn base() -> u64 {
        unsafe {
            ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
                Version::V1_2_0 => 0x3C51CF0,
                Version::V1_2_1 => 0x3C51D10,
                Version::V1_2_2 => 0x3C51D30,
                Version::V1_2_3 => 0x3C54D50,
                Version::V1_3_0 |
                Version::V1_3_1 |
                Version::V1_3_2 => 0x3C668C0,
                Version::V1_4_0 |
                Version::V1_4_1 => 0x3C09B50,
                Version::V1_5_0 => 0x3C21900,
                Version::V1_6_0 => 0x3C32B20,
                Version::V1_7_0 => 0x3C4D4E0,
                Version::V1_8_0 |
                Version::V1_8_1 => 0x3CDB400,
                Version::V1_9_0 |
                Version::V1_9_1 |
                Version::V2_0_0 |
                Version::V2_0_1 => 0x3CDE840,
                Version::V2_2_0 => 0x3D67A50,
                Version::V2_2_3 |
                Version::V2_3_0 => 0x3D67A70,
                Version::V2_4_0 |
                Version::V2_5_0 |
                Version::V2_6_0 |
                Version::V2_6_1 => 0x3D67A50,
                Version::Invalid => 0x0,
            }
        }
    }
}

pub mod user_input_manager {
    use crate::core::attach::{Version, ATTACHED_PROCESS};
    pub fn base() -> u64 {
        unsafe {
            ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
                Version::V1_2_0 => 0x45255C8,
                Version::V1_2_1 => 0x45251E8,
                Version::V1_2_2 => 0x4525208,
                Version::V1_2_3 => 0x4528228,
                Version::V1_3_0 => 0x4539DA8,
                Version::V1_3_1 |
                Version::V1_3_2 => 0x4539D98,
                Version::V1_4_0 |
                Version::V1_4_1 => 0x44DD6E8,
                Version::V1_5_0 => 0x44F5828,
                Version::V1_6_0 => 0x45075C8,
                Version::V1_7_0 => 0x4521F88,
                Version::V1_8_0 |
                Version::V1_8_1 => 0x45B1918,
                Version::V1_9_0 |
                Version::V1_9_1 |
                Version::V2_0_0 |
                Version::V2_0_1 => 0x45B4D48,
                Version::V2_2_0 => 0x485DB68,
                Version::V2_2_3 |
                Version::V2_3_0 => 0x485DB88,
                Version::V2_4_0 |
                Version::V2_5_0 |
                Version::V2_6_0 |
                Version::V2_6_1 => 0x485DC18,
                Version::Invalid => 0x0,
            }
        }
    }
    pub const STEAM_INPUT: u64 = 0x88B;
}

pub mod cs_emk_system {
    use crate::core::attach::{Version, ATTACHED_PROCESS};
    pub fn base() -> u64 {
        unsafe {
            ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
                Version::V1_2_0 => 0x3C51E78,
                Version::V1_2_1 => 0x3C51E98,
                Version::V1_2_2 => 0x3C51EB8,
                Version::V1_2_3 => 0x3C54ED8,
                Version::V1_3_0 |
                Version::V1_3_1 |
                Version::V1_3_2 => 0x3C66A40,
                Version::V1_4_0 |
                Version::V1_4_1 => 0x3C09CD0,
                Version::V1_5_0 => 0x3C21A80,
                Version::V1_6_0 => 0x3C32CA0,
                Version::V1_7_0 => 0x3C4D660,
                Version::V1_8_0 |
                Version::V1_8_1 => 0x3CDB580,
                Version::V1_9_0 |
                Version::V1_9_1 |
                Version::V2_0_0 |
                Version::V2_0_1 => 0x3CDE9C0,
                Version::V2_2_0 => 0x3D67BD0,
                Version::V2_2_3 |
                Version::V2_3_0 => 0x3D67BF0,
                Version::V2_4_0 |
                Version::V2_5_0 |
                Version::V2_6_0 |
                Version::V2_6_1 => 0x3D67BD0,
                Version::Invalid => 0x0,
            }
        }
    }
}
