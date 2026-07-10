use crate::{
    input::{request_input, request_search},
    theme::theme,
};
use eldenring::resources::{
    aow::{AFFINITIES, Affinity, Aow, aow_array},
    items::{Categories, Item},
};
use nucleo_matcher::Utf32String;
use ratatui::style::Modifier;
use ratatui::{
    style::Style,
    text::Line,
    widgets::{ListItem, List},
};

/// Shared options for configuring items (used by both items tab and presets tab).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemOption {
    Quantity,
    Upgrade,
    AshOfWar,
    Affinity,
}

impl ItemOption {
    pub const ARRAY: &[ItemOption] = &[
        Self::Quantity,
        Self::Upgrade,
        Self::AshOfWar,
        Self::Affinity,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Quantity => "Quantity",
            Self::Upgrade => "Upgrade",
            Self::AshOfWar => "Ash of War",
            Self::Affinity => "Affinity",
        }
    }

    /// Check if this option is relevant for the given item/options.
    pub fn is_relevant(&self, item: &Item, aow: &Aow, affinity: &Affinity) -> bool {
        match self {
            Self::Quantity => item.stack_size > 1,
            Self::Upgrade => matches!(item.category, Categories::Weapons | Categories::SpiritAshes),
            Self::AshOfWar => {
                matches!(item.category, Categories::Weapons)
                    && item.weapon_type.is_some()
                    && item.gem_mount_type != Some(0)
            }
            Self::Affinity => {
                matches!(item.category, Categories::Weapons)
                    && item.weapon_type.is_some()
                    && item.gem_mount_type != Some(0)
                    && aow.id >= 0
                    && aow.supports_affinity(affinity.flag)
            }
        }
    }

    /// Display the current value for this option.
    pub fn display_value(
        &self,
        item: &Item,
        qty: u64,
        upgrade: u64,
        aow: &Aow,
        affinity: &Affinity,
    ) -> String {
        match self {
            Self::Quantity => format!("{} / {}", qty, item.stack_size),
            Self::Upgrade => format!("{}", upgrade),
            Self::AshOfWar => aow.name.to_string(),
            Self::Affinity => affinity.name.to_string(),
        }
    }

    /// Render this option as a list item, with strikethrough for irrelevant options.
    pub fn to_list_item(
        &self,
        item: &Item,
        qty: u64,
        upgrade: u64,
        aow: &Aow,
        affinity: &Affinity,
        selected_idx: Option<usize>,
        self_idx: usize,
    ) -> ListItem<'static> {
        let relevant = self.is_relevant(item, aow, affinity);
        let value = self.display_value(item, qty, upgrade, aow, affinity);
        let text = format!("{}: {}", self.label(), value);

        let style = if relevant {
            Style::default()
        } else {
            Style::default().add_modifier(Modifier::CROSSED_OUT).fg(theme().muted)
        };

        let mut list_item = ListItem::from(Line::raw(text)).style(style);
        if selected_idx == Some(self_idx) {
            list_item = list_item.style(Style::from(theme().accent).bold());
        }
        list_item
    }

    /// Build a highlighted options list for a given item and options.
    pub fn options_list(
        item: &Item,
        qty: u64,
        upgrade: u64,
        aow: &Aow,
        affinity: &Affinity,
        selected_idx: Option<usize>,
    ) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY
            .iter()
            .enumerate()
            .map(|(i, opt)| opt.to_list_item(item, qty, upgrade, aow, affinity, selected_idx, i))
            .collect();

        List::new(items)
            .highlight_style(Style::from(theme().accent).bold())
            .highlight_symbol(crate::theme::HIGHLIGHT_SYMBOL)
    }
}

/// Compute the final item ID by applying affinity and upgrade offsets.
/// This mirrors the logic in `Item::spawn`.
pub fn compute_final_item_id(item: &Item, aow: &Aow, affinity: &Affinity, upgrade: u64) -> i64 {
    let affinity_offset = if aow.supports_affinity(affinity.flag) {
        affinity.id_offset
    } else {
        0
    };
    let upgrade_offset = match item.category {
        Categories::Weapons | Categories::SpiritAshes => upgrade as i64,
        _ => 0,
    };
    (item.id as i64) + affinity_offset + upgrade_offset
}

/// Compute the AOW ID for an item based on current options.
pub fn compute_aow_id(aow: &Aow) -> i64 {
    if aow.id >= 0 {
        aow.id
    } else {
        -1
    }
}

/// Clamp an item value (quantity or upgrade) to the item's valid range.
///
/// This delegates to `Item::clamp_quantity` and `Item::clamp_upgrade`,
/// matching the canonical behavior used by the items tab when spawning items.
pub fn clamp_item_value(item: &Item, value: u64, is_quantity: bool) -> u64 {
    if is_quantity {
        // Clamp quantity to item's stack size
        item.clamp_quantity(value as i64)
            .unwrap_or(value as i64) as u64
    } else {
        // Clamp upgrade via Item::clamp_upgrade (same as items tab)
        item.clamp_upgrade(value as i64)
            .unwrap_or(value as i64) as u64
    }
}

