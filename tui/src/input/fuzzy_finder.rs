use {
    crate::{
        app::App,
        common::helpers::bordered_block,
        event::{Event, KeyContext, send_event},
        input::Input,
        panes::{Pane, TableController, TablePane, TableView},
        popup::{Popup, PopupState, centered_popup},
        screen::Screen,
        theme::{self, theme},
    },
    crossterm::event::KeyCode,
    nucleo_matcher::{
        Matcher,
        Utf32String,
        pattern::{CaseMatching, Normalization, Pattern},
    },
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Margin, Rect},
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Cell, Paragraph, Row},
    },
    std::{cell::RefCell, rc::Rc},
};

pub struct FuzzyFinder {
    input:       Input,
    table:       TablePane,
    popup_state: PopupState,
    match_count: usize,
    entries:     Option<Vec<Utf32String>>,
    matched:     Rc<RefCell<Vec<Matched>>>,
    matcher:     Matcher,
    pattern:     Pattern,
    pub request: Option<&'static dyn SearchRequest>,
}

struct SearchController {
    matched: Rc<RefCell<Vec<Matched>>>,
}

impl SearchController {
    fn new(matched: Rc<RefCell<Vec<Matched>>>) -> Self {
        Self {
            matched,
        }
    }
}

pub trait SearchRequest: Send + Sync {
    fn items(&self) -> Vec<Utf32String>;
    fn jump(&self, app: &mut App, selected: usize) {
        app.current_screen()
            .tab_manager()
            .current_tab_mut()
            .pane_manager()
            .set_current_list_idx(selected);
    }
}

impl TableController for SearchController {
    fn make_table_view(&self) -> TableView {
        let mut longest_name_len = 0;
        let mut name_cells: Vec<Cell> = Vec::new();
        let mut label_cells: Vec<Cell> = Vec::new();

        self.matched.borrow().iter().for_each(|item| {
            let name_len = item.name.len();
            if name_len > longest_name_len {
                longest_name_len = name_len
            }

            let (name_span, label_span) = item.highlight_line();
            name_cells.push(highlighted_cell(name_span, Style::from(theme().fg)));
            if let Some(label_span) = label_span {
                label_cells.push(highlighted_cell(label_span, Style::from(theme().muted)));
            }
        });

        let widths = if !label_cells.is_empty() {
            vec![
                Constraint::Min(longest_name_len as u16 + 1),
                Constraint::Fill(1),
            ]
        } else {
            vec![Constraint::Fill(1)]
        };

        let rows = if !label_cells.is_empty() {
            name_cells
                .into_iter()
                .zip(label_cells)
                .map(|(name, label)| Row::new([name, label]))
                .collect()
        } else {
            name_cells
                .into_iter()
                .map(|name| Row::new([name]))
                .collect()
        };

        TableView::new(rows).with_widths(widths)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.peek_code() == Some(KeyCode::Enter) {
            let matched = self.matched.borrow();
            if !matched.is_empty() {
                let idx = matched[selected].idx;
                send_event(Event::SearchResult(idx));
            }
        }
    }
}

impl Screen for FuzzyFinder {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let [search_area, results_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .areas(rect);

        let search_block = bordered_block(Some("Search"));
        frame.render_widget(&search_block, search_area);
        let search_area = search_block.inner(search_area);

        let [prompt_area, input_area, mut counter_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(self.input.visible_width() as u16),
                Constraint::Fill(1),
            ])
            .areas(search_area);

        let prompt = Paragraph::new(theme::HIGHLIGHT_SYMBOL);
        frame.render_widget(prompt, prompt_area);

        self.input.update_width(search_area.width);
        let input = Paragraph::new(self.input.to_string()).style(theme().fg);
        self.input.set_cursor(frame, input_area);
        frame.render_widget(input, input_area);

