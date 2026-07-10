use crate::{
    item::spawn_item_raw,
    preset_storage::{LazyPresetStore, PresetEntry, PresetStore},
    resources::items::{
        Item, armor::ARMOR, arrows::ARROWS, ashes_of_war::ASHES_OF_WAR,
        bell_bearings::BELL_BEARINGS, consumables::CONSUMABLES, cookbooks::COOKBOOKS,
        crafting_materials::CRAFTING_MATERIALS, crystal_tears::CRYSTAL_TEARS,
        incantations::INCANTATIONS, key_items::KEY_ITEMS, pots_and_perfumes::POTS_AND_PERFUMES,
        prattling_pate::PRATTLING_PATE, sorceries::SORCERIES, spirit_ashes::SPIRIT_ASHES,
        talismans::TALISMANS, upgrade_materials::UPGRADE_MATERIALS, weapons::WEAPONS,
    },
};
use anyhow::Result;
use gubtool_core::sys::error::{AccessType, ProcResult};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// Delay between item spawns in presets (milliseconds).
///
/// Prevents game lag from item flooding. 50ms was chosen as a balance
/// between responsiveness and avoiding overwhelming the game's item
/// spawn system. For presets with many items, total spawn time is
/// approximately `count * 50ms`.
pub const ITEM_SPAWN_DELAY_MS: u64 = 50;

/// A single item entry in a preset.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemEntry {
    pub item_id: i64,
    pub quantity: i64,
    pub aow_id: i64,
    /// Affinity offset applied to the item ID (0 if no affinity)
    pub affinity_offset: i64,
    /// Upgrade level applied to the item ID
    pub upgrade: i64,
}

/// A preset containing a list of items to spawn.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemPreset {
    pub name: String,
    pub items: Vec<ItemEntry>,
}

impl PresetEntry for ItemPreset {
    fn name(&self) -> &str {
        &self.name
    }
}

/// Shared preset store for item presets.
static ITEM_STORE: LazyPresetStore<ItemPreset> =
    Lazy::new(|| PresetStore::new("item_presets.json"));

/// Load item presets from disk.
pub fn load_presets() -> Result<Vec<ItemPreset>> {
    ITEM_STORE.load()
}

/// Save item presets to disk atomically.
pub fn save_presets(presets: &[ItemPreset]) -> Result<()> {
    ITEM_STORE.save(presets)
}

/// Validate a preset name: must be non-empty and not contain only whitespace.
pub fn validate_preset_name(name: &str) -> Result<()> {
    PresetStore::<ItemPreset>::validate_name(name)
}

/// Add a new item preset and save it.
pub fn add_preset(name: &str, items: Vec<ItemEntry>) -> Result<()> {
    ITEM_STORE.add(ItemPreset {
        name: name.to_string(),
        items,
    })
}

/// Delete an item preset by index into the custom presets vec.
pub fn delete_preset(index: usize) -> Result<()> {
    ITEM_STORE.delete(index)
}

/// Apply an item preset by index - spawns all items in the preset.
/// Spawns items with a small delay between each to prevent game lag.
///
/// `progress` is an optional callback called after each item spawn
/// with `(current_item_index, total_items)` for progress reporting.
pub async fn apply_preset<F>(index: usize, mut progress: Option<F>) -> ProcResult
where
    F: FnMut(usize, usize) + Send,
{
    let all_presets = get_all_presets();
    let preset = all_presets.get(index).ok_or_else(|| {
        gubtool_core::sys::error::ProcessError::partial_access(
            AccessType::Read("item_presets"),
            0,
            0,
        )
    })?;

    // Validate all item IDs before spawning to prevent spawning invalid items
    // that could crash the game or cause unexpected behavior.
    let all = all_items();
    for entry in &preset.items {
        if !all.iter().any(|item| item.id as i64 == entry.item_id) {
            eprintln!("Skipping invalid item ID {} in preset '{}'", entry.item_id, preset.name);
            continue;
        }
    }

    let total = preset.items.len();

    // Spawn each item in the preset, continuing on individual failures.
    // This ensures partial application rather than bailing on the first error.
    // Small delay between spawns prevents game lag from item flooding.
    for (i, entry) in preset.items.iter().enumerate() {
        // Compute the final item ID by applying affinity and upgrade offsets.
        // Use checked_add to prevent overflow from user-created presets
        // with large affinity_offset or upgrade values.
        let final_item_id = match entry.item_id
            .checked_add(entry.affinity_offset)
            .and_then(|id| id.checked_add(entry.upgrade))
        {
            Some(id) => id,
            None => {
                eprintln!(
                    "Skipping item {} in preset '{}': ID overflow (base={}, affinity={}, upgrade={})",
                    entry.item_id, preset.name, entry.item_id, entry.affinity_offset, entry.upgrade
                );
                if let Some(ref mut cb) = progress {
                    cb(i + 1, total);
                }
                continue;
            }
        };
        if let Err(e) = spawn_item_raw(final_item_id, entry.quantity, entry.aow_id) {
            eprintln!("Failed to spawn item {}: {}", entry.item_id, e);
        }
        // Report progress after each spawn
        if let Some(ref mut cb) = progress {
            cb(i + 1, total);
        }
        tokio::time::sleep(std::time::Duration::from_millis(ITEM_SPAWN_DELAY_MS)).await;
    }
    Ok(())
}

/// Get built-in default presets.
pub fn get_default_presets() -> Vec<ItemPreset> {
    vec![
        ItemPreset {
            name: "All Weapons".to_string(),
            items: weapons().iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Talismans".to_string(),
            items: TALISMANS.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Sorceries".to_string(),
            items: SORCERIES.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Incantations".to_string(),
            items: INCANTATIONS.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Potions".to_string(),
            items: POTS_AND_PERFUMES.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 99,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Spirit Ashes".to_string(),
            items: SPIRIT_ASHES.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Ashes of War".to_string(),
            items: ASHES_OF_WAR.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Armor Sets".to_string(),
            items: ARMOR.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Key Items".to_string(),
            items: KEY_ITEMS.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Upgrade Materials".to_string(),
            items: UPGRADE_MATERIALS.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 99,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Crafting Materials".to_string(),
            items: CRAFTING_MATERIALS.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 99,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Consumables".to_string(),
            items: CONSUMABLES.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 99,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Cookbooks".to_string(),
            items: COOKBOOKS.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Crystal Tears".to_string(),
            items: CRYSTAL_TEARS.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Arrows".to_string(),
            items: ARROWS.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 99,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Bell Bearings".to_string(),
            items: BELL_BEARINGS.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
        ItemPreset {
            name: "All Prattling Pates".to_string(),
            items: PRATTLING_PATE.iter().map(|item| ItemEntry {
                item_id: item.id as i64,
                quantity: 1,
                aow_id: -1,
                affinity_offset: 0,
                upgrade: 0,
            }).collect(),
        },
    ]
}

/// Get all items across all categories.
fn all_items() -> Vec<&'static Item> {
    let mut items = Vec::new();
    items.extend(WEAPONS.iter());
    items.extend(TALISMANS.iter());
    items.extend(SORCERIES.iter());
    items.extend(INCANTATIONS.iter());
    items.extend(POTS_AND_PERFUMES.iter());
    items.extend(SPIRIT_ASHES.iter());
    items.extend(ASHES_OF_WAR.iter());
    items.extend(ARMOR.iter());
    items.extend(KEY_ITEMS.iter());
    items.extend(UPGRADE_MATERIALS.iter());
    items.extend(CRAFTING_MATERIALS.iter());
    items.extend(CONSUMABLES.iter());
    items.extend(COOKBOOKS.iter());
    items.extend(CRYSTAL_TEARS.iter());
    items.extend(ARROWS.iter());
    items.extend(BELL_BEARINGS.iter());
    items.extend(PRATTLING_PATE.iter());
    items
}

/// Get all weapons.
fn weapons() -> Vec<&'static Item> {
    WEAPONS.iter().collect()
}

/// Get all available item presets (defaults + user-created).
pub fn get_all_presets() -> Vec<ItemPreset> {
    ITEM_STORE.get_all(get_default_presets())
}
