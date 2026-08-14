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
        id: 0x40001b59,
        name: "Agheel's Flame",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b1c,
        name: "Ancient Dragons' Lightning Spear",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001afe,
        name: "Ancient Dragons' Lightning Strike",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9e84,
        name: "Aspects of the Crucible: Bloom",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001d60,
        name: "Aspects of the Crucible: Breath",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001d56,
        name: "Aspects of the Crucible: Horns",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001d4c,
        name: "Aspects of the Crucible: Tail",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9e7a,
        name: "Aspects of the Crucible: Thorns",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000196e,
        name: "Assassin's Approach",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400018ba,
        name: "Barrier of Gold",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9fe2,
        name: "Bayle's Flame Lightning",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9fd8,
        name: "Bayle's Tyranny",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001aa4,
        name: "Beast Claw",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001ac2,
        name: "Bestial Constitution",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a90,
        name: "Bestial Sling",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001ab8,
        name: "Bestial Vitality",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001d6a,
        name: "Black Blade",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001842,
        name: "Black Flame",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000186a,
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
        id: 0x4000191f,
        name: "Blessing of the Erdtree",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000191e,
        name: "Blessing's Boon",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400018a6,
        name: "Bloodboon",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400018b0,
        name: "Bloodflame Blade",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000189c,
        name: "Bloodflame Talons",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b6d,
        name: "Borealis's Mist",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400017e8,
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
        id: 0x400013b0,
        name: "Death Lightning",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a2c,
        name: "Discus of Light",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea2b2,
        name: "Divine Beast Tornado",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea2bc,
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
        id: 0x40001b3b,
        name: "Dragonbolt Blessing",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9f7e,
        name: "Dragonbolt of Florissax",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b94,
        name: "Dragonclaw",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b58,
        name: "Dragonfire",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b6c,
        name: "Dragonice",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001ba8,
        name: "Dragonmaw",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b77,
        name: "Ekzykes's Decay",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a40,
        name: "Elden Stars",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b30,
        name: "Electrify Armament",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9f88,
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
        id: 0x401ea2f8,
        name: "Fire Serpent",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001edc,
        name: "Fire's Deadly Sin",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001932,
        name: "Flame Fortification",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000177a,
        name: "Flame Sling",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400017de,
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
        id: 0x400017a2,
        name: "Flame, Grant Me Strength",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400017ac,
        name: "Flame, Protect Me",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b1d,
        name: "Fortissax's Lightning Spear",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001c98,
        name: "Frenzied Burst",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b09,
        name: "Frozen Lightning Spear",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9d1c,
        name: "Furious Blade of Ansbach",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9fec,
        name: "Ghostflame Breath",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea29e,
        name: "Giant Golden Arc",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x400017d4,
        name: "Giantsflame Take Thee",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b80,
        name: "Glintstone Breath",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea294,
        name: "Golden Arcs",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001edf,
        name: "Golden Lightning Fortification",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400019c8,
        name: "Golden Vow",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001916,
        name: "Great Heal",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001bb2,
        name: "Greyoll's Roar",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001aae,
        name: "Gurranq's Beast Claw",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001915,
        name: "Heal",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9d80,
        name: "Heal from Afar",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b12,
        name: "Honed Bolt",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001ca2,
        name: "Howl of Shabriri",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a54,
        name: "Immutable Shield",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001c84,
        name: "Inescapable Frenzy",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9f74,
        name: "Knight's Lightning Spear",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9e98,
        name: "Land of Shadow",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b26,
        name: "Lansseax's Glaive",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a68,
        name: "Law of Causality",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a4a,
        name: "Law of Regression",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9eac,
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
        id: 0x40001af4,
        name: "Lightning Spear",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b08,
        name: "Lightning Strike",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a5e,
        name: "Litany of Proper Death",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001929,
        name: "Lord's Aid",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000195a,
        name: "Lord's Divine Fortification",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001917,
        name: "Lord's Heal",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000193c,
        name: "Magic Fortification",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b62,
        name: "Magma Breath",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea30c,
        name: "Messmer's Orb",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea104,
        name: "Midra's Flame of Frenzy",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9e8e,
        name: "Minor Erdtree",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9eb6,
        name: "Multilayered Ring of Light",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000187e,
        name: "Noble Presence",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001771,
        name: "O, Flame!",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a7c,
        name: "Order Healing",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a72,
        name: "Order's Blade",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001c20,
        name: "Pest Threads",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea0aa,
        name: "Pest-Thread Spears",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b8a,
        name: "Placidusax's Ruin",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001c3e,
        name: "Poison Armament",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001c34,
        name: "Poison Mist",
        ..Item::default_incantations()
    },
    Item {
        id: 0x400018c4,
        name: "Protection of the Erdtree",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a36,
        name: "Radagon's Rings of Light",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea302,
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
        id: 0x401e9f10,
        name: "Roar of Rugalea",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b76,
        name: "Rotten Breath",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea0a0,
        name: "Rotten Butterflies",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001c48,
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
        id: 0x40001b81,
        name: "Smarag's Glintstone Breath",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea2a8,
        name: "Spira",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a9a,
        name: "Stone of Gurranq",
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000184c,
        name: "Surge, O Flame!",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001c2a,
        name: "Swarm of Flies",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001c8e,
        name: "The Flame of Frenzy",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b63,
        name: "Theodorix's Magma",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001a2d,
        name: "Triple Rings of Light",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001c8f,
        name: "Unendurable Frenzy",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001914,
        name: "Urgent Heal",
        ..Item::default_incantations()
    },
    Item {
        id: 0x40001b3a,
        name: "Vyke's Dragonbolt",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401ea230,
        name: "Watchful Spirit",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000178e,
        name: "Whirl, O Flame!",
        ..Item::default_incantations()
    },
    Item {
        id: 0x401e9ea2,
        name: "Wrath from Afar",
        dlc: true,
        ..Item::default_incantations()
    },
    Item {
        id: 0x4000190a,
        name: "Wrath of Gold",
        ..Item::default_incantations()
    },
];
