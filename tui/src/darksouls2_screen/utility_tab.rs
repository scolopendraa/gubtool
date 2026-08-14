use {
    crate::{
        event::{AnyhowExt, KeyContext},
        impl_tablecontroller_for_commands,
        panes::{PaneManager, TabPane, TableController, TablePane, TableView},
        screen::Screen,
    },
    crossterm::event::KeyCode,
    darksouls2::{
        menu,
        resources::menus::{MENUS, SHOPS, TRADES},
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
                TablePane::new_static(&ToggleItems).boxed(),
                TablePane::new_static(&ActionItems).boxed(),
                TabPane::new(&["Menus", "Shops", "Trades"], vec![
                    TablePane::new_static(&MenuItems).freeze(),
                    TablePane::new_static(&ShopItems).freeze(),
                    TablePane::new_static(&TradeItems).freeze(),
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

const TOGGLE_ITEMS: [Command; 4] = [
    Command::Toggle(&utility::FastQuitout),
    Command::Toggle(&utility::SkipCredits),
    Command::Toggle(&utility::DisableRoll),
    Command::Toggle(&utility::DisableBackstep),
];

const ACTION_ITEMS: [Command; 2] = [
    Command::Value(ValCmd::U8(&utility::NewGame)),
    Command::Unit(&utility::Quitout),
];

impl_tablecontroller_for_commands!(ToggleItems, TOGGLE_ITEMS);
impl_tablecontroller_for_commands!(ActionItems, ACTION_ITEMS);

struct ShopItems;
impl TableController for ShopItems {
    fn make_table_view(&self) -> TableView {
        let rows = SHOPS
            .iter()
            .map(|shop| Row::new([shop.to_string()]))
            .collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            menu::open_shop(SHOPS[selected]).send_error();
        }
    }
}

struct MenuItems;
impl TableController for MenuItems {
    fn make_table_view(&self) -> TableView {
        let rows = MENUS
            .iter()
            .map(|menu| Row::new([menu.to_string()]))
            .collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            menu::open_menu(MENUS[selected]).send_error();
        }
    }
}

struct TradeItems;
impl TableController for TradeItems {
    fn make_table_view(&self) -> TableView {
        let rows = TRADES
            .iter()
            .map(|trade| Row::new([trade.to_string()]))
            .collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            menu::open_trade(TRADES[selected]).send_error();
        }
    }
}
