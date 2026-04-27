use crate::{
    config::{Config, ui_state::UiState},
    tui::common::{block, centered_rect, tab_state::TabState},
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    style::{Modifier, Style},
    widgets::{BorderType, Clear, List, ListItem, ListState},
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
    list: ListState,
}

impl ThemeSelector {
    pub fn new() -> Self {
        Self {
            list: ListState::default().with_selected(Some(0)),
        }
    }
    pub fn draw(&mut self, frame: &mut Frame, current_theme: &ThemeName) {
        let layout = centered_rect(75, 75, frame.area());
        frame.render_widget(Clear, layout);
        frame.render_stateful_widget(Self::themes_list(current_theme), layout, &mut self.list);
    }
    pub fn handle_keys(&mut self, key: KeyEvent, current_theme: &mut ThemeName) {
        match (key.code, key.modifiers) {
            (KeyCode::Char('j') | KeyCode::Down, _) => self.list.select_next(),
            (KeyCode::Char('k') | KeyCode::Up, _) => self.list.select_previous(),
            (KeyCode::Enter, _) => {
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
        List::new(
        ThemeName::all()
            .iter()
            .map(|theme| {
                let name = if selected_theme == theme {
                    format!("*{}", theme.display_name())
                } else {
                    format!(" {}", theme.display_name())
                };
                ListItem::new(name)
            })
            .collect::<Vec<ListItem>>())
            .block(block(Some("Themes"), None))
            .highlight_style(Style::from(theme().accent).bold())
            .highlight_symbol(HIGHLIGHT_SYMBOL)
    }
}

impl TabState {
    pub fn block_style(&self, list_idx: usize) -> Style {
        if self.current_list == list_idx {
            Style::new().fg(theme().fg)
        } else {
            Style::new().fg(theme().fg).add_modifier(Modifier::DIM)
        }
    }

    pub fn highlight_style(&self, list_idx: usize) -> Style {
        if self.current_list == list_idx {
            Style::from(theme().accent).bold()
        } else {
            Style::from(theme().accent).bold().add_modifier(Modifier::DIM)
        }
    }
}