/// Builder for constructing `item_presets::ItemEntry`.
pub struct ItemEntryBuilder {
    item_id: i64,
    quantity: i64,
    aow_id: i64,
    affinity_offset: i64,
    upgrade: i64,
}

impl ItemEntryBuilder {
    pub fn new() -> Self {
        Self {
            item_id: 0,
            quantity: 1,
            aow_id: -1,
            affinity_offset: 0,
            upgrade: 0,
        }
    }

    pub fn with_options(mut self, item: &Item, aow: &Aow, affinity: &Affinity, upgrade: u64) -> Self {
        self.item_id = item.id as i64;
        self.aow_id = compute_aow_id(aow);
        self.affinity_offset = if aow.supports_affinity(affinity.flag) {
            affinity.id_offset
        } else {
            0
        };
        self.upgrade = upgrade as i64;
        self
    }

    pub fn with_quantity(mut self, qty: u64) -> Self {
        self.quantity = qty as i64;
        self
    }

    pub fn build(self) -> eldenring::item_presets::ItemEntry {
        eldenring::item_presets::ItemEntry {
            item_id: self.item_id,
            quantity: self.quantity,
            aow_id: self.aow_id,
            affinity_offset: self.affinity_offset,
            upgrade: self.upgrade,
        }
    }
}

/// Closure type for updating item spawn state after an option is executed.
///
/// The closure receives the updated (quantity, upgrade, aow, affinity) values
/// and is responsible for applying them to the appropriate tab state.
pub type SpawnUpdate = dyn FnOnce(u64, u64, Aow, Affinity);

/// Execute a spawn option, updating state via the provided closure.
///
/// This centralizes the option execution logic that was previously
/// duplicated in `items_tab.rs` and `presets_tab.rs`.
///
/// Each variant handles:
/// - Relevance checking (skips irrelevant options silently)
/// - Input collection (quantity/upgrade via `request_input`, AOW/affinity via `request_search`)
/// - Value clamping (via `clamp_item_value`)
/// - Auto-reset affinity when AOW changes and doesn't support current affinity
///
/// The `update` closure is called with the new (quantity, upgrade, aow, affinity)
/// values after the option completes. The closure is responsible for applying
/// these values to the tab state using `mutate_app!`.
pub async fn execute_spawn_option(
    option: ItemOption,
    item: Item,
    current_qty: u64,
    current_upgrade: u64,
    current_aow: Aow,
    current_affinity: Affinity,
    update: impl FnOnce(u64, u64, Aow, Affinity),
) {
    match option {
        ItemOption::Quantity => {
            if item.stack_size > 1 {
                if let Some(val) = request_input::<u64>(None).await {
                    let clamped = clamp_item_value(&item, val, true);
                    update(clamped, current_upgrade, current_aow, current_affinity);
                }
            }
        }
        ItemOption::Upgrade => {
            if matches!(item.category, Categories::Weapons | Categories::SpiritAshes) {
                if let Some(val) = request_input::<u64>(None).await {
                    let clamped = clamp_item_value(&item, val, false);
                    update(current_qty, clamped, current_aow, current_affinity);
                }
            }
        }
        ItemOption::AshOfWar => {
            if item.weapon_type.is_some() && item.gem_mount_type != Some(0) {
                let available: Vec<Aow> = aow_array()
                    .iter()
                    .filter(|aow| aow.supports_item(item))
                    .cloned()
                    .collect();
                if !available.is_empty() {
                    let entries: Vec<Utf32String> = available
                        .iter()
                        .map(|aow| Utf32String::from(aow.name))
                        .collect();
                    if let Some(selected) = request_search(entries).await {
                        let new_aow = available[selected];
                        let new_affinity = if new_aow.supports_affinity(current_affinity.flag) {
                            current_affinity
                        } else {
                            AFFINITIES[0]
                        };
                        update(current_qty, current_upgrade, new_aow, new_affinity);
                    }
                }
            }
        }
        ItemOption::Affinity => {
            if item.weapon_type.is_some()
                && item.gem_mount_type != Some(0)
                && current_aow.id >= 0
            {
                let available: Vec<Affinity> = AFFINITIES
                    .iter()
                    .filter(|aff| current_aow.supports_affinity(aff.flag))
                    .cloned()
                    .collect();
                if !available.is_empty() {
                    let entries: Vec<Utf32String> = available
                        .iter()
                        .map(|aff| Utf32String::from(aff.name))
                        .collect();
                    if let Some(selected) = request_search(entries).await {
                        update(current_qty, current_upgrade, current_aow, available[selected]);
                    }
                }
            }
        }
    }
}
