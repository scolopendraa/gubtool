use crate::{
    app::App,
    common::{
        block, blockless_list, label_list, stateful_list::StatefulList, tab_state::TabState, ItemOption,
    },
    eldenring_screen::GameState,
    event::AnyhowExt,
    input::request_search,
    mutate_app, spawn_task,
    theme::theme,
};
use crossterm::event::{KeyCode, KeyEvent};
use eldenring::{
    item,
    resources::{
        aow::{AFFINITIES, Affinity, Aow, aow_array},
        items::{Categories, Item, items_array},
    },
};
use nucleo_matcher::Utf32String;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{List, ListItem},
};
use std::thread;

const ITEMS_IDX: usize = 0;
const OPTIONS_IDX: usize = 1;
const MASS_SPAWN_IDX: usize = 2;

pub struct ItemTab {
    tab: TabState,
    item: Item,
    quantity: u64,
    upgrade: u64,
    aow: Aow,
    affinity: Affinity,
}

impl ItemTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[ITEMS_IDX] = StatefulList::new(0);
        list_states[OPTIONS_IDX] = StatefulList::new(ItemOption::ARRAY.len());
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

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
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
                Constraint::Length(5),
                Constraint::Fill(1)
            ])
            .areas(right_area);

        let (item_names, item_labels) = self.items_list();
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
            ItemOption::options_list(
                &self.item, self.quantity, self.upgrade,
                &self.aow, &self.affinity,
                None,
            ),
            options,
            &mut self.tab.get_list_state(OPTIONS_IDX),
        );
        frame.render_stateful_widget(
            self.mass_spawn_list(),
            mass_spawn,
            &mut self.tab.get_list_state(MASS_SPAWN_IDX),
        );
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.handle_item_switch();

        if self.tab.current_list == ITEMS_IDX {
            self.tab.set_length(ITEMS_IDX, items_array(GameState::dlc()).len());
        }

        self.tab.handle_keys(key);

        match key.code {
            KeyCode::Enter => {
                self.handle_enter()
            }
            KeyCode::Char('f') => {
                spawn_task! {
                    let entries = items_array(GameState::dlc()).iter()
                        .map(|item| Utf32String::from(format!("{}|{}", item.name, item.category)))
                        .collect();
                    if let Some(new_idx) = request_search(entries).await {
                        mutate_app!(|app: &mut App| {
                            let items_tab = &mut app.elden_ring.items;
                            items_tab.tab.set_list_selected(ITEMS_IDX, new_idx);
                            items_tab.handle_item_switch();
                        });
                    }
                }
            }
            _ => ()
        }
        self.handle_item_switch();
    }

    fn handle_enter(&mut self) {
        let Some(selected) = self.tab.current_list_selected() else { return };

        match self.tab.current_list {
            ITEMS_IDX => {
                self.item.spawn(
                    self.quantity as i64,
                    self.upgrade as i64,
                    self.aow,
                    self.affinity,
                ).send_error();
            }
            OPTIONS_IDX => {
                self.handle_option(ItemOption::ARRAY[selected]);
            }
            MASS_SPAWN_IDX => {
                thread::spawn(move || {
                    item::mass_spawn(Categories::ARRAY[selected]).send_error();
                });
            }
            _ => (),
        }
    }

    fn items_list(&self) -> (List<'static>, List<'static>) {
        let items: (Vec<ListItem>, Vec<ListItem>) = items_array(GameState::dlc()).iter()
            .map(|item| (
                    ListItem::from(item.name),
                    ListItem::from(Line::raw(format!("{}", item.category)).style(Style::from(theme().muted)))
            ))
            .collect();
        (
            blockless_list(items.0, &self.tab, ITEMS_IDX),
            label_list(items.1, &self.tab, ITEMS_IDX)
        )
    }

    fn mass_spawn_list(&self) -> List<'static> {
        let items: Vec<ListItem> = Categories::ARRAY.iter().map(|item| ListItem::from(Line::raw(item.to_string()))).collect();
        crate::common::tabs_list(items, Some("Mass Spawn"), &self.tab, MASS_SPAWN_IDX)
    }

    fn handle_option(&mut self, option: ItemOption) {
        let item = self.item;
        let qty = self.quantity;
        let upgrade = self.upgrade;
        let aow = self.aow;
        let affinity = self.affinity;
        spawn_task! {
            crate::common::execute_spawn_option(
                option,
                item,
                qty,
                upgrade,
                aow,
                affinity,
                |nq, nu, na, nf| {
                    mutate_app!(|app: &mut App| {
                        let tab = &mut app.elden_ring.items;
                        tab.quantity = nq;
                        tab.upgrade = nu;
                        tab.aow = na;
                        tab.affinity = nf;
                    });
                },
            ).await;
        }
    }

    pub fn handle_item_switch(&mut self) {
        let Some(new_idx) = self.tab.get_list_selected(ITEMS_IDX) else { return };
        let new_item = items_array(GameState::dlc())[new_idx];
        self.item = new_item;

        if let Some(new_quantity) = new_item.clamp_quantity(self.quantity as i64) {
            self.quantity = new_quantity as u64;
        }

        if let Some(new_upgrade) = new_item.clamp_upgrade(self.upgrade as i64) {
            self.upgrade = new_upgrade as u64;
        }

        if !self.aow.supports_item(new_item) {
            self.aow = aow_array()[0];
        }
        if !self.aow.supports_affinity(self.affinity.flag) {
            self.affinity = AFFINITIES[0];
        }
    }
}

