use crate::{
    config::{Config, ui_state::UiState},
    core::attach::Game,
    tui::{
        app::CurrentScreen,
        common::{centered_rect, list, stateful_list::StatefulList},
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    widgets::{Clear, List, ListItem},
};


const GAMES: &[Game] = &[
    Game::DarkSoulsII,
    Game::EldenRing,
];
pub struct GameScreenSelector {
    list: StatefulList,
}

impl GameScreenSelector {
    pub fn new() -> Self {
        Self {
            list: StatefulList::new(GAMES.len()),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let layout = centered_rect(40, 40, frame.area());
        frame.render_widget(Clear, layout);

        frame.render_stateful_widget(Self::list(), layout, &mut self.list.state);
    }
    fn list() -> List<'static> {
        let items = GAMES.iter()
        .map(|game| ListItem::new(format!("{}", game)) )
        .collect();
        list(items, Some("Game Screen Selection"))
    }

    pub fn handle_keys(&mut self, key: KeyEvent, game_screen: &mut Game, current_screen: &mut CurrentScreen) {
        self.list.handle_keys(key);
        match (key.code, key.modifiers) {
            (KeyCode::Char('q') | KeyCode::Esc, _) => *current_screen = CurrentScreen::Game,
            (KeyCode::Enter, _) => {
                if let Some(idx) = self.list.selected() {
                    let game = GAMES[idx];
                    *game_screen = game;
                    let _ = UiState::update(|c| c.global.game_screen = game );
                    *current_screen = CurrentScreen::Game
                }
            }
            _ => (),
        }
    }
}