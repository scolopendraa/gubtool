use crate::{
    app::App,
    common::{
        block, blockless_list, controls::draw_controls, label_list, stateful_list::StatefulList, tab_state::TabState, ItemOption, ItemEntryBuilder,
    },
    eldenring_screen::GameState,
    event::{AnyhowExt, Event, InfoType, ResultExt, send_event},
    input::{request_multi_search, request_search, request_string},
    mutate_app, spawn_task,
    theme::theme,
};
use crossterm::event::{KeyCode, KeyEvent};
use eldenring::{
    grace_presets,
    item_presets,
    resources::{
        aow::{AFFINITIES, Affinity, Aow, aow_array},
        graces::graces_array,
        items::{Categories, items_array},
    },
};
use nucleo_matcher::Utf32String;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::Line,
    widgets::{List, ListItem, ListState},
};

const GRACE_PRESETS_IDX: usize = 0;
const ITEM_PRESETS_IDX: usize = 1;

const CREATE_NEW_IDX: usize = 0;

const CONTROLS: &[(&str, &str)] = &[
    ("enter", "Apply/Create"),
    ("d", "Delete"),
    ("esc", "Back"),
];

/// Offset applied to preset indices to account for "Create New" at position 0.
const PRESET_INDEX_OFFSET: usize = 1;

/// Per-item options stored during creation mode.
#[derive(Clone, Copy)]
struct ItemOptions {
    quantity: u64,
    aow: Aow,
    affinity: Affinity,
    upgrade: u64,
}

/// State for the preset creation flow.
struct PresetCreationState {
    /// Items selected for the preset (indices into items_array)
    selected_indices: Vec<usize>,
    /// Currently being configured item index (into selected_indices)
    current_item_idx: usize,
    /// State for the items list rendering
    items_list_state: ListState,
    /// State for the options list rendering
    options_list_state: ListState,
    /// Index of the selected option in the options list (0=Quantity, 1=Upgrade, ...)
    options_list_selected: usize,
    /// Configured items ready to be added to preset (one entry per selected item)
    configured_items: Vec<item_presets::ItemEntry>,
    /// Per-item options (aow, affinity, upgrade) for restoring when switching items
    per_item_options: Vec<ItemOptions>,
    /// Current quantity being configured
    quantity: u64,
    /// Current upgrade level being configured
    upgrade: u64,
    /// Current AOW being configured
    aow: Aow,
    /// Current affinity being configured
    affinity: Affinity,
}

impl PresetCreationState {
    fn new(selected_indices: Vec<usize>) -> Self {
        let items = items_array(GameState::dlc());
        // Initialize all items with defaults immediately
        let configured_items: Vec<item_presets::ItemEntry> = selected_indices
            .iter()
            .map(|&idx| {
                let item = items.get(idx).cloned().expect("item index out of bounds");
                let upgrade = match item.category {
                    Categories::SpiritAshes => 0u64,
                    Categories::Weapons => match (item.gem_mount_type, item.upgrade_type) {
                        (Some(1) | Some(2), _) => 0,
                        (_, Some(1)) => 0,
                        _ => 0,
                    },
                    _ => 0,
                };
                ItemEntryBuilder::new()
                    .with_options(&item, &aow_array()[0], &AFFINITIES[0], upgrade)
                    .with_quantity(1)
                    .build()
            })
            .collect();
        // Initialize per-item options with defaults
        let per_item_options: Vec<ItemOptions> = selected_indices
            .iter()
            .map(|&idx| {
                let item = items.get(idx).cloned().expect("item index out of bounds");
                let upgrade = match item.category {
                    Categories::SpiritAshes => 0,
                    Categories::Weapons => match (item.gem_mount_type, item.upgrade_type) {
                        (Some(1) | Some(2), _) => 0,
                        (_, Some(1)) => 0,
                        _ => 0,
                    },
                    _ => 0,
                };
                ItemOptions {
                    quantity: 1,
                    aow: aow_array()[0],
                    affinity: AFFINITIES[0],
                    upgrade,
                }
            })
            .collect();
        Self {
            selected_indices,
            current_item_idx: 0,
            items_list_state: ListState::default().with_selected(Some(0)),
            options_list_state: ListState::default().with_selected(Some(0)),
            options_list_selected: 0,
            configured_items,
            per_item_options,
            quantity: 1,
            upgrade: 0,
            aow: aow_array()[0],
            affinity: AFFINITIES[0],
        }
    }

