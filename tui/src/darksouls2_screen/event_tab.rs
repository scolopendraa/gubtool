use {
    crate::{
        common::event_log_table::draw_logging_enabled_line,
        event::KeyContext,
        impl_tablecontroller_for_commands,
        panes::{PaneManager, TablePane},
        screen::Screen,
    },
    crossterm::event::KeyCode,
    darksouls2::event::{self, Ds2EventLogger},
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
                TablePane::event_logs(Ds2EventLogger::default()).boxed(),
            ]),
        }
    }
}

impl Screen for EventTab {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rect);
        self.pane_manager.draw(frame, &layout);

        let enabled = event::StartEventLogger.is().unwrap_or_default();
        draw_logging_enabled_line(frame, layout[1], enabled);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

const COMMANDS: [Command; 17] = [
    Command::Toggle(&event::KingsRingAcquired),
    Command::Toggle(&event::NashandraUnlocked),
    Command::Toggle(&event::AldiaUnlocked),
    Command::Toggle(&event::FlexileShipDocked),
    Command::Toggle(&event::BelfryGargoylesGateOpen),
    Command::Toggle(&event::MythaPoisonDrained),
    Command::Toggle(&event::BrumeTowerActivated),
    Command::Toggle(&event::AavaVisible),
    Command::Toggle(&event::UndoAlsanasSeal),
    Command::Toggle(&event::SkipIvoryKingGauntlet),
    Command::Toggle(&event::DisableLoyceKnights),
    Command::Toggle(&event::DarkChasmLitShadedWoods),
    Command::Toggle(&event::DarkChasmLitDrangleicCastle),
    Command::Toggle(&event::DarkChasmLitBlackGulch),
    Command::Toggle(&event::FreeLoyceKnightOuterWall),
    Command::Toggle(&event::FreeLoyceKnightAbandonedDwelling),
    Command::Toggle(&event::FreeLoyceKnightLowerGarrison),
];

impl_tablecontroller_for_commands!(Commands, COMMANDS);
