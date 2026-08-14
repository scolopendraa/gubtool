mod pointers_popup;
use {
    crate::{
        common::controls::{Control, HelpPopup},
        event::{KeyContext, ResultExt},
        input::request_input,
        memory_viewer_screen::pointers_popup::PointersPopup,
        popup::{Popup, PopupState},
        screen::Screen,
        spawn_task,
        theme::theme,
    },
    crossterm::event::{KeyCode, KeyModifiers},
    gubtool_core::memory_viewer::{self, MemoryViewer},
    ratatui::{
        Frame,
        layout::{
            Constraint,
            Direction::{self, Horizontal},
            Layout,
            Rect,
        },
        style::{Modifier, Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Borders, Paragraph, Wrap},
    },
    std::sync::{LazyLock, Mutex, MutexGuard},
};

const HELP_ENTRIES: [Control; 15] = [
    Control::new("g", "Jump to module relative address"),
    Control::new("ctrl-g", "Jump to absolute address"),
    Control::new("enter", "Write byte"),
    Control::new("d", "Write dword"),
    Control::new("q", "Write qword"),
    Control::new("ctrl-d", "Copy dword"),
    Control::new("ctrl-q", "Copy qword"),
    Control::new("y", "Copy module relative address"),
    Control::new("ctrl-y", "Copy absolute address"),
    Control::new("b", "Jump to absolute address at selected"),
    Control::new("ctrl-b", "Jump to relative address at selected"),
    Control::new("u", "Undo jump"),
    Control::new("ctrl-r", "Redo jump"),
    Control::new("i", "Increment selected"),
    Control::new("p", "Show cached pointers"),
];

static MEMORY_VIEWER: LazyLock<Mutex<MemoryViewer>> =
    LazyLock::new(|| Mutex::new(MemoryViewer::new()));

fn memory_viewer() -> MutexGuard<'static, MemoryViewer> {
    MEMORY_VIEWER.lock().unwrap()
}

pub struct MemoryViewerScreen {
    help:          HelpPopup,
    pointers:      PointersPopup,
    bytes_per_row: i64,
    frame_heigth:  i64,
    popup_state:   PopupState,
}

impl Screen for MemoryViewerScreen {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        {
            let mut m = memory_viewer();
            m.poll();
            if let Some(current_row) = self.current_row(&m) {
                let diff = current_row as i64 - (self.frame_heigth - 1);
                if diff >= 0 {
                    m.increment_base(self.bytes_per_row * diff);
                }
            }
        }

        let block = Block::new()
            .borders(Borders::TOP | Borders::BOTTOM)
            .bg(theme().bg);
        let [address, bytes] = Layout::default()
            .direction(Horizontal)
            .constraints(vec![Constraint::Max(15), Constraint::Fill(1)])
            .areas(block.inner(rect));

        frame.render_widget(block, rect);

        self.update_width_and_heigth(bytes);

        frame.render_widget(self.addresses_paragraph(), address);
        frame.render_widget(self.memory_paragraph(), bytes);

