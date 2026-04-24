use anyhow::{Result, anyhow};
use ratatui_themes::ThemeName;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use crate::{
    core::{
        chr_ins::ChrInsExt,
        game_state::{self, GameStateFlags},
        player::{self, player_ins},
        utility,
    },
    offsets::chr_dbg_flags::ChrDbgOffsets,
    tui::app::App
};

pub trait Config: Serialize + for<'a> Deserialize<'a> + Default + Clone {
    fn get_path() -> Result<PathBuf>;
    fn read() -> Result<Self> where Self: Sized;
    fn write(&self) -> Result<()>;
    fn update<F>(modifier: F) -> Result<()>
    where F: FnOnce(&mut Self);
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Preferences {
    pub no_death: bool,
    pub no_damage: bool,
    pub rfbs_on_load: bool,
    pub infinite_poise: bool,
    pub fps: Option<f32>,
    pub remove_logo: bool,
    pub mute_music: bool,
    pub disable_area_target_cards: bool,
    pub stutter_fix: bool,
}

impl Config for Preferences {
    fn get_path() -> Result<PathBuf> {
        let Some(home_dir) = env::home_dir() else {
            return Err(anyhow!("Home directory not found"));
        };
        Ok(home_dir
            .join(".local")
            .join("state")
            .join("gubtool")
            .join("preferences.toml"))
    }
    fn read() -> Result<Self> {
        let config_path = Self::get_path()?;
        if !config_path.exists() {
            return Err(anyhow!("Config file not found"));
        }
        let contents = fs::read_to_string(config_path).map_err(|_| {
            anyhow!("Error while reading preferences.toml. Preferences not initialized.")
        })?;

        let preferences: Preferences = toml::from_str(&contents).map_err(|_| {
            anyhow!("Error while parsing preferences.toml. Preferences not initialized.")
        })?;
        Ok(preferences)
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
        F: FnOnce(&mut Preferences),
    {
        let mut toml = Self::read().unwrap_or_default();
        modifier(&mut toml);
        Self::write(&toml)
    }
}
impl Preferences {
    pub fn apply() -> Result<()> {
        let config_path = Self::get_path()?;
        if !config_path.exists() {
            return Ok(());
        }
        let config: Self = Self::read()?;

        if config.no_death {
            player::set_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath, true).ok();
        }
        if config.no_damage {
            game_state::set_state_flag(GameStateFlags::PlayerNoDamage, true).ok();
            player_ins().set_no_damage(true).ok();
        }
        if config.rfbs_on_load {
            game_state::set_state_flag(GameStateFlags::Rfbs, true).ok();
        }
        if config.infinite_poise {
            player::set_infinite_poise(true).ok();
        }
        if let Some(val) = config.fps {
            utility::set_fps_cap(val).ok();
        }
        if config.remove_logo {
            utility::set_logo_patch(true).ok();
        }
        if config.mute_music {
            utility::mute_music(true).ok();
        }
        if config.stutter_fix {
            utility::set_stutter_fix(true).ok();
        }
        if config.disable_area_target_cards {
            game_state::set_state_flag(GameStateFlags::TitleCards, true).ok();
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiState {
    pub theme: ThemeName,
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

impl Default for UiState {
    fn default() -> Self {
        Self {
            theme: ThemeName::default(),
            player_set_health: 100,
            give_runes: 10000,
            target_set_health: 1,
            target_set_health_pct: 50,
            target_act: 1,
            target_act_array: vec![],
            event: None,
            revive_first_encounter: false,
            warp_on_revive: false,
        }
    }
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
        app.theme  = config.theme;
        app.player.hp = config.player_set_health;
        app.player.runes = config.give_runes;
        app.target.hp_val = config.target_set_health;
        app.target.hp_percentage = config.target_set_health_pct;
        app.target.act = config.target_act;
        app.target.act_array = config.target_act_array;
        app.event.event = config.event;
        app.event.first_encounter = config.revive_first_encounter;
        app.event.warp = config.warp_on_revive;
    }
}
