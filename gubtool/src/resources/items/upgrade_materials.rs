use super::Item;

impl Item {
    const fn default_upgrade_materials() -> Self {
        Self {
            category: super::Categories::UpgradeMaterials,
            stack_size: 999,
            max_storage: 999,
            ..Item::default()
        }
    }
}

pub static UPGRADE_MATERIALS: [Item; 39] = [
    Item {
        id: 0x40002774,
        name: "Smithing Stone [1]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002775,
        name: "Smithing Stone [2]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002776,
        name: "Smithing Stone [3]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002777,
        name: "Smithing Stone [4]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002778,
        name: "Smithing Stone [5]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002779,
        name: "Smithing Stone [6]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x4000277A,
        name: "Smithing Stone [7]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x4000277B,
        name: "Smithing Stone [8]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x4000279C,
        name: "Ancient Dragon Smithing Stone",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027B0,
        name: "Somber Smithing Stone [1]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027B1,
        name: "Somber Smithing Stone [2]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027B2,
        name: "Somber Smithing Stone [3]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027B3,
        name: "Somber Smithing Stone [4]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027B4,
        name: "Somber Smithing Stone [5]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027B5,
        name: "Somber Smithing Stone [6]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027B6,
        name: "Somber Smithing Stone [7]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027B7,
        name: "Somber Smithing Stone [8]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027D8,
        name: "Somber Smithing Stone [9]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027B8,
        name: "Somber Ancient Dragon Smithing Stone",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A94,
        name: "Grave Glovewort [1]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A95,
        name: "Grave Glovewort [2]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A96,
        name: "Grave Glovewort [3]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A97,
        name: "Grave Glovewort [4]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A98,
        name: "Grave Glovewort [5]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A99,
        name: "Grave Glovewort [6]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A9A,
        name: "Grave Glovewort [7]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A9B,
        name: "Grave Glovewort [8]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A9C,
        name: "Grave Glovewort [9]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A9D,
        name: "Great Grave Glovewort",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A9E,
        name: "Ghost Glovewort [1]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002A9F,
        name: "Ghost Glovewort [2]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002AA0,
        name: "Ghost Glovewort [3]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002AA1,
        name: "Ghost Glovewort [4]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002AA2,
        name: "Ghost Glovewort [5]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002AA3,
        name: "Ghost Glovewort [6]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002AA4,
        name: "Ghost Glovewort [7]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002AA5,
        name: "Ghost Glovewort [8]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002AA6,
        name: "Ghost Glovewort [9]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002AA7,
        name: "Great Ghost Glovewort",
        ..Item::default_upgrade_materials()
    },
];
