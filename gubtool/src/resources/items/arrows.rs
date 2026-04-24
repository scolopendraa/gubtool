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
        id: 0x02FAF080,
        name: "Arrows",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FB1790,
        name: "Fire Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FB3EA0,
        name: "Serpent Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FB65B0,
        name: "Bone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FB8CC0,
        name: "St. Trina's Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FBDAE0,
        name: "Shattershard Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FC2900,
        name: "Rainbow Stone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FC5010,
        name: "Golden Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FC7720,
        name: "Dwelling Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FC9E30,
        name: "Bone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FCEC50,
        name: "Firebone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FD1360,
        name: "Firebone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FD3A70,
        name: "Poisonbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FD6180,
        name: "Poisonbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FD8890,
        name: "Sleepbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FDAFA0,
        name: "Sleepbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FDD6B0,
        name: "Stormwing Bone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FDFDC0,
        name: "Lightningbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FE24D0,
        name: "Lightningbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FE4BE0,
        name: "Rainbow Stone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FE72F0,
        name: "Shattershard Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FE9A00,
        name: "Spiritflame Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FEE820,
        name: "Magicbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FF0F30,
        name: "Magicbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FF3640,
        name: "Haligbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FF5D50,
        name: "Haligbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FF8460,
        name: "Bloodbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FFAB70,
        name: "Bloodbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FFD280,
        name: "Coldbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x02FFF990,
        name: "Coldbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x030020A0,
        name: "Rotbone Arrow (Fletched)",
        ..Item::default_arrow()
    },
    Item {
        id: 0x030047B0,
        name: "Rotbone Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 0x030A32C0,
        name: "Great Arrow",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030A59D0,
        name: "Golem's Great Arrow",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030A80E0,
        name: "Golden Great Arrow",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030AA7F0,
        name: "Golem's Magic Arrow",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030ACF00,
        name: "Radahn's Spear",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030AF610,
        name: "Bone Great Arrow (Fletched)",
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030B1D20,
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
        id: 0x03199C10,
        name: "Lightning Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x0319C320,
        name: "Perfumer's Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x0319EA30,
        name: "Black-Key Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031A1140,
        name: "Burred Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031A3850,
        name: "Meteor Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031A5F60,
        name: "Explosive Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031A8670,
        name: "Golden Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031AAD80,
        name: "Lordsworn's Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031AD490,
        name: "Bone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031AFBA0,
        name: "Firebone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031B22B0,
        name: "Lightningbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031B49C0,
        name: "Magicbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031B70D0,
        name: "Haligbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031B97E0,
        name: "Poisonbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031BBEF0,
        name: "Bloodbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031BE600,
        name: "Coldbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031C0D10,
        name: "Rotbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031C3420,
        name: "Sleepbone Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x031C5B30,
        name: "Flaming Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 0x0328B740,
        name: "Ballista Bolt",
        stack_size: 20,
        ..Item::default_arrow()
    },
    Item {
        id: 0x0328DE50,
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
        id: 0x03292C70,
        name: "Bone Ballista Bolt",
        stack_size: 20,
        ..Item::default_arrow()
    },
    Item {
        id: 0x03032DE0,
        name: "Piquebone Arrow (Fletched)",
        dlc: true,
        ..Item::default_arrow()
    },
    Item {
        id: 0x030354F0,
        name: "Piquebone Arrow",
        dlc: true,
        ..Item::default_arrow()
    },
    Item {
        id: 0x0311D3E0,
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
