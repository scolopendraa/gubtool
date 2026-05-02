use crate::{
    config::{Config, ui_state::UiState},
    tui::{
        app::CurrentScreen,
        common::{centered_rect, list, stateful_list::StatefulList},
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    widgets::{BorderType, Clear, List, ListItem},
};
use ratatui_themes::{ThemeName, ThemePalette};
use std::sync::{OnceLock, RwLock};

pub static THEME: OnceLock<RwLock<ThemePalette>> = OnceLock::new();

pub fn theme() -> std::sync::RwLockReadGuard<'static, ThemePalette> {
    THEME.get().unwrap().read().unwrap()
}

pub const HIGHLIGHT_SYMBOL: &str = "> ";
pub const BORDER_TYPE: BorderType = BorderType::Rounded;

pub struct ThemeSelector {
    list: StatefulList,
}

impl ThemeSelector {
    pub fn new() -> Self {
        Self {
            list: StatefulList::new(ThemeName::all().len()),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, current_theme: &ThemeName) {
        let layout = centered_rect(75, 75, frame.area());
        frame.render_widget(Clear, layout);
        frame.render_stateful_widget(Self::themes_list(current_theme), layout, &mut self.list.state);
    }

    pub fn handle_keys(&mut self, key: KeyEvent, current_theme: &mut ThemeName, current_screen: &mut CurrentScreen) {
        self.list.handle_keys(key);
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => *current_screen = CurrentScreen::Game,
            KeyCode::Enter => {
                if let Some(idx) = self.list.selected() {
                    let theme = ThemeName::all()[idx];
                    *current_theme = theme;
                    UiState::update(|c| { c.global.theme = theme; }).ok();
                    *THEME.get().unwrap().write().unwrap() = theme.palette();
                }
            }
            _ => ()
        }
    }
    fn themes_list(selected_theme: &ThemeName) -> List<'static> {
        let items = ThemeName::all().iter()
            .map(|theme| {
                let name = if selected_theme == theme {
                    format!("*{}", theme.display_name())
                } else {
                    format!(" {}", theme.display_name())
                };
                ListItem::new(name)
            }).collect();
        list(items, Some("Theme Selection"))
    }
}
