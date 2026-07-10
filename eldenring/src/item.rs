use crate::{
    event,
    mem::*,
    offsets::{
        code_cave::CaveOffset,
        module_offsets::{BasePointer, Function},
    },
    resources::{
        ASM,
        aow::{AFFINITIES, Affinity, Aow, aow_array},
        items::{
            Categories, Item, armor::ARMOR, arrows::ARROWS, ashes_of_war::ASHES_OF_WAR,
            bell_bearings::BELL_BEARINGS, consumables::CONSUMABLES, cookbooks::COOKBOOKS,
            crafting_materials::CRAFTING_MATERIALS, crystal_tears::CRYSTAL_TEARS,
            incantations::INCANTATIONS, key_items::KEY_ITEMS, pots_and_perfumes::POTS_AND_PERFUMES,
            prattling_pate::PRATTLING_PATE, sorceries::SORCERIES, spirit_ashes::SPIRIT_ASHES,
            talismans::TALISMANS, upgrade_materials::UPGRADE_MATERIALS, weapons::WEAPONS,
        },
    },
    utils::{DlcError, VersionError, dlc_check, player_loaded_check, version_check},
};
use gubtool_core::{slice_ops::*, sys::error::ProcResult};

/// Spawn an item with raw parameters.
/// This is useful for presets and bulk operations.
pub fn spawn_item_raw(
    item_id: i64,
    quantity: i64,
    aow_id: i64,
) -> ProcResult {
    itemspawn(item_id, quantity, aow_id, false, quantity)
}

fn itemspawn(
    item_id: i64,
    quantity: i64,
    aow_id: i64,
    is_quantity_adjustable: bool,
    max_quantity: i64,
) -> ProcResult {
    let mut item_struct: [u8; 96] = [0x0; 96];
    write_to_slice::<i32>(&mut item_struct, 0x40, 1)?;
    write_to_slice::<u32>(&mut item_struct, 0x44, item_id)?;
    write_to_slice::<i32>(&mut item_struct, 0x48, quantity)?;
    write_to_slice::<i32>(&mut item_struct, 0x4C, -1)?;
    write_to_slice::<i32>(&mut item_struct, 0x50, aow_id)?;

    let mut fun = ASM.get_function("item_spawn");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("item_struct"), CaveOffset::ItemSpawnStruct)?;
    write_addr_to_slice(&mut asm, fun.reloc("check_quantity_flag"), CaveOffset::ShouldCheckQuantity)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_get_item_quantity"), Function::GetPlayerItemQuantityById)?;
    write_addr_to_slice(&mut asm, fun.reloc("max_quantity"), CaveOffset::MaxQuantity)?;
    write_addr_to_slice(&mut asm, fun.reloc("map_item_man_impl"), BasePointer::MapItemManImpl)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_item_spawn"), Function::ItemSpawn)?;

    let _handle = ITEM_SPAWN_MUTEX.lock().unwrap();

    write::<u8>(CaveOffset::ShouldCheckQuantity, is_quantity_adjustable as u8)?;
    write::<i32>(CaveOffset::MaxQuantity, max_quantity as i32)?;
    write_bytes(CaveOffset::ItemSpawnStruct, &item_struct)?;

    spawn_thread_join(CaveOffset::ItemSpawnAsm, asm)
}

pub fn mass_spawn(category: Categories) -> anyhow::Result<()> {
    let items: &'static [Item] = match category {
            Categories::Armor => &ARMOR,
            Categories::Arrows => &ARROWS,
            Categories::AshesOfWar => &ASHES_OF_WAR,
            Categories::BellBearings => &BELL_BEARINGS,
            Categories::Consumables => &CONSUMABLES,
            Categories::Cookbooks => &COOKBOOKS,
            Categories::CraftingMaterials => &CRAFTING_MATERIALS,
            Categories::CrystalTears => &CRYSTAL_TEARS,
            Categories::Incantations => &INCANTATIONS,
            Categories::KeyItems => &KEY_ITEMS,
            Categories::PotsAndPerfumes => &POTS_AND_PERFUMES,
            Categories::PrattlingPate => &PRATTLING_PATE,
            Categories::Sorceries => &SORCERIES,
            Categories::SpiritAshes => &SPIRIT_ASHES,
            Categories::Talismans => &TALISMANS,
            Categories::UpgradeMaterials => &UPGRADE_MATERIALS,
            Categories::Weapons => &WEAPONS,
    };
    for item in items {
        if let Err(err) = item.spawn(1, 0, aow_array()[0], AFFINITIES[0]) &&
            !(err.is::<DlcError>() || err.is::<VersionError>()) {
                return Err(err);
        }
    }
    Ok(())
}

impl Item {
    pub fn spawn(
        &self,
        quantity: i64,
        upgrade: i64,
        aow: Aow,
        affinity: Affinity,
    ) -> anyhow::Result<()> {
        player_loaded_check()?;
        if self.dlc {
            if !self.requires_activated_dlc() {
                version_check()?;
            } else {
                dlc_check()?;
            }
        }

        let quantity = self.clamp_quantity(quantity).unwrap_or(quantity);
        let upgrade = self.clamp_upgrade(upgrade).unwrap_or(upgrade);
        let aow_id = if aow.supports_item(*self) { aow.id } else { -1 };
        let affinity_offset = if aow.supports_affinity(affinity.flag) { affinity.id_offset } else { 0 };
        let id = (self.id as i64) + affinity_offset + upgrade;
        let max_quantity = self.max_storage + self.stack_size;
        let is_quantity_adjustable = self.stack_size > 1;

        if let Some(event) = self.event_id {
            event::set_event(event, true)?;
        }
        itemspawn(id, quantity, aow_id, is_quantity_adjustable, max_quantity as i64)?;
        Ok(())
    }
    pub fn clamp_quantity(&self, quantity: i64) -> Option<i64> {
        (quantity > self.stack_size as i64).then_some(self.stack_size as i64)
    }
    pub fn clamp_upgrade(&self, upgrade: i64) -> Option<i64> {
        if upgrade == 0 {
            return None;
        }
        match self.category {
            Categories::Weapons if upgrade > 25 && matches!(self.gem_mount_type, Some(1) | Some(2)) => Some(25),
            Categories::Weapons if upgrade > 10 && self.upgrade_type == Some(1) => Some(10),
            Categories::SpiritAshes if upgrade > 10 => Some(10),
            Categories::Weapons | Categories::SpiritAshes => None,
            _ => Some(0)
        }
    }
    pub fn requires_activated_dlc(&self) -> bool {
        matches!(self.category,
            Categories::BellBearings|
            Categories::Consumables|
            Categories::Cookbooks|
            Categories::CraftingMaterials|
            Categories::CrystalTears|
            Categories::Incantations|
            Categories::KeyItems|
            Categories::PotsAndPerfumes|
            Categories::PrattlingPate|
            Categories::Sorceries)
    }
}
