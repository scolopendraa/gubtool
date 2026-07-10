use crate::{
    common::{block, centered_rect, stateful_list::StatefulList},
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
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};
use ratatui::style::Stylize;

pub struct MultiFuzzyFinder {
    matcher: Matcher,
    input: Input,
    pattern: Pattern,
    pub entries: Option<Vec<Utf32String>>,
    matched: Vec<Matched>,
    list_state: StatefulList,
    match_count: usize,
    pub selected: Vec<usize>,
    pub show: bool,
    sender: Option<tokio::sync::oneshot::Sender<Vec<usize>>>,
}

impl Default for MultiFuzzyFinder {
    fn default() -> Self {
        Self {
            matcher: Matcher::new(Config::DEFAULT.match_paths()),
            input: Input::default(),
            pattern: Pattern::default(),
            entries: None,
            matched: Vec::new(),
            list_state: StatefulList::new(0),
            match_count: 0,
            selected: Vec::new(),
            show: false,
            sender: None,
        }
    }
}

impl MultiFuzzyFinder {
    pub fn show(
        &mut self,
        entries: Vec<Utf32String>,
        sender: tokio::sync::oneshot::Sender<Vec<usize>>,
    ) {
        self.entries = Some(entries);
        self.sender = Some(sender);
        self.selected.clear();
        self.update_matches();
        self.show = true;
    }

    fn selected_idx(&self) -> Option<usize> {
        self.list_state.selected().map(|selected| self.matched[selected].idx)
    }

    fn is_selected(&self, idx: usize) -> bool {
        self.selected.contains(&idx)
    }

    fn toggle_selection(&mut self, idx: usize) {
        if self.is_selected(idx) {
            self.selected.retain(|&i| i != idx);
        } else {
            self.selected.push(idx);
        }
    }

    fn update_matches(&mut self) {
        self.pattern
            .reparse(&self.input.text, CaseMatching::Smart, Normalization::Smart);

        self.matched.clear();

        for (idx, path) in self.entries.as_deref().into_iter().flatten().enumerate() {
            let mut indices = Vec::new();
            let score = self
                .pattern
                .indices(path.slice(..), &mut self.matcher, &mut indices);

            if score.is_some() {
                indices.sort_unstable();
                indices.dedup();

                self.matched
                    .push(Matched::new(path.to_string(), idx, score, &indices));
            }
        }
        self.match_count = self.matched.len();
        self.matched.sort_by(|a, b| b.score.cmp(&a.score));
        self.list_state.select(0);
    }

    pub fn draw_checked(&mut self, frame: &mut Frame) {
        if !self.show {
            return;
        }

        let layout = centered_rect(75, 80, frame.area());
        frame.render_widget(Clear, layout);

        let [search_area, results_area, selected_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(2),
                Constraint::Length(4),
            ])
            .areas(layout);

        // Search bar
        let search_block = block(Some("Search"), None);
        frame.render_widget(&search_block, search_area);
        let search_inner = search_block.inner(search_area);

