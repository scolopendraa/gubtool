use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::ListState;

#[derive(Clone)]
pub struct TabState {
    pub current_list: usize,
    pub lists: Vec<ListState>,
    pub list_sizes: Vec<usize>,
}

impl Default for TabState {
    fn default() -> Self {
        Self {
            current_list: 0,
            lists: vec![ListState::default().with_selected(Some(0)); 2],
            list_sizes: vec![0; 2],
        }
    }
}

impl TabState {
    pub fn handle_keys(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => self.decrement_current_list(28),
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => self.increment_current_list(28),

            (KeyCode::Char('h') | KeyCode::Left, KeyModifiers::CONTROL) => self.current_list = 0,
            (KeyCode::Char('l') | KeyCode::Right, KeyModifiers::CONTROL) => self.current_list = 1,
            (KeyCode::Char('j') | KeyCode::Down, KeyModifiers::CONTROL) => {
                if self.lists.len() > 2 && self.current_list == 1 {
                    self.current_list = 2
                }
            }
            (KeyCode::Char('k') | KeyCode::Up, KeyModifiers::CONTROL) => {
                if self.lists.len() > 2 && self.current_list == 2 {
                    self.current_list = 1
                }
            }
            (KeyCode::Char('j') | KeyCode::Down, _) => self.increment_current_list(1),
            (KeyCode::Char('k') | KeyCode::Up, _) => self.decrement_current_list(1),

            (KeyCode::Char('G'), _) => self.select_last_entry(),
            (KeyCode::Char('g'), _) => self.select_first_entry(),
            _ => ()
        }
    }

    fn decrement_current_list(&mut self, val: usize) {
        if let Some(current) = self.lists[self.current_list].selected() {
            let new_val = current.saturating_sub(val);
            *self.lists[self.current_list].selected_mut() = Some(new_val);
        }
    }

    fn increment_current_list(&mut self, val: usize) {
        if let Some(current) = self.lists[self.current_list].selected() {
            let new_val = current
                .saturating_add(val)
                .min(self.list_sizes[self.current_list].saturating_sub(1));
            *self.lists[self.current_list].selected_mut() = Some(new_val);
        }
    }

    fn select_first_entry(&mut self) {
        *self.lists[self.current_list].selected_mut() = Some(0);
    }

    fn select_last_entry(&mut self) {
        let idx = self.list_sizes[self.current_list].saturating_sub(1);
        *self.lists[self.current_list].selected_mut() = Some(idx);
    }
}