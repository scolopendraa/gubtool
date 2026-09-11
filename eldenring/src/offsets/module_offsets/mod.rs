use macros::base_address;

pub mod scan;
mod r#struct;
mod versions;

#[base_address]
pub enum BasePointer {
    WorldChrMan,
    FieldArea,
    GameMan,
    GameDataMan,
    MenuMan,
    CsEmkSystem,
    VirtualMemFlag,
    DamageManager,
    WorldAreaTimeImpl,
    MapItemManImpl,
    DlUserInputManagerImpl,
    CsFlipperImp,
    CsDlcImp,
    LockTgtManImp,
    LuaEventSystem,
}

#[base_address]
pub enum Function {
    GraceWarp,
    BlockWarp,
    GetPlayerItemQuantityById,
    ItemSpawn,
    GiveRunes,
    GetEvent,
    SetEvent,
    SetSpeffect,
    RemoveSpeffect,
    GetChrInsByEntityId,
    EmevdSwitch,
    EmkEventInsCtor,
    ExternalEventTempCtor,
    ExecuteTalkCommand,
    AddCoolTime,
}

#[base_address]
pub enum Hook {
    LockedTargetPointer,
    TargetNoStagger,
    PlayerNoGrab,
    PlayerInfinitePoise,
    WarpCoordWrite,
    WarpAngleWrite,
    GetForceActIdx,
    SetRequestedAction,
    NoTimePassOnDeath,
}

#[base_address]
pub enum Patch {
    NoLogo,
    FpsCap,
    MuteMusic,
    PauseWorld,
    TorrentDisabledUnderworld,
    WhistleDisabled,
    OpenMap,
    CloseMap,
    CanFastTravel,
    NoRuneLossOnDeath,
    DisableAreaWelcomeMessage,
    BadQuitMessage,
}

#[base_address]
pub enum Data {
    ChrDbgFlags,
    MapDbgFlags,
}

#[base_address]
pub enum ExternalFunctionPointer {
    Kernel32CreateThread,
    Kernel32CloseHandle,
    Kernel32LoadLibraryW,
}
