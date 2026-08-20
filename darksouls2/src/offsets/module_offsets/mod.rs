pub mod scan;
mod structs;
mod versions;

use {
    crate::offsets::module_offsets::structs::module_offsets,
    gubtool_core::{address::Address, attached::module_base, impl_address_patch},
};

#[derive(Clone, Copy)]
pub enum BasePointer {
    GameManagerImp,
    KatanaMainApp,
}

#[derive(Clone, Copy)]
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
}

#[derive(Clone, Copy)]
pub enum Hook {
    LockedTargetPointer,
    PlayerNoDamage,
    InfinitePoise,
    CreditsSkip,
    FasterMenu,
    SetSharedFlag,
    EventLog,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum Data {
    MapId,
}

#[derive(Clone, Copy)]
pub enum ExternalFunctionPointer {
    Kernel32CreateThread,
    Kernel32CloseHandle,
    Kernel32Sleep,
    Kernel32LoadLibraryW,
}

impl Address for BasePointer {
    fn addr(&self) -> u64 {
        let f = &module_offsets().base_ptrs;
        let offset = match self {
            Self::GameManagerImp => f.game_manager_imp,
            Self::KatanaMainApp => f.katana_main_app,
        };
        module_base() + offset
    }
}

impl Address for Function {
    fn addr(&self) -> u64 {
        let f = &module_offsets().functions;
        let offset = match self {
            Self::GiveSouls => f.give_souls,
            Self::Warp => f.warp,
            Self::ItemSpawn => f.item_spawn,
            Self::BuildItemDialogue => f.build_item_dialogue,
            Self::ShowItemDialogue => f.show_item_dialogue,
            Self::CurrentItemQuantityCheck => f.current_item_quantity_check,
            Self::SetEvent => f.set_event,
            Self::MapEntityFromMapIdAndObjId => f.map_entity_from_map_id_and_obj_id,
            Self::GetStateActComponent => f.get_state_act_component,
            Self::MakeSound => f.make_sound,
            Self::BonfireRest => f.bonfire_rest,
            Self::BonfireUnlock => f.bonfire_unlock,
            Self::OpenMenu => f.open_menu,
            Self::MenuChrState => f.menu_chr_state,
            Self::LevelUp => f.level_up,
            Self::LevelLookup => f.level_lookup,
            Self::ChrSetAction => f.chr_set_action,
            Self::EzStateExternalEventCtor => f.ez_state_external_event_ctor,
            Self::EzStateExecuteEvent => f.ez_state_execute_event,
        };
        module_base() + offset
    }
}

impl Address for Hook {
    fn addr(&self) -> u64 {
        let f = &module_offsets().hooks;
        let offset = match self {
            Self::LockedTargetPointer => f.locked_target_pointer,
            Self::PlayerNoDamage => f.player_no_damage,
            Self::InfinitePoise => f.infinite_poise,
            Self::CreditsSkip => f.credits_skip,
            Self::FasterMenu => f.faster_menu,
            Self::SetSharedFlag => f.set_shared_flag,
            Self::EventLog => f.event_log,
        };
        module_base() + offset
    }
}

impl Address for Patch {
    fn addr(&self) -> u64 {
        let f = &module_offsets().patches;

        let offset = match self {
            Self::InfiniteStamina => f.infinite_stamina,
            Self::InfiniteConsumables => f.infinite_consumables,
            Self::InfiniteDurability => f.infinite_durability,
            Self::InfiniteCasts => f.infinite_casts,
            Self::NoSoulGain => f.no_soul_gain,
            Self::NoHollowing => f.no_hollowing,
            Self::NoSoulLoss => f.no_soul_loss,
            Self::PlayerHidden => f.player_hidden,
            Self::PlayerSilent => f.player_silent,
            Self::SkipLogos => f.skip_logos,
            Self::MenuTransition => f.menu_transition,
            Self::NoRoll => f.no_roll,
            Self::NoBackstep => f.no_backstep,
        };
        module_base() + offset
    }
}

impl Address for Data {
    fn addr(&self) -> u64 {
        let f = &module_offsets().data;
        let offset = match self {
            Self::MapId => f.map_id,
        };
        module_base() + offset
    }
}

impl Address for ExternalFunctionPointer {
    fn addr(&self) -> u64 {
        let f = &module_offsets().external_fn_ptrs;
        let offset = match self {
            Self::Kernel32CreateThread => f.kernel32_create_thread,
            Self::Kernel32CloseHandle => f.kernel32_close_handle,
            Self::Kernel32Sleep => f.kernel32_sleep,
            Self::Kernel32LoadLibraryW => f.kernel32_load_library_w,
        };
        module_base() + offset
    }
}

impl_address_patch!(BasePointer, Function, Hook, Patch, Data, ExternalFunctionPointer);
