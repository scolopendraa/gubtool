use crate::{
    event,
    preset_storage::{LazyPresetStore, PresetEntry, PresetStore},
    resources::graces::graces_array,
    utils::is_dlc_available,
};
use gubtool_core::sys::error::{AccessType, ProcResult};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// A preset containing a list of grace entity IDs to unlock.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GracePreset {
    pub name: String,
    pub grace_entity_ids: Vec<i64>,
}

impl PresetEntry for GracePreset {
    fn name(&self) -> &str {
        &self.name
    }
}

/// Shared preset store for grace presets.
static GRACE_STORE: LazyPresetStore<GracePreset> =
    Lazy::new(|| PresetStore::new("grace_presets.json"));

/// Load grace presets from disk.
pub fn load_presets() -> anyhow::Result<Vec<GracePreset>> {
    GRACE_STORE.load()
}

/// Save grace presets to disk atomically.
pub fn save_presets(presets: &[GracePreset]) -> anyhow::Result<()> {
    GRACE_STORE.save(presets)
}

/// Validate a preset name: must be non-empty and not contain only whitespace.
pub fn validate_preset_name(name: &str) -> anyhow::Result<()> {
    PresetStore::<GracePreset>::validate_name(name)
}

/// Add a new grace preset and save it.
pub fn add_preset(name: &str, grace_entity_ids: Vec<i64>) -> anyhow::Result<()> {
    GRACE_STORE.add(GracePreset {
        name: name.to_string(),
        grace_entity_ids,
    })
}

/// Delete a grace preset by index into the custom presets vec.
pub fn delete_preset(index: usize) -> anyhow::Result<()> {
    GRACE_STORE.delete(index)
}

/// Apply a grace preset by index - unlocks all graces in the preset.
pub fn apply_preset(index: usize) -> ProcResult {
    let all_presets = get_all_presets();
    let preset = all_presets.get(index).ok_or_else(|| {
        gubtool_core::sys::error::ProcessError::partial_access(
            AccessType::Read("grace_presets"),
            0,
            0,
        )
    })?;

    // Unlock each grace by its flag_id, continuing on individual failures.
    // This ensures partial application rather than bailing on the first error.
    let graces = graces_array(is_dlc_available());
    for &grace_id in &preset.grace_entity_ids {
        if let Some(grace) = graces.iter().find(|g| g.grace_entity_id == grace_id) {
            if let Err(e) = event::set_event(grace.flag_id as u32, true) {
                eprintln!(
                    "Failed to unlock grace {} (flag {}): {}",
                    grace_id, grace.flag_id, e
                );
            }
        }
    }
    Ok(())
}

/// Get built-in default presets.
pub fn get_default_presets() -> Vec<GracePreset> {
    let graces = graces_array(is_dlc_available());
    let all_grace_ids: Vec<i64> = graces.iter().map(|g| g.grace_entity_id).collect();

    vec![
        GracePreset {
            name: "All Graces".to_string(),
            grace_entity_ids: all_grace_ids.clone(),
        },
        GracePreset {
            name: "Limgrave Only".to_string(),
            grace_entity_ids: graces
                .iter()
                .filter(|g| g.main_area == "Limgrave")
                .map(|g| g.grace_entity_id)
                .collect(),
        },
        GracePreset {
            name: "Roundtable Hold".to_string(),
            grace_entity_ids: graces
                .iter()
                .filter(|g| g.main_area == "Roundtable Hold")
                .map(|g| g.grace_entity_id)
                .collect(),
        },
        GracePreset {
            name: "Haligtree".to_string(),
            grace_entity_ids: graces
                .iter()
                .filter(|g| {
                    g.main_area == "Miquella's Haligtree"
                        || g.main_area == "Elphael, Brace of the Haligtree"
                })
                .map(|g| g.grace_entity_id)
                .collect(),
        },
        GracePreset {
            name: "Stormveil Castle".to_string(),
            grace_entity_ids: graces
                .iter()
                .filter(|g| g.main_area == "Stormveil Castle")
                .map(|g| g.grace_entity_id)
                .collect(),
        },
        GracePreset {
            name: "Liurnia".to_string(),
            grace_entity_ids: graces
                .iter()
                .filter(|g| g.main_area == "Liurnia of the Lakes")
                .map(|g| g.grace_entity_id)
                .collect(),
        },
        GracePreset {
            name: "Caelid".to_string(),
            grace_entity_ids: graces
                .iter()
                .filter(|g| g.main_area == "Caelid")
                .map(|g| g.grace_entity_id)
                .collect(),
        },
        GracePreset {
            name: "Altus Plateau".to_string(),
            grace_entity_ids: graces
                .iter()
                .filter(|g| g.main_area == "Altus Plateau")
                .map(|g| g.grace_entity_id)
                .collect(),
        },
        GracePreset {
            name: "Leyndell".to_string(),
            grace_entity_ids: graces
                .iter()
                .filter(|g| {
                    g.main_area == "Leyndell, Royal Capital"
                        || g.main_area == "Leyndell, Ashen Capital"
                })
                .map(|g| g.grace_entity_id)
                .collect(),
        },
        GracePreset {
            name: "Starting Area".to_string(),
            grace_entity_ids: vec![
                18001950,   // Cave of Knowledge
                18001951,   // Stranded Graveyard
                1042361951, // The First Step
            ],
        },
    ]
}

/// Get all available grace presets (defaults + user-created).
pub fn get_all_presets() -> Vec<GracePreset> {
    GRACE_STORE.get_all(get_default_presets())
}
