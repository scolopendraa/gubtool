/// Generic preset storage infrastructure.
///
/// Provides shared file I/O, locking, validation, and CRUD operations
/// for preset-based systems (grace presets, item presets, etc.).
///
/// Each preset type implements the `PresetEntry` trait and creates its own
/// `PresetStore` instance with a unique file name.

use anyhow::Result;
use gubtool_core::appdata::app_data_dir;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs, path::PathBuf, sync::Mutex};

/// Trait for preset entries that can be stored.
///
/// Implementors must provide a name for duplicate detection and be
/// serializable to/from JSON.
pub trait PresetEntry: Serialize + DeserializeOwned + Clone {
    /// Returns the preset's display name.
    fn name(&self) -> &str;
}

/// Generic preset store with shared file I/O and locking.
///
/// Each store instance manages a single preset file and provides
/// thread-safe load/save/add/delete operations.
pub struct PresetStore<E: PresetEntry> {
    lock: Mutex<()>,
    file_name: &'static str,
    _marker: std::marker::PhantomData<E>,
}

impl<E: PresetEntry> PresetStore<E> {
    /// Creates a new preset store for the given file name.
    pub const fn new(file_name: &'static str) -> Self {
        Self {
            lock: Mutex::new(()),
            file_name,
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns the full path to the preset file.
    fn presets_file(&self) -> PathBuf {
        let mut dir = app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
        dir.push("eldenring");
        dir.push(self.file_name);
        dir
    }

    /// Load presets from disk.
    ///
    /// Returns an empty vec if the file doesn't exist.
    pub fn load(&self) -> Result<Vec<E>> {
        let path = self.presets_file();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let data = fs::read_to_string(&path)?;
        let presets: Vec<E> = serde_json::from_str(&data)?;
        Ok(presets)
    }

    /// Save presets to disk atomically.
    ///
    /// Writes to a temp file first, then renames to the target path
    /// to prevent corruption if the process crashes during the write.
    pub fn save(&self, presets: &[E]) -> Result<()> {
        let data = serde_json::to_string_pretty(presets)?;
        let path = self.presets_file();
        let dir = path.parent().ok_or_else(|| {
            anyhow::anyhow!("Failed to get parent directory for preset file")
        })?;
        fs::create_dir_all(dir)?;

        let mut tmp_path = path.clone();
        tmp_path.set_extension("json.tmp");

        fs::write(&tmp_path, &data)?;
        fs::rename(&tmp_path, &path)?;

        Ok(())
    }

    /// Validate a preset name: must be non-empty and not contain only whitespace.
    pub fn validate_name(name: &str) -> Result<()> {
        if name.trim().is_empty() {
            anyhow::bail!("Preset name cannot be empty or whitespace");
        }
        Ok(())
    }

    /// Add a new preset and save it.
    ///
    /// Checks for duplicate names (case-insensitive) and serializes
    /// the load-modify-save sequence under the store lock.
    pub fn add(&self, entry: E) -> Result<()> {
        Self::validate_name(entry.name())?;

        let _lock = self.lock.lock().unwrap();
        let mut presets = self.load()?;

        if presets.iter().any(|p| p.name().to_lowercase() == entry.name().to_lowercase()) {
            anyhow::bail!("A preset with the name '{}' already exists", entry.name());
        }

        presets.push(entry);
        self.save(&presets)?;
        Ok(())
    }

    /// Delete a preset by index into the custom presets vec.
    ///
    /// The index refers to the position in the user-created presets
    /// (i.e., after removing default presets).
    pub fn delete(&self, index: usize) -> Result<()> {
        let _lock = self.lock.lock().unwrap();
        let presets = self.load()?;
        if index >= presets.len() {
            anyhow::bail!(
                "Preset index {} out of bounds ({} custom presets)",
                index,
                presets.len()
            );
        }
        let mut presets = presets;
        presets.remove(index);
        self.save(&presets)?;
        Ok(())
    }

    /// Get all available presets (defaults + user-created).
    ///
    /// The defaults are prepended to the user-created presets.
    pub fn get_all(&self, defaults: Vec<E>) -> Vec<E> {
        let mut presets = defaults;
        if let Ok(user_presets) = self.load() {
            presets.extend(user_presets);
        }
        presets
    }
}

/// Type alias for lazy-initialized preset stores.
pub type LazyPresetStore<E> = Lazy<PresetStore<E>>;
