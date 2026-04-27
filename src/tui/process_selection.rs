use crate::{
    core::attach::{self, ATTACHED_PROCESS, GameProcess},
    tui::{
        common::{block, centered_rect, controls_line},
        theme::{self, theme},
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Cell, Clear, Paragraph, Row, Table, TableState},
};

const CONTROLS: &[(&str, &str)] = &[
    ("Enter", "Attach"),
    ("q", "Exit"),
];

pub struct ProcessSelector {
    pub state: TableState,
    pub available_processes: Vec<GameProcess>,
}

impl ProcessSelector {
    pub fn new() -> Self {
        Self {
            state: TableState::default().with_selected(0),
            available_processes: Vec::new(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.available_processes = attach::get_processes();
        let layout = centered_rect(75, 75, frame.area());
        frame.render_widget(Clear, layout);
        let [_padding, path_area, help_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .areas(layout);

        frame.render_stateful_widget(Self::table(&self.available_processes), layout, &mut self.state);
        frame.render_widget(Self::path_paragraph(&self.available_processes, self.state.selected()), path_area);
        frame.render_widget(controls_line(CONTROLS), help_area);
    }

    fn path_paragraph(processes: &[GameProcess], selected: Option<usize>) -> Paragraph<'static> {
        let text = {
            if let Some(idx) = selected && idx < processes.len() {
                format!("{}", processes[idx].path.display())
            } else {
                "".to_string()
            }
        };
        Paragraph::new(text).alignment(Alignment::Center)
    }

    fn table(processes: &[GameProcess]) -> Table<'static> {
        let mut rows: Vec<Row> = Vec::new();
        for process in processes {
            let comm = unsafe {
                if ATTACHED_PROCESS.pid == process.pid {
                    format!("*{}", process.comm)
                } else {
                    format!(" {}", process.comm)
                }
            };
            let row = Row::new(vec![
                Cell::from(comm),
                Cell::from(process.pid.to_string()),
                Cell::from(format!("{}", process.version)),
            ]);
            rows.push(row);
        }
        let header = Row::new(vec![
            Cell::from("Name"),
            Cell::from("PID"),
            Cell::from("Game Version"),
        ]).bold();
        let widths = [
            Constraint::Max(20),
            Constraint::Max(10),
            Constraint::Fill(1),
        ];
        Table::new(rows, widths)
            .header(header)
            .highlight_symbol(theme::HIGHLIGHT_SYMBOL)
            .row_highlight_style(Style::from(theme().accent).bold())
            .block(block(Some("Valid Processes"), None))
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            (KeyCode::Char('j') | KeyCode::Down, _) => self.state.select_next(),
            (KeyCode::Char('k') | KeyCode::Up, _) => self.state.select_previous(),
            _ => ()
        }
    }
}