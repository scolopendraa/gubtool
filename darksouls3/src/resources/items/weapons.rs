use super::Item;

impl Item {
    const fn default_weapon() -> Self {
        Self {
            category: super::ItemCategory::Weapons,
            stack_size: 1,
            max_storage: 1,
            ..Item::default()
        }
    }
}

pub static WEAPONS: [Item; 286] = [
    Item {
        name: "Ancient Dragon Greatshield",
        id: 0x13599d0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Anri's Straight Sword",
        id: 0x2206f0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Aquamarine Dagger",
        id: 0x116520,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Arbalest",
        id: 0xd662d0,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Archdeacon's Great Staff",
        id: 0xc80af0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Arstor's Spear",
        id: 0x8bec50,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Astora Greatsword",
        id: 0x5c9ef0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Astora Straight Sword",
        id: 0x2191c0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Avelyn",
        id: 0xd6ff10,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Bandit's Knife",
        id: 0xf6950,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Barbed Straight Sword",
        id: 0x21b8d0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Bastard Sword",
        id: 0x5b8d80,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Battle Axe",
        id: 0x6af6d0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Black Blade",
        id: 0x4cc070,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Black Bow of Pharis",
        id: 0xd7e970,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Black Iron Greatshield",
        id: 0x150ea00,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Black Knight Glaive",
        id: 0x9ae070,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Black Knight Greataxe",
        id: 0x9959d0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Black Knight Greatsword",
        id: 0x5d8950,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Black Knight Shield",
        id: 0x141f5e0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Black Knight Sword",
        id: 0x5f5e10,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Blacksmith Hammer",
        id: 0x7e57c0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Bloodlust",
        id: 0x4ce780,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Blue Wooden Shield",
        id: 0x143f1b0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Bonewheel Shield",
        id: 0x151ad50,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Brigand Axe",
        id: 0x6b1de0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Brigand Twindaggers",
        id: 0xf50e60,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Broadsword",
        id: 0x1ed2a0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Broken Straight Sword",
        id: 0x1ef9b0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Buckler",
        id: 0x1312d00,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Butcher Knife",
        id: 0x6be130,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Caduceus Round Shield",
        id: 0x1341330,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Caestus",
        id: 0xa7ffd0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Caitha's Chime",
        id: 0xca06c0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Canvas Talisman",
        id: 0xca7bf0,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Carthus Curved Greatsword",
        id: 0x3df360,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Carthus Curved Sword",
        id: 0x3dcc50,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Carthus Shield",
        id: 0x14180b0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Carthus Shotel",
        id: 0x3eb6b0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Cathedral Knight Greatshield",
        id: 0x14fffa0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Cathedral Knight Greatsword",
        id: 0x5e73b0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Chaos Blade",
        id: 0x4c9960,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Claw",
        id: 0xa7d8c0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Claymore",
        id: 0x5bdba0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Cleric's Candlestick",
        id: 0x20f580,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Cleric's Sacred Chime",
        id: 0xc99190,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Club",
        id: 0x7a1200,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Composite Bow",
        id: 0xd5eda0,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Corvian Greatknife",
        id: 0x10a1d0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Court Sorcerer's Staff",
        id: 0xc91c60,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Crescent Axe",
        id: 0x6b6c00,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Crescent Moon Sword",
        id: 0x3e8fa0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Crest Shield",
        id: 0x1430750,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Crimson Parma",
        id: 0x134af70,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Crow Quills",
        id: 0xf66df0,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Crow Talons",
        id: 0xa89c10,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Crucifix of the Mad King",
        id: 0x8d4be0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Crystal Chime",
        id: 0xca2dd0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Crystal Sage's Rapier",
        id: 0x2e6300,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Curse Ward Greatshield",
        id: 0x1518640,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dagger",
        id: 0xf4240,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Dancer's Enchanted Swords",
        id: 0xf4c040,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dark Hand",
        id: 0xa87500,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Dark Sword",
        id: 0x5f3700,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Darkdrift",
        id: 0x4d0e90,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Darkmoon Longbow",
        id: 0xd79b50,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Demon's Fist",
        id: 0xa84df0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Demon's Greataxe",
        id: 0x6ca480,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Demon's Scar",
        id: 0x3f04d0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragon Crest Shield",
        id: 0x1432e60,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragon Tooth",
        id: 0x7e09a0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragonhead Greatshield",
        id: 0x1452a30,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragonhead Shield",
        id: 0x135e7f0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 0,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragonrider Bow",
        id: 0xd6b0f0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragonslayer Greataxe",
        id: 0x6c7d70,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragonslayer Greatbow",
        id: 0xcf8500,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragonslayer Greatshield",
        id: 0x1504dc0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragonslayer Spear",
        id: 0x8cafa0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragonslayer Swordspear",
        id: 0x8bc540,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Dragonslayer's Axe",
        id: 0x6c0840,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Drakeblood Greatsword",
        id: 0x609690,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Drang Hammers",
        id: 0xf61fd0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Drang Twinspears",
        id: 0xf5aaa0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Earth Seeker",
        id: 0x6d8ee0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "East-West Shield",
        id: 0x142b930,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Eastern Iron Shield",
        id: 0x134d680,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Eleonora",
        id: 0x6ccb90,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Elkhorn Round Shield",
        id: 0x133c510,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Estoc",
        id: 0x2dc6c0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Ethereal Oak Shield",
        id: 0x1450320,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Executioner's Greatsword",
        id: 0x21dfe0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Exile Greatsword",
        id: 0x5dd770,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Falchion",
        id: 0x3da540,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Farron Greatsword",
        id: 0x5e9ac0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Firelink Greatsword",
        id: 0x60bda0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Flamberge",
        id: 0x5db060,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Follower Javelin",
        id: 0x8cd6b0,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Follower Sabre",
        id: 0x3eddc0,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Follower Shield",
        id: 0x135c0e0,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Follower Torch",
        id: 0x15f1ad0,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Four-Pronged Plow",
        id: 0x8adae0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Frayed Blade",
        id: 0x4d35a0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Friede's Great Scythe",
        id: 0x9b55a0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Fume Ultra Greatsword",
        id: 0x60e4b0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Gael's Greatsword",
        id: 0x227c20,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Gargoyle Flame Hammer",
        id: 0x7caa10,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Gargoyle Flame Spear",
        id: 0x8b01f0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Ghru Rotshield",
        id: 0x13328d0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Giant Door Shield",
        id: 0xf5f8c0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Glaive",
        id: 0x98e4a0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Golden Falcon Shield",
        id: 0x1354bb0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Golden Ritual Spear",
        id: 0xc83200,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Golden Wing Crest Shield",
        id: 0x143caa0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Gotthard Twinswords",
        id: 0xf53570,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Grass Crest Shield",
        id: 0x1437c80,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Great Club",
        id: 0x7b4a80,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Great Corvian Scythe",
        id: 0x99cf00,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Great Mace",
        id: 0x7bbfb0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Great Machete",
        id: 0xf4e750,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Great Scythe",
        id: 0x989680,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Great Wooden Hammer",
        id: 0x7c8300,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Greataxe",
        id: 0x6b9310,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Greatlance",
        id: 0x8a8cc0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Greatshield of Glory",
        id: 0x1515f30,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Greatsword",
        id: 0x5c50d0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Greatsword of Judgment",
        id: 0x5e2590,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Gundyr's Halberd",
        id: 0x9a1d20,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Halberd",
        id: 0x990bb0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Hand Axe",
        id: 0x6acfc0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Handmaid's Dagger",
        id: 0x111700,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Harald Curved Greatsword",
        id: 0x6159e0,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Harpe",
        id: 0x102ca0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Havel's Greatshield",
        id: 0x13376f0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Hawkwood's Shield",
        id: 0x1323e70,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Heavy Crossbow",
        id: 0xd74d30,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Heretic's Staff",
        id: 0xc8f550,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Heysel Pick",
        id: 0x7d6d60,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Hollowslayer Greatsword",
        id: 0x604870,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Immolation Tinder",
        id: 0x9b0780,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Irithyll Rapier",
        id: 0x2e8a10,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Irithyll Straight Sword",
        id: 0x20a760,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Iron Round Shield",
        id: 0x1326580,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Izalith Staff",
        id: 0xc96a80,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Kite Shield",
        id: 0x13301c0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Knight Shield",
        id: 0x1410b80,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Knight's Crossbow",
        id: 0xd72620,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Large Club",
        id: 0x7afc60,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Large Leather Shield",
        id: 0x131c940,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Leather Shield",
        id: 0x1348860,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Ledo's Great Hammer",
        id: 0x7ef400,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Light Crossbow",
        id: 0xd63bc0,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Llewellyn Shield",
        id: 0x134fd90,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Long Sword",
        id: 0x1eab90,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Longbow",
        id: 0xd689e0,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Lorian's Greatsword",
        id: 0x5f8520,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Lothric Knight Greatshield",
        id: 0x14fd890,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Lothric Knight Greatsword",
        id: 0x5d1420,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Lothric Knight Long Spear",
        id: 0x8ab3d0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Lothric Knight Shield",
        id: 0x1409650,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Lothric Knight Sword",
        id: 0x1f6ee0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Lothric War Banner",
        id: 0x8d24d0,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Lothric's Holy Sword",
        id: 0x5fd340,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Lucerne",
        id: 0x98bd90,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Mace",
        id: 0x7a3910,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Mail Breaker",
        id: 0x2dedd0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Man Serpent Hatchet",
        id: 0x6d19b0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Man-grub's Staff",
        id: 0xc7e3e0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Manikin Claws",
        id: 0xa826e0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Mendicant's Staff",
        id: 0xc795c0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Millwood Battle Axe",
        id: 0x6d67d0,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Millwood Greatbow",
        id: 0xd85ea0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Moaning Shield",
        id: 0x15074d0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Moonlight Greatsword",
        id: 0x606f80,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Morion Blade",
        id: 0x2143a0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Morne's Great Hammer",
        id: 0x7e7ed0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Morning Star",
        id: 0x7a6020,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Murakumo",
        id: 0x5cc600,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Murky Hand Scythe",
        id: 0x118c30,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Murky Longstaff",
        id: 0xccc5e0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Notched Whip",
        id: 0xb7de50,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Old King's Great Hammer",
        id: 0x7cf830,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Old Wolf Curved Sword",
        id: 0x610bc0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Onikiri and Ubadachi",
        id: 0xf58390,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Onislayer Greatbow",
        id: 0xd7c260,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Onyx Blade",
        id: 0x222e00,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Painting Guardian's Curved Sword",
        id: 0x3e6890,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Parrying Dagger",
        id: 0xf9060,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Partizan",
        id: 0x89c970,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Pickaxe",
        id: 0x7de290,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Pierce Shield",
        id: 0x1429220,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Pike",
        id: 0x8c6180,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Plank Shield",
        id: 0x1346150,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Pontiff Knight Curved Sword",
        id: 0x3e1a70,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Pontiff Knight Great Scythe",
        id: 0x99a7f0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Pontiff Knight Shield",
        id: 0x14159a0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Porcine Shield",
        id: 0x1448df0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Preacher's Right Arm",
        id: 0xcd1400,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Priest's Chime",
        id: 0xc9b8a0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Profaned Greatsword",
        id: 0x5e4ca0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Pyromancer's Parting Flame",
        id: 0xcc9ed0,
        upgrade_type: Some(1),
        infusable: false,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Pyromancy Flame",
        id: 0xcc77c0,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Quakestone Hammer",
        id: 0x7eccf0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Rapier",
        id: 0x2e14e0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Red Hilted Halberd",
        id: 0x9ab960,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Red and White Shield",
        id: 0x1343a40,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Reinforced Club",
        id: 0x7a8730,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Repeating Crossbow",
        id: 0xd885b0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Ricard's Rapier",
        id: 0x2e3bf0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Ringed Knight Paired Greatswords",
        id: 0xf69500,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Ringed Knight Spear",
        id: 0x8cfdc0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Ringed Knight Straight Sword",
        id: 0x225510,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Rose of Ariandel",
        id: 0xb82c70,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Rotten Ghru Curved Sword",
        id: 0x205940,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Rotten Ghru Dagger",
        id: 0xfde80,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Rotten Ghru Spear",
        id: 0x8b2900,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Round Shield",
        id: 0x131a230,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Sacred Bloom Shield",
        id: 0x13572c0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Sacred Chime of Filianore",
        id: 0xccecf0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Sage's Crystal Staff",
        id: 0xc8ce40,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Saint Bident",
        id: 0x8c1360,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Saint's Talisman",
        id: 0xcaca10,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Saint-tree Bellvine",
        id: 0xc9dfb0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Scholar's Candlestick",
        id: 0x1053b0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Scimitar",
        id: 0x3d7e30,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Sellsword Twinblades",
        id: 0xf42400,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Shield of Want",
        id: 0x144b500,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Short Bow",
        id: 0xd5c690,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Shortsword",
        id: 0x1e8480,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Shotel",
        id: 0x3d3010,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Silver Eagle Kite Shield",
        id: 0x14418c0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Silver Knight Shield",
        id: 0x1424400,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Small Leather Shield",
        id: 0x1315410,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Smough's Great Hammer",
        id: 0x7e30b0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Sniper Crossbow",
        id: 0xd83790,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Soldering Iron",
        id: 0x8b7720,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Sorcerer's Staff",
        id: 0xc747a0,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Spear",
        id: 0x895440,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Spider Shield",
        id: 0x1435570,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Spiked Mace",
        id: 0x7ea5e0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Spiked Shield",
        id: 0x1426b10,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Spirit Tree Crest Shield",
        id: 0x14466e0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Splitleaf Greatsword",
        id: 0x9b2e90,
        upgrade_type: Some(1),
        infusable: true,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "Spotted Whip",
        id: 0xb80560,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Stone Greatshield",
        id: 0x151d460,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Stone Parma",
        id: 0x1443fd0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Storm Curved Sword",
        id: 0x3e4180,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Storm Ruler",
        id: 0x6132d0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Storyteller's Staff",
        id: 0xc76eb0,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Sunless Talisman",
        id: 0xcaa300,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Sunlight Shield",
        id: 0x142e040,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Sunlight Straight Sword",
        id: 0x203230,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Sunlight Talisman",
        id: 0xca54e0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Sunset Shield",
        id: 0x143a390,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Tailbone Short Sword",
        id: 0x107ac0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Tailbone Spear",
        id: 0x8b5010,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Talisman",
        id: 0xc72090,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Target Shield",
        id: 0x1339e00,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Thrall Axe",
        id: 0x6c5660,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Torch",
        id: 0x15ef3c0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Twin Dragon Greatshield",
        id: 0x1513820,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Twin Princes' Greatsword",
        id: 0x5fac30,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Uchigatana",
        id: 0x4c4b40,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Valorheart",
        id: 0xf646e0,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 1,
        ..Item::default_weapon()
    },
    Item {
        name: "Vordt's Great Hammer",
        id: 0x7cd120,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Warden Twinblades",
        id: 0xf47220,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Wargod Wooden Shield",
        id: 0x144dc10,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Warpick",
        id: 0x7dbb80,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Warrior's Round Shield",
        id: 0x133ec20,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Washing Pole",
        id: 0x4c7250,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Whip",
        id: 0xb71b00,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "White Birch Bow",
        id: 0xd77440,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 2,
        ..Item::default_weapon()
    },
    Item {
        name: "White Hair Talisman",
        id: 0xcaf120,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Winged Knight Halberd",
        id: 0x99f610,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Winged Knight Twinaxes",
        id: 0xf49930,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Winged Spear",
        id: 0x897b50,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Witch's Locks",
        id: 0xb7b740,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Witchtree Branch",
        id: 0xc94370,
        upgrade_type: Some(1),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Wolf Knight's Greatshield",
        id: 0x1511110,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Wolf Knight's Greatsword",
        id: 0x602160,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Wolnir's Holy Sword",
        id: 0x5ffa50,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Wooden Shield",
        id: 0x132dab0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
    Item {
        name: "Yhorm's Great Machete",
        id: 0x5f0ff0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Yhorm's Greatshield",
        id: 0x150c2f0,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Yorshka's Chime",
        id: 0xc88020,
        upgrade_type: Some(2),
        infusable: false,
        ..Item::default_weapon()
    },
    Item {
        name: "Yorshka's Spear",
        id: 0x8c3a70,
        upgrade_type: Some(2),
        infusable: false,
        dlc: 0,
        ..Item::default_weapon()
    },
    Item {
        name: "Zweihander",
        id: 0x5c29c0,
        upgrade_type: Some(1),
        infusable: true,
        ..Item::default_weapon()
    },
];
