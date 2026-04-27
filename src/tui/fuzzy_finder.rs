use crate::tui::{
    common::{block, centered_rect},
    input::Input,
    theme::{self, theme},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use nucleo_matcher::{
    Config, Matcher, Utf32String,
    pattern::{CaseMatching, Normalization, Pattern},
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

pub struct FuzzyFinder {
    pub matcher: Matcher,
    pub input: Input,
    pattern: Pattern,
    pub list: Option<Vec<Utf32String>>,
    pub matched: StatefulList<Matched>,
    pub total_count: usize,
    pub match_count: usize,
}

impl Default for FuzzyFinder {
    fn default() -> Self {
        Self {
            matcher: Matcher::new(Config::DEFAULT.match_paths()),
            input: Input::new(theme::HIGHLIGHT_SYMBOL),
            pattern: Pattern::default(),
            list: None,
            matched: StatefulList::with_items(Vec::new()),
            total_count: 0,
            match_count: 0,
        }
    }
}

impl FuzzyFinder {
    pub fn set_items(&mut self, list: Vec<Utf32String>) {
        self.total_count = list.len();
        self.list = Some(list);
    }

    pub fn selected_idx(&self) -> Option<usize> {
        self.matched.get_selected().map(|item| item.idx)
    }

    pub fn reset(&mut self) {
        self.input.set_text("");
        self.list.take();
    }

    pub fn update_matches(&mut self) {
        self.pattern
            .reparse(&self.input.text, CaseMatching::Smart, Normalization::Smart);

        self.matched.items.clear();

        for (idx, path) in self.list.as_deref().into_iter().flatten().enumerate() {
            let mut indices = Vec::new();
            let score = self
                .pattern
                .indices(path.slice(..), &mut self.matcher, &mut indices);

            if score.is_some() {
                indices.sort_unstable();
                indices.dedup();

                self.matched.items
                    .push(Matched::new(path.to_string(), idx, score, &indices));
            }
        }
        self.match_count = self.matched.items.len();
        self.matched.items.sort_by(|a, b| b.score.cmp(&a.score));
        self.matched.select_first();
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let layout = centered_rect(75, 75, frame.area());
        frame.render_widget(Clear, layout);

        let [search_area, results_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .areas(layout);

        let search_block = block(Some("Search"), None);
        frame.render_widget(&search_block, search_area);
        let search_area = search_block.inner(search_area);

        let [prompt_area, input_area, mut counter_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(self.input.cursor_offset),
                Constraint::Length(self.input.visible_width() as u16),
                Constraint::Fill(1),
            ])
            .areas(search_area);

        let prompt = Paragraph::new(self.input.prompt.clone());
        frame.render_widget(prompt, prompt_area);

        self.input.update_width(search_area.width);
        let input = Paragraph::new(self.input.to_string()).style(theme().fg);
        self.input.set_cursor(frame, prompt_area);
        frame.render_widget(input, input_area);

        counter_area = counter_area.inner(Margin::new(1, 0));
        let counter = format!("{} / {}",
            self.match_count, self.total_count
        );
        if counter.len() <= counter_area.width.into() {
            let counter = Paragraph::new(counter).right_aligned().style(theme().fg);
            frame.render_widget(counter, counter_area);
        }
        let selected_idx = self.matched.state.selected().unwrap_or(0);
        let mut names = Vec::new();
        let mut labels = Vec::new();
        let mut labels_len = 0;

        self.matched.items.iter().enumerate()
            .for_each(|(idx, item)| {
                let mut name_span = item.highlight_line().0;
                name_span.insert(0, (if selected_idx == idx { theme::HIGHLIGHT_SYMBOL } else { "  " }, false));
                names.push(Self::highlighted_list_item(idx, selected_idx, name_span, Style::from(theme().fg)));

                if let (Some(label), Some(label_span)) = (&item.label, item.highlight_line().1) {
                    labels_len = labels_len.max(label.chars().count() + 3);
                    labels.push(Self::highlighted_list_item(idx, selected_idx, label_span, Style::from(theme().muted)));
                }
            });

        let results_block = block(Some("Results"), None);
        frame.render_widget(&results_block, results_area);
        let inner = results_block.inner(results_area);

        let [name_area, label_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(40),
                Constraint::Max(labels_len as u16),
            ])
            .areas(inner);
        frame.render_stateful_widget(
            List::new(names),
            name_area,
            &mut self.matched.state
        );
        frame.render_stateful_widget(
            List::new(labels).block(Block::default().borders(Borders::LEFT)),
            label_area,
            &mut self.matched.state
        );
    }

    fn highlighted_list_item(idx: usize, selected_idx: usize, line: Vec<(&str, bool)>, style: Style) -> ListItem<'static> {
        let spans = line.into_iter()
            .map(|(slice, highlighted)| {
                let content = slice.to_string();
                if highlighted {
                    Span::raw(content).style(theme().warning)
                } else if selected_idx == idx {
                    Span::raw(content).style(theme().accent)
                } else {
                    Span::raw(content)
                }
            })
            .collect::<Vec<Span>>();
        ListItem::new(Line::from(spans)).style(style)
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.matched.increment(28);
            }
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                self.matched.decrement(28);
            }
            (KeyCode::Down, _) | (KeyCode::Tab, _) | (KeyCode::Char('j'), KeyModifiers::CONTROL) => {
                self.matched.increment(1);
            }
            (KeyCode::Up, _) | (KeyCode::BackTab, _) | (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                self.matched.decrement(1);
            }
            _ => {
                let _ = self.input.handle_keys(key);
                self.update_matches();
            }
        }
    }
}


pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        let mut stateful_list = StatefulList {
            state: ListState::default(),
            items,
        };
        stateful_list.select_first();
        stateful_list
    }
    fn select_with_index(&mut self, index: usize) {
        self.state.select(if self.items.is_empty() {
            None
        } else {
            Some(index)
        });
    }
    pub fn increment(&mut self, val: usize) {
        if let Some(idx) = self.state.selected() {
            let new_idx = {
                if idx + val > self.items.len().saturating_sub(1) {
                    self.items.len().saturating_sub(1)
                } else {
                    idx + val
                }
            };
            self.select_with_index(new_idx);
        }
    }
    pub fn decrement(&mut self, val: usize) {
        if let Some(idx) = self.state.selected() {
            self.select_with_index(idx.saturating_sub(val));
        }
    }
    pub fn select_first(&mut self) {
        self.select_with_index(0);
    }
    pub fn get_selected(&self) -> Option<&T> {
        self.state.selected().and_then(|idx| self.items.get(idx))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Matched {
    pub idx: usize,
    pub score: Option<u32>,
    pub name: String,
    pub label: Option<String>,
    pub name_indices: Vec<u32>,
    pub label_indices: Option<Vec<u32>>,
}

impl Matched {
    pub fn new(text: String, idx: usize, score: Option<u32>, indices: &[u32]) -> Self {
        let (name, label, name_indices, label_indices) =
        if let Some(split_byte_idx) = text.find('|') {
            let split_char_idx = text[..split_byte_idx].chars().count();

            let (name_part, label_part) = text.split_at(split_byte_idx);
            let label_part = &label_part[1..];

            let mut name_indices = Vec::new();
            let mut label_indices = Vec::new();

            for &i in indices {
                if i < split_char_idx as u32 {
                    name_indices.push(i);
                } else if i > split_char_idx as u32 {
                    let new_i = i.saturating_sub(split_char_idx as u32 + 1);
                    label_indices.push(new_i);
                }
            }
            (String::from(name_part), Some(String::from(label_part)), name_indices, Some(label_indices))
        } else {
            (text, None, Vec::from(indices), None)
        };
        Self { idx, score, name, label, name_indices, label_indices }
    }

    pub fn highlight_line(&self) -> (Vec<(&str, bool)>, Option<Vec<(&str, bool)>>) {
        let name_highlights = Self::highlight_slice(&self.name, &self.name_indices);
        let label_highlights =
        if let (Some(label), Some(label_indices)) = (&self.label, &self.label_indices) {
            Some(Self::highlight_slice(label, label_indices))
        } else {
            None
        };
        (name_highlights, label_highlights)
    }

    fn highlight_slice<'a>(text: &'a str, indices: &'a [u32]) -> Vec<(&'a str, bool)> {
        if indices.is_empty() {
            return vec![(text, false)];
        }
        let mut slices = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut highlighted = vec![false; chars.len()];

        for &idx in indices {
            if (idx as usize) < highlighted.len() {
                highlighted[idx as usize] = true;
            }
        }
        let mut start_byte = 0;
        let mut current_char_pos = 0;

        while current_char_pos <  chars.len() {
            let is_highlighted = highlighted[current_char_pos];
            let run_start_char = current_char_pos;

            while current_char_pos < chars.len() && highlighted[current_char_pos] == is_highlighted {
                current_char_pos += 1;
            }

            let run_str_len: usize = chars[run_start_char..current_char_pos]
                .iter().map(|c| c.len_utf8()).sum();

            let end_byte = start_byte + run_str_len;
            slices.push((&text[start_byte..end_byte], is_highlighted));
            start_byte = end_byte;
        }
        slices
    }
}
