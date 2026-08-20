use {
    crate::{
        event,
        mem::*,
        offsets::{
            code_cave::CaveAddr,
            module_offsets::{BasePointer, Function},
        },
        resources::{
            ASM,
            aow::{AFFINITIES, AOW, Affinity, Aow},
            items::{
                Categories,
                Item,
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
            },
        },
        utils::{DlcError, VersionError, dlc_check, player_loaded_check, version_check},
    },
    gubtool_core::{address::POINTER, slice_ops::*},
};

pub struct ItemSpawnRequest {
    pub item:     Item,
    pub quantity: i64,
    pub upgrade:  i64,
    pub aow:      Aow,
    pub affinity: Affinity,
}

impl ItemSpawnRequest {
    pub fn new(item: Item) -> Self {
        Self {
            item,
            quantity: 1,
            upgrade: 0,
            aow: AOW[0],
            affinity: AFFINITIES[0],
        }
    }
    pub fn with_quantity(mut self, quantity: i64) -> Self {
        self.quantity = quantity;
        self
    }
    pub fn with_upgrade(mut self, upgrade: i64) -> Self {
        self.upgrade = upgrade;
        self
    }
    pub fn spawn(&mut self) -> anyhow::Result<()> {
        player_loaded_check()?;

        if self.item.dlc || self.aow.dlc {
            if !self.item.requires_activated_dlc() {
                version_check()?;
            } else {
                dlc_check()?;
            }
        }

        self.clamp_values();

        let id = (self.item.id as i64) + self.affinity.id_offset + self.upgrade;
        let max_quantity = self.item.max_storage + self.item.stack_size;
        let is_quantity_adjustable = self.item.stack_size > 1;

        if let Some(event) = self.item.event_id {
            event::set_event(event, true)?;
        }

        itemspawn(id, self.quantity, self.aow.id, is_quantity_adjustable, max_quantity as i64)
    }
    pub fn clamp_values(&mut self) {
        self.quantity = self.quantity.clamp(1, self.item.stack_size as i64);

        let new_upgrade = match self.item.category {
            Categories::Weapons
                if self.upgrade > 25 && matches!(self.item.gem_mount_type, Some(1) | Some(2)) =>
            {
                Some(25)
            }
            Categories::Weapons if self.upgrade > 10 && self.item.upgrade_type == Some(1) => {
                Some(10)
            }
            Categories::SpiritAshes if self.upgrade > 10 => Some(10),
            Categories::Weapons | Categories::SpiritAshes => None,
            _ => Some(0),
        };
        if let Some(v) = new_upgrade {
            self.upgrade = v
        }

        if !self.aow.supports_item(self.item) {
            self.aow = AOW[0];
        }
        if !self.aow.supports_affinity(self.affinity) {
            self.affinity = AFFINITIES[0];
        }
    }

    pub fn can_quantity(&self) -> bool {
        self.item.stack_size > 1
    }

    pub fn can_upgrade(&self) -> bool {
        matches!(self.item.category, Categories::Weapons | Categories::SpiritAshes)
    }

    pub fn can_aow(&self) -> bool {
        self.item.weapon_type.is_some() && (self.item.gem_mount_type != Some(0))
    }
}

fn itemspawn(
    item_id: i64,
    quantity: i64,
    aow_id: i64,
    is_quantity_adjustable: bool,
    max_quantity: i64,
) -> anyhow::Result<()> {
    let mut item_struct: [u8; 96] = [0x0; 96];
    write_to_slice::<i32>(&mut item_struct, 0x40, 1)?;
    write_to_slice::<u32>(&mut item_struct, 0x44, item_id)?;
    write_to_slice::<i32>(&mut item_struct, 0x48, quantity)?;
    write_to_slice::<i32>(&mut item_struct, 0x4c, -1)?;
    write_to_slice::<i32>(&mut item_struct, 0x50, aow_id)?;

    let mut fun = ASM.get_function("item_spawn");

    fun.patch::<POINTER>("item_struct", CaveAddr::ItemSpawnStruct);
    fun.patch::<POINTER>("check_quantity_flag", CaveAddr::ShouldCheckQuantity);
    fun.patch::<POINTER>("fn_get_item_quantity", Function::GetPlayerItemQuantityById);
    fun.patch::<POINTER>("max_quantity", CaveAddr::MaxQuantity);
    fun.patch::<POINTER>("map_item_man_impl", BasePointer::MapItemManImpl);
    fun.patch::<POINTER>("fn_item_spawn", Function::ItemSpawn);

    write::<u8>(CaveAddr::ShouldCheckQuantity, is_quantity_adjustable as u8)?;
    write::<i32>(CaveAddr::MaxQuantity, max_quantity as i32)?;
    write_bytes(CaveAddr::ItemSpawnStruct, &item_struct)?;

    run_custom_function(fun)
}

pub fn mass_spawn(category: Categories, quantity: i64, upgrade: i64) -> anyhow::Result<()> {
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
        let mut request = ItemSpawnRequest::new(*item)
            .with_quantity(quantity)
            .with_upgrade(upgrade);

        if let Err(err) = request.spawn()
            && !(err.is::<DlcError>() || err.is::<VersionError>())
        {
            return Err(err);
        }
    }
    Ok(())
}

impl Item {
    pub fn valid_aows(&self) -> Vec<Aow> {
        AOW.into_iter()
            .filter(|aow| aow.supports_item(*self))
            .collect()
    }
    pub fn requires_activated_dlc(&self) -> bool {
        matches!(
            self.category,
            Categories::BellBearings
                | Categories::Consumables
                | Categories::Cookbooks
                | Categories::CraftingMaterials
                | Categories::CrystalTears
                | Categories::Incantations
                | Categories::KeyItems
                | Categories::PotsAndPerfumes
                | Categories::PrattlingPate
                | Categories::Sorceries
        )
    }
}

impl Aow {
    pub fn valid_affinities(&self) -> Vec<Affinity> {
        AFFINITIES
            .into_iter()
            .filter(|affinity| self.supports_affinity(*affinity))
            .collect()
    }
}
