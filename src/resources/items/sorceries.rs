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
        id: 0x4000114F,
        name: "Adula's Moonblade",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400011F8,
        name: "Ambush Shard",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001389,
        name: "Ancient Death Rancor",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401E96DC,
        name: "Blades of Stone",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000132E,
        name: "Briars of Punishment",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001324,
        name: "Briars of Sin",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000FF0,
        name: "Cannon of Haima",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000114E,
        name: "Carian Greatsword",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400010CD,
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
        id: 0x401EA17C,
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
        id: 0x40000FB5,
        name: "Comet",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001068,
        name: "Comet Azur",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000FDC,
        name: "Crystal Barrage",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000FFA,
        name: "Crystal Burst",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000119E,
        name: "Crystal Release",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400011A8,
        name: "Crystal Torrent",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000122A,
        name: "Eternal Darkness",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001392,
        name: "Explosive Ghostflame",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000139C,
        name: "Fia's Mist",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401EA172,
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
        id: 0x4000118A,
        name: "Frozen Armament",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001018,
        name: "Gavel of Haima",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400012CA,
        name: "Gelmir's Fury",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400010CC,
        name: "Glintblade Phalanx",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401E9556,
        name: "Glintblade Trio",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000FE6,
        name: "Glintstone Arc",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000FB4,
        name: "Glintstone Cometshard",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001130,
        name: "Glintstone Icecrag",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401E9614,
        name: "Glintstone Nail",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401E961E,
        name: "Glintstone Nails",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000FA0,
        name: "Glintstone Pebble",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000FC8,
        name: "Glintstone Stars",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401E96E6,
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
        id: 0x40000FA1,
        name: "Great Glintstone Shard",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400013F6,
        name: "Great Oracular Bubble",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400010CE,
        name: "Greatblade Phalanx",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401E97AE,
        name: "Impenetrable Thorns",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000111C,
        name: "Loretta's Greatbow",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000111D,
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
        id: 0x400012C0,
        name: "Magma Shot",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401E97A4,
        name: "Mantle of Thorns",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401E9CC2,
        name: "Mass of Putrescence",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000125C,
        name: "Meteorite",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000125D,
        name: "Meteorite of Astel",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401E954C,
        name: "Miriam's Vanishing",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000120C,
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
        id: 0x400013EC,
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
        id: 0x401E9560,
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
        id: 0x401E9808,
        name: "Rings of Spectral Light",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000100E,
        name: "Rock Blaster",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001266,
        name: "Rock Sling",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400012D4,
        name: "Roiling Magma",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x400012DE,
        name: "Rykard's Rancor",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000116C,
        name: "Scholar's Armament",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001176,
        name: "Scholar's Shield",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000FBE,
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
        id: 0x40000FD2,
        name: "Star Shower",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000102C,
        name: "Starlight",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000107C,
        name: "Stars of Ruin",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40000FAA,
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
        id: 0x400013A6,
        name: "Tibia's Summons",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x40001234,
        name: "Unseen Blade",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000123E,
        name: "Unseen Form",
        ..Item::default_sorceries()
    },
    Item {
        id: 0x401E9CB8,
        name: "Vortex of Putrescence",
        dlc: true,
        ..Item::default_sorceries()
    },
    Item {
        id: 0x4000113A,
        name: "Zamor Ice Storm",
        ..Item::default_sorceries()
    },
];
