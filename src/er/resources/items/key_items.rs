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
        id: 0x40001FAD,
        name: "Academy Glintstone Key (Player)",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FEE,
        name: "Academy Glintstone Key (Thops)",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400022A2,
        name: "Academy Scroll",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FE6,
        name: "Amber Draught",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FCE,
        name: "Amber Starlight",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000BB8,
        name: "Ancestral Infant's Head",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(12027090),
        ..Item::default_key_item()
    },
    Item {
        id: 0x400022A1,
        name: "Ancient Dragon Prayerbook",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3DC,
        name: "Ancient Ruins Cross Message",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(2047477000),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000229B,
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
        id: 0x40001FEC,
        name: "Black Knifeprint",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3D3,
        name: "Black Syrup",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400642),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000230E,
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
        id: 0x401E9038,
        name: "Bondstone",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FAF,
        name: "Carian Inverted Statue",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3DB,
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
        id: 0x4000200A,
        name: "Champion's Song Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580020),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FEB,
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
        id: 0x4000251C,
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
        id: 0x401EA3C7,
        name: "Cross Map",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FFF,
        name: "Cursemark of Death",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(34117500),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FE8,
        name: "Dancer's Castanets",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400181),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FB9,
        name: "Dark Moon Ring",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(114),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000082A,
        name: "Deathroot",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FA9,
        name: "Dectus Medallion (Left)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(1046367500),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FAA,
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
        id: 0x401EA48A,
        name: "Domain of Dragons Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580120),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400022A0,
        name: "Dragon Cult Prayerbook",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000274C,
        name: "Dragon Heart",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FC6,
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
        id: 0x40000B93,
        name: "Elden Remembrance",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000006F,
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
        id: 0x40001FBE,
        name: "Fingerprint Grape",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FDF,
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
        id: 0x400000FA,
        name: "Flask of Wondrous Physick",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(60020),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000200D,
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
        id: 0x401EA3D9,
        name: "Furnace Keeper's Note",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(2049477000),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3C6,
        name: "Gaol Lower Level Key",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3C5,
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
        id: 0x4000230D,
        name: "Glintstone Whetblade",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(65680),
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000BF,
        name: "Godrick's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(191),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FD4,
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
        id: 0x40001FE2,
        name: "Gold Sewing Needle",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(1037467000),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000229E,
        name: "Golden Order Principia",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000271A,
        name: "Golden Seed",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FFC,
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
        id: 0x40001FEF,
        name: "Haligtree Secret Medallion (Left)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400280),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FF0,
        name: "Haligtree Secret Medallion (Right)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400130),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3CB,
        name: "Heart of Bayle",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA99C,
        name: "Hefty Cracked Pot",
        stack_size: 10,
        max_storage: 0,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3C8,
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
        id: 0x401E90BA,
        name: "Horned Bairn",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3C3,
        name: "Igon's Furled Finger",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400710),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FFA,
        name: "Imbued Sword Key",
        stack_size: 99,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA488,
        name: "Incursion Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580100),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FC3,
        name: "Irina's Letter",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400080),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8CC8,
        name: "Iris of Grace",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8CD2,
        name: "Iris of Occultation",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000230A,
        name: "Iron Whetblade",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(65610),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3D7,
        name: "Keep Wall Key",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FFE,
        name: "Knifeprint Clue",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8C6E,
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
        id: 0x40001FF9,
        name: "Larval Tear (Base Game)",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3E1,
        name: "Larval Tear (DLC)",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3CF,
        name: "Letter for Freyja",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400625),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FBF,
        name: "Letter from Volcano Manor (Istvan)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400073),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FC4,
        name: "Letter from Volcano Manor (Rileigh)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400074),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FED,
        name: "Letter to Bernahl",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400290),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FE7,
        name: "Letter to Patches",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400180),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FDB,
        name: "Lord of Blood's Favor (Bloody)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400033),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FDA,
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
        id: 0x400000C4,
        name: "Malenia's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(196),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FD9,
        name: "Malenia's Great Rune (Dectivated)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(176),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000085C,
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
        id: 0x4000272E,
        name: "Memory Stone",
        stack_size: 8,
        max_storage: 0,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FF6,
        name: "Mending Rune of Perfect Order",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(9500),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FF7,
        name: "Mending Rune of the Death-Prince",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(9502),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FF8,
        name: "Mending Rune of the Fell Curse",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(9504),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3D5,
        name: "Messmer's Kindling",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(510460),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3E2,
        name: "Message from Leda",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580600),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000BE0,
        name: "Mimic's Veil",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(10007970),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FD2,
        name: "Miniature Ranni",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400394),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FFB,
        name: "Miniature Ranni (Empty)",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3C0,
        name: "Miquella's Great Rune",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000088E,
        name: "Miquella's Needle",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(400324),
        ..Item::default_key_item()
    },
    Item {
        id: 0x400021D4,
        name: "Mirage Riddle",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000C3,
        name: "Mohg's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(195),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FD8,
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
        id: 0x401EA3DD,
        name: "Monk's Missive",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(2048457510),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000C1,
        name: "Morgott's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(193),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FD6,
        name: "Morgott's Great Rune (Dectivated)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(173),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3CC,
        name: "New Cross Map",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000BC2,
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
        id: 0x401E90C4,
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
        id: 0x401EA3E4,
        name: "Prayer Room Key",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400696),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8C64,
        name: "Priestess Heart",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000200C,
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
        id: 0x400000C0,
        name: "Radahn's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(192),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FD5,
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
        id: 0x40001FC5,
        name: "Red Letter",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400075),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000230B,
        name: "Red-Hot Whetblade",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(65640),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000200E,
        name: "Redmane Painting",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(580060),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000BC3,
        name: "Regal Omen Bairn",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(290050),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8FDB,
        name: "Remembrance of a God and a Lord",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B8D,
        name: "Remembrance of Hoarah Loux",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8FDE,
        name: "Remembrance of Putrescence",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B8C,
        name: "Remembrance of the Black Blade",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B89,
        name: "Remembrance of the Blasphemous",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B8B,
        name: "Remembrance of the Blood Lord",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8FD9,
        name: "Remembrance of the Dancing Lion",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B8E,
        name: "Remembrance of the Dragonlord",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B91,
        name: "Remembrance of the Fire Giant",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B8F,
        name: "Remembrance of the Full Moon Queen",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B86,
        name: "Remembrance of the Grafted",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8FD5,
        name: "Remembrance of the Impaler",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B90,
        name: "Remembrance of the Lichdragon",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8FDC,
        name: "Remembrance of the Lord of Frenzied Flame",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8FDD,
        name: "Remembrance of the Mother of Fingers",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B94,
        name: "Remembrance of the Naturalborn",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B88,
        name: "Remembrance of the Omen King",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B92,
        name: "Remembrance of the Regal Ancestor",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B8A,
        name: "Remembrance of the Rot Goddess",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8FD8,
        name: "Remembrance of the Saint of the Bud",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8FD6,
        name: "Remembrance of the Shadow Sunflower",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000B87,
        name: "Remembrance of the Starscourge",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8FD7,
        name: "Remembrance of the Twin Moon Knight",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8FD4,
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
        id: 0x401EABF4,
        name: "Revered Spirit Ash",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000251D,
        name: "Ritual Pot",
        stack_size: 10,
        max_storage: 0,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401E8C5A,
        name: "Rock Heart",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000201F,
        name: "Rogier's Letter",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400356),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FAB,
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
        id: 0x401EA3D0,
        name: "Ruins Map (1st)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400660),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3D1,
        name: "Ruins Map (2nd)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400661),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3D2,
        name: "Ruins Map (3rd)",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400662),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001F4A,
        name: "Rusty Key",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FC2,
        name: "Rya's Necklace",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400000C2,
        name: "Rykard's Great Rune (Activated)",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(194),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FD7,
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
        id: 0x4000230C,
        name: "Sanctified Whetblade",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(65660),
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EAB90,
        name: "Scadutree Fragment",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40000CF8,
        name: "Scriptstone",
        stack_size: 10,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3CE,
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
        id: 0x40001FD0,
        name: "Sellen's Primal Glintstone",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400100),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FE9,
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
        id: 0x40001FCF,
        name: "Seluvis's Introduction",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FE4,
        name: "Seluvis's Potion",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FC1,
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
        id: 0x40001FE1,
        name: "Sewing Needle",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x400004D8,
        name: "Shabriri Grape",
        stack_size: 99,
        max_storage: 999,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000006D,
        name: "Small Golden Effigy",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60230),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000006E,
        name: "Small Red Effigy",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60250),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000200B,
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
        id: 0x40001FDE,
        name: "Spirit Calling Bell",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(60110),
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001F40,
        name: "Stonesword Key",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3DE,
        name: "Storehouse Cross Message",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(21017180),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3CD,
        name: "Storeroom Key",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(20007480),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FE3,
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
        id: 0x4000006C,
        name: "Taunter's Tongue",
        stack_size: 1,
        max_storage: 0,
        event_id: Some(60300),
        ..Item::default_key_item()
    },
    Item {
        id: 0x400007F8,
        name: "Telescope",
        stack_size: 1,
        max_storage: 600,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA489,
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
        id: 0x401E8CDC,
        name: "Thiollier's Concoction",
        stack_size: 99,
        max_storage: 600,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x40001FC0,
        name: "Tonic of Forgetfulness",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3E0,
        name: "Torn Diary Page",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x401EA3E5,
        name: "Tower of Shadow Message",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(20007830),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000229A,
        name: "Two Fingers' Prayerbook",
        stack_size: 1,
        max_storage: 1,
        event_id: None,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000230F,
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
        id: 0x40001FC9,
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
        id: 0x401EA3C4,
        name: "Well Depths Key",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(20007510),
        dlc: true,
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000218E,
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
        id: 0x40000C08,
        name: "Wraith Calling Bell",
        stack_size: 1,
        max_storage: 600,
        event_id: Some(1037427900),
        ..Item::default_key_item()
    },
    Item {
        id: 0x4000201D,
        name: "Zorayas's Letter",
        stack_size: 1,
        max_storage: 1,
        event_id: Some(400091),
        ..Item::default_key_item()
    },
];
