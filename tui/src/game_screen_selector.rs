use {
    crate::{
        event::{Event, KeyContext, send_event},
        panes::{TableController, TablePane, TableView},
        popup::{Popup, PopupState, centered_popup},
        screen::Screen,
        ui_state::UiState,
    },
    config::Config,
    crossterm::event::KeyCode,
    gubtool_core::game_version::Game,
    ratatui::{Frame, layout::Rect, widgets::Row},
};

const GAMES: [Game; 2] = [Game::DarkSouls2, Game::EldenRing];

pub struct GameScreenSelector {
    list:        TablePane,
    popup_state: PopupState,
}

impl Popup for GameScreenSelector {
    fn screen(&mut self) -> &mut dyn Screen {
        &mut self.list
    }
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        centered_popup(50, 50, frame.area())
    }
    fn close_on_key(&self, ctx: &mut KeyContext) -> bool {
        ctx.key(KeyCode::Esc) || ctx.key_char('q') || ctx.key_enter()
    }
}

struct GameList;
impl TableController for GameList {
    fn make_table_view(&self) -> TableView {
        let items = GAMES
            .iter()
            .map(|game| Row::new([format!("{}", game)]))
            .collect();
        TableView::new(items)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.peek_code() == Some(KeyCode::Enter) {
            let game = GAMES[selected];
            send_event(Event::GameScreen(game));
            let _ = UiState::update(|c| c.global.game_screen = game);
        }
    }
}

impl GameScreenSelector {
    pub fn new() -> Self {
        Self {
            list:        TablePane::new_static(&GameList)
                .freeze()
                .with_title("Select Game Screen"),
            popup_state: PopupState::default(),
        }
    }
}
