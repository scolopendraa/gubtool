use super::Item;

impl Item {
    const fn default_key_item() -> Self {
        Self {
            category: super::ItemCategory::KeyItems,
            stack_size: 1,
            max_storage: 1,
            ..Item::default()
        }
    }
}

pub static KEY_ITEMS: [Item; 79] = [
    Item {
        id: 0x400000be,
        name: "Ashen Estus Flask",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000845,
        name: "Basin of Vows",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000173,
        name: "Binoculars",
        max_storage: 600,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000086e,
        name: "Blood of the Dark Soul",
        dlc: 2,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000849,
        name: "Braille Divine Sunlight Tome",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000847,
        name: "Braille Divine Tome of Carim",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000848,
        name: "Braille Divine Tome of Lothric",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000086a,
        name: "Captain's Ashes",
        dlc: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000850,
        name: "Carthus Pyromancy Tome",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007da,
        name: "Cell Key",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000869,
        name: "Champion's Bones",
        stack_size: 99,
        dlc: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000013b,
        name: "Church Guardian Shiv",
        dlc: 2,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000084e,
        name: "Cinders of a Lord",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000084d,
        name: "Cinders of a Lord",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000084c,
        name: "Cinders of a Lord",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000084b,
        name: "Cinders of a Lord",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000859,
        name: "Coiled Sword",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000015f,
        name: "Coiled Sword Fragment",
        max_storage: 600,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000086b,
        name: "Contraption Key",
        dlc: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000841,
        name: "Cornyx's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000856,
        name: "Crystal Scroll",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000860,
        name: "Deep Braille Divine Tome",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000028b,
        name: "Divine Spear Fragment",
        dlc: 2,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000867,
        name: "Dragon Chaser's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000179,
        name: "Dragon Head Stone",
        max_storage: 600,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000017a,
        name: "Dragon Torso Stone",
        max_storage: 600,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000083c,
        name: "Dreamchaser's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000181,
        name: "Dried Finger",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007db,
        name: "Dungeon Ground Floor Key",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000868,
        name: "Easterner's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000085d,
        name: "Estus Shard",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000862,
        name: "Excrement-covered Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000085a,
        name: "Eyes of a Fire Keeper",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000186,
        name: "Fire Keeper Soul",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000085c,
        name: "Golden Scroll",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007de,
        name: "Grand Archives Key",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007d9,
        name: "Grave Key",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000853,
        name: "Grave Warden Pyromancy Tome",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000083e,
        name: "Grave Warden's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000084f,
        name: "Great Swamp Pyromancy Tome",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000083f,
        name: "Greirat's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000085e,
        name: "Hawkwood's Swordgrass",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000208,
        name: "Hello Carving",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000020c,
        name: "Help me! Carving",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000865,
        name: "Hollow's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000020b,
        name: "I'm sorry Carving",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000843,
        name: "Irina's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000851,
        name: "Izalith Pyromancy Tome",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007d7,
        name: "Jailbreaker's Key",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007d8,
        name: "Jailer's Key Ring",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000842,
        name: "Karla's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007d1,
        name: "Lift Chamber Key",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000855,
        name: "Logan's Scroll",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000861,
        name: "Londor Braille Divine Tome",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000846,
        name: "Loretta's Bone",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000083b,
        name: "Mortician's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007dc,
        name: "Old Cell Key",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000086d,
        name: "Old Woman's Ashes",
        dlc: 2,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000840,
        name: "Orbeck's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000083d,
        name: "Paladin's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000866,
        name: "Patches' Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000863,
        name: "Prisoner Chief's Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000852,
        name: "Quelana Pyromancy Tome",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000028a,
        name: "Ritual Spear Fragment",
        dlc: 2,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000006c,
        name: "Roster of Knights",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000854,
        name: "Sage's Scroll",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007d5,
        name: "Small Doll",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000086c,
        name: "Small Envoy Banner",
        dlc: 2,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000836,
        name: "Small Lothric Banner",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000085b,
        name: "Sword of Avowal",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000209,
        name: "Thank you Carving",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007dd,
        name: "Tower Key",
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007df,
        name: "Tower Key",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000857,
        name: "Transposing Kiln",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000085f,
        name: "Undead Bone Shard",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000020a,
        name: "Very good! Carving",
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000864,
        name: "Xanthous Ashes",
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000086f,
        name: "Young Grass Dew",
        dlc: 2,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000844,
        name: "Yuria's Ashes",
        ..Item::default_key_item()
    },
];
