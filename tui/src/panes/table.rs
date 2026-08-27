use {
    crate::{
        common::{
            controls::{Control, draw_controls},
            helpers::bordered_block,
        },
        event::{Event, KeyContext, send_event},
        panes::{Pane, TableController, TableControllerSource},
        screen::Screen,
        theme::{get_theme_version, theme},
    },
    crossterm::event::{KeyCode, KeyModifiers},
    ratatui::{
        Frame,
        buffer::Buffer,
        layout::{Constraint, Rect},
        style::{Modifier, Stylize},
        widgets::{Row, Table, TableState},
    },
    shared::command::Command,
};

pub struct TablePane {
    pane_controller:   TableControllerSource,
    table:             Table<'static>,
    state:             TableState,
    size:              usize,
    is_frozen:         bool,
    is_non_selectable: bool,
    has_header:        bool,
    title:             Option<&'static str>,
    controls:          Option<&'static [Control]>,
    theme_version:     Option<usize>,
}

pub struct TableView {
    rows:   Vec<Row<'static>>,
    widths: Option<Widths>,
    header: Option<Row<'static>>,
}

impl TableView {
    pub fn new(rows: Vec<Row<'static>>) -> Self {
        Self {
            rows,
            widths: None,
            header: None,
        }
    }

    pub fn with_widths<T>(mut self, widths: T) -> Self
    where T: Into<Widths> {
        self.widths = Some(widths.into());
        self
    }

    pub fn with_header(mut self, header: Row<'static>) -> Self {
        self.header = Some(header);
        self
    }
}

impl TablePane {
    pub fn new_static(table_controller: &'static dyn TableController) -> Self {
        let table = Table::default();
        let state = TableState::default().with_selected(Some(0));
        let pane_controller = TableControllerSource::Static(table_controller);
        Self {
            table,
            state,
            size: 0,
            pane_controller,
            is_frozen: false,
            title: None,
            controls: None,
            is_non_selectable: false,
            has_header: false,
            theme_version: None,
        }
    }

    pub fn new_owned<C>(table_controller: C) -> Self
    where C: TableController + 'static {
        let table = Table::default();
        let state = TableState::default().with_selected(Some(0));
        let pane_controller = TableControllerSource::Owned(Box::new(table_controller));
        Self {
            table,
            state,
            size: 0,
            pane_controller,
            is_frozen: false,
            title: None,
            controls: None,
            is_non_selectable: false,
            has_header: false,
            theme_version: None,
        }
    }

    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn as_non_selectable(mut self) -> Self {
        self.state = TableState::default();
        self.is_non_selectable = true;
        self
    }

    pub fn with_controls(mut self, controls: &'static [Control]) -> Self {
        self.controls = Some(controls);
        self
    }

    pub fn freeze(mut self) -> Self {
        self.is_frozen = true;
        self
    }
}

impl Pane for TablePane {
    fn select(&mut self, index: usize) {
        self.state.select(Some(index));
    }
    fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
    fn current_command(&self) -> Option<Command> {
        if let Some(idx) = self.selected() {
            self.pane_controller.get_command(idx).copied()
        } else {
            None
        }
    }
    fn draw_pane(&mut self, frame: &mut Frame, rect: Rect, active: bool) {
        if !self.is_frozen {
            self.update_container();
        } else {
            let theme_version = get_theme_version();
            if self.theme_version != Some(theme_version) {
                self.theme_version = Some(theme_version);
                self.update_container();
            }
        }

        let mut block = bordered_block(self.title);
        let inner = block.inner(rect);

        if !active {
            block = block.add_modifier(Modifier::DIM);
        }

        let mut height = inner.height as usize;
        if self.has_header {
            height = height.saturating_sub(1)
        }
        let max_offset = self.size.saturating_sub(height);
        if self.current_offset() > max_offset {
            self.set_offset(max_offset);
        }

        frame.render_widget(block, rect);
        frame.render_stateful_widget(&self.table, inner, &mut self.state);

        self.highlight_row(frame.buffer_mut(), inner, active);

        if let Some(controls) = self.controls {
            draw_controls(frame, rect, controls);
        }
    }
}

impl Screen for TablePane {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.draw_active(frame, area);
    }
    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        if self.is_non_selectable {
            self.handle_keys_non_selectable(ctx);
        } else {
            self.handle_keys_selectable(ctx);
        }
    }
}

impl TablePane {
    pub fn update_container(&mut self) {
        let view = self.pane_controller.make_table_view();
        self.size = view.rows.len();

        let widths = view
            .widths
            .unwrap_or(Widths::Static(&[Constraint::Fill(1)]));

        let mut table = Table::new(view.rows, widths.as_slice());

        if let Some(header) = view.header {
            self.has_header = true;
            table = table.header(header);
        }

        self.table = table
    }
    pub fn increment_saturating(&mut self, val: usize) {
        if let Some(idx) = self.selected() {
            let new_idx = idx.saturating_add(val).min(self.size.saturating_sub(1));
            self.select(new_idx);
        }
    }
    pub fn decrement_saturating(&mut self, val: usize) {
        if let Some(idx) = self.selected() {
            self.select(idx.saturating_sub(val));
        }
    }
    pub fn increment_wrapping(&mut self, val: usize) {
        if self.size == 0 {
            return;
        }

        if let Some(idx) = self.selected() {
            self.select((idx + val) % self.size);
        }
    }
    pub fn decrement_wrapping(&mut self, val: usize) {
        if self.size == 0 {
            return;
        }

        if let Some(idx) = self.selected() {
            self.select((idx + self.size - (val % self.size)) % self.size);
        }
    }
    fn set_offset(&mut self, index: usize) {
        *self.state.offset_mut() = index
    }
    fn current_offset(&self) -> usize {
        self.state.offset()
    }
    fn increment_offset(&mut self, val: usize) {
        self.set_offset(self.current_offset() + val);
    }
    fn decrement_offset(&mut self, val: usize) {
        self.set_offset(self.current_offset().saturating_sub(val));
    }
    pub fn handle_keys_selected(&self, ctx: &mut KeyContext) {
        let selected = self.selected().unwrap_or(0);
        self.pane_controller.handle_keys_selected(selected, ctx);
    }
    fn handle_keys_selectable(&mut self, ctx: &mut KeyContext) {
        if ctx.key_with_modifiers(KeyCode::Char('u'), KeyModifiers::CONTROL) {
            self.decrement_saturating(28);
        }

        if ctx.key_with_modifiers(KeyCode::Char('d'), KeyModifiers::CONTROL) {
            self.increment_saturating(28);
        }

        if ctx.key_with_modifiers(KeyCode::Char('j'), KeyModifiers::NONE)
            || ctx.key_with_modifiers(KeyCode::Down, KeyModifiers::NONE)
        {
            self.increment_wrapping(1);
        }

        if ctx.key_with_modifiers(KeyCode::Char('k'), KeyModifiers::NONE)
            || ctx.key_with_modifiers(KeyCode::Up, KeyModifiers::NONE)
        {
            self.decrement_wrapping(1);
        }

        if ctx.key_char('g') {
            self.select(0);
        }

        if ctx.key_char('G') {
            self.select(self.size.saturating_sub(1));
        }

        if ctx.key_char('c')
            && let Some(command) = self.current_command()
        {
            send_event(Event::CliCommandInfo(command));
        }

        self.handle_keys_selected(ctx);
    }
    fn handle_keys_non_selectable(&mut self, ctx: &mut KeyContext) {
        if ctx.key_with_modifiers(KeyCode::Char('u'), KeyModifiers::CONTROL) {
            self.decrement_offset(28);
        }

        if ctx.key_with_modifiers(KeyCode::Char('d'), KeyModifiers::CONTROL) {
            self.increment_offset(28);
        }

        if ctx.key_char('j') || ctx.key(KeyCode::Down) {
            self.decrement_offset(1);
        }

        if ctx.key_char('k') || ctx.key(KeyCode::Up) {
            self.increment_offset(1);
        }

        if ctx.key_char('g') {
            self.set_offset(0);
        }

        if ctx.key_char('G') {
            self.set_offset(self.size.saturating_sub(1));
        }

        self.handle_keys_selected(ctx);
    }

    fn highlight_row(&self, buf: &mut Buffer, area: Rect, active: bool) {
        if let Some(selected) = self.selected() {
            let theme = theme();
            let top_left = if self.has_header { area.y + 1 } else { area.y };
            let y = top_left + selected as u16 - self.current_offset() as u16;

            for x in area.left()..area.right() {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    if active {
                        if cell.fg == theme.error {
                            cell.set_bg(theme.accent);
                        } else {
                            cell.set_fg(theme.accent);
                            cell.modifier.insert(Modifier::REVERSED);
                        }
                    } else {
                        cell.set_fg(theme.accent);
                    }

                    cell.modifier.insert(Modifier::BOLD);
                }
            }
        }
    }
}

pub enum Widths {
    Static(&'static [Constraint]),
    Dynamic(Vec<Constraint>),
}

impl Widths {
    fn as_slice(&self) -> &[Constraint] {
        match self {
            Widths::Static(widths) => widths,
            Widths::Dynamic(widths) => widths,
        }
    }
}

impl<const N: usize> From<&'static [Constraint; N]> for Widths {
    fn from(widths: &'static [Constraint; N]) -> Self {
        Widths::Static(widths)
    }
}

impl From<Vec<Constraint>> for Widths {
    fn from(value: Vec<Constraint>) -> Self {
        Widths::Dynamic(value)
    }
}
