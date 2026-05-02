use super::Item;

impl Item {
    const fn default_weapon() -> Self {
        Self {
            category: super::Categories::Weapons,
            stack_size: 1,
            ..Item::default()
        }
    }
}

pub static WEAPONS: &[Item; 325] = &[
    Item {
        id: 5225000,
        name: "Aged Smelter Sword",
        max_upgrade: Some(5),
        infuse_id: None,
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 2540000,
        name: "Aldia Hammer",
        max_upgrade: Some(10),
        infuse_id: Some(101500),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 4400000,
        name: "Alonne Greatbow",
        max_upgrade: Some(10),
        infuse_id: Some(102200),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 5010000,
        name: "Arced Sword",
        max_upgrade: Some(5),
        infuse_id: Some(111400),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 4060000,
        name: "Archdrake Chime",
        max_upgrade: Some(10),
        infuse_id: Some(102500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 2720000,
        name: "Archdrake Mace",
        max_upgrade: Some(10),
        infuse_id: Some(101600),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11250000,
        name: "Archdrake Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 3860000,
        name: "Archdrake Staff",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 1380000,
        name: "Ashen Warrior Sword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 4660000,
        name: "Avelyn",
        max_upgrade: Some(10),
        infuse_id: Some(102300),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 3930000,
        name: "Azal's Staff",
        max_upgrade: Some(5),
        infuse_id: None,
        durability: Some(4),
        ..Item::default_weapon()
    },
    Item {
        id: 2020000,
        name: "Bandit Axe",
        max_upgrade: Some(10),
        infuse_id: Some(101700),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 2220000,
        name: "Bandit Greataxe",
        max_upgrade: Some(10),
        infuse_id: Some(101800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1010000,
        name: "Bandit's Knife",
        max_upgrade: Some(10),
        infuse_id: Some(100400),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2560000,
        name: "Barbed Club",
        max_upgrade: Some(5),
        infuse_id: Some(111500),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1800000,
        name: "Bastard Sword",
        max_upgrade: Some(10),
        infuse_id: Some(101200),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 3870000,
        name: "Bat Staff",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 2010000,
        name: "Battle Axe",
        max_upgrade: Some(10),
        infuse_id: Some(101700),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 4270000,
        name: "Bell Keeper Bow",
        max_upgrade: Some(10),
        infuse_id: Some(102100),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 11470000,
        name: "Bell Keeper Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11005000,
        name: "Benhart's Parma",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1770000,
        name: "Berserker Blade",
        max_upgrade: Some(5),
        infuse_id: Some(121100),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 1790000,
        name: "Bewitched Alonne Sword",
        max_upgrade: Some(5),
        infuse_id: Some(111100),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 2290000,
        name: "Black Dragon Greataxe",
        max_upgrade: Some(5),
        infuse_id: Some(111800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1920000,
        name: "Black Dragon Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111200),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 11480000,
        name: "Black Dragon Shield",
        max_upgrade: Some(5),
        infuse_id: Some(112800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1350000,
        name: "Black Dragon Sword",
        max_upgrade: Some(5),
        infuse_id: Some(111000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2530000,
        name: "Black Dragon Warpick",
        max_upgrade: Some(5),
        infuse_id: Some(111500),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 5500000,
        name: "Black Flamestone Dagger",
        max_upgrade: Some(10),
        infuse_id: Some(100400),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11800000,
        name: "Black Flamestone Parma",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 2300000,
        name: "Black Knight Greataxe",
        max_upgrade: Some(5),
        infuse_id: Some(121800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1930000,
        name: "Black Knight Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(121200),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 3300000,
        name: "Black Knight Halberd",
        max_upgrade: Some(5),
        infuse_id: Some(122000),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 5290000,
        name: "Black Knight Ultra Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(121300),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1520000,
        name: "Black Scorpion Stinger",
        max_upgrade: Some(5),
        infuse_id: Some(120500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 4150000,
        name: "Black Witch's Staff",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 2520000,
        name: "Blacksmith's Hammer",
        max_upgrade: Some(10),
        infuse_id: Some(101500),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1730000,
        name: "Blacksteel Katana",
        max_upgrade: Some(10),
        infuse_id: Some(101100),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 3620000,
        name: "Bloodied Whip",
        max_upgrade: Some(10),
        infuse_id: Some(100300),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11730000,
        name: "Blossom Kite Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1140000,
        name: "Blue Dagger",
        max_upgrade: Some(5),
        infuse_id: Some(100400),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1280000,
        name: "Blue Flame",
        max_upgrade: Some(5),
        infuse_id: Some(121000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 3280000,
        name: "Blue Knight's Halberd",
        max_upgrade: Some(5),
        infuse_id: Some(122000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11210000,
        name: "Blue Wooden Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 1870000,
        name: "Bluemoon Greatsword",
        max_upgrade: Some(10),
        infuse_id: Some(101200),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 3530000,
        name: "Bone Fist",
        max_upgrade: Some(5),
        infuse_id: Some(120100),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 3060000,
        name: "Bone Scythe",
        max_upgrade: Some(5),
        infuse_id: Some(111900),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 11495000,
        name: "Bone Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 3880000,
        name: "Bone Staff",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 5520000,
        name: "Bound Hand Axe",
        max_upgrade: Some(10),
        infuse_id: Some(101700),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11820000,
        name: "Bound Wooden Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 4280000,
        name: "Bow of Want",
        max_upgrade: Some(5),
        infuse_id: Some(112100),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1230000,
        name: "Broadsword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1200000,
        name: "Broken Straight Sword",
        max_upgrade: Some(10),
        infuse_id: Some(100400),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 1070000,
        name: "Broken Thief Sword",
        max_upgrade: Some(10),
        infuse_id: Some(100400),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 11000000,
        name: "Buckler",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 2090000,
        name: "Butcher's Knife",
        max_upgrade: Some(5),
        infuse_id: Some(111700),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 3500000,
        name: "Caestus",
        max_upgrade: Some(10),
        infuse_id: Some(100100),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 4090000,
        name: "Caitha's Chime",
        max_upgrade: Some(10),
        infuse_id: Some(102500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 2890000,
        name: "Channeler's Trident",
        max_upgrade: Some(10),
        infuse_id: Some(100600),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 1720000,
        name: "Chaos Blade",
        max_upgrade: Some(5),
        infuse_id: Some(111100),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 1420000,
        name: "Chaos Rapier",
        max_upgrade: Some(5),
        infuse_id: Some(120500),
        durability: Some(45),
        ..Item::default_weapon()
    },
    Item {
        id: 11395000,
        name: "Chaos Shield",
        max_upgrade: Some(5),
        infuse_id: Some(122800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2940000,
        name: "Chariot Lance",
        max_upgrade: Some(5),
        infuse_id: None,
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1996000,
        name: "Charred Loyce Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(121200),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 11295000,
        name: "Charred Loyce Shield",
        max_upgrade: Some(5),
        infuse_id: Some(122800),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 4120000,
        name: "Chime of Screams",
        max_upgrade: Some(5),
        infuse_id: Some(112500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 4050000,
        name: "Chime of Want",
        max_upgrade: Some(5),
        infuse_id: Some(112500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 3410000,
        name: "Claws",
        max_upgrade: Some(10),
        infuse_id: Some(100200),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 1820000,
        name: "Claymore",
        max_upgrade: Some(10),
        infuse_id: Some(101200),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11110000,
        name: "Cleric's Parma",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 4010000,
        name: "Cleric's Sacred Chime",
        max_upgrade: Some(10),
        infuse_id: Some(102500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 11120000,
        name: "Cleric's Small Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2400000,
        name: "Club",
        max_upgrade: Some(10),
        infuse_id: Some(101500),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 4220000,
        name: "Composite Bow",
        max_upgrade: Some(10),
        infuse_id: Some(102100),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2440000,
        name: "Craftsman's Hammer",
        max_upgrade: Some(5),
        infuse_id: Some(121500),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 2200000,
        name: "Crescent Axe",
        max_upgrade: Some(5),
        infuse_id: Some(121700),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 3040000,
        name: "Crescent Sickle",
        max_upgrade: Some(10),
        infuse_id: Some(101900),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11091000,
        name: "Crimson Parma",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 5280000,
        name: "Crypt Blacksword",
        max_upgrade: Some(5),
        infuse_id: Some(111300),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 11140000,
        name: "Cursed Bone Shield",
        max_upgrade: Some(5),
        infuse_id: Some(122700),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 5040000,
        name: "Curved Dragon Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111400),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 5050000,
        name: "Curved Nil Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111400),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 5360000,
        name: "Curved Twinblade",
        max_upgrade: Some(5),
        infuse_id: Some(120800),
        durability: Some(125),
        ..Item::default_weapon()
    },
    Item {
        id: 1000000,
        name: "Dagger",
        max_upgrade: Some(10),
        infuse_id: Some(100400),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 5410000,
        name: "Dark Pyromancy Flame",
        max_upgrade: Some(10),
        infuse_id: None,
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1760000,
        name: "Darkdrift",
        max_upgrade: Some(10),
        infuse_id: Some(101100),
        durability: Some(20),
        ..Item::default_weapon()
    },
    Item {
        id: 1960000,
        name: "Defender Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111200),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11475000,
        name: "Defender's Shield",
        max_upgrade: Some(5),
        infuse_id: Some(112800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2710000,
        name: "Demon's Great Hammer",
        max_upgrade: Some(5),
        infuse_id: Some(121600),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 4110000,
        name: "Disc Chime",
        max_upgrade: Some(10),
        infuse_id: Some(102500),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 4040000,
        name: "Dragon Chime",
        max_upgrade: Some(5),
        infuse_id: Some(122500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 2680000,
        name: "Dragon Tooth",
        max_upgrade: Some(5),
        infuse_id: Some(121600),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 4240000,
        name: "Dragonrider Bow",
        max_upgrade: Some(5),
        infuse_id: Some(112100),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 11560000,
        name: "Dragonrider Greatshield",
        max_upgrade: Some(5),
        infuse_id: Some(112900),
        durability: Some(100),
        ..Item::default_weapon()
    },
    Item {
        id: 5330000,
        name: "Dragonrider Twinblade",
        max_upgrade: Some(5),
        infuse_id: Some(110800),
        durability: Some(120),
        ..Item::default_weapon()
    },
    Item {
        id: 3290000,
        name: "Dragonrider's Halberd",
        max_upgrade: Some(5),
        infuse_id: Some(112000),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 4420000,
        name: "Dragonslayer Greatbow",
        max_upgrade: Some(5),
        infuse_id: Some(112200),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2896000,
        name: "Dragonslayer Spear",
        max_upgrade: Some(5),
        infuse_id: None,
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2080000,
        name: "Dragonslayer's Crescent Axe",
        max_upgrade: Some(5),
        infuse_id: Some(121700),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1990000,
        name: "Drakeblood Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111200),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2740000,
        name: "Drakekeeper's Great Hammer",
        max_upgrade: Some(10),
        infuse_id: Some(101600),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 2310000,
        name: "Drakekeeper's Greataxe",
        max_upgrade: Some(10),
        infuse_id: Some(101800),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 11640000,
        name: "Drakekeeper's Greatshield",
        max_upgrade: Some(10),
        infuse_id: Some(102900),
        durability: Some(130),
        ..Item::default_weapon()
    },
    Item {
        id: 11485000,
        name: "Drakekeeper's Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(110),
        ..Item::default_weapon()
    },
    Item {
        id: 1370000,
        name: "Drakekeeper's Sword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 5275000,
        name: "Drakekeeper's Ultra Greatsword",
        max_upgrade: Some(10),
        infuse_id: Some(101300),
        durability: Some(110),
        ..Item::default_weapon()
    },
    Item {
        id: 3350000,
        name: "Drakekeeper's Warpick",
        max_upgrade: Some(10),
        infuse_id: Some(102000),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 5230000,
        name: "Drakewing Ultra Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111300),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 11230000,
        name: "Drangleic Shield",
        max_upgrade: Some(5),
        infuse_id: Some(122800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1850000,
        name: "Drangleic Sword",
        max_upgrade: Some(5),
        infuse_id: Some(121200),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1630000,
        name: "Eleum Loyce",
        max_upgrade: Some(5),
        infuse_id: Some(110900),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 1440000,
        name: "Espada Ropera",
        max_upgrade: Some(10),
        infuse_id: Some(100500),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 1400000,
        name: "Estoc",
        max_upgrade: Some(10),
        infuse_id: Some(100500),
        durability: Some(45),
        ..Item::default_weapon()
    },
    Item {
        id: 1600000,
        name: "Falchion",
        max_upgrade: Some(10),
        infuse_id: Some(100900),
        durability: Some(45),
        ..Item::default_weapon()
    },
    Item {
        id: 1810000,
        name: "Flamberge",
        max_upgrade: Some(10),
        infuse_id: Some(101200),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11030000,
        name: "Foot Soldier Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 1240000,
        name: "Foot Soldier Sword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(20),
        ..Item::default_weapon()
    },
    Item {
        id: 3020000,
        name: "Full Moon Sickle",
        max_upgrade: Some(10),
        infuse_id: Some(101900),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 1290000,
        name: "Fume Sword",
        max_upgrade: Some(5),
        infuse_id: Some(101000),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 5250000,
        name: "Fume Ultra Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111300),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2895000,
        name: "Gargoyle Bident",
        max_upgrade: Some(5),
        infuse_id: Some(110600),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2250000,
        name: "Giant Stone Axe",
        max_upgrade: Some(5),
        infuse_id: Some(111800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2690000,
        name: "Giant Warrior Club",
        max_upgrade: Some(10),
        infuse_id: Some(101600),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11050000,
        name: "Golden Falcon Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11310000,
        name: "Golden Wing Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2930000,
        name: "Grand Lance",
        max_upgrade: Some(10),
        infuse_id: Some(100700),
        durability: Some(105),
        ..Item::default_weapon()
    },
    Item {
        id: 11370000,
        name: "Grand Spirit Tree Shield",
        max_upgrade: Some(5),
        infuse_id: Some(122800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2620000,
        name: "Great Club",
        max_upgrade: Some(10),
        infuse_id: Some(101600),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 3010000,
        name: "Great Machete",
        max_upgrade: Some(10),
        infuse_id: Some(101900),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 3000000,
        name: "Great Scythe",
        max_upgrade: Some(10),
        infuse_id: Some(101900),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2210000,
        name: "Greataxe",
        max_upgrade: Some(10),
        infuse_id: Some(101800),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 11650000,
        name: "Greatshield of Glory",
        max_upgrade: Some(10),
        infuse_id: Some(102900),
        durability: Some(120),
        ..Item::default_weapon()
    },
    Item {
        id: 5210000,
        name: "Greatsword",
        max_upgrade: Some(10),
        infuse_id: Some(101300),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1997000,
        name: "Greatsword of the Forlorn",
        max_upgrade: Some(5),
        infuse_id: Some(121200),
        durability: Some(65),
        scholar_only: true,
        ..Item::default_weapon()
    },
    Item {
        id: 2070000,
        name: "Gyrm Axe",
        max_upgrade: Some(10),
        infuse_id: Some(101700),
        durability: Some(120),
        ..Item::default_weapon()
    },
    Item {
        id: 2630000,
        name: "Gyrm Great Hammer",
        max_upgrade: Some(10),
        infuse_id: Some(101600),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 2260000,
        name: "Gyrm Greataxe",
        max_upgrade: Some(10),
        infuse_id: Some(101800),
        durability: Some(140),
        ..Item::default_weapon()
    },
    Item {
        id: 11600000,
        name: "Gyrm Greatshield",
        max_upgrade: Some(10),
        infuse_id: Some(102900),
        durability: Some(100),
        ..Item::default_weapon()
    },
    Item {
        id: 3220000,
        name: "Halberd",
        max_upgrade: Some(10),
        infuse_id: Some(102000),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2000000,
        name: "Hand Axe",
        max_upgrade: Some(10),
        infuse_id: Some(101700),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 2500000,
        name: "Handmaid's Ladle",
        max_upgrade: Some(10),
        infuse_id: Some(101500),
        durability: Some(10),
        ..Item::default_weapon()
    },
    Item {
        id: 11590000,
        name: "Havel's Greatshield",
        max_upgrade: Some(5),
        infuse_id: Some(122900),
        durability: Some(120),
        ..Item::default_weapon()
    },
    Item {
        id: 4610000,
        name: "Heavy Crossbow",
        max_upgrade: Some(10),
        infuse_id: Some(102300),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2920000,
        name: "Heide Greatlance",
        max_upgrade: Some(10),
        infuse_id: Some(100700),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 1320000,
        name: "Heide Knight Sword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2900000,
        name: "Heide Lance",
        max_upgrade: Some(5),
        infuse_id: Some(120700),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2870000,
        name: "Heide Spear",
        max_upgrade: Some(10),
        infuse_id: Some(100600),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 3240000,
        name: "Helix Halberd",
        max_upgrade: Some(5),
        infuse_id: Some(122000),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 11420000,
        name: "Hollow Soldier Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 5530000,
        name: "Homunculus Mace",
        max_upgrade: Some(10),
        infuse_id: Some(101500),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11830000,
        name: "Homunculus Wooden Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 4290000,
        name: "Hunter's Blackbow",
        max_upgrade: Some(10),
        infuse_id: Some(102100),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1580000,
        name: "Ice Rapier",
        max_upgrade: Some(5),
        infuse_id: Some(100500),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 4080000,
        name: "Idol's Chime",
        max_upgrade: Some(5),
        infuse_id: Some(122500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 2030000,
        name: "Infantry Axe",
        max_upgrade: Some(10),
        infuse_id: Some(101700),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 2660000,
        name: "Iron King Hammer",
        max_upgrade: Some(5),
        infuse_id: Some(111600),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 11020000,
        name: "Iron Parma",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 5255000,
        name: "Ivory King Ultra Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111300),
        durability: Some(200),
        ..Item::default_weapon()
    },
    Item {
        id: 1390000,
        name: "Ivory Straight Sword",
        max_upgrade: Some(5),
        infuse_id: None,
        durability: Some(250),
        ..Item::default_weapon()
    },
    Item {
        id: 1980000,
        name: "Key to the Embedded",
        max_upgrade: Some(10),
        infuse_id: None,
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11550000,
        name: "King's Mirror",
        max_upgrade: Some(5),
        infuse_id: Some(112900),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 11260000,
        name: "King's Shield",
        max_upgrade: Some(5),
        infuse_id: None,
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 5240000,
        name: "King's Ultra Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111300),
        durability: Some(220),
        ..Item::default_weapon()
    },
    Item {
        id: 2600000,
        name: "Large Club",
        max_upgrade: Some(10),
        infuse_id: Some(101600),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 11200000,
        name: "Large Leather Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 4600000,
        name: "Light Crossbow",
        max_upgrade: Some(10),
        infuse_id: Some(102300),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11240000,
        name: "Lion Clan Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2240000,
        name: "Lion Greataxe",
        max_upgrade: Some(10),
        infuse_id: Some(101800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 3830000,
        name: "Lizard Staff",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 11080000,
        name: "Llewellyn Shield",
        max_upgrade: Some(5),
        infuse_id: Some(122700),
        durability: Some(110),
        ..Item::default_weapon()
    },
    Item {
        id: 4210000,
        name: "Long Bow",
        max_upgrade: Some(10),
        infuse_id: Some(102100),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1220000,
        name: "Longsword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 5295000,
        name: "Lost Sinner's Sword",
        max_upgrade: Some(5),
        infuse_id: Some(111300),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 1995000,
        name: "Loyce Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(121200),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11290000,
        name: "Loyce Shield",
        max_upgrade: Some(5),
        infuse_id: Some(122800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 3200000,
        name: "Lucerne",
        max_upgrade: Some(10),
        infuse_id: Some(102000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2410000,
        name: "Mace",
        max_upgrade: Some(10),
        infuse_id: Some(101500),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2470000,
        name: "Mace of the Insolent",
        max_upgrade: Some(10),
        infuse_id: Some(101500),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11130000,
        name: "Magic Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1410000,
        name: "Mail Breaker",
        max_upgrade: Some(10),
        infuse_id: Some(100500),
        durability: Some(45),
        ..Item::default_weapon()
    },
    Item {
        id: 1830000,
        name: "Majestic Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(121200),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1831000,
        name: "Majestic Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(121200),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 3420000,
        name: "Malformed Claws",
        max_upgrade: Some(10),
        infuse_id: Some(100200),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 2700000,
        name: "Malformed Shell",
        max_upgrade: Some(5),
        infuse_id: Some(121600),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 2670000,
        name: "Malformed Skull",
        max_upgrade: Some(10),
        infuse_id: Some(101600),
        durability: Some(20),
        ..Item::default_weapon()
    },
    Item {
        id: 3430000,
        name: "Manikin Claws",
        max_upgrade: Some(10),
        infuse_id: Some(100200),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 1110000,
        name: "Manikin Knife",
        max_upgrade: Some(10),
        infuse_id: Some(100400),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 1640000,
        name: "Manikin Sabre",
        max_upgrade: Some(10),
        infuse_id: Some(100900),
        durability: Some(45),
        ..Item::default_weapon()
    },
    Item {
        id: 11070000,
        name: "Manikin Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 1740000,
        name: "Manslayer",
        max_upgrade: Some(5),
        infuse_id: Some(121100),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 11570000,
        name: "Mastodon Greatshield",
        max_upgrade: Some(10),
        infuse_id: Some(102900),
        durability: Some(100),
        ..Item::default_weapon()
    },
    Item {
        id: 1880000,
        name: "Mastodon Greatsword",
        max_upgrade: Some(10),
        infuse_id: Some(101200),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 3270000,
        name: "Mastodon Halberd",
        max_upgrade: Some(10),
        infuse_id: Some(102000),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1680000,
        name: "Melu Scimitar",
        max_upgrade: Some(10),
        infuse_id: Some(100900),
        durability: Some(75),
        ..Item::default_weapon()
    },
    Item {
        id: 1910000,
        name: "Mirrah Greatsword",
        max_upgrade: Some(10),
        infuse_id: Some(101200),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11270000,
        name: "Mirrah Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1690000,
        name: "Monastery Scimitar",
        max_upgrade: Some(10),
        infuse_id: Some(100900),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 11380000,
        name: "Moon Butterfly Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 1871000,
        name: "Moonlight Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111200),
        durability: Some(75),
        ..Item::default_weapon()
    },
    Item {
        id: 2420000,
        name: "Morning Star",
        max_upgrade: Some(10),
        infuse_id: Some(101500),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 5000000,
        name: "Murakumo",
        max_upgrade: Some(10),
        infuse_id: Some(101400),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1040000,
        name: "Mytha's Bent Blade",
        max_upgrade: Some(5),
        infuse_id: None,
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 3610000,
        name: "Notched Whip",
        max_upgrade: Some(10),
        infuse_id: None,
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11630000,
        name: "Old Knight Greatshield",
        max_upgrade: Some(10),
        infuse_id: Some(102900),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 1950000,
        name: "Old Knight Greatsword",
        max_upgrade: Some(10),
        infuse_id: Some(101200),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 3330000,
        name: "Old Knight Halberd",
        max_upgrade: Some(10),
        infuse_id: Some(102000),
        durability: Some(20),
        ..Item::default_weapon()
    },
    Item {
        id: 2730000,
        name: "Old Knight Hammer",
        max_upgrade: Some(10),
        infuse_id: Some(101600),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 3340000,
        name: "Old Knight Pike",
        max_upgrade: Some(10),
        infuse_id: Some(102000),
        durability: Some(20),
        ..Item::default_weapon()
    },
    Item {
        id: 5285000,
        name: "Old Knight Ultra Greatsword",
        max_upgrade: Some(10),
        infuse_id: Some(101300),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 11280000,
        name: "Old Knight's Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(20),
        ..Item::default_weapon()
    },
    Item {
        id: 1911000,
        name: "Old Mirrah Greatsword",
        max_upgrade: Some(10),
        infuse_id: Some(101200),
        durability: Some(20),
        ..Item::default_weapon()
    },
    Item {
        id: 3660000,
        name: "Old Whip",
        max_upgrade: Some(5),
        infuse_id: Some(120300),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 3850000,
        name: "Olenford's Staff",
        max_upgrade: Some(5),
        infuse_id: Some(122400),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 11530000,
        name: "Orma's Greatshield",
        max_upgrade: Some(5),
        infuse_id: Some(122900),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 1100000,
        name: "Parrying Dagger",
        max_upgrade: Some(10),
        infuse_id: Some(100400),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2830000,
        name: "Partizan",
        max_upgrade: Some(10),
        infuse_id: Some(100600),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11620000,
        name: "Pate's Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102900),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2880000,
        name: "Pate's Spear",
        max_upgrade: Some(5),
        infuse_id: Some(120600),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11700000,
        name: "Phoenix Parma",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 2610000,
        name: "Pickaxe",
        max_upgrade: Some(10),
        infuse_id: Some(101500),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2820000,
        name: "Pike",
        max_upgrade: Some(10),
        infuse_id: Some(100600),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 3910000,
        name: "Pilgrim's Spontoon",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 11490000,
        name: "Porcine Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(20),
        ..Item::default_weapon()
    },
    Item {
        id: 4430000,
        name: "Possessed Armor Greatbow",
        max_upgrade: Some(10),
        infuse_id: Some(102200),
        durability: Some(100),
        ..Item::default_weapon()
    },
    Item {
        id: 1260000,
        name: "Possessed Armor Sword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 4030000,
        name: "Priest's Chime",
        max_upgrade: Some(10),
        infuse_id: Some(102500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 4100000,
        name: "Protective Chime",
        max_upgrade: Some(10),
        infuse_id: Some(102500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 11610000,
        name: "Pursuer's Greatshield",
        max_upgrade: Some(5),
        infuse_id: Some(112900),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 5270000,
        name: "Pursuer's Ultra Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111300),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1250000,
        name: "Puzzling Stone Sword",
        max_upgrade: Some(5),
        infuse_id: Some(121000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 5400000,
        name: "Pyromancy Flame",
        max_upgrade: Some(10),
        infuse_id: None,
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 2950000,
        name: "Rampart Golem Lance",
        max_upgrade: Some(10),
        infuse_id: Some(100700),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 11455000,
        name: "Rampart Golem Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(5),
        ..Item::default_weapon()
    },
    Item {
        id: 1500000,
        name: "Rapier",
        max_upgrade: Some(10),
        infuse_id: Some(100500),
        durability: Some(45),
        ..Item::default_weapon()
    },
    Item {
        id: 11740000,
        name: "Rebel's Greatshield",
        max_upgrade: Some(10),
        infuse_id: Some(102900),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 5350000,
        name: "Red Iron Twinblade",
        max_upgrade: Some(10),
        infuse_id: Some(100800),
        durability: Some(175),
        ..Item::default_weapon()
    },
    Item {
        id: 1660000,
        name: "Red Rust Scimitar",
        max_upgrade: Some(5),
        infuse_id: Some(120900),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11450000,
        name: "Red Rust Shield",
        max_upgrade: Some(5),
        infuse_id: Some(122800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1330000,
        name: "Red Rust Sword",
        max_upgrade: Some(5),
        infuse_id: Some(121000),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 11540000,
        name: "Reeve's Greatshield",
        max_upgrade: Some(5),
        infuse_id: Some(122900),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 2430000,
        name: "Reinforced Club",
        max_upgrade: Some(10),
        infuse_id: Some(101500),
        durability: Some(20),
        ..Item::default_weapon()
    },
    Item {
        id: 3940000,
        name: "Retainer Staff",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 1160000,
        name: "Retainer's Short Sword",
        max_upgrade: Some(10),
        infuse_id: Some(100400),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 1530000,
        name: "Ricard's Rapier",
        max_upgrade: Some(10),
        infuse_id: Some(100500),
        durability: Some(45),
        ..Item::default_weapon()
    },
    Item {
        id: 3320000,
        name: "Roaring Halberd",
        max_upgrade: Some(5),
        infuse_id: Some(112000),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 1130000,
        name: "Royal Dirk",
        max_upgrade: Some(5),
        infuse_id: Some(120400),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 1940000,
        name: "Royal Greatsword",
        max_upgrade: Some(10),
        infuse_id: Some(101200),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 11430000,
        name: "Royal Kite Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1900000,
        name: "Ruler's Sword",
        max_upgrade: Some(5),
        infuse_id: Some(111200),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2750000,
        name: "Sacred Chime Hammer",
        max_upgrade: Some(5),
        infuse_id: Some(111600),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 4670000,
        name: "Sanctum Crossbow",
        max_upgrade: Some(10),
        infuse_id: Some(102300),
        durability: Some(45),
        ..Item::default_weapon()
    },
    Item {
        id: 2760000,
        name: "Sanctum Mace",
        max_upgrade: Some(10),
        infuse_id: Some(101600),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 4680000,
        name: "Sanctum Repeating Crossbow",
        max_upgrade: Some(10),
        infuse_id: Some(102300),
        durability: Some(35),
        ..Item::default_weapon()
    },
    Item {
        id: 11150000,
        name: "Sanctum Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 3250000,
        name: "Santier's Spear",
        max_upgrade: Some(5),
        infuse_id: Some(122000),
        durability: Some(500),
        ..Item::default_weapon()
    },
    Item {
        id: 3251000,
        name: "Santier's Spear",
        max_upgrade: Some(5),
        infuse_id: Some(122000),
        durability: Some(500),
        ..Item::default_weapon()
    },
    Item {
        id: 1650000,
        name: "Scimitar",
        max_upgrade: Some(10),
        infuse_id: Some(100900),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 3210000,
        name: "Scythe",
        max_upgrade: Some(10),
        infuse_id: Some(102000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 3050000,
        name: "Scythe of Nahr Alma",
        max_upgrade: Some(5),
        infuse_id: Some(121900),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 3080000,
        name: "Scythe of the Forlorn",
        max_upgrade: Some(5),
        infuse_id: Some(121900),
        durability: Some(55),
        scholar_only: true,
        ..Item::default_weapon()
    },
    Item {
        id: 3070000,
        name: "Scythe of Want",
        max_upgrade: Some(5),
        infuse_id: None,
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 4230000,
        name: "Sea Bow",
        max_upgrade: Some(10),
        infuse_id: Some(102100),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1050000,
        name: "Shadow Dagger",
        max_upgrade: Some(10),
        infuse_id: Some(100400),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 4630000,
        name: "Shield Crossbow",
        max_upgrade: Some(5),
        infuse_id: Some(112300),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11330000,
        name: "Shield of the Insolent",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 4200000,
        name: "Short Bow",
        max_upgrade: Some(10),
        infuse_id: Some(102100),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1210000,
        name: "Shortsword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1610000,
        name: "Shotel",
        max_upgrade: Some(10),
        infuse_id: Some(100900),
        durability: Some(45),
        ..Item::default_weapon()
    },
    Item {
        id: 11220000,
        name: "Silver Eagle Kite Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 11350000,
        name: "Silverblack Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2100000,
        name: "Silverblack Sickle",
        max_upgrade: Some(10),
        infuse_id: Some(101700),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2860000,
        name: "Silverblack Spear",
        max_upgrade: Some(10),
        infuse_id: Some(100600),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11390000,
        name: "Slumbering Dragon Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11010000,
        name: "Small Leather Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2960000,
        name: "Smelter Hammer",
        max_upgrade: Some(10),
        infuse_id: Some(101600),
        durability: Some(300),
        ..Item::default_weapon()
    },
    Item {
        id: 5220000,
        name: "Smelter Sword",
        max_upgrade: Some(5),
        infuse_id: None,
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 3800000,
        name: "Sorcerer's Staff",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 5370000,
        name: "Sorcerer's Twinblade",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(100),
        ..Item::default_weapon()
    },
    Item {
        id: 2800000,
        name: "Spear",
        max_upgrade: Some(10),
        infuse_id: Some(100600),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 1670000,
        name: "Spider Fang",
        max_upgrade: Some(5),
        infuse_id: Some(110900),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 1430000,
        name: "Spider's Silk",
        max_upgrade: Some(5),
        infuse_id: Some(110500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 11300000,
        name: "Spirit Tree Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2850000,
        name: "Spitfire Spear",
        max_upgrade: Some(5),
        infuse_id: Some(110600),
        durability: Some(65),
        ..Item::default_weapon()
    },
    Item {
        id: 3630000,
        name: "Spotted Whip",
        max_upgrade: Some(5),
        infuse_id: Some(110300),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 3810000,
        name: "Staff of Amana",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(45),
        ..Item::default_weapon()
    },
    Item {
        id: 3890000,
        name: "Staff of Wisdom",
        max_upgrade: Some(5),
        infuse_id: Some(122400),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11360000,
        name: "Stone Parma",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 2840000,
        name: "Stone Soldier Spear",
        max_upgrade: Some(10),
        infuse_id: Some(100600),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 5310000,
        name: "Stone Twinblade",
        max_upgrade: Some(10),
        infuse_id: Some(100800),
        durability: Some(120),
        ..Item::default_weapon()
    },
    Item {
        id: 1360000,
        name: "Sun Sword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11710000,
        name: "Sunlight Parma",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 3900000,
        name: "Sunset Staff",
        max_upgrade: Some(5),
        infuse_id: Some(122400),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 3310000,
        name: "Syan's Halberd",
        max_upgrade: Some(10),
        infuse_id: Some(102000),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 11040000,
        name: "Target Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 1060000,
        name: "Thief Dagger",
        max_upgrade: Some(10),
        infuse_id: Some(100400),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 1860000,
        name: "Thorned Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111200),
        durability: Some(85),
        ..Item::default_weapon()
    },
    Item {
        id: 11510000,
        name: "Tower Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102900),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 11840000,
        name: "Transgressor's Leather Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 5540000,
        name: "Transgressor's Staff",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11500000,
        name: "Twin Dragon Greatshield",
        max_upgrade: Some(10),
        infuse_id: Some(102900),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 5340000,
        name: "Twinblade",
        max_upgrade: Some(10),
        infuse_id: Some(100800),
        durability: Some(210),
        ..Item::default_weapon()
    },
    Item {
        id: 4440000,
        name: "Twin-headed Greatbow",
        max_upgrade: Some(5),
        infuse_id: Some(122200),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1700000,
        name: "Uchigatana",
        max_upgrade: Some(10),
        infuse_id: Some(101100),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 1150000,
        name: "Umbral Dagger",
        max_upgrade: Some(5),
        infuse_id: None,
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 11160000,
        name: "Varangian Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102700),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 1270000,
        name: "Varangian Sword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 11320000,
        name: "Vessel Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1620000,
        name: "Warped Sword",
        max_upgrade: Some(5),
        infuse_id: Some(110900),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 1710000,
        name: "Washing Pole",
        max_upgrade: Some(10),
        infuse_id: Some(101100),
        durability: Some(20),
        ..Item::default_weapon()
    },
    Item {
        id: 11720000,
        name: "Watchdragon Parma",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 1970000,
        name: "Watcher Greatsword",
        max_upgrade: Some(5),
        infuse_id: Some(111200),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11185000,
        name: "Watcher's Shield",
        max_upgrade: Some(5),
        infuse_id: Some(112700),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 3600000,
        name: "Whip",
        max_upgrade: Some(10),
        infuse_id: Some(100300),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 11750000,
        name: "Wicked Eye Greatshield",
        max_upgrade: Some(10),
        infuse_id: Some(102900),
        durability: Some(70),
        ..Item::default_weapon()
    },
    Item {
        id: 2810000,
        name: "Winged Spear",
        max_upgrade: Some(10),
        infuse_id: Some(100600),
        durability: Some(50),
        ..Item::default_weapon()
    },
    Item {
        id: 4020000,
        name: "Witchtree Bellvine",
        max_upgrade: Some(10),
        infuse_id: Some(102500),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 3820000,
        name: "Witchtree Branch",
        max_upgrade: Some(10),
        infuse_id: Some(102400),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 11400000,
        name: "Wooden Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 3440000,
        name: "Work Hook",
        max_upgrade: Some(10),
        infuse_id: Some(100200),
        durability: Some(40),
        ..Item::default_weapon()
    },
    Item {
        id: 3370000,
        name: "Wrathful Axe",
        max_upgrade: Some(5),
        infuse_id: Some(112000),
        durability: Some(80),
        ..Item::default_weapon()
    },
    Item {
        id: 5510000,
        name: "Yellow Quartz Longsword",
        max_upgrade: Some(10),
        infuse_id: Some(101000),
        durability: Some(30),
        ..Item::default_weapon()
    },
    Item {
        id: 11810000,
        name: "Yellow Quartz Shield",
        max_upgrade: Some(10),
        infuse_id: Some(102800),
        durability: Some(60),
        ..Item::default_weapon()
    },
    Item {
        id: 2855000,
        name: "Yorgh's Spear",
        max_upgrade: Some(5),
        infuse_id: Some(110600),
        durability: Some(90),
        ..Item::default_weapon()
    },
    Item {
        id: 5200000,
        name: "Zweihander",
        max_upgrade: Some(10),
        infuse_id: Some(101300),
        durability: Some(60),
        ..Item::default_weapon()
    },
];