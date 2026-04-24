use super::Item;

impl Item {
    const fn default_crafting_materials() -> Self {
        Self {
            category: super::Categories::CraftingMaterials,
            stack_size: 999,
            max_storage: 999,
            ..Item::default()
        }
    }
}

pub static CRAFTING_MATERIALS: [Item; 108] = [
    Item {
        id: 0x40005141,
        name: "Aeonian Butterfly",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003C3C,
        name: "Albinauric Bloodclot",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050C9,
        name: "Altus Bloom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050D3,
        name: "Arteria Leaf",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003AB6,
        name: "Beast Blood",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401EBF18,
        name: "Beast Horn",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003AA2,
        name: "Beast Liver",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2A6,
        name: "Black Pyrefly",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2BF,
        name: "Blessed Bone Shard",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000515F,
        name: "Blood-Tainted Excrement",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050F3,
        name: "Bloodrose",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005169,
        name: "Budding Cave Moss",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003ACA,
        name: "Budding Horn",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005168,
        name: "Cave Moss",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2BA,
        name: "Congealed Putrescence",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003B1A,
        name: "Crab Eggs",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000512C,
        name: "Cracked Crystal",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000510E,
        name: "Crystal Bud",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000516A,
        name: "Crystal Cave Moss",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2AE,
        name: "Deep-Purple Lily",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2A5,
        name: "Dewgem",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050E6,
        name: "Dewkissed Herba",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2B7,
        name: "Dragon's Calorbloom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2BE,
        name: "Ember of Messmer",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2B9,
        name: "Empyrean-Blood Burgeon",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050C8,
        name: "Erdleaf Flower",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005104,
        name: "Eye of Yelough",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050B4,
        name: "Faded Erdleaf Flower",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2B8,
        name: "Finger Mimic",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050CA,
        name: "Fire Blossom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003AD4,
        name: "Flight Pinion",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401EBF40,
        name: "Fly Mold",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005174,
        name: "Formic Rock",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003AE8,
        name: "Four-Toed Fowl Foot",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2AD,
        name: "Frozen Maggot",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050AC,
        name: "Fulgurbloom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2C3,
        name: "Furnace Visage",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2B4,
        name: "Gas Stone",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2B5,
        name: "Ghostflame Bloom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2A7,
        name: "Glintslab Firefly",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000514C,
        name: "Glintstone Firefly",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000514B,
        name: "Gold Firefly",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000515E,
        name: "Gold-Tinged Excrement",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005154,
        name: "Golden Centipede",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050F1,
        name: "Golden Rowa",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050CB,
        name: "Golden Sunflower",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2B6,
        name: "Grave Cricket",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2B2,
        name: "Grave Keeper's Brainpan",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050AE,
        name: "Grave Violet",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005177,
        name: "Gravel Stone",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003B06,
        name: "Great Dragonfly Head",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003BED,
        name: "Hefty Beast Bone",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050D2,
        name: "Herba",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2B3,
        name: "Horn-Strewn Excrement",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003AFC,
        name: "Human Bone Shard",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2A4,
        name: "Knot Resin",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003B24,
        name: "Land Octopus Ovary",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003C32,
        name: "Living Jar Shard",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003AAC,
        name: "Lump of Flesh",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005119,
        name: "Melted Mushroom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050AD,
        name: "Miquella's Lily",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003B2E,
        name: "Miranda Powder",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005118,
        name: "Mushroom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2C1,
        name: "Nailstone",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005140,
        name: "Nascent Butterfly",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2AF,
        name: "Nectarblood Burgeon",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003AC0,
        name: "Old Fang",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401EBF2C,
        name: "Pearlescent Scale",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050AA,
        name: "Poisonbloom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2A1,
        name: "Rada Fruit",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2BD,
        name: "Rauh Burrow",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2C0,
        name: "Red Fulgurbloom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2A2,
        name: "Redflesh Mushroom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000510F,
        name: "Rimed Crystal Bud",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050F2,
        name: "Rimed Rowa",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005127,
        name: "Root Resin",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2BB,
        name: "Roundrock",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050F0,
        name: "Rowa Fruit",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400006E0,
        name: "Ruin Fragment",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005111,
        name: "Sacramental Bud",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000513B,
        name: "Sanctuary Stone",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2AC,
        name: "Sanguine Amaryllis",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2AB,
        name: "Scarlet Bud",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401EBF36,
        name: "Scorpion Liver",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2A9,
        name: "Shadow Sunflower",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2C2,
        name: "Sharp Gravel Stone",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000514A,
        name: "Silver Firefly",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005159,
        name: "Silver Tear Husk",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003A98,
        name: "Sliver of Meat",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003B10,
        name: "Slumbering Egg",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005142,
        name: "Smoldering Butterfly",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401EBF22,
        name: "Spirit Calculus",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2BC,
        name: "Spiritgrave Stone",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003C46,
        name: "Stormhawk Feather",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003C28,
        name: "String",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003B38,
        name: "Strip of White Flesh",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2B1,
        name: "Swollen Grape",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050CD,
        name: "Tarnished Golden Sunflower",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003BEC,
        name: "Thin Beast Bones",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2AA,
        name: "Toxic Mossling",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005122,
        name: "Toxic Mushroom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050AB,
        name: "Trina's Lily",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003AF2,
        name: "Turtle Neck Meat",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005172,
        name: "Volcanic Stone",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2A3,
        name: "Whiteflesh Mushroom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2B0,
        name: "Winter-Lantern Fly",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000516D,
        name: "Yellow Ember",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ED2A8,
        name: "Yellow Fulgurbloom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
];
