use super::Item;

impl Item {
    const fn default_ring() -> Self {
        Self {
            category: super::ItemCategory::Rings,
            stack_size: 1,
            max_storage: 1,
            ..Item::default()
        }
    }
}

pub static RINGS: [Item; 118] = [
    Item {
        name: "Aldrich's Ruby",
        id: 0x2000508c,
        ..Item::default_ring()
    },
    Item {
        name: "Aldrich's Sapphire",
        id: 0x20005096,
        ..Item::default_ring()
    },
    Item {
        name: "Ashen Estus Ring",
        id: 0x200050e6,
        ..Item::default_ring()
    },
    Item {
        name: "Baneful Bird Ring",
        id: 0x20005032,
        ..Item::default_ring()
    },
    Item {
        name: "Bellowing Dragoncrest Ring",
        id: 0x20004f07,
        ..Item::default_ring()
    },
    Item {
        name: "Bloodbite Ring+1",
        id: 0x20004e85,
        ..Item::default_ring()
    },
    Item {
        name: "Bloodbite Ring",
        id: 0x20004e84,
        ..Item::default_ring()
    },
    Item {
        name: "Blue Tearstone Ring",
        id: 0x20004ed4,
        ..Item::default_ring()
    },
    Item {
        name: "Calamity Ring",
        id: 0x20005078,
        ..Item::default_ring()
    },
    Item {
        name: "Carthus Bloodring",
        id: 0x200050fa,
        ..Item::default_ring()
    },
    Item {
        name: "Carthus Milkring",
        id: 0x20004fe2,
        ..Item::default_ring()
    },
    Item {
        name: "Chillbite Ring",
        id: 0x20005208,
        dlc: 1,
        ..Item::default_ring()
    },
    Item {
        name: "Chloranthy Ring",
        id: 0x20004e2a,
        ..Item::default_ring()
    },
    Item {
        name: "Chloranthy Ring+1",
        id: 0x20004e2b,
        ..Item::default_ring()
    },
    Item {
        name: "Chloranthy Ring+2",
        id: 0x20004e2c,
        ..Item::default_ring()
    },
    Item {
        name: "Chloranthy Ring+3",
        id: 0x20004e2d,
        dlc: 2,
        ..Item::default_ring()
    },
    Item {
        name: "Covetous Gold Serpent Ring",
        id: 0x20004fa6,
        ..Item::default_ring()
    },
    Item {
        name: "Covetous Gold Serpent Ring+1",
        id: 0x20004fa7,
        ..Item::default_ring()
    },
    Item {
        name: "Covetous Gold Serpent Ring+2",
        id: 0x20004fa8,
        ..Item::default_ring()
    },
    Item {
        name: "Covetous Gold Serpent Ring+3",
        id: 0x20004fa9,
        dlc: 2,
        ..Item::default_ring()
    },
    Item {
        name: "Covetous Silver Serpent Ring",
        id: 0x20004fb0,
        ..Item::default_ring()
    },
    Item {
        name: "Covetous Silver Serpent Ring+1",
        id: 0x20004fb1,
        ..Item::default_ring()
    },
    Item {
        name: "Covetous Silver Serpent Ring+2",
        id: 0x20004fb2,
        ..Item::default_ring()
    },
    Item {
        name: "Covetous Silver Serpent Ring+3",
        id: 0x20004fb3,
        dlc: 2,
        ..Item::default_ring()
    },
    Item {
        name: "Cursebite Ring",
        id: 0x20004e98,
        ..Item::default_ring()
    },
    Item {
        name: "Dark Clutch Ring",
        id: 0x20005028,
        ..Item::default_ring()
    },
    Item {
        name: "Dark Stoneplate Ring",
        id: 0x20004e70,
        ..Item::default_ring()
    },
    Item {
        name: "Dark Stoneplate Ring+1",
        id: 0x20004e71,
        ..Item::default_ring()
    },
    Item {
        name: "Dark Stoneplate Ring+2",
        id: 0x20004e72,
        ..Item::default_ring()
    },
    Item {
        name: "Darkmoon Blade Covenant Ring",
        id: 0x20004f7e,
        ..Item::default_ring()
    },
    Item {
        name: "Darkmoon Ring",
        id: 0x20004f6a,
        ..Item::default_ring()
    },
    Item {
        name: "Deep Ring",
        id: 0x20004f60,
        ..Item::default_ring()
    },
    Item {
        name: "Dragonscale Ring",
        id: 0x2000515e,
        ..Item::default_ring()
    },
    Item {
        name: "Dusk Crown Ring",
        id: 0x20004f4c,
        ..Item::default_ring()
    },
    Item {
        name: "Estus Ring",
        id: 0x200050dc,
        ..Item::default_ring()
    },
    Item {
        name: "Farron Ring",
        id: 0x20005082,
        ..Item::default_ring()
    },
    Item {
        name: "Fire Clutch Ring",
        id: 0x2000501e,
        ..Item::default_ring()
    },
    Item {
        name: "Flame Stoneplate Ring",
        id: 0x20004e52,
        ..Item::default_ring()
    },
    Item {
        name: "Flame Stoneplate Ring+1",
        id: 0x20004e53,
        ..Item::default_ring()
    },
    Item {
        name: "Flame Stoneplate Ring+2",
        id: 0x20004e54,
        ..Item::default_ring()
    },
    Item {
        name: "Fleshbite Ring+1",
        id: 0x20004ea3,
        ..Item::default_ring()
    },
    Item {
        name: "Fleshbite Ring",
        id: 0x20004ea2,
        ..Item::default_ring()
    },
    Item {
        name: "Flynn's Ring",
        id: 0x2000503c,
        ..Item::default_ring()
    },
    Item {
        name: "Great Swamp Ring",
        id: 0x20004f10,
        ..Item::default_ring()
    },
    Item {
        name: "Havel's Ring",
        id: 0x20004e34,
        ..Item::default_ring()
    },
    Item {
        name: "Havel's Ring+1",
        id: 0x20004e35,
        ..Item::default_ring()
    },
    Item {
        name: "Havel's Ring+2",
        id: 0x20004e36,
        ..Item::default_ring()
    },
    Item {
        name: "Havel's Ring+3",
        id: 0x20004e37,
        dlc: 2,
        ..Item::default_ring()
    },
    Item {
        name: "Hawk Ring",
        id: 0x20004f92,
        ..Item::default_ring()
    },
    Item {
        name: "Hornet Ring",
        id: 0x20004f9c,
        ..Item::default_ring()
    },
    Item {
        name: "Horsehoof Ring",
        id: 0x200050f0,
        ..Item::default_ring()
    },
    Item {
        name: "Hunter's Ring",
        id: 0x20004ff6,
        ..Item::default_ring()
    },
    Item {
        name: "Knight Slayer's Ring",
        id: 0x20005000,
        ..Item::default_ring()
    },
    Item {
        name: "Knight's Ring",
        id: 0x20004fec,
        ..Item::default_ring()
    },
    Item {
        name: "Leo Ring",
        id: 0x20004ee8,
        ..Item::default_ring()
    },
    Item {
        name: "Life Ring",
        id: 0x20004e20,
        ..Item::default_ring()
    },
    Item {
        name: "Life Ring+1",
        id: 0x20004e21,
        ..Item::default_ring()
    },
    Item {
        name: "Life Ring+2",
        id: 0x20004e22,
        ..Item::default_ring()
    },
    Item {
        name: "Life Ring+3",
        id: 0x20004e23,
        ..Item::default_ring()
    },
    Item {
        name: "Lightning Clutch Ring",
        id: 0x20005014,
        ..Item::default_ring()
    },
    Item {
        name: "Lingering Dragoncrest Ring",
        id: 0x20004f2e,
        ..Item::default_ring()
    },
    Item {
        name: "Lingering Dragoncrest Ring+1",
        id: 0x20004f2f,
        ..Item::default_ring()
    },
    Item {
        name: "Lingering Dragoncrest Ring+2",
        id: 0x20004f30,
        ..Item::default_ring()
    },
    Item {
        name: "Lloyd's Shield Ring",
        id: 0x200050be,
        ..Item::default_ring()
    },
    Item {
        name: "Lloyd's Sword Ring",
        id: 0x200050b4,
        ..Item::default_ring()
    },
    Item {
        name: "Magic Clutch Ring",
        id: 0x2000500a,
        ..Item::default_ring()
    },
    Item {
        name: "Magic Stoneplate Ring",
        id: 0x20004e66,
        ..Item::default_ring()
    },
    Item {
        name: "Magic Stoneplate Ring+1",
        id: 0x20004e67,
        ..Item::default_ring()
    },
    Item {
        name: "Magic Stoneplate Ring+2",
        id: 0x20004e68,
        ..Item::default_ring()
    },
    Item {
        name: "Morne's Ring",
        id: 0x20004f1a,
        ..Item::default_ring()
    },
    Item {
        name: "Obscuring Ring",
        id: 0x20005064,
        ..Item::default_ring()
    },
    Item {
        name: "Poisonbite Ring+1",
        id: 0x20004e8f,
        ..Item::default_ring()
    },
    Item {
        name: "Poisonbite Ring",
        id: 0x20004e8e,
        ..Item::default_ring()
    },
    Item {
        name: "Pontiff's Left Eye",
        id: 0x20005136,
        ..Item::default_ring()
    },
    Item {
        name: "Pontiff's Right Eye",
        id: 0x2000510e,
        ..Item::default_ring()
    },
    Item {
        name: "Priestess Ring",
        id: 0x20004ec0,
        ..Item::default_ring()
    },
    Item {
        name: "Prisoner's Chain",
        id: 0x20005046,
        ..Item::default_ring()
    },
    Item {
        name: "Red Tearstone Ring",
        id: 0x20004eca,
        ..Item::default_ring()
    },
    Item {
        name: "Reversal Ring",
        id: 0x20005104,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of Favor",
        id: 0x20004e3e,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of Favor+1",
        id: 0x20004e3f,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of Favor+2",
        id: 0x20004e40,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of Favor+3",
        id: 0x20004e41,
        dlc: 2,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of Sacrifice",
        id: 0x20004ef2,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of Steel Protection",
        id: 0x20004e48,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of Steel Protection+1",
        id: 0x20004e49,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of Steel Protection+2",
        id: 0x20004e4a,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of Steel Protection+3",
        id: 0x20004e4b,
        dlc: 2,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of the Evil Eye",
        id: 0x2000506e,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of the Evil Eye+1",
        id: 0x2000506f,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of the Evil Eye+2",
        id: 0x20005070,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of the Evil Eye+3",
        id: 0x20005071,
        dlc: 2,
        ..Item::default_ring()
    },
    Item {
        name: "Ring of the Sun's First Born",
        id: 0x20004f1b,
        ..Item::default_ring()
    },
    Item {
        name: "Sage Ring",
        id: 0x20004f38,
        ..Item::default_ring()
    },
    Item {
        name: "Sage Ring+1",
        id: 0x20004f39,
        ..Item::default_ring()
    },
    Item {
        name: "Sage Ring+2",
        id: 0x20004f3a,
        ..Item::default_ring()
    },
    Item {
        name: "Saint's Ring",
        id: 0x20004f56,
        ..Item::default_ring()
    },
    Item {
        name: "Scholar Ring",
        id: 0x20004eb6,
        ..Item::default_ring()
    },
    Item {
        name: "Silvercat Ring",
        id: 0x20004fc4,
        ..Item::default_ring()
    },
    Item {
        name: "Skull Ring",
        id: 0x20004fce,
        ..Item::default_ring()
    },
    Item {
        name: "Slumbering Dragoncrest Ring",
        id: 0x20004f42,
        ..Item::default_ring()
    },
    Item {
        name: "Speckled Stoneplate Ring",
        id: 0x20004e7a,
        ..Item::default_ring()
    },
    Item {
        name: "Speckled Stoneplate Ring+1",
        id: 0x20004e7b,
        ..Item::default_ring()
    },
    Item {
        name: "Sun Princess Ring",
        id: 0x20004fba,
        ..Item::default_ring()
    },
    Item {
        name: "Thunder Stoneplate Ring",
        id: 0x20004e5c,
        ..Item::default_ring()
    },
    Item {
        name: "Thunder Stoneplate Ring+1",
        id: 0x20004e5d,
        ..Item::default_ring()
    },
    Item {
        name: "Thunder Stoneplate Ring+2",
        id: 0x20004e5e,
        ..Item::default_ring()
    },
    Item {
        name: "Untrue Dark Ring",
        id: 0x20005050,
        ..Item::default_ring()
    },
    Item {
        name: "Untrue White Ring",
        id: 0x20004fd8,
        ..Item::default_ring()
    },
    Item {
        name: "Witch's Ring",
        id: 0x20004f11,
        ..Item::default_ring()
    },
    Item {
        name: "Wolf Ring",
        id: 0x20004ede,
        ..Item::default_ring()
    },
    Item {
        name: "Wolf Ring+1",
        id: 0x20004edf,
        ..Item::default_ring()
    },
    Item {
        name: "Wolf Ring+2",
        id: 0x20004ee0,
        ..Item::default_ring()
    },
    Item {
        name: "Wolf Ring+3",
        id: 0x20004ee1,
        dlc: 2,
        ..Item::default_ring()
    },
    Item {
        name: "Wood Grain Ring",
        id: 0x20004eac,
        ..Item::default_ring()
    },
    Item {
        name: "Wood Grain Ring+1",
        id: 0x20004ead,
        ..Item::default_ring()
    },
    Item {
        name: "Wood Grain Ring+2",
        id: 0x20004eae,
        ..Item::default_ring()
    },
    Item {
        name: "Young Dragon Ring",
        id: 0x20004f06,
        ..Item::default_ring()
    },
];
