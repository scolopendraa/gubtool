use macros::base_address;

pub mod scan;
mod r#struct;
mod versions;

#[base_address]
pub enum BasePointer {
    WorldObjManImpl,
    WorldChrManImp,
    GameMan,
    GameDataMan,
    MenuMan,
    MapItemMan,
    EventFlagMan,
    WorldAiManager,
    CsDlc,
    SprjFlipperImp,
}

#[base_address]
pub enum Function {
    SetEventFlag,
    BonfireWarp,
    GetItemQuantity,
    ItemSpawn,
    EzStateExternalEventCtor,
    ExecuteTalkCommand,
}

#[base_address]
pub enum Hook {
    WarpCoords,
    WarpAngle,
    LockedTargetPointer,
}

#[base_address]
pub enum Patch {
    DisableLogos,
    GameSpeed,
    CameraRotation,
    FullShop,
}

#[base_address]
pub enum Data {
    DebugFlags,
}

#[base_address]
pub enum ExternalFunctionPointer {
    Kernel32CreateThread,
    Kernel32CloseHandle,
    Kernel32LoadLibraryW,
}
