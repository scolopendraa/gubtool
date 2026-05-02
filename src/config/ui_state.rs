use crate::{
    config::Config,
    core::attach::Game,
    tui::{self, app::App},
};
use anyhow::{Result, anyhow};
use ratatui_themes::ThemeName;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UiState {
    #[serde(rename = "global")]
    pub global: GlobalState,

    #[serde(rename = "dark_souls_2")]
    dark_souls_2: Ds2State,

    #[serde(rename = "elden_ring")]
    elden_ring: ErState,
}


impl Config for UiState {
    fn get_path() -> Result<PathBuf> {
        let Some(home_dir) = env::home_dir() else {
            return Err(anyhow!("Home directory not found"));
        };
        Ok(home_dir
            .join(".local")
            .join("state")
            .join("gubtool")
            .join("ui_state.toml"))
    }
    fn read() -> Result<Self> {
        let config_path = Self::get_path()?;
        if !config_path.exists() {
            return Err(anyhow!("Config file not found"));
        }
        let contents = fs::read_to_string(config_path)?;
        let ui_state: UiState = toml::from_str(&contents)?;
        Ok(ui_state)
    }
    fn write(&self) -> Result<()> {
        let path = Self::get_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let toml = toml::to_string(self)?;
        fs::write(path, toml)?;
        Ok(())
    }
    fn update<F>(modifier: F) -> Result<()>
    where
        F: FnOnce(&mut Self),
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
        config.elden_ring.apply(&mut app.elden_ring);
        config.dark_souls_2.apply();
    }
    pub fn update_er<F>(modifier: F) -> Result<()>
    where
        F: FnOnce(&mut ErState),
    {
        let mut er_state: ErState = Self::read().unwrap_or_default().elden_ring;
        modifier(&mut er_state);
        Self::update(|c| c.elden_ring = er_state)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GlobalState {
    pub theme: ThemeName,
    pub game_screen: Game,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            theme: ThemeName::RosePine,
            game_screen: Game::EldenRing,
        }
    }
}

impl GlobalState {
    fn apply(self, app: &mut tui::app::App) {
        app.theme = self.theme;
        app.game_screen = self.game_screen;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ErState {
    pub player_set_health: i32,
    pub give_runes: i64,
    pub target_set_health: i32,
    pub target_set_health_pct: i32,
    pub target_act: i32,
    pub target_act_array: Vec<i32>,
    pub event: Option<u32>,
    pub revive_first_encounter: bool,
    pub warp_on_revive: bool,
}

impl Default for ErState {
    fn default() -> Self {
        Self {
            player_set_health: 100,
            give_runes: 999999,
            target_set_health: 1,
            target_set_health_pct: 50,
            target_act: 1,
            target_act_array: vec![],
            event: None,
            revive_first_encounter: false,
            warp_on_revive: true,
        }
    }
}

impl ErState {
    fn apply(self, er: &mut tui::er::EldenRing) {
        er.player.hp = self.player_set_health;
        er.player.runes = self.give_runes;
        er.target.hp_val = self.target_set_health;
        er.target.hp_percentage = self.target_set_health_pct;
        er.target.act = self.target_act;
        er.target.act_array = self.target_act_array;
        er.event.event = self.event;
        er.event.first_encounter = self.revive_first_encounter;
        er.event.warp = self.warp_on_revive;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ds2State {
}

impl Default for Ds2State {
    fn default() -> Self {
        Self {
        }
    }
}

impl Ds2State {
    fn apply(self) {
    }
}