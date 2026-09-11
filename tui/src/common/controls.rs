use {
    crate::{
        event::KeyContext,
        panes::{TableController, TablePane, TableView},
        popup::{Popup, PopupState, centered_popup},
        screen::Screen,
        theme::theme,
    },
    ratatui::{
        Frame,
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        text::{Line, Span},
        widgets::{Cell, Row},
    },
    shared::command::ToggleCommand,
};

pub struct Control {
    key:    &'static str,
    action: &'static str,
    toggle: Option<&'static dyn ToggleCommand>,
}

impl Control {
    pub const fn new(key: &'static str, action: &'static str) -> Self {
        Self {
            key,
            action,
            toggle: None,
        }
    }
    pub const fn with_toggle(mut self, command: &'static dyn ToggleCommand) -> Self {
        self.toggle = Some(command);
        self
    }
}

pub fn draw_controls(frame: &mut Frame, rect: Rect, controls: &[Control]) {
    let controls = controls_line(controls);
    let [_, controls_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
        .areas(rect);
    frame.render_widget(controls, controls_area);
}

fn controls_line(controls: &[Control]) -> Line<'static> {
    let fg = theme().fg;
    let mut spans: Vec<Span> = controls
        .iter()
        .flat_map(|control| {
            let mut ctrl = vec![
                Span::raw("[").style(fg),
                Span::raw(control.key.to_string()).style(theme().info),
                Span::raw("→ ").style(fg),
                Span::raw(control.action.to_string()).style(fg),
            ];

            if let Some(toggle) = control.toggle {
                let state = toggle.is().unwrap_or_default();
                let state_str = if state { "[X]" } else { "[ ]" };
                ctrl.push(Span::raw(state_str).style(theme().secondary));
            }

            ctrl.push(Span::raw("] ").style(fg));
            ctrl
        })
        .collect();

    spans.pop();
    spans.push(Span::raw("]").style(fg));
    Line::from(spans).alignment(Alignment::Center)
}

pub struct HelpPopup {
    pane:        TablePane,
    popup_state: PopupState,
}

impl HelpPopup {
    pub fn new(entries: &'static [Control]) -> Self {
        let controller = HelpController::new(entries);

        let pane = TablePane::new_owned(controller)
            .as_non_selectable()
            .with_title("Controls")
            .freeze();
        Self {
            pane,
            popup_state: PopupState::default(),
        }
    }
}

struct HelpController {
    entries: &'static [Control],
}

impl HelpController {
    const fn new(entries: &'static [Control]) -> Self {
        Self {
            entries,
        }
    }
}

impl TableController for HelpController {
    fn make_table_view(&self) -> TableView {
        let mut max_key_len = 0;
        let mut max_action_len = 0;
        let rows: Vec<Row> = self
            .entries
            .iter()
            .map(|f| {
                let key_len = f.key.len();
                if key_len > max_key_len {
                    max_key_len = key_len
                }

                let action_len = f.action.len();
                if action_len > max_action_len {
                    max_action_len = action_len
                }

                Row::new([
                    Cell::from(f.key).style(theme().info),
                    Cell::from(f.action).style(theme().fg),
                ])
            })
            .collect();

        TableView::new(rows).with_widths(vec![
            Constraint::Min(max_key_len as u16 + 2),
            Constraint::Min(max_action_len as u16 + 2),
        ])
    }
    fn handle_keys_selected(&self, _selected: usize, _ctx: &mut KeyContext) {}
}

impl Popup for HelpPopup {
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn screen(&mut self) -> &mut dyn Screen {
        &mut self.pane
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        centered_popup(60, 75, frame.area())
    }
    fn close_on_key(&self, ctx: &mut KeyContext) -> bool {
        ctx.key_any()
    }
}
