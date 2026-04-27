use crate::core::attach::{Version, version};

pub const HANDLE: u64 = 0x8;
pub const BLOCK_ID: u64 = 0x38;
pub const CHR_CTRL: u64 = 0x58;
pub const NPC_PARAM_ID: u64 = 0x60;
pub const CHR_ID: u64 = 0x64;
pub const TEAM_TYPE: u64 = 0x6C;
pub const SPECIAL_EFFECT: u64 = 0x178;

pub fn entity_id() -> u64 {
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
        Version::ER1_7_0 => 0x1E4,
    _ => 0x1E8,
}
}

pub const MODULES: u64 = 0x190;
pub const CHR_DATA_MODULE: u64 = 0x0;
pub const CHR_TIME_ACT_MODULE: u64 = 0x18;
pub const CHR_RESIST_MODULE: u64 = 0x20;
pub const CHR_BEHAVIOR_MODULE: u64 = 0x28;
pub const CHR_SUPER_ARMOR_MODULE: u64 = 0x40;
pub const CHR_PHYSICS_MODULE: u64 = 0x68;
pub const CHR_RIDE_MODULE: u64 = 0xE8;


pub mod data_offsets {
    pub const HEALTH: u64 = 0x138;
    pub const MAX_HEALTH: u64 = 0x13C;
    pub const FP: u64 = 0x148;
    pub const MAX_FP: u64 = 0x14C;
    pub const SP: u64 = 0x154;
    pub const MAX_SP: u64 = 0x158;
}

pub mod time_act_offsets {
    pub const ANIMATION_ID: u64 = 0xD0;
}

pub mod super_armor_offsets {
    pub const CURRENT_POISE: u64 = 0x10;
    pub const MAX_POISE: u64 = 0x14;
    pub const POISE_TIMER: u64 = 0x1C;
}

pub mod physics_offsets {
    pub const COORDS: u64 = 0x70;
    pub const NO_GRAVITY: u64 = 0x1D6;
    pub const HURT_CAPSULE_RADIUS: u64 = 0x344;
}

pub mod behavior_offsets {
    pub const ANIMATION_SPEED: u64 = 0x17C8;
}

pub mod ai_think_offsets {
use crate::core::attach::{Version, version};
    pub const NPC_THINK_PARAM_ID: u64 = 0x28;
    pub const LUA_TIMERS_ARRAY: u64 = 0x8C;
    pub const LUA_NUMBERS_ARRAY: u64 = 0x6CC;

    pub fn last_act() -> u64 {
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
            Version::ER1_7_0 => 0xE9B2,
            _ => 0xE9C2,
        }
    }

    pub fn force_act() -> u64 {
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
            Version::ER1_7_0 => 0xE9B1,
            _ => 0xE9C1,
        }
    }
}

pub mod ride_offsets {
    pub const IS_HORSE_WHISTLE_DISABLED: u64 = 0x164;
}

pub fn data_flags() -> u64 {
    match version() {
        Version::ER1_2_0 |
        Version::ER1_2_1 |
        Version::ER1_2_2 |
        Version::ER1_2_3 |
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x197,
        _ => 0x19B,
    }
}

pub mod bit_flags {
    pub const NO_DEATH: u8 = 0b00000001;
    pub const NO_DAMAGE: u8 = 0b00000010;
    pub const DISABLE_AI: u8 = 0b00000001;
}

pub fn manipulator() -> u64 {
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
        Version::ER1_7_0 => 0x570,
        _ => 0x580,
    }
}