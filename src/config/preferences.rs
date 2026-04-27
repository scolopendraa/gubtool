use crate::{
    config::Config,
    er::{
        chr_ins::ChrInsExt,
        game_state::{self, GameStateFlags},
        offsets::chr_dbg_flags::ChrDbgOffsets,
        player::{self, player_ins},
        utility,
    },
};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Preferences {
    #[serde(rename = "dark_souls_2")]
    pub dark_souls_2: Ds2Preferences,

    #[serde(rename = "elden_ring")]
    pub elden_ring: ErPreferences,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ErPreferences {
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ds2Preferences {

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
    pub fn update_er<F>(modifier: F) -> Result<()>
    where
        F: FnOnce(&mut ErPreferences),
    {
        let mut er_prefs: ErPreferences = Self::read().unwrap_or_default().elden_ring;
        modifier(&mut er_prefs);
        Self::update(|c| c.elden_ring = er_prefs)
    }
    pub fn apply_elden_ring() -> Result<()> {
        let config_path = Self::get_path()?;
        if !config_path.exists() {
            return Ok(());
        }
        let er: ErPreferences = Self::read()?.elden_ring;

        if er.no_death {
            player::set_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath, true).ok();
        }
        if er.no_damage {
            game_state::set_state_flag(GameStateFlags::PlayerNoDamage, true).ok();
            player_ins().set_no_damage(true).ok();
        }
        if er.rfbs_on_load {
            game_state::set_state_flag(GameStateFlags::Rfbs, true).ok();
        }
        if er.infinite_poise {
            player::set_infinite_poise(true).ok();
        }
        if let Some(val) = er.fps {
            utility::set_fps_cap(val).ok();
        }
        if er.remove_logo {
            utility::set_logo_patch(true).ok();
        }
        if er.mute_music {
            utility::mute_music(true).ok();
        }
        if er.stutter_fix {
            utility::set_stutter_fix(true).ok();
        }
        if er.disable_area_target_cards {
            game_state::set_state_flag(GameStateFlags::TitleCards, true).ok();
        }
        Ok(())
    }

    pub fn apply_dark_souls_2() -> Result<()> {
        Ok(())
    }
}