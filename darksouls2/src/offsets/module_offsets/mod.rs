use macros::base_address;

pub mod scan;
mod r#struct;
mod versions;

#[base_address]
pub enum BasePointer {
    GameManagerImp,
    KatanaMainApp,
}

#[base_address]
pub enum Function {
    GiveSouls,
    Warp,
    ItemSpawn,
    BuildItemDialogue,
    ShowItemDialogue,
    CurrentItemQuantityCheck,
    SetEvent,
    MapEntityFromMapIdAndObjId,
    GetStateActComponent,
    MakeSound,
    BonfireRest,
    BonfireUnlock,
    OpenMenu,
    MenuChrState,
    LevelUp,
    LevelLookup,
    ChrSetAction,
    EzStateExternalEventCtor,
    EzStateExecuteEvent,
    ApplySpeffect,
    TriggerNg,
    ResetEnemy,
}

#[base_address]
pub enum Hook {
    LockedTargetPointer,
    PlayerNoDamage,
    InfinitePoise,
    CreditsSkip,
    FasterMenu,
    SetSharedFlag,
    EventLog,
    DisableAi,
}

#[base_address]
pub enum Patch {
    InfiniteStamina,
    InfiniteConsumables,
    InfiniteDurability,
    InfiniteCasts,
    NoSoulGain,
    NoHollowing,
    NoSoulLoss,
    PlayerHidden,
    PlayerSilent,
    SkipLogos,
    MenuTransition,
    NoRoll,
    NoBackstep,
}

#[base_address]
pub enum Data {
    MapId,
}

#[base_address]
pub enum ExternalFunctionPointer {
    Kernel32CreateThread,
    Kernel32CloseHandle,
    Kernel32Sleep,
    Kernel32LoadLibraryW,
}
