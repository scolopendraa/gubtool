use super::Item;

impl Item {
    const fn default_incantations() -> Self {
        Self {
            category: super::Categories::Incantations,
            max_storage: 600,
            ..Item::default()
        }
    }
}

pub static INCANTATIONS: [Item; 129] = [
    Item {
        id: 0x40001B59,
        name: "Agheel's Flame",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B1C,
        name: "Ancient Dragons' Lightning Spear",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001AFE,
        name: "Ancient Dragons' Lightning Strike",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9E84,
        name: "Aspects of the Crucible: Bloom",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001D60,
        name: "Aspects of the Crucible: Breath",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001D56,
        name: "Aspects of the Crucible: Horns",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001D4C,
        name: "Aspects of the Crucible: Tail",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9E7A,
        name: "Aspects of the Crucible: Thorns",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000196E,
        name: "Assassin's Approach",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400018BA,
        name: "Barrier of Gold",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9FE2,
        name: "Bayle's Flame Lightning",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9FD8,
        name: "Bayle's Tyranny",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001AA4,
        name: "Beast Claw",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001AC2,
        name: "Bestial Constitution",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A90,
        name: "Bestial Sling",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001AB8,
        name: "Bestial Vitality",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001D6A,
        name: "Black Blade",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001842,
        name: "Black Flame",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000186A,
        name: "Black Flame Blade",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001860,
        name: "Black Flame Ritual",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001874,
        name: "Black Flame's Protection",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000191F,
        name: "Blessing of the Erdtree",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000191E,
        name: "Blessing's Boon",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400018A6,
        name: "Bloodboon",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400018B0,
        name: "Bloodflame Blade",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000189C,
        name: "Bloodflame Talons",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B6D,
        name: "Borealis's Mist",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400017E8,
        name: "Burn, O Flame!",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001770,
        name: "Catch Flame",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001928,
        name: "Cure Poison",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001982,
        name: "Darkness",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400013B0,
        name: "Death Lightning",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A2C,
        name: "Discus of Light",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA2B2,
        name: "Divine Beast Tornado",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA2BC,
        name: "Divine Bird Feathers",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001950,
        name: "Divine Fortification",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B3B,
        name: "Dragonbolt Blessing",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9F7E,
        name: "Dragonbolt of Florissax",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B94,
        name: "Dragonclaw",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B58,
        name: "Dragonfire",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B6C,
        name: "Dragonice",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001BA8,
        name: "Dragonmaw",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B77,
        name: "Ekzykes's Decay",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A40,
        name: "Elden Stars",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B30,
        name: "Electrify Armament",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9F88,
        name: "Electrocharge",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001918,
        name: "Erdtree Heal",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA2F8,
        name: "Fire Serpent",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001EDC,
        name: "Fire's Deadly Sin",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001932,
        name: "Flame Fortification",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000177A,
        name: "Flame Sling",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400017DE,
        name: "Flame of the Fell God",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001798,
        name: "Flame, Cleanse Me",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001784,
        name: "Flame, Fall Upon Them",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400017A2,
        name: "Flame, Grant Me Strength",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400017AC,
        name: "Flame, Protect Me",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B1D,
        name: "Fortissax's Lightning Spear",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001C98,
        name: "Frenzied Burst",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B09,
        name: "Frozen Lightning Spear",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9D1C,
        name: "Furious Blade of Ansbach",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9FEC,
        name: "Ghostflame Breath",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA29E,
        name: "Giant Golden Arc",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x400017D4,
        name: "Giantsflame Take Thee",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B80,
        name: "Glintstone Breath",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA294,
        name: "Golden Arcs",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001EDF,
        name: "Golden Lightning Fortification",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400019C8,
        name: "Golden Vow",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001916,
        name: "Great Heal",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001BB2,
        name: "Greyoll's Roar",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001AAE,
        name: "Gurranq's Beast Claw",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001915,
        name: "Heal",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9D80,
        name: "Heal from Afar",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B12,
        name: "Honed Bolt",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001CA2,
        name: "Howl of Shabriri",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A54,
        name: "Immutable Shield",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001C84,
        name: "Inescapable Frenzy",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9F74,
        name: "Knight's Lightning Spear",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9E98,
        name: "Land of Shadow",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B26,
        name: "Lansseax's Glaive",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A68,
        name: "Law of Causality",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A4A,
        name: "Law of Regression",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9EAC,
        name: "Light of Miquella",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001946,
        name: "Lightning Fortification",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001AF4,
        name: "Lightning Spear",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B08,
        name: "Lightning Strike",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A5E,
        name: "Litany of Proper Death",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001929,
        name: "Lord's Aid",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000195A,
        name: "Lord's Divine Fortification",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001917,
        name: "Lord's Heal",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000193C,
        name: "Magic Fortification",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B62,
        name: "Magma Breath",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA30C,
        name: "Messmer's Orb",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA104,
        name: "Midra's Flame of Frenzy",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9E8E,
        name: "Minor Erdtree",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9EB6,
        name: "Multilayered Ring of Light",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000187E,
        name: "Noble Presence",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001771,
        name: "O, Flame!",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A7C,
        name: "Order Healing",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A72,
        name: "Order's Blade",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001C20,
        name: "Pest Threads",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA0AA,
        name: "Pest-Thread Spears",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B8A,
        name: "Placidusax's Ruin",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001C3E,
        name: "Poison Armament",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001C34,
        name: "Poison Mist",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400018C4,
        name: "Protection of the Erdtree",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A36,
        name: "Radagon's Rings of Light",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA302,
        name: "Rain of Fire",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001900,
        name: "Rejection",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9F10,
        name: "Roar of Rugalea",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B76,
        name: "Rotten Breath",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA0A0,
        name: "Rotten Butterflies",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001C48,
        name: "Scarlet Aeonia",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001856,
        name: "Scouring Black Flame",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001978,
        name: "Shadow Bait",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B81,
        name: "Smarag's Glintstone Breath",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA2A8,
        name: "Spira",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A9A,
        name: "Stone of Gurranq",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000184C,
        name: "Surge, O Flame!",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001C2A,
        name: "Swarm of Flies",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001C8E,
        name: "The Flame of Frenzy",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B63,
        name: "Theodorix's Magma",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001A2D,
        name: "Triple Rings of Light",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001C8F,
        name: "Unendurable Frenzy",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001914,
        name: "Urgent Heal",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001B3A,
        name: "Vyke's Dragonbolt",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401EA230,
        name: "Watchful Spirit",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000178E,
        name: "Whirl, O Flame!",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401E9EA2,
        name: "Wrath from Afar",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000190A,
        name: "Wrath of Gold",
        ..Item::default_incantations()
    },
];
