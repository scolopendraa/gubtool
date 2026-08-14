use super::Item;

impl Item {
    const fn default_arrow() -> Self {
        Self {
            category: super::Categories::Arrows,
            stack_size: 99,
            max_storage: 600,
            ..Item::default()
        }
    }
}

pub static ARROWS: [Item; 68] = [
    Item {
        id: 0x02faf080,
        name: "Arrows",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fb1790,
        name: "Fire Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fb3ea0,
        name: "Serpent Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fb65b0,
        name: "Bone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fb8cc0,
        name: "St. Trina's Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fbdae0,
        name: "Shattershard Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fc2900,
        name: "Rainbow Stone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fc5010,
        name: "Golden Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fc7720,
        name: "Dwelling Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fc9e30,
        name: "Bone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fcec50,
        name: "Firebone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fd1360,
        name: "Firebone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fd3a70,
        name: "Poisonbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fd6180,
        name: "Poisonbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fd8890,
        name: "Sleepbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fdafa0,
        name: "Sleepbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fdd6b0,
        name: "Stormwing Bone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fdfdc0,
        name: "Lightningbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fe24d0,
        name: "Lightningbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fe4be0,
        name: "Rainbow Stone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fe72f0,
        name: "Shattershard Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fe9a00,
        name: "Spiritflame Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fee820,
        name: "Magicbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02ff0f30,
        name: "Magicbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02ff3640,
        name: "Haligbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02ff5d50,
        name: "Haligbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02ff8460,
        name: "Bloodbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02ffab70,
        name: "Bloodbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02ffd280,
        name: "Coldbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02fff990,
        name: "Coldbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x030020a0,
        name: "Rotbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x030047b0,
        name: "Rotbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x030a32c0,
        name: "Great Arrow",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030a59d0,
        name: "Golem's Great Arrow",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030a80e0,
        name: "Golden Great Arrow",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030aa7f0,
        name: "Golem's Magic Arrow",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030acf00,
        name: "Radahn's Spear",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030af610,
        name: "Bone Great Arrow (Fletched)",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030b1d20,
        name: "Bone Great Arrow",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x03197500,
        name: "Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x03199c10,
        name: "Lightning Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x0319c320,
        name: "Perfumer's Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x0319ea30,
        name: "Black-Key Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031a1140,
        name: "Burred Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031a3850,
        name: "Meteor Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031a5f60,
        name: "Explosive Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031a8670,
        name: "Golden Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031aad80,
        name: "Lordsworn's Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031ad490,
        name: "Bone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031afba0,
        name: "Firebone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031b22b0,
        name: "Lightningbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031b49c0,
        name: "Magicbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031b70d0,
        name: "Haligbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031b97e0,
        name: "Poisonbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031bbef0,
        name: "Bloodbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031be600,
        name: "Coldbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031c0d10,
        name: "Rotbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031c3420,
        name: "Sleepbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031c5b30,
        name: "Flaming Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x0328b740,
        name: "Ballista Bolt",
        stack_size: 20,
        ..Item::default_arrow()
    },
    Item {
        id: 0x0328de50,
        name: "Lightning Greatbolt",
        stack_size: 20,
        ..Item::default_arrow()
    },
    Item {
        id: 0x03290560,
        name: "Explosive Greatbolt",
        stack_size: 20,
        ..Item::default_arrow()
    },
    Item {
        id: 0x03292c70,
        name: "Bone Ballista Bolt",
        stack_size: 20,
        ..Item::default_arrow()
    },
    Item {
        id: 0x03032de0,
        name: "Piquebone Arrow (Fletched)",
        dlc: true,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030354f0,
        name: "Piquebone Arrow",
        dlc: true,
        ..Item::default_arrow()
    },
    Item {
        id: 0x0311d3e0,
        name: "Igon's Harpoon",
        stack_size: 30,
        dlc: true,
        ..Item::default_arrow()
    },
    Item {
        id: 0x03216440,
        name: "Piquebone Bolt",
        dlc: true,
        ..Item::default_arrow()
    },
    Item {
        id: 0x03305860,
        name: "Rabbath's Greatbolt",
        stack_size: 20,
        dlc: true,
        ..Item::default_arrow()
    },
];
