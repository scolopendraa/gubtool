use super::Item;

impl Item {
    const fn default_key_item() -> Self {
        Self {
            category: super::Categories::KeyItems,
            ..Item::default()
        }
    }
}

pub static KEY_ITEMS: &[Item; 67] = &[
    Item {
        id: 60355000,
        name: "Aged Feather",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 51030000,
        name: "Aldia Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50840000,
        name: "Antiquated Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50910000,
        name: "Ashen Mist Heart",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62150000,
        name: "Awestone",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 50800000,
        name: "Bastille Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 6100000,
        name: "Binoculars",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62160000,
        name: "Black Separation Crystal",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62020000,
        name: "Bone of Order",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50830000,
        name: "Brightstone Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50940000,
        name: "Champion's Tablet",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62050000,
        name: "Cracked Blue Eye Orb",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 62060000,
        name: "Cracked Red Eye Orb",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 51000000,
        name: "Crushed Eye Orb",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62070000,
        name: "Dragon Eye",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 60405000,
        name: "Dragon Head Stone",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 60405010,
        name: "Dragon Head Stone",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62130000,
        name: "Dragon Scale",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 52650000,
        name: "Dragon Stone",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 52000000,
        name: "Dragon Talon",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 60406000,
        name: "Dragon Torso Stone",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 60406010,
        name: "Dragon Torso Stone",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62000000,
        name: "Dried Fingers",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50990000,
        name: "Dull Ember",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 60155000,
        name: "Estus Flask",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 60525000,
        name: "Estus Flask Shard",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 52300000,
        name: "Eternal Sanctum Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 53600000,
        name: "Eye of the Priestess",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50850000,
        name: "Fang Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50820000,
        name: "Forgotten Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 52200000,
        name: "Frozen Flower",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 52500000,
        name: "Garrison Ward Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50900000,
        name: "Giant's Kinship",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 52100000,
        name: "Heavy Iron Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 60470000,
        name: "Hello Carving",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50860000,
        name: "House Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 60490000,
        name: "I'm Sorry Carving",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50810000,
        name: "Iron Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50610000,
        name: "Key to King's Passage",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50950000,
        name: "Ladder Miniature",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50870000,
        name: "Lenigrast's Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62190000,
        name: "Petrified Egg",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62140000,
        name: "Rat Tail",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 62045000,
        name: "Red Sign Soapstone",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50890000,
        name: "Rotunda Lockstone",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 53100000,
        name: "Scorching Iron Scepter",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62170000,
        name: "Seed of a Tree of Giants",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 51010000,
        name: "Simpleton's Spice",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 51020000,
        name: "Skeptic's Spice",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 50885000,
        name: "Small Smooth &amp; Silky Stone",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 62040000,
        name: "Small White Sign Soapstone",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 53200000,
        name: "Smelter Wedge",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 50880000,
        name: "Smooth &amp; Silky Stone",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 50600000,
        name: "Soldier Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50920000,
        name: "Soul of a Giant",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 53300000,
        name: "Soul of Nadalia, Bride of Ash",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 50960000,
        name: "Soul Vessel",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 60526000,
        name: "Sublime Bone Dust",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 62120000,
        name: "Sunlight Medal",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 60480000,
        name: "Thank You Carving",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62100000,
        name: "Token of Fidelity",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 62110000,
        name: "Token of Spite",
        stack_size: 99,
        ..Item::default_key_item()
    },
    Item {
        id: 52400000,
        name: "Tower Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50930000,
        name: "Tseldora Den Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 50970000,
        name: "Undead Lockaway Key",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 60500000,
        name: "Very Good! Carving",
        stack_size: 1,
        ..Item::default_key_item()
    },
    Item {
        id: 62030000,
        name: "White Sign Soapstone",
        stack_size: 1,
        ..Item::default_key_item()
    },
];