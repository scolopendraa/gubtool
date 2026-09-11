use gubtool_core::{
    attached::version,
    game_version::EldenRingVersion::{self, *},
};

pub const HANDLE: i32 = 0x8;
pub const BLOCK_ID: i32 = 0x38;
pub const CHR_CTRL: i32 = 0x58;
pub const NPC_PARAM_ID: i32 = 0x60;
pub const CHR_ID: i32 = 0x64;
pub const TEAM_TYPE: i32 = 0x6c;
pub const SPECIAL_EFFECT: i32 = 0x178;

pub fn entity_id() -> i32 {
    match version::<EldenRingVersion>() {
        Some(v) if v <= Version1_7_0 => 0x1e4,
        _ => 0x1e8,
    }
}

pub const MODULES: i32 = 0x190;
pub const CHR_DATA_MODULE: i32 = 0x0;
pub const CHR_TIME_ACT_MODULE: i32 = 0x18;
pub const CHR_RESIST_MODULE: i32 = 0x20;
pub const CHR_BEHAVIOR_MODULE: i32 = 0x28;
pub const CHR_SUPER_ARMOR_MODULE: i32 = 0x40;
pub const CHR_PHYSICS_MODULE: i32 = 0x68;
pub const CHR_RIDE_MODULE: i32 = 0xe8;

pub mod data_offsets {
    pub const HEALTH: i32 = 0x138;
    pub const MAX_HEALTH: i32 = 0x13c;
    pub const FP: i32 = 0x148;
    pub const MAX_FP: i32 = 0x14c;
    pub const SP: i32 = 0x154;
    pub const MAX_SP: i32 = 0x158;
}

pub mod time_act_offsets {
    pub const ANIMATION_ID: i32 = 0xd0;
}

pub mod super_armor_offsets {
    pub const CURRENT_POISE: i32 = 0x10;
    pub const MAX_POISE: i32 = 0x14;
    pub const POISE_TIMER: i32 = 0x1c;
}

pub mod physics_offsets {
    pub const COORDS: i32 = 0x70;
    pub const NO_GRAVITY: i32 = 0x1d6;
    pub const HURT_CAPSULE_RADIUS: i32 = 0x344;
}

pub mod behavior_offsets {
    pub const ANIMATION_SPEED: i32 = 0x17c8;
}

pub mod speffect_offsets {
    pub const HEAD: i32 = 0x8;
}

pub mod speffect_entry {
    pub const PARAM_DATA: i32 = 0x0;
    pub const ID: i32 = 0x8;
    pub const NEXT: i32 = 0x30;
    pub const TIME_LEFT: i32 = 0x40;
    pub const DURATION: i32 = 0x48;
}

pub mod ai_think_offsets {
    use gubtool_core::{
        attached::version,
        game_version::EldenRingVersion::{self, *},
    };
    pub const NPC_THINK_PARAM_ID: i32 = 0x28;
    pub const LUA_TIMERS_ARRAY: i32 = 0x8c;
    pub const LUA_NUMBERS_ARRAY: i32 = 0x6cc;

    pub fn last_act() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version1_7_0 => 0xe9b2,
            _ => 0xe9c2,
        }
    }

    pub fn force_act() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version1_7_0 => 0xe9b1,
            _ => 0xe9c1,
        }
    }

    pub fn ai_attack_comp() -> i32 {
        match version::<EldenRingVersion>() {
            Some(v) if v <= Version1_7_0 => 0xdf00,
            _ => 0xdf10,
        }
    }

    pub mod attack_comp_offsets {
        pub const COOLDOWN_COUNT: i32 = 0x14;
        pub const COOLDOWN_LIST: i32 = 0x18;
    }
}

pub mod ride_offsets {
    pub const IS_HORSE_WHISTLE_DISABLED: i32 = 0x164;
}

pub fn data_flags() -> i32 {
    match version::<EldenRingVersion>() {
        Some(v) if v <= Version1_3_2 => 0x197,
        _ => 0x19b,
    }
}

pub mod bit_flags {
    pub const NO_DEATH: u8 = 0b00000001;
    pub const NO_DAMAGE: u8 = 0b00000010;
    pub const DISABLE_AI: u8 = 0b00000001;
}

pub fn com_manipulator() -> i32 {
    match version::<EldenRingVersion>() {
        Some(v) if v <= Version1_7_0 => 0x570,
        _ => 0x580,
    }
}
