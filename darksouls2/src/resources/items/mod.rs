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

use {
    crate::resources::items::{
        armor::ARMOR,
        arrows::ARROWS,
        consumables::CONSUMABLES,
        gestures::GESTURES,
        key_items::KEY_ITEMS,
        rings::RINGS,
        spells::SPELLS,
        upgrade_materials::UPGRADE_MATERIALS,
        weapons::WEAPONS,
    },
    once_cell::sync::Lazy,
    std::fmt,
};

#[derive(Clone, Copy)]
pub struct Item {
    pub id:           i32,
    pub name:         &'static str,
    pub stack_size:   i32,
    pub category:     ItemCategory,
    pub infuse_id:    Option<i32>,
    pub max_upgrade:  Option<i32>,
    pub durability:   Option<i32>,
    pub scholar_only: bool,
}

impl Item {
    const fn default() -> Self {
        Self {
            id:           0,
            name:         "",
            stack_size:   0,
            category:     ItemCategory::Armor,
            infuse_id:    None,
            max_upgrade:  None,
            durability:   None,
            scholar_only: false,
        }
    }
}

pub static ITEMS: Lazy<Vec<Item>> = Lazy::new(|| {
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

#[derive(Clone, Copy)]
pub enum ItemCategory {
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

impl fmt::Display for ItemCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            ItemCategory::Armor => "Armor",
            ItemCategory::Arrows => "Arrows",
            ItemCategory::Consumables => "Consumables",
            ItemCategory::Gestures => "Gestures",
            ItemCategory::KeyItems => "Key Items",
            ItemCategory::Rings => "Rings",
            ItemCategory::Spells => "Spells",
            ItemCategory::UpgradeMaterials => "Upgrade Materials",
            ItemCategory::Weapons => "Weapons",
        };
        write!(f, "{}", name)
    }
}

impl ItemCategory {
    pub const ARRAY: &[ItemCategory] = &[
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
