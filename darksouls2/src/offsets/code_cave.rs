use {
    crate::offsets::Offset,
    gubtool_core::{address::Address, attached::module_base},
};

pub const SIZE: usize = 0x5000;

pub const BASE: Offset = Offset {
    vanilla: 0x1250000,
    scholar: 0x1800000,
};

#[repr(u64)]
#[derive(Clone, Copy)]
pub enum CaveAddress {
    WorkerThreadPort      = 0x0, // u16

    SavedTargetPointer    = 0x10, // u64
    ForceActChrAi         = 0x18, // u64
    ForceActId            = 0x20, // i32
    ForceActFlag          = 0x24, // u8
    SavedActBuffer        = 0x25, // 0x50

    ItemArgs              = 0x100, // 0x23
    ItemSpawnStack        = 0x130, // 0x300

    WarpRequestStruct     = 0x430, // 0x40
    CreditsModifyOnceFlag = 0x470, // u8
    OpenMenuArgs          = 0x480, // 0x30
    NpcTalkArgs           = 0x4b0, // 0x30
    NpcPos                = 0x4e0, // 0x20

    LevelUpBuffer         = 0x500, // 0x100
    NegativeFlag          = 0x600, // u8

    EzStateParams         = 0x610, // [i32; 3]
    LookedUpStateActCtrl  = 0x640,

    StateHandlerFlags     = 0xf00, // 0x100
    // Hooks
    PlayerNoDamageHook    = 0x1000, // 0x2C
    InfinitePoiseHook     = 0x1030, // 0x2C
    SaveTargetHook        = 0x1060, // 0x2D
    CreditsSkipHook       = 0x1090, // 0x2A
    FasterMenuHook        = 0x10c0, // 0x1A
    EventLogHook          = 0x10e0, // 0x41
    IvorySkipHook         = 0x1130, // 0xC1
    IvoryKnightsHook      = 0x1210, // 0x24
    TargetActHook         = 0x1240, // 0x98

    RunThreadAsm          = 0x2001, // 0x60

    DllPath               = 0x3000, // 0x208
    DllInjectCode         = 0x3210, // 0x50
    CustomFunction        = 0x3250, // 0x500

    EventLogWriteIdx      = 0x3750, // i32
    EventLogBuffer        = 0x3754, // 0x1000
}

impl Address for CaveAddress {
    fn addr(&self) -> u64 {
        module_base()
            .saturating_add(BASE.resolve())
            .saturating_add(*self as u64)
    }
}

pub mod item_args_offsets {
    pub const ADJUST_QUANTITY_FLAG: u64 = 0x2; // u8
    pub const MAX_QUANTITY: u64 = 0x3; // i32
    pub const ITEM_COUNT: u64 = 0x7; // i32
    pub const CURRENT_QUANTITY: u64 = 0xb; // i32
    pub const STACK_COUNT: u64 = 0xf; // i32
    pub const ITEM_STRUCT: u64 = 0x13; // 0x16
}
pub mod item_struct_offsets {
    pub const ITEM_ID: u64 = 0x4; // i32
    pub const DURABILITY: u64 = 0x8; // f32
    pub const QUANTITY: u64 = 0xc; // i16
    pub const UPGRADE: u64 = 0xe; // u8
    pub const INFUSION: u64 = 0xf; // u8
}
