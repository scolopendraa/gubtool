use super::Item;

impl Item {
    const fn default_spirit_ashes() -> Self {
        Self {
            category: super::Categories::SpiritAshes,
            stack_size: 1,
            max_storage: 600,
            ..Item::default()
        }
    }
}

pub static SPIRIT_ASHES: [Item; 84] = [
    Item {
        id: 0x40033838,
        name: "Albinauric Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40033068,
        name: "Ancestral Follower Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021cc58,
        name: "Ancient Dragon Florissax",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003e800,
        name: "Ancient Dragon Knight Kristoff",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003cca8,
        name: "Archer Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400372d0,
        name: "Avionette Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40038270,
        name: "Azula Beastman Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40031510,
        name: "Banished Knight Engvall",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40031128,
        name: "Banished Knight Oleg",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40035f48,
        name: "Battlemage Hugues",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021b8d0,
        name: "Bigmouth Imp Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40030d40,
        name: "Black Knife Tiche",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021b4e8,
        name: "Black Knight Captain Huw",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021b100,
        name: "Black Knight Commander Andreas",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40037aa0,
        name: "Blackflame Monk Amon",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x402195a8,
        name: "Bloodfiend Hexer's Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003a598,
        name: "Bloodhound Knight Floh",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40036330,
        name: "Clayman Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40036718,
        name: "Cleanrot Knight Finlay",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40032c80,
        name: "Crystalian Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x402191c0,
        name: "Curseblade Meera",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40039210,
        name: "Demi-Human Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021a930,
        name: "Demi-Human Swordsman Yosh",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40034bc0,
        name: "Depraved Perfumer Carmaan",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021c488,
        name: "Divine Bird Warrior Ornis",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003ff70,
        name: "Dolores the Sleeping Arrow Puppet",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003f7a0,
        name: "Dung Eater Puppet",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400318f8,
        name: "Fanged Imp Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003fb88,
        name: "Finger Maiden Therolina Puppet",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021d428,
        name: "Fingercreeper Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40219d78,
        name: "Fire Knight Hilde",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021d810,
        name: "Fire Knight Queelign",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400376b8,
        name: "Fire Monk Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40038e28,
        name: "Giant Rat Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40035390,
        name: "Glintstone Sorcerer Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003d090,
        name: "Godrick Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40219990,
        name: "Gravebird Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003c8c0,
        name: "Greatshield Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003e418,
        name: "Haligtree Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021c870,
        name: "Horned Warrior Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021a548,
        name: "Inquisitor Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40040358,
        name: "Jarwight Puppet",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021dfe0,
        name: "Jolán and Anna",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40038658,
        name: "Kaiden Sellsword Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40036b00,
        name: "Kindred of Rot Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003b920,
        name: "Land Squirt Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40031ce0,
        name: "Latenna the Albinauric",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003d860,
        name: "Leyndell Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003efd0,
        name: "Lhutel the Headless",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40038a40,
        name: "Lone Wolf Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003b538,
        name: "Mad Pumpkin Head Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021bcb8,
        name: "Man-Fly Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40037e88,
        name: "Man-Serpent Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40036ee8,
        name: "Marionette Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003e030,
        name: "Mausoleum Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021ad18,
        name: "Messmer Soldier Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40032898,
        name: "Mimic Tear Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003bd08,
        name: "Miranda Sprout Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003f3b8,
        name: "Nepheli Loux Puppet",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400324b0,
        name: "Nightmaiden & Swordstress Puppets",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003ad68,
        name: "Noble Sorcerer Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400320c8,
        name: "Nomad Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003c4d8,
        name: "Omenkiller Rollo",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400343f0,
        name: "Oracle Envoy Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40035b60,
        name: "Page Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40034fa8,
        name: "Perfumer Tricia",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400347d8,
        name: "Putrid Corpse Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003dc48,
        name: "Radahn Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003d478,
        name: "Raya Lucaria Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003ebe8,
        name: "Redmane Knight Ogha",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400395f8,
        name: "Rotten Stray Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40034008,
        name: "Skeletal Bandit Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40033c20,
        name: "Skeletal Militiaman Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003c0f0,
        name: "Soldjars of Fortune Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021a160,
        name: "Spider Scorpion Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400399e0,
        name: "Spirit Jellyfish Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003a1b0,
        name: "Stormhawk Deenh",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021dbf8,
        name: "Swordhand of Night Jolán",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021c0a0,
        name: "Taylew the Golem Smith",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40035778,
        name: "Twinsage Sorcerer Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003b150,
        name: "Vulgar Militia Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003a980,
        name: "Wandering Noble Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40039dc8,
        name: "Warhawk Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40033450,
        name: "Winged Misbegotten Ashes",
        ..Item::default_spirit_ashes()
    },
];
