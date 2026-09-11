use {
    crate::{
        event::{AnyhowExt, KeyContext, request_search},
        input::fuzzy_finder::SearchRequest,
        panes::{PaneManager, TableController, TablePane, TableView},
        screen::Screen,
        spawn_task,
        theme::theme,
    },
    darksouls3::resources::{bonfires, bosses},
    nucleo_matcher::Utf32String,
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        style::Stylize,
        widgets::{Cell, Row},
    },
    std::thread,
};

pub(super) struct TravelTab {
    pub pane_manager: PaneManager,
}

impl TravelTab {
    pub fn new() -> Self {
        TravelTab {
            pane_manager: PaneManager::new(vec![
                TablePane::new_static(&BossList)
                    .with_title("Bosses")
                    .freeze()
                    .boxed(),
                TablePane::new_static(&BonfireTable)
                    .with_title("Bonfires")
                    .freeze()
                    .boxed(),
            ]),
        }
    }
}

impl Screen for TravelTab {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(rect);

        self.pane_manager.draw(frame, &layout);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

struct BossList;
impl TableController for BossList {
    fn make_table_view(&self) -> TableView {
        let rows = bosses::BOSSES
            .iter()
            .map(|boss| Row::new([boss.name]))
            .collect();

        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            spawn_task! {
                bosses::BOSSES[selected]
                    .warp()
                    .await
                    .send_error()
            }
        }

        if ctx.key_char('f') {
            request_search(&BossesSearch);
        }
    }
}

struct BonfireTable;
impl TableController for BonfireTable {
    fn make_table_view(&self) -> TableView {
        let rows = bonfires::BONFIRES
            .iter()
            .map(|bonfire| {
                Row::new([
                    Cell::from(bonfire.name),
                    Cell::from(bonfire.main_area).fg(theme().muted),
                ])
            })
            .collect();
        TableView::new(rows).with_widths(&[Constraint::Min(30), Constraint::Max(26)])
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            thread::spawn(move || bonfires::BONFIRES[selected].warp().send_error());
        }

        if ctx.key_char('f') {
            request_search(&BonfireSearch);
        }
    }
}

struct BossesSearch;
impl SearchRequest for BossesSearch {
    fn items(&self) -> Vec<Utf32String> {
        bosses::BOSSES
            .iter()
            .map(|boss| Utf32String::from(boss.name))
            .collect()
    }
}

struct BonfireSearch;
impl SearchRequest for BonfireSearch {
    fn items(&self) -> Vec<Utf32String> {
        bonfires::BONFIRES
            .iter()
            .map(|bonfire| Utf32String::from(format!("{}|{}", bonfire.name, bonfire.main_area)))
            .collect()
    }
}
