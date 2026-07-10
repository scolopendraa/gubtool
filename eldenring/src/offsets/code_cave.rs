use gubtool_core::{address::Address, attached::module_base};

pub const SIZE: usize = 0x8000;
pub const BASE_OFFSET: u64 = 0x4000000;

#[repr(u64)]
#[derive(Clone, Copy)]
pub enum CaveOffset {
    SavedTargetPointer = 0x0,                           // u64
    LookedUpHandle = 0x8,                               // u64
    LookedUpEntityId = 0x10,                            // u64

    ActArray = 0x20,                                    // 0x28
    CurrentActIdx = 0x48,                               // i32
    ActSeqeunceShouldRun = 0x4C,                        // u8

    EmevdArgs = 0x50,                                   // 0x28

    WarpCoords = 0x90,                                  // 0x16
    WarpAngle = 0xA0,                                   // 0x8

    EzStateParams = 0xB0,                               // 0x28

    ItemSpawnStruct = 0xE0,                             // 0x60
    MaxQuantity = 0x140,                                // i32
    ShouldCheckQuantity = 0x144,                        // u8

    DisableRollFlag = 0x150,                            // u8
    DisableJumpFlag = 0x151,                            // u8
    DisableBackstepFlag = 0x152,                        // u8

    StateHandlerFlags = 0xF00,                          // 0x100
    // Position save slots (2 slots, each: u32 block_id + [f32;3] coords + f32 angle = 24 bytes)
    SavedPos1 = 0x1200,                                 // 0x18
    SavedPos2 = 0x1218,                                 // 0x18
    // Achievement patch original bytes (7 bytes)
    AchievementPatchOriginal = 0x1230,                  // 0x7
    // Time of day save (4 bytes - f32 value)
    SavedTimeOfDay = 0x1238,                            // 0x4
    // Death detection flag (1 byte)
    DeathFlag = 0x123C,                                 // 0x1
    // No rune loss on death - original patched bytes (6 bytes)
    NoRuneLossPatchOriginal = 0x1240,                   // 0x6
    // Hooks
    SaveTargetHook = 0x1000,                            // 0x1D
    TargetNoStaggerHook = 0x1020,                       // 0x22
    ForceActSequenceHook = 0x1050,                      // 0x5C
    InfinitePoiseHook = 0x10B0,                         // 0x82
    NoGrabHook = 0x1140,                                // 0x2E
    WarpCoordsHook = 0x1170,                            // 0x13
    WarpAngleHook = 0x1190,                             // 0x13
    ActionHook = 0x11B0,                                // 0x32
    EventLogHook = 0x11F0,                              // 0x44
    // Shellcode
    RunThreadAsm = 0x2001,                              // 0x60
    // Keep at least 16 bytes of buffer
    // for completion flag and appended flag setter
    GraceWarpAsm = 0x2070,                              // 0x31
    BlockWarpAsm = 0x20C0,                              // 0x2B
    ItemSpawnAsm = 0x2100,                              // 0x73
    SetEventAsm = 0x2190,                               // 0x30
    GiveRunesAsm = 0x21E0,                              // 0x29
    EzStateTalkAsm = 0x2230,                            // 0xB8
    EmevdAsm = 0x2300,                                  // 0xE0
    ChrInsFromEntityIdAsm = 0x2400,                     // 0x3A
    SetSpeffectAsm = 0x2450,                            // 0x29
    RemoveSpeffectAsm = 0x2490,                         // 0x29

    EventLogWriteIdx = 0x3FFC,                          // i32
    EventLogBuffer = 0x4000,                            // 0x1000
}

impl Address for CaveOffset {
    fn addr(&self) -> u64 {
        module_base()
            .saturating_add(BASE_OFFSET)
            .saturating_add(*self as u64)
    }
}