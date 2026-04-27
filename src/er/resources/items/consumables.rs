use super::Item;

impl Item {
    const fn default_consumables() -> Self {
        Self {
            category: super::Categories::Consumables,
            ..Item::default()
        }
    }
}

pub static CONSUMABLES: [Item; 161] = [
    Item {
        id: 0x401E8CB4,
        name: "Ancient Dragon's Blessing",
        stack_size: 1,
        max_storage: 0,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000D20,
        name: "Baldachin's Blessing",
        stack_size: 1,
        max_storage: 0,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000D16,
        name: "Bewitching Branch",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8804,
        name: "Blessing of Marika",
        stack_size: 1,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005A0,
        name: "Blood Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000334,
        name: "Boiled Crab",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000033E,
        name: "Boiled Prawn",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E9038,
        name: "Bondstone",
        stack_size: 1,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400006AE,
        name: "Bone Dart",
        stack_size: 40,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E9007,
        name: "Broken Rune",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E90CE,
        name: "Call of Tibia",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E9196,
        name: "Charming Branch",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400003C0,
        name: "Clarifying Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000046A,
        name: "Clarifying Cured Meat",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000532,
        name: "Clarifying White Cured Meat",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400006CC,
        name: "Crystal Dart",
        stack_size: 40,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000BD6,
        name: "Cuckoo Glintstone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000474,
        name: "Dappled Cured Meat",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000053C,
        name: "Dappled White Cured Meat",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E88CC,
        name: "Dragon Communion Flesh",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8A48,
        name: "Dragon Communion Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8B2E,
        name: "Dragon Communion Harpoon",
        stack_size: 5,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8A02,
        name: "Dragonbolt Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E88D1,
        name: "Dragonscale Flesh",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005C8,
        name: "Dragonwound Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000604,
        name: "Drawstring Blood Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8A66,
        name: "Drawstring Dragonbolt Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8A8E,
        name: "Drawstring Eternal Sleep Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005DC,
        name: "Drawstring Fire Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8A7A,
        name: "Drawstring Golden Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005FA,
        name: "Drawstring Holy Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005E6,
        name: "Drawstring Lightning Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005F0,
        name: "Drawstring Magic Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8A5C,
        name: "Drawstring Messmerfire Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000618,
        name: "Drawstring Poison Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000636,
        name: "Drawstring Rot Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8A70,
        name: "Drawstring Royal Magic Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000060E,
        name: "Drawstring Soporific Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8A2A,
        name: "Eternal Sleep Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400004BA,
        name: "Exalted Flesh",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000726,
        name: "Explosive Stone",
        stack_size: 20,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000727,
        name: "Explosive Stone Clump",
        stack_size: 20,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400006D6,
        name: "Fan Daggers",
        stack_size: 40,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8AC0,
        name: "Festive Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E88D6,
        name: "Fingerprint Nostrum",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E90EC,
        name: "Fire Coil",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000578,
        name: "Fire Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E90F6,
        name: "Fire Spritestone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000488,
        name: "Fireproof Dried Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E88EA,
        name: "Fireproof Pickled Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005BE,
        name: "Freezing Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000CEF,
        name: "Frenzyflame Stone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000032C,
        name: "Frozen Raisin",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000096,
        name: "Furlcalling Finger Remedy",
        stack_size: 999,
        max_storage: 0,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40002710,
        name: "Glass Shard",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E9100,
        name: "Glinting Nail",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000BEA,
        name: "Glintstone Scrap",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400007EE,
        name: "Glowstone",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8A16,
        name: "Golden Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E894E,
        name: "Golden Horn Tender",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B5D,
        name: "Golden Rune [10]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B5E,
        name: "Golden Rune [11]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B5F,
        name: "Golden Rune [12]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B60,
        name: "Golden Rune [13]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B54,
        name: "Golden Rune [1]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B55,
        name: "Golden Rune [2]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B56,
        name: "Golden Rune [3]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B57,
        name: "Golden Rune [4]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B58,
        name: "Golden Rune [5]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B59,
        name: "Golden Rune [6]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B5A,
        name: "Golden Rune [7]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B5B,
        name: "Golden Rune [8]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B5C,
        name: "Golden Rune [9]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E90E2,
        name: "Golden Vow",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400004B0,
        name: "Gold-Pickled Fowl Foot",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8931,
        name: "Gourmet Scorpion Stew",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8931,
        name: "Gourmet Scorpion Stew",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000802,
        name: "Grace Mimic",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000BFE,
        name: "Gravity Stone Chunk",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000BF4,
        name: "Gravity Stone Fan",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B62,
        name: "Hero's Rune [1]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B63,
        name: "Hero's Rune [2]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B64,
        name: "Hero's Rune [3]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B65,
        name: "Hero's Rune [4]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B66,
        name: "Hero's Rune [5]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000596,
        name: "Holy Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000049C,
        name: "Holyproof Dried Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E88FE,
        name: "Holyproof Pickled Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E90BA,
        name: "Horned Bairn",
        stack_size: 1,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000456,
        name: "Immunizing Cured Meat",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000051E,
        name: "Immunizing White Cured Meat",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8B24,
        name: "Innard Meat",
        stack_size: 40,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000460,
        name: "Invigorating Cured Meat",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000528,
        name: "Invigorating White Cured Meat",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8CC8,
        name: "Iris of Grace",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8CD2,
        name: "Iris of Occultation",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400006C2,
        name: "Kukri",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000BAE,
        name: "Lands Between Rune",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000BEB,
        name: "Large Glintstone Scrap",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E9006,
        name: "Leda's Rune",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000582,
        name: "Lightning Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000492,
        name: "Lightningproof Dried Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E88F4,
        name: "Lightningproof Pickled Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B67,
        name: "Lord's Rune",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E880E,
        name: "Lulling Branch",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000058C,
        name: "Magic Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E9010,
        name: "Marika's Rune",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E89F8,
        name: "Messmerfire Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000384,
        name: "Neutralizing Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000B61,
        name: "Numen's Rune",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8908,
        name: "Opaline Pickled Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E90C4,
        name: "Perfumed Oil of Ranah",
        stack_size: 1,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000044C,
        name: "Pickled Turtle Neck",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005B4,
        name: "Poison Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400006B8,
        name: "Poisonbone Dart",
        stack_size: 40,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000730,
        name: "Poisoned Stone",
        stack_size: 20,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000731,
        name: "Poisoned Stone Clump",
        stack_size: 20,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8C50,
        name: "Polter Stone",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400003AC,
        name: "Preserving Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000D21,
        name: "Radiant Baldachin's Blessing",
        stack_size: 1,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400007E4,
        name: "Rainbow Stone",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400004D3,
        name: "Raw Meat Dumpling",
        stack_size: 3,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400003B6,
        name: "Rejuvenating Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005D2,
        name: "Rot Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000032A,
        name: "Rowa Raisin",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8A0C,
        name: "Royal Magic Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400000BE,
        name: "Rune Arc",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E900F,
        name: "Rune of an Unsung Hero",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E893A,
        name: "Sacred Bloody Flesh",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8930,
        name: "Scorpion Stew",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8930,
        name: "Scorpion Stew",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000CF8,
        name: "Scriptstone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E9008,
        name: "Shadow Realm Rune [1]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E9009,
        name: "Shadow Realm Rune [2]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E900A,
        name: "Shadow Realm Rune [3]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E900B,
        name: "Shadow Realm Rune [4]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E900C,
        name: "Shadow Realm Rune [5]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E900D,
        name: "Shadow Realm Rune [6]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E900E,
        name: "Shadow Realm Rune [7]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000069A,
        name: "Shield Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8944,
        name: "Silver Horn Tender",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400004A6,
        name: "Silver-Pickled Fowl Foot",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000848,
        name: "Soap",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000834,
        name: "Soft Cotton",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005AA,
        name: "Soporific Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000047E,
        name: "Spellproof Dried Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E88E0,
        name: "Spellproof Pickled Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E87A0,
        name: "Spirit Raisin",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E9039,
        name: "Spritestone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000038E,
        name: "Stanching Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000050A,
        name: "Starlight Shards",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400003A2,
        name: "Stimulating Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40001F40,
        name: "Stonesword Key",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E916E,
        name: "Sunwarmth Stone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E90D8,
        name: "Surging Frenzied Flame",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000032B,
        name: "Sweet Raisin",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000398,
        name: "Thawfrost Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8CDC,
        name: "Thiollier's Concoction",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400006A4,
        name: "Throwing Dagger",
        stack_size: 40,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000CEE,
        name: "Warming Stone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401E8912,
        name: "Well-Pickled Turtle Neck",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
];
