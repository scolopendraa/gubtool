use crate::core::attach::{Version, module_handle, version};

pub const SIZE: usize = 0x5000;

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_3 => 0x0,
        Version::Vanilla1_0_4 => 0x0,
        Version::Vanilla1_0_5 => 0x0,
        Version::Vanilla1_0_6 => 0x0,
        Version::Vanilla1_0_7 => 0x0,
        Version::Vanilla1_0_10 => 0x0,
        Version::Vanilla1_0_11 => 0x117A154,
        Version::Vanilla1_0_12 => 0x114B079,
        Version::Scholar1_0_1 => 0x0,
        Version::Scholar1_0_2 => 0x169B246,
        Version::Scholar1_0_3 => 0x16FA40A,
        _ => 0x0,
    }
}

pub const BONFIRE_ID: u64 = 0x0;                        // i32

pub const EVENT_WARP_PARAMS: u64 = 0x100;               // [u8; 56]
pub const WARP_COORDS: u64 = 0x138;                     // [f32; 16]

pub const ITEM_ARGS: u64 = 0x178;                       // [u8; 35]
pub mod item_args_offsets {
    pub const SHOULD_EXIT_FLAG: u64 = 0x0;              // u8
    pub const SHOULD_PROCESS_FLAG: u64 = 0x1;           // u8
    pub const ADJUST_QUANTITY_FLAG: u64 = 0x2;          // u8
    pub const MAX_QUANTITY: u64 = 0x3;                  // i32
    pub const ITEM_COUNT: u64 = 0x7;                    // i32
    pub const CURRENT_QUANTITY: u64 = 0xB;              // i32
    pub const STACK_COUNT: u64 = 0xF;                   // i32
    pub const ITEM_STRUCT: u64 = 0x13;                  // [u8; 16]
}
pub mod item_struct_offsets {
    pub const ITEM_ID: u64 = 0x4;                       // i32
    pub const DURABILITY: u64 = 0x8;                    // f32
    pub const QUANTITY: u64 = 0xC;                      // i16
    pub const UPGRADE: u64 = 0xE;                       // u8
    pub const INFUSION: u64 = 0xF;                      // u8
}
pub const ITEM_SPAWN_STACK: u64 = 0x19B;                // [u8; 0x300]

pub const STATE_HANDLER_FLAGS: u64 = 0xF00;             // [u8; 256]

// Hooks
pub const WARP_COORDS_HOOK: u64 = 0x1000;               // [u8; 124]
pub const IVORY_SKIP_HOOK: u64 = 0x1080;                // [u8; 219]
pub const IVORY_KNIGHTS_HOOK: u64 = 0x1160;              // [u8; 36]

// Shellcode
pub const RUN_THREAD_ASM: u64 = 0x2000;                 // [u8; 39]
// Keep at least 16 bytes of buffer
// for completion flag and appended asm bytes
pub const SOULS_GIVE_ASM: u64 = 0x2030;                 // [u8; 41]
pub const BONFIRE_WARP_ASM: u64 = 0x2070;               // [u8; 56]
pub const EVENT_WARP_ASM: u64 = 0x20C0;                 // [u8; 37]
pub const ITEM_SPAWN_ASM: u64 = 0x20F0;                 // [u8; 293]

pub const EMPTY_SPACE: u64 = 0x4000;                    //