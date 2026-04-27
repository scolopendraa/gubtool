use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Frame, layout::Rect};
use std::{fmt::Display, ops::RangeBounds};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Clone)]
enum InputChange {
    Insert,
    Append,
    Delete,
}

#[derive(Clone)]
pub struct Input {
    pub text: String,
    pub prompt: String,
    idx: usize,
    offset: usize,
    cursor_position: u16,
    pub cursor_offset: u16,
    available_width: usize,
    change: Option<InputChange>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            text: String::new(),
            prompt: String::new(),
            idx: 0,
            offset: 0,
            cursor_position: 0,
            cursor_offset: 0,
            available_width: usize::MAX,
            change: None,
        }
    }
}

impl Input {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_owned(),
            cursor_offset: prompt.len() as u16,
            ..Self::default()
        }
    }

    pub fn set_text(&mut self, text: &str) {
        text.clone_into(&mut self.text);
        self.set_idx(text.len());
    }

    pub fn set_idx(&mut self, idx: usize) {
        self.idx = if idx > self.text.len() {
            self.text.len()
        } else {
            idx
        };

        self.cursor_position = self.text[..self.idx].width() as u16;
        self.check_lower_bound();
        self.check_higher_bound();
    }

    fn clear_range<R: RangeBounds<usize>>(&mut self, range: R) {
        if self.text.drain(range).next().is_some() {
            self.change = Some(InputChange::Delete);
        }
    }

    fn insert_key(&mut self, ch: char) {
        if self.idx == self.text.len() {
            self.text.push(ch);
            self.change = Some(InputChange::Append);
        } else {
            self.text.insert(self.idx, ch);
            self.change = Some(InputChange::Insert);
        }

        self.idx += ch.len_utf8();
        self.cursor_position += ch.width().unwrap() as u16;
        self.check_higher_bound();
    }

    fn pop_key(&mut self) {
        if self.idx == 0 {
            return;
        }

        let (offset, ch) = self.text[..self.idx]
            .grapheme_indices(true)
            .next_back()
            .unwrap();
        self.cursor_position -= ch.width() as u16;
        self.clear_range(offset..self.idx);
        self.idx = offset;
        self.check_lower_bound();
    }

    fn move_cursor_left(&mut self) {
        if self.idx == 0 {
            return;
        }

        let (offset, ch) = self.text[..self.idx]
            .grapheme_indices(true)
            .next_back()
            .unwrap();
        self.cursor_position -= ch.width() as u16;
        self.idx = offset;
        self.check_lower_bound();
    }

    fn move_cursor_right(&mut self) {
        if self.idx == self.text.len() {
            return;
        }

        let (offset, ch) = self.text[self.idx..]
            .grapheme_indices(true)
            .next()
            .map(|(offset, ch)| (self.idx + offset + ch.len(), ch))
            .unwrap();
        self.cursor_position += ch.width() as u16;
        self.idx = offset;
        self.check_higher_bound();
    }

    fn move_cursor_one_word_left(&mut self) {
        let idx = self.text[..self.idx]
            .unicode_word_indices()
            .next_back()
            .map_or(0, |(offset, _)| offset);
        self.cursor_position -= self.text[idx..self.idx].width() as u16;
        self.idx = idx;
        self.check_lower_bound();
    }

    fn move_cursor_one_word_right(&mut self) {
        let old_idx = self.idx;
        self.idx = self.text[self.idx..]
            .unicode_word_indices()
            .nth(1)
            .map_or(self.text.len(), |(offset, _)| self.idx + offset);
        self.cursor_position += self.text[old_idx..self.idx].width() as u16;
        self.check_higher_bound();
    }

    fn move_cursor_to_beginning_of_line(&mut self) {
        self.idx = 0;
        self.offset = 0;
        self.cursor_position = 0;
    }

    fn move_cursor_to_end_of_line(&mut self) {
        self.idx = self.text.len();
        self.cursor_position = self.text.width() as u16;
        self.check_higher_bound();
    }

    fn delete_word_before_cursor(&mut self) {
        let old_idx = self.idx;
        self.move_cursor_one_word_left();
        self.clear_range(self.idx..old_idx);
        self.check_lower_bound();
    }

    fn clear_line(&mut self) {
        if !self.text.is_empty() {
            self.text.clear();
            self.idx = 0;
            self.offset = 0;
            self.cursor_position = 0;
            self.change = Some(InputChange::Delete);
        }
    }

    fn clear_to_right(&mut self) {
        self.clear_range(self.idx..);
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.change = None;

        match (key.code, key.modifiers) {
            (KeyCode::Left, KeyModifiers::CONTROL) => self.move_cursor_one_word_left(),
            (KeyCode::Right, KeyModifiers::CONTROL) => self.move_cursor_one_word_right(),
            (KeyCode::Left, _) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                self.move_cursor_left();
            }
            (KeyCode::Right, _) | (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
                self.move_cursor_right();
            }
            (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                self.move_cursor_to_beginning_of_line();
            }
            (KeyCode::Char('e'), KeyModifiers::CONTROL) => self.move_cursor_to_end_of_line(),
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => self.delete_word_before_cursor(),
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => self.clear_line(),
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => self.clear_to_right(),
            (KeyCode::Backspace, _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.pop_key();
            }
            (KeyCode::Char(c), _) => self.insert_key(c),
            _ => (),
        }
    }

    pub fn update_width(&mut self, width: u16) {
        self.available_width = usize::from(width.saturating_sub(self.cursor_offset));
        self.check_higher_bound();
    }

    fn check_lower_bound(&mut self) {
        self.offset = self.offset.min(self.idx);
    }

    fn check_higher_bound(&mut self) {
        let substring = &self.text[self.offset..self.idx];
        let mut width = substring.width();

        for (offset, ch) in substring.grapheme_indices(true) {
            if width < self.available_width {
                self.offset += offset;
                return;
            }

            width -= ch.width();
        }
    }

    pub fn visible_width(&self) -> usize {
        self.text[self.offset..].width()
    }

    pub fn cursor_position(&self) -> u16 {
        self.cursor_position + self.cursor_offset - self.text[..self.offset].width() as u16
    }

    pub fn set_cursor(&self, frame: &mut Frame, area: Rect) {
        frame.set_cursor_position((area.x + self.cursor_position(), area.y));
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.text[self.offset..])
    }
}
