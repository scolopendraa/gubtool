use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn base() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x3C481B8,
            Version::V1_2_1 => 0x3C481D8,
            Version::V1_2_2 => 0x3C481F8,
            Version::V1_2_3 => 0x3C4B218,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x3C5CD78,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x3C00028,
            Version::V1_5_0 => 0x3C17EE8,
            Version::V1_6_0 => 0x3C29108,
            Version::V1_7_0 => 0x3C43AC8,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x3CD1948,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x3CD4D88,
            Version::V2_2_0 => 0x3D5DF38,
            Version::V2_2_3 |
            Version::V2_3_0 => 0x3D5DF58,
            Version::V2_4_0 |
            Version::V2_5_0 |
            Version::V2_6_0 |
            Version::V2_6_1 => 0x3D5DF38,
            Version::Invalid => 0x0,
        }
    }
}

pub const PLAYER_GAME_DATA: u64 = 0x8;

pub mod player_game_data_offsets {
    pub const VIGOR: u64 = 0x3C;
    pub const MIND: u64 = 0x40;
    pub const ENDURANCE: u64 = 0x44;
    pub const STRENGTH: u64 = 0x48;
    pub const DEXTERITY: u64 = 0x4C;
    pub const INTELLIGENCE: u64 = 0x50;
    pub const FAITH: u64 = 0x54;
    pub const ARCANE: u64 = 0x58;
    pub const RUNE_LEVEL: u64 = 0x68;
    pub const RUNES: u64 = 0x6C;
    pub const RUNE_MEMORY: u64 = 0x70;
    pub const SCADUTREE: u64 = 0xFC;
    pub const SPIRIT_ASH: u64 = 0xFD;
    pub const RUNE_ARC: u64 = 0xFF;
}

pub const OPTIONS: u64 = 0x58;

pub mod options_offsets {
    pub const MUSIC: u64 = 0x4;
}

pub const IGT: u64 = 0xA0;
pub const NEW_GAME: u64 = 0x120;

pub fn torrent_handle() -> u64 {
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
            Version::V2_0_1 => 0x930,
            _ => 0x950,
        }
    }
}
