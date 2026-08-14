use {
    crate::{
        common::controls::Control,
        event::{Event, KeyContext, ResultExt, send_event},
        panes::{Pane, TableController, TablePane, TableView},
        popup::{Popup, PopupState, centered_popup},
        screen::Screen,
    },
    crossterm::event::{KeyCode, KeyModifiers},
    gubtool_core::attached,
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Margin, Rect},
        style::Stylize,
        widgets::{Block, Borders, Paragraph, Row, Wrap},
    },
};

const CONTROLS: [Control; 2] = [
    Control::new("Enter", "Attach"),
    Control::new("ctrl-k", "Kill"),
];

pub struct ProcessSelector {
    table:       TablePane,
    popup_state: PopupState,
}

struct ProcessTable;
impl TableController for ProcessTable {
    fn make_table_view(&self) -> TableView {
        attached::refresh_processes();

        let processes = attached::game_processes();
        let rows: Vec<Row> = processes
            .iter()
            .map(|process| {
                let comm = if attached::pid() == Ok(process.pid) {
                    format!("*{}", process.comm)
                } else {
                    format!(" {}", process.comm)
                };
                Row::new([
                    comm,
                    process.pid.to_string(),
                    format!("{}", process.game_version),
                ])
            })
            .collect::<Vec<Row>>();

        let header = Row::new(["Name", "PID", "Game Version"]).bold();

        TableView::new(rows).with_header(header).with_widths(&[
            Constraint::Min(28),
            Constraint::Min(10),
            Constraint::Min(28),
        ])
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            let processes = attached::game_processes();
            if selected < processes.len() {
                processes[selected].attach().send_error();
                send_event(Event::Attach);
            }
        }

        if ctx.key_with_modifiers(KeyCode::Char('k'), KeyModifiers::CONTROL) {
            let processes = attached::game_processes();
            if selected < processes.len() {
                processes[selected].kill();
            }
        }
    }
}

impl Popup for ProcessSelector {
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn screen(&mut self) -> &mut dyn Screen {
        &mut self.table
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        centered_popup(75, 75, frame.area())
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        if self.table.selected().is_none() {
            self.table.select(0);
        }
        self.table.draw(frame, rect);

        let [_processes_area, path_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(4)])
            .areas(rect.inner(Margin::new(1, 1)));

        let text = {
            let processes = attached::game_processes();
            if let Some(idx) = self.table.selected()
                && idx < processes.len()
            {
                format!("{}", processes[idx].exe_path.display())
            } else {
                "".to_string()
            }
        };
        let path = Paragraph::new(text)
            .wrap(Wrap {
                trim: true,
            })
            .block(Block::new().borders(Borders::TOP));

        frame.render_widget(path, path_area);
    }
}

impl ProcessSelector {
    pub fn new() -> Self {
        Self {
            table:       TablePane::new_static(&ProcessTable)
                .with_title("Process Selection")
                .with_controls(&CONTROLS),
            popup_state: PopupState::default(),
        }
    }
}
