use anyhow::{Result, anyhow, ensure};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, TableState},
};
use ratatui_themes::Style;

use crate::tui::{
    common::tab_state::TabState,
    theme::{self, theme},
};

pub mod stateful_list;
pub mod tab_state;
pub mod tabs_widget;

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


pub fn controls_line(controls: &[(&str, &str)]) -> Line<'static> {
    let mut spans = controls
        .iter()
        .flat_map(|(key, action)| {
            vec![
                Span::raw("[").style(theme().fg),
                Span::raw(key.to_string()).style(theme().info),
                Span::raw("→ ").style(theme().fg),
                Span::raw(action.to_string()).style(theme().fg),
                Span::raw("] ").style(theme().fg),
            ]
        })
        .collect::<Vec<_>>();
    spans.pop();
    spans.push(Span::raw("]").style(theme().fg));
    Line::from(spans)
        .alignment(Alignment::Center)
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


pub fn parse_act_sequence(input: String) -> Result<Vec<i32>> {
    input
        .split_whitespace()
        .map(|s| {
            let val = s
                .parse::<i32>()
                .map_err(|_| anyhow!("Expects integers seperated by spaces"))?;
            ensure!(val <= 50, "Highest act number is 50");
            Ok(val)
        })
        .collect()
}
