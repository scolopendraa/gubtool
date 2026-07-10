pub mod controls;
pub mod event_log_table;
pub mod item_options;
pub mod stateful_list;
pub mod tab_state;
pub mod tabs_widget;

pub use item_options::*;

use crate::{
    common::tab_state::TabState,
    theme::{self, theme},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, TableState},
};
use ratatui_themes::Style;
use std::fmt::Display;

pub trait StrExt {
    fn create_toggle_str(self, val: bool) -> String;
}

impl StrExt for &str {
    fn create_toggle_str(self, val: bool) -> String {
        let ret = match val {
            true => "[X]",
            false => "[ ]",
        };
        format!("{ret} {self}")
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, layout: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(layout);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn block<'a>(title: Option<&'a str>, style: Option<Style>) -> Block<'a> {
    Block::bordered()
        .title(Line::from(title.unwrap_or("")).style(Style::from(theme().secondary)))
        .style(style.unwrap_or(Style::new().fg(theme().fg)))
        .bg(theme().bg)
        .border_type(theme::BORDER_TYPE)
}

pub fn list<'a>(items: Vec<ListItem<'a>>, title: Option<&'a str>) -> List<'a> {
    List::new(items)
        .block(block(title, None))
        .highlight_style(Style::from(theme().accent).bold())
        .highlight_symbol(theme::HIGHLIGHT_SYMBOL)
}

pub fn tabs_list<'a>(items: Vec<ListItem<'a>>, title: Option<&'a str>, tabstate: &TabState, list_idx: usize) -> List<'a> {
    List::new(items)
        .block(block(title, Some(tabstate.block_style(list_idx))))
        .highlight_style(tabstate.highlight_style(list_idx))
        .highlight_symbol(theme::HIGHLIGHT_SYMBOL)
}

pub fn blockless_list<'a>(items: Vec<ListItem<'a>>, tabstate: &TabState, list_idx: usize) -> List<'a> {
    List::new(items)
        .style(tabstate.block_style(list_idx))
        .highlight_style(tabstate.highlight_style(list_idx))
        .highlight_symbol(theme::HIGHLIGHT_SYMBOL)
}

pub fn label_list<'a>(items: Vec<ListItem<'a>>, tabstate: &TabState, list_idx: usize) -> List<'a> {
    List::new(items)
        .block(Block::default().borders(Borders::LEFT))
        .highlight_style(tabstate.highlight_style(list_idx))
}

pub trait ListExt {
    fn handle_keys(&mut self, key: KeyEvent);
}

macro_rules! impl_handle_keys {
    ($t:ty) => {
        impl ListExt for $t {
            fn handle_keys(&mut self, key: KeyEvent) {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                        for _ in 0..28 { self.select_previous() }
                    }
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        for _ in 0..28 { self.select_next() }
                    }
                    (KeyCode::Char('j') | KeyCode::Down, _) => self.select_next(),
                    (KeyCode::Char('k') | KeyCode::Up, _) => self.select_previous(),
                    (KeyCode::Char('G'), _) => self.select_first(),
                    (KeyCode::Char('g'), _) => self.select_last(),
                    _ => ()
                }
            }
        }
    };
}
impl_handle_keys!(TableState);

pub fn draw_popup_selector(title: &'static str, items: &[impl Display], state: &mut ListState, frame: &mut Frame) {
    let rect = centered_rect(50, 50, frame.area());
    let list_items = items.iter().map(|item| {
        ListItem::new(item.to_string())
    }).collect();
    let list = list(list_items, Some(title));
    frame.render_widget(Clear, rect);
    frame.render_stateful_widget(list, rect, state);

}