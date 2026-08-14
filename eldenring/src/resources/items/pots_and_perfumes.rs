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
        id: 0x40000e1a,
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
        id: 0x401e8764,
        name: "Eternal Sleep Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000014a,
        name: "Fetid Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000012c,
        name: "Fire Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000168,
        name: "Freezing Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e8746,
        name: "Frenzied Flame Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000012e,
        name: "Giantsflame Fire Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e85ca,
        name: "Hefty Fetid Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e85ac,
        name: "Hefty Fire Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e85d4,
        name: "Hefty Fly Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e85e8,
        name: "Hefty Freezing Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e86ec,
        name: "Hefty Frenzied Flame Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e8728,
        name: "Hefty Furnace Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e85c0,
        name: "Hefty Lightning Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e8714,
        name: "Hefty Magic Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e85fc,
        name: "Hefty Oil Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e85f2,
        name: "Hefty Poison Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e870a,
        name: "Hefty Rancor Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e8732,
        name: "Hefty Red Lightning Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e85b6,
        name: "Hefty Rock Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e871e,
        name: "Hefty Rot Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e86d8,
        name: "Hefty Volcano Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000015e,
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
        id: 0x4000017c,
        name: "Oil Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000172,
        name: "Poison Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000dfc,
        name: "Poison Spraymist",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000028a,
        name: "Rancor Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e873c,
        name: "Red Lightning Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000012d,
        name: "Redmane Fire Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000251d,
        name: "Ritual Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001ae,
        name: "Roped Fetid Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000190,
        name: "Roped Fire Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001d6,
        name: "Roped Fly Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x401e8778,
        name: "Roped Frenzied Flame Pot",
        dlc: true,
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001fe,
        name: "Roped Holy Water Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001a4,
        name: "Roped Lightning Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001cc,
        name: "Roped Magic Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001c2,
        name: "Roped Oil Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001b8,
        name: "Roped Poison Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x400001ea,
        name: "Roped Volcano Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000029e,
        name: "Rot Pot",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x4000015f,
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
        id: 0x40000dde,
        name: "Bloodboil Aromatic",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000dc0,
        name: "Ironjar Aromatic",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000db6,
        name: "Spark Aromatic",
        ..Item::default_pots_and_perfumes()
    },
    Item {
        id: 0x40000dac,
        name: "Uplifting Aromatic",
        ..Item::default_pots_and_perfumes()
    },
];
