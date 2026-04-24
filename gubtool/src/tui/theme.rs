use crossterm::event::{KeyCode, KeyEvent};
use std::sync::{OnceLock, RwLock};
use ratatui::{
    Frame,
    style::{Modifier, Style},
    widgets::{BorderType, Clear, List, ListItem}
};
use ratatui_themes::{ThemeName, ThemePalette};
use crate::{
    config::{Config, UiState},
    tui::{
        app::{App, CurrentScreen},
        tabs::TabState,
        block, centered_rect,
    }
};

pub static THEME: OnceLock<RwLock<ThemePalette>> = OnceLock::new();

pub fn theme() -> std::sync::RwLockReadGuard<'static, ThemePalette> {
    THEME.get().unwrap().read().unwrap()
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let layout = centered_rect(75, 75, frame.area());
    frame.render_widget(Clear, layout);
    frame.render_stateful_widget(themes_list(&app.theme), layout, &mut app.theme_list);
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Char('j') | KeyCode::Down, _) => app.theme_list.select_next(),
        (KeyCode::Char('k') | KeyCode::Up, _) => app.theme_list.select_previous(),
        (KeyCode::Char('q') | KeyCode::Esc, _) => app.current_screen = CurrentScreen::Tab,
        (KeyCode::Enter, _) => {
            if let Some(idx) = app.theme_list.selected() {
                let theme = ThemeName::all()[idx];
                app.theme = theme;
                UiState::update(|c| { c.theme = theme; }).ok();
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

pub const HIGHLIGHT_SYMBOL: &str = "> ";
pub const BORDER_TYPE: BorderType = BorderType::Rounded;


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
