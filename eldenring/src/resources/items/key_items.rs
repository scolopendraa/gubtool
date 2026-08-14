use super::Item;

impl Item {
    const fn default_key_item() -> Self {
        Self {
            category: super::Categories::KeyItems,
            ..Item::default()
        }
    }
}

pub static KEY_ITEMS: [Item; 213] = [
    Item {
        id: 0x40001fad,
        name: "Academy Glintstone Key (Player)",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fee,
        name: "Academy Glintstone Key (Thops)",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400022a2,
        name: "Academy Scroll",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fe6,
        name: "Amber Draught",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fce,
        name: "Amber Starlight",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000bb8,
        name: "Ancestral Infant's Head",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(12027090),
        ..Item::default_key_item()
    },
    Item {
        id: 0x400022a1,
        name: "Ancient Dragon Prayerbook",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3dc,
        name: "Ancient Ruins Cross Message",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(2047477000),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000229b,
        name: "Assassin's Prayerbook",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002313,
        name: "Beast Eye",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400239),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fec,
        name: "Black Knifeprint",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3d3,
        name: "Black Syrup",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400642),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000230e,
        name: "Black Whetblade",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(65720),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000820,
        name: "Blasphemous Claw",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(400292),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000066,
        name: "Bloody Finger",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60270),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000069,
        name: "Blue Cipher Ring",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60290),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e9038,
        name: "Bondstone",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001faf,
        name: "Carian Inverted Statue",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3db,
        name: "Castle Cross Message",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(2047447710),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000852,
        name: "Celestial Dew",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000200a,
        name: "Champion's Song Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580020),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001feb,
        name: "Chrysalids' Memento",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002292,
        name: "Conspectus Scroll",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000251c,
        name: "Cracked Pot",
        stack_size: 20,
        max_storage: 0,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002134,
        name: "Crafting Kit",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(60120),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3c7,
        name: "Cross Map",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fff,
        name: "Cursemark of Death",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(34117500),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fe8,
        name: "Dancer's Castanets",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400181),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fb9,
        name: "Dark Moon Ring",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(114),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000082a,
        name: "Deathroot",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fa9,
        name: "Dectus Medallion (Left)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(1046367500),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001faa,
        name: "Dectus Medallion (Right)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(1051397900),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002007,
        name: "Discarded Palace Key",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400159),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea48a,
        name: "Domain of Dragons Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580120),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400022a0,
        name: "Dragon Cult Prayerbook",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000274c,
        name: "Dragon Heart",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fc6,
        name: "Drawing-Room Key",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400072),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000065,
        name: "Duelist's Furled Finger",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60240),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b93,
        name: "Elden Remembrance",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000006f,
        name: "Festering Bloody Finger",
        stack_size: 99,
        max_storage: 99,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000067,
        name: "Finger Severer",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60310),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fbe,
        name: "Fingerprint Grape",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fdf,
        name: "Fingerslayer Blade",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(12027080),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002297,
        name: "Fire Monks' Prayerbook",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000fa,
        name: "Flask of Wondrous Physick",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(60020),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000200d,
        name: "Flightless Bird Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580050),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000096,
        name: "Furlcalling Finger Remedy",
        stack_size: 999,
        max_storage: 0,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3d9,
        name: "Furnace Keeper's Note",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(2049477000),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3c6,
        name: "Gaol Lower Level Key",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3c5,
        name: "Gaol Upper Level Key",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002298,
        name: "Giant's Prayerbook",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000230d,
        name: "Glintstone Whetblade",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(65680),
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000bf,
        name: "Godrick's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(191),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fd4,
        name: "Godrick's Great Rune (Deactivated)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(171),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002299,
        name: "Godskin Prayerbook",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fe2,
        name: "Gold Sewing Needle",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(1037467000),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000229e,
        name: "Golden Order Principia",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000271a,
        name: "Golden Seed",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001ffc,
        name: "Golden Tailoring Tools",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(60150),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002760,
        name: "Great Rune of the Unborn",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(197),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fef,
        name: "Haligtree Secret Medallion (Left)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400280),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001ff0,
        name: "Haligtree Secret Medallion (Right)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400130),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3cb,
        name: "Heart of Bayle",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea99c,
        name: "Hefty Cracked Pot",
        stack_size: 10,
        max_storage: 0,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3c8,
        name: "Hole-Laden Necklace",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400660),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002008,
        name: "Homing Instinct Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580000),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e90ba,
        name: "Horned Bairn",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3c3,
        name: "Igon's Furled Finger",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400710),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001ffa,
        name: "Imbued Sword Key",
        stack_size: 99,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea488,
        name: "Incursion Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580100),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fc3,
        name: "Irina's Letter",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400080),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8cc8,
        name: "Iris of Grace",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8cd2,
        name: "Iris of Occultation",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000230a,
        name: "Iron Whetblade",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(65610),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3d7,
        name: "Keep Wall Key",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001ffe,
        name: "Knifeprint Clue",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8c6e,
        name: "Lamenter's Mask",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000816,
        name: "Lantern",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001ff9,
        name: "Larval Tear (Base Game)",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3e1,
        name: "Larval Tear (DLC)",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3cf,
        name: "Letter for Freyja",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400625),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fbf,
        name: "Letter from Volcano Manor (Istvan)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400073),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fc4,
        name: "Letter from Volcano Manor (Rileigh)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400074),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fed,
        name: "Letter to Bernahl",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400290),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fe7,
        name: "Letter to Patches",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400180),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fdb,
        name: "Lord of Blood's Favor (Bloody)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400033),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fda,
        name: "Lord of Blood's Favor (White)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400031),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002756,
        name: "Lost Ashes of War",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000c4,
        name: "Malenia's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(196),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fd9,
        name: "Malenia's Great Rune (Dectivated)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(176),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000085c,
        name: "Margit's Shackle",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(110000),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002006,
        name: "Meeting Place Map",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000272e,
        name: "Memory Stone",
        stack_size: 8,
        max_storage: 0,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001ff6,
        name: "Mending Rune of Perfect Order",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(9500),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001ff7,
        name: "Mending Rune of the Death-Prince",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(9502),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001ff8,
        name: "Mending Rune of the Fell Curse",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(9504),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3d5,
        name: "Messmer's Kindling",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(510460),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3e2,
        name: "Message from Leda",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580600),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000be0,
        name: "Mimic's Veil",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(10007970),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fd2,
        name: "Miniature Ranni",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400394),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001ffb,
        name: "Miniature Ranni (Empty)",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3c0,
        name: "Miquella's Great Rune",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000088e,
        name: "Miquella's Needle",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(400324),
        ..Item::default_key_item()
    },
    Item {
        id: 0x400021d4,
        name: "Mirage Riddle",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000c3,
        name: "Mohg's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(195),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fd8,
        name: "Mohg's Great Rune (Dectivated)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(175),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000866,
        name: "Mohg's Shackle",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(35007310),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3dd,
        name: "Monk's Missive",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(2048457510),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000c1,
        name: "Morgott's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(193),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fd6,
        name: "Morgott's Great Rune (Dectivated)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(173),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3cc,
        name: "New Cross Map",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000bc2,
        name: "Omen Bairn",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(35007990),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002526,
        name: "Perfume Bottle",
        stack_size: 10,
        max_storage: 0,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e90c4,
        name: "Perfumed Oil of Ranah",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000087,
        name: "Phantom Great Rune",
        stack_size: 99,
        max_storage: 0,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3e4,
        name: "Prayer Room Key",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400696),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8c64,
        name: "Priestess Heart",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000200c,
        name: "Prophecy Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580040),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000870,
        name: "Pureblood Knight's Medal",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(400032),
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000c0,
        name: "Radahn's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(192),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fd5,
        name: "Radahn's Great Rune (Dectivated)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(172),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000070,
        name: "Recusant Finger",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60260),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fc5,
        name: "Red Letter",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400075),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000230b,
        name: "Red-Hot Whetblade",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(65640),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000200e,
        name: "Redmane Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580060),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000bc3,
        name: "Regal Omen Bairn",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(290050),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8fdb,
        name: "Remembrance of a God and a Lord",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b8d,
        name: "Remembrance of Hoarah Loux",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8fde,
        name: "Remembrance of Putrescence",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b8c,
        name: "Remembrance of the Black Blade",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b89,
        name: "Remembrance of the Blasphemous",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b8b,
        name: "Remembrance of the Blood Lord",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8fd9,
        name: "Remembrance of the Dancing Lion",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b8e,
        name: "Remembrance of the Dragonlord",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b91,
        name: "Remembrance of the Fire Giant",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b8f,
        name: "Remembrance of the Full Moon Queen",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b86,
        name: "Remembrance of the Grafted",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8fd5,
        name: "Remembrance of the Impaler",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b90,
        name: "Remembrance of the Lichdragon",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8fdc,
        name: "Remembrance of the Lord of Frenzied Flame",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8fdd,
        name: "Remembrance of the Mother of Fingers",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b94,
        name: "Remembrance of the Naturalborn",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b88,
        name: "Remembrance of the Omen King",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b92,
        name: "Remembrance of the Regal Ancestor",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b8a,
        name: "Remembrance of the Rot Goddess",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8fd8,
        name: "Remembrance of the Saint of the Bud",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8fd6,
        name: "Remembrance of the Shadow Sunflower",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000b87,
        name: "Remembrance of the Starscourge",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8fd7,
        name: "Remembrance of the Twin Moon Knight",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8fd4,
        name: "Remembrance of the Wild Boar Rider",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002009,
        name: "Resurrection Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580010),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401eabf4,
        name: "Revered Spirit Ash",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000251d,
        name: "Ritual Pot",
        stack_size: 10,
        max_storage: 0,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8c5a,
        name: "Rock Heart",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000201f,
        name: "Rogier's Letter",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400356),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fab,
        name: "Rold Medallion",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002293,
        name: "Royal House Scroll",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3d0,
        name: "Ruins Map (1st)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400660),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3d1,
        name: "Ruins Map (2nd)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400661),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3d2,
        name: "Ruins Map (3rd)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400662),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001f4a,
        name: "Rusty Key",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fc2,
        name: "Rya's Necklace",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000c2,
        name: "Rykard's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(194),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fd7,
        name: "Rykard's Great Rune (Dectivated)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(174),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002724,
        name: "Sacred Tear",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000230c,
        name: "Sanctified Whetblade",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(65660),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401eab90,
        name: "Scadutree Fragment",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000cf8,
        name: "Scriptstone",
        stack_size: 10,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3ce,
        name: "Secret Rite Scroll",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(21017340),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002001,
        name: "Seedbed Curse",
        stack_size: 99,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fd0,
        name: "Sellen's Primal Glintstone",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400100),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fe9,
        name: "Sellian Sealbreaker",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400102),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002312,
        name: "Sellia's Secret",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400311),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fcf,
        name: "Seluvis's Introduction",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fe4,
        name: "Seluvis's Potion",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fc1,
        name: "Serpent's Amnion",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002005,
        name: "Sewer-Gaol Key",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400380),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fe1,
        name: "Sewing Needle",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400004d8,
        name: "Shabriri Grape",
        stack_size: 99,
        max_storage: 999,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000006d,
        name: "Small Golden Effigy",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60230),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000006e,
        name: "Small Red Effigy",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60250),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000200b,
        name: "Sorcerer Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580030),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000082,
        name: "Spectral Steed Whistle",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60100),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fde,
        name: "Spirit Calling Bell",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(60110),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001f40,
        name: "Stonesword Key",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3de,
        name: "Storehouse Cross Message",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(21017180),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3cd,
        name: "Storeroom Key",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(20007480),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fe3,
        name: "Tailoring Tools",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(60140),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002738,
        name: "Talisman Pouch (Enia)",
        stack_size: 3,
        max_storage: 0,
        event_id: Some(60500),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002738,
        name: "Talisman Pouch (Golden Shade Godfrey)",
        stack_size: 3,
        max_storage: 0,
        event_id: Some(60520),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002738,
        name: "Talisman Pouch (Margit/Morgott)",
        stack_size: 3,
        max_storage: 0,
        event_id: Some(60510),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000064,
        name: "Tarnished's Furled Finger",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60220),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000006c,
        name: "Taunter's Tongue",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60300),
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007f8,
        name: "Telescope",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea489,
        name: "The Sacred Tower Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580110),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002002,
        name: "The Stormhawk King",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(10017010),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401e8cdc,
        name: "Thiollier's Concoction",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fc0,
        name: "Tonic of Forgetfulness",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3e0,
        name: "Torn Diary Page",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3e5,
        name: "Tower of Shadow Message",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(20007830),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000229a,
        name: "Two Fingers' Prayerbook",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000230f,
        name: "Unalloyed Gold Needle (Broken)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(530405),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002310,
        name: "Unalloyed Gold Needle (Gowry)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400310),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002004,
        name: "Unalloyed Gold Needle (Millicent)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400321),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002311,
        name: "Valkyrie's Prosthesis",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001fc9,
        name: "Volcano Manor Invitation",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400090),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002314,
        name: "Weathered Dagger",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40002023,
        name: "Weathered Map",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401ea3c4,
        name: "Well Depths Key",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(20007510),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000218e,
        name: "Whetstone Knife",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(60130),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000068,
        name: "White Cipher Ring",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60280),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000c08,
        name: "Wraith Calling Bell",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(1037427900),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000201d,
        name: "Zorayas's Letter",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400091),
        ..Item::default_key_item()
    },
];