        counter_area = counter_area.inner(Margin::new(1, 0));
        let counter =
            format!("{} / {}", self.match_count, self.entries.as_ref().unwrap_or(&vec![]).len());
        if counter.len() <= counter_area.width.into() {
            let counter = Paragraph::new(counter).right_aligned().style(theme().fg);
            frame.render_widget(counter, counter_area);
        }
        self.table.draw(frame, results_area);
    }
    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        let prev_match_count = self.match_count;

        if ctx.key(KeyCode::Tab) || ctx.key(KeyCode::Down) {
            self.table.increment_wrapping(1);
        }

        if ctx.key(KeyCode::BackTab) || ctx.key(KeyCode::Up) {
            self.table.decrement_wrapping(1);
        }

        self.table.handle_keys_selected(ctx);
        self.input.handle_keys(ctx);

        self.update_matches();

        if self.match_count != prev_match_count {
            self.table.select(0);
        }
    }
}

impl Popup for FuzzyFinder {
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn screen(&mut self) -> &mut dyn Screen {
        self
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        centered_popup(75, 75, frame.area())
    }
    fn close(&mut self) {
        self.input.set_text("");
        self.entries.take();
        self.table.select(0);
        self.popup_state.close();
    }
    fn close_on_key(&self, ctx: &mut KeyContext) -> bool {
        ctx.key(KeyCode::Enter) | ctx.key(KeyCode::Esc)
    }
}

impl Default for FuzzyFinder {
    fn default() -> Self {
        let matched = Rc::new(RefCell::new(Vec::new()));
        Self {
            match_count: 0,
            input: Input::default(),
            popup_state: PopupState::default(),
            entries: None,
            matcher: Matcher::default(),
            pattern: Pattern::default(),
            table: TablePane::new_owned(SearchController::new(matched.clone()))
                .freeze()
                .with_title("Results"),
            matched,
            request: None,
        }
    }
}

impl FuzzyFinder {
    pub fn show(&mut self, request: &'static dyn SearchRequest) {
        self.request = Some(request);
        self.entries = Some(request.items());
        self.update_matches();
        self.popup_state.open();
    }

    fn update_matches(&mut self) {
        self.pattern
            .reparse(&self.input.text, CaseMatching::Smart, Normalization::Smart);

        {
            let mut matched = self.matched.borrow_mut();
            matched.clear();

            for (idx, path) in self.entries.as_deref().into_iter().flatten().enumerate() {
                let mut indices = Vec::new();
                let score = self
                    .pattern
                    .indices(path.slice(..), &mut self.matcher, &mut indices);

                if score.is_some() {
                    indices.sort_unstable();
                    indices.dedup();

                    matched.push(Matched::new(path.to_string(), idx, score, &indices));
                }
            }
            self.match_count = matched.len();
            matched.sort_by_key(|x| std::cmp::Reverse(x.score));
        }

        self.table.update_container();
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Matched {
    idx:           usize,
    score:         Option<u32>,
    name:          String,
    label:         Option<String>,
    name_indices:  Vec<u32>,
    label_indices: Option<Vec<u32>>,
}

impl Matched {
    fn new(text: String, idx: usize, score: Option<u32>, indices: &[u32]) -> Self {
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

    fn highlight_line(&self) -> (Vec<(&str, bool)>, Option<Vec<(&str, bool)>>) {
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

        while current_char_pos < chars.len() {
            let is_highlighted = highlighted[current_char_pos];
            let run_start_char = current_char_pos;

            while current_char_pos < chars.len() && highlighted[current_char_pos] == is_highlighted
            {
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

fn highlighted_cell(line: Vec<(&str, bool)>, style: Style) -> Cell<'static> {
    let spans = line
        .into_iter()
        .map(|(slice, highlighted)| {
            let content = slice.to_string();
            if highlighted {
                Span::raw(content).fg(theme().error)
            } else {
                Span::raw(content)
            }
        })
        .collect::<Vec<Span>>();
    Cell::new(Line::from(spans)).style(style)
}
