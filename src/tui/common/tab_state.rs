use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{style::Modifier, widgets::ListState};
use ratatui_themes::Style;

use crate::tui::{common::stateful_list::StatefulList, theme::theme};

#[derive(Clone)]
pub struct TabState {
    pub current_list: usize,
    pub lists_states: Vec<StatefulList>,
}

impl TabState {
    pub fn new(list_states: Vec<StatefulList>) -> Self {
        Self {
            current_list: 0,
            lists_states: list_states
        }
    }
    pub fn get_list_state(&mut self, idx: usize) -> &mut ListState {
        &mut self.lists_states[idx].state
    }
    pub fn get_list_selected(&self, list_idx: usize) -> Option<usize> {
        self.lists_states[list_idx].selected()
    }
    pub fn set_list_selected(&mut self, list_idx: usize, selected: usize) {
        self.lists_states[list_idx].select(selected)
    }
    pub fn set_length(&mut self, idx: usize, len: usize) {
        self.lists_states[idx].size = len
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            (KeyCode::Char('h') | KeyCode::Left, KeyModifiers::CONTROL) => self.current_list = 0,
            (KeyCode::Char('l') | KeyCode::Right, KeyModifiers::CONTROL) => {
                if !matches!(self.current_list, 2 | 3) {
                    self.current_list = 1
                }
            },
            (KeyCode::Char('j') | KeyCode::Down, KeyModifiers::CONTROL) => {
                if self.lists_states.len() > 2 && self.current_list == 1 {
                    self.current_list = 2
                }
            }
            (KeyCode::Char('k') | KeyCode::Up, KeyModifiers::CONTROL) => {
                if self.lists_states.len() > 2 && self.current_list == 2 {
                    self.current_list = 1
                }
            }
            _ => {
                self.lists_states[self.current_list].handle_keys(key)
            }
        }
    }
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