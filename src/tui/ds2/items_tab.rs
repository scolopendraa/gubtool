use crate::{
    ds2::{
        item::mass_spawn,
        resources::items::{Categories, Item, infusions::Infusions, items_array},
    },
    send_input_event,
    tui::{
        app::App,
        common::{
            block, blockless_list, label_list, stateful_list::StatefulList, tab_state::TabState,
            tabs_list,
        },
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
    Infusion,
}

pub struct ItemTab {
    tab: TabState,
    item: Item,
    pub quantity: i32,
    pub upgrade: i32,
    pub infusion: Infusions,
}

const ITEMS_IDX: usize = 0;
const OPTIONS_IDX: usize = 1;
const MASS_SPAWN_IDX: usize = 2;

impl ItemTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[ITEMS_IDX] = StatefulList::new(1);
        list_states[OPTIONS_IDX] = StatefulList::new(3);
        list_states[MASS_SPAWN_IDX] = StatefulList::new(Categories::ARRAY.len());
        ItemTab {
            tab: TabState::new(list_states),
            item: items_array()[0],
            quantity: 1,
            upgrade: 0,
            infusion: Infusions::Normal,
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

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.handle_item_switch();
        if self.tab.current_list == ITEMS_IDX {
            self.tab.set_length(ITEMS_IDX, items_array().len());
        }
        self.tab.handle_keys(key);

        match key.code {
            KeyCode::Enter => {
                self.handle_select()
            }
            KeyCode::Char('f') => {
                let list = items_array().iter()
                    .map(|item| Utf32String::from(format!("{}|{}", item.name, item.category)))
                    .collect();
                let function = |app: &mut App| {
                    app.dark_souls_2.items.tab.set_list_selected(
                        ITEMS_IDX,
                        app.fuzzy_finder.selected_idx().unwrap(),
                    );
                    app.dark_souls_2.items.handle_item_switch()
                };
                send_event(Event::Search((list, function)))
            }
            KeyCode::Char('s') => {
                if self.tab.current_list == OPTIONS_IDX &&
                let Some(selected) = self.tab.get_list_selected(OPTIONS_IDX) {
                    OptionsItems::ARRAY[selected].set_input(self);
                }
            }
            _ => ()
        }
        self.handle_item_switch();
    }

    fn handle_select(&self) {
        if self.tab.current_list == MASS_SPAWN_IDX &&
        let Some(selected) = self.tab.get_list_selected(MASS_SPAWN_IDX) {
            thread::spawn(move || {
                mass_spawn(Categories::ARRAY[selected]).send_error();
            });
        }

        if self.tab.current_list == ITEMS_IDX || self.tab.current_list == OPTIONS_IDX {
            self.item.spawn(
                self.quantity,
                self.upgrade,
                self.infusion as u8 as i32,
            ).send_error();
        }
    }

    fn items_list(&self) -> (List<'static>, List<'static>) {
        let items: (Vec<ListItem>, Vec<ListItem>) = items_array().iter()
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

    pub fn handle_item_switch(&mut self) {
        let Some(new_idx) = self.tab.get_list_selected(ITEMS_IDX) else { return };
        let new_item = items_array()[new_idx];
        self.item = new_item;

        if self.upgrade > self.item.max_upgrade.unwrap_or_default() {
            self.upgrade = self.item.max_upgrade.unwrap_or_default()
        }

        if self.quantity > self.item.stack_size {
            self.quantity = self.item.stack_size
        }

        if !self.item.available_infusions().contains(&self.infusion) {
            self.infusion = Infusions::Normal
        }
    }
    fn can_quantity(&self) -> bool {
        self.item.stack_size > 1
    }
    fn can_upgrade(&self) -> bool {
        self.item.max_upgrade.is_some()
    }
    fn can_infuse(&self) -> bool {
        self.item.infuse_id.is_some()
    }
}

impl OptionsItems {
    fn set_input(&self, item_tab: &mut ItemTab) {
        match self {
            Self::Quantity => {
                if item_tab.can_quantity() {
                    send_input_event!(text, app, {
                        if let Ok(v) = text.parse() {
                            app.dark_souls_2.items.quantity = v;
                            app.dark_souls_2.items.handle_item_switch()
                        }
                    })
                }
            },
            Self::Upgrade => {
                if item_tab.can_upgrade() {
                    send_input_event!(text, app, {
                        if let Ok(v) = text.parse() {
                            app.dark_souls_2.items.upgrade = v;
                            app.dark_souls_2.items.handle_item_switch()
                        }
                    })
                }
            },
            Self::Infusion => {
                if item_tab.can_infuse() {
                    let list = item_tab.item.available_infusions().iter()
                        .map(|infusion| Utf32String::from(format!("{}", infusion)))
                        .collect();
                    let function = |app: &mut App| {
                        let entries = app.dark_souls_2.items.item.available_infusions();
                        app.dark_souls_2.items.infusion = entries[app.fuzzy_finder.selected_idx().unwrap()];
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
            Self::Infusion => {
                ListItem::new(format!("Affinity: {}", item_tab.infusion))
                    .style(options_style(item_tab.can_infuse()))
            }
        }
    }
    const ARRAY: &[OptionsItems] = &[
        Self::Quantity,
        Self::Upgrade,
        Self::Infusion,
    ];
    fn list(item_tab: &ItemTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(item_tab)).collect();
        tabs_list(items, None, &item_tab.tab, OPTIONS_IDX)
    }
}

fn options_style(show: bool) -> Style {
    if show { Style::default() } else { Style::new().add_modifier(Modifier::CROSSED_OUT) }
}