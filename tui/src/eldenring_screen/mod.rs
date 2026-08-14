mod elden_beast_map;
mod event_tab;
mod items_tab;
mod player_tab;
mod target_tab;
mod travel_tab;
mod utility_tab;

use {
    crate::{
        common::tabs::TabManager,
        eldenring_screen::{
            event_tab::EventTab,
            items_tab::ItemTab,
            player_tab::PlayerTab,
            target_tab::TargetTab,
            travel_tab::TravelTab,
            utility_tab::UtilityTab,
        },
        impl_tab,
        screen::Screen,
    },
    eldenring::{player, target},
};

impl_tab!(PlayerTab, TargetTab, ItemTab, UtilityTab, TravelTab, EventTab);

pub(super) struct EldenRingScreen {
    pub tab_manager: TabManager,
}

impl EldenRingScreen {
    pub fn new() -> Self {
        Self {
            tab_manager: TabManager::new(
                "Elden Ring",
                &["Player", "Target", "Utility", "Items", "Travel", "Events"],
                vec![
                    PlayerTab::new().boxed(),
                    TargetTab::new().boxed(),
                    UtilityTab::new().boxed(),
                    ItemTab::new().boxed(),
                    TravelTab::new().boxed(),
                    EventTab::new().boxed(),
                ],
            ),
        }
    }
}

pub fn dbg_lines() -> Vec<String> {
    vec![
        format!("target: {:#X?}", target::target()),
        format!("player loaded: {}", eldenring::is_player_loaded()),
        format!("dlc available: {}", eldenring::is_dlc_available()),
        format!(
            "target chr id: {}",
            target::target()
                .chr_ins()
                .and_then(|chr| chr.chr_id())
                .unwrap_or_default()
        ),
        format!(
            "target entity id: {}",
            target::target()
                .chr_ins()
                .and_then(|chr| chr.entity_id())
                .unwrap_or_default()
        ),
        format!(
            "player block: {}",
            player::player()
                .chr_ins()
                .and_then(|chr| chr.block_id())
                .unwrap_or_default()
        ),
        format!(
            "target block: {}",
            target::target()
                .chr_ins()
                .and_then(|chr| chr.block_id())
                .unwrap_or_default()
        ),
    ]
}
