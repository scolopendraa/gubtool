use std::thread;
use crossterm::event::{KeyCode, KeyEvent};
use nucleo_matcher::Utf32String;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::Line,
    widgets::{List, ListItem, ListState},
    Frame,
};
use crate::{
    core::item,
    resources::{
        self,
        aow::{AFFINITIES, Affinity, Aow, aow_array},
        items::{Categories, Item, items_array},
    },
    tui::{
        app::App,
        fuzzy_finder::Picker,
        input_handler::InputField,
        tabs::TabState,
        block, blockless_list, label_list, list, ResultExt, theme,
    },
};

enum OptionsItems {
    Quantity,
    Upgrade,
    AshOfWar,
    Affinity,
}

#[derive(Clone)]
pub struct ItemTab {
    tab: TabState,
    item: Item,
    pub quantity: i64,
    pub upgrade: i64,
    aow: Aow,
    affinity: Affinity,
}

const ITEMS_IDX: usize = 0;
const OPTIONS_IDX: usize = 1;
const MASS_SPAWN_IDX: usize = 2;

impl ItemTab {
    pub fn new() -> Self {
        let mut sizes = vec![0; 3];
        sizes[ITEMS_IDX] = 0;
        sizes[OPTIONS_IDX] = OptionsItems::ARRAY.len();
        sizes[MASS_SPAWN_IDX] = Categories::ARRAY.len();
        ItemTab {
            tab: TabState {
                lists: vec![ListState::default().with_selected(Some(0)); 3],
                list_sizes: sizes,
                ..TabState::default()
            },
            item: items_array(false)[0],
            quantity: 1,
            upgrade: 0,
            aow: aow_array()[0],
            affinity: AFFINITIES[0],
        }
    }

    pub fn handle_item_switch(&mut self, dlc_available: bool) {
        let Some(new_idx) = self.tab.lists[ITEMS_IDX].selected() else { return };
        let new_item = items_array(dlc_available)[new_idx];
        self.item = new_item;

        if let Some(new_quantity) = new_item.clamp_quantity(self.quantity) {
            self.quantity = new_quantity;
        }

        if let Some(new_upgrade) = new_item.clamp_upgrade(self.upgrade) {
            self.upgrade = new_upgrade;
        }

        if !self.aow.supports_item(new_item) {
            self.aow = aow_array()[0];
        }
        if !self.aow.supports_affinity(self.affinity.flag) {
            self.affinity = AFFINITIES[0];
        }
    }

    fn can_aow(&self) -> bool {
        self.item.weapon_type.is_some() && (self.item.gem_mount_type != Some(0))
    }

    fn can_upgrade(&self) -> bool {
        matches!(self.item.category, Categories::Weapons | Categories::SpiritAshes)
    }

    fn can_quantity(&self) -> bool {
        self.item.stack_size > 1
    }
}

impl OptionsItems {
    fn set_input(&self, app: &mut App) {
        match self {
            Self::Quantity => {
                if app.item.can_quantity() {
                    app.open_input(InputField::Quantity)
                }
            },
            Self::Upgrade => {
                if app.item.can_upgrade() {
                    app.open_input(InputField::Upgrade)
                }
            },
            Self::AshOfWar => {
                if app.item.can_aow() {
                    app.set_fuzzy_finder(Box::new(AowSearch::new(app)))
                }
            },
            Self::Affinity => {
                if app.item.can_aow() {
                    app.set_fuzzy_finder(Box::new(AffinitySearch::new(app)))
                }
            },
        }
    }
    fn to_list_item(&self, app: &App) -> ListItem<'static> {
        match self {
            Self::Quantity => {
                ListItem::new(format!("Quantity: {}", app.item.quantity))
                    .style(options_style(app.item.can_quantity()))
            }
            Self::Upgrade => {
                ListItem::new(format!("Upgrade: {}", app.item.upgrade))
                    .style(options_style(app.item.can_upgrade()))
            }
            Self::AshOfWar => {
                ListItem::new(format!("Ash of War: {}", app.item.aow.name))
                    .style(options_style(app.item.can_aow()))
            }
            Self::Affinity => {
                ListItem::new(format!("Affinity: {}", app.item.affinity.name))
                    .style(options_style(app.item.can_aow()))
            }
        }
    }
    const ARRAY: &[OptionsItems] = &[
        Self::Quantity,
        Self::Upgrade,
        Self::AshOfWar,
        Self::Affinity,
    ];
    fn list(app: &App) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(app)).collect();
        list(items, None, &app.item.tab, OPTIONS_IDX)
    }
}
fn options_style(show: bool) -> Style {
    if show {
        Style::default()
    } else {
        Style::new()
            .add_modifier(Modifier::CROSSED_OUT)
    }
}

