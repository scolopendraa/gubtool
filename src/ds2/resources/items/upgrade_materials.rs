use super::Item;

impl Item {
    const fn default_upgrade_material() -> Self {
        Self {
            category: super::Categories::UpgradeMaterials,
            stack_size: 99,
            ..Item::default()
        }
    }
}

pub static UPGRADE_MATERIALS: &[Item; 16] = &[
    Item {
        id: 60970000,
        name: "Titanite Shard",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 60975000,
        name: "Large Titanite Shard",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 60980000,
        name: "Titanite Chunk",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 60990000,
        name: "Titanite Slab",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61000000,
        name: "Twinkling Titanite",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61030000,
        name: "Petrified Dragon Bone",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61060000,
        name: "Faintstone",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61070000,
        name: "Boltstone",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61080000,
        name: "Firedrake Stone",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61090000,
        name: "Darknight Stone",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61100000,
        name: "Poison Stone",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61110000,
        name: "Bleed Stone",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61130000,
        name: "Raw Stone",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61140000,
        name: "Magic Stone",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61150000,
        name: "Old Mundane Stone",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 61160000,
        name: "Palestone",
        ..Item::default_upgrade_material()
    },
];