use crate::core::attach::{Version, module_handle, version};

pub const SIZE: usize = 0x5000;

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x3D380A2,
        Version::ER1_2_1 => 0x3F01A17,
        Version::ER1_2_2 => 0x3D3D54A,
        Version::ER1_2_3 => 0x3E757D7,
        Version::ER1_3_0 => 0x3DB6060,
        Version::ER1_3_1 => 0x3F3E3CB,
        Version::ER1_3_2 => 0x41966EE,
        Version::ER1_4_0 => 0x3E58205,
        Version::ER1_4_1 => 0x3E713F3,
        Version::ER1_5_0 => 0x3DCEF33,
        Version::ER1_6_0 => 0x3E36AE2,
        Version::ER1_7_0 => 0x3FDBABC,
        Version::ER1_8_0 => 0x3E513D8,
        Version::ER1_8_1 => 0x3ED69A2,
        Version::ER1_9_0 => 0x3E6613E,
        Version::ER1_9_1 => 0x3F024D7,
        Version::ER2_0_0 => 0x3E69BA3,
        Version::ER2_0_1 => 0x3EB3B8F,
        Version::ER2_2_0 => 0x4006422,
        Version::ER2_2_3 => 0x3FF0A97,
        Version::ER2_3_0 => 0x3EEB4C2,
        Version::ER2_4_0 => 0x43FB066,
        Version::ER2_5_0 => 0x41B52E2,
        Version::ER2_6_0 => 0x416D7DA,
        Version::ER2_6_1 => 0x3F626D3,
        _ => 0x0,
    }
}

pub const TARGET_POINTER: u64 = 0x0;                    // u64
pub const TARGET_HANDLE: u64 = 0x8;                     // u64
pub const LOOKED_UP_CHR_INS: u64 = 0x10;                // u64

pub const ACT_ARRAY: u64 = 0x20;                        // [i32, 10]
pub const CURRENT_IDX: u64 = 0x48;                      // i32
pub const SHOULD_RUN: u64 = 0x4C;                       // u8

pub const EMEVD_ARGS: u64 = 0x50;                       // [u8; 40]
pub const EVENT_RESULT: u64 = 0x78;                     // u8

pub const WARP_COORDS: u64 = 0x80;                      // [f32; 4]
pub const WARP_ANGLE: u64 = 0x90;                       // [f32; 2]

pub const EZ_STATE_TALK_PARAMS: u64 = 0xA0;             // [i32; 10]

pub const ITEM_SPAWN_STRUCT: u64 = 0xC0;                // [u8; 96]
pub const MAX_QUANTITY: u64 = 0x120;                    // i32
pub const SHOULD_CHECK_QUANTITY: u64 = 0x124;           // u8

pub const STATE_HANDLER_FLAGS: u64 = 0xF00;             // [u8; 256]

// Hooks
pub const TARGET_POINTER_HOOK: u64 = 0x1000;                // [u8; 19]
pub const TARGET_NO_STAGGER_HOOK: u64 = 0x1020;          // [u8; 28]
pub const FORCE_ACT_SEQUENCE_HOOK: u64 = 0x1040;         // [u8; 78]
pub const INFINITE_POISE_HOOK: u64 = 0x1090;             // [u8; 111]
pub const NO_GRAB_HOOK: u64 = 0x1100;                    // [u8; 40]
pub const WARP_COORDS_HOOK: u64 = 0x1130;                // [u8; 19]
pub const WARP_ANGLE_HOOK: u64 = 0x1150;                 // [u8; 19]

// Shellcode
pub const RUN_THREAD_ASM: u64 = 0x2000;                 // [u8; 39]
// Keep at least 16 bytes of buffer
// for completion flag and appended asm bytes
pub const RUNE_GIVE_ASM: u64 = 0x2030;                  // [u8; 41]
pub const GRACE_WARP_ASM: u64 = 0x2050;                 // [u8; 49]
pub const BLOCK_WARP_ASM: u64 = 0x20A0;                 // [u8; 43]
pub const ITEM_SPAWN_ASM: u64 = 0x20D0;                 // [u8; 80]
pub const SET_EVENT_ASM: u64 = 0x2130;                  // [u8; 51]
pub const GET_EVENT_ASM: u64 = 0x2180;                  // [u8; 53]
pub const EZ_STATE_TALK_ASM: u64 = 0x21D0;              // [u8; 167]
pub const EMEVD_ASM: u64 = 0x2290;                      // [u8; 224]
pub const CHR_INS_FROM_ENTITY_ID_ASM: u64 = 0x2390;     // [u8; 55]
pub const SET_SPEFFECT_ASM: u64 = 0x23E0;               // [u8; 41]
pub const REMOVE_SPEFFECT_ASM: u64 = 0x2410;            // [u8; 41]
