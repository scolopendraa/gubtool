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
        id: 0x40003c3c,
        name: "Albinauric Bloodclot",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050c9,
        name: "Altus Bloom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050d3,
        name: "Arteria Leaf",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003ab6,
        name: "Beast Blood",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ebf18,
        name: "Beast Horn",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003aa2,
        name: "Beast Liver",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2a6,
        name: "Black Pyrefly",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2bf,
        name: "Blessed Bone Shard",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000515f,
        name: "Blood-Tainted Excrement",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050f3,
        name: "Bloodrose",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005169,
        name: "Budding Cave Moss",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003aca,
        name: "Budding Horn",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005168,
        name: "Cave Moss",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2ba,
        name: "Congealed Putrescence",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003b1a,
        name: "Crab Eggs",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000512c,
        name: "Cracked Crystal",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000510e,
        name: "Crystal Bud",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000516a,
        name: "Crystal Cave Moss",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2ae,
        name: "Deep-Purple Lily",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2a5,
        name: "Dewgem",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050e6,
        name: "Dewkissed Herba",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2b7,
        name: "Dragon's Calorbloom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2be,
        name: "Ember of Messmer",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2b9,
        name: "Empyrean-Blood Burgeon",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050c8,
        name: "Erdleaf Flower",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005104,
        name: "Eye of Yelough",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050b4,
        name: "Faded Erdleaf Flower",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2b8,
        name: "Finger Mimic",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050ca,
        name: "Fire Blossom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003ad4,
        name: "Flight Pinion",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ebf40,
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
        id: 0x40003ae8,
        name: "Four-Toed Fowl Foot",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2ad,
        name: "Frozen Maggot",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050ac,
        name: "Fulgurbloom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2c3,
        name: "Furnace Visage",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2b4,
        name: "Gas Stone",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2b5,
        name: "Ghostflame Bloom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2a7,
        name: "Glintslab Firefly",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000514c,
        name: "Glintstone Firefly",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000514b,
        name: "Gold Firefly",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000515e,
        name: "Gold-Tinged Excrement",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005154,
        name: "Golden Centipede",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050f1,
        name: "Golden Rowa",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050cb,
        name: "Golden Sunflower",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2b6,
        name: "Grave Cricket",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2b2,
        name: "Grave Keeper's Brainpan",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050ae,
        name: "Grave Violet",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005177,
        name: "Gravel Stone",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003b06,
        name: "Great Dragonfly Head",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003bed,
        name: "Hefty Beast Bone",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050d2,
        name: "Herba",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2b3,
        name: "Horn-Strewn Excrement",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003afc,
        name: "Human Bone Shard",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2a4,
        name: "Knot Resin",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003b24,
        name: "Land Octopus Ovary",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003c32,
        name: "Living Jar Shard",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003aac,
        name: "Lump of Flesh",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005119,
        name: "Melted Mushroom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050ad,
        name: "Miquella's Lily",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003b2e,
        name: "Miranda Powder",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005118,
        name: "Mushroom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2c1,
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
        id: 0x401ed2af,
        name: "Nectarblood Burgeon",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003ac0,
        name: "Old Fang",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ebf2c,
        name: "Pearlescent Scale",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050aa,
        name: "Poisonbloom",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2a1,
        name: "Rada Fruit",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2bd,
        name: "Rauh Burrow",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2c0,
        name: "Red Fulgurbloom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2a2,
        name: "Redflesh Mushroom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000510f,
        name: "Rimed Crystal Bud",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050f2,
        name: "Rimed Rowa",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005127,
        name: "Root Resin",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2bb,
        name: "Roundrock",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050f0,
        name: "Rowa Fruit",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400006e0,
        name: "Ruin Fragment",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005111,
        name: "Sacramental Bud",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000513b,
        name: "Sanctuary Stone",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2ac,
        name: "Sanguine Amaryllis",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2ab,
        name: "Scarlet Bud",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ebf36,
        name: "Scorpion Liver",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2a9,
        name: "Shadow Sunflower",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2c2,
        name: "Sharp Gravel Stone",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000514a,
        name: "Silver Firefly",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005159,
        name: "Silver Tear Husk",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003a98,
        name: "Sliver of Meat",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003b10,
        name: "Slumbering Egg",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005142,
        name: "Smoldering Butterfly",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ebf22,
        name: "Spirit Calculus",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2bc,
        name: "Spiritgrave Stone",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003c46,
        name: "Stormhawk Feather",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003c28,
        name: "String",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003b38,
        name: "Strip of White Flesh",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2b1,
        name: "Swollen Grape",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x400050cd,
        name: "Tarnished Golden Sunflower",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003bec,
        name: "Thin Beast Bones",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2aa,
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
        id: 0x400050ab,
        name: "Trina's Lily",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40003af2,
        name: "Turtle Neck Meat",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x40005172,
        name: "Volcanic Stone",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2a3,
        name: "Whiteflesh Mushroom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2b0,
        name: "Winter-Lantern Fly",
        dlc: true,
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x4000516d,
        name: "Yellow Ember",
        ..Item::default_crafting_materials()
    },
    Item {
        id: 0x401ed2a8,
        name: "Yellow Fulgurbloom",
        dlc: true,
        ..Item::default_crafting_materials()
    },
];