        let [prompt_area, input_area, mut counter_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(self.input.visible_width() as u16),
                Constraint::Fill(1),
            ])
            .areas(search_inner);

        let prompt = Paragraph::new(theme::HIGHLIGHT_SYMBOL);
        frame.render_widget(prompt, prompt_area);

        self.input.update_width(search_area.width);
        let input = Paragraph::new(self.input.to_string()).style(theme().fg);
        self.input.set_cursor(frame, input_area);
        frame.render_widget(input, input_area);

        counter_area = counter_area.inner(Margin::new(1, 0));
        let counter = format!(
            "{} / {} ({} selected) (x: toggle)",
            self.match_count,
            self.entries.as_ref().unwrap_or(&vec![]).len(),
            self.selected.len()
        );
        if counter.len() <= counter_area.width.into() {
            let counter = Paragraph::new(counter).right_aligned().style(theme().fg);
            frame.render_widget(counter, counter_area);
        }

        // Results list
        let results_block = block(Some("Results"), None);
        frame.render_widget(&results_block, results_area);
        let inner = results_block.inner(results_area);

        let [name_area, label_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(40),
                Constraint::Max(30),
            ])
            .areas(inner);

        let selected_idx = self.list_state.selected().unwrap_or(0);
        let mut names = Vec::new();
        let mut labels = Vec::new();
        let mut labels_len = 0;

        self.matched.iter().enumerate().for_each(|(idx, item)| {
            let checkbox = if self.is_selected(item.idx) { "[X] " } else { "[ ] " };
            let mut name_span = item.highlight_line().0;
            name_span.insert(0, (checkbox, false));
            names.push(Self::highlighted_list_item(
                idx,
                selected_idx,
                name_span,
                Style::from(theme().fg),
            ));

            if let (Some(label), Some(label_span)) = (&item.label, item.highlight_line().1) {
                labels_len = labels_len.max(label.chars().count() + 3);
                labels.push(Self::highlighted_list_item(
                    idx,
                    selected_idx,
                    label_span,
                    Style::from(theme().muted),
                ));
            }
        });

        frame.render_stateful_widget(
            List::new(names),
            name_area,
            &mut self.list_state.state,
        );
        frame.render_stateful_widget(
            List::new(labels)
                .block(Block::default().borders(Borders::LEFT)),
            label_area,
            &mut self.list_state.state,
        );

        // Selected items list
        let selected_block = block(Some("Selected"), None);
        frame.render_widget(&selected_block, selected_area);
        let selected_inner = selected_block.inner(selected_area);

        let [sel_name_area, sel_label_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(40),
                Constraint::Max(30),
            ])
            .areas(selected_inner);

        let mut sel_names = Vec::new();
        let mut sel_labels = Vec::new();

        for &idx in &self.selected {
            if let Some(entry) = self.entries.as_ref().and_then(|e| e.get(idx)) {
                let text = entry.to_string();
                let (name, label) = if let Some(split_byte_idx) = text.find('|') {
                    let (name_part, label_part) = text.split_at(split_byte_idx);
                    (name_part.to_string(), Some(label_part[1..].to_string()))
                } else {
                    (text, None)
                };
                sel_names.push(ListItem::from(Line::raw(name)));
                if let Some(l) = label {
                    sel_labels.push(ListItem::from(Line::raw(l).fg(theme().muted)));
                }
            }
        }

        if self.selected.is_empty() {
            sel_names.push(ListItem::from(
                Line::raw("[No items selected]").style(Style::default().fg(theme().muted)),
            ));
        }

        frame.render_widget(List::new(sel_names), sel_name_area);
        frame.render_widget(
            List::new(sel_labels)
                .block(Block::default().borders(Borders::LEFT)),
            sel_label_area,
        );
    }

    fn highlighted_list_item(
        idx: usize,
        selected_idx: usize,
        line: Vec<(&str, bool)>,
        style: Style,
    ) -> ListItem<'static> {
        let spans = line
            .into_iter()
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
        self.list_state.size = self.match_count;

        match (key.code, key.modifiers) {
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.list_state.increment(28);
            }
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                self.list_state.decrement(28);
            }
            (KeyCode::Down, _)
            | (KeyCode::Tab, _)
            | (KeyCode::Char('j'), KeyModifiers::CONTROL) => {
                self.list_state.increment(1);
            }
            (KeyCode::Up, _)
            | (KeyCode::BackTab, _)
            | (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                self.list_state.decrement(1);
            }
            (KeyCode::Char('x'), _) => {
                // Toggle selection for current item
                if let Some(idx) = self.selected_idx() {
                    self.toggle_selection(idx);
                }
            }
            (KeyCode::Enter, _) => {
                if let Some(tx) = self.sender.take() {
                    let selected = self.selected.clone();
                    let _ = tx.send(selected);
                }
                self.input.set_text("");
                self.entries.take();
                self.show = false;
            }
            (KeyCode::Esc, _) => {
                // Send an empty selection to the caller to prevent hanging
                // on rx.await if the user cancels the finder.
                if let Some(tx) = self.sender.take() {
                    let _ = tx.send(Vec::new());
                }
                self.input.set_text("");
                self.entries.take();
                self.show = false;
            }
            _ => {
                let _ = self.input.handle_keys(key);
                self.update_matches();
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Matched {
    idx: usize,
    score: Option<u32>,
    name: String,
    label: Option<String>,
    name_indices: Vec<u32>,
    label_indices: Option<Vec<u32>>,
}

impl Matched {
    fn new(
        text: String,
        idx: usize,
        score: Option<u32>,
        indices: &[u32],
    ) -> Self {
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
                (
                    String::from(name_part),
                    Some(String::from(label_part)),
                    name_indices,
                    Some(label_indices),
                )
            } else {
                (text, None, Vec::from(indices), None)
            };
        Self {
            idx,
            score,
            name,
            label,
            name_indices,
            label_indices,
        }
    }

    fn highlight_line(
        &self,
    ) -> (Vec<(&str, bool)>, Option<Vec<(&str, bool)>>) {
        let name_highlights = Self::highlight_slice(&self.name, &self.name_indices);
        let label_highlights =
            if let (Some(label), Some(label_indices)) = (&self.label, &self.label_indices) {
                Some(Self::highlight_slice(label, label_indices))
            } else {
                None
            };
        (name_highlights, label_highlights)
    }

    fn highlight_slice<'a>(
        text: &'a str,
        indices: &'a [u32],
    ) -> Vec<(&'a str, bool)> {
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

        while current_char_pos < chars.len() {
            let is_highlighted = highlighted[current_char_pos];
            let run_start_char = current_char_pos;

            while current_char_pos < chars.len() && highlighted[current_char_pos] == is_highlighted {
                current_char_pos += 1;
            }

            let run_str_len: usize = chars[run_start_char..current_char_pos]
                .iter()
                .map(|c| c.len_utf8())
                .sum();

            let end_byte = start_byte + run_str_len;
            slices.push((&text[start_byte..end_byte], is_highlighted));
            start_byte = end_byte;
        }
        slices
    }
}
