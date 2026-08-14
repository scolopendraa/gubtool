mod event_tab;
mod items_tab;
mod player_tab;
mod target_tab;
mod travel_tab;
mod utility_tab;

use {
    crate::{
        common::tabs::TabManager,
        darksouls2_screen::{
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
    darksouls2::{bonfire, player, target, utility},
};

impl_tab!(PlayerTab, TargetTab, ItemTab, UtilityTab, TravelTab, EventTab);

pub struct DarkSouls2Screen {
    pub tab_manager: TabManager,
}

impl DarkSouls2Screen {
    pub fn new() -> Self {
        Self {
            tab_manager: TabManager::new(
                "Dark Souls II",
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
        format!("loaded: {}", darksouls2::is_player_loaded()),
        format!("area id: {:#X}", utility::get_area_id().unwrap_or_default()),
        format!("player coords: {:?}", player::position().unwrap_or_default()),
        format!(
            "player quaternion: {:?}",
            player::player()
                .chr_ctrl()
                .and_then(|chr| chr.rot_quaternion())
                .unwrap_or_default()
        ),
        format!("last rested bonfire id: {}", bonfire::get_last_bonfire_id().unwrap_or_default()),
    ]
}
