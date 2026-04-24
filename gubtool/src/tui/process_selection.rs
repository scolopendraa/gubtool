use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Cell, Clear, Paragraph, Row, Table}
};
use crate::{
    core::attach::{self, ATTACHED_PROCESS, GameProcess},
    tui::{
        app::{App, CurrentScreen},
        theme::{self, theme},
        block, centered_rect, controls_line, ResultExt,
    }
};

const CONTROLS: &[(&str, &str)] = &[
    ("Enter", "Attach"),
    ("q", "Exit"),
];

pub fn draw(frame: &mut Frame, app: &mut App) {
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

    frame.render_stateful_widget(table(&app.available_processes), layout, &mut app.processes_state);
    frame.render_widget(path_paragraph(&app.available_processes, app.processes_state.selected()), path_area);
    frame.render_widget(controls_line(CONTROLS), help_area);
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Char('j') | KeyCode::Down, _) => app.processes_state.select_next(),
        (KeyCode::Char('k') | KeyCode::Up, _) => app.processes_state.select_previous(),
        (KeyCode::Char('q') | KeyCode::Esc, _) => app.current_screen = CurrentScreen::Tab,
        (KeyCode::Enter, _) => {
            let tx = app.sender.clone();
            if let Some(selected) = app.processes_state.selected() {
                let mut processes = attach::get_processes();
                if selected < processes.len() {
                    let process = processes.remove(selected);
                    app.try_attach(Some(process), false).send_error(tx);
                }
            }
        },
        _ => ()
    }
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
            Cell::from(format!("{}", process.version)),
            Cell::from(process.pid.to_string()),
        ]);
        rows.push(row);
    }
    let header = Row::new(vec![
        Cell::from("Name"),
        Cell::from("Game Version"),
        Cell::from("PID"),
    ]).bold();
    let widths = [
        Constraint::Min(16),
        Constraint::Min(10),
        Constraint::Min(11),
    ];
    Table::new(rows, widths)
        .header(header)
        .highlight_symbol(theme::HIGHLIGHT_SYMBOL)
        .row_highlight_style(Style::from(theme().accent).bold())
        .block(block(Some("Valid Processes"), None))
}
