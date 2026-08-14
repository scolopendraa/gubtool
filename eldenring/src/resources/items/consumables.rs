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
        id: 0x401e8cb4,
        name: "Ancient Dragon's Blessing",
        stack_size: 1,
        max_storage: 0,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000d20,
        name: "Baldachin's Blessing",
        stack_size: 1,
        max_storage: 0,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000d16,
        name: "Bewitching Branch",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8804,
        name: "Blessing of Marika",
        stack_size: 1,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005a0,
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
        id: 0x4000033e,
        name: "Boiled Prawn",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e9038,
        name: "Bondstone",
        stack_size: 1,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400006ae,
        name: "Bone Dart",
        stack_size: 40,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e9007,
        name: "Broken Rune",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e90ce,
        name: "Call of Tibia",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e9196,
        name: "Charming Branch",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400003c0,
        name: "Clarifying Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000046a,
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
        id: 0x400006cc,
        name: "Crystal Dart",
        stack_size: 40,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000bd6,
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
        id: 0x4000053c,
        name: "Dappled White Cured Meat",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e88cc,
        name: "Dragon Communion Flesh",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8a48,
        name: "Dragon Communion Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8b2e,
        name: "Dragon Communion Harpoon",
        stack_size: 5,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8a02,
        name: "Dragonbolt Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e88d1,
        name: "Dragonscale Flesh",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005c8,
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
        id: 0x401e8a66,
        name: "Drawstring Dragonbolt Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8a8e,
        name: "Drawstring Eternal Sleep Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005dc,
        name: "Drawstring Fire Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8a7a,
        name: "Drawstring Golden Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005fa,
        name: "Drawstring Holy Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005e6,
        name: "Drawstring Lightning Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005f0,
        name: "Drawstring Magic Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8a5c,
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
        id: 0x401e8a70,
        name: "Drawstring Royal Magic Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000060e,
        name: "Drawstring Soporific Grease",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8a2a,
        name: "Eternal Sleep Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400004ba,
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
        id: 0x400006d6,
        name: "Fan Daggers",
        stack_size: 40,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8ac0,
        name: "Festive Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e88d6,
        name: "Fingerprint Nostrum",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e90ec,
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
        id: 0x401e90f6,
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
        id: 0x401e88ea,
        name: "Fireproof Pickled Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005be,
        name: "Freezing Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000cef,
        name: "Frenzyflame Stone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000032c,
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
        id: 0x401e9100,
        name: "Glinting Nail",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000bea,
        name: "Glintstone Scrap",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400007ee,
        name: "Glowstone",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8a16,
        name: "Golden Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e894e,
        name: "Golden Horn Tender",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b5d,
        name: "Golden Rune [10]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b5e,
        name: "Golden Rune [11]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b5f,
        name: "Golden Rune [12]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b60,
        name: "Golden Rune [13]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b54,
        name: "Golden Rune [1]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b55,
        name: "Golden Rune [2]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b56,
        name: "Golden Rune [3]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b57,
        name: "Golden Rune [4]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b58,
        name: "Golden Rune [5]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b59,
        name: "Golden Rune [6]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b5a,
        name: "Golden Rune [7]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b5b,
        name: "Golden Rune [8]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b5c,
        name: "Golden Rune [9]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e90e2,
        name: "Golden Vow",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400004b0,
        name: "Gold-Pickled Fowl Foot",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8931,
        name: "Gourmet Scorpion Stew",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8931,
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
        id: 0x40000bfe,
        name: "Gravity Stone Chunk",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000bf4,
        name: "Gravity Stone Fan",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b62,
        name: "Hero's Rune [1]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b63,
        name: "Hero's Rune [2]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b64,
        name: "Hero's Rune [3]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b65,
        name: "Hero's Rune [4]",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b66,
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
        id: 0x4000049c,
        name: "Holyproof Dried Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e88fe,
        name: "Holyproof Pickled Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e90ba,
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
        id: 0x4000051e,
        name: "Immunizing White Cured Meat",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8b24,
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
        id: 0x401e8cc8,
        name: "Iris of Grace",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8cd2,
        name: "Iris of Occultation",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400006c2,
        name: "Kukri",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000bae,
        name: "Lands Between Rune",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000beb,
        name: "Large Glintstone Scrap",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e9006,
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
        id: 0x401e88f4,
        name: "Lightningproof Pickled Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000b67,
        name: "Lord's Rune",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e880e,
        name: "Lulling Branch",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000058c,
        name: "Magic Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e9010,
        name: "Marika's Rune",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e89f8,
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
        id: 0x40000b61,
        name: "Numen's Rune",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8908,
        name: "Opaline Pickled Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e90c4,
        name: "Perfumed Oil of Ranah",
        stack_size: 1,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000044c,
        name: "Pickled Turtle Neck",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005b4,
        name: "Poison Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400006b8,
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
        id: 0x401e8c50,
        name: "Polter Stone",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400003ac,
        name: "Preserving Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000d21,
        name: "Radiant Baldachin's Blessing",
        stack_size: 1,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400007e4,
        name: "Rainbow Stone",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400004d3,
        name: "Raw Meat Dumpling",
        stack_size: 3,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400003b6,
        name: "Rejuvenating Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400005d2,
        name: "Rot Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000032a,
        name: "Rowa Raisin",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8a0c,
        name: "Royal Magic Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400000be,
        name: "Rune Arc",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e900f,
        name: "Rune of an Unsung Hero",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e893a,
        name: "Sacred Bloody Flesh",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8930,
        name: "Scorpion Stew",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8930,
        name: "Scorpion Stew",
        stack_size: 1,
        max_storage: 1,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000cf8,
        name: "Scriptstone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e9008,
        name: "Shadow Realm Rune [1]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e9009,
        name: "Shadow Realm Rune [2]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e900a,
        name: "Shadow Realm Rune [3]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e900b,
        name: "Shadow Realm Rune [4]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e900c,
        name: "Shadow Realm Rune [5]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e900d,
        name: "Shadow Realm Rune [6]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e900e,
        name: "Shadow Realm Rune [7]",
        stack_size: 99,
        max_storage: 600,
        dlc: true,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000069a,
        name: "Shield Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8944,
        name: "Silver Horn Tender",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400004a6,
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
        id: 0x400005aa,
        name: "Soporific Grease",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000047e,
        name: "Spellproof Dried Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e88e0,
        name: "Spellproof Pickled Liver",
        stack_size: 5,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e87a0,
        name: "Spirit Raisin",
        stack_size: 30,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e9039,
        name: "Spritestone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000038e,
        name: "Stanching Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000050a,
        name: "Starlight Shards",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400003a2,
        name: "Stimulating Boluses",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40001f40,
        name: "Stonesword Key",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e916e,
        name: "Sunwarmth Stone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e90d8,
        name: "Surging Frenzied Flame",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x4000032b,
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
        id: 0x401e8cdc,
        name: "Thiollier's Concoction",
        stack_size: 99,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x400006a4,
        name: "Throwing Dagger",
        stack_size: 40,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x40000cee,
        name: "Warming Stone",
        stack_size: 10,
        max_storage: 600,
        ..Item::default_consumables()
    },
    Item {
        id: 0x401e8912,
        name: "Well-Pickled Turtle Neck",
        stack_size: 10,
        max_storage: 999,
        ..Item::default_consumables()
    },
];
