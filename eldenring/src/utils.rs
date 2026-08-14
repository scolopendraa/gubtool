pub use offsets::module_offsets::scan::*;
use {
    crate::{event::get_event, is_dlc_available, is_player_loaded, offsets, resources::ASM},
    anyhow::bail,
    gubtool_core::{attached::version, game_version::EldenRingVersion::*},
    std::{
        thread,
        time::{Duration, Instant},
    },
    thiserror::Error,
};

#[derive(Error, Debug)]
#[error("DLC not found")]
pub struct DlcError;

#[derive(Error, Debug)]
#[error("Requires version 1.12 or above")]
pub struct VersionError;

#[derive(Error, std::fmt::Debug)]
#[error("Player not loaded")]
pub struct LoadedError;

pub fn dlc_check() -> anyhow::Result<()> {
    crate::mem::ensure_game()?;
    if is_dlc_available() {
        Ok(())
    } else {
        Err(DlcError)
    }?;
    Ok(())
}

pub fn is_version_dlc_compat() -> bool {
    matches!(
        version(),
        Some(Version2_2_0)
            | Some(Version2_2_3)
            | Some(Version2_3_0)
            | Some(Version2_4_0)
            | Some(Version2_5_0)
            | Some(Version2_6_0)
            | Some(Version2_6_1)
            | Some(Version2_6_2)
            | None
    )
}

pub fn version_check() -> Result<(), VersionError> {
    if !is_version_dlc_compat() {
        Err(VersionError)
    } else {
        Ok(())
    }
}

pub fn player_loaded_check() -> anyhow::Result<()> {
    crate::mem::ensure_game()?;
    if is_player_loaded() {
        Ok(())
    } else {
        Err(LoadedError)
    }?;
    Ok(())
}

pub(crate) fn wait_for_event(event_id: u32, state: bool, timeout_secs: u64) -> anyhow::Result<()> {
    let start = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    while get_event(event_id)? != state {
        if start.elapsed() > timeout {
            bail!("Event flag {} was not set to {} within {:#?}", event_id, state, timeout)
        }
        thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

pub(crate) fn wait_for_cutscence_completion() -> anyhow::Result<()> {
    wait_for_event(2200, true, 30)?;
    wait_for_event(2200, false, 120)
}

pub fn print_asm_sizes() {
    println!("Elden Ring");
    ASM.print_function_sizes();
    println!("\n");
}
