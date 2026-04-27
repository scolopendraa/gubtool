use super::Item;

impl Item {
    const fn default_bell_bearing() -> Self {
        Self {
            category: super::Categories::BellBearings,
            ..Item::default()
        }
    }
}

pub static BELL_BEARINGS: [Item; 62] = [
    Item {
        id: 0x400022E9,
        name: "Abandoned Merchant's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022D4,
        name: "Bernahl's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022EE,
        name: "Blackguard's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022F1,
        name: "Bone Peddler's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022EF,
        name: "Corhyn's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022D3,
        name: "D's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x40002303,
        name: "Ghost-Glovewort Picker's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x40002304,
        name: "Ghost-Glovewort Picker's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x40002305,
        name: "Ghost-Glovewort Picker's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x40002300,
        name: "Glovewort Picker's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x40002301,
        name: "Glovewort Picker's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x40002302,
        name: "Glovewort Picker's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022D6,
        name: "Gostoc's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022F0,
        name: "Gowry's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022F4,
        name: "Gravity Stone Peddler's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401EA749,
        name: "Greasemonger's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401EA746,
        name: "Herbalist's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022E1,
        name: "Hermit Merchant's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022E8,
        name: "Hermit Merchant's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022EA,
        name: "Hermit Merchant's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401EA74B,
        name: "Igon's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022EC,
        name: "Iji's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022EB,
        name: "Imprisoned Merchant's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022DE,
        name: "Isolated Merchant's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022DF,
        name: "Isolated Merchant's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022E7,
        name: "Isolated Merchant's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022D8,
        name: "Kalé's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022F2,
        name: "Meat Peddler's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022F3,
        name: "Medicine Peddler's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022D5,
        name: "Miriel's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401EA74A,
        name: "Moldmonger's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401EA744,
        name: "Moore's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401EA747,
        name: "Mushroom-Seller's Bell Bearing [1]",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401EA748,
        name: "Mushroom-Seller's Bell Bearing [2]",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022E5,
        name: "Nomadic Merchant's Bell Bearing [10]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022D9,
        name: "Nomadic Merchant's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022DA,
        name: "Nomadic Merchant's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022DB,
        name: "Nomadic Merchant's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022DC,
        name: "Nomadic Merchant's Bell Bearing [4]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022DD,
        name: "Nomadic Merchant's Bell Bearing [5]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022E0,
        name: "Nomadic Merchant's Bell Bearing [6]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022E2,
        name: "Nomadic Merchant's Bell Bearing [7]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022E3,
        name: "Nomadic Merchant's Bell Bearing [8]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022E4,
        name: "Nomadic Merchant's Bell Bearing [9]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022D0,
        name: "Patches' Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022CE,
        name: "Pidia's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022ED,
        name: "Rogier's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022D1,
        name: "Sellen's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022CF,
        name: "Seluvis's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022F7,
        name: "Smithing-Stone Miner's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022F8,
        name: "Smithing-Stone Miner's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022F9,
        name: "Smithing-Stone Miner's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022FA,
        name: "Smithing-Stone Miner's Bell Bearing [4]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022FB,
        name: "Somberstone Miner's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022FC,
        name: "Somberstone Miner's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022FD,
        name: "Somberstone Miner's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022FE,
        name: "Somberstone Miner's Bell Bearing [4]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022FF,
        name: "Somberstone Miner's Bell Bearing [5]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401EA74C,
        name: "Spellmachinist's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401EA74D,
        name: "String-Seller's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022D7,
        name: "Thops's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401EA745,
        name: "Ymir's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
];