    fn current_item(&self) -> Option<&eldenring::resources::items::Item> {
        self.selected_indices
            .get(self.current_item_idx)
            .and_then(|&idx| items_array(GameState::dlc()).get(idx))
    }

    /// Update the configured entry for the current item with current options,
    /// and save per-item options for restoring when switching items.
    fn update_current_entry(&mut self) {
        let item = match self.current_item() {
            Some(i) => i.clone(),
            None => return,
        };

        let qty = if item.stack_size > 1 {
            self.quantity
        } else {
            1
        };

        let entry = ItemEntryBuilder::new()
            .with_options(&item, &self.aow, &self.affinity, self.upgrade)
            .with_quantity(qty)
            .build();

        if let Some(e) = self.configured_items.get_mut(self.current_item_idx) {
            *e = entry;
        }

        // Save per-item options for restoring when switching items
        if let Some(opts) = self.per_item_options.get_mut(self.current_item_idx) {
            opts.quantity = self.quantity;
            opts.aow = self.aow;
            opts.affinity = self.affinity;
            opts.upgrade = self.upgrade;
        }
    }

    /// Save current item's options and load new item's options when switching.
    fn switch_item(&mut self, new_idx: usize) {
        // Save current item's options
        if let Some(entry) = self.per_item_options.get_mut(self.current_item_idx) {
            entry.quantity = self.quantity;
            entry.aow = self.aow;
            entry.affinity = self.affinity;
            entry.upgrade = self.upgrade;
        }
        // Load new item's options
        self.current_item_idx = new_idx;
        if let Some(opts) = self.per_item_options.get(new_idx) {
            self.quantity = opts.quantity;
            self.aow = opts.aow;
            self.affinity = opts.affinity;
            self.upgrade = opts.upgrade;
        }
    }
}

pub struct PresetsTab {
    tab: TabState,
    /// State for preset creation flow (None = not in creation mode)
    creation_state: Option<PresetCreationState>,
    /// Which list is active during creation mode (0=items, 1=options)
    creation_list_active: usize,
}

