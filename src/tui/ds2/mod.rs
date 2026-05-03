mod items_tab;
mod player_tab;
mod travel_tab;
mod utility_tab;

use crate::{ds2::game_state::GameStateHandler, tui::{
    common::tabs_widget::TabsWidget,
    ds2::{items_tab::ItemTab, player_tab::PlayerTab, travel_tab::TravelTab, utility_tab::UtilityTab}, event::ResultExt,
}};
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
pub struct DarkSouls2 {
    tabs_widget: TabsWidget,
    game_state: GameStateHandler,
    player: PlayerTab,
    utility: UtilityTab,
    items: ItemTab,
    travel: TravelTab,
}

impl DarkSouls2 {
    pub fn new() -> Self {
        Self {
            tabs_widget: TabsWidget {
                current_tab: 0,
                title: "Dark Souls II",
                tabs: &["Player", "Target", "Utility", "Items", "Travel"],
            },
            game_state: GameStateHandler::new(),
            player: PlayerTab::new(),
            utility: UtilityTab::new(),
            items: ItemTab::new(),
            travel: TravelTab::new(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let layout = self.tabs_widget.draw(frame, layout);

        match self.tabs_widget.tabs[self.tabs_widget.current_tab as usize] {
            "Player" => self.player.draw(frame, layout),
            "Target" => (),
            "Utility" => self.utility.draw(frame, layout),
            "Items" => self.items.draw(frame, layout),
            "Travel" => self.travel.draw(frame, layout),
            _ => (),
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.tabs_widget.handle_keys(key);

        match self.tabs_widget.tabs[self.tabs_widget.current_tab as usize] {
            "Player" => self.player.handle_keys(key),
            "Target" => (),
            "Utility" => self.utility.handle_keys(key),
            "Items" => self.items.handle_keys(key),
            "Travel" => self.travel.handle_keys(key),
            _ => (),
        }
    }

    pub fn background_tick(&mut self) {
        self.game_state.poll().send_error();
    }

    pub fn render_tick(&mut self) {
    }

    pub fn on_unattach(&mut self) {
    }

    pub fn on_attach(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}