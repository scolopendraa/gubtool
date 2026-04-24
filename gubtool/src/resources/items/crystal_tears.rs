use super::Item;

impl Item {
    const fn default_crystal_tears() -> Self {
        Self {
            category: super::Categories::CrystalTears,
            ..Item::default()
        }
    }
}

pub static CRYSTAL_TEARS: [Item; 40] = [
    Item {
        id: 0x401EAFAA,
        name: "Bloodsucking Cracked Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002AFC,
        name: "Cerulean Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002AFD,
        name: "Cerulean Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B11,
        name: "Cerulean Hidden Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401EAF96,
        name: "Cerulean-Sapping Cracked Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002AFF,
        name: "Crimson Bubbletear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002AFA,
        name: "Crimson Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002AFB,
        name: "Crimson Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401EAF8C,
        name: "Crimson-Sapping Cracked Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B01,
        name: "Crimsonburst Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401EAF82,
        name: "Crimsonburst Dried Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002AF8,
        name: "Crimsonspill Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B0C,
        name: "Crimsonwhorl Bubbletear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401EAFBE,
        name: "Deflecting Hardtear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B0E,
        name: "Dexterity-knot Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B10,
        name: "Faith-knot Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B14,
        name: "Flame-Shrouding Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401EAFB4,
        name: "Glovewort Crystal Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B02,
        name: "Greenburst Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002AF9,
        name: "Greenspill Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B17,
        name: "Holy-Shrouding Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B0F,
        name: "Intelligence-knot Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B0A,
        name: "Leaden Hardtear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B16,
        name: "Lightning-Shrouding Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B15,
        name: "Magic-Shrouding Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401EAFA0,
        name: "Oil-Soaked Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B00,
        name: "Opaline Bubbletear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B03,
        name: "Opaline Hardtear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B13,
        name: "Purifying Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B09,
        name: "Ruptured Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B08,
        name: "Ruptured Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002AFE,
        name: "Speckled Hardtear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B06,
        name: "Spiked Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B12,
        name: "Stonebarb Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B0D,
        name: "Strength-knot Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B05,
        name: "Thorny Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B0B,
        name: "Twiggy Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401EAF78,
        name: "Viridian Hidden Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B07,
        name: "Windy Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002B04,
        name: "Winged Crystal Tear",
        ..Item::default_crystal_tears()
    },
];
