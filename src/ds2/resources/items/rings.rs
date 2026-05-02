use super::Item;

impl Item {
    const fn default_ring() -> Self {
        Self {
            category: super::Categories::Rings,
            stack_size: 1,
            ..Item::default()
        }
    }
}

pub static RINGS: &[Item; 122] = &[
    Item {
        id: 40010000,
        name: "Life Ring",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40010001,
        name: "Life Ring+1",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40010002,
        name: "Life Ring+2",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40010003,
        name: "Life Ring+3",
        durability: Some(25),
        ..Item::default_ring()
    },
    Item {
        id: 40020000,
        name: "Chloranthy Ring",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40020001,
        name: "Chloranthy Ring+1",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40020002,
        name: "Chloranthy Ring+2",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40030000,
        name: "Royal Soldier's Ring",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40030001,
        name: "Royal Soldier's Ring+1",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40030002,
        name: "Royal Soldier's Ring+2",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40040000,
        name: "First Dragon Ring",
        durability: Some(30),
        ..Item::default_ring()
    },
    Item {
        id: 40040001,
        name: "Second Dragon Ring",
        durability: Some(25),
        ..Item::default_ring()
    },
    Item {
        id: 40040002,
        name: "Third Dragon Ring",
        durability: Some(20),
        ..Item::default_ring()
    },
    Item {
        id: 40050000,
        name: "Ring of Steel Protection",
        durability: Some(140),
        ..Item::default_ring()
    },
    Item {
        id: 40050001,
        name: "Ring of Steel Protection+1",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40050002,
        name: "Ring of Steel Protection+2",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40060000,
        name: "Spell Quartz Ring",
        durability: Some(140),
        ..Item::default_ring()
    },
    Item {
        id: 40060001,
        name: "Spell Quartz Ring+1",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40060002,
        name: "Spell Quartz Ring+2",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40060003,
        name: "Spell Quartz Ring+3",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40070000,
        name: "Flame Quartz Ring",
        durability: Some(140),
        ..Item::default_ring()
    },
    Item {
        id: 40070001,
        name: "Flame Quartz Ring+1",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40070002,
        name: "Flame Quartz Ring+2",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40070003,
        name: "Flame Quartz Ring+3",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40080000,
        name: "Thunder Quartz Ring",
        durability: Some(140),
        ..Item::default_ring()
    },
    Item {
        id: 40080001,
        name: "Thunder Quartz Ring+1",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40080002,
        name: "Thunder Quartz Ring+2",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40080003,
        name: "Thunder Quartz Ring+3",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40090000,
        name: "Dark Quartz Ring",
        durability: Some(140),
        ..Item::default_ring()
    },
    Item {
        id: 40090001,
        name: "Dark Quartz Ring+1",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40090002,
        name: "Dark Quartz Ring+2",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40090003,
        name: "Dark Quartz Ring+3",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40100000,
        name: "Poisonbite Ring",
        durability: Some(140),
        ..Item::default_ring()
    },
    Item {
        id: 40100001,
        name: "Poisonbite Ring+1",
        durability: Some(60),
        ..Item::default_ring()
    },
    Item {
        id: 40110000,
        name: "Bloodbite Ring",
        durability: Some(140),
        ..Item::default_ring()
    },
    Item {
        id: 40110001,
        name: "Bloodbite Ring+1",
        durability: Some(60),
        ..Item::default_ring()
    },
    Item {
        id: 40120000,
        name: "Bracing Knuckle Ring",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40120001,
        name: "Bracing Knuckle Ring+1",
        durability: Some(85),
        ..Item::default_ring()
    },
    Item {
        id: 40120002,
        name: "Bracing Knuckle Ring+2",
        durability: Some(70),
        ..Item::default_ring()
    },
    Item {
        id: 40130000,
        name: "Cursebite Ring",
        durability: Some(140),
        ..Item::default_ring()
    },
    Item {
        id: 40135000,
        name: "Ash Knuckle Ring",
        durability: Some(140),
        ..Item::default_ring()
    },
    Item {
        id: 40140000,
        name: "Dispelling Ring",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40140001,
        name: "Dispelling Ring+1",
        durability: Some(35),
        ..Item::default_ring()
    },
    Item {
        id: 40150000,
        name: "Ring of Resistance",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40150001,
        name: "Ring of Resistance+1",
        durability: Some(35),
        ..Item::default_ring()
    },
    Item {
        id: 40160000,
        name: "Ring of Blades",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40160001,
        name: "Ring of Blades+1",
        durability: Some(85),
        ..Item::default_ring()
    },
    Item {
        id: 40160002,
        name: "Ring of Blades+2",
        durability: Some(70),
        ..Item::default_ring()
    },
    Item {
        id: 40210000,
        name: "Ring of Knowledge",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40220000,
        name: "Ring of Prayer",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40230000,
        name: "Stone Ring",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40260000,
        name: "Red Tearstone Ring",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40280000,
        name: "Blue Tearstone Ring",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40290000,
        name: "Ring of Giants",
        durability: Some(150),
        ..Item::default_ring()
    },
    Item {
        id: 40290001,
        name: "Ring of Giants+1",
        durability: Some(115),
        ..Item::default_ring()
    },
    Item {
        id: 40290002,
        name: "Ring of Giants+2",
        durability: Some(95),
        ..Item::default_ring()
    },
    Item {
        id: 40295000,
        name: "Old Leo Ring",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40300000,
        name: "Ring of Soul Protection",
        durability: Some(340),
        ..Item::default_ring()
    },
    Item {
        id: 40310000,
        name: "Ring of Life Protection",
        durability: Some(340),
        ..Item::default_ring()
    },
    Item {
        id: 40320000,
        name: "Lingering Dragoncrest Ring",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40320001,
        name: "Lingering Dragoncrest Ring+1",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40320002,
        name: "Lingering Dragoncrest Ring+2",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40330000,
        name: "Clear Bluestone Ring",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40330001,
        name: "Clear Bluestone Ring+1",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40330002,
        name: "Clear Bluestone Ring+2",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40340000,
        name: "Northern Ritual Band",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40340001,
        name: "Northern Ritual Band+1",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40340002,
        name: "Northern Ritual Band+2",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40350000,
        name: "Southern Ritual Band",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40350001,
        name: "Southern Ritual Band+1",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40350002,
        name: "Southern Ritual Band+2",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40360000,
        name: "Covetous Gold Serpent Ring",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40360001,
        name: "Covetous Gold Serpent Ring+1",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40360002,
        name: "Covetous Gold Serpent Ring+2",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40370000,
        name: "Covetous Silver Serpent Ring",
        durability: Some(90),
        ..Item::default_ring()
    },
    Item {
        id: 40370001,
        name: "Covetous Silver Serpent Ring+1",
        durability: Some(70),
        ..Item::default_ring()
    },
    Item {
        id: 40370002,
        name: "Covetous Silver Serpent Ring+2",
        durability: Some(55),
        ..Item::default_ring()
    },
    Item {
        id: 40390000,
        name: "Ring of the Evil Eye",
        durability: Some(105),
        ..Item::default_ring()
    },
    Item {
        id: 40390001,
        name: "Ring of the Evil Eye+1",
        durability: Some(80),
        ..Item::default_ring()
    },
    Item {
        id: 40390002,
        name: "Ring of the Evil Eye+2",
        durability: Some(65),
        ..Item::default_ring()
    },
    Item {
        id: 40400000,
        name: "Ring of Restoration",
        durability: Some(70),
        ..Item::default_ring()
    },
    Item {
        id: 40410000,
        name: "Ring of Binding",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40420000,
        name: "Silvercat Ring",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40430000,
        name: "Redeye Ring",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40440000,
        name: "Gower's Ring of Protection",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40450000,
        name: "Name-engraved Ring",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40460000,
        name: "Slumbering Dragoncrest Ring",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40470000,
        name: "Hawk Ring",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40480000,
        name: "Old Sun Ring",
        durability: Some(75),
        ..Item::default_ring()
    },
    Item {
        id: 40500000,
        name: "Illusory Ring of a Conqueror",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40510000,
        name: "King's Ring",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40520000,
        name: "Ring of the Dead",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40530000,
        name: "Ring of Thorns",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40530001,
        name: "Ring of Thorns+1",
        durability: Some(85),
        ..Item::default_ring()
    },
    Item {
        id: 40530002,
        name: "Ring of Thorns+2",
        durability: Some(70),
        ..Item::default_ring()
    },
    Item {
        id: 40540000,
        name: "Delicate String",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40550000,
        name: "White Ring",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40610000,
        name: "Ring of Whispers",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40620000,
        name: "Illusory Ring of the Exalted",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 40700000,
        name: "Crest of the Rat",
        durability: Some(100),
        ..Item::default_ring()
    },
    Item {
        id: 40710000,
        name: "Bell Keeper's Seal",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40720000,
        name: "Guardian's Seal",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 40730000,
        name: "Crest of Blood",
        durability: Some(100),
        ..Item::default_ring()
    },
    Item {
        id: 40740000,
        name: "Blue Seal",
        durability: Some(110),
        ..Item::default_ring()
    },
    Item {
        id: 40750000,
        name: "Abyss Seal",
        durability: Some(100),
        ..Item::default_ring()
    },
    Item {
        id: 40760000,
        name: "Vanquisher's Seal",
        durability: Some(100),
        ..Item::default_ring()
    },
    Item {
        id: 40770000,
        name: "Sun Seal",
        durability: Some(100),
        ..Item::default_ring()
    },
    Item {
        id: 40780000,
        name: "Ancient Dragon Seal",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 41000000,
        name: "Simpleton's Ring",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 41010000,
        name: "Strength Ring",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 41020000,
        name: "Dexterity Ring",
        durability: Some(120),
        ..Item::default_ring()
    },
    Item {
        id: 41030000,
        name: "Ivory Warrior Ring",
        durability: Some(30),
        ..Item::default_ring()
    },
    Item {
        id: 41040000,
        name: "Sorcery Clutch Ring",
        durability: Some(30),
        ..Item::default_ring()
    },
    Item {
        id: 41050000,
        name: "Lightning Clutch Ring",
        durability: Some(30),
        ..Item::default_ring()
    },
    Item {
        id: 41060000,
        name: "Fire Clutch Ring",
        durability: Some(30),
        ..Item::default_ring()
    },
    Item {
        id: 41070000,
        name: "Dark Clutch Ring",
        durability: Some(30),
        ..Item::default_ring()
    },
    Item {
        id: 41090000,
        name: "Baneful Bird Ring",
        durability: Some(45),
        ..Item::default_ring()
    },
    Item {
        id: 41100000,
        name: "Flynn's Ring",
        durability: Some(30),
        ..Item::default_ring()
    },
    Item {
        id: 41110000,
        name: "Ring of the Embedded",
        durability: Some(80),
        ..Item::default_ring()
    },
    Item {
        id: 41120000,
        name: "Ring of the Living",
        durability: Some(130),
        ..Item::default_ring()
    },
    Item {
        id: 41130000,
        name: "Yorgh's Ring",
        durability: Some(10),
        ..Item::default_ring()
    },
    Item {
        id: 42000000,
        name: "Agape Ring",
        durability: Some(500),
        scholar_only: true,
        ..Item::default_ring()
    },
];