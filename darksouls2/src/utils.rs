pub use crate::{offsets::module_offsets::scan::*, resources::print_asm_sizes};
use {
    crate::{is_player_loaded, resources::map_ids::MapId, utility},
    anyhow::Result,
    gubtool_core::attached::is_32,
    std::fmt::Debug,
    thiserror::Error,
};

#[derive(Error, Debug)]
#[error("Requires Scholar of the First Sin")]
pub struct ScholarError;

#[derive(Error, Debug)]
#[error("Player not loaded")]
pub struct LoadedError;

#[derive(Error, Debug)]
#[error("Must be in {map_id}")]
pub struct AreaError {
    map_id: MapId,
}

pub fn player_loaded_check() -> anyhow::Result<()> {
    crate::mem::ensure_game()?;
    if is_player_loaded() { Ok(()) } else { Err(LoadedError) }?;
    Ok(())
}

pub fn scholar_check() -> Result<(), ScholarError> {
    if is_32() { Err(ScholarError) } else { Ok(()) }
}

pub fn area_check(map_id: MapId) -> anyhow::Result<()> {
    player_loaded_check()?;

    if let Ok(id) = utility::get_area_id()
        && id == map_id as u32
    {
        Ok(())
    } else {
        Err(AreaError {
            map_id,
        })
    }?;
    Ok(())
}
