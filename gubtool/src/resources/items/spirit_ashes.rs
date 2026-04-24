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
        id: 0x4021CC58,
        name: "Ancient Dragon Florissax",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003E800,
        name: "Ancient Dragon Knight Kristoff",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003CCA8,
        name: "Archer Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400372D0,
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
        id: 0x40035F48,
        name: "Battlemage Hugues",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021B8D0,
        name: "Bigmouth Imp Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40030D40,
        name: "Black Knife Tiche",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021B4E8,
        name: "Black Knight Captain Huw",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021B100,
        name: "Black Knight Commander Andreas",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40037AA0,
        name: "Blackflame Monk Amon",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x402195A8,
        name: "Bloodfiend Hexer's Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003A598,
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
        id: 0x40032C80,
        name: "Crystalian Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x402191C0,
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
        id: 0x4021A930,
        name: "Demi-Human Swordsman Yosh",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40034BC0,
        name: "Depraved Perfumer Carmaan",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021C488,
        name: "Divine Bird Warrior Ornis",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003FF70,
        name: "Dolores the Sleeping Arrow Puppet",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003F7A0,
        name: "Dung Eater Puppet",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400318F8,
        name: "Fanged Imp Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003FB88,
        name: "Finger Maiden Therolina Puppet",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021D428,
        name: "Fingercreeper Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40219D78,
        name: "Fire Knight Hilde",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021D810,
        name: "Fire Knight Queelign",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400376B8,
        name: "Fire Monk Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40038E28,
        name: "Giant Rat Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40035390,
        name: "Glintstone Sorcerer Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003D090,
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
        id: 0x4003C8C0,
        name: "Greatshield Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003E418,
        name: "Haligtree Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021C870,
        name: "Horned Warrior Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021A548,
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
        id: 0x4021DFE0,
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
        id: 0x40036B00,
        name: "Kindred of Rot Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003B920,
        name: "Land Squirt Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40031CE0,
        name: "Latenna the Albinauric",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003D860,
        name: "Leyndell Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003EFD0,
        name: "Lhutel the Headless",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40038A40,
        name: "Lone Wolf Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003B538,
        name: "Mad Pumpkin Head Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021BCB8,
        name: "Man-Fly Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40037E88,
        name: "Man-Serpent Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40036EE8,
        name: "Marionette Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003E030,
        name: "Mausoleum Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021AD18,
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
        id: 0x4003BD08,
        name: "Miranda Sprout Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003F3B8,
        name: "Nepheli Loux Puppet",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400324B0,
        name: "Nightmaiden & Swordstress Puppets",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003AD68,
        name: "Noble Sorcerer Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400320C8,
        name: "Nomad Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003C4D8,
        name: "Omenkiller Rollo",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400343F0,
        name: "Oracle Envoy Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40035B60,
        name: "Page Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40034FA8,
        name: "Perfumer Tricia",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400347D8,
        name: "Putrid Corpse Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003DC48,
        name: "Radahn Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003D478,
        name: "Raya Lucaria Soldier Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003EBE8,
        name: "Redmane Knight Ogha",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400395F8,
        name: "Rotten Stray Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40034008,
        name: "Skeletal Bandit Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40033C20,
        name: "Skeletal Militiaman Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003C0F0,
        name: "Soldjars of Fortune Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021A160,
        name: "Spider Scorpion Ashes",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x400399E0,
        name: "Spirit Jellyfish Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003A1B0,
        name: "Stormhawk Deenh",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021DBF8,
        name: "Swordhand of Night Jolán",
        dlc: true,
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4021C0A0,
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
        id: 0x4003B150,
        name: "Vulgar Militia Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x4003A980,
        name: "Wandering Noble Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40039DC8,
        name: "Warhawk Ashes",
        ..Item::default_spirit_ashes()
    },
    Item {
        id: 0x40033450,
        name: "Winged Misbegotten Ashes",
        ..Item::default_spirit_ashes()
    },
];
