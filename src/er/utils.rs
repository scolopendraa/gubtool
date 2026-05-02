use anyhow::{Result, anyhow, ensure};
use thiserror::Error;

use crate::{
    core::attach::{Game, Version, game, module_handle, version},
    er::{
        game_state::is_loaded, mem::*, offsets::{cs_dlc_imp}
    },
};

#[derive(Error, Debug)]
#[error("DLC not found")]
pub struct DlcError;

#[derive(Error, Debug)]
#[error("Requires version 1.12 or above")]
pub struct VersionError;

pub fn is_dlc_available() -> Result<bool> {
    read::<u64>(cs_dlc_imp::base())
        .and_then(|addr| read::<u8>(addr + cs_dlc_imp::BYTE_FLAGS + cs_dlc_imp::flags::DLC_CHECK))
        .map(|val| val == 1)
}

pub fn dlc_check() -> Result<()> {
    ensure!(is_dlc_available()?, DlcError);
    Ok(())
}

pub fn is_version_dlc_compat() -> bool {
    if game() == Game::EldenRing && module_handle() != 0 {
        matches!(version(),
            Version::ER2_2_0 |
            Version::ER2_2_3 |
            Version::ER2_3_0 |
            Version::ER2_4_0 |
            Version::ER2_5_0 |
            Version::ER2_6_0 |
            Version::ER2_6_1)
    } else {
        true
    }
}

pub fn version_check() -> Result<()> {
    ensure!(is_version_dlc_compat(), VersionError);
    Ok(())
}

pub fn character_loaded_check() -> Result<()> {
    let loaded = is_loaded().map_err(|_| anyhow!("Character not loaded"))?;
    ensure!(loaded, "Character not loaded");
    Ok(())
}
