use {
    crate::{
        event::KeyContext,
        impl_tablecontroller_for_commands,
        panes::{PaneManager, TablePane},
        screen::Screen,
    },
    crossterm::event::KeyCode,
    eldenring::player::{self, Stat},
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
    },
    shared::command::{Command, ValCmd},
};

pub(super) struct PlayerTab {
    pub pane_manager: PaneManager,
}

impl PlayerTab {
    pub fn new() -> Self {
        PlayerTab {
            pane_manager: PaneManager::new(vec![
                TablePane::new_static(&Toggles).boxed(),
                TablePane::new_static(&Actions).boxed(),
                TablePane::new_static(&Stats).with_title("Stats").boxed(),
            ]),
        }
    }
}

impl Screen for PlayerTab {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let [area_one, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(rect);

        let [area_two, area_three] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(right);

        let layout = [area_one, area_two, area_three];

        self.pane_manager.draw(frame, &layout);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

const TOGGLES: [Command; 16] = [
    Command::Toggle(&player::NoDeath),
    Command::Toggle(&player::NoDamage),
    Command::Toggle(&player::InfinitePoise),
    Command::Toggle(&player::OneShot),
    Command::Toggle(&player::RuneArc),
    Command::Toggle(&player::Silent),
    Command::Toggle(&player::Hidden),
    Command::Toggle(&player::InfiniteStamina),
    Command::Toggle(&player::InfiniteFp),
    Command::Toggle(&player::InfiniteConsumables),
    Command::Toggle(&player::InfiniteArrows),
    Command::Toggle(&player::NoRuneLossOnDeath),
    Command::Toggle(&player::NoTimePassOnDeath),
    Command::Toggle(&player::SetRfbsOnLoad),
    Command::Toggle(&player::TorrentAnywhere),
    Command::Toggle(&player::TorrentNoDeath),
];

const ACTIONS: [Command; 5] = [
    Command::Value(ValCmd::I32(&player::Health)),
    Command::Value(ValCmd::U32(&player::Runes)),
    Command::Value(ValCmd::F32(&player::AnimationSpeed)),
    Command::Unit(&player::Die),
    Command::Unit(&player::Rest),
];

const STATS: [Command; 10] = [
    Command::Stat(&Stat::Vigor),
    Command::Stat(&Stat::Mind),
    Command::Stat(&Stat::Endurance),
    Command::Stat(&Stat::Strength),
    Command::Stat(&Stat::Dexterity),
    Command::Stat(&Stat::Intelligence),
    Command::Stat(&Stat::Faith),
    Command::Stat(&Stat::Arcane),
    Command::Stat(&Stat::ScadutreeBlessing),
    Command::Stat(&Stat::ReveredSpiritAsh),
];

impl_tablecontroller_for_commands!(Toggles, TOGGLES);
impl_tablecontroller_for_commands!(Actions, ACTIONS);
impl_tablecontroller_for_commands!(Stats, STATS);