        self.help.draw_if_open(frame);
        self.pointers.draw_if_open(frame);
    }
    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        if self.help.handle_keys_if_open(ctx) {
            return;
        }
        if self.pointers.handle_keys_if_open(ctx) {
            return;
        }

        let mut m = memory_viewer();

        if ctx.key_char('p') {
            self.pointers.show();
        }

        if ctx.key_char('l') {
            m.increment_highlighted(1);
        }

        if ctx.key_char('h') {
            if m.highlighted_offset == 0 {
                m.increment_base(-self.bytes_per_row);
            }
            m.increment_highlighted(-1);
        }

        if ctx.key_char('j') {
            m.increment_highlighted(self.bytes_per_row)
        }

        if ctx.key_char('k') {
            if m.highlighted_offset < self.bytes_per_row as u64 {
                m.increment_base(-self.bytes_per_row)
            }
            m.increment_highlighted(-self.bytes_per_row)
        }

        if ctx.key_f(1) {
            self.help.show();
        }

        if ctx.key_with_modifiers(KeyCode::Char('y'), KeyModifiers::CONTROL) {
            m.copy_absolute_address_at_highlighted();
        }

        if ctx.key_char('y') {
            m.copy_relative_address_at_highlighted();
        }

        if ctx.key_with_modifiers(KeyCode::Char('q'), KeyModifiers::CONTROL) {
            m.copy_qword_at_highlighted();
        }

        if ctx.key_with_modifiers(KeyCode::Char('d'), KeyModifiers::CONTROL) {
            m.copy_dword_at_highlighted();
        }

        if ctx.key_char('u') {
            m.jump_backwards();
        }

        if ctx.key_with_modifiers(KeyCode::Char('r'), KeyModifiers::CONTROL) {
            m.jump_forwards();
        }

        if ctx.key_with_modifiers(KeyCode::Char('b'), KeyModifiers::CONTROL) {
            m.jump_relative_i32_at_highlighted();
        }

        if ctx.key_char('b') {
            m.jump_absolute_at_highlighted();
        }

        if ctx.key_enter() {
            spawn_task! {
                if let Some(val) = request_input::<u8>(Some("Write byte")).await {
                    memory_viewer().write_at_highlighted::<u8>(val).send_error();
                }
            }
        } else if ctx.key_char('q') {
            spawn_task! {
                if let Some(val) = request_input::<u64>(Some("Write qword")).await {
                    memory_viewer().write_at_highlighted::<u64>(val).send_error();
                }
            }
        } else if ctx.key_char('d') {
            spawn_task! {
                if let Some(val) = request_input::<u32>(Some("Write dword")).await {
                    memory_viewer().write_at_highlighted::<u32>(val).send_error();
                }
            }
        } else if ctx.key_with_modifiers(KeyCode::Char('g'), KeyModifiers::CONTROL) {
            spawn_task! {
                if let Some(val) = request_input::<u64>(Some("Jump absolute")).await {
                    memory_viewer().jump(val);
                }
            }
        } else if ctx.key_char('g') {
            spawn_task! {
                if let Some(val) = request_input::<u64>(Some("Jump module relative")).await {
                    memory_viewer().jump_module_relative(val);
                }
            }
        } else if ctx.key_char('i') {
            spawn_task! {
                if let Some(val) = request_input::<i64>(Some("Increment selected")).await {
                    memory_viewer().increment_highlighted(val);
                }
            }
        }
    }
}

impl Popup for MemoryViewerScreen {
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn screen(&mut self) -> &mut dyn Screen {
        self
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Fill(1)])
            .split(frame.area());
        layout[1]
    }
    fn close_on_key(&self, ctx: &mut KeyContext) -> bool {
        ctx.key(KeyCode::Esc)
    }
}

impl MemoryViewerScreen {
    pub fn new() -> Self {
        Self {
            pointers:      PointersPopup::new(),
            help:          HelpPopup::new(&HELP_ENTRIES),
            bytes_per_row: 0,
            frame_heigth:  0,
            popup_state:   PopupState::default(),
        }
    }

    fn memory_paragraph(&self) -> Paragraph<'static> {
        let theme = theme();

        let m = memory_viewer();
        let mut spans = Vec::new();
        m.bytes.iter().enumerate().for_each(|(idx, byte)| {
            let address = m.base_address.saturating_add(idx as u64);
            let is_highlighted = m.highlighted_offset == idx as u64;

            let text_color = if m.changed_highlights.contains_key(&address) {
                theme.error
            } else if m.copied_highlights.contains_key(&address) {
                theme.success
            } else {
                theme.fg
            };

            let mut style = Style::new().fg(text_color);

            if is_highlighted {
                style.add_modifier = Modifier::REVERSED;
            }

            let content = if m.read_successful {
                format!("{:02x}", byte)
            } else {
                String::from("??")
            };

            spans.push(Span::styled(content, style));
            spans.push(Span::raw(" "))
        });
        Paragraph::new(Line::from(spans)).wrap(Wrap {
            trim: false,
        })
    }

    fn addresses_paragraph(&self) -> Paragraph<'static> {
        let mut lines = Vec::new();
        if let Some(rows) = memory_viewer::READ_SIZE.checked_div(self.bytes_per_row as usize) {
            for i in 0..rows {
                lines.push(Line::from(format!(
                    "{:#X}",
                    memory_viewer()
                        .base_address
                        .saturating_add(self.bytes_per_row as u64 * i as u64)
                )));
            }
        }
        Paragraph::new(lines).block(Block::new().borders(Borders::RIGHT))
    }

    fn update_width_and_heigth(&mut self, rect: Rect) {
        self.bytes_per_row = ((rect.width + 1) / 3) as i64;
        self.frame_heigth = rect.height as i64
    }

    fn current_row(&self, m: &MemoryViewer) -> Option<u64> {
        m.highlighted_offset
            .checked_div_euclid(self.bytes_per_row as u64)
    }
}
