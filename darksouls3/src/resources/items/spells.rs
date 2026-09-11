use super::Item;

impl Item {
    const fn default_spell() -> Self {
        Self {
            category: super::ItemCategory::Spells,
            stack_size: 1,
            max_storage: 99,
            ..Item::default()
        }
    }
}

pub static SPELLS: [Item; 106] = [
    Item {
        name: "Affinity",
        id: 0x401875b8,
        ..Item::default_spell()
    },
    Item {
        name: "Aural Decoy",
        id: 0x4014b0e0,
        ..Item::default_spell()
    },
    Item {
        name: "Cast Light",
        id: 0x40149970,
        ..Item::default_spell()
    },
    Item {
        name: "Chameleon",
        id: 0x4014acf8,
        ..Item::default_spell()
    },
    Item {
        name: "Crystal Hail",
        id: 0x4014f348,
        ..Item::default_spell()
    },
    Item {
        name: "Crystal Magic Weapon",
        id: 0x40140500,
        ..Item::default_spell()
    },
    Item {
        name: "Crystal Soul Spear",
        id: 0x4013f178,
        ..Item::default_spell()
    },
    Item {
        name: "Dark Edge",
        id: 0x40189cc8,
        ..Item::default_spell()
    },
    Item {
        name: "Deep Soul",
        id: 0x4013f560,
        ..Item::default_spell()
    },
    Item {
        name: "Farron Dart",
        id: 0x40124f80,
        ..Item::default_spell()
    },
    Item {
        name: "Farron Flashsword",
        id: 0x4014fb18,
        ..Item::default_spell()
    },
    Item {
        name: "Farron Hail",
        id: 0x4014ef60,
        ..Item::default_spell()
    },
    Item {
        name: "Frozen Weapon",
        id: 0x401408e8,
        dlc: 1,
        ..Item::default_spell()
    },
    Item {
        name: "Great Deep Soul",
        id: 0x4013f948,
        ..Item::default_spell()
    },
    Item {
        name: "Great Farron Dart",
        id: 0x40127690,
        ..Item::default_spell()
    },
    Item {
        name: "Great Heavy Soul Arrow",
        id: 0x4013e1d8,
        ..Item::default_spell()
    },
    Item {
        name: "Great Magic Barrier",
        id: 0x40365628,
        ..Item::default_spell()
    },
    Item {
        name: "Great Magic Shield",
        id: 0x40144f38,
        ..Item::default_spell()
    },
    Item {
        name: "Great Magic Weapon",
        id: 0x40140118,
        ..Item::default_spell()
    },
    Item {
        name: "Great Soul Arrow",
        id: 0x4013da08,
        ..Item::default_spell()
    },
    Item {
        name: "Great Soul Dregs",
        id: 0x401879a0,
        dlc: 2,
        ..Item::default_spell()
    },
    Item {
        name: "Heavy Soul Arrow",
        id: 0x4013ddf0,
        ..Item::default_spell()
    },
    Item {
        name: "Hidden Body",
        id: 0x40147648,
        ..Item::default_spell()
    },
    Item {
        name: "Hidden Weapon",
        id: 0x40147260,
        ..Item::default_spell()
    },
    Item {
        name: "Homing Crystal Soulmass",
        id: 0x4013e9a8,
        ..Item::default_spell()
    },
    Item {
        name: "Homing Soulmass",
        id: 0x4013e5c0,
        ..Item::default_spell()
    },
    Item {
        name: "Magic Barrier",
        id: 0x40365240,
        ..Item::default_spell()
    },
    Item {
        name: "Magic Shield",
        id: 0x40144b50,
        ..Item::default_spell()
    },
    Item {
        name: "Magic Weapon",
        id: 0x4013fd30,
        ..Item::default_spell()
    },
    Item {
        name: "Old Moonlight",
        id: 0x4014ff00,
        dlc: 2,
        ..Item::default_spell()
    },
    Item {
        name: "Pestilent Mist",
        id: 0x401a8ce0,
        ..Item::default_spell()
    },
    Item {
        name: "Repair",
        id: 0x4014a528,
        ..Item::default_spell()
    },
    Item {
        name: "Snap Freeze",
        id: 0x401a90c8,
        dlc: 1,
        ..Item::default_spell()
    },
    Item {
        name: "Soul Arrow",
        id: 0x4013d620,
        ..Item::default_spell()
    },
    Item {
        name: "Soul Greatsword",
        id: 0x4014f730,
        ..Item::default_spell()
    },
    Item {
        name: "Soul Spear",
        id: 0x4013ed90,
        ..Item::default_spell()
    },
    Item {
        name: "Soul Stream",
        id: 0x4018b820,
        ..Item::default_spell()
    },
    Item {
        name: "Spook",
        id: 0x4014a910,
        ..Item::default_spell()
    },
    Item {
        name: "Twisted Wall of Light",
        id: 0x40193138,
        ..Item::default_spell()
    },
    Item {
        name: "White Dragon Breath",
        id: 0x4014e790,
        ..Item::default_spell()
    },
    Item {
        name: "Acid Surge",
        id: 0x4024f4f0,
        ..Item::default_spell()
    },
    Item {
        name: "Black Fire Orb",
        id: 0x4027d350,
        ..Item::default_spell()
    },
    Item {
        name: "Black Flame",
        id: 0x40256e08,
        ..Item::default_spell()
    },
    Item {
        name: "Black Serpent",
        id: 0x4024ba58,
        ..Item::default_spell()
    },
    Item {
        name: "Boulder Heave",
        id: 0x40282170,
        ..Item::default_spell()
    },
    Item {
        name: "Bursting Fireball",
        id: 0x4027fa60,
        ..Item::default_spell()
    },
    Item {
        name: "Carthus Beacon",
        id: 0x40286f90,
        ..Item::default_spell()
    },
    Item {
        name: "Carthus Flame Arc",
        id: 0x402527b8,
        ..Item::default_spell()
    },
    Item {
        name: "Chaos Bed Vestiges",
        id: 0x402579c0,
        ..Item::default_spell()
    },
    Item {
        name: "Chaos Storm",
        id: 0x40256638,
        ..Item::default_spell()
    },
    Item {
        name: "Fire Orb",
        id: 0x4024a6d0,
        ..Item::default_spell()
    },
    Item {
        name: "Fire Surge",
        id: 0x4024b288,
        ..Item::default_spell()
    },
    Item {
        name: "Fire Whip",
        id: 0x40256a20,
        ..Item::default_spell()
    },
    Item {
        name: "Fireball",
        id: 0x40249f00,
        ..Item::default_spell()
    },
    Item {
        name: "Firestorm",
        id: 0x4024aab8,
        ..Item::default_spell()
    },
    Item {
        name: "Flame Fan",
        id: 0x40258190,
        dlc: 2,
        ..Item::default_spell()
    },
    Item {
        name: "Flash Sweat",
        id: 0x40251818,
        ..Item::default_spell()
    },
    Item {
        name: "Floating Chaos",
        id: 0x40257da8,
        dlc: 1,
        ..Item::default_spell()
    },
    Item {
        name: "Great Chaos Fire Orb",
        id: 0x40256250,
        ..Item::default_spell()
    },
    Item {
        name: "Great Combustion",
        id: 0x4024c9f8,
        ..Item::default_spell()
    },
    Item {
        name: "Iron Flesh",
        id: 0x40251430,
        ..Item::default_spell()
    },
    Item {
        name: "Poison Mist",
        id: 0x4024ed20,
        ..Item::default_spell()
    },
    Item {
        name: "Power Within",
        id: 0x40253b40,
        ..Item::default_spell()
    },
    Item {
        name: "Profaned Flame",
        id: 0x402575d8,
        ..Item::default_spell()
    },
    Item {
        name: "Profuse Sweat",
        id: 0x402717d0,
        ..Item::default_spell()
    },
    Item {
        name: "Rapport",
        id: 0x40252ba0,
        ..Item::default_spell()
    },
    Item {
        name: "Sacred Flame",
        id: 0x40284880,
        ..Item::default_spell()
    },
    Item {
        name: "Seething Chaos",
        id: 0x402896a0,
        dlc: 2,
        ..Item::default_spell()
    },
    Item {
        name: "Toxic Mist",
        id: 0x4024f108,
        ..Item::default_spell()
    },
    Item {
        name: "Warmth",
        id: 0x4025b070,
        ..Item::default_spell()
    },
    Item {
        name: "Atonement",
        id: 0x4039ada0,
        ..Item::default_spell()
    },
    Item {
        name: "Blessed Weapon",
        id: 0x40395f80,
        ..Item::default_spell()
    },
    Item {
        name: "Bountiful Light",
        id: 0x40358338,
        ..Item::default_spell()
    },
    Item {
        name: "Bountiful Sunlight",
        id: 0x40357b68,
        ..Item::default_spell()
    },
    Item {
        name: "Caressing Tears",
        id: 0x40358720,
        ..Item::default_spell()
    },
    Item {
        name: "Dark Blade",
        id: 0x40378ac0,
        ..Item::default_spell()
    },
    Item {
        name: "Darkmoon Blade",
        id: 0x4036cb58,
        ..Item::default_spell()
    },
    Item {
        name: "Dead Again",
        id: 0x40387520,
        ..Item::default_spell()
    },
    Item {
        name: "Deep Protection",
        id: 0x40398690,
        ..Item::default_spell()
    },
    Item {
        name: "Divine Pillars of Light",
        id: 0x4038c340,
        ..Item::default_spell()
    },
    Item {
        name: "Dorhys' Gnawing",
        id: 0x40363eb8,
        ..Item::default_spell()
    },
    Item {
        name: "Emit Force",
        id: 0x4035e4e0,
        ..Item::default_spell()
    },
    Item {
        name: "Force",
        id: 0x4035dd10,
        ..Item::default_spell()
    },
    Item {
        name: "Gnaw",
        id: 0x40363ad0,
        ..Item::default_spell()
    },
    Item {
        name: "Great Heal",
        id: 0x40356fb0,
        ..Item::default_spell()
    },
    Item {
        name: "Great Lightning Spear",
        id: 0x40362f18,
        ..Item::default_spell()
    },
    Item {
        name: "Heal Aid",
        id: 0x403540d0,
        ..Item::default_spell()
    },
    Item {
        name: "Heal",
        id: 0x403567e0,
        ..Item::default_spell()
    },
    Item {
        name: "Homeward",
        id: 0x4035b9e8,
        ..Item::default_spell()
    },
    Item {
        name: "Lifehunt Scythe",
        id: 0x4038ea50,
        ..Item::default_spell()
    },
    Item {
        name: "Lightning Arrow",
        id: 0x40358b08,
        dlc: 2,
        ..Item::default_spell()
    },
    Item {
        name: "Lightning Blade",
        id: 0x4036c770,
        ..Item::default_spell()
    },
    Item {
        name: "Lightning Spear",
        id: 0x40362b30,
        ..Item::default_spell()
    },
    Item {
        name: "Lightning Stake",
        id: 0x40389c30,
        ..Item::default_spell()
    },
    Item {
        name: "Lightning Storm",
        id: 0x403636e8,
        ..Item::default_spell()
    },
    Item {
        name: "Med Heal",
        id: 0x40356bc8,
        ..Item::default_spell()
    },
    Item {
        name: "Projected Heal",
        id: 0x40364688,
        dlc: 2,
        ..Item::default_spell()
    },
    Item {
        name: "Replenishment",
        id: 0x40357780,
        ..Item::default_spell()
    },
    Item {
        name: "Sacred Oath",
        id: 0x40365df8,
        ..Item::default_spell()
    },
    Item {
        name: "Seek Guidance",
        id: 0x40360420,
        ..Item::default_spell()
    },
    Item {
        name: "Soothing Sunlight",
        id: 0x40357398,
        ..Item::default_spell()
    },
    Item {
        name: "Sunlight Spear",
        id: 0x40363300,
        ..Item::default_spell()
    },
    Item {
        name: "Tears of Denial",
        id: 0x4035b600,
        ..Item::default_spell()
    },
    Item {
        name: "Vow of Silence",
        id: 0x4036a448,
        ..Item::default_spell()
    },
    Item {
        name: "Way of White Corona",
        id: 0x403642a0,
        dlc: 1,
        ..Item::default_spell()
    },
    Item {
        name: "Wrath of the Gods",
        id: 0x4035e0f8,
        ..Item::default_spell()
    },
];
