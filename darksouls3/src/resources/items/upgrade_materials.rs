use super::Item;

impl Item {
    const fn default_upgrade_material() -> Self {
        Self {
            category: super::ItemCategory::UpgradeMaterials,
            stack_size: 99,
            max_storage: 600,
            ..Item::default()
        }
    }
}

pub static UPGRADE_MATERIALS: [Item; 26] = [
    Item {
        id: 0x400003e8,
        name: "Titanite Shard",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400003e9,
        name: "Large Titanite Shard",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400003ea,
        name: "Titanite Chunk",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400003eb,
        name: "Titanite Slab",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400003fc,
        name: "Titanite Scale",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x40000406,
        name: "Twinkling Titanite",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x4000044c,
        name: "Heavy Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x40000456,
        name: "Sharp Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x40000460,
        name: "Refined Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x4000046a,
        name: "Crystal Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x40000474,
        name: "Simple Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x4000047e,
        name: "Fire Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x40000488,
        name: "Chaos Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x40000492,
        name: "Lightning Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x4000049c,
        name: "Deep Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400004a6,
        name: "Dark Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400004b0,
        name: "Poison Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400004ba,
        name: "Blood Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400004c4,
        name: "Raw Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400004ce,
        name: "Blessed Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400004d8,
        name: "Hollow Gem",
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x40000837,
        name: "Farron Coal",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x40000839,
        name: "Giant's Coal",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x4000083a,
        name: "Profaned Coal",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x40000838,
        name: "Sage's Coal",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_upgrade_material()
    },
    Item {
        id: 0x400004e2,
        name: "Shriving Stone",
        ..Item::default_upgrade_material()
    },
];
