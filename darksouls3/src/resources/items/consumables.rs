use super::Item;

impl Item {
    const fn default_consumable() -> Self {
        Self {
            category: super::ItemCategory::Consumables,
            max_storage: 600,
            ..Item::default()
        }
    }
}

pub static CONSUMABLES: [Item; 104] = [
    Item {
        name: "Alluring Skull",
        id: 0x40000126,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Black Bug Pellet",
        id: 0x400001cd,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Black Firebomb",
        id: 0x40000129,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Bloodred Moss Clump",
        id: 0x4000010e,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Blooming Purple Moss Clump",
        id: 0x40000110,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Blue Bug Pellet",
        id: 0x400001ca,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Budding Green Blossom",
        id: 0x40000106,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Carthus Rouge",
        id: 0x4000014f,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Charcoal Pine Bundle",
        id: 0x40000154,
        stack_size: 20,
        ..Item::default_consumable()
    },
    Item {
        name: "Charcoal Pine Resin",
        id: 0x4000014a,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Cracked Red Eye Orb",
        id: 0x4000006f,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Dark Sigil",
        id: 0x400001ea,
        stack_size: 8,
        ..Item::default_consumable()
    },
    Item {
        name: "Divine Blessing",
        id: 0x400000f0,
        stack_size: 1,
        ..Item::default_consumable()
    },
    Item {
        name: "Duel Charm",
        id: 0x40000130,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Dung Pie",
        id: 0x40000125,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Ember",
        id: 0x400001f4,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Filianore's Spear Ornament",
        id: 0x4000017b,
        stack_size: 99,
        dlc: 2,
        ..Item::default_consumable()
    },
    Item {
        name: "Firebomb",
        id: 0x40000124,
        stack_size: 20,
        ..Item::default_consumable()
    },
    Item {
        name: "Forked Pale Tongue",
        id: 0x40000170,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Gold Pine Bundle",
        id: 0x40000155,
        stack_size: 20,
        ..Item::default_consumable()
    },
    Item {
        name: "Gold Pine Resin",
        id: 0x4000014b,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Green Blossom",
        id: 0x40000104,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Hidden Blessing",
        id: 0x400000f1,
        stack_size: 1,
        ..Item::default_consumable()
    },
    Item {
        name: "Homeward Bone",
        id: 0x4000015e,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Human Dregs",
        id: 0x4000016f,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Human Pine Resin",
        id: 0x4000014e,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Kukri",
        id: 0x40000122,
        stack_size: 30,
        ..Item::default_consumable()
    },
    Item {
        name: "Lightning Urn",
        id: 0x4000012c,
        stack_size: 20,
        ..Item::default_consumable()
    },
    Item {
        name: "Pale Pine Resin",
        id: 0x40000150,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Pale Tongue",
        id: 0x40000175,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Pendant",
        id: 0x40000178,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Poison Throwing Knife",
        id: 0x40000137,
        stack_size: 40,
        ..Item::default_consumable()
    },
    Item {
        name: "Prism Stone",
        id: 0x40000172,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Proof of a Concord Kept",
        id: 0x40000174,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Proof of a Concord Well Kept",
        id: 0x40000171,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Purging Stone",
        id: 0x40000112,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Purple Moss Clump",
        id: 0x4000010f,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Red Bug Pellet",
        id: 0x400001cb,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Repair Powder",
        id: 0x40000118,
        stack_size: 20,
        ..Item::default_consumable()
    },
    Item {
        name: "Rime-blue Moss Clump",
        id: 0x40000114,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Rope Black Firebomb",
        id: 0x4000012e,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Rope Firebomb",
        id: 0x4000012b,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Rotten Pine Resin",
        id: 0x40000157,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Rubbish",
        id: 0x4000017c,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Rusted Coin",
        id: 0x400001c7,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Rusted Gold Coin",
        id: 0x400001c9,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Seed of a Giant Tree",
        id: 0x400001b8,
        stack_size: 3,
        ..Item::default_consumable()
    },
    Item {
        name: "Siegbräu",
        id: 0x400001c8,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Stalk Dung Pie",
        id: 0x4000012f,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Sunlight Medal",
        id: 0x40000177,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Throwing Knife",
        id: 0x40000136,
        stack_size: 40,
        ..Item::default_consumable()
    },
    Item {
        name: "Twinkling Dragon Head Stone",
        id: 0x40000183,
        stack_size: 1,
        ..Item::default_consumable()
    },
    Item {
        name: "Twinkling Dragon Torso Stone",
        id: 0x40000184,
        stack_size: 1,
        ..Item::default_consumable()
    },
    Item {
        name: "Undead Hunter Charm",
        id: 0x40000128,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Vertebra Shackle",
        id: 0x40000176,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Wolf's Blood Swordgrass",
        id: 0x4000016e,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Yellow Bug Pellet",
        id: 0x400001cc,
        stack_size: 5,
        ..Item::default_consumable()
    },
    Item {
        name: "Young White Branch",
        id: 0x400001c6,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Young White Branch",
        id: 0x400001cf,
        stack_size: 10,
        ..Item::default_consumable()
    },
    Item {
        name: "Fading Soul",
        id: 0x40000190,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Deserted Corpse",
        id: 0x40000191,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Large Soul of a Deserted Corpse",
        id: 0x40000192,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of an Unknown Traveler",
        id: 0x40000193,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Large Soul of an Unknown Traveler",
        id: 0x40000194,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Nameless Soldier",
        id: 0x40000195,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Large Soul of a Nameless Soldier",
        id: 0x40000196,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Weary Warrior",
        id: 0x40000197,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Large Soul of a Weary Warrior",
        id: 0x40000198,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Crestfallen Knight",
        id: 0x40000199,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Large Soul of a Crestfallen Knight",
        id: 0x4000019a,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Proud Paladin",
        id: 0x4000019b,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Large Soul of a Proud Paladin",
        id: 0x4000019c,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of an Intrepid Hero",
        id: 0x4000019d,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Large Soul of an Intrepid Hero",
        id: 0x4000019e,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Seasoned Warrior",
        id: 0x4000019f,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Large Soul of a Seasoned Warrior",
        id: 0x400001a0,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of an Old Hand",
        id: 0x400001a1,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Venerable Old Hand",
        id: 0x400001a2,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Champion",
        id: 0x400001a3,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Great Champion",
        id: 0x400001a4,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Champion Gundyr",
        id: 0x400002c8,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of the Dancer",
        id: 0x400002ca,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Crystal Sage",
        id: 0x400002cb,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of the Blood of the Wolf",
        id: 0x400002cd,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Consumed Oceiros",
        id: 0x400002ce,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Boreal Valley Vordt",
        id: 0x400002cf,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of the Old Demon King",
        id: 0x400002d0,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Dragonslayer Armour",
        id: 0x400002d1,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of the Nameless King",
        id: 0x400002d2,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Pontiff Sulyvahn",
        id: 0x400002d4,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Aldrich",
        id: 0x400002d5,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of High Lord Wolnir",
        id: 0x400002d6,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of the Rotted Greatwood",
        id: 0x400002d7,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Rosaria",
        id: 0x400002d8,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of the Deacons of the Deep",
        id: 0x400002d9,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Yhorm the Giant",
        id: 0x400002dc,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of the Lords",
        id: 0x400002dd,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Demon",
        id: 0x400002e3,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of the Twin Princes",
        id: 0x400002db,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of a Stray Demon",
        id: 0x400002e7,
        stack_size: 99,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Sister Friede",
        id: 0x400002e8,
        stack_size: 99,
        dlc: 1,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Slave Knight Gael",
        id: 0x400002e9,
        stack_size: 99,
        dlc: 2,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of the Demon Prince",
        id: 0x400002ea,
        stack_size: 99,
        dlc: 2,
        ..Item::default_consumable()
    },
    Item {
        name: "Soul of Darkeater Midir",
        id: 0x400002eb,
        stack_size: 99,
        dlc: 2,
        ..Item::default_consumable()
    },
];
