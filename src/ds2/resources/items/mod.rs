use std::fmt;

use once_cell::sync::Lazy;

use crate::ds2::{
    resources::items::{
        armor::ARMOR, arrows::ARROWS, consumables::CONSUMABLES, gestures::GESTURES,
        key_items::KEY_ITEMS, rings::RINGS, spells::SPELLS, upgrade_materials::UPGRADE_MATERIALS,
        weapons::WEAPONS,
    },
    utils::is_scholar,
};

pub mod armor;
pub mod arrows;
pub mod consumables;
pub mod gestures;
pub mod infusions;
pub mod key_items;
pub mod rings;
pub mod spells;
pub mod upgrade_materials;
pub mod weapons;

#[derive(Clone, Copy)]
pub struct Item {
    pub id: i32,
    pub name: &'static str,
    pub stack_size: i32,
    pub category: Categories,
    pub infuse_id: Option<i32>,
    pub max_upgrade: Option<i32>,
    pub durability: Option<i32>,
    pub scholar_only: bool,
}

impl Item {
    const fn default() -> Self {
        Self {
            id: 0,
            name: "",
            stack_size: 0,
            category: Categories::Armor,
            infuse_id: None,
            max_upgrade: None,
            durability: None,
            scholar_only: false,
        }
    }
}

static ITEMS: Lazy<Vec<Item>> = Lazy::new(|| {
    let mut items: Vec<Item> = Vec::new();
    items.extend(ARMOR);
    items.extend(ARROWS);
    items.extend(CONSUMABLES);
    items.extend(GESTURES);
    items.extend(KEY_ITEMS);
    items.extend(RINGS);
    items.extend(SPELLS);
    items.extend(UPGRADE_MATERIALS);
    items.extend(WEAPONS);
    items.sort_by(|a, b| a.name.cmp(b.name));
    items
});

static ITEMS_NO_SCHOLAR: Lazy<Vec<Item>> = Lazy::new(|| {
    ITEMS
        .iter()
        .filter(|item| !item.scholar_only)
        .cloned()
        .collect()
});

pub fn items_array() -> &'static [Item] {
    if is_scholar() {
        &ITEMS
    } else {
        &ITEMS_NO_SCHOLAR
    }
}

#[derive(Clone, Copy)]
pub enum Categories {
    Armor,
    Arrows,
    Consumables,
    Gestures,
    KeyItems,
    Rings,
    Spells,
    UpgradeMaterials,
    Weapons,
}

impl fmt::Display for Categories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Categories::Armor => "Armor",
            Categories::Arrows => "Arrows",
            Categories::Consumables => "Consumables",
            Categories::Gestures => "Gestures",
            Categories::KeyItems => "Key Items",
            Categories::Rings => "Rings",
            Categories::Spells => "Spells",
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
        Self::Consumables,
        Self::Gestures,
        Self::KeyItems,
        Self::Rings,
        Self::Spells,
        Self::UpgradeMaterials,
        Self::Weapons,
    ];
}
