use anyhow::{Result, anyhow, ensure};
use thiserror::Error;

use crate::{
    core::attach::{Game, Version, game, version},
    ds2::game_state::is_loaded,
};

#[derive(Error, Debug)]
#[error("Requires Scholar of the First Sin")]
pub struct ScholarError;

pub fn is_scholar() -> bool {
    game() != Game::DarkSoulsII || matches!(
        version(),
        Version::Scholar1_0_1
            | Version::Scholar1_0_2
            | Version::Scholar1_0_3
            | Version::ScholarUnknown
    )
}

pub fn character_loaded_check() -> Result<()> {
    let loaded = is_loaded().map_err(|_| anyhow!("Character not loaded"))?;
    ensure!(loaded, "Character not loaded");
    Ok(())
}