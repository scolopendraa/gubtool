use super::Item;

impl Item {
    const fn default_sorceries() -> Self {
        Self {
            category: super::Categories::Sorceries,
            max_storage: 600,
            ..Item::default()
        }
    }
}

pub static SORCERIES: [Item; 84] = [
    Item {
        id: 0x4000114f,
        name: "Adula's Moonblade",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400011f8,
        name: "Ambush Shard",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001389,
        name: "Ancient Death Rancor",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e96dc,
        name: "Blades of Stone",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000132e,
        name: "Briars of Punishment",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001324,
        name: "Briars of Sin",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000ff0,
        name: "Cannon of Haima",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000114e,
        name: "Carian Greatsword",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400010cd,
        name: "Carian Phalanx",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001162,
        name: "Carian Piercer",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001220,
        name: "Carian Retaliation",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001158,
        name: "Carian Slicer",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401ea17c,
        name: "Cherishing Fingers",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001271,
        name: "Collapsing Stars",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000fb5,
        name: "Comet",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001068,
        name: "Comet Azur",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000fdc,
        name: "Crystal Barrage",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000ffa,
        name: "Crystal Burst",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000119e,
        name: "Crystal Release",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400011a8,
        name: "Crystal Torrent",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000122a,
        name: "Eternal Darkness",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001392,
        name: "Explosive Ghostflame",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000139c,
        name: "Fia's Mist",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401ea172,
        name: "Fleeting Microcosm",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001072,
        name: "Founding Rain of Stars",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001144,
        name: "Freezing Mist",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000118a,
        name: "Frozen Armament",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001018,
        name: "Gavel of Haima",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400012ca,
        name: "Gelmir's Fury",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400010cc,
        name: "Glintblade Phalanx",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e9556,
        name: "Glintblade Trio",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000fe6,
        name: "Glintstone Arc",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000fb4,
        name: "Glintstone Cometshard",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001130,
        name: "Glintstone Icecrag",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e9614,
        name: "Glintstone Nail",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e961e,
        name: "Glintstone Nails",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000fa0,
        name: "Glintstone Pebble",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000fc8,
        name: "Glintstone Stars",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e96e6,
        name: "Gravitational Missile",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001270,
        name: "Gravity Well",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000fa1,
        name: "Great Glintstone Shard",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400013f6,
        name: "Great Oracular Bubble",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400010ce,
        name: "Greatblade Phalanx",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e97ae,
        name: "Impenetrable Thorns",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000111c,
        name: "Loretta's Greatbow",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000111d,
        name: "Loretta's Mastery",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001180,
        name: "Lucidity",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001112,
        name: "Magic Downpour",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001126,
        name: "Magic Glintblade",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400012c0,
        name: "Magma Shot",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e97a4,
        name: "Mantle of Thorns",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e9cc2,
        name: "Mass of Putrescence",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000125c,
        name: "Meteorite",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000125d,
        name: "Meteorite of Astel",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e954c,
        name: "Miriam's Vanishing",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000120c,
        name: "Night Comet",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001964,
        name: "Night Maiden's Mist",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001202,
        name: "Night Shard",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400013ec,
        name: "Oracle Bubbles",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001388,
        name: "Rancorcall",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001109,
        name: "Ranni's Dark Moon",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e9560,
        name: "Rellana's Twin Moons",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001108,
        name: "Rennala's Full Moon",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e9808,
        name: "Rings of Spectral Light",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000100e,
        name: "Rock Blaster",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001266,
        name: "Rock Sling",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400012d4,
        name: "Roiling Magma",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400012de,
        name: "Rykard's Rancor",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000116c,
        name: "Scholar's Armament",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001176,
        name: "Scholar's Shield",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000fbe,
        name: "Shard Spiral",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001004,
        name: "Shatter Earth",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001194,
        name: "Shattering Crystal",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000fd2,
        name: "Star Shower",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000102c,
        name: "Starlight",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000107c,
        name: "Stars of Ruin",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000faa,
        name: "Swift Glintstone Shard",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001022,
        name: "Terra Magica",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001216,
        name: "Thops's Barrier",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400013a6,
        name: "Tibia's Summons",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001234,
        name: "Unseen Blade",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000123e,
        name: "Unseen Form",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401e9cb8,
        name: "Vortex of Putrescence",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000113a,
        name: "Zamor Ice Storm",
        ..Item::default_sorceries()
    },
];
