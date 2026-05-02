use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::ListState;

#[derive(Clone)]
pub struct StatefulList {
    pub state: ListState,
    pub size: usize,
}

impl StatefulList {
    pub fn new(size: usize) -> Self {
        StatefulList {
            state: ListState::default().with_selected(Some(0)),
            size,
        }
    }
    pub fn select(&mut self, index: usize) {
        self.state.select(Some(index));
    }
    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
    pub fn increment(&mut self, val: usize) {
        if let Some(idx) = self.state.selected() {
            let new_idx = {
                if idx + val > self.size.saturating_sub(1) {
                    self.size.saturating_sub(1)
                } else {
                    idx + val
                }
            };
            self.select(new_idx);
        }
    }
    pub fn decrement(&mut self, val: usize) {
        if let Some(idx) = self.state.selected() {
            self.select(idx.saturating_sub(val));
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => self.decrement(28),
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => self.increment(28),
            (KeyCode::Char('j') | KeyCode::Down, _) => self.increment(1),
            (KeyCode::Char('k') | KeyCode::Up, _) => self.decrement(1),
            (KeyCode::Char('g'), _) => self.select(0),
            (KeyCode::Char('G'), _) => self.select(self.size.saturating_sub(1)),
            _ => (),
        }
    }
}
