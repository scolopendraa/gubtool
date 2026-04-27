mod elden_beast_map;
mod event_tab;
mod items_tab;
mod player_tab;
mod target_tab;
mod travel_tab;
mod utility_tab;

use crate::{config::preferences::Preferences, tui::common::tabs_widget::TabsWidget};
use anyhow::anyhow;
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use {
    crate::{
        er::{chr_ins::ChrIns, game_state::GameStateHandler, player, target},
        tui::er::elden_beast_map::EldenBeastMap,
    },
    event_tab::EventTab,
    items_tab::ItemTab,
    player_tab::PlayerTab,
    target_tab::TargetTab,
    travel_tab::TravelTab,
    utility_tab::UtilityTab,
};
pub struct EldenRing {
    tabs_widget: TabsWidget,
    pub er_info: ErInfo,
    pub game_state: GameStateHandler,
    pub player: PlayerTab,
    pub target: TargetTab,
    pub item: ItemTab,
    pub utility: UtilityTab,
    pub travel: TravelTab,
    pub event: EventTab,
    pub elden_beast_map: EldenBeastMap,
}

pub struct ErInfo {
    dlc: bool,
    loaded: bool,
    player_ins: ChrIns,
    target_ins: ChrIns,
}

impl EldenRing {
    pub fn new() -> Self {
        Self {
            tabs_widget: TabsWidget {
                current_tab: 0,
                title: "Elden Ring",
                tabs: &["Player", "Target", "Utility", "Items", "Travel", "Events"],
            },
            er_info: ErInfo {
                dlc: true,
                loaded: false,
                player_ins: Err(anyhow!("")),
                target_ins: Err(anyhow!("")),
            },
            game_state: GameStateHandler::new(),
            player: PlayerTab::new(),
            target: TargetTab::new(),
            utility: UtilityTab::new(),
            item: ItemTab::new(),
            travel: TravelTab::new(),
            event: EventTab::new(),
            elden_beast_map: EldenBeastMap::default(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let layout = self.tabs_widget.draw(frame, layout);

        match self.tabs_widget.tabs[self.tabs_widget.current_tab as usize] {
            "Player" => self.player.draw(frame, layout, &self.er_info),
            "Target" => self.target.draw(frame, layout, &self.er_info),
            "Utility" => self.utility.draw(frame, layout, &self.er_info),
            "Items" => self.item.draw(frame, layout, &self.er_info),
            "Travel" => self.travel.draw(frame, layout, &self.er_info),
            "Events" => self.event.draw(frame, layout, &self.er_info),
            _ => (),
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.tabs_widget.handle_keys(key);

        match self.tabs_widget.tabs[self.tabs_widget.current_tab as usize] {
            "Player" => self.player.handle_keys(key, &self.er_info),
            "Target" => self.target.handle_keys(key, &self.er_info),
            "Utility" => self.utility.handle_keys(key, &self.er_info),
            "Items" => self.item.handle_keys(key, &self.er_info),
            "Travel" => self.travel.handle_keys(key, &self.er_info),
            "Events" => self.event.handle_keys(key, &self.er_info),
            _ => (),
        }
    }

    pub fn background_tick(&mut self) {
        self.game_state.poll().ok();
        self.er_info.dlc = self.game_state.dlc;
        self.er_info.loaded = self.game_state.loaded;
        self.er_info.player_ins = player::player_ins();
    }

    pub fn render_tick(&mut self) {
        self.er_info.target_ins = target::target_ins();
    }

    pub fn on_unattach(&mut self) {
        self.game_state = GameStateHandler::new();
    }

    pub fn on_attach(&mut self) -> anyhow::Result<()> {
        self.game_state = GameStateHandler::new();
        target::install_target_hook()?;
        Preferences::apply_elden_ring()
    }
}