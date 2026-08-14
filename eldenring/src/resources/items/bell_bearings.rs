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
        id: 0x400022e9,
        name: "Abandoned Merchant's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022d4,
        name: "Bernahl's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022ee,
        name: "Blackguard's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022f1,
        name: "Bone Peddler's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022ef,
        name: "Corhyn's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022d3,
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
        id: 0x400022d6,
        name: "Gostoc's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022f0,
        name: "Gowry's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022f4,
        name: "Gravity Stone Peddler's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401ea749,
        name: "Greasemonger's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401ea746,
        name: "Herbalist's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022e1,
        name: "Hermit Merchant's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022e8,
        name: "Hermit Merchant's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022ea,
        name: "Hermit Merchant's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401ea74b,
        name: "Igon's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022ec,
        name: "Iji's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022eb,
        name: "Imprisoned Merchant's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022de,
        name: "Isolated Merchant's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022df,
        name: "Isolated Merchant's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022e7,
        name: "Isolated Merchant's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022d8,
        name: "Kalé's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022f2,
        name: "Meat Peddler's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022f3,
        name: "Medicine Peddler's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022d5,
        name: "Miriel's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401ea74a,
        name: "Moldmonger's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401ea744,
        name: "Moore's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401ea747,
        name: "Mushroom-Seller's Bell Bearing [1]",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401ea748,
        name: "Mushroom-Seller's Bell Bearing [2]",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022e5,
        name: "Nomadic Merchant's Bell Bearing [10]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022d9,
        name: "Nomadic Merchant's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022da,
        name: "Nomadic Merchant's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022db,
        name: "Nomadic Merchant's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022dc,
        name: "Nomadic Merchant's Bell Bearing [4]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022dd,
        name: "Nomadic Merchant's Bell Bearing [5]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022e0,
        name: "Nomadic Merchant's Bell Bearing [6]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022e2,
        name: "Nomadic Merchant's Bell Bearing [7]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022e3,
        name: "Nomadic Merchant's Bell Bearing [8]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022e4,
        name: "Nomadic Merchant's Bell Bearing [9]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022d0,
        name: "Patches' Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022ce,
        name: "Pidia's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022ed,
        name: "Rogier's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022d1,
        name: "Sellen's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022cf,
        name: "Seluvis's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022f7,
        name: "Smithing-Stone Miner's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022f8,
        name: "Smithing-Stone Miner's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022f9,
        name: "Smithing-Stone Miner's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022fa,
        name: "Smithing-Stone Miner's Bell Bearing [4]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022fb,
        name: "Somberstone Miner's Bell Bearing [1]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022fc,
        name: "Somberstone Miner's Bell Bearing [2]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022fd,
        name: "Somberstone Miner's Bell Bearing [3]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022fe,
        name: "Somberstone Miner's Bell Bearing [4]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022ff,
        name: "Somberstone Miner's Bell Bearing [5]",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401ea74c,
        name: "Spellmachinist's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401ea74d,
        name: "String-Seller's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x400022d7,
        name: "Thops's Bell Bearing",
        ..Item::default_bell_bearing()
    },
    Item {
        id: 0x401ea745,
        name: "Ymir's Bell Bearing",
        dlc: true,
        ..Item::default_bell_bearing()
    },
];
