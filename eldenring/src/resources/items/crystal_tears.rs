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
        id: 0x401eafaa,
        name: "Bloodsucking Cracked Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002afc,
        name: "Cerulean Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002afd,
        name: "Cerulean Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b11,
        name: "Cerulean Hidden Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401eaf96,
        name: "Cerulean-Sapping Cracked Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002aff,
        name: "Crimson Bubbletear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002afa,
        name: "Crimson Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002afb,
        name: "Crimson Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401eaf8c,
        name: "Crimson-Sapping Cracked Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b01,
        name: "Crimsonburst Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401eaf82,
        name: "Crimsonburst Dried Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002af8,
        name: "Crimsonspill Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b0c,
        name: "Crimsonwhorl Bubbletear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401eafbe,
        name: "Deflecting Hardtear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b0e,
        name: "Dexterity-knot Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b10,
        name: "Faith-knot Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b14,
        name: "Flame-Shrouding Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401eafb4,
        name: "Glovewort Crystal Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b02,
        name: "Greenburst Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002af9,
        name: "Greenspill Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b17,
        name: "Holy-Shrouding Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b0f,
        name: "Intelligence-knot Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b0a,
        name: "Leaden Hardtear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b16,
        name: "Lightning-Shrouding Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b15,
        name: "Magic-Shrouding Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401eafa0,
        name: "Oil-Soaked Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b00,
        name: "Opaline Bubbletear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b03,
        name: "Opaline Hardtear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b13,
        name: "Purifying Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b09,
        name: "Ruptured Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b08,
        name: "Ruptured Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002afe,
        name: "Speckled Hardtear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b06,
        name: "Spiked Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b12,
        name: "Stonebarb Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b0d,
        name: "Strength-knot Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b05,
        name: "Thorny Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b0b,
        name: "Twiggy Cracked Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x401eaf78,
        name: "Viridian Hidden Tear",
        dlc: true,
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b07,
        name: "Windy Crystal Tear",
        ..Item::default_crystal_tears()
    },
    Item {
        id: 0x40002b04,
        name: "Winged Crystal Tear",
        ..Item::default_crystal_tears()
    },
];
