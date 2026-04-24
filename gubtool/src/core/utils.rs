use anyhow::{Result, anyhow, ensure};
use thiserror::Error;
use crate::{
    core::{
        mem::*,
        attach::{ATTACHED_PROCESS, Version},
    },
    offsets::{cs_dlc_imp, world_chr_man},
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
    matches!(unsafe { ATTACHED_PROCESS.version },
        Version::V2_2_0 |
        Version::V2_2_3 |
        Version::V2_3_0 |
        Version::V2_4_0 |
        Version::V2_5_0 |
        Version::V2_6_0 |
        Version::V2_6_1)
}

pub fn version_check() -> Result<()> {
    ensure!(is_version_dlc_compat(), VersionError);
    Ok(())
}

pub fn is_character_loaded() -> Result<bool> {
    read::<u64>(world_chr_man::base())
        .and_then(|addr| read::<u64>(addr + world_chr_man::player_ins()))
        .map(|val| val != 0)
}

pub fn character_loaded_check() -> Result<()> {
    is_character_loaded().map_err(|_| anyhow!("Character not loaded"))?;
    Ok(())
}
