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
    darksouls2::{
        item::{self, ItemSpawnRequest},
        resources::items::{Categories, ITEMS, infusions::Infusion},
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

static SPAWN_REQUEST: LazyLock<Mutex<ItemSpawnRequest>> = LazyLock::new(|| {
    Mutex::new(ItemSpawnRequest {
        item:     ITEMS[0],
        quantity: 1,
        upgrade:  0,
        infusion: Infusion::Normal,
    })
});

pub(super) struct ItemTab {
    pub pane_manager: PaneManager,
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
            .constraints(vec![Constraint::Percentage(65), Constraint::Fill(1)])
            .areas(layout);

        let [options, mass_spawn] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(5), Constraint::Fill(1)])
            .areas(right_area);

        self.pane_manager
            .draw(frame, &[item_area, options, mass_spawn]);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

impl ItemTab {
    pub fn new() -> Self {
        ItemTab {
            pane_manager: PaneManager::new(vec![
                TablePane::new_static(&ItemSelector)
                    .with_title("Items")
                    .freeze()
                    .boxed(),
                TablePane::new_static(&OptionsItems).boxed(),
                TablePane::new_static(&MassSpawnItems)
                    .with_title("Mass Spawn")
                    .freeze()
                    .boxed(),
            ]),
        }
    }
}

struct ItemSelector;
impl TableController for ItemSelector {
    fn make_table_view(&self) -> TableView {
        let rows = ITEMS
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
            let item = SPAWN_REQUEST.lock().unwrap();
            item.spawn().send_error();
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
            Row::new([format!("Infusion: {}", item.infusion)])
                .style(item_options_style(item.can_infuse())),
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
                    if let Some(val) = request_input::<u32>(None).await {
                        let mut item = SPAWN_REQUEST.lock().unwrap();
                        item.quantity = val;
                    }
                }
            }
            1 if item.can_upgrade() => {
                drop(item);
                spawn_task! {
                    if let Some(val) = request_input::<u32>(None).await {
                        let mut item = SPAWN_REQUEST.lock().unwrap();
                        item.upgrade = val;
                    }
                }
            }
            2 if item.can_infuse() => request_search(&InfusionSearch),
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

struct InfusionSearch;
impl SearchRequest for InfusionSearch {
    fn items(&self) -> Vec<Utf32String> {
        SPAWN_REQUEST
            .lock()
            .unwrap()
            .item
            .available_infusions()
            .iter()
            .map(|infusion| Utf32String::from(format!("{}", infusion)))
            .collect()
    }
    fn jump(&self, _app: &mut App, selected: usize) {
        let mut spawn_request = SPAWN_REQUEST.lock().unwrap();
        let infusions = spawn_request.item.available_infusions();
        spawn_request.infusion = infusions[selected];
    }
}

struct MassSpawnItems;
impl TableController for MassSpawnItems {
    fn make_table_view(&self) -> TableView {
        let rows = Categories::ARRAY
            .iter()
            .map(|item| Row::new([item.to_string()]))
            .collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            thread::spawn(move || {
                let guard = SPAWN_REQUEST.lock().unwrap();
                let item = guard.clone();
                drop(guard);
                item::mass_spawn(
                    Categories::ARRAY[selected],
                    item.quantity,
                    item.upgrade,
                    item.infusion,
                )
                .send_error();
            });
        }
    }
}
