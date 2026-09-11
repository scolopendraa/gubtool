use {
    crate::{
        event::KeyContext,
        impl_tablecontroller_for_commands,
        panes::{PaneManager, TablePane},
        screen::Screen,
    },
    crossterm::event::KeyCode,
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
    },
    shared::command::Command,
};

pub(super) struct PlayerTab {
    pub pane_manager: PaneManager,
}

impl PlayerTab {
    pub fn new() -> Self {
        PlayerTab {
            pane_manager: PaneManager::new(vec![
                TablePane::new_static(&ToggleItems).boxed(),
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

        let layout = [area_one];

        self.pane_manager.draw(frame, &layout);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

const TOGGLE_ITEMS: [Command; 7] = [
    Command::Toggle(&darksouls3::player::NoDeath),
    Command::Toggle(&darksouls3::player::OneShot),
    Command::Toggle(&darksouls3::player::InfiniteStamina),
    Command::Toggle(&darksouls3::player::InfiniteFp),
    Command::Toggle(&darksouls3::player::InfiniteArrows),
    Command::Toggle(&darksouls3::player::Hidden),
    Command::Toggle(&darksouls3::player::Silent),
    // Command::Toggle(&darksouls3::player::Ember),
];

impl_tablecontroller_for_commands!(ToggleItems, TOGGLE_ITEMS);
