#[macros::code_cave(0x4ccf4ed)]
pub enum CaveAddr {
    WorkerThreadPort = 0x0,   // u16
    RunThreadAsm     = 0x10,  // 0x60
    DllPath          = 0x100, // 0x208
    DllInjectCode    = 0x400, // 0x22
    CustomFunction   = 0x450, // 0x500

    WarpCoords       = 0x900, // 0x10
    WarpAngle        = 0x910, // 0x10

    ItemArgs         = 0x920, // 0x18
    ItemSpawnOutput  = 0x940, // 0x10

    TalkCommandArgs  = 0x950, // 0x10

    SavedTargetPointer = 0x960, // u64

    // Hooks
    EventLogHook     = 0x1000, // 0x43
    WarpCoordsHook   = 0x1050, // 0x21
    WarpAngleHook    = 0x1100, // 0x21
    SaveTargetHook   = 0x1130,

    EventLogWriteIdx = 0x3750, // i32
    EventLogBuffer   = 0x3754, // 0x1000
}
