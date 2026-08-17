use gubtool_core::{address::Address, attached::module_base};

pub const SIZE: usize = 0x8000;
pub const BASE_OFFSET: u64 = 0x4000000;

#[repr(u64)]
#[derive(Clone, Copy)]
pub enum CaveAddress {
    WorkerThreadPort     = 0x0, // u16

    SavedTargetPointer   = 0x10, // u64
    SavedHandle          = 0x18, // u64
    LookedUpEntityId     = 0x20, // u64

    ActArray             = 0x30, // 0x28
    CurrentActIdx        = 0x58, // i32
    ActSeqeunceShouldRun = 0x5c, // u8

    EmevdArgs            = 0x60, // 0x28

    WarpCoords           = 0x90, // 0x16
    WarpAngle            = 0xa0, // 0x8

    EzStateParams        = 0xb0, // 0x28

    ItemSpawnStruct      = 0xe0,  // 0x60
    MaxQuantity          = 0x140, // i32
    ShouldCheckQuantity  = 0x144, // u8

    DisableRollFlag      = 0x150, // u8
    DisableJumpFlag      = 0x151, // u8
    DisableBackstepFlag  = 0x152, // u8

    StateHandlerFlags    = 0xf00, // 0x100
    // Hooks
    SaveTargetHook       = 0x1000, // 0x1D
    TargetNoStaggerHook  = 0x1020, // 0x22
    ForceActSequenceHook = 0x1050, // 0x5C
    InfinitePoiseHook    = 0x10b0, // 0x82
    NoGrabHook           = 0x1140, // 0x2E
    WarpCoordsHook       = 0x1170, // 0x13
    WarpAngleHook        = 0x1190, // 0x13
    ActionHook           = 0x11b0, // 0x32
    EventLogHook         = 0x11f0, // 0x44

    RunThreadAsm         = 0x2001, // 0x60

    DllPath              = 0x3000, // 0x208
    DllInjectCode        = 0x3210, // 0x22
    CustomFunction       = 0x3250, // 0x500

    EventLogWriteIdx     = 0x3750, // i32
    EventLogBuffer       = 0x3754, // 0x1000
}

impl Address for CaveAddress {
    fn addr(&self) -> u64 {
        module_base()
            .saturating_add(BASE_OFFSET)
            .saturating_add(*self as u64)
    }
}
