use super::Item;

impl Item {
    const fn default_spell() -> Self {
        Self {
            category: super::Categories::Spells,
            stack_size: 1,
            ..Item::default()
        }
    }
}

pub static SPELLS: &[Item; 106] = &[
    Item {
        id: 31010000,
        name: "Soul Arrow",
        ..Item::default_spell()
    },
    Item {
        id: 31020000,
        name: "Great Soul Arrow",
        ..Item::default_spell()
    },
    Item {
        id: 31030000,
        name: "Heavy Soul Arrow",
        ..Item::default_spell()
    },
    Item {
        id: 31040000,
        name: "Great Heavy Soul Arrow",
        ..Item::default_spell()
    },
    Item {
        id: 31050000,
        name: "Homing Soul Arrow",
        ..Item::default_spell()
    },
    Item {
        id: 31060000,
        name: "Heavy Homing Soul Arrow",
        ..Item::default_spell()
    },
    Item {
        id: 31070000,
        name: "Homing Soulmass",
        ..Item::default_spell()
    },
    Item {
        id: 31080000,
        name: "Homing Crystal Soulmass",
        ..Item::default_spell()
    },
    Item {
        id: 31090000,
        name: "Soul Spear",
        ..Item::default_spell()
    },
    Item {
        id: 31100000,
        name: "Crystal Soul Spear",
        ..Item::default_spell()
    },
    Item {
        id: 31110000,
        name: "Shockwave",
        ..Item::default_spell()
    },
    Item {
        id: 31120000,
        name: "Soul Spear Barrage",
        ..Item::default_spell()
    },
    Item {
        id: 31130000,
        name: "Soul Shower",
        ..Item::default_spell()
    },
    Item {
        id: 31140000,
        name: "Soul Greatsword",
        ..Item::default_spell()
    },
    Item {
        id: 31150000,
        name: "Soul Vortex",
        ..Item::default_spell()
    },
    Item {
        id: 31160000,
        name: "Soul Bolt",
        ..Item::default_spell()
    },
    Item {
        id: 31170000,
        name: "Soul Geyser",
        ..Item::default_spell()
    },
    Item {
        id: 31180000,
        name: "Magic Weapon",
        ..Item::default_spell()
    },
    Item {
        id: 31190000,
        name: "Great Magic Weapon",
        ..Item::default_spell()
    },
    Item {
        id: 31200000,
        name: "Crystal Magic Weapon",
        ..Item::default_spell()
    },
    Item {
        id: 31210000,
        name: "Strong Magic Shield",
        ..Item::default_spell()
    },
    Item {
        id: 31220000,
        name: "Yearn",
        ..Item::default_spell()
    },
    Item {
        id: 31230000,
        name: "Hush",
        ..Item::default_spell()
    },
    Item {
        id: 31240000,
        name: "Fall Control",
        ..Item::default_spell()
    },
    Item {
        id: 31250000,
        name: "Hidden Weapon",
        ..Item::default_spell()
    },
    Item {
        id: 31260000,
        name: "Repair",
        ..Item::default_spell()
    },
    Item {
        id: 31270000,
        name: "Cast Light",
        ..Item::default_spell()
    },
    Item {
        id: 31280000,
        name: "Chameleon",
        ..Item::default_spell()
    },
    Item {
        id: 31290000,
        name: "Unleash Magic",
        ..Item::default_spell()
    },
    Item {
        id: 31300000,
        name: "Soul Flash",
        ..Item::default_spell()
    },
    Item {
        id: 31310000,
        name: "Focus Souls",
        ..Item::default_spell()
    },
    Item {
        id: 32010000,
        name: "Heal",
        ..Item::default_spell()
    },
    Item {
        id: 32020000,
        name: "Med Heal",
        ..Item::default_spell()
    },
    Item {
        id: 32030000,
        name: "Great Heal Excerpt",
        ..Item::default_spell()
    },
    Item {
        id: 32040000,
        name: "Great Heal",
        ..Item::default_spell()
    },
    Item {
        id: 32050000,
        name: "Soothing Sunlight",
        ..Item::default_spell()
    },
    Item {
        id: 32060000,
        name: "Replenishment",
        ..Item::default_spell()
    },
    Item {
        id: 32070000,
        name: "Resplendent Life",
        ..Item::default_spell()
    },
    Item {
        id: 32080000,
        name: "Bountiful Sunlight",
        ..Item::default_spell()
    },
    Item {
        id: 32090000,
        name: "Caressing Prayer",
        ..Item::default_spell()
    },
    Item {
        id: 32100000,
        name: "Force",
        ..Item::default_spell()
    },
    Item {
        id: 32110000,
        name: "Wrath of the Gods",
        ..Item::default_spell()
    },
    Item {
        id: 32120000,
        name: "Emit Force",
        ..Item::default_spell()
    },
    Item {
        id: 32130000,
        name: "Heavenly Thunder",
        ..Item::default_spell()
    },
    Item {
        id: 32140000,
        name: "Lightning Spear",
        ..Item::default_spell()
    },
    Item {
        id: 32150000,
        name: "Great Lightning Spear",
        ..Item::default_spell()
    },
    Item {
        id: 32160000,
        name: "Sunlight Spear",
        ..Item::default_spell()
    },
    Item {
        id: 32170000,
        name: "Soul Appease",
        ..Item::default_spell()
    },
    Item {
        id: 32180000,
        name: "Blinding Bolt",
        ..Item::default_spell()
    },
    Item {
        id: 32190000,
        name: "Magic Barrier",
        ..Item::default_spell()
    },
    Item {
        id: 32200000,
        name: "Great Magic Barrier",
        ..Item::default_spell()
    },
    Item {
        id: 32210000,
        name: "Homeward",
        ..Item::default_spell()
    },
    Item {
        id: 32220000,
        name: "Guidance",
        ..Item::default_spell()
    },
    Item {
        id: 32230000,
        name: "Sacred Oath",
        ..Item::default_spell()
    },
    Item {
        id: 32240000,
        name: "Unveil",
        ..Item::default_spell()
    },
    Item {
        id: 32250000,
        name: "Perseverance",
        ..Item::default_spell()
    },
    Item {
        id: 32260000,
        name: "Sunlight Blade",
        ..Item::default_spell()
    },
    Item {
        id: 32300000,
        name: "Denial",
        ..Item::default_spell()
    },
    Item {
        id: 32310000,
        name: "Splintering Lightning Spear",
        ..Item::default_spell()
    },
    Item {
        id: 33010000,
        name: "Fireball",
        ..Item::default_spell()
    },
    Item {
        id: 33020000,
        name: "Fire Orb",
        ..Item::default_spell()
    },
    Item {
        id: 33030000,
        name: "Great Fireball",
        ..Item::default_spell()
    },
    Item {
        id: 33040000,
        name: "Great Chaos Fireball",
        ..Item::default_spell()
    },
    Item {
        id: 33050000,
        name: "Firestorm",
        ..Item::default_spell()
    },
    Item {
        id: 33060000,
        name: "Fire Tempest",
        ..Item::default_spell()
    },
    Item {
        id: 33070000,
        name: "Chaos Storm",
        ..Item::default_spell()
    },
    Item {
        id: 33080000,
        name: "Combustion",
        ..Item::default_spell()
    },
    Item {
        id: 33090000,
        name: "Great Combustion",
        ..Item::default_spell()
    },
    Item {
        id: 33100000,
        name: "Fire Whip",
        ..Item::default_spell()
    },
    Item {
        id: 33110000,
        name: "Poison Mist",
        ..Item::default_spell()
    },
    Item {
        id: 33120000,
        name: "Toxic Mist",
        ..Item::default_spell()
    },
    Item {
        id: 33130000,
        name: "Acid Surge",
        ..Item::default_spell()
    },
    Item {
        id: 33140000,
        name: "Lingering Flame",
        ..Item::default_spell()
    },
    Item {
        id: 33150000,
        name: "Flame Swathe",
        ..Item::default_spell()
    },
    Item {
        id: 33160000,
        name: "Forbidden Sun",
        ..Item::default_spell()
    },
    Item {
        id: 33170000,
        name: "Flame Weapon",
        ..Item::default_spell()
    },
    Item {
        id: 33180000,
        name: "Flash Sweat",
        ..Item::default_spell()
    },
    Item {
        id: 33190000,
        name: "Iron Flesh",
        ..Item::default_spell()
    },
    Item {
        id: 33200000,
        name: "Warmth",
        ..Item::default_spell()
    },
    Item {
        id: 33210000,
        name: "Immolation",
        ..Item::default_spell()
    },
    Item {
        id: 33300000,
        name: "Fire Snake",
        ..Item::default_spell()
    },
    Item {
        id: 33310000,
        name: "Dance of Fire",
        ..Item::default_spell()
    },
    Item {
        id: 33320000,
        name: "Outcry",
        ..Item::default_spell()
    },
    Item {
        id: 34010000,
        name: "Dark Orb",
        ..Item::default_spell()
    },
    Item {
        id: 34020000,
        name: "Dark Hail",
        ..Item::default_spell()
    },
    Item {
        id: 34030000,
        name: "Dark Fog",
        ..Item::default_spell()
    },
    Item {
        id: 34040000,
        name: "Affinity",
        ..Item::default_spell()
    },
    Item {
        id: 34050000,
        name: "Dead Again",
        ..Item::default_spell()
    },
    Item {
        id: 34060000,
        name: "Dark Weapon",
        ..Item::default_spell()
    },
    Item {
        id: 34070000,
        name: "Whisper of Despair",
        ..Item::default_spell()
    },
    Item {
        id: 34080000,
        name: "Repel",
        ..Item::default_spell()
    },
    Item {
        id: 34090000,
        name: "Twisted Barricade",
        ..Item::default_spell()
    },
    Item {
        id: 34100000,
        name: "Numbness",
        ..Item::default_spell()
    },
    Item {
        id: 34300000,
        name: "Dark Greatsword",
        ..Item::default_spell()
    },
    Item {
        id: 34310000,
        name: "Recollection",
        ..Item::default_spell()
    },
    Item {
        id: 35010000,
        name: "Scraps of Life",
        ..Item::default_spell()
    },
    Item {
        id: 35020000,
        name: "Darkstorm",
        ..Item::default_spell()
    },
    Item {
        id: 35030000,
        name: "Resonant Soul",
        ..Item::default_spell()
    },
    Item {
        id: 35040000,
        name: "Great Resonant Soul",
        ..Item::default_spell()
    },
    Item {
        id: 35050000,
        name: "Climax",
        ..Item::default_spell()
    },
    Item {
        id: 35060000,
        name: "Resonant Flesh",
        ..Item::default_spell()
    },
    Item {
        id: 35070000,
        name: "Resonant Weapon",
        ..Item::default_spell()
    },
    Item {
        id: 35080000,
        name: "Lifedrain Patch",
        ..Item::default_spell()
    },
    Item {
        id: 35090000,
        name: "Profound Still",
        ..Item::default_spell()
    },
    Item {
        id: 35300000,
        name: "Promised Walk of Peace",
        ..Item::default_spell()
    },
    Item {
        id: 35310000,
        name: "Dark Dance",
        ..Item::default_spell()
    },
];