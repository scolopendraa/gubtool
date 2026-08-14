use {
    crate::{
        app::App,
        common::helpers::item_options_style,
        event::{AnyhowExt, KeyContext, request_search},
        input::{fuzzy_finder::SearchRequest, request_input},
        panes::{PaneManager, TableController, TablePane, TableView},
        screen::Screen,
        spawn_task,
        theme::theme,
    },
    eldenring::{
        item::{self, ItemSpawnRequest},
        resources::items::{Categories, ITEMS},
    },
    nucleo_matcher::Utf32String,
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        style::Stylize,
        widgets::{Cell, Row},
    },
    std::{
        sync::{LazyLock, Mutex},
        thread,
    },
};

static SPAWN_REQUEST: LazyLock<Mutex<ItemSpawnRequest>> =
    LazyLock::new(|| Mutex::new(ItemSpawnRequest::new(ITEMS[0])));

pub(super) struct ItemTab {
    pub pane_manager: PaneManager,
}

impl ItemTab {
    pub fn new() -> Self {
        ItemTab {
            pane_manager: PaneManager::new(vec![
                TablePane::new_static(&ItemsTable).freeze().boxed(),
                TablePane::new_static(&OptionsItems).boxed(),
                TablePane::new_static(&MassSpawnList)
                    .freeze()
                    .with_title("Mass Spawn")
                    .boxed(),
            ]),
        }
    }
}

impl Screen for ItemTab {
    fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        {
            let mut item = SPAWN_REQUEST.lock().unwrap();
            let item_list_idx = self.pane_manager.get_list_selected(0);
            if let Some(idx) = item_list_idx {
                item.item = ITEMS[idx];
            }
            item.clamp_values();
        }

        let [item_area, right_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(60), Constraint::Fill(1)])
            .areas(layout);

        let [options, mass_spawn] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(6), Constraint::Fill(1)])
            .areas(right_area);
        let layout = [item_area, options, mass_spawn];

        self.pane_manager.draw(frame, &layout);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

struct ItemsTable;
impl TableController for ItemsTable {
    fn make_table_view(&self) -> TableView {
        let rows: Vec<Row> = ITEMS
            .iter()
            .map(|item| {
                Row::new([
                    Cell::from(item.name),
                    Cell::from(format!("{}", item.category)).fg(theme().muted),
                ])
            })
            .collect();

        TableView::new(rows).with_widths(&[Constraint::Min(40), Constraint::Max(25)])
    }
    fn handle_keys_selected(&self, _selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            SPAWN_REQUEST.lock().unwrap().spawn().send_error();
        }

        if ctx.key_char('f') {
            request_search(&ItemSearch);
        }
    }
}

struct OptionsItems;
impl TableController for OptionsItems {
    fn make_table_view(&self) -> TableView {
        let item = SPAWN_REQUEST.lock().unwrap();

        let items = vec![
            Row::new([format!("Quantity: {}", item.quantity)])
                .style(item_options_style(item.can_quantity())),
            Row::new([format!("Upgrade: {}", item.upgrade)])
                .style(item_options_style(item.can_upgrade())),
            Row::new([format!("Ash Of War: {}", item.aow.name)])
                .style(item_options_style(item.can_aow())),
            Row::new([format!("Affinity: {}", item.affinity.name)])
                .style(item_options_style(item.can_aow())),
        ];
        TableView::new(items)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if !ctx.key_enter() {
            return;
        }

        let item = SPAWN_REQUEST.lock().unwrap();
        match selected {
            0 if item.can_quantity() => {
                drop(item);
                spawn_task! {
                    if let Some(val) = request_input::<i64>(None).await {
                        let mut item = SPAWN_REQUEST.lock().unwrap();
                        item.quantity = val;
                    }
                }
            }
            1 if item.can_upgrade() => {
                drop(item);
                spawn_task! {
                    if let Some(val) = request_input::<i64>(None).await {
                        let mut item = SPAWN_REQUEST.lock().unwrap();
                        item.upgrade = val;
                    }
                }
            }
            2 if item.can_aow() => request_search(&AowSearch),
            3 if item.can_aow() => request_search(&AffinitySearch),
            _ => (),
        }
    }
}

struct ItemSearch;
impl SearchRequest for ItemSearch {
    fn items(&self) -> Vec<Utf32String> {
        ITEMS
            .iter()
            .map(|item| Utf32String::from(format!("{}|{}", item.name, item.category)))
            .collect()
    }
}

struct AowSearch;
impl SearchRequest for AowSearch {
    fn items(&self) -> Vec<Utf32String> {
        SPAWN_REQUEST
            .lock()
            .unwrap()
            .item
            .valid_aows()
            .iter()
            .map(|aow| Utf32String::from(aow.name))
            .collect()
    }
    fn jump(&self, _app: &mut App, selected: usize) {
        let mut spawn_request = SPAWN_REQUEST.lock().unwrap();
        spawn_request.aow = spawn_request.item.valid_aows()[selected];
    }
}

struct AffinitySearch;
impl SearchRequest for AffinitySearch {
    fn items(&self) -> Vec<Utf32String> {
        SPAWN_REQUEST
            .lock()
            .unwrap()
            .aow
            .valid_affinities()
            .iter()
            .map(|affinity| Utf32String::from(affinity.name))
            .collect()
    }
    fn jump(&self, _app: &mut App, selected: usize) {
        let mut spawn_request = SPAWN_REQUEST.lock().unwrap();
        spawn_request.affinity = spawn_request.aow.valid_affinities()[selected];
    }
}

struct MassSpawnList;
impl TableController for MassSpawnList {
    fn make_table_view(&self) -> TableView {
        let rows = Categories::ARRAY
            .iter()
            .map(|item| Row::new([item.to_string()]))
            .collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            let item = SPAWN_REQUEST.lock().unwrap();
            let quantity = item.quantity;
            let upgrade = item.upgrade;

            thread::spawn(move || {
                item::mass_spawn(Categories::ARRAY[selected], quantity, upgrade).send_error();
            });
        }
    }
}
