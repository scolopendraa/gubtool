use {
    serde::{Deserialize, Serialize},
    std::fmt::Display,
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameVersion {
    EldenRing(EldenRingVersion),
    DarkSouls2(DarkSouls2Version),
}

impl GameVersion {
    pub fn game(&self) -> Game {
        match self {
            Self::EldenRing(_) => Game::EldenRing,
            Self::DarkSouls2(_) => Game::DarkSouls2,
        }
    }
}

pub trait Version: Copy + Sized {
    fn from_game_version(game_version: &GameVersion) -> Option<Self>;
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Game {
    EldenRing,
    DarkSouls2,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DarkSouls2Version {
    Vanilla1_0_3,
    Vanilla1_0_4,
    Vanilla1_0_5,
    Vanilla1_0_6,
    Vanilla1_0_7,
    Vanilla1_0_10,
    Vanilla1_0_11,
    Vanilla1_0_12,
    Scholar1_0_1,
    Scholar1_0_2,
    Scholar1_0_3,
    VanillaUnknown,
    ScholarUnknown,
}

impl Version for DarkSouls2Version {
    fn from_game_version(game_version: &GameVersion) -> Option<Self> {
        match game_version {
            GameVersion::DarkSouls2(version) => Some(*version),
            _ => None,
        }
    }
}

impl Display for DarkSouls2Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Self::Vanilla1_0_3 => "Dark Souls II v1.0.3",
            Self::Vanilla1_0_4 => "Dark Souls II v1.0.4",
            Self::Vanilla1_0_5 => "Dark Souls II v1.0.5",
            Self::Vanilla1_0_6 => "Dark Souls II v1.0.6",
            Self::Vanilla1_0_7 => "Dark Souls II v1.0.7",
            Self::Vanilla1_0_10 => "Dark Souls II v1.0.10",
            Self::Vanilla1_0_11 => "Dark Souls II v1.0.11",
            Self::Vanilla1_0_12 => "Dark Souls II v1.0.12",
            Self::VanillaUnknown => "Unknown",
            Self::Scholar1_0_1 => "Dark Souls II SOTFS v1.0.1",
            Self::Scholar1_0_2 => "Dark Souls II SOTFS v1.0.2",
            Self::Scholar1_0_3 => "Dark Souls II SOTFS v1.0.3",
            Self::ScholarUnknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EldenRingVersion {
    Version1_2_0,
    Version1_2_1,
    Version1_2_2,
    Version1_2_3,
    Version1_3_0,
    Version1_3_1,
    Version1_3_2,
    Version1_4_0,
    Version1_4_1,
    Version1_5_0,
    Version1_6_0,
    Version1_7_0,
    Version1_8_0,
    Version1_8_1,
    Version1_9_0,
    Version1_9_1,
    Version2_0_0,
    Version2_0_1,
    Version2_2_0,
    Version2_2_3,
    Version2_3_0,
    Version2_4_0,
    Version2_5_0,
    Version2_6_0,
    Version2_6_1,
    Version2_6_2,
    VersionUnknown,
}

impl Version for EldenRingVersion {
    fn from_game_version(game_version: &GameVersion) -> Option<Self> {
        match game_version {
            GameVersion::EldenRing(version) => Some(*version),
            _ => None,
        }
    }
}

impl Display for EldenRingVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Self::Version1_2_0 => "Elden Ring v1.02",
            Self::Version1_2_1 => "Elden Ring v1.02.1",
            Self::Version1_2_2 => "Elden Ring v1.02.2",
            Self::Version1_2_3 => "Elden Ring v1.02.3",
            Self::Version1_3_0 => "Elden Ring v1.03",
            Self::Version1_3_1 => "Elden Ring v1.03.1",
            Self::Version1_3_2 => "Elden Ring v1.03.2",
            Self::Version1_4_0 => "Elden Ring v1.04",
            Self::Version1_4_1 => "Elden Ring v1.04.1",
            Self::Version1_5_0 => "Elden Ring v1.05",
            Self::Version1_6_0 => "Elden Ring v1.06",
            Self::Version1_7_0 => "Elden Ring v1.07",
            Self::Version1_8_0 => "Elden Ring v1.08",
            Self::Version1_8_1 => "Elden Ring v1.08.1",
            Self::Version1_9_0 => "Elden Ring v1.09",
            Self::Version1_9_1 => "Elden Ring v1.09.1",
            Self::Version2_0_0 => "Elden Ring v1.10",
            Self::Version2_0_1 => "Elden Ring v1.10.1",
            Self::Version2_2_0 => "Elden Ring v1.12",
            Self::Version2_2_3 => "Elden Ring v1.12.3",
            Self::Version2_3_0 => "Elden Ring v1.13",
            Self::Version2_4_0 => "Elden Ring v1.14",
            Self::Version2_5_0 => "Elden Ring v1.15",
            Self::Version2_6_0 => "Elden Ring v1.16",
            Self::Version2_6_1 => "Elden Ring v1.16.1",
            Self::Version2_6_2 => "Elden Ring v1.16.2",
            Self::VersionUnknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EldenRing => write!(f, "Elden Ring"),
            Self::DarkSouls2 => write!(f, "Dark Souls II"),
        }
    }
}

impl Display for GameVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EldenRing(version) => write!(f, "{version}"),
            Self::DarkSouls2(version) => write!(f, "{version}"),
        }
    }
}
