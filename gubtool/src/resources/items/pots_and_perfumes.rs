use super::Item;

impl Item {
    const fn default_pots_and_perfumes() -> Self {
        Self {
            category: super::Categories::PotsAndPerfumes,
            stack_size: 10,
            max_storage: 600,
            ..Item::default()
        }
    }
}

pub static POTS_AND_PERFUMES: [Item; 57] = [
    Item {
        id: 0x40000295,
        name: "Academy Magic Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000E1A,
        name: "Acid Spraymist",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000262,
        name: "Albinauric Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000186,
        name: "Alluring Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000141,
        name: "Ancient Dragonbolt Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000187,
        name: "Beastlure Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000276,
        name: "Cursed-Blood Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E8764,
        name: "Eternal Sleep Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000014A,
        name: "Fetid Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000012C,
        name: "Fire Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000168,
        name: "Freezing Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E8746,
        name: "Frenzied Flame Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000012E,
        name: "Giantsflame Fire Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E85CA,
        name: "Hefty Fetid Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E85AC,
        name: "Hefty Fire Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E85D4,
        name: "Hefty Fly Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E85E8,
        name: "Hefty Freezing Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E86EC,
        name: "Hefty Frenzied Flame Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E8728,
        name: "Hefty Furnace Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E85C0,
        name: "Hefty Lightning Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E8714,
        name: "Hefty Magic Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E85FC,
        name: "Hefty Oil Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E85F2,
        name: "Hefty Poison Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E870A,
        name: "Hefty Rancor Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E8732,
        name: "Hefty Red Lightning Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E85B6,
        name: "Hefty Rock Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E871E,
        name: "Hefty Rot Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E86D8,
        name: "Hefty Volcano Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000015E,
        name: "Holy Water Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000140,
        name: "Lightning Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000294,
        name: "Magic Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000017C,
        name: "Oil Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000172,
        name: "Poison Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000DFC,
        name: "Poison Spraymist",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000028A,
        name: "Rancor Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E873C,
        name: "Red Lightning Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000012D,
        name: "Redmane Fire Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000251D,
        name: "Ritual Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001AE,
        name: "Roped Fetid Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000190,
        name: "Roped Fire Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001D6,
        name: "Roped Fly Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401E8778,
        name: "Roped Frenzied Flame Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001FE,
        name: "Roped Holy Water Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001A4,
        name: "Roped Lightning Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001CC,
        name: "Roped Magic Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001C2,
        name: "Roped Oil Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001B8,
        name: "Roped Poison Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001EA,
        name: "Roped Volcano Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000029E,
        name: "Rot Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000015F,
        name: "Sacred Order Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000280,
        name: "Sleep Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000154,
        name: "Swarm Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000258,
        name: "Volcano Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000DDE,
        name: "Bloodboil Aromatic",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000DC0,
        name: "Ironjar Aromatic",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000DB6,
        name: "Spark Aromatic",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000DAC,
        name: "Uplifting Aromatic",
        ..Item::default_pots_and_perfumes()
    },
];