pub fn draw(frame: &mut Frame, app: &mut App, layout: Rect) {
    let [item_area, right_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(60),
            Constraint::Fill(1)
        ])
        .areas(layout);

    let items_block = block(Some("Items"), Some(app.item.tab.block_style(ITEMS_IDX)));
    frame.render_widget(&items_block, item_area);
    let inner = items_block.inner(item_area);

    let [item_name, item_category] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(40),
            Constraint::Max(25)])
        .areas(inner);

    let [options, mass_spawn] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(6),
            Constraint::Fill(1)
        ])
        .areas(right_area);

    let (item_names, item_labels) = items_list(app);
    frame.render_stateful_widget(
        item_names,
        item_name,
        &mut app.item.tab.lists[ITEMS_IDX]
    );
    frame.render_stateful_widget(
        item_labels,
        item_category,
        &mut app.item.tab.lists[ITEMS_IDX]
    );
    frame.render_stateful_widget(
        OptionsItems::list(app),
        options,
        &mut app.item.tab.lists[OPTIONS_IDX]
    );
    frame.render_stateful_widget(
        mass_spawn_list(app),
        mass_spawn,
        &mut app.item.tab.lists[MASS_SPAWN_IDX]
    );
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let dlc_available = app.game_state.dlc;
    app.item.handle_item_switch(dlc_available);
    app.item.tab.list_sizes[ITEMS_IDX] = items_array(dlc_available).len();

    let tab = &mut app.item.tab;
    tab.handle_keys(key);

    match key.code {
        KeyCode::Enter => {
            handle_select(app)
        }
        KeyCode::Char('f') => {
            app.set_fuzzy_finder(Box::new(ItemsSearch::new(app)))
        }
        KeyCode::Char('s') => {
            if tab.current_list == OPTIONS_IDX &&
            let Some(selected_idx) = tab.lists[OPTIONS_IDX].selected() {
                OptionsItems::ARRAY[selected_idx].set_input(app);
            }
        }
        _ => ()
    }
    app.item.handle_item_switch(dlc_available);
}

fn handle_select(app: &App) {
    let tab = &app.item.tab;
    let current_list = tab.current_list;

    if current_list == MASS_SPAWN_IDX &&
    let Some(selected_idx) = tab.lists[current_list].selected() {
        let tx = app.sender.clone();
        thread::spawn(move || {
            item::mass_spawn(Categories::ARRAY[selected_idx]).send_error(tx);
        });
    }

    if current_list == ITEMS_IDX || current_list == OPTIONS_IDX {
        app.item.item.spawn(
            app.item.quantity,
            app.item.upgrade,
            app.item.aow,
            app.item.affinity,
        ).send_error(app.sender.clone());
    }
}

fn items_list(app: &App) -> (List<'static>, List<'static>) {
    let items: (Vec<ListItem>, Vec<ListItem>) = items_array(app.game_state.dlc).iter()
        .map(|item| (
                ListItem::from(item.name),
                ListItem::from(Line::raw(format!("{}", item.category)).fg(theme().muted))
        ))
        .collect();
    (
        blockless_list(items.0, &app.item.tab, ITEMS_IDX),
        label_list(items.1, &app.item.tab, ITEMS_IDX)
    )
}

fn mass_spawn_list(app: &mut App) -> List<'static> {
    let items: Vec<ListItem> = Categories::ARRAY.iter().map(|item| ListItem::from(Line::raw(item.to_string()))).collect();
    list(items, Some("Mass Spawn"), &app.item.tab, MASS_SPAWN_IDX)
}

pub struct ItemsSearch {
    pub entries: Vec<Utf32String>,
}

impl ItemsSearch {
    pub fn new(app: &App) -> Self {
        let entries: Vec<Utf32String> = items_array(app.game_state.dlc).iter()
            .map(|item| Utf32String::from(format!("{}|{}", item.name, item.category)))
            .collect();
        Self { entries }
    }
}

impl Picker for ItemsSearch {
    fn items(&self) -> Vec<Utf32String> {
        self.entries.to_vec()
    }
    fn jump(&self, idx: usize, app: &mut App) {
        *app.item.tab.lists[ITEMS_IDX].selected_mut() = Some(idx);
        let dlc_available = app.game_state.dlc;
        app.item.handle_item_switch(dlc_available);
    }
}

pub struct AowSearch {
    pub entries: Vec<Utf32String>,
}

impl AowSearch {
    pub fn new(app: &App) -> Self {
        let entries: Vec<Utf32String> = aow_array().iter()
            .filter(|aow| aow.supports_item(app.item.item))
            .map(|aow| Utf32String::from(aow.name))
            .collect();
        Self { entries }
    }
}

impl Picker for AowSearch {
    fn items(&self) -> Vec<Utf32String> {
        self.entries.to_vec()
    }
    fn jump(&self, idx: usize, app: &mut App) {
        let entries: Vec<Aow> = aow_array().iter()
            .filter(|aow| aow.supports_item(app.item.item))
            .cloned().collect();
        app.item.aow = entries[idx];
    }
}

pub struct AffinitySearch {
    pub entries: Vec<Utf32String>,
}

impl AffinitySearch {
    pub fn new(app: &App) -> Self {
        let entries: Vec<Utf32String> = AFFINITIES.iter()
            .filter(|affinity| app.item.aow.supports_affinity(affinity.flag))
            .map(|affinity| Utf32String::from(affinity.name))
            .collect();
        Self { entries }
    }
}

impl Picker for AffinitySearch {
    fn items(&self) -> Vec<Utf32String> {
        self.entries.to_vec()
    }
    fn jump(&self, idx: usize, app: &mut App) {
        let entries: Vec<resources::aow::Affinity> = AFFINITIES.iter()
            .filter(|affinity| app.item.aow.supports_affinity(affinity.flag))
            .cloned().collect();
        app.item.affinity = entries[idx];
    }
}
