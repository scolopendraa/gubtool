mod event_tab;
mod items_tab;
mod player_tab;
mod travel_tab;
mod utility_tab;

use crate::{
    common::tabs::TabManager,
    darksouls3_screen::{
        event_tab::EventTab,
        items_tab::ItemTab,
        player_tab::PlayerTab,
        travel_tab::TravelTab,
        utility_tab::UtilityTab,
    },
    impl_tab,
    screen::Screen,
};

impl_tab!(PlayerTab, UtilityTab, ItemTab, TravelTab, EventTab);

pub struct DarkSouls3Screen {
    pub tab_manager: TabManager,
}

impl DarkSouls3Screen {
    pub fn new() -> Self {
        Self {
            tab_manager: TabManager::new(
                "Dark Souls III",
                &["Player", "Utility", "Items", "Travel", "Events"],
                vec![
                    PlayerTab::new().boxed(),
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
        format!("is player loaded: {}", darksouls3::is_player_loaded()),
        format!("stats: {:?}", darksouls3::player::player_game_data()),
    ]
}
