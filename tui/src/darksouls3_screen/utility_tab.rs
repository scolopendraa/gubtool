use {
    crate::{
        common::controls::Control,
        event::{AnyhowExt, KeyContext},
        impl_tablecontroller_for_commands,
        panes::{PaneManager, TabPane, TableController, TablePane, TableView},
        screen::Screen,
    },
    crossterm::event::KeyCode,
    darksouls3::{resources::talk_commands, utility},
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        widgets::Row,
    },
    shared::command::{Command, ToggleCommand, ValCmd},
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
                    TablePane::new_static(&MenuItems).freeze(),
                    TablePane::new_static(&ShopItems)
                        .freeze()
                        .with_controls(&SHOP_CONTROLS),
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

const TOGGLES: [Command; 1] = [Command::Toggle(&utility::DisableCameraFollow)];

const ACTIONS: [Command; 4] = [
    Command::Value(ValCmd::F32(&utility::FpsCap)),
    Command::Value(ValCmd::F32(&utility::GameSpeed)),
    Command::Value(ValCmd::U32(&utility::ClearCount)),
    Command::Unit(&utility::Quitout),
];

impl_tablecontroller_for_commands!(Toggles, TOGGLES);
impl_tablecontroller_for_commands!(Actions, ACTIONS);

struct MenuItems;
impl TableController for MenuItems {
    fn make_table_view(&self) -> TableView {
        let rows = talk_commands::MENUS
            .iter()
            .map(|menu| Row::new([menu.name]))
            .collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            talk_commands::MENUS[selected].execute().send_error();
        }
    }
}

const SHOP_CONTROLS: [Control; 1] =
    [Control::new("r", "Access Full Shops").with_toggle(&utility::AccessFullShops)];

struct ShopItems;
impl TableController for ShopItems {
    fn make_table_view(&self) -> TableView {
        let rows = talk_commands::SHOPS
            .iter()
            .map(|shop| Row::new([shop.name]))
            .collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            talk_commands::SHOPS[selected].execute().send_error();
        }

        if ctx.key_char('r') {
            utility::AccessFullShops.toggle().send_error();
        }
    }
}
