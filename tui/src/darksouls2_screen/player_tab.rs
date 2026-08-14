use {
    crate::{
        event::{KeyContext, ResultExt},
        impl_tablecontroller_for_commands,
        panes::{PaneManager, TableController, TablePane, TableView},
        popup::{Popup, PopupState, centered_popup},
        screen::Screen,
    },
    crossterm::event::KeyCode,
    darksouls2::{
        covenant::{self, CovenantKind, covenants_with_progress},
        player::{self, Stat},
    },
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        style::Stylize,
        widgets::Row,
    },
    shared::command::{Command, ValCmd},
};

pub(super) struct PlayerTab {
    pub pane_manager: PaneManager,
    covenant_popup:   CovenantPopup,
}

impl PlayerTab {
    pub fn new() -> Self {
        PlayerTab {
            pane_manager:   PaneManager::new(vec![
                TablePane::new_static(&ToggleItems).boxed(),
                TablePane::new_static(&ActionItems).boxed(),
                TablePane::new_static(&StatItems)
                    .with_title("Stats")
                    .boxed(),
            ]),
            covenant_popup: CovenantPopup {
                pane:        TablePane::new_static(&CovenantTable),
                popup_state: PopupState::default(),
            },
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
        self.covenant_popup.draw_if_open(frame);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        if self.covenant_popup.handle_keys_if_open(ctx) {
            return;
        }

        if let Some(Command::Empty(_)) = self.pane_manager.current_command()
            && ctx.key_enter()
        {
            self.covenant_popup.show();
        }

        self.pane_manager.handle_keys(ctx);
    }
}

const TOGGLE_ITEMS: [Command; 10] = [
    Command::Toggle(&player::NoDeath),
    Command::Toggle(&player::NoDamage),
    Command::Toggle(&player::InfinitePoise),
    Command::Toggle(&player::InfiniteStamina),
    Command::Toggle(&player::InfiniteDurability),
    Command::Toggle(&player::InfiniteConsumables),
    Command::Toggle(&player::NoHollowing),
    Command::Toggle(&player::NoSoulLoss),
    Command::Toggle(&player::Hidden),
    Command::Toggle(&player::Silent),
];

const ACTION_ITEMS: [Command; 3] = [
    Command::Value(ValCmd::I32(&player::Health)),
    Command::Value(ValCmd::U32(&player::Souls)),
    Command::Empty(&covenant::Covenant),
];

const STAT_ITEMS: [Command; 9] = [
    Command::Stat(&Stat::Vigor),
    Command::Stat(&Stat::Endurance),
    Command::Stat(&Stat::Vitality),
    Command::Stat(&Stat::Attunement),
    Command::Stat(&Stat::Strength),
    Command::Stat(&Stat::Dexterity),
    Command::Stat(&Stat::Intelligence),
    Command::Stat(&Stat::Faith),
    Command::Stat(&Stat::Adaptability),
];

impl_tablecontroller_for_commands!(ToggleItems, TOGGLE_ITEMS);
impl_tablecontroller_for_commands!(ActionItems, ACTION_ITEMS);
impl_tablecontroller_for_commands!(StatItems, STAT_ITEMS);

struct CovenantTable;
impl TableController for CovenantTable {
    fn make_table_view(&self) -> TableView {
        let covenants = covenants_with_progress();
        let mut rows: Vec<Row> = Vec::new();

        for info in covenants {
            let covenant = if info.covenant == covenant::Covenant.get().unwrap_or_default() {
                format!("*{}", info.covenant)
            } else {
                format!("{}", info.covenant)
            };
            let progress = match info.progress {
                Some(v) => format!("{:02}", v),
                None => "".to_string(),
            };
            let rank = match info.rank {
                Some(v) => format!("{}", v),
                None => "".to_string(),
            };
            let row = Row::new([covenant, progress, rank]);
            rows.push(row);
        }
        let header = Row::new(["Covenant", "Progress", "Rank"]).bold();

        TableView::new(rows).with_header(header).with_widths(&[
            Constraint::Min(22),
            Constraint::Max(10),
            Constraint::Max(5),
        ])
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            let covenant = CovenantKind::try_from(selected as u8).unwrap();
            covenant::Covenant.set(covenant).send_error();
        }
    }
}

struct CovenantPopup {
    pane:        TablePane,
    popup_state: PopupState,
}

impl Popup for CovenantPopup {
    fn screen(&mut self) -> &mut dyn Screen {
        &mut self.pane
    }
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        centered_popup(60, 60, frame.area())
    }
}
