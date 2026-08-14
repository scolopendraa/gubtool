use {
    crate::{
        common::event_log_table::draw_logging_enabled_line,
        event::KeyContext,
        impl_tablecontroller_for_commands,
        panes::{PaneManager, TablePane},
        screen::Screen,
    },
    crossterm::event::KeyCode,
    eldenring::event::{self, ErEventLogger},
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
    },
    shared::command::{Command, ToggleCommand},
};

pub(super) struct EventTab {
    pub pane_manager: PaneManager,
}

impl EventTab {
    pub fn new() -> Self {
        EventTab {
            pane_manager: PaneManager::new(vec![
                TablePane::new_static(&Commands).boxed(),
                TablePane::event_logs(ErEventLogger::default()).boxed(),
            ]),
        }
    }
}

impl Screen for EventTab {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(rect);

        self.pane_manager.draw(frame, &layout);

        let enabled = event::EventLogHook.is().unwrap_or_default();
        draw_logging_enabled_line(frame, layout[1], enabled);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

const COMMANDS: [Command; 4] = [
    Command::Toggle(&event::DlcClear),
    Command::Unit(&event::FightEldenBeast),
    Command::Unit(&event::FightFortissax),
    Command::Unit(&event::UnlockMetyr),
];

impl_tablecontroller_for_commands!(Commands, COMMANDS);
