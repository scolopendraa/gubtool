use super::Item;

impl Item {
    const fn default_weapons() -> Self {
        Self {
            category: super::Categories::Weapons,
            ..Item::default()
        }
    }
}

pub static WEAPONS: [Item; 479] = [
    Item {
        id: 0x1fa9780,
        name: "Academy Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2ed830,
        name: "Alabaster Lord's Sword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x271c350,
        name: "Albinauric Bow",
        weapon_type: Some(51),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1d9a200,
        name: "Albinauric Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fa7070,
        name: "Albinauric Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x44aa20,
        name: "Ancient Meteoric Ore Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2796470,
        name: "Ansbach's Longbow",
        weapon_type: Some(51),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1eba360,
        name: "Ant's Skull Plate",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4ce780,
        name: "Antspur Rapier",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xbee330,
        name: "Anvil Hammer",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2915940,
        name: "Arbalest",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f98610,
        name: "Astrologer's Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15fb710,
        name: "Axe of Godfrey",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe57e00,
        name: "Axe of Godrick",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fb0cb0,
        name: "Azur's Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3d83120,
        name: "Backhand Blade",
        weapon_type: Some(92),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x6b6c00,
        name: "Bandit's Curved Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2eff40,
        name: "Banished Knight's Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1131db0,
        name: "Banished Knight's Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1d97af0,
        name: "Banished Knight's Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x10b5580,
        name: "Barbed Staff-Spear",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2dc6c0,
        name: "Bastard Sword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xc6d270,
        name: "Bastard's Stars",
        weapon_type: Some(24),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xd59f80,
        name: "Battle Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb76920,
        name: "Battle Hammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4153a20,
        name: "Beast Claw",
        weapon_type: Some(95),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1dd4b80,
        name: "Beast Crest Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb964f0,
        name: "Beastclaw Greathammer",
        weapon_type: Some(23),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7b4a80,
        name: "Beastman's Cleaver",
        weapon_type: Some(11),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6af6d0,
        name: "Beastman's Curved Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1caade0,
        name: "Beastman's Jar-Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16f2060,
        name: "Beast-Repellent Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x272adb0,
        name: "Black Bow",
        weapon_type: Some(51),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf6950,
        name: "Black Knife",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1de35e0,
        name: "Black Leather Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xbf0a40,
        name: "Black Steel Greathammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1efe920,
        name: "Black Steel Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xa05eb0,
        name: "Black Steel Twinblade",
        weapon_type: Some(14),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x11b340,
        name: "Blade of Calling",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2fe9a0,
        name: "Blasphemous Blade",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xbf3150,
        name: "Bloodfiend's Arm",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xfc6160,
        name: "Bloodfiend's Fork",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xfc8870,
        name: "Bloodfiend's Sacred Spear",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x14fffa0,
        name: "Bloodhound Claws",
        weapon_type: Some(37),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7a8730,
        name: "Bloodhound's Fang",
        weapon_type: Some(11),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x116520,
        name: "Bloodstained Dagger",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x5b8d80,
        name: "Bloody Helice",
        weapon_type: Some(16),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1dd99a0,
        name: "Blue Crest Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1da8c60,
        name: "Blue-Gold Kite Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1cb4a20,
        name: "Blue-White Wooden Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf58390,
        name: "Bolt of Gransax",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x269fb20,
        name: "Bone Bow",
        weapon_type: Some(50),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xeca9f0,
        name: "Bonny Butchering Knife",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1db0190,
        name: "Brass Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e90b50,
        name: "Briar Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xba0130,
        name: "Brick Hammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ed2a0,
        name: "Broadsword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1c9c380,
        name: "Buckler",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe6b680,
        name: "Butchering Knife",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1406f40,
        name: "Caestus",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1dcd650,
        name: "Candletree Wooden Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x21b8d0,
        name: "Cane Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fa2250,
        name: "Carian Glintblade Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fabe90,
        name: "Carian Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1dbebf0,
        name: "Carian Knight's Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2143a0,
        name: "Carian Knight's Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f8e9d0,
        name: "Carian Regal Scepter",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x269ad0,
        name: "Carian Sorcery Sword",
        weapon_type: Some(15),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3b9d3b0,
        name: "Carian Thrusting Shield",
        weapon_type: Some(90),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xd689e0,
        name: "Celebrant's Cleaver",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf50e60,
        name: "Celebrant's Rib-Rake",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x102ca0,
        name: "Celebrant's Sickle",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb916d0,
        name: "Celebrant's Skull",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xc6f980,
        name: "Chainlink Flail",
        weapon_type: Some(24),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3aa9170,
        name: "Chilling Perfume Bottle",
        weapon_type: Some(89),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x10eff0,
        name: "Cinquedea",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1426b10,
        name: "Cipher Pata",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x20768c0,
        name: "Clawmark Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15752a0,
        name: "Claws of Night",
        weapon_type: Some(37),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xf49930,
        name: "Clayman's Harpoon",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3085e0,
        name: "Claymore",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4c7250,
        name: "Cleanrot Knight's Sword",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf4c040,
        name: "Cleanrot Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1421cf0,
        name: "Clinging Bone",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa7ffd0,
        name: "Club",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x203230,
        name: "Coded Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ccd0c0,
        name: "Coil Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x11344c0,
        name: "Commander's Standard",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2631d50,
        name: "Composite Bow",
        weapon_type: Some(50),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb9b310,
        name: "Cranial Vessel Candlestand",
        weapon_type: Some(23),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x291ce70,
        name: "Crepus's Black-Key Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe556f0,
        name: "Crescent Moon Axe",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1eb5540,
        name: "Crossed-Tree Towershield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf5d1b0,
        name: "Cross-Naginata",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e8bd30,
        name: "Crucible Hornshield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x100590,
        name: "Crystal Knife",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf47220,
        name: "Crystal Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f82680,
        name: "Crystal Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x20ce70,
        name: "Crystal Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ec1890,
        name: "Cuckoo Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3d87f40,
        name: "Curseblade's Cirque",
        weapon_type: Some(92),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xa84df0,
        name: "Curved Club",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb85380,
        name: "Curved Great Club",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf4240,
        name: "Dagger",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x72bf00,
        name: "Dancing Blade of Ranah",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x39b4f30,
        name: "Dane's Footwork",
        weapon_type: Some(88),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2f2650,
        name: "Dark Moon Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3ab06a0,
        name: "Deadly Poison Perfume Bottle",
        weapon_type: Some(89),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xec82e0,
        name: "Death Knight's Longhaft Axe",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xdd67b0,
        name: "Death Knight's Twin Axes",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xf5f8c0,
        name: "Death Ritual Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x30d400,
        name: "Death's Poker",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f874a0,
        name: "Demi-Human Queen's Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16694e0,
        name: "Devonia's Hammer",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xba2840,
        name: "Devourer's Scepter",
        weapon_type: Some(23),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f95f00,
        name: "Digger's Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7a6020,
        name: "Dismounter",
        weapon_type: Some(11),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e89620,
        name: "Distinguished Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2080500,
        name: "Dragon Communion Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15fde20,
        name: "Dragon Greatclaw",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x114cb60,
        name: "Dragon Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x5c29c0,
        name: "Dragon King's Cragblade",
        weapon_type: Some(16),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e84800,
        name: "Dragon Towershield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e8e440,
        name: "Dragonclaw Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3f6dcb0,
        name: "Dragon-Hunter's Great Katana",
        weapon_type: Some(94),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x8a65b0,
        name: "Dragonscale Blade",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x39b2820,
        name: "Dryleaf Arts",
        weapon_type: Some(88),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x20e6da0,
        name: "Dryleaf Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3b9aca0,
        name: "Dueling Shield",
        weapon_type: Some(90),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x15f9000,
        name: "Duelist Greataxe",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ebf180,
        name: "Eclipse Crest Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ddc0b0,
        name: "Eclipse Crest Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6c5660,
        name: "Eclipse Shotel",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x9959d0,
        name: "Eleonora's Poleblade",
        weapon_type: Some(14),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15f68f0,
        name: "Envoy's Greathorn",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa95f60,
        name: "Envoy's Horn",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb98c00,
        name: "Envoy's Long Horn",
        weapon_type: Some(23),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x118c30,
        name: "Erdsteel Dagger",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2721170,
        name: "Erdtree Bow",
        weapon_type: Some(51),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x28153b0,
        name: "Erdtree Greatbow",
        weapon_type: Some(53),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e98080,
        name: "Erdtree Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x207ddf0,
        name: "Erdtree Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4c4b40,
        name: "Estoc",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa037a0,
        name: "Euporia",
        weapon_type: Some(14),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xe61a40,
        name: "Executioner's Greataxe",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6acfc0,
        name: "Falchion",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1602c40,
        name: "Fallingstar Beast Jaw",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7297f0,
        name: "Falx",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xc6ab60,
        name: "Family Heads",
        weapon_type: Some(24),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x206cc80,
        name: "Finger Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ea43d0,
        name: "Fingerprint Stone Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x44f840,
        name: "Fire Knight's Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x20e94b0,
        name: "Fire Knight's Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x170a70,
        name: "Fire Knight's Shortsword",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3aa6a60,
        name: "Firespark Perfume Bottle",
        weapon_type: Some(89),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xc68450,
        name: "Flail",
        weapon_type: Some(24),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2e8a10,
        name: "Flamberge",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1dcfd60,
        name: "Flame Crest Wooden Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xaf79e0,
        name: "Flowerstone Gavel",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x6bba20,
        name: "Flowing Curved Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2dedd0,
        name: "Forked Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xd5c690,
        name: "Forked Hatchet",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xdddce0,
        name: "Forked-Tongue Hatchet",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2082c10,
        name: "Frenzied Flame Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3aab880,
        name: "Frenzyflame Perfume Bottle",
        weapon_type: Some(89),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x81da30,
        name: "Freyja's Greatsword",
        weapon_type: Some(11),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x4d0e90,
        name: "Frozen Needle",
        weapon_type: Some(15),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2910b20,
        name: "Full Moon Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe704a0,
        name: "Gargoyle's Black Axe",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x99f610,
        name: "Gargoyle's Black Blades",
        weapon_type: Some(14),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1151980,
        name: "Gargoyle's Black Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x30fb10,
        name: "Gargoyle's Blackblade",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe6dd90,
        name: "Gargoyle's Great Axe",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x30acf0,
        name: "Gargoyle's Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x114f270,
        name: "Gargoyle's Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x99cf00,
        name: "Gargoyle's Twinblade",
        weapon_type: Some(14),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x166e300,
        name: "Gazing Finger",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f84d90,
        name: "Gelmir Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1607a60,
        name: "Ghiza's Wheel",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16ef950,
        name: "Ghostflame Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x160a170,
        name: "Giant-Crusher",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1321760,
        name: "Giant's Red Braid",
        weapon_type: Some(39),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2071aa0,
        name: "Giant's Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ec66b0,
        name: "Gilded Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1cbbf50,
        name: "Gilded Iron Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1140810,
        name: "Glaive",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1053b0,
        name: "Glintstone Kris",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f78a40,
        name: "Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x98bd90,
        name: "Godskin Peeler",
        weapon_type: Some(14),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x5bb490,
        name: "Godskin Stitcher",
        weapon_type: Some(16),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3e1a70,
        name: "Godslayer's Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x206f390,
        name: "Godslayer's Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e9a790,
        name: "Golden Beast Crest Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f95f0,
        name: "Golden Epitaph",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ec3fa0,
        name: "Golden Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x113e100,
        name: "Golden Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e11c10,
        name: "Golden Lion Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x305ed0,
        name: "Golden Order Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x207b6e0,
        name: "Golden Order Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x148aca0,
        name: "Golem Fist",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2810590,
        name: "Golem Greatbow",
        weapon_type: Some(53),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x160c880,
        name: "Golem's Halberd",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3e8fa0,
        name: "Grafted Blade Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x14159a0,
        name: "Grafted Dragon",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x12211d0,
        name: "Grave Scythe",
        weapon_type: Some(31),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x20741b0,
        name: "Gravel Stone Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15f41e0,
        name: "Great Club",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x5bdba0,
        name: "Great Épée",
        weapon_type: Some(16),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3f6b5a0,
        name: "Great Katana",
        weapon_type: Some(94),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x10a1d0,
        name: "Great Knife",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb80560,
        name: "Great Mace",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe52fe0,
        name: "Great Omenkiller Cleaver",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb9da20,
        name: "Great Stars",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1db28a0,
        name: "Great Turtle Shell",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe4e1c0,
        name: "Greataxe",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2817ac0,
        name: "Greatbow",
        weapon_type: Some(53),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb74210,
        name: "Greathorn Hammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3d0900,
        name: "Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x358ef0,
        name: "Greatsword of Damnation",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x456d70,
        name: "Greatsword of Radahn (Light)",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x451f50,
        name: "Greatsword of Radahn (Lord)",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x362b30,
        name: "Greatsword of Solitude",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x6d19b0,
        name: "Grossmesser",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1145630,
        name: "Guardian's Swordspear",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x112a880,
        name: "Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ec8dc0,
        name: "Haligtree Crest Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x12238e0,
        name: "Halo Scythe",
        weapon_type: Some(31),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa91140,
        name: "Hammer",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xd5eda0,
        name: "Hand Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x29f6300,
        name: "Hand Ballista",
        weapon_type: Some(56),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x89a260,
        name: "Hand of Malenia",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x262cf30,
        name: "Harp Bow",
        weapon_type: Some(50),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1dd2470,
        name: "Hawk Crest Wooden Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1de0ed0,
        name: "Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x29095f0,
        name: "Heavy Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2fc290,
        name: "Helphen's Steeple",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xd72620,
        name: "Highland Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x14fb180,
        name: "Hookclaws",
        weapon_type: Some(37),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x271ea60,
        name: "Horn Bow",
        weapon_type: Some(51),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x820140,
        name: "Horned Warrior's Greatsword",
        weapon_type: Some(11),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x72e610,
        name: "Horned Warrior's Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1dcaf40,
        name: "Horse Crest Wooden Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x131f050,
        name: "Hoslow's Petal Whip",
        weapon_type: Some(39),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1cbe660,
        name: "Ice Crest Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xd6d800,
        name: "Icerind Hatchet",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ea6ae0,
        name: "Icon Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2887fa0,
        name: "Igon's Greatbow",
        weapon_type: Some(53),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xf61fd0,
        name: "Inquisitor's Girandole",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fe410,
        name: "Inseparable Sword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1dde7c0,
        name: "Inverted Hawk Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1eb7c50,
        name: "Inverted Hawk Towershield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x14180b0,
        name: "Iron Ball",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xd63bc0,
        name: "Iron Cleaver",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2e14e0,
        name: "Iron Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1cb9840,
        name: "Iron Roundshield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf66df0,
        name: "Iron Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x113e10,
        name: "Ivory Sickle",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x29f8a10,
        name: "Jar Cannon",
        weapon_type: Some(56),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xd614b0,
        name: "Jawbone Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ea1cc0,
        name: "Jellyfish Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x141f5e0,
        name: "Katar",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1d905c0,
        name: "Kite Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2e6300,
        name: "Knight's Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x175fe30,
        name: "Lamenting Visage",
        weapon_type: Some(87),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x10450a0,
        name: "Lance",
        weapon_type: Some(28),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb71b00,
        name: "Large Club",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1dc8830,
        name: "Large Leather Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x225510,
        name: "Lazuli Glintstone Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4061ef0,
        name: "Leda's Sword",
        weapon_type: Some(93),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2906ee0,
        name: "Light Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3aadf90,
        name: "Lightning Perfume Bottle",
        weapon_type: Some(89),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x280de80,
        name: "Lion Greatbow",
        weapon_type: Some(53),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x35b600,
        name: "Lizard Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2719c40,
        name: "Longbow",
        weapon_type: Some(51),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe5a510,
        name: "Longhaft Axe",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e8480,
        name: "Longsword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2e3bf0,
        name: "Lordsworn's Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ecdbe0,
        name: "Lordsworn's Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f20c0,
        name: "Lordsworn's Straight Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1142f20,
        name: "Loretta's War Sickle",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x112f6a0,
        name: "Lucerne",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fb33c0,
        name: "Lusat's Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa7d8c0,
        name: "Mace",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1488590,
        name: "Madding Hand",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x6b9310,
        name: "Magma Blade",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x131a230,
        name: "Magma Whip Candlestick",
        weapon_type: Some(39),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7aae40,
        name: "Magma Wyrm's Scalesword",
        weapon_type: Some(11),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16e360,
        name: "Main-gauche",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3d5720,
        name: "Maliketh's Black Blade",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1eb2e30,
        name: "Manor Towershield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ca11a0,
        name: "Man-Serpent's Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6ca480,
        name: "Mantis Blade",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3010b0,
        name: "Marais Executioner's Sword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xaa22b0,
        name: "Marika's Hammer",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1d92cd0,
        name: "Marred Leather Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1d953e0,
        name: "Marred Wooden Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ff7980,
        name: "Maternal Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e0a6e0,
        name: "Messmer Soldier Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xdd8ec0,
        name: "Messmer Soldier's Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x10b2e70,
        name: "Messmer Soldier's Spear",
        weapon_type: Some(28),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x89c970,
        name: "Meteoric Ore Blade",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fb5ad0,
        name: "Meteorite Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(2),
        ..Item::default_weapons()
    },
    Item {
        id: 0x405f7e0,
        name: "Milady",
        weapon_type: Some(93),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2191c0,
        name: "Miquellan Knight's Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2628110,
        name: "Misbegotten Shortbow",
        weapon_type: Some(50),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xfb770,
        name: "Miséricorde",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1038d50,
        name: "Mohgwyn's Sacred Spear",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7b2370,
        name: "Monk's Flameblade",
        weapon_type: Some(11),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa93850,
        name: "Monk's Flamemace",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x454660,
        name: "Moonrithyll's Knight Sword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x8a3ea0,
        name: "Moonveil",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7b98a0,
        name: "Morgott's Cursed Sword",
        weapon_type: Some(11),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa89c10,
        name: "Morning Star",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x897b50,
        name: "Nagakiba",
        weapon_type: Some(13),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x175d720,
        name: "Nanaya's Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xc65d40,
        name: "Nightrider Flail",
        weapon_type: Some(24),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1136bd0,
        name: "Nightrider Glaive",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4d35a0,
        name: "Noble's Estoc",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2206f0,
        name: "Noble's Slender Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa9ad80,
        name: "Nox Flowing Hammer",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fbd00,
        name: "Nox Flowing Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1298be0,
        name: "Obsidian Lamina",
        weapon_type: Some(31),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x7afc60,
        name: "Omen Cleaver",
        weapon_type: Some(11),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ea91f0,
        name: "One-Eyed Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7a3910,
        name: "Onyx Lord's Greatsword",
        weapon_type: Some(11),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2eb120,
        name: "Ordovis's Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f6ee0,
        name: "Ornamental Straight Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf9060,
        name: "Parrying Dagger",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf4e750,
        name: "Partisan",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1483770,
        name: "Pata",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1c9ea90,
        name: "Perfumer's Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x112cf90,
        name: "Pest's Glaive",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xb93de0,
        name: "Pickaxe",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf53570,
        name: "Pike",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ca5fc0,
        name: "Pillory Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1485e80,
        name: "Poisoned Hand",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x11a70b0,
        name: "Poleblade of the Bud",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x15ef3c0,
        name: "Prelate's Inferno Crozier",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fa4960,
        name: "Prince of Death's Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x27286a0,
        name: "Pulley Bow",
        weapon_type: Some(51),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x290e410,
        name: "Pulley Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x81b320,
        name: "Putrescence Cleaver",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x632ea0,
        name: "Queelign's Greatsword",
        weapon_type: Some(16),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2a70420,
        name: "Rabbath's Cannon",
        weapon_type: Some(56),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3f703c0,
        name: "Rakshasa's Great Katana",
        weapon_type: Some(94),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x4c9960,
        name: "Rapier",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15026b0,
        name: "Raptor Talons",
        weapon_type: Some(37),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4156130,
        name: "Red Bear's Claw",
        weapon_type: Some(95),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x262a820,
        name: "Red Branch Shortbow",
        weapon_type: Some(50),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1dd7290,
        name: "Red Crest Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1cad4f0,
        name: "Red Thorn Roundshield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ebca70,
        name: "Redmane Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xfde80,
        name: "Reduvia",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x21dfe0,
        name: "Regalia of Eochaid",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4064600,
        name: "Rellana's Twin Blades",
        weapon_type: Some(93),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x297c1e0,
        name: "Repeating Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ca38b0,
        name: "Rickety Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1cb7130,
        name: "Rift Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa9d490,
        name: "Ringed Finger",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xd662d0,
        name: "Ripple Blade",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x11392e0,
        name: "Ripple Crescent Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x89f080,
        name: "Rivers of Blood",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1cb2310,
        name: "Riveted Wooden Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4cc070,
        name: "Rogier's Rapier",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xd77440,
        name: "Rosus' Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xba4f50,
        name: "Rotten Battle Hammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf69500,
        name: "Rotten Crystal Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fba8f0,
        name: "Rotten Crystal Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x227c20,
        name: "Rotten Crystal Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1613db0,
        name: "Rotten Greataxe",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16116a0,
        name: "Rotten Staff",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1da1730,
        name: "Round Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3df360,
        name: "Royal Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3e4180,
        name: "Ruins Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe5cc20,
        name: "Rusted Anchor",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2f4d60,
        name: "Sacred Relic Sword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xd74d30,
        name: "Sacrificial Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6c0840,
        name: "Scavenger's Curved Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa98670,
        name: "Scepter of the All-Knowing",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6cf2a0,
        name: "Scimitar",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1da3e40,
        name: "Scorpion Kite Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x107ac0,
        name: "Scorpion's Stinger",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1cafc00,
        name: "Scripture Wooden Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x121eac0,
        name: "Scythe",
        weapon_type: Some(31),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16f4770,
        name: "Sentry's Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2723880,
        name: "Serpent Bow",
        weapon_type: Some(51),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e0f500,
        name: "Serpent Crest Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xcdfe60,
        name: "Serpent Flail",
        weapon_type: Some(24),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x8a8cc0,
        name: "Serpentbone Blade",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6c7d70,
        name: "Serpent-God's Curved Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x103db70,
        name: "Serpent-Hunter",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x166bbf0,
        name: "Shadow Sunflower Blossom",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x6b44f0,
        name: "Shamshir",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x148d3b0,
        name: "Shield of Night",
        weapon_type: Some(65),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1db9dd0,
        name: "Shield of the Guilty",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf42400,
        name: "Short Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1eab90,
        name: "Short Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2625a00,
        name: "Shortbow",
        weapon_type: Some(50),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6b1de0,
        name: "Shotel",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x103b460,
        name: "Siluria's Tree",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1d9f020,
        name: "Silver Mirrorshield",
        weapon_type: Some(67),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xdd40a0,
        name: "Smithscript Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3d85830,
        name: "Smithscript Cirque",
        weapon_type: Some(92),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3c8eee0,
        name: "Smithscript Dagger",
        weapon_type: Some(91),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xbebc20,
        name: "Smithscript Greathammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1d18bb0,
        name: "Smithscript Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xfbc520,
        name: "Smithscript Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1cc0d70,
        name: "Smoldering Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x29020c0,
        name: "Soldier's Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf44b10,
        name: "Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x10b0760,
        name: "Spear of the Impaler",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1409650,
        name: "Spiked Caestus",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa8ea30,
        name: "Spiked Club",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1eae010,
        name: "Spiked Palisade Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf646e0,
        name: "Spiked Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1cca9b0,
        name: "Spiralhorn Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x20ebbc0,
        name: "Spiraltree Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x11a49a0,
        name: "Spirit Glaive",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x7270e0,
        name: "Spirit Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x297e8f0,
        name: "Spread Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x16ed240,
        name: "St. Trina's Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fbd000,
        name: "Staff of Loss",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1600530,
        name: "Staff of the Avatar",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ff5270,
        name: "Staff of the Great Beyond",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1fb81e0,
        name: "Staff of the Guilty",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x141a7c0,
        name: "Star Fist",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2673c0,
        name: "Star-Lined Sword",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3dcc50,
        name: "Starscourge Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16e8420,
        name: "Steel-Wire Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa9fba0,
        name: "Stone Club",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x26c1e0,
        name: "Stone-Sheathed Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xd7c260,
        name: "Stormhawk Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1d9c910,
        name: "Sun Realm Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xfc1340,
        name: "Swift Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3567e0,
        name: "Sword Lance",
        weapon_type: Some(16),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x271000,
        name: "Sword of Darkness",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x26e8f0,
        name: "Sword of Light",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3037c0,
        name: "Sword of Milos",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x90f560,
        name: "Sword of Night",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x20a760,
        name: "Sword of Night and Flame",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x216ab0,
        name: "Sword of St. Trina",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1481060,
        name: "Thiollier's Hidden Needle",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1317b20,
        name: "Thorned Whip",
        weapon_type: Some(39),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x138ce20,
        name: "Tooth Whip",
        weapon_type: Some(39),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x16e3600,
        name: "Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xf55c80,
        name: "Torchpole",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x10477b0,
        name: "Treespear",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3eb6b0,
        name: "Troll Knight's Sword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3d7e30,
        name: "Troll's Golden Sword",
        weapon_type: Some(7),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x160ef90,
        name: "Troll's Hammer",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1da6550,
        name: "Twinbird Kite Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x989680,
        name: "Twinblade",
        weapon_type: Some(14),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x990bb0,
        name: "Twinned Knight Swords",
        weapon_type: Some(14),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x895440,
        name: "Uchigatana",
        weapon_type: Some(13),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1323e70,
        name: "Urumi",
        weapon_type: Some(39),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa8c320,
        name: "Varré's Bouquet",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x264cb0,
        name: "Velvet Sword of St. Trina",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x14fd890,
        name: "Venomous Fang",
        weapon_type: Some(37),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f03740,
        name: "Verdigris Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1424400,
        name: "Veteran's Prosthesis",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1eab900,
        name: "Visage Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x113b9f0,
        name: "Vulgar Militia Saw",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x114a450,
        name: "Vulgar Militia Shotel",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1042990,
        name: "Vyke's War Spear",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x10c8e0,
        name: "Wakizashi",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x222e00,
        name: "Warhawk's Talon",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe508d0,
        name: "Warped Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xa87500,
        name: "Warpick",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3d3010,
        name: "Watchdog's Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15f1ad0,
        name: "Watchdog's Staff",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1f47d0,
        name: "Weathered Straight Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1312d00,
        name: "Whip",
        weapon_type: Some(39),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6be130,
        name: "Wing of Astel",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xe68f70,
        name: "Winged Greathorn",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x122d520,
        name: "Winged Scythe",
        weapon_type: Some(31),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1e0cdf0,
        name: "Wolf Crest Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ecb4d0,
        name: "Wooden Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7ad550,
        name: "Zamor Curved Sword",
        weapon_type: Some(11),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3da540,
        name: "Zweihander",
        weapon_type: Some(7),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
];
