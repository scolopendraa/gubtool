mod elden_beast_map;
mod event_tab;
mod items_tab;
mod player_tab;
mod presets_tab;
mod target_tab;
mod travel_tab;
mod utility_tab;

use crate::{
    common::tabs_widget::TabsWidget,
    eldenring_screen::{
        elden_beast_map::EldenBeastMap, event_tab::EventTab, items_tab::ItemTab,
        player_tab::PlayerTab, presets_tab::PresetsTab, target_tab::TargetTab,
        travel_tab::TravelTab, utility_tab::UtilityTab,
    },
};
use eldenring::travel;
use crossterm::event::KeyEvent;
use eldenring::{
    chr_ins::{ChrIns, ChrInsExt},
    game_state::{GameStateHandler, StateFlags},
    player, target,
};
use ratatui::{Frame, layout::Rect};

pub struct EldenRing {
    tabs_widget: TabsWidget,
    game_state: GameStateHandler,
    player: PlayerTab,
    target: TargetTab,
    items: ItemTab,
    utility: UtilityTab,
    travel: TravelTab,
    presets: PresetsTab,
    event: EventTab,
    elden_beast_map: EldenBeastMap,
}

static mut GAME_STATE: GameState = {
    GameState {
        dlc: true,
        loaded: false,
        target_ins: Ok(0),
        player_ins: Ok(0),
        state_flags: StateFlags::const_default(),
    }
};

struct GameState {
    dlc: bool,
    loaded: bool,
    player_ins: ChrIns,
    target_ins: ChrIns,
    state_flags: StateFlags,
}

impl GameState {
    pub fn dlc() -> bool {
        unsafe { GAME_STATE.dlc }
    }
    pub fn loaded() -> bool {
        unsafe { GAME_STATE.loaded }
    }
    pub fn target_ins() -> &'static ChrIns {
        unsafe {
            let ptr: *const ChrIns = std::ptr::addr_of!(GAME_STATE.target_ins);
            &*ptr
        }
    }
    pub fn player_ins() -> &'static ChrIns {
        unsafe {
            let ptr: *const ChrIns = std::ptr::addr_of!(GAME_STATE.player_ins);
            &*ptr
        }
    }
    pub fn state_flags() -> StateFlags {
        unsafe { GAME_STATE.state_flags }
    }
}

impl EldenRing {
    pub fn new() -> Self {
        Self {
            tabs_widget: TabsWidget {
                current_tab: 0,
                title: Some("Elden Ring"),
                tabs: &["Player", "Target", "Utility", "Items", "Travel", "Presets", "Events"],
            },
            game_state: GameStateHandler::new(),
            player: PlayerTab::new(),
            target: TargetTab::new(),
            utility: UtilityTab::new(),
            items: ItemTab::new(),
            travel: TravelTab::new(),
            presets: PresetsTab::new(),
            event: EventTab::new(),
            elden_beast_map: EldenBeastMap::default(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let layout = self.tabs_widget.draw(frame, layout);

        match self.tabs_widget.tabs[self.tabs_widget.current_tab as usize] {
            "Player" => self.player.draw(frame, layout),
            "Target" => self.target.draw(frame, layout),
            "Utility" => self.utility.draw(frame, layout),
            "Items" => self.items.draw(frame, layout),
            "Travel" => self.travel.draw(frame, layout),
            "Presets" => self.presets.draw(frame, layout),
            "Events" => self.event.draw(frame, layout),
            _ => (),
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent, block_inputs: bool) {
        match self.tabs_widget.tabs[self.tabs_widget.current_tab as usize] {
            "Player" => self.player.handle_keys(key),
            "Target" => self.target.handle_keys(key),
            "Utility" => self.utility.handle_keys(key),
            "Items" => self.items.handle_keys(key),
            "Travel" => self.travel.handle_keys(key),
            "Presets" => self.presets.handle_keys(key),
            "Events" => self.event.handle_keys(key),
            _ => (),
        }

        if block_inputs { return; }

        self.tabs_widget.handle_keys(key);
    }

    pub fn background_tick(&mut self) {
        let _ = self.game_state.poll();

        unsafe {
            GAME_STATE.dlc = self.game_state.dlc;
            GAME_STATE.loaded = self.game_state.loaded;
            GAME_STATE.player_ins = player::player_ins();
        }
    }

    pub fn render_tick(&mut self) {
        unsafe {
            GAME_STATE.target_ins = target::target_ins();
            let game_state_ptr: *mut GameState = &raw mut GAME_STATE;
            let _ = (*game_state_ptr).state_flags.update();
        }
    }

    pub fn on_unattach(&mut self) {
        // Clean up any hooks that might still be installed
        let _ = travel::cleanup_warp_hooks();
        unsafe {
            GAME_STATE.dlc = true;
            GAME_STATE.loaded = false;
            GAME_STATE.state_flags = StateFlags::const_default();
        }
    }

    pub fn on_attach(&mut self) -> anyhow::Result<()> {
        self.game_state = GameStateHandler::new();
        target::install_target_hook()?;
        Ok(())
    }
}

pub fn dbg_lines() -> Vec<String> {
    vec![
        format!("player loaded: {}", GameState::loaded()),
        format!("dlc available: {}", GameState::dlc()),
        format!("target pointer: {:#X}", GameState::target_ins().unwrap_or_default()),
        format!("target chr id: {}", GameState::target_ins().chr_id().unwrap_or_default()),
        format!("target entity id: {}", GameState::target_ins().entity_id().unwrap_or_default()),
    ]
}