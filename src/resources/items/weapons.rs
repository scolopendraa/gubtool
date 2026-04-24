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
        id: 0x1FA9780,
        name: "Academy Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2ED830,
        name: "Alabaster Lord's Sword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x271C350,
        name: "Albinauric Bow",
        weapon_type: Some(51),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1D9A200,
        name: "Albinauric Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FA7070,
        name: "Albinauric Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x44AA20,
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
        id: 0x1EBA360,
        name: "Ant's Skull Plate",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4CE780,
        name: "Antspur Rapier",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xBEE330,
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
        id: 0x1F98610,
        name: "Astrologer's Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15FB710,
        name: "Axe of Godfrey",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE57E00,
        name: "Axe of Godrick",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FB0CB0,
        name: "Azur's Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3D83120,
        name: "Backhand Blade",
        weapon_type: Some(92),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x6B6C00,
        name: "Bandit's Curved Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2EFF40,
        name: "Banished Knight's Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1131DB0,
        name: "Banished Knight's Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1D97AF0,
        name: "Banished Knight's Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x10B5580,
        name: "Barbed Staff-Spear",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2DC6C0,
        name: "Bastard Sword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xC6D270,
        name: "Bastard's Stars",
        weapon_type: Some(24),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xD59F80,
        name: "Battle Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB76920,
        name: "Battle Hammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4153A20,
        name: "Beast Claw",
        weapon_type: Some(95),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DD4B80,
        name: "Beast Crest Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB964F0,
        name: "Beastclaw Greathammer",
        weapon_type: Some(23),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7B4A80,
        name: "Beastman's Cleaver",
        weapon_type: Some(11),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6AF6D0,
        name: "Beastman's Curved Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CAADE0,
        name: "Beastman's Jar-Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16F2060,
        name: "Beast-Repellent Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x272ADB0,
        name: "Black Bow",
        weapon_type: Some(51),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF6950,
        name: "Black Knife",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DE35E0,
        name: "Black Leather Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xBF0A40,
        name: "Black Steel Greathammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EFE920,
        name: "Black Steel Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xA05EB0,
        name: "Black Steel Twinblade",
        weapon_type: Some(14),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x11B340,
        name: "Blade of Calling",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2FE9A0,
        name: "Blasphemous Blade",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xBF3150,
        name: "Bloodfiend's Arm",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xFC6160,
        name: "Bloodfiend's Fork",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xFC8870,
        name: "Bloodfiend's Sacred Spear",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x14FFFA0,
        name: "Bloodhound Claws",
        weapon_type: Some(37),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7A8730,
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
        id: 0x5B8D80,
        name: "Bloody Helice",
        weapon_type: Some(16),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DD99A0,
        name: "Blue Crest Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DA8C60,
        name: "Blue-Gold Kite Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CB4A20,
        name: "Blue-White Wooden Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF58390,
        name: "Bolt of Gransax",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x269FB20,
        name: "Bone Bow",
        weapon_type: Some(50),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xECA9F0,
        name: "Bonny Butchering Knife",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DB0190,
        name: "Brass Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E90B50,
        name: "Briar Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xBA0130,
        name: "Brick Hammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ED2A0,
        name: "Broadsword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1C9C380,
        name: "Buckler",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE6B680,
        name: "Butchering Knife",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1406F40,
        name: "Caestus",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DCD650,
        name: "Candletree Wooden Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x21B8D0,
        name: "Cane Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FA2250,
        name: "Carian Glintblade Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FABE90,
        name: "Carian Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DBEBF0,
        name: "Carian Knight's Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2143A0,
        name: "Carian Knight's Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F8E9D0,
        name: "Carian Regal Scepter",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x269AD0,
        name: "Carian Sorcery Sword",
        weapon_type: Some(15),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3B9D3B0,
        name: "Carian Thrusting Shield",
        weapon_type: Some(90),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xD689E0,
        name: "Celebrant's Cleaver",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF50E60,
        name: "Celebrant's Rib-Rake",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x102CA0,
        name: "Celebrant's Sickle",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB916D0,
        name: "Celebrant's Skull",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xC6F980,
        name: "Chainlink Flail",
        weapon_type: Some(24),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3AA9170,
        name: "Chilling Perfume Bottle",
        weapon_type: Some(89),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x10EFF0,
        name: "Cinquedea",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1426B10,
        name: "Cipher Pata",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x20768C0,
        name: "Clawmark Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15752A0,
        name: "Claws of Night",
        weapon_type: Some(37),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xF49930,
        name: "Clayman's Harpoon",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3085E0,
        name: "Claymore",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4C7250,
        name: "Cleanrot Knight's Sword",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF4C040,
        name: "Cleanrot Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1421CF0,
        name: "Clinging Bone",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA7FFD0,
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
        id: 0x1CCD0C0,
        name: "Coil Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x11344C0,
        name: "Commander's Standard",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2631D50,
        name: "Composite Bow",
        weapon_type: Some(50),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB9B310,
        name: "Cranial Vessel Candlestand",
        weapon_type: Some(23),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x291CE70,
        name: "Crepus's Black-Key Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE556F0,
        name: "Crescent Moon Axe",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EB5540,
        name: "Crossed-Tree Towershield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF5D1B0,
        name: "Cross-Naginata",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E8BD30,
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
        id: 0xF47220,
        name: "Crystal Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F82680,
        name: "Crystal Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x20CE70,
        name: "Crystal Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EC1890,
        name: "Cuckoo Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3D87F40,
        name: "Curseblade's Cirque",
        weapon_type: Some(92),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xA84DF0,
        name: "Curved Club",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB85380,
        name: "Curved Great Club",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF4240,
        name: "Dagger",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x72BF00,
        name: "Dancing Blade of Ranah",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x39B4F30,
        name: "Dane's Footwork",
        weapon_type: Some(88),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2F2650,
        name: "Dark Moon Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3AB06A0,
        name: "Deadly Poison Perfume Bottle",
        weapon_type: Some(89),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xEC82E0,
        name: "Death Knight's Longhaft Axe",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xDD67B0,
        name: "Death Knight's Twin Axes",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xF5F8C0,
        name: "Death Ritual Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x30D400,
        name: "Death's Poker",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F874A0,
        name: "Demi-Human Queen's Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16694E0,
        name: "Devonia's Hammer",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xBA2840,
        name: "Devourer's Scepter",
        weapon_type: Some(23),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F95F00,
        name: "Digger's Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7A6020,
        name: "Dismounter",
        weapon_type: Some(11),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E89620,
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
        id: 0x15FDE20,
        name: "Dragon Greatclaw",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x114CB60,
        name: "Dragon Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x5C29C0,
        name: "Dragon King's Cragblade",
        weapon_type: Some(16),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E84800,
        name: "Dragon Towershield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E8E440,
        name: "Dragonclaw Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3F6DCB0,
        name: "Dragon-Hunter's Great Katana",
        weapon_type: Some(94),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x8A65B0,
        name: "Dragonscale Blade",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x39B2820,
        name: "Dryleaf Arts",
        weapon_type: Some(88),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x20E6DA0,
        name: "Dryleaf Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3B9ACA0,
        name: "Dueling Shield",
        weapon_type: Some(90),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x15F9000,
        name: "Duelist Greataxe",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EBF180,
        name: "Eclipse Crest Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DDC0B0,
        name: "Eclipse Crest Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6C5660,
        name: "Eclipse Shotel",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x9959D0,
        name: "Eleonora's Poleblade",
        weapon_type: Some(14),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15F68F0,
        name: "Envoy's Greathorn",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA95F60,
        name: "Envoy's Horn",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB98C00,
        name: "Envoy's Long Horn",
        weapon_type: Some(23),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x118C30,
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
        id: 0x28153B0,
        name: "Erdtree Greatbow",
        weapon_type: Some(53),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E98080,
        name: "Erdtree Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x207DDF0,
        name: "Erdtree Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4C4B40,
        name: "Estoc",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA037A0,
        name: "Euporia",
        weapon_type: Some(14),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xE61A40,
        name: "Executioner's Greataxe",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6ACFC0,
        name: "Falchion",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1602C40,
        name: "Fallingstar Beast Jaw",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7297F0,
        name: "Falx",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xC6AB60,
        name: "Family Heads",
        weapon_type: Some(24),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x206CC80,
        name: "Finger Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EA43D0,
        name: "Fingerprint Stone Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x44F840,
        name: "Fire Knight's Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x20E94B0,
        name: "Fire Knight's Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x170A70,
        name: "Fire Knight's Shortsword",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3AA6A60,
        name: "Firespark Perfume Bottle",
        weapon_type: Some(89),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xC68450,
        name: "Flail",
        weapon_type: Some(24),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2E8A10,
        name: "Flamberge",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DCFD60,
        name: "Flame Crest Wooden Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xAF79E0,
        name: "Flowerstone Gavel",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x6BBA20,
        name: "Flowing Curved Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2DEDD0,
        name: "Forked Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xD5C690,
        name: "Forked Hatchet",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xDDDCE0,
        name: "Forked-Tongue Hatchet",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2082C10,
        name: "Frenzied Flame Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3AAB880,
        name: "Frenzyflame Perfume Bottle",
        weapon_type: Some(89),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x81DA30,
        name: "Freyja's Greatsword",
        weapon_type: Some(11),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x4D0E90,
        name: "Frozen Needle",
        weapon_type: Some(15),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2910B20,
        name: "Full Moon Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE704A0,
        name: "Gargoyle's Black Axe",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x99F610,
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
        id: 0x30FB10,
        name: "Gargoyle's Blackblade",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE6DD90,
        name: "Gargoyle's Great Axe",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x30ACF0,
        name: "Gargoyle's Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x114F270,
        name: "Gargoyle's Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x99CF00,
        name: "Gargoyle's Twinblade",
        weapon_type: Some(14),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x166E300,
        name: "Gazing Finger",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F84D90,
        name: "Gelmir Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1607A60,
        name: "Ghiza's Wheel",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16EF950,
        name: "Ghostflame Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x160A170,
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
        id: 0x2071AA0,
        name: "Giant's Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EC66B0,
        name: "Gilded Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CBBF50,
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
        id: 0x1053B0,
        name: "Glintstone Kris",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F78A40,
        name: "Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x98BD90,
        name: "Godskin Peeler",
        weapon_type: Some(14),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x5BB490,
        name: "Godskin Stitcher",
        weapon_type: Some(16),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3E1A70,
        name: "Godslayer's Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x206F390,
        name: "Godslayer's Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E9A790,
        name: "Golden Beast Crest Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F95F0,
        name: "Golden Epitaph",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EC3FA0,
        name: "Golden Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x113E100,
        name: "Golden Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E11C10,
        name: "Golden Lion Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x305ED0,
        name: "Golden Order Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x207B6E0,
        name: "Golden Order Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x148ACA0,
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
        id: 0x160C880,
        name: "Golem's Halberd",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3E8FA0,
        name: "Grafted Blade Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x14159A0,
        name: "Grafted Dragon",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x12211D0,
        name: "Grave Scythe",
        weapon_type: Some(31),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x20741B0,
        name: "Gravel Stone Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15F41E0,
        name: "Great Club",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x5BDBA0,
        name: "Great Épée",
        weapon_type: Some(16),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3F6B5A0,
        name: "Great Katana",
        weapon_type: Some(94),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x10A1D0,
        name: "Great Knife",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB80560,
        name: "Great Mace",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE52FE0,
        name: "Great Omenkiller Cleaver",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB9DA20,
        name: "Great Stars",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DB28A0,
        name: "Great Turtle Shell",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE4E1C0,
        name: "Greataxe",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2817AC0,
        name: "Greatbow",
        weapon_type: Some(53),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB74210,
        name: "Greathorn Hammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3D0900,
        name: "Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x358EF0,
        name: "Greatsword of Damnation",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x456D70,
        name: "Greatsword of Radahn (Light)",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x451F50,
        name: "Greatsword of Radahn (Lord)",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x362B30,
        name: "Greatsword of Solitude",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x6D19B0,
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
        id: 0x112A880,
        name: "Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EC8DC0,
        name: "Haligtree Crest Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x12238E0,
        name: "Halo Scythe",
        weapon_type: Some(31),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA91140,
        name: "Hammer",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xD5EDA0,
        name: "Hand Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x29F6300,
        name: "Hand Ballista",
        weapon_type: Some(56),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x89A260,
        name: "Hand of Malenia",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x262CF30,
        name: "Harp Bow",
        weapon_type: Some(50),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DD2470,
        name: "Hawk Crest Wooden Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DE0ED0,
        name: "Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x29095F0,
        name: "Heavy Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2FC290,
        name: "Helphen's Steeple",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xD72620,
        name: "Highland Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x14FB180,
        name: "Hookclaws",
        weapon_type: Some(37),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x271EA60,
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
        id: 0x72E610,
        name: "Horned Warrior's Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DCAF40,
        name: "Horse Crest Wooden Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x131F050,
        name: "Hoslow's Petal Whip",
        weapon_type: Some(39),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CBE660,
        name: "Ice Crest Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xD6D800,
        name: "Icerind Hatchet",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EA6AE0,
        name: "Icon Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2887FA0,
        name: "Igon's Greatbow",
        weapon_type: Some(53),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xF61FD0,
        name: "Inquisitor's Girandole",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FE410,
        name: "Inseparable Sword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DDE7C0,
        name: "Inverted Hawk Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EB7C50,
        name: "Inverted Hawk Towershield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x14180B0,
        name: "Iron Ball",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xD63BC0,
        name: "Iron Cleaver",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2E14E0,
        name: "Iron Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CB9840,
        name: "Iron Roundshield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF66DF0,
        name: "Iron Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x113E10,
        name: "Ivory Sickle",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x29F8A10,
        name: "Jar Cannon",
        weapon_type: Some(56),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xD614B0,
        name: "Jawbone Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EA1CC0,
        name: "Jellyfish Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x141F5E0,
        name: "Katar",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1D905C0,
        name: "Kite Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2E6300,
        name: "Knight's Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x175FE30,
        name: "Lamenting Visage",
        weapon_type: Some(87),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x10450A0,
        name: "Lance",
        weapon_type: Some(28),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB71B00,
        name: "Large Club",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DC8830,
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
        id: 0x4061EF0,
        name: "Leda's Sword",
        weapon_type: Some(93),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2906EE0,
        name: "Light Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3AADF90,
        name: "Lightning Perfume Bottle",
        weapon_type: Some(89),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x280DE80,
        name: "Lion Greatbow",
        weapon_type: Some(53),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x35B600,
        name: "Lizard Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2719C40,
        name: "Longbow",
        weapon_type: Some(51),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE5A510,
        name: "Longhaft Axe",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E8480,
        name: "Longsword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2E3BF0,
        name: "Lordsworn's Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ECDBE0,
        name: "Lordsworn's Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F20C0,
        name: "Lordsworn's Straight Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1142F20,
        name: "Loretta's War Sickle",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x112F6A0,
        name: "Lucerne",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FB33C0,
        name: "Lusat's Glintstone Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA7D8C0,
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
        id: 0x6B9310,
        name: "Magma Blade",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x131A230,
        name: "Magma Whip Candlestick",
        weapon_type: Some(39),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7AAE40,
        name: "Magma Wyrm's Scalesword",
        weapon_type: Some(11),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16E360,
        name: "Main-gauche",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3D5720,
        name: "Maliketh's Black Blade",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EB2E30,
        name: "Manor Towershield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CA11A0,
        name: "Man-Serpent's Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6CA480,
        name: "Mantis Blade",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3010B0,
        name: "Marais Executioner's Sword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xAA22B0,
        name: "Marika's Hammer",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1D92CD0,
        name: "Marred Leather Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1D953E0,
        name: "Marred Wooden Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FF7980,
        name: "Maternal Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E0A6E0,
        name: "Messmer Soldier Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xDD8EC0,
        name: "Messmer Soldier's Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x10B2E70,
        name: "Messmer Soldier's Spear",
        weapon_type: Some(28),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x89C970,
        name: "Meteoric Ore Blade",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FB5AD0,
        name: "Meteorite Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(2),
        ..Item::default_weapons()
    },
    Item {
        id: 0x405F7E0,
        name: "Milady",
        weapon_type: Some(93),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2191C0,
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
        id: 0xFB770,
        name: "Miséricorde",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1038D50,
        name: "Mohgwyn's Sacred Spear",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7B2370,
        name: "Monk's Flameblade",
        weapon_type: Some(11),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA93850,
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
        id: 0x8A3EA0,
        name: "Moonveil",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7B98A0,
        name: "Morgott's Cursed Sword",
        weapon_type: Some(11),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA89C10,
        name: "Morning Star",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x897B50,
        name: "Nagakiba",
        weapon_type: Some(13),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x175D720,
        name: "Nanaya's Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xC65D40,
        name: "Nightrider Flail",
        weapon_type: Some(24),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1136BD0,
        name: "Nightrider Glaive",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4D35A0,
        name: "Noble's Estoc",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2206F0,
        name: "Noble's Slender Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA9AD80,
        name: "Nox Flowing Hammer",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FBD00,
        name: "Nox Flowing Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1298BE0,
        name: "Obsidian Lamina",
        weapon_type: Some(31),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x7AFC60,
        name: "Omen Cleaver",
        weapon_type: Some(11),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EA91F0,
        name: "One-Eyed Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7A3910,
        name: "Onyx Lord's Greatsword",
        weapon_type: Some(11),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2EB120,
        name: "Ordovis's Greatsword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F6EE0,
        name: "Ornamental Straight Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF9060,
        name: "Parrying Dagger",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF4E750,
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
        id: 0x1C9EA90,
        name: "Perfumer's Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x112CF90,
        name: "Pest's Glaive",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xB93DE0,
        name: "Pickaxe",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF53570,
        name: "Pike",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CA5FC0,
        name: "Pillory Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1485E80,
        name: "Poisoned Hand",
        weapon_type: Some(35),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x11A70B0,
        name: "Poleblade of the Bud",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x15EF3C0,
        name: "Prelate's Inferno Crozier",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FA4960,
        name: "Prince of Death's Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x27286A0,
        name: "Pulley Bow",
        weapon_type: Some(51),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x290E410,
        name: "Pulley Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x81B320,
        name: "Putrescence Cleaver",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x632EA0,
        name: "Queelign's Greatsword",
        weapon_type: Some(16),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x2A70420,
        name: "Rabbath's Cannon",
        weapon_type: Some(56),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3F703C0,
        name: "Rakshasa's Great Katana",
        weapon_type: Some(94),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x4C9960,
        name: "Rapier",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15026B0,
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
        id: 0x262A820,
        name: "Red Branch Shortbow",
        weapon_type: Some(50),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DD7290,
        name: "Red Crest Heater Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CAD4F0,
        name: "Red Thorn Roundshield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EBCA70,
        name: "Redmane Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xFDE80,
        name: "Reduvia",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x21DFE0,
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
        id: 0x297C1E0,
        name: "Repeating Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CA38B0,
        name: "Rickety Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CB7130,
        name: "Rift Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA9D490,
        name: "Ringed Finger",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xD662D0,
        name: "Ripple Blade",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x11392E0,
        name: "Ripple Crescent Halberd",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x89F080,
        name: "Rivers of Blood",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CB2310,
        name: "Riveted Wooden Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x4CC070,
        name: "Rogier's Rapier",
        weapon_type: Some(15),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xD77440,
        name: "Rosus' Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xBA4F50,
        name: "Rotten Battle Hammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF69500,
        name: "Rotten Crystal Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FBA8F0,
        name: "Rotten Crystal Staff",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x227C20,
        name: "Rotten Crystal Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1613DB0,
        name: "Rotten Greataxe",
        weapon_type: Some(41),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16116A0,
        name: "Rotten Staff",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DA1730,
        name: "Round Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3DF360,
        name: "Royal Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3E4180,
        name: "Ruins Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE5CC20,
        name: "Rusted Anchor",
        weapon_type: Some(19),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2F4D60,
        name: "Sacred Relic Sword",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xD74D30,
        name: "Sacrificial Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6C0840,
        name: "Scavenger's Curved Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA98670,
        name: "Scepter of the All-Knowing",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6CF2A0,
        name: "Scimitar",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DA3E40,
        name: "Scorpion Kite Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x107AC0,
        name: "Scorpion's Stinger",
        weapon_type: Some(1),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CAFC00,
        name: "Scripture Wooden Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x121EAC0,
        name: "Scythe",
        weapon_type: Some(31),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16F4770,
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
        id: 0x1E0F500,
        name: "Serpent Crest Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xCDFE60,
        name: "Serpent Flail",
        weapon_type: Some(24),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x8A8CC0,
        name: "Serpentbone Blade",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6C7D70,
        name: "Serpent-God's Curved Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x103DB70,
        name: "Serpent-Hunter",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x166BBF0,
        name: "Shadow Sunflower Blossom",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x6B44F0,
        name: "Shamshir",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x148D3B0,
        name: "Shield of Night",
        weapon_type: Some(65),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DB9DD0,
        name: "Shield of the Guilty",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF42400,
        name: "Short Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EAB90,
        name: "Short Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2625A00,
        name: "Shortbow",
        weapon_type: Some(50),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6B1DE0,
        name: "Shotel",
        weapon_type: Some(9),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x103B460,
        name: "Siluria's Tree",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1D9F020,
        name: "Silver Mirrorshield",
        weapon_type: Some(67),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xDD40A0,
        name: "Smithscript Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3D85830,
        name: "Smithscript Cirque",
        weapon_type: Some(92),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3C8EEE0,
        name: "Smithscript Dagger",
        weapon_type: Some(91),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xBEBC20,
        name: "Smithscript Greathammer",
        weapon_type: Some(23),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1D18BB0,
        name: "Smithscript Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xFBC520,
        name: "Smithscript Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CC0D70,
        name: "Smoldering Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x29020C0,
        name: "Soldier's Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF44B10,
        name: "Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x10B0760,
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
        id: 0xA8EA30,
        name: "Spiked Club",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1EAE010,
        name: "Spiked Palisade Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF646E0,
        name: "Spiked Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1CCA9B0,
        name: "Spiralhorn Shield",
        weapon_type: Some(65),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x20EBBC0,
        name: "Spiraltree Seal",
        weapon_type: Some(61),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x11A49A0,
        name: "Spirit Glaive",
        weapon_type: Some(29),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x7270E0,
        name: "Spirit Sword",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x297E8F0,
        name: "Spread Crossbow",
        weapon_type: Some(55),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x16ED240,
        name: "St. Trina's Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FBD000,
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
        id: 0x1FF5270,
        name: "Staff of the Great Beyond",
        weapon_type: Some(57),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x1FB81E0,
        name: "Staff of the Guilty",
        weapon_type: Some(57),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x141A7C0,
        name: "Star Fist",
        weapon_type: Some(35),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x2673C0,
        name: "Star-Lined Sword",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3DCC50,
        name: "Starscourge Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x16E8420,
        name: "Steel-Wire Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA9FBA0,
        name: "Stone Club",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x26C1E0,
        name: "Stone-Sheathed Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0xD7C260,
        name: "Stormhawk Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1D9C910,
        name: "Sun Realm Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xFC1340,
        name: "Swift Spear",
        weapon_type: Some(25),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3567E0,
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
        id: 0x26E8F0,
        name: "Sword of Light",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x3037C0,
        name: "Sword of Milos",
        weapon_type: Some(5),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x90F560,
        name: "Sword of Night",
        weapon_type: Some(13),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x20A760,
        name: "Sword of Night and Flame",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x216AB0,
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
        id: 0x1317B20,
        name: "Thorned Whip",
        weapon_type: Some(39),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x138CE20,
        name: "Tooth Whip",
        weapon_type: Some(39),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x16E3600,
        name: "Torch",
        weapon_type: Some(87),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xF55C80,
        name: "Torchpole",
        weapon_type: Some(25),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x10477B0,
        name: "Treespear",
        weapon_type: Some(28),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3EB6B0,
        name: "Troll Knight's Sword",
        weapon_type: Some(7),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3D7E30,
        name: "Troll's Golden Sword",
        weapon_type: Some(7),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x160EF90,
        name: "Troll's Hammer",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1DA6550,
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
        id: 0x990BB0,
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
        id: 0x1323E70,
        name: "Urumi",
        weapon_type: Some(39),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA8C320,
        name: "Varré's Bouquet",
        weapon_type: Some(21),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x264CB0,
        name: "Velvet Sword of St. Trina",
        weapon_type: Some(3),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        dlc: true,
        ..Item::default_weapons()
    },
    Item {
        id: 0x14FD890,
        name: "Venomous Fang",
        weapon_type: Some(37),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F03740,
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
        id: 0x1EAB900,
        name: "Visage Shield",
        weapon_type: Some(69),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x113B9F0,
        name: "Vulgar Militia Saw",
        weapon_type: Some(29),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x114A450,
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
        id: 0x10C8E0,
        name: "Wakizashi",
        weapon_type: Some(1),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x222E00,
        name: "Warhawk's Talon",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE508D0,
        name: "Warped Axe",
        weapon_type: Some(17),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0xA87500,
        name: "Warpick",
        weapon_type: Some(21),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3D3010,
        name: "Watchdog's Greatsword",
        weapon_type: Some(7),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x15F1AD0,
        name: "Watchdog's Staff",
        weapon_type: Some(41),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1F47D0,
        name: "Weathered Straight Sword",
        weapon_type: Some(3),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1312D00,
        name: "Whip",
        weapon_type: Some(39),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x6BE130,
        name: "Wing of Astel",
        weapon_type: Some(9),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0xE68F70,
        name: "Winged Greathorn",
        weapon_type: Some(19),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x122D520,
        name: "Winged Scythe",
        weapon_type: Some(31),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1E0CDF0,
        name: "Wolf Crest Shield",
        weapon_type: Some(67),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x1ECB4D0,
        name: "Wooden Greatshield",
        weapon_type: Some(69),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
    Item {
        id: 0x7AD550,
        name: "Zamor Curved Sword",
        weapon_type: Some(11),
        gem_mount_type: Some(0),
        upgrade_type: Some(1),
        ..Item::default_weapons()
    },
    Item {
        id: 0x3DA540,
        name: "Zweihander",
        weapon_type: Some(7),
        gem_mount_type: Some(2),
        upgrade_type: Some(0),
        ..Item::default_weapons()
    },
];
