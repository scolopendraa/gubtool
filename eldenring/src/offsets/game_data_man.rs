use gubtool_core::{attached::version, game_version::EldenRingVersion::*};

pub const PLAYER_GAME_DATA: u64 = 0x8;

#[repr(u64)]
#[derive(Clone, Copy)]
pub enum PlayerGameDataOffset {
    Vigor        = 0x3c,
    Mind         = 0x40,
    Endurance    = 0x44,
    Strength     = 0x48,
    Dexterity    = 0x4c,
    Intelligence = 0x50,
    Faith        = 0x54,
    Arcane       = 0x58,
    RuneLevel    = 0x68,
    Runes        = 0x6c,
    RuneMemory   = 0x70,
    Scadutree    = 0xfc,
    SpiritAsh    = 0xfd,
    RuneArc      = 0xff,
}

pub const OPTIONS: u64 = 0x58;

pub mod options_offsets {
    pub const MUSIC: u64 = 0x4;
}

pub const IGT: u64 = 0xa0;
pub const NEW_GAME: u64 = 0x120;

pub fn torrent_handle() -> u64 {
    match version() {
        Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) | Some(Version1_2_3)
        | Some(Version1_3_0) | Some(Version1_3_1) | Some(Version1_3_2) | Some(Version1_4_0)
        | Some(Version1_4_1) | Some(Version1_5_0) | Some(Version1_6_0) | Some(Version1_7_0)
        | Some(Version1_8_0) | Some(Version1_8_1) | Some(Version1_9_0) | Some(Version1_9_1)
        | Some(Version2_0_0) | Some(Version2_0_1) => 0x930,
        _ => 0x950,
    }
}