impl PresetsTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 2];
        list_states[GRACE_PRESETS_IDX] = StatefulList::new(0);
        list_states[ITEM_PRESETS_IDX] = StatefulList::new(0);
        PresetsTab {
            tab: TabState::new(list_states),
            creation_state: None,
            creation_list_active: 0,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        // If in creation mode, show split-screen config UI
        if self.creation_state.is_some() {
            self.draw_creation_mode(frame, layout);
            return;
        }

        // Normal preset list view
        let [grace_area, item_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .areas(layout);

        // Grace Presets pane (left)
        let grace_block = block(Some("Grace Presets"), Some(self.tab.block_style(GRACE_PRESETS_IDX)));
        let grace_inner = grace_block.inner(grace_area);
        frame.render_widget(&grace_block, grace_area);

        let [grace_name, grace_area_inner] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(40),
                Constraint::Max(25),
            ])
            .areas(grace_inner);

        let (grace_names, grace_areas) = self.grace_presets_list();
        frame.render_stateful_widget(
            grace_names,
            grace_name,
            &mut self.tab.get_list_state(GRACE_PRESETS_IDX),
        );
        frame.render_stateful_widget(
            grace_areas,
            grace_area_inner,
            &mut self.tab.get_list_state(GRACE_PRESETS_IDX),
        );

        // Item Presets pane (right)
        let item_block = block(Some("Item Presets"), Some(self.tab.block_style(ITEM_PRESETS_IDX)));
        let item_inner = item_block.inner(item_area);
        frame.render_widget(&item_block, item_area);

        let [item_name, item_area_inner] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(40),
                Constraint::Max(25),
            ])
            .areas(item_inner);

        let (item_names, item_areas) = self.item_presets_list();
        frame.render_stateful_widget(
            item_names,
            item_name,
            &mut self.tab.get_list_state(ITEM_PRESETS_IDX),
        );
        frame.render_stateful_widget(
            item_areas,
            item_area_inner,
            &mut self.tab.get_list_state(ITEM_PRESETS_IDX),
        );

        draw_controls(frame, layout, CONTROLS);
    }

    /// Draw the split-screen configuration UI for preset creation.
    fn draw_creation_mode(&mut self, frame: &mut Frame, layout: Rect) {
        let state = self.creation_state.as_mut().unwrap();

        // Layout the two panes with room for controls at the bottom
        let [content_area, controls_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .areas(layout);

        let [items_area, options_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(55),
                Constraint::Percentage(45),
            ])
            .areas(content_area);

        let selected_count = state.selected_indices.len();

        // Left: Selected items list
        let items_title = format!("Items (editing {}/{}):", state.current_item_idx + 1, selected_count);
        let items_block = block(
            Some(&items_title),
            if self.creation_list_active == 0 {
                Some(Style::new().fg(theme().fg))
            } else {
                Some(Style::new().fg(theme().fg).add_modifier(Modifier::DIM))
            },
        );
        let items_inner = items_block.inner(items_area);
        frame.render_widget(&items_block, items_area);

        let [items_name, items_label] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(40),
                Constraint::Max(25),
            ])
            .areas(items_inner);

        let (item_names, item_labels) = Self::creation_items_list(&self.tab, state);
        state.items_list_state.select(Some(state.current_item_idx));
        frame.render_stateful_widget(
            item_names,
            items_name,
            &mut state.items_list_state,
        );
        frame.render_stateful_widget(
            item_labels,
            items_label,
            &mut state.items_list_state,
        );

        // Right: Options panel
        let options_block = block(
            Some("Options"),
            if self.creation_list_active == 1 {
                Some(Style::new().fg(theme().fg))
            } else {
                Some(Style::new().fg(theme().fg).add_modifier(Modifier::DIM))
            },
        );
        let options_inner = options_block.inner(options_area);
        frame.render_widget(&options_block, options_area);

        // Split options area: top line for item name, rest for options list
        let [item_name_area, options_list_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .areas(options_inner);

        // Show current item name
        if let Some(item) = state.current_item() {
            let item_line = Line::from(format!("{} [{}]", item.name, item.category))
                .style(theme().accent);
            frame.render_widget(List::new(vec![ListItem::from(item_line)]), item_name_area);
        }

        // Options list
        let current_item = match state.current_item() {
            Some(item) => item,
            None => {
                state.options_list_state.select(Some(state.options_list_selected));
                frame.render_stateful_widget(
                    List::new::<Vec<ListItem<'static>>>(vec![]),
                    options_list_area,
                    &mut state.options_list_state,
                );
                return;
            }
        };
        let options_list = ItemOption::options_list(
            current_item,
            state.quantity,
            state.upgrade,
            &state.aow,
            &state.affinity,
            Some(state.options_list_selected),
        );
        state.options_list_state.select(Some(state.options_list_selected));
        frame.render_stateful_widget(
            options_list,
            options_list_area,
            &mut state.options_list_state,
        );

        // Render controls at the bottom
        let creation_controls: &[(&str, &str)] = &[
            ("ctrl-l/h", "Switch pane"),
            ("j/k, up/down", "Navigate"),
            ("enter", "Save/Apply"),
            ("esc", "Cancel"),
        ];
        draw_controls(frame, controls_area, creation_controls);
    }

    fn creation_items_list(tab: &TabState, state: &PresetCreationState) -> (List<'static>, List<'static>) {
        let mut names: Vec<ListItem> = Vec::new();
        let mut labels: Vec<ListItem> = Vec::new();

        for (_i, &idx) in state.selected_indices.iter().enumerate() {
            let items = items_array(GameState::dlc());
            if let Some(item) = items.get(idx) {
                let name = ListItem::from(item.name);
                let label = ListItem::from(Line::raw(format!("{}", item.category)).fg(theme().muted));
                names.push(name);
                labels.push(label);
            }
        }

        (
            blockless_list(names, tab, 0),
            label_list(labels, tab, 0),
        )
    }

    fn is_create_new(&self, list_idx: usize) -> bool {
        list_idx == CREATE_NEW_IDX
    }

    fn is_default_preset(&self, list_idx: usize, num_defaults: usize) -> bool {
        list_idx > CREATE_NEW_IDX && (list_idx - PRESET_INDEX_OFFSET) < num_defaults
    }

    fn preset_index_from_list(&self, list_idx: usize) -> usize {
        list_idx.saturating_sub(PRESET_INDEX_OFFSET)
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        // Handle creation mode first
        if self.creation_state.is_some() {
            self.handle_creation_keys(key);
            return;
        }

        let grace_defaults = grace_presets::get_default_presets().len();
        let item_defaults = item_presets::get_default_presets().len();

        self.tab.set_length(
            GRACE_PRESETS_IDX,
            grace_defaults + grace_presets::load_presets().map_or(0, |p| p.len()) + 1,
        );
        self.tab.set_length(
            ITEM_PRESETS_IDX,
            item_defaults + item_presets::load_presets().map_or(0, |p| p.len()) + 1,
        );

        self.tab.handle_keys(key);

        match key.code {
            KeyCode::Enter => {
                self.handle_select()
            }
            KeyCode::Char('f') => {
                let entries = if self.tab.current_list == GRACE_PRESETS_IDX {
                    let mut entries: Vec<Utf32String> = grace_presets::get_all_presets()
                        .iter()
                        .map(|preset| Utf32String::from(preset.name.clone()))
                        .collect();
                    entries.push(Utf32String::from("Create New".to_string()));
                    entries
                } else {
                    let mut entries: Vec<Utf32String> = item_presets::get_all_presets()
                        .iter()
                        .map(|preset| Utf32String::from(preset.name.clone()))
                        .collect();
                    entries.push(Utf32String::from("Create New".to_string()));
                    entries
                };
                spawn_task! {
                    if let Some(new_idx) = request_search(entries).await {
                        mutate_app!(|app: &mut App| {
                            let tab = &mut app.elden_ring.presets.tab;
                            let adjusted_idx = new_idx.saturating_sub(1);
                            tab.set_list_selected(tab.current_list, adjusted_idx);
                        });
                    }
                }
            }
            KeyCode::Char('d') => {
                if self.tab.current_list == GRACE_PRESETS_IDX
                    && let Some(selected) = self.tab.get_list_selected(GRACE_PRESETS_IDX)
                {
                    if !self.is_create_new(selected) && !self.is_default_preset(selected, grace_defaults) {
                        let custom_idx = self.preset_index_from_list(selected) - grace_defaults;
                        grace_presets::delete_preset(custom_idx).send_error();
                    }
                } else if self.tab.current_list == ITEM_PRESETS_IDX
                    && let Some(selected) = self.tab.get_list_selected(ITEM_PRESETS_IDX)
                {
                    if !self.is_create_new(selected) && !self.is_default_preset(selected, item_defaults) {
                        let custom_idx = self.preset_index_from_list(selected) - item_defaults;
                        item_presets::delete_preset(custom_idx).send_error();
                    }
                }
            }
            _ => (),
        }
    }

    /// Handle keys during preset creation mode.
    fn handle_creation_keys(&mut self, key: KeyEvent) {
        let state = self.creation_state.as_mut().unwrap();
        match (key.code, key.modifiers) {
            (KeyCode::Esc, _) => {
                // Cancel creation mode
                self.creation_state = None;
            }
            (KeyCode::Enter, _) => {
                if self.creation_list_active == 0 {
                    // On items list: save the preset
                    state.update_current_entry();
                    let configured_items = state.configured_items.clone();
                    spawn_task! {
                        if let Some(name) = request_string(Some("Preset name")).await {
                            if !name.is_empty() {
                                item_presets::add_preset(&name, configured_items).send_error();
                            }
                        }
                        mutate_app!(|app: &mut App| {
                            app.elden_ring.presets.creation_state = None;
                        });
                    }
                } else {
                    // On options list: execute the selected option
                    if let Some(option) = ItemOption::ARRAY.get(state.options_list_selected) {
                        self.handle_preset_option(option.clone());
                    }
                }
            }
            (KeyCode::Char('l') | KeyCode::Char('h'), crossterm::event::KeyModifiers::CONTROL) => {
                // Switch between items list and options list (ctrl-l = right, ctrl-h = left)
                self.creation_list_active = if key.code == KeyCode::Char('l') { 1 } else { 0 };
            }
            (KeyCode::Char('j') | KeyCode::Down, _) => {
                if self.creation_list_active == 0 {
                    // Navigate items list down
                    if state.current_item_idx + 1 < state.selected_indices.len() {
                        state.switch_item(state.current_item_idx + 1);
                    }
                } else {
                    // Navigate options list down
                    if state.options_list_selected + 1 < ItemOption::ARRAY.len() {
                        state.options_list_selected += 1;
                    }
                }
            }
            (KeyCode::Char('k') | KeyCode::Up, _) => {
                if self.creation_list_active == 0 {
                    // Navigate items list up
                    if state.current_item_idx > 0 {
                        state.switch_item(state.current_item_idx - 1);
                    }
                } else {
                    // Navigate options list up
                    if state.options_list_selected > 0 {
                        state.options_list_selected -= 1;
                    }
                }
            }
            _ => (),
        }
    }

    /// Handle the selected spawn option for the current item in creation mode.
    fn handle_preset_option(&mut self, option: ItemOption) {
        let state = self.creation_state.as_ref().unwrap();
        let item = state
            .current_item()
            .cloned()
            .unwrap_or(items_array(GameState::dlc())[0]);
        let qty = state.quantity;
        let upgrade = state.upgrade;
        let aow = state.aow;
        let affinity = state.affinity;
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
                        let presets = &mut app.elden_ring.presets;
                        if let Some(ref mut st) = presets.creation_state {
                            st.quantity = nq;
                            st.upgrade = nu;
                            st.aow = na;
                            st.affinity = nf;
                            st.update_current_entry();
                        }
                    });
                },
            ).await;
        }
    }

    fn handle_select(&self) {
        let Some(selected_idx) = self.tab.get_list_selected(self.tab.current_list) else {
            return;
        };

        // "Create New" was selected
        if self.is_create_new(selected_idx) {
            if self.tab.current_list == GRACE_PRESETS_IDX {
                self.create_grace_preset();
            } else {
                self.create_item_preset();
            }
            return;
        }

        let preset_idx = self.preset_index_from_list(selected_idx);
        if self.tab.current_list == GRACE_PRESETS_IDX {
            grace_presets::apply_preset(preset_idx).send_error();
        } else if self.tab.current_list == ITEM_PRESETS_IDX {
            // Get preset name for progress messages
            let preset_name = item_presets::get_all_presets()
                .get(preset_idx)
                .map(|p| p.name.clone())
                .unwrap_or_default();
            spawn_task! {
                item_presets::apply_preset(preset_idx, Some(&mut |current: usize, total: usize| {
                    send_event(Event::Info((
                        format!("Spawning preset '{}': {}/{} items", preset_name, current, total),
                        InfoType::Success,
                    )));
                })).await.send_error()
            }
        }
    }

    fn create_grace_preset(&self) {
        spawn_task! {
            let graces = graces_array(GameState::dlc());
            let entries: Vec<Utf32String> = graces
                .iter()
                .map(|g| Utf32String::from(format!("{}|{}", g.name, g.main_area)))
                .collect();

            let selected_indices = request_multi_search(entries).await;

            if !selected_indices.is_empty() {
                let grace_entity_ids: Vec<i64> = selected_indices
                    .iter()
                    .filter_map(|&idx| graces.get(idx).map(|g| g.grace_entity_id))
                    .collect();

                if !grace_entity_ids.is_empty() {
                    if let Some(name) = request_string(Some("Preset name")).await {
                        if !name.is_empty() {
                            grace_presets::add_preset(&name, grace_entity_ids).send_error();
                        }
                    }
                }
            }
        }
    }

    fn create_item_preset(&self) {
        spawn_task! {
            let items = items_array(GameState::dlc());
            let entries: Vec<Utf32String> = items
                .iter()
                .map(|item| Utf32String::from(format!("{}|{}", item.name, item.category)))
                .collect();

            // Multi-select items
            let selected_indices = request_multi_search(entries).await;

            if !selected_indices.is_empty() {
                // Enter creation mode with selected items
                mutate_app!(|app: &mut App| {
                    let presets = &mut app.elden_ring.presets;
                    presets.creation_state = Some(PresetCreationState::new(selected_indices));
                    presets.creation_list_active = 0;
                });
            }
        }
    }

    fn grace_presets_list(&self) -> (List<'static>, List<'static>) {
        let presets = grace_presets::get_all_presets();
        let mut names: Vec<ListItem> = vec![ListItem::from("Create New")];
        let mut labels: Vec<ListItem> = vec![ListItem::from(Line::raw("")).fg(theme().muted)];
        for preset in &presets {
            names.push(ListItem::from(preset.name.clone()));
            labels.push(ListItem::from(Line::raw("Grace")).fg(theme().muted));
        }
        (
            blockless_list(names, &self.tab, GRACE_PRESETS_IDX),
            label_list(labels, &self.tab, GRACE_PRESETS_IDX),
        )
    }

    fn item_presets_list(&self) -> (List<'static>, List<'static>) {
        let presets = item_presets::get_all_presets();
        let mut names: Vec<ListItem> = vec![ListItem::from("Create New")];
        let mut labels: Vec<ListItem> = vec![ListItem::from(Line::raw("")).fg(theme().muted)];
        for preset in &presets {
            names.push(ListItem::from(preset.name.clone()));
            labels.push(ListItem::from(Line::raw("Item")).fg(theme().muted));
        }
        (
            blockless_list(names, &self.tab, ITEM_PRESETS_IDX),
            label_list(labels, &self.tab, ITEM_PRESETS_IDX),
        )
    }
}
