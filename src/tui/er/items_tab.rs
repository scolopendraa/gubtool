use crate::{
    er::{
        item,
        resources::{
            self,
            aow::{AFFINITIES, Affinity, Aow, aow_array},
            items::{Categories, Item, items_array},
        },
    },
    send_input_event,
    tui::{
        app::App,
        common::{
            block, blockless_list, label_list, stateful_list::StatefulList, tab_state::TabState,
            tabs_list,
        },
        er::ErInfo,
        event::{Event, ResultExt, send_event},
        theme::theme,
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use nucleo_matcher::Utf32String;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::Line,
    widgets::{List, ListItem},
};
use std::thread;

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
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[ITEMS_IDX] = StatefulList::new(0);
        list_states[OPTIONS_IDX] = StatefulList::new(OptionsItems::ARRAY.len());
        list_states[MASS_SPAWN_IDX] = StatefulList::new(Categories::ARRAY.len());
        ItemTab {
            tab: TabState::new(list_states),
            item: items_array(false)[0],
            quantity: 1,
            upgrade: 0,
            aow: aow_array()[0],
            affinity: AFFINITIES[0],
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect, er: &ErInfo) {
        let [item_area, right_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(60),
                Constraint::Fill(1)
            ])
            .areas(layout);

        let items_block = block(Some("Items"), Some(self.tab.block_style(ITEMS_IDX)));
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

        let (item_names, item_labels) = self.items_list(er.dlc);
        frame.render_stateful_widget(
            item_names,
            item_name,
            &mut self.tab.get_list_state(ITEMS_IDX),
        );
        frame.render_stateful_widget(
            item_labels,
            item_category,
            &mut self.tab.get_list_state(ITEMS_IDX),
        );
        frame.render_stateful_widget(
            OptionsItems::list(&self),
            options,
            &mut self.tab.get_list_state(OPTIONS_IDX),
        );
        frame.render_stateful_widget(
            self.mass_spawn_list(),
            mass_spawn,
            &mut self.tab.get_list_state(MASS_SPAWN_IDX),
        );
    }

    pub fn handle_keys(&mut self, key: KeyEvent, er: &ErInfo) {
        self.handle_item_switch(er.dlc);
        if self.tab.current_list == ITEMS_IDX {
            self.tab.set_length(ITEMS_IDX, items_array(er.dlc).len());
        }

        self.tab.handle_keys(key);

        match key.code {
            KeyCode::Enter => {
                self.handle_select()
            }
            KeyCode::Char('f') => {
                let list = items_array(er.dlc).iter()
                    .map(|item| Utf32String::from(format!("{}|{}", item.name, item.category)))
                    .collect();
                let function = |app: &mut App| {
                    app.elden_ring.items.tab.set_list_selected(
                        ITEMS_IDX,
                        app.fuzzy_finder.selected_idx().unwrap(),
                    );
                    app.elden_ring.items.handle_item_switch(app.elden_ring.game_state.dlc)
                };
                send_event(Event::Search((list, function)))
            }
            KeyCode::Char('s') => {
                if self.tab.current_list == OPTIONS_IDX &&
                let Some(selected_idx) = self.tab.get_list_selected(OPTIONS_IDX) {
                    OptionsItems::ARRAY[selected_idx].set_input(self);
                }
            }
            _ => ()
        }
        self.handle_item_switch(er.dlc);
    }

    fn handle_select(&self) {
        if self.tab.current_list == MASS_SPAWN_IDX &&
        let Some(selected) = self.tab.get_list_selected(MASS_SPAWN_IDX) {
            thread::spawn(move || {
                item::mass_spawn(Categories::ARRAY[selected]).send_error();
            });
        }

        if self.tab.current_list == ITEMS_IDX || self.tab.current_list == OPTIONS_IDX {
            self.item.spawn(
                self.quantity,
                self.upgrade,
                self.aow,
                self.affinity,
            ).send_error();
        }
    }

    fn items_list(&self, dlc: bool) -> (List<'static>, List<'static>) {
        let items: (Vec<ListItem>, Vec<ListItem>) = items_array(dlc).iter()
            .map(|item| (
                    ListItem::from(item.name),
                    ListItem::from(Line::raw(format!("{}", item.category)).fg(theme().muted))
            ))
            .collect();
        (
            blockless_list(items.0, &self.tab, ITEMS_IDX),
            label_list(items.1, &self.tab, ITEMS_IDX)
        )
    }

    fn mass_spawn_list(&self) -> List<'static> {
        let items: Vec<ListItem> = Categories::ARRAY.iter().map(|item| ListItem::from(Line::raw(item.to_string()))).collect();
        tabs_list(items, Some("Mass Spawn"), &self.tab, MASS_SPAWN_IDX)
    }

    pub fn handle_item_switch(&mut self, dlc_available: bool) {
        let Some(new_idx) = self.tab.get_list_selected(ITEMS_IDX) else { return };
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
    fn set_input(&self, item_tab: &mut ItemTab) {
        match self {
            Self::Quantity => {
                if item_tab.can_quantity() {
                    send_input_event!(text, app, {
                        if let Ok(v) = text.parse() {
                            app.elden_ring.items.quantity = v;
                            app.elden_ring.items.handle_item_switch(
                                app.elden_ring.er_info.dlc
                            )
                        }
                    })
                }
            },
            Self::Upgrade => {
                if item_tab.can_upgrade() {
                    send_input_event!(text, app, {
                        if let Ok(v) = text.parse() {
                            app.elden_ring.items.upgrade = v;
                            app.elden_ring.items.handle_item_switch(
                                app.elden_ring.er_info.dlc
                            )
                        }
                    })
                }
            },
            Self::AshOfWar => {
                if item_tab.can_aow() {
                    let list = aow_array().iter()
                        .filter(|aow| aow.supports_item(item_tab.item))
                        .map(|aow| Utf32String::from(aow.name))
                        .collect();
                    let function = |app: &mut App| {
                        let entries: Vec<Aow> = aow_array().iter()
                            .filter(|aow| aow.supports_item(app.elden_ring.items.item))
                            .cloned().collect();
                        app.elden_ring.items.aow = entries[app.fuzzy_finder.selected_idx().unwrap()];
                    };
                    send_event(Event::Search((list, function)))
                }
            },
            Self::Affinity => {
                if item_tab.can_aow() {
                    let list = AFFINITIES.iter()
                        .filter(|affinity| item_tab.aow.supports_affinity(affinity.flag))
                        .map(|affinity| Utf32String::from(affinity.name))
                        .collect();
                    let function = |app: &mut App| {
                        let entries: Vec<resources::aow::Affinity> = AFFINITIES.iter()
                            .filter(|affinity| app.elden_ring.items.aow.supports_affinity(affinity.flag))
                            .cloned().collect();
                        app.elden_ring.items.affinity = entries[app.fuzzy_finder.selected_idx().unwrap()];
                    };
                    send_event(Event::Search((list, function)))
                }
            },
        }
    }
    fn to_list_item(&self, item_tab: &ItemTab) -> ListItem<'static> {
        match self {
            Self::Quantity => {
                ListItem::new(format!("Quantity: {}", item_tab.quantity))
                    .style(options_style(item_tab.can_quantity()))
            }
            Self::Upgrade => {
                ListItem::new(format!("Upgrade: {}", item_tab.upgrade))
                    .style(options_style(item_tab.can_upgrade()))
            }
            Self::AshOfWar => {
                ListItem::new(format!("Ash of War: {}", item_tab.aow.name))
                    .style(options_style(item_tab.can_aow()))
            }
            Self::Affinity => {
                ListItem::new(format!("Affinity: {}", item_tab.affinity.name))
                    .style(options_style(item_tab.can_aow()))
            }
        }
    }
    const ARRAY: &[OptionsItems] = &[
        Self::Quantity,
        Self::Upgrade,
        Self::AshOfWar,
        Self::Affinity,
    ];
    fn list(item_tab: &ItemTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(item_tab)).collect();
        tabs_list(items, None, &item_tab.tab, OPTIONS_IDX)
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