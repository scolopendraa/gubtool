pub mod armor;
pub mod arrows;
pub mod ashes_of_war;
pub mod bell_bearings;
pub mod consumables;
pub mod cookbooks;
pub mod crafting_materials;
pub mod crystal_tears;
pub mod incantations;
pub mod key_items;
pub mod pots_and_perfumes;
pub mod prattling_pate;
pub mod sorceries;
pub mod spirit_ashes;
pub mod talismans;
pub mod upgrade_materials;
pub mod weapons;

use clap::ValueEnum;
use once_cell::sync::Lazy;
use std::fmt;

use crate::er::{
    resources::items::{
        armor::ARMOR, arrows::ARROWS, ashes_of_war::ASHES_OF_WAR, bell_bearings::BELL_BEARINGS,
        consumables::CONSUMABLES, cookbooks::COOKBOOKS, crafting_materials::CRAFTING_MATERIALS,
        crystal_tears::CRYSTAL_TEARS, incantations::INCANTATIONS, key_items::KEY_ITEMS,
        pots_and_perfumes::POTS_AND_PERFUMES, prattling_pate::PRATTLING_PATE, sorceries::SORCERIES,
        spirit_ashes::SPIRIT_ASHES, talismans::TALISMANS, upgrade_materials::UPGRADE_MATERIALS,
        weapons::WEAPONS,
    },
    utils::is_version_dlc_compat,
};

#[derive(Clone, Copy)]
pub struct Item {
    pub id: u32,
    pub name: &'static str,
    pub stack_size: i32,
    pub max_storage: i32,
    pub category: Categories,
    pub weapon_type: Option<u8>,
    pub gem_mount_type: Option<u8>,
    pub upgrade_type: Option<u8>,
    pub event_id: Option<u32>,
    pub dlc: bool,
}

impl Item {
    const fn default() -> Self {
        Self {
            id: 0,
            name: "",
            stack_size: 1,
            max_storage: 1,
            category: Categories::Armor,
            weapon_type: None,
            gem_mount_type: None,
            upgrade_type: None,
            event_id: None,
            dlc: false,
        }
    }
}

static ITEMS: Lazy<Vec<Item>> = Lazy::new(|| {
    let mut items = Vec::new();
    items.extend(ARMOR);
    items.extend(ARROWS);
    items.extend(ASHES_OF_WAR);
    items.extend(BELL_BEARINGS);
    items.extend(CONSUMABLES);
    items.extend(COOKBOOKS);
    items.extend(CRAFTING_MATERIALS);
    items.extend(CRYSTAL_TEARS);
    items.extend(INCANTATIONS);
    items.extend(KEY_ITEMS);
    items.extend(POTS_AND_PERFUMES);
    items.extend(PRATTLING_PATE);
    items.extend(SORCERIES);
    items.extend(SPIRIT_ASHES);
    items.extend(TALISMANS);
    items.extend(UPGRADE_MATERIALS);
    items.extend(WEAPONS);
    items.sort_by(|a, b| a.name.cmp(b.name));
    items
});

static ITEMS_NO_DLC_VERSION: Lazy<Vec<Item>> =
    Lazy::new(|| ITEMS.iter().filter(|item| !item.dlc).cloned().collect());

static ITEMS_DLC_NOT_ACTIVE: Lazy<Vec<Item>> = Lazy::new(|| {
    ITEMS
        .iter()
        .filter(|item| !item.dlc || !item.requires_activated_dlc())
        .cloned()
        .collect()
});

pub fn items_array(dlc_active: bool) -> &'static [Item] {
    if dlc_active {
        &ITEMS
    } else if is_version_dlc_compat() {
        &ITEMS_DLC_NOT_ACTIVE
    } else {
        &ITEMS_NO_DLC_VERSION
    }
}

#[derive(Clone, Copy, PartialEq, ValueEnum)]
pub enum Categories {
    Armor,
    Arrows,
    AshesOfWar,
    BellBearings,
    Consumables,
    Cookbooks,
    CraftingMaterials,
    CrystalTears,
    Incantations,
    KeyItems,
    PotsAndPerfumes,
    PrattlingPate,
    Sorceries,
    SpiritAshes,
    Talismans,
    UpgradeMaterials,
    Weapons,
}

impl fmt::Display for Categories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Categories::Armor => "Armor",
            Categories::Arrows => "Arrows",
            Categories::AshesOfWar => "Ashes of War",
            Categories::BellBearings => "Bell Bearings",
            Categories::Consumables => "Consumables",
            Categories::Cookbooks => "Cookbooks",
            Categories::CraftingMaterials => "Crafting Materials",
            Categories::CrystalTears => "Crystal Tears",
            Categories::Incantations => "Incantations",
            Categories::KeyItems => "Key Items",
            Categories::PotsAndPerfumes => "Pots and Perfumes",
            Categories::PrattlingPate => "Prattling Pate",
            Categories::Sorceries => "Sorceries",
            Categories::SpiritAshes => "Spirit Ashes",
            Categories::Talismans => "Talismans",
            Categories::UpgradeMaterials => "Upgrade Materials",
            Categories::Weapons => "Weapons",
        };
        write!(f, "{}", name)
    }
}

impl Categories {
    pub const ARRAY: &[Categories] = &[
        Self::Armor,
        Self::Arrows,
        Self::AshesOfWar,
        Self::BellBearings,
        Self::Consumables,
        Self::Cookbooks,
        Self::CraftingMaterials,
        Self::CrystalTears,
        Self::Incantations,
        Self::KeyItems,
        Self::PotsAndPerfumes,
        Self::PrattlingPate,
        Self::Sorceries,
        Self::SpiritAshes,
        Self::Talismans,
        Self::UpgradeMaterials,
        Self::Weapons,
    ];
}
