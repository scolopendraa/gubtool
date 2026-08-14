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
        id: 0x4000277a,
        name: "Smithing Stone [7]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x4000277b,
        name: "Smithing Stone [8]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x4000279c,
        name: "Ancient Dragon Smithing Stone",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027b0,
        name: "Somber Smithing Stone [1]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027b1,
        name: "Somber Smithing Stone [2]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027b2,
        name: "Somber Smithing Stone [3]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027b3,
        name: "Somber Smithing Stone [4]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027b4,
        name: "Somber Smithing Stone [5]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027b5,
        name: "Somber Smithing Stone [6]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027b6,
        name: "Somber Smithing Stone [7]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027b7,
        name: "Somber Smithing Stone [8]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027d8,
        name: "Somber Smithing Stone [9]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x400027b8,
        name: "Somber Ancient Dragon Smithing Stone",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a94,
        name: "Grave Glovewort [1]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a95,
        name: "Grave Glovewort [2]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a96,
        name: "Grave Glovewort [3]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a97,
        name: "Grave Glovewort [4]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a98,
        name: "Grave Glovewort [5]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a99,
        name: "Grave Glovewort [6]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a9a,
        name: "Grave Glovewort [7]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a9b,
        name: "Grave Glovewort [8]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a9c,
        name: "Grave Glovewort [9]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a9d,
        name: "Great Grave Glovewort",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a9e,
        name: "Ghost Glovewort [1]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002a9f,
        name: "Ghost Glovewort [2]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002aa0,
        name: "Ghost Glovewort [3]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002aa1,
        name: "Ghost Glovewort [4]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002aa2,
        name: "Ghost Glovewort [5]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002aa3,
        name: "Ghost Glovewort [6]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002aa4,
        name: "Ghost Glovewort [7]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002aa5,
        name: "Ghost Glovewort [8]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002aa6,
        name: "Ghost Glovewort [9]",
        ..Item::default_upgrade_materials()
    },
    Item {
        id: 0x40002aa7,
        name: "Great Ghost Glovewort",
        ..Item::default_upgrade_materials()
    },
];
