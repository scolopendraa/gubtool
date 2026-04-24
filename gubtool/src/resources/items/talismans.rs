use super::Item;

impl Item {
    const fn default_talismans() -> Self {
        Self {
            category: super::Categories::Talismans,
            ..Item::default()
        }
    }
}

pub static TALISMANS: [Item; 154] = [
    Item {
        id: 0x200003E8,
        name: "Crimson Amber Medallion",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003E9,
        name: "Crimson Amber Medallion +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003EA,
        name: "Crimson Amber Medallion +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003F2,
        name: "Cerulean Amber Medallion",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003F3,
        name: "Cerulean Amber Medallion +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003F4,
        name: "Cerulean Amber Medallion +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003FC,
        name: "Viridian Amber Medallion",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003FD,
        name: "Viridian Amber Medallion +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003FE,
        name: "Viridian Amber Medallion +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000406,
        name: "Arsenal Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000407,
        name: "Arsenal Charm +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000408,
        name: "Great-Jar's Arsenal",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000410,
        name: "Erdtree's Favor",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000411,
        name: "Erdtree's Favor +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000412,
        name: "Erdtree's Favor +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000041A,
        name: "Radagon's Scarseal",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000041B,
        name: "Radagon's Soreseal",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000424,
        name: "Starscourge Heirloom",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000042E,
        name: "Prosthesis-Wearer Heirloom",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000438,
        name: "Stargazer Heirloom",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000442,
        name: "Two Fingers Heirloom",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000044C,
        name: "Silver Scarab",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000456,
        name: "Gold Scarab",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000474,
        name: "Moon of Nokstella",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000047E,
        name: "Green Turtle Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000488,
        name: "Stalwart Horn Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000489,
        name: "Stalwart Horn Charm +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000492,
        name: "Immunizing Horn Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000493,
        name: "Immunizing Horn Charm +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000049C,
        name: "Clarifying Horn Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000049D,
        name: "Clarifying Horn Charm +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004A6,
        name: "Prince of Death's Pustule",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004A7,
        name: "Prince of Death's Cyst",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004B0,
        name: "Mottled Necklace",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004B1,
        name: "Mottled Necklace +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004BA,
        name: "Bull-Goat's Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004C4,
        name: "Marika's Scarseal",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004C5,
        name: "Marika's Soreseal",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004CE,
        name: "Warrior Jar Shard",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004CF,
        name: "Shard of Alexander",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004E2,
        name: "Millicent's Prosthesis",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200007D0,
        name: "Magic Scorpion Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200007DA,
        name: "Lightning Scorpion Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200007E4,
        name: "Fire Scorpion Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200007EE,
        name: "Sacred Scorpion Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200007F8,
        name: "Red-Feathered Branchsword",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000802,
        name: "Ritual Sword Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000080C,
        name: "Spear Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000816,
        name: "Hammer Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000820,
        name: "Winged Sword Insignia",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000821,
        name: "Rotten Winged Sword Insignia",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000082A,
        name: "Dagger Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000834,
        name: "Arrow's Reach Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000083E,
        name: "Blue Dancer Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000848,
        name: "Twinblade Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000852,
        name: "Axe Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000085C,
        name: "Lance Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000866,
        name: "Arrow's Sting Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000870,
        name: "Lord of Blood's Exultation",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000087A,
        name: "Kindred of Rot's Exultation",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000884,
        name: "Claw Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000088E,
        name: "Roar Medallion",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000898,
        name: "Curved Sword Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200008A2,
        name: "Companion Jar",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200008AC,
        name: "Perfumer's Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000BB8,
        name: "Graven-School Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000BB9,
        name: "Graven-Mass Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000BE0,
        name: "Faithful's Canvas Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000BEA,
        name: "Flock's Canvas Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000BF4,
        name: "Old Lord's Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000BFE,
        name: "Radagon Icon",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000C08,
        name: "Primal Glintstone Blade",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000C12,
        name: "Godfrey Icon",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FA0,
        name: "Dragoncrest Shield Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FA1,
        name: "Dragoncrest Shield Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FA2,
        name: "Dragoncrest Shield Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FA3,
        name: "Dragoncrest Greatshield Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FAA,
        name: "Spelldrake Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FAB,
        name: "Spelldrake Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FAC,
        name: "Spelldrake Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FB4,
        name: "Flamedrake Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FB5,
        name: "Flamedrake Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FB6,
        name: "Flamedrake Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FBE,
        name: "Boltdrake Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FBF,
        name: "Boltdrake Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FC0,
        name: "Boltdrake Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FC8,
        name: "Haligdrake Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FC9,
        name: "Haligdrake Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FCA,
        name: "Haligdrake Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FD2,
        name: "Pearldrake Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FD3,
        name: "Pearldrake Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FD4,
        name: "Pearldrake Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FDC,
        name: "Crucible Scale Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FE6,
        name: "Crucible Feather Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FF0,
        name: "Blue-Feathered Branchsword",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000FFA,
        name: "Ritual Shield Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001004,
        name: "Greatshield Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000100E,
        name: "Crucible Knot Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001388,
        name: "Crimson Seed Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001392,
        name: "Cerulean Seed Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000139C,
        name: "Blessed Dew Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200013A6,
        name: "Taker's Cameo",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200013B0,
        name: "Godskin Swaddling Cloth",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200013BA,
        name: "Assassin's Crimson Dagger",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200013C4,
        name: "Assassin's Cerulean Dagger",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001770,
        name: "Crepus's Vial",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000177A,
        name: "Concealing Veil",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001784,
        name: "Carian Filigreed Crest",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001798,
        name: "Longtail Cat Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017A2,
        name: "Shabriri's Woe",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017AC,
        name: "Daedicar's Woe",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017B6,
        name: "Sacrificial Twig",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017C0,
        name: "Furled Finger's Trick-Mirror",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017CA,
        name: "Host's Trick-Mirror",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017DE,
        name: "Ancestral Spirit's Horne",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001B58,
        name: "Crimson Amber Medallion +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001B62,
        name: "Cerulean Amber Medallion +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001B6C,
        name: "Viridian Amber Medallion +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001B76,
        name: "Two-Headed Turtle Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001B80,
        name: "Stalwart Horn Charm +2",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001B8A,
        name: "Immunizing Horn Charm +2",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001B94,
        name: "Clarifying Horn Charm +2",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001BA8,
        name: "Mottled Necklace +2",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001BB2,
        name: "Spelldrake Talisman +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001BBC,
        name: "Flamedrake Talisman +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001BC6,
        name: "Boltdrake Talisman +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001BD0,
        name: "Golden Braid",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001BDA,
        name: "Pearldrake Talisman +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001BE4,
        name: "Crimson Seed Talisman +1",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001BEE,
        name: "Cerulean Seed Talisman +1",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001F40,
        name: "Blessed Blue Dew Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001F4A,
        name: "Fine Crucible Feather Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001F54,
        name: "Outer God Heirloom",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001F5E,
        name: "Shattered Stone Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001F68,
        name: "Two-Handed Sword Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001F72,
        name: "Crusade Insignia",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001F7C,
        name: "Aged One's Exultation",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001F86,
        name: "Arrow's Soaring Sting Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001F9A,
        name: "Pearl Shield Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001FA4,
        name: "Dried Bouquet",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001FAE,
        name: "Smithing Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001FB8,
        name: "Ailment Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001FC2,
        name: "Retaliatory Crossed-Tree",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001FCC,
        name: "Lacerating Crossed-Tree",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001FD6,
        name: "Sharpshot Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001FE0,
        name: "St. Trina's Smile",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001FEA,
        name: "Talisman of the Dread",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001FF4,
        name: "Enraged Divine Beast",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001FFE,
        name: "Beloved Stardust",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20002008,
        name: "Talisman of Lord's Bestowal",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20002012,
        name: "Verdigris Discus",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000201C,
        name: "Rellana's Cameo",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20002026,
        name: "Blade of Mercy",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20002030,
        name: "Talisman of All Crucibles",
        dlc: true,
        ..Item::default_talismans()
    },
];
