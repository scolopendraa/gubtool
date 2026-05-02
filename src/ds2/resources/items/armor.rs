use super::Item;

impl Item {
    const fn default_armor() -> Self {
        Self {
            category: super::Categories::Armor,
            stack_size: 1,
            ..Item::default()
        }
    }
}

pub static ARMOR: &[Item; 435] = &[
    Item {
        id: 25060101,
        name: "Agdayne's Black Robe",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 25060102,
        name: "Agdayne's Cuffs",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 25060103,
        name: "Agdayne's Kilt",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 23060101,
        name: "Alonne Captain Armor",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23060100,
        name: "Alonne Captain Helm",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23061101,
        name: "Alonne Knight Armor",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 23061102,
        name: "Alonne Knight Gauntlets",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 23061100,
        name: "Alonne Knight Helm",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 23061103,
        name: "Alonne Knight Leggings",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 26800101,
        name: "Alonne's Armor",
        max_upgrade: Some(5),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 26800102,
        name: "Alonne's Gauntlets",
        max_upgrade: Some(5),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 26800100,
        name: "Alonne's Helm",
        max_upgrade: Some(5),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 26800103,
        name: "Alonne's Leggings",
        max_upgrade: Some(5),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21490101,
        name: "Alva Armor",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21490102,
        name: "Alva Gauntlets",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21490100,
        name: "Alva Helm",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21490103,
        name: "Alva Leggings",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 22310103,
        name: "Archdrake Boots",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 22310102,
        name: "Archdrake Gloves",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 22310100,
        name: "Archdrake Helm",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 22310101,
        name: "Archdrake Robes",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21360101,
        name: "Armor of Aurous",
        max_upgrade: Some(5),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 21361101,
        name: "Armor of Aurous",
        max_upgrade: Some(5),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 26940101,
        name: "Armor of the Forlorn",
        max_upgrade: Some(5),
        durability: Some(80),
        scholar_only: true,
        ..Item::default_armor()
    },
    Item {
        id: 21230103,
        name: "Astrologist's Bottoms",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21230102,
        name: "Astrologist's Gauntlets",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21230101,
        name: "Astrologist's Robe",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22110101,
        name: "Bandit Armor",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 22110103,
        name: "Bandit Boots",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 22110102,
        name: "Bandit Gauntlets",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 27530101,
        name: "Bell Keeper Bellyband",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 27530102,
        name: "Bell Keeper Cuffs",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 27530100,
        name: "Bell Keeper Helmet",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 27530103,
        name: "Bell Keeper Trousers",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 27430101,
        name: "Benhart's Armor",
        max_upgrade: Some(10),
        durability: Some(72),
        ..Item::default_armor()
    },
    Item {
        id: 27430103,
        name: "Benhart's Boots",
        max_upgrade: Some(10),
        durability: Some(72),
        ..Item::default_armor()
    },
    Item {
        id: 27430102,
        name: "Benhart's Gauntlets",
        max_upgrade: Some(10),
        durability: Some(72),
        ..Item::default_armor()
    },
    Item {
        id: 27430100,
        name: "Benhart's Knight Helm",
        max_upgrade: Some(10),
        durability: Some(72),
        ..Item::default_armor()
    },
    Item {
        id: 27680103,
        name: "Black Boots",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21330101,
        name: "Black Dragon Armor",
        max_upgrade: Some(5),
        durability: Some(225),
        ..Item::default_armor()
    },
    Item {
        id: 21330102,
        name: "Black Dragon Gauntlets",
        max_upgrade: Some(5),
        durability: Some(225),
        ..Item::default_armor()
    },
    Item {
        id: 21330100,
        name: "Black Dragon Helm",
        max_upgrade: Some(5),
        durability: Some(225),
        ..Item::default_armor()
    },
    Item {
        id: 21330103,
        name: "Black Dragon Leggings",
        max_upgrade: Some(5),
        durability: Some(225),
        ..Item::default_armor()
    },
    Item {
        id: 27680102,
        name: "Black Gloves",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22180100,
        name: "Black Hollow Mage Hood",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22180101,
        name: "Black Hollow Mage Robe",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 27680100,
        name: "Black Hood",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21020101,
        name: "Black Leather Armor",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21020103,
        name: "Black Leather Boots",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21020102,
        name: "Black Leather Gloves",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 27680101,
        name: "Black Robes",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21502100,
        name: "Black Witch Domino Mask",
        max_upgrade: Some(5),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21500102,
        name: "Black Witch Gloves",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21501100,
        name: "Black Witch Hat",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21500101,
        name: "Black Witch Robe",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21500103,
        name: "Black Witch Trousers",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21500100,
        name: "Black Witch Veil",
        max_upgrade: Some(5),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 21390103,
        name: "Blood-Stained Skirt",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22540100,
        name: "Bone Crown",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 22540102,
        name: "Bone King Cuffs",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 22540101,
        name: "Bone King Robe",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 22540103,
        name: "Bone King Skirt",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 21080101,
        name: "Brigand Armor",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21080102,
        name: "Brigand Gauntlets",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21080100,
        name: "Brigand Hood",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21080103,
        name: "Brigand Trousers",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 27510100,
        name: "Cale's Helm",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 27510101,
        name: "Cale's Leather Armor",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 27510103,
        name: "Cale's Shoes",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21480101,
        name: "Catarina Armor",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 21480102,
        name: "Catarina Gauntlets",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 21480100,
        name: "Catarina Helm",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 21480103,
        name: "Catarina Leggings",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 27710103,
        name: "Chaos Boots",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 27710102,
        name: "Chaos Gloves",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 27710100,
        name: "Chaos Hood",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 27710101,
        name: "Chaos Robe",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26890101,
        name: "Charred Loyce Armor",
        max_upgrade: Some(5),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 26890102,
        name: "Charred Loyce Gauntlets",
        max_upgrade: Some(5),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 26890100,
        name: "Charred Loyce Helm",
        max_upgrade: Some(5),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 26890103,
        name: "Charred Loyce Leggings",
        max_upgrade: Some(5),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 27420102,
        name: "Creighton's Chain Gloves",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 27420103,
        name: "Creighton's Chain Leggings",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 27420101,
        name: "Creighton's Chainmail",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 27420100,
        name: "Creighton's Steel Mask",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 21640100,
        name: "Crown of the Ivory King",
        max_upgrade: Some(5),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 21630100,
        name: "Crown of the Old Iron King",
        max_upgrade: Some(5),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 21650100,
        name: "Crown of the Sunken King",
        max_upgrade: Some(5),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 22360101,
        name: "Dark Armor",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 22360102,
        name: "Dark Gauntlets",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 22360103,
        name: "Dark Leggings",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 22360100,
        name: "Dark Mask",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23160102,
        name: "Desert Sorceress Gloves",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 23160100,
        name: "Desert Sorceress Hood",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 23160103,
        name: "Desert Sorceress Skirt",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 23160101,
        name: "Desert Sorceress Top",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21390102,
        name: "Dingy Cuffs",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 21390100,
        name: "Dingy Hood",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 21390101,
        name: "Dingy Robe",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 23170103,
        name: "Dragon Acolyte Boots",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 23170102,
        name: "Dragon Acolyte Gloves",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 23170100,
        name: "Dragon Acolyte Mask",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 23170101,
        name: "Dragon Acolyte Robe",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 23171100,
        name: "Dragon Sage Hood",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26100101,
        name: "Dragonrider Armor",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 26100102,
        name: "Dragonrider Gauntlets",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 26100100,
        name: "Dragonrider Helm",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 26100103,
        name: "Dragonrider Leggings",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 21600101,
        name: "Drakeblood Armor",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21600102,
        name: "Drakeblood Gauntlets",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21600100,
        name: "Drakeblood Helm",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21600103,
        name: "Drakeblood Leggings",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 23310101,
        name: "Drakekeeper Armor",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23310103,
        name: "Drakekeeper Boots",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23310102,
        name: "Drakekeeper Gauntlets",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23310100,
        name: "Drakekeeper Helm",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 27240102,
        name: "Drangleic Gauntlets",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 27240100,
        name: "Drangleic Helm",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 27240103,
        name: "Drangleic Leggings",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 27240101,
        name: "Drangleic Mail",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 21430100,
        name: "Durgo's Hat",
        max_upgrade: Some(5),
        durability: Some(48),
        ..Item::default_armor()
    },
    Item {
        id: 21060101,
        name: "Elite Knight Armor",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21060102,
        name: "Elite Knight Gloves",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21060100,
        name: "Elite Knight Helm",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21060103,
        name: "Elite Knight Leggings",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21440102,
        name: "Engraved Gauntlets",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26180101,
        name: "Executioner Armor",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 26180102,
        name: "Executioner Gauntlets",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 26180100,
        name: "Executioner Helm",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 26180103,
        name: "Executioner Leggings",
        max_upgrade: Some(5),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 23130101,
        name: "Falconer Armor",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 23130103,
        name: "Falconer Boots",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 23130102,
        name: "Falconer Gloves",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 23130100,
        name: "Falconer Helm",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 21320101,
        name: "Faraam Armor",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21320103,
        name: "Faraam Boots",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21320102,
        name: "Faraam Gauntlets",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21320100,
        name: "Faraam Helm",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21690103,
        name: "Flower Skirt",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21460103,
        name: "Flying Feline Boots",
        max_upgrade: Some(5),
        durability: Some(66),
        ..Item::default_armor()
    },
    Item {
        id: 26510103,
        name: "Fume Sorcerer Boots",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 26510102,
        name: "Fume Sorcerer Gloves",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 26510100,
        name: "Fume Sorcerer Mask",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 26510101,
        name: "Fume Sorcerer Robes",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21360102,
        name: "Gauntlets of Aurous",
        max_upgrade: Some(5),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 21361102,
        name: "Gauntlets of Aurous",
        max_upgrade: Some(5),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 26940102,
        name: "Gauntlets of the Forlorn",
        max_upgrade: Some(5),
        durability: Some(75),
        scholar_only: true,
        ..Item::default_armor()
    },
    Item {
        id: 23120103,
        name: "Grave Warden Bottoms",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 23120102,
        name: "Grave Warden Cuffs",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 23120100,
        name: "Grave Warden Mask",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 23120101,
        name: "Grave Warden Top",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 22340101,
        name: "Gyrm Armor",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 22340103,
        name: "Gyrm Boots",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 22340102,
        name: "Gyrm Gloves",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 22340100,
        name: "Gyrm Helm",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 22350101,
        name: "Gyrm Warrior Armor",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 22350103,
        name: "Gyrm Warrior Boots",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 22350102,
        name: "Gyrm Warrior Gloves",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 22351100,
        name: "Gyrm Warrior Greathelm",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 22350100,
        name: "Gyrm Warrior Helm",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 27440101,
        name: "Hard Leather Armor",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 27440103,
        name: "Hard Leather Boots",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 27440102,
        name: "Hard Leather Gauntlets",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 21160101,
        name: "Havel's Armor",
        max_upgrade: Some(5),
        durability: Some(255),
        ..Item::default_armor()
    },
    Item {
        id: 21160102,
        name: "Havel's Gauntlets",
        max_upgrade: Some(5),
        durability: Some(255),
        ..Item::default_armor()
    },
    Item {
        id: 21160100,
        name: "Havel's Helm",
        max_upgrade: Some(5),
        durability: Some(255),
        ..Item::default_armor()
    },
    Item {
        id: 21160103,
        name: "Havel's Leggings",
        max_upgrade: Some(5),
        durability: Some(255),
        ..Item::default_armor()
    },
    Item {
        id: 21070103,
        name: "Heavy Boots",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 23010101,
        name: "Heide Knight Chainmail",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23010102,
        name: "Heide Knight Gauntlets",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23010100,
        name: "Heide Knight Greathelm",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23011100,
        name: "Heide Knight Iron Mask",
        max_upgrade: Some(10),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 23010103,
        name: "Heide Knight Leggings",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 21360100,
        name: "Helm of Aurous",
        max_upgrade: Some(5),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 21361100,
        name: "Helm of Aurous",
        max_upgrade: Some(5),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 27700103,
        name: "Hexer's Boots",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 27700102,
        name: "Hexer's Gloves",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 27700100,
        name: "Hexer's Hood",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 27700101,
        name: "Hexer's Robes",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22030101,
        name: "Hollow Infantry Armor",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22030103,
        name: "Hollow Infantry Boots",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22030102,
        name: "Hollow Infantry Gloves",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22030100,
        name: "Hollow Infantry Helm",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 21670100,
        name: "Hollow Skin",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22020101,
        name: "Hollow Soldier Armor",
        max_upgrade: Some(10),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 22020102,
        name: "Hollow Soldier Gauntlets",
        max_upgrade: Some(10),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 22020100,
        name: "Hollow Soldier Helm",
        max_upgrade: Some(10),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 22020103,
        name: "Hollow Soldier Leggings",
        max_upgrade: Some(10),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 26940100,
        name: "Hood of the Forlorn",
        max_upgrade: Some(5),
        durability: Some(40),
        scholar_only: true,
        ..Item::default_armor()
    },
    Item {
        id: 21040100,
        name: "Hunter's Hat",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 25110101,
        name: "Imperious Armor",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 25110102,
        name: "Imperious Gloves",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 25110100,
        name: "Imperious Helm",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 25110103,
        name: "Imperious Leggings",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21100100,
        name: "Imported Hood",
        max_upgrade: Some(10),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 21100102,
        name: "Imported Manchettes",
        max_upgrade: Some(10),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 21100103,
        name: "Imported Trousers",
        max_upgrade: Some(10),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 21100101,
        name: "Imported Tunic",
        max_upgrade: Some(10),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 22031101,
        name: "Infantry Armor",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22031103,
        name: "Infantry Boots",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22031102,
        name: "Infantry Gloves",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22031100,
        name: "Infantry Helm",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 25100101,
        name: "Insolent Armor",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 25100103,
        name: "Insolent Boots",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 25100102,
        name: "Insolent Gloves",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 25100100,
        name: "Insolent Helm",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 22510101,
        name: "Ironclad Armor",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 22510102,
        name: "Ironclad Gauntlets",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 22510100,
        name: "Ironclad Helm",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 22510103,
        name: "Ironclad Leggings",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 26900101,
        name: "Ivory King Armor",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26900102,
        name: "Ivory King Gauntlets",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26900100,
        name: "Ivory King Helm",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26900103,
        name: "Ivory King Leggings",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21210100,
        name: "Jester's Cap",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 21210102,
        name: "Jester's Gloves",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 21210101,
        name: "Jester's Robes",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 21210103,
        name: "Jester's Tights",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 25130101,
        name: "King's Armor",
        max_upgrade: Some(5),
        durability: Some(120),
        ..Item::default_armor()
    },
    Item {
        id: 25130100,
        name: "King's Crown",
        max_upgrade: Some(5),
        durability: Some(230),
        ..Item::default_armor()
    },
    Item {
        id: 25130102,
        name: "King's Gauntlets",
        max_upgrade: Some(5),
        durability: Some(120),
        ..Item::default_armor()
    },
    Item {
        id: 25130103,
        name: "King's Leggings",
        max_upgrade: Some(5),
        durability: Some(120),
        ..Item::default_armor()
    },
    Item {
        id: 21050101,
        name: "Knight Armor",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21050102,
        name: "Knight Gauntlets",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21050100,
        name: "Knight Helm",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21050103,
        name: "Knight Leggings",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21040101,
        name: "Leather Armor",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21040103,
        name: "Leather Boots",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21040102,
        name: "Leather Gloves",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21360103,
        name: "Leggings of Aurous",
        max_upgrade: Some(5),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 21361103,
        name: "Leggings of Aurous",
        max_upgrade: Some(5),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 26940103,
        name: "Leggings of the Forlorn",
        max_upgrade: Some(5),
        durability: Some(75),
        scholar_only: true,
        ..Item::default_armor()
    },
    Item {
        id: 25090100,
        name: "Leydia Black Hood",
        max_upgrade: Some(5),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 25090101,
        name: "Leydia Black Robe",
        max_upgrade: Some(5),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 25120102,
        name: "Leydia Gauntlets",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 25120100,
        name: "Leydia White Hood",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 25120101,
        name: "Leydia White Robe",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22190102,
        name: "Lion Mage Cuffs",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22190101,
        name: "Lion Mage Robe",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22190103,
        name: "Lion Mage Skirt",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 23080101,
        name: "Lion Warrior Cape",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 23080102,
        name: "Lion Warrior Cuffs",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 23081100,
        name: "Lion Warrior Helm",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 23080103,
        name: "Lion Warrior Skirt",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 27210101,
        name: "Llewellyn Armor",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 27210102,
        name: "Llewellyn Gloves",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 27210103,
        name: "Llewellyn Shoes",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 25040101,
        name: "Looking Glass Armor",
        max_upgrade: Some(5),
        durability: Some(150),
        ..Item::default_armor()
    },
    Item {
        id: 25040102,
        name: "Looking Glass Gauntlets",
        max_upgrade: Some(5),
        durability: Some(150),
        ..Item::default_armor()
    },
    Item {
        id: 25040103,
        name: "Looking Glass Leggings",
        max_upgrade: Some(5),
        durability: Some(150),
        ..Item::default_armor()
    },
    Item {
        id: 25040100,
        name: "Looking Glass Mask",
        max_upgrade: Some(5),
        durability: Some(150),
        ..Item::default_armor()
    },
    Item {
        id: 26880101,
        name: "Loyce Armor",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 26880102,
        name: "Loyce Gauntlets",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 26880100,
        name: "Loyce Helm",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 26880103,
        name: "Loyce Leggings",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 27520102,
        name: "Lucatiel's Gloves",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 27520100,
        name: "Lucatiel's Mask",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 27520103,
        name: "Lucatiel's Trousers",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 27520101,
        name: "Lucatiel's Vest",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 27550101,
        name: "Mad Warrior Armor",
        max_upgrade: Some(5),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 27550102,
        name: "Mad Warrior Gauntlets",
        max_upgrade: Some(5),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 27550103,
        name: "Mad Warrior Leggings",
        max_upgrade: Some(5),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 27550100,
        name: "Mad Warrior Mask",
        max_upgrade: Some(5),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 21350102,
        name: "Manchettes of Judgment",
        max_upgrade: Some(5),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22240103,
        name: "Manikin Boots",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 22240102,
        name: "Manikin Gloves",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 22240100,
        name: "Manikin Mask",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 22240101,
        name: "Manikin Top",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 21350100,
        name: "Mask of Judgment",
        max_upgrade: Some(5),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 23150101,
        name: "Mastodon Armor",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23150102,
        name: "Mastodon Gauntlets",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23150100,
        name: "Mastodon Helm",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 23150103,
        name: "Mastodon Leggings",
        max_upgrade: Some(10),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 21700100,
        name: "Minotaur Helm",
        max_upgrade: Some(10),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 27521100,
        name: "Mirrah Hat",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21370100,
        name: "Monastery Headcloth",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21370102,
        name: "Monastery Long Gloves",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21370101,
        name: "Monastery Longshirt",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21370103,
        name: "Monastery Skirt",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21470102,
        name: "Moon Butterfly Cuffs",
        max_upgrade: Some(5),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 21470100,
        name: "Moon Butterfly Hat",
        max_upgrade: Some(5),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 21470103,
        name: "Moon Butterfly Skirt",
        max_upgrade: Some(5),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 21470101,
        name: "Moon Butterfly Wings",
        max_upgrade: Some(5),
        durability: Some(35),
        ..Item::default_armor()
    },
    Item {
        id: 21230100,
        name: "Moon Hat",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 27830100,
        name: "Nahr Alma Hood",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 27830101,
        name: "Nahr Alma Robes",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21610100,
        name: "Northwarder Hood",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21610102,
        name: "Northwarder Manchettes",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21610101,
        name: "Northwarder Robe",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21610103,
        name: "Northwarder Trousers",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21660100,
        name: "Old Bell Helm",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 22512101,
        name: "Old Ironclad Armor",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 22512102,
        name: "Old Ironclad Gauntlets",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 22512100,
        name: "Old Ironclad Helm",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 22512103,
        name: "Old Ironclad Leggings",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 23300101,
        name: "Old Knight Armor",
        max_upgrade: Some(10),
        durability: Some(20),
        ..Item::default_armor()
    },
    Item {
        id: 23300102,
        name: "Old Knight Gauntlets",
        max_upgrade: Some(10),
        durability: Some(20),
        ..Item::default_armor()
    },
    Item {
        id: 23300100,
        name: "Old Knight Helm",
        max_upgrade: Some(10),
        durability: Some(20),
        ..Item::default_armor()
    },
    Item {
        id: 23300103,
        name: "Old Knight Leggings",
        max_upgrade: Some(10),
        durability: Some(20),
        ..Item::default_armor()
    },
    Item {
        id: 21010101,
        name: "Pate's Armor",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 21010102,
        name: "Pate's Gloves",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 21010100,
        name: "Pate's Helm",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 21010103,
        name: "Pate's Trousers",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 22480101,
        name: "Peasant Attire",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 22480100,
        name: "Peasant Hat",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 22480102,
        name: "Peasant Long Gloves",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 22480103,
        name: "Peasant Trousers",
        max_upgrade: Some(10),
        durability: Some(45),
        ..Item::default_armor()
    },
    Item {
        id: 26260102,
        name: "Penal Handcuffs",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26260100,
        name: "Penal Mask",
        max_upgrade: Some(5),
        durability: Some(120),
        ..Item::default_armor()
    },
    Item {
        id: 26260103,
        name: "Penal Skirt",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26260101,
        name: "Penal Straightjacket",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21680100,
        name: "Pharros Mask",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22062102,
        name: "Priestess Gloves",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22062100,
        name: "Priestess Headpiece",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22062101,
        name: "Priestess Robe",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22062103,
        name: "Priestess Skirt",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22270102,
        name: "Prisoner's Gloves",
        max_upgrade: Some(10),
        durability: Some(25),
        ..Item::default_armor()
    },
    Item {
        id: 22270100,
        name: "Prisoner's Hood",
        max_upgrade: Some(10),
        durability: Some(25),
        ..Item::default_armor()
    },
    Item {
        id: 22271100,
        name: "Prisoner's Hood",
        max_upgrade: Some(10),
        durability: Some(25),
        ..Item::default_armor()
    },
    Item {
        id: 22270101,
        name: "Prisoner's Tatters",
        max_upgrade: Some(10),
        durability: Some(25),
        ..Item::default_armor()
    },
    Item {
        id: 22271101,
        name: "Prisoner's Tatters",
        max_upgrade: Some(10),
        durability: Some(25),
        ..Item::default_armor()
    },
    Item {
        id: 22270103,
        name: "Prisoner's Waistcloth",
        max_upgrade: Some(10),
        durability: Some(25),
        ..Item::default_armor()
    },
    Item {
        id: 26750101,
        name: "Raime's Armor",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 26750102,
        name: "Raime's Gauntlets",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 26750100,
        name: "Raime's Helm",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 26750103,
        name: "Raime's Leggings",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 26590101,
        name: "Rampart Golem Armor",
        max_upgrade: Some(5),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 26590102,
        name: "Rampart Golem Gauntlets",
        max_upgrade: Some(5),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 26590100,
        name: "Rampart Golem Helm",
        max_upgrade: Some(5),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 26590103,
        name: "Rampart Golem Leggings",
        max_upgrade: Some(5),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 23081101,
        name: "Red Lion Warrior Cape",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26770101,
        name: "Retainer Robe",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21350101,
        name: "Robe of Judgment",
        max_upgrade: Some(5),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22080101,
        name: "Rogue Armor",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22080102,
        name: "Rogue Gauntlets",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22080100,
        name: "Rogue Hood",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22080103,
        name: "Rogue Leggings",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22021101,
        name: "Royal Soldier Armor",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 22021102,
        name: "Royal Soldier Gauntlets",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 22021100,
        name: "Royal Soldier Helm",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 22021103,
        name: "Royal Soldier Leggings",
        max_upgrade: Some(10),
        durability: Some(70),
        ..Item::default_armor()
    },
    Item {
        id: 22520101,
        name: "Royal Swordsman Armor",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 22520102,
        name: "Royal Swordsman Gloves",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 22520100,
        name: "Royal Swordsman Helm",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 22520103,
        name: "Royal Swordsman Leggings",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 23250101,
        name: "Ruin Armor",
        max_upgrade: Some(5),
        durability: Some(105),
        ..Item::default_armor()
    },
    Item {
        id: 23250102,
        name: "Ruin Gauntlets",
        max_upgrade: Some(5),
        durability: Some(105),
        ..Item::default_armor()
    },
    Item {
        id: 23250100,
        name: "Ruin Helm",
        max_upgrade: Some(5),
        durability: Some(105),
        ..Item::default_armor()
    },
    Item {
        id: 23250103,
        name: "Ruin Leggings",
        max_upgrade: Some(5),
        durability: Some(105),
        ..Item::default_armor()
    },
    Item {
        id: 23140101,
        name: "Rusted Mastodon Armor",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 23140102,
        name: "Rusted Mastodon Gauntlets",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 23140100,
        name: "Rusted Mastodon Helm",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 23140103,
        name: "Rusted Mastodon Leggings",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 27690101,
        name: "Saint's Dress",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 27690100,
        name: "Saint's Hood",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 27690102,
        name: "Saint's Long Gloves",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 27690103,
        name: "Saint's Trousers",
        max_upgrade: Some(5),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26650101,
        name: "Sanctum Knight Armor",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 26650102,
        name: "Sanctum Knight Gauntlets",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 26650100,
        name: "Sanctum Knight Helm",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 26650103,
        name: "Sanctum Knight Leggings",
        max_upgrade: Some(5),
        durability: Some(90),
        ..Item::default_armor()
    },
    Item {
        id: 26700100,
        name: "Sanctum Priestess Tiara",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 26660102,
        name: "Sanctum Soldier Gauntlet",
        max_upgrade: Some(10),
        durability: Some(80),
        ..Item::default_armor()
    },
    Item {
        id: 22230102,
        name: "Shadow Gauntlets",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 22230103,
        name: "Shadow Leggings",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 22230100,
        name: "Shadow Mask",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 22230101,
        name: "Shadow Top",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 23040101,
        name: "Singer's Dress",
        max_upgrade: Some(5),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 23050101,
        name: "Smelter Demon Armor",
        max_upgrade: Some(5),
        durability: Some(160),
        ..Item::default_armor()
    },
    Item {
        id: 23050102,
        name: "Smelter Demon Gauntlets",
        max_upgrade: Some(5),
        durability: Some(160),
        ..Item::default_armor()
    },
    Item {
        id: 23050100,
        name: "Smelter Demon Helm",
        max_upgrade: Some(5),
        durability: Some(160),
        ..Item::default_armor()
    },
    Item {
        id: 23050103,
        name: "Smelter Demon Leggings",
        max_upgrade: Some(5),
        durability: Some(160),
        ..Item::default_armor()
    },
    Item {
        id: 22110100,
        name: "Spiked Bandit Helm",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 27440100,
        name: "Standard Helm",
        max_upgrade: Some(10),
        durability: Some(65),
        ..Item::default_armor()
    },
    Item {
        id: 22220101,
        name: "Steel Armor",
        max_upgrade: Some(10),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 22220102,
        name: "Steel Gauntlets",
        max_upgrade: Some(10),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 22220100,
        name: "Steel Helm",
        max_upgrade: Some(10),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 22220103,
        name: "Steel Leggings",
        max_upgrade: Some(10),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 22530101,
        name: "Syan's Armor",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 22530102,
        name: "Syan's Gauntlets",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 22530100,
        name: "Syan's Helm",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 22530103,
        name: "Syan's Leggings",
        max_upgrade: Some(10),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21710100,
        name: "Symbol of Avarice",
        max_upgrade: Some(10),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 27950101,
        name: "Targray's Armor",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 27950100,
        name: "Targray's Helm",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 27950103,
        name: "Targray's Leggings",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 27950102,
        name: "Targray's Manifers",
        max_upgrade: Some(10),
        durability: Some(75),
        ..Item::default_armor()
    },
    Item {
        id: 21070100,
        name: "Tattered Cloth Hood",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21070102,
        name: "Tattered Cloth Manchettes",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21070101,
        name: "Tattered Cloth Robe",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 21020100,
        name: "Thief Mask",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 23320101,
        name: "Throne Defender Armor",
        max_upgrade: Some(5),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 23320102,
        name: "Throne Defender Gauntlets",
        max_upgrade: Some(5),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 23320100,
        name: "Throne Defender Helm",
        max_upgrade: Some(5),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 23320103,
        name: "Throne Defender Leggings",
        max_upgrade: Some(5),
        durability: Some(100),
        ..Item::default_armor()
    },
    Item {
        id: 23340101,
        name: "Throne Watcher Armor",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 23340102,
        name: "Throne Watcher Gauntlets",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 23340100,
        name: "Throne Watcher Helm",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 23340103,
        name: "Throne Watcher Leggings",
        max_upgrade: Some(5),
        durability: Some(95),
        ..Item::default_armor()
    },
    Item {
        id: 21350103,
        name: "Tights of Judgment",
        max_upgrade: Some(5),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21140103,
        name: "Traveling Merchant Boots",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21140101,
        name: "Traveling Merchant Coat",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21140102,
        name: "Traveling Merchant Gloves",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21140100,
        name: "Traveling Merchant Hat",
        max_upgrade: Some(10),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 22460100,
        name: "Tseldora Cap",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22460102,
        name: "Tseldora Manchettes",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22460101,
        name: "Tseldora Robe",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22460103,
        name: "Tseldora Trousers",
        max_upgrade: Some(10),
        durability: Some(40),
        ..Item::default_armor()
    },
    Item {
        id: 22130101,
        name: "Varangian Armor",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22130102,
        name: "Varangian Cuffs",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22130100,
        name: "Varangian Helm",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 22130103,
        name: "Varangian Leggings",
        max_upgrade: Some(10),
        durability: Some(55),
        ..Item::default_armor()
    },
    Item {
        id: 23330101,
        name: "Velstadt's Armor",
        max_upgrade: Some(5),
        durability: Some(140),
        ..Item::default_armor()
    },
    Item {
        id: 23330102,
        name: "Velstadt's Gauntlets",
        max_upgrade: Some(5),
        durability: Some(140),
        ..Item::default_armor()
    },
    Item {
        id: 23330100,
        name: "Velstadt's Helm",
        max_upgrade: Some(5),
        durability: Some(140),
        ..Item::default_armor()
    },
    Item {
        id: 23330103,
        name: "Velstadt's Leggings",
        max_upgrade: Some(5),
        durability: Some(140),
        ..Item::default_armor()
    },
    Item {
        id: 23070101,
        name: "Vengarl's Armor",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 23070103,
        name: "Vengarl's Boots",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 23070102,
        name: "Vengarl's Gloves",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 23070100,
        name: "Vengarl's Helm",
        max_upgrade: Some(5),
        durability: Some(85),
        ..Item::default_armor()
    },
    Item {
        id: 21030103,
        name: "Wanderer Boots",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21030101,
        name: "Wanderer Coat",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21030100,
        name: "Wanderer Hood",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21030102,
        name: "Wanderer Manchettes",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22370100,
        name: "Warlock Mask",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22182100,
        name: "White Hollow Mage Hood",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22182101,
        name: "White Hollow Mage Robe",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22060102,
        name: "White Priest Gloves",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22060100,
        name: "White Priest Headpiece",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22060101,
        name: "White Priest Robe",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 22060103,
        name: "White Priest Skirt",
        max_upgrade: Some(10),
        durability: Some(50),
        ..Item::default_armor()
    },
    Item {
        id: 21340100,
        name: "Xanthous Crown",
        max_upgrade: Some(5),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21340102,
        name: "Xanthous Gloves",
        max_upgrade: Some(5),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21340101,
        name: "Xanthous Overcoat",
        max_upgrade: Some(5),
        durability: Some(60),
        ..Item::default_armor()
    },
    Item {
        id: 21340103,
        name: "Xanthous Waistcloth",
        max_upgrade: Some(5),
        durability: Some(60),
        ..Item::default_armor()
    },
];