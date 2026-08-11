use crate::{
    app::App,
    event::{Event, send_event},
    theme::{ThemeChoice, set_theme},
};
use anyhow::Result;
use config::Config;
use gubtool_core::{
    appdata::{AppDataError, app_data_dir},
    game_version::Game,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UiState {
    #[serde(rename = "global")]
    pub global: GlobalState,
}

impl Config for UiState {
    fn get_path() -> Result<PathBuf, AppDataError> {
        let appdata_dir = app_data_dir()?;
        Ok(appdata_dir.join("ui_state.toml"))
    }
    fn read() -> Result<Self, AppDataError> {
        let config_path = Self::get_path()?;
        let contents = fs::read_to_string(config_path)?;
        let ui_state: UiState = toml::from_str(&contents)?;
        Ok(ui_state)
    }
    fn write(&self) -> Result<(), AppDataError> {
        let path = Self::get_path()?;
        let toml = toml::to_string(self)?;
        fs::write(path, toml)?;
        Ok(())
    }
    fn update<F>(modifier: F) -> Result<(), AppDataError>
    where
        F: FnOnce(&mut UiState),
    {
        let mut toml = Self::read().unwrap_or_default();
        modifier(&mut toml);
        Self::write(&toml)
    }
}

impl UiState {
    pub fn apply(app: &mut App) {
        let config: Self = Self::read().unwrap_or_default();
        config.global.apply(app);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GlobalState {
    pub theme: ThemeChoice,
    pub game_screen: Game,
    pub has_pressed_f1: bool,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            theme: ThemeChoice::default(),
            game_screen: Game::EldenRing,
            has_pressed_f1: false,
        }
    }
}

impl GlobalState {
    fn apply(self, app: &mut App) {
        send_event(Event::GameScreen(self.game_screen));
        set_theme(self.theme);
        app.has_pressed_f1 = self.has_pressed_f1;
    }
}
