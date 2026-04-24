use anyhow::Result;
use crate::{
    resources::{
        items::{
            armor::ARMOR,
            arrows::ARROWS,
            ashes_of_war::ASHES_OF_WAR,
            bell_bearings::BELL_BEARINGS,
            consumables::CONSUMABLES,
            cookbooks::COOKBOOKS,
            crafting_materials::CRAFTING_MATERIALS,
            crystal_tears::CRYSTAL_TEARS,
            incantations::INCANTATIONS,
            key_items::KEY_ITEMS,
            pots_and_perfumes::POTS_AND_PERFUMES,
            prattling_pate::PRATTLING_PATE,
            sorceries::SORCERIES,
            spirit_ashes::SPIRIT_ASHES,
            talismans::TALISMANS,
            upgrade_materials::UPGRADE_MATERIALS,
            weapons::WEAPONS,
            Categories, Item
        },
        aow::{AFFINITIES, Affinity, Aow, aow_array},
        asm,
    },
    core::{
        mem::*,
        event::set_event,
        utils::{DlcError, VersionError, character_loaded_check, dlc_check, version_check},
    },
    offsets::{self, code_cave, functions},
};

pub fn itemspawn(item_id: i64, quantity: i64, aow_id: i64,
    is_quantity_adjustable: bool, max_quantity: i64) -> Result<()> {
    let location = code_cave::base() + code_cave::ITEM_SPAWN_ASM;
    let should_adjust_quantity = code_cave::base() + code_cave::SHOULD_CHECK_QUANTITY;
    let max_quantity_location = code_cave::base() + code_cave::MAX_QUANTITY;
    let item_struct_location = code_cave::base() + code_cave::ITEM_SPAWN_STRUCT;

    let mut item_struct: [u8; 96] = [0x0; 96];
    write_to_slice::<i32>(&mut item_struct, 0x40, 1)?;
    write_to_slice::<u32>(&mut item_struct, 0x44, item_id)?;
    write_to_slice::<i32>(&mut item_struct, 0x48, quantity)?;
    write_to_slice::<i32>(&mut item_struct, 0x4C, -1)?;
    write_to_slice::<i32>(&mut item_struct, 0x50, aow_id)?;

    let mut asm = asm::ITEM_SPAWN;
    write_to_slice::<i32>(&mut asm, 7, rel_i32(item_struct_location, location + 11)?)?;
    write_to_slice::<i32>(&mut asm, 13, rel_i32(should_adjust_quantity, location + 17)?)?;
    write_to_slice::<i32>(&mut asm, 25, rel_i32(functions::get_player_item_quantity_by_id(), location + 29)?)?;
    write_to_slice::<i32>(&mut asm, 31, rel_i32(max_quantity_location, location + 35)?)?;
    write_to_slice::<i32>(&mut asm, 52, rel_i32(offsets::map_item_impl::base(), location + 56)?)?;
    write_to_slice::<i32>(&mut asm, 71, rel_i32(functions::item_spawn(), location + 75)?)?;
    let asm = append_flag_setter(location, &asm)?;

    let _handle = ITEM_SPAWN_MUTEX.lock().unwrap();

    write::<u8>(should_adjust_quantity, is_quantity_adjustable as u8)?;
    write::<i32>(max_quantity_location, max_quantity as i32)?;
    write_bytes(item_struct_location, &item_struct)?;
    write_bytes(location, &asm)?;
    run_win_thread_wait(location)
}

pub fn mass_spawn(category: Categories) -> Result<()> {
    let items: Vec<Item> = match category {
            Categories::Armor => ARMOR.to_vec(),
            Categories::Arrows => ARROWS.to_vec(),
            Categories::AshesOfWar => ASHES_OF_WAR.to_vec(),
            Categories::BellBearings => BELL_BEARINGS.to_vec(),
            Categories::Consumables => CONSUMABLES.to_vec(),
            Categories::Cookbooks => COOKBOOKS.to_vec(),
            Categories::CraftingMaterials => CRAFTING_MATERIALS.to_vec(),
            Categories::CrystalTears => CRYSTAL_TEARS.to_vec(),
            Categories::Incantations => INCANTATIONS.to_vec(),
            Categories::KeyItems => KEY_ITEMS.to_vec(),
            Categories::PotsAndPerfumes => POTS_AND_PERFUMES.to_vec(),
            Categories::PrattlingPate => PRATTLING_PATE.to_vec(),
            Categories::Sorceries => SORCERIES.to_vec(),
            Categories::SpiritAshes => SPIRIT_ASHES.to_vec(),
            Categories::Talismans => TALISMANS.to_vec(),
            Categories::UpgradeMaterials => UPGRADE_MATERIALS.to_vec(),
            Categories::Weapons => WEAPONS.to_vec(),
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
    pub fn spawn(&self, quantity: i64, upgrade: i64, aow: Aow, affinity: Affinity) -> Result<()> {
        character_loaded_check()?;
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
            set_event(event, true)?;
        }
        itemspawn(id, quantity, aow_id, is_quantity_adjustable, max_quantity as i64)
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
