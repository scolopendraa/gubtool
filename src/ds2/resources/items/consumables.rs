use super::Item;

impl Item {
    const fn default_consumable() -> Self {
        Self {
            category: super::Categories::Consumables,
            stack_size: 99,
            ..Item::default()
        }
    }
}

pub static CONSUMABLES: &[Item; 116] = &[
    Item {
        id: 60010000,
        name: "Lifegem",
        ..Item::default_consumable()
    },
    Item {
        id: 60020000,
        name: "Radiant Lifegem",
        ..Item::default_consumable()
    },
    Item {
        id: 60030000,
        name: "Old Radiant Lifegem",
        ..Item::default_consumable()
    },
    Item {
        id: 60035000,
        name: "Elizabeth Mushroom",
        ..Item::default_consumable()
    },
    Item {
        id: 60036000,
        name: "Dried Root",
        ..Item::default_consumable()
    },
    Item {
        id: 60040000,
        name: "Amber Herb",
        ..Item::default_consumable()
    },
    Item {
        id: 60050000,
        name: "Twilight Herb",
        ..Item::default_consumable()
    },
    Item {
        id: 60060000,
        name: "Wilted Dusk Herb",
        ..Item::default_consumable()
    },
    Item {
        id: 60070000,
        name: "Poison Moss",
        ..Item::default_consumable()
    },
    Item {
        id: 60090000,
        name: "Monastery Charm",
        ..Item::default_consumable()
    },
    Item {
        id: 60100000,
        name: "Dragon Charm",
        ..Item::default_consumable()
    },
    Item {
        id: 60105000,
        name: "Divine Blessing",
        ..Item::default_consumable()
    },
    Item {
        id: 60110000,
        name: "Rouge Water",
        ..Item::default_consumable()
    },
    Item {
        id: 60120000,
        name: "Crimson Water",
        ..Item::default_consumable()
    },
    Item {
        id: 60151000,
        name: "Human Effigy",
        ..Item::default_consumable()
    },
    Item {
        id: 60160000,
        name: "Small Blue Burr",
        ..Item::default_consumable()
    },
    Item {
        id: 60170000,
        name: "Small Yellow Burr",
        ..Item::default_consumable()
    },
    Item {
        id: 60180000,
        name: "Small Orange Burr",
        ..Item::default_consumable()
    },
    Item {
        id: 60190000,
        name: "Dark Troches",
        ..Item::default_consumable()
    },
    Item {
        id: 60200000,
        name: "Common Fruit",
        ..Item::default_consumable()
    },
    Item {
        id: 60210000,
        name: "Red Leech Troches",
        ..Item::default_consumable()
    },
    Item {
        id: 60230000,
        name: "Triclops Snake Troches",
        ..Item::default_consumable()
    },
    Item {
        id: 60235000,
        name: "Old Growth Balm",
        ..Item::default_consumable()
    },
    Item {
        id: 60236000,
        name: "Vine Balm",
        ..Item::default_consumable()
    },
    Item {
        id: 60237000,
        name: "Blackweed Balm",
        ..Item::default_consumable()
    },
    Item {
        id: 60238000,
        name: "Goldenfruit Balm",
        ..Item::default_consumable()
    },
    Item {
        id: 60239000,
        name: "Brightbug",
        ..Item::default_consumable()
    },
    Item {
        id: 60240000,
        name: "Aromatic Ooze",
        ..Item::default_consumable()
    },
    Item {
        id: 60250000,
        name: "Gold Pine Resin",
        ..Item::default_consumable()
    },
    Item {
        id: 60260000,
        name: "Charcoal Pine Resin",
        ..Item::default_consumable()
    },
    Item {
        id: 60270000,
        name: "Dark Pine Resin",
        ..Item::default_consumable()
    },
    Item {
        id: 60280000,
        name: "Rotten Pine Resin",
        ..Item::default_consumable()
    },
    Item {
        id: 60290000,
        name: "Bleeding Serum",
        ..Item::default_consumable()
    },
    Item {
        id: 60310000,
        name: "Green Blossom",
        ..Item::default_consumable()
    },
    Item {
        id: 60320000,
        name: "Rusted Coin",
        ..Item::default_consumable()
    },
    Item {
        id: 60350000,
        name: "Homeward Bone",
        ..Item::default_consumable()
    },
    Item {
        id: 60370000,
        name: "Silver Talisman",
        ..Item::default_consumable()
    },
    Item {
        id: 60410000,
        name: "Repair Powder",
        ..Item::default_consumable()
    },
    Item {
        id: 60420000,
        name: "Torch",
        stack_size: 1,
        ..Item::default_consumable()
    },
    Item {
        id: 60430000,
        name: "Flame Butterfly",
        ..Item::default_consumable()
    },
    Item {
        id: 60450000,
        name: "Prism Stone",
        ..Item::default_consumable()
    },
    Item {
        id: 60510000,
        name: "Rubbish",
        ..Item::default_consumable()
    },
    Item {
        id: 60511000,
        name: "Petrified Something",
        ..Item::default_consumable()
    },
    Item {
        id: 60527000,
        name: "Bonfire Ascetic",
        ..Item::default_consumable()
    },
    Item {
        id: 60530000,
        name: "Alluring Skull",
        ..Item::default_consumable()
    },
    Item {
        id: 60531000,
        name: "Lloyd's Talisman",
        ..Item::default_consumable()
    },
    Item {
        id: 60536000,
        name: "Pharros' Lockstone",
        ..Item::default_consumable()
    },
    Item {
        id: 60537000,
        name: "Fragrant Branch of Yore",
        ..Item::default_consumable()
    },
    Item {
        id: 60538000,
        name: "Fire Seed",
        ..Item::default_consumable()
    },
    Item {
        id: 60540000,
        name: "Throwing Knife",
        ..Item::default_consumable()
    },
    Item {
        id: 60550000,
        name: "Witching Urn",
        ..Item::default_consumable()
    },
    Item {
        id: 60560000,
        name: "Lightning Urn",
        ..Item::default_consumable()
    },
    Item {
        id: 60570000,
        name: "Firebomb",
        ..Item::default_consumable()
    },
    Item {
        id: 60575000,
        name: "Black Firebomb",
        ..Item::default_consumable()
    },
    Item {
        id: 60580000,
        name: "Hexing Urn",
        ..Item::default_consumable()
    },
    Item {
        id: 60590000,
        name: "Poison Throwing Knife",
        ..Item::default_consumable()
    },
    Item {
        id: 60595000,
        name: "Dung Pie",
        ..Item::default_consumable()
    },
    Item {
        id: 60600000,
        name: "Lacerating Knife",
        ..Item::default_consumable()
    },
    Item {
        id: 60610000,
        name: "Corrosive Urn",
        ..Item::default_consumable()
    },
    Item {
        id: 60620000,
        name: "Holy Water Urn",
        ..Item::default_consumable()
    },
    Item {
        id: 60625000,
        name: "Fading Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 60630000,
        name: "Soul of a Lost Undead",
        ..Item::default_consumable()
    },
    Item {
        id: 60640000,
        name: "Large Soul of a Lost Undead",
        ..Item::default_consumable()
    },
    Item {
        id: 60650000,
        name: "Soul of a Nameless Soldier",
        ..Item::default_consumable()
    },
    Item {
        id: 60660000,
        name: "Large Soul of a Nameless Soldier",
        ..Item::default_consumable()
    },
    Item {
        id: 60670000,
        name: "Soul of a Proud Knight",
        ..Item::default_consumable()
    },
    Item {
        id: 60680000,
        name: "Large Soul of a Proud Knight",
        ..Item::default_consumable()
    },
    Item {
        id: 60690000,
        name: "Soul of a Brave Warrior",
        ..Item::default_consumable()
    },
    Item {
        id: 60700000,
        name: "Large Soul of a Brave Warrior",
        ..Item::default_consumable()
    },
    Item {
        id: 60710000,
        name: "Soul of a Hero",
        ..Item::default_consumable()
    },
    Item {
        id: 60720000,
        name: "Soul of a Great Hero",
        ..Item::default_consumable()
    },
    Item {
        id: 64000000,
        name: "Soul of the Pursuer",
        ..Item::default_consumable()
    },
    Item {
        id: 64010000,
        name: "Soul of the Last Giant",
        ..Item::default_consumable()
    },
    Item {
        id: 64020000,
        name: "Dragonrider Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64030000,
        name: "Old Dragonslayer Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64040000,
        name: "Flexile Sentry Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64050000,
        name: "Ruin Sentinel Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64060000,
        name: "Soul of the Lost Sinner",
        ..Item::default_consumable()
    },
    Item {
        id: 64070000,
        name: "Executioner's Chariot Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64080000,
        name: "Skeleton Lord's Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64090000,
        name: "Covetous Demon Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64100000,
        name: "Mytha, the Baneful Queen Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64110000,
        name: "Smelter Demon Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64120000,
        name: "Old Iron King Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64130000,
        name: "Royal Rat Vanguard Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64140000,
        name: "Soul of the Rotten",
        ..Item::default_consumable()
    },
    Item {
        id: 64150000,
        name: "Scorpioness Najka Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64160000,
        name: "Royal Rat Authority Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64170000,
        name: "Soul of the Duke's Dear Freja",
        ..Item::default_consumable()
    },
    Item {
        id: 64180000,
        name: "Looking Glass Knight Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64190000,
        name: "Demon of Song Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64200000,
        name: "Soul of Velstadt",
        ..Item::default_consumable()
    },
    Item {
        id: 64210000,
        name: "Soul of the King",
        ..Item::default_consumable()
    },
    Item {
        id: 64220000,
        name: "Guardian Dragon Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64230000,
        name: "Ancient Dragon Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64240000,
        name: "Giant Lord Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64250000,
        name: "Soul of Nashandra",
        ..Item::default_consumable()
    },
    Item {
        id: 64260000,
        name: "Throne Defender Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64270000,
        name: "Throne Watcher Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64280000,
        name: "Darklurker Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64290000,
        name: "Belfry Gargoyle Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64300000,
        name: "Old Witch Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64310000,
        name: "Old King Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64320000,
        name: "Old Dead One Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64330000,
        name: "Old Paledrake Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64500000,
        name: "Soul of Sinh, the Slumbering Dragon",
        ..Item::default_consumable()
    },
    Item {
        id: 64510000,
        name: "Soul of the Fume Knight",
        ..Item::default_consumable()
    },
    Item {
        id: 64520000,
        name: "Soul of Aava, the King's Pet",
        ..Item::default_consumable()
    },
    Item {
        id: 64530000,
        name: "Soul of Elana, Squalid Queen",
        ..Item::default_consumable()
    },
    Item {
        id: 64540000,
        name: "Soul of Nadalia, Bride of Ash",
        ..Item::default_consumable()
    },
    Item {
        id: 64550000,
        name: "Soul of Alsanna, Silent Oracle",
        ..Item::default_consumable()
    },
    Item {
        id: 64560000,
        name: "Soul of Sir Alonne",
        ..Item::default_consumable()
    },
    Item {
        id: 64580000,
        name: "Soul of the Ivory King",
        ..Item::default_consumable()
    },
    Item {
        id: 64590000,
        name: "Soul of Zallen, the King's Pet",
        ..Item::default_consumable()
    },
    Item {
        id: 64600000,
        name: "Loyce Soul",
        ..Item::default_consumable()
    },
    Item {
        id: 64610000,
        name: "Soul of Lud, the King's Pet",
        ..Item::default_consumable()
    },
];