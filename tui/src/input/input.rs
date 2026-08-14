use {
    crate::event::KeyContext,
    crossterm::event::{KeyCode, KeyModifiers},
    ratatui::{Frame, layout::Rect},
    std::{fmt::Display, ops::RangeBounds},
    unicode_segmentation::UnicodeSegmentation,
    unicode_width::{UnicodeWidthChar, UnicodeWidthStr},
};

pub struct Input {
    pub text:          String,
    idx:               usize,
    offset:            usize,
    cursor_position:   u16,
    pub cursor_offset: u16,
    available_width:   usize,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            text:            String::new(),
            idx:             0,
            offset:          0,
            cursor_position: 0,
            cursor_offset:   0,
            available_width: usize::MAX,
        }
    }
}

impl Input {
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
        if self.text.drain(range).next().is_some() {}
    }

    fn insert_key(&mut self, ch: char) {
        if self.idx == self.text.len() {
            self.text.push(ch);
        } else {
            self.text.insert(self.idx, ch);
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

    pub fn clear_line(&mut self) {
        if !self.text.is_empty() {
            self.text.clear();
            self.idx = 0;
            self.offset = 0;
            self.cursor_position = 0;
        }
    }

    fn clear_to_right(&mut self) {
        self.clear_range(self.idx..);
    }

    pub fn handle_keys(&mut self, ctx: &mut KeyContext) {
        if ctx.key_with_modifiers(KeyCode::Left, KeyModifiers::CONTROL) {
            self.move_cursor_one_word_left();
        }

        if ctx.key_with_modifiers(KeyCode::Right, KeyModifiers::CONTROL) {
            self.move_cursor_one_word_right();
        }

        if ctx.key_with_modifiers(KeyCode::Char('b'), KeyModifiers::CONTROL)
            || ctx.key_with_modifiers(KeyCode::Left, KeyModifiers::NONE)
        {
            self.move_cursor_left();
        }

        if ctx.key_with_modifiers(KeyCode::Char('f'), KeyModifiers::CONTROL)
            || ctx.key_with_modifiers(KeyCode::Right, KeyModifiers::NONE)
        {
            self.move_cursor_right();
        }

        if ctx.key_with_modifiers(KeyCode::Char('a'), KeyModifiers::CONTROL) {
            self.move_cursor_to_beginning_of_line();
        }

        if ctx.key_with_modifiers(KeyCode::Char('e'), KeyModifiers::CONTROL) {
            self.move_cursor_to_end_of_line();
        }

        if ctx.key_with_modifiers(KeyCode::Char('w'), KeyModifiers::CONTROL) {
            self.delete_word_before_cursor();
        }

        if ctx.key_with_modifiers(KeyCode::Char('u'), KeyModifiers::CONTROL) {
            self.clear_line();
        }

        if ctx.key_with_modifiers(KeyCode::Char('k'), KeyModifiers::CONTROL) {
            self.clear_to_right();
        }

        if ctx.key_with_modifiers(KeyCode::Char('h'), KeyModifiers::CONTROL)
            || ctx.key_with_modifiers(KeyCode::Backspace, KeyModifiers::NONE)
        {
            self.pop_key();
        }

        if let Some(KeyCode::Char(c)) = ctx.peek_code() {
            ctx.consume();
            self.insert_key(c);
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
