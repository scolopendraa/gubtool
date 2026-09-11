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
    pub name:         &'static str,
    pub category:     ItemCategory,
    pub id:           u32,
    pub stack_size:   u32,
    pub max_storage:  u32,
    pub upgrade_type: Option<u32>,
    pub infusable:    bool,
    pub dlc:          u8,
}

impl Item {
    const fn default() -> Self {
        Self {
            name:         "",
            category:     ItemCategory::Armor,
            id:           0,
            stack_size:   1,
            max_storage:  1,
            upgrade_type: None,
            infusable:    false,
            dlc:          0,
        }
    }
}

pub static ITEMS: Lazy<Vec<Item>> = Lazy::new(|| {
    let mut items: Vec<Item> = Vec::new();
    items.extend(ARMOR);
    items.extend(ARROWS);
    items.extend(CONSUMABLES);
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
    pub const ARRAY: [ItemCategory; 9] = [
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
