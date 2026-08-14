use {
    crate::{
        event::{AnyhowExt, KeyContext},
        impl_tablecontroller_for_commands,
        panes::{PaneManager, TabPane, TableController, TablePane, TableView},
        screen::Screen,
    },
    crossterm::event::KeyCode,
    eldenring::{
        resources::talk_commands::{MENUS, SHOPS},
        utility,
    },
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        widgets::Row,
    },
    shared::command::{Command, ValCmd},
};

pub(super) struct UtilityTab {
    pub pane_manager: PaneManager,
}

impl UtilityTab {
    pub fn new() -> Self {
        UtilityTab {
            pane_manager: PaneManager::new(vec![
                TablePane::new_static(&Toggles).boxed(),
                TablePane::new_static(&Actions).boxed(),
                TabPane::new(&["Menus", "Shops"], vec![
                    TablePane::new_static(&MenusList).freeze(),
                    TablePane::new_static(&ShopsList).freeze(),
                ])
                .boxed(),
            ]),
        }
    }
}

impl Screen for UtilityTab {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let [area_one, right_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(rect);

        let [area_two, area_three] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(right_area);

        let layout = [area_one, area_two, area_three];

        self.pane_manager.draw(frame, &layout);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

const TOGGLES: [Command; 13] = [
    Command::Toggle(&utility::DisableLogos),
    Command::Toggle(&utility::MuteMusic),
    Command::Toggle(&utility::ShowAllMaps),
    Command::Toggle(&utility::ShowAllGraces),
    Command::Toggle(&utility::StutterFix),
    Command::Toggle(&utility::FreezeWorld),
    Command::Toggle(&utility::DisableAreaWelcomeMessage),
    Command::Toggle(&utility::DrawHitboxes),
    Command::Toggle(&utility::MapInCombat),
    Command::Toggle(&utility::TravelInDungeons),
    Command::Toggle(&utility::DisableRoll),
    Command::Toggle(&utility::DisableJump),
    Command::Toggle(&utility::DisableBackstep),
];

const ACTIONS: [Command; 5] = [
    Command::Value(ValCmd::F32(&utility::FpsCap)),
    Command::Value(ValCmd::F32(&utility::GameSpeed)),
    Command::Unit(&utility::Quitout),
    Command::Value(ValCmd::I32(&utility::ClearCount)),
    Command::Unit(&utility::TriggerNewGameCycle),
];

impl_tablecontroller_for_commands!(Toggles, TOGGLES);
impl_tablecontroller_for_commands!(Actions, ACTIONS);

struct MenusList;
impl TableController for MenusList {
    fn make_table_view(&self) -> TableView {
        let rows = MENUS.iter().map(|menu| Row::new([menu.name])).collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            MENUS[selected].execute().send_error();
        }
    }
}

struct ShopsList;
impl TableController for ShopsList {
    fn make_table_view(&self) -> TableView {
        let rows = SHOPS.iter().map(|shop| Row::new([shop.name])).collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            SHOPS[selected].execute().send_error();
        }
    }
}
