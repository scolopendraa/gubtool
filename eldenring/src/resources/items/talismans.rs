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
        id: 0x200003e8,
        name: "Crimson Amber Medallion",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003e9,
        name: "Crimson Amber Medallion +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003ea,
        name: "Crimson Amber Medallion +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003f2,
        name: "Cerulean Amber Medallion",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003f3,
        name: "Cerulean Amber Medallion +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003f4,
        name: "Cerulean Amber Medallion +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003fc,
        name: "Viridian Amber Medallion",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003fd,
        name: "Viridian Amber Medallion +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200003fe,
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
        id: 0x2000041a,
        name: "Radagon's Scarseal",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000041b,
        name: "Radagon's Soreseal",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000424,
        name: "Starscourge Heirloom",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000042e,
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
        id: 0x2000044c,
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
        id: 0x2000047e,
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
        id: 0x2000049c,
        name: "Clarifying Horn Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000049d,
        name: "Clarifying Horn Charm +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004a6,
        name: "Prince of Death's Pustule",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004a7,
        name: "Prince of Death's Cyst",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004b0,
        name: "Mottled Necklace",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004b1,
        name: "Mottled Necklace +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004ba,
        name: "Bull-Goat's Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004c4,
        name: "Marika's Scarseal",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004c5,
        name: "Marika's Soreseal",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004ce,
        name: "Warrior Jar Shard",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004cf,
        name: "Shard of Alexander",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200004e2,
        name: "Millicent's Prosthesis",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200007d0,
        name: "Magic Scorpion Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200007da,
        name: "Lightning Scorpion Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200007e4,
        name: "Fire Scorpion Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200007ee,
        name: "Sacred Scorpion Charm",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200007f8,
        name: "Red-Feathered Branchsword",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000802,
        name: "Ritual Sword Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000080c,
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
        id: 0x2000082a,
        name: "Dagger Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000834,
        name: "Arrow's Reach Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000083e,
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
        id: 0x2000085c,
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
        id: 0x2000087a,
        name: "Kindred of Rot's Exultation",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000884,
        name: "Claw Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000088e,
        name: "Roar Medallion",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000898,
        name: "Curved Sword Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200008a2,
        name: "Companion Jar",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200008ac,
        name: "Perfumer's Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000bb8,
        name: "Graven-School Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000bb9,
        name: "Graven-Mass Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000be0,
        name: "Faithful's Canvas Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000bea,
        name: "Flock's Canvas Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000bf4,
        name: "Old Lord's Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000bfe,
        name: "Radagon Icon",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000c08,
        name: "Primal Glintstone Blade",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000c12,
        name: "Godfrey Icon",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fa0,
        name: "Dragoncrest Shield Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fa1,
        name: "Dragoncrest Shield Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fa2,
        name: "Dragoncrest Shield Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fa3,
        name: "Dragoncrest Greatshield Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000faa,
        name: "Spelldrake Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fab,
        name: "Spelldrake Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fac,
        name: "Spelldrake Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fb4,
        name: "Flamedrake Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fb5,
        name: "Flamedrake Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fb6,
        name: "Flamedrake Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fbe,
        name: "Boltdrake Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fbf,
        name: "Boltdrake Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fc0,
        name: "Boltdrake Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fc8,
        name: "Haligdrake Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fc9,
        name: "Haligdrake Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fca,
        name: "Haligdrake Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fd2,
        name: "Pearldrake Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fd3,
        name: "Pearldrake Talisman +1",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fd4,
        name: "Pearldrake Talisman +2",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fdc,
        name: "Crucible Scale Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000fe6,
        name: "Crucible Feather Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000ff0,
        name: "Blue-Feathered Branchsword",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20000ffa,
        name: "Ritual Shield Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001004,
        name: "Greatshield Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000100e,
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
        id: 0x2000139c,
        name: "Blessed Dew Talisman",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200013a6,
        name: "Taker's Cameo",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200013b0,
        name: "Godskin Swaddling Cloth",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200013ba,
        name: "Assassin's Crimson Dagger",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200013c4,
        name: "Assassin's Cerulean Dagger",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001770,
        name: "Crepus's Vial",
        ..Item::default_talismans()
    },
    Item {
        id: 0x2000177a,
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
        id: 0x200017a2,
        name: "Shabriri's Woe",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017ac,
        name: "Daedicar's Woe",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017b6,
        name: "Sacrificial Twig",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017c0,
        name: "Furled Finger's Trick-Mirror",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017ca,
        name: "Host's Trick-Mirror",
        ..Item::default_talismans()
    },
    Item {
        id: 0x200017de,
        name: "Ancestral Spirit's Horne",
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001b58,
        name: "Crimson Amber Medallion +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001b62,
        name: "Cerulean Amber Medallion +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001b6c,
        name: "Viridian Amber Medallion +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001b76,
        name: "Two-Headed Turtle Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001b80,
        name: "Stalwart Horn Charm +2",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001b8a,
        name: "Immunizing Horn Charm +2",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001b94,
        name: "Clarifying Horn Charm +2",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001ba8,
        name: "Mottled Necklace +2",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001bb2,
        name: "Spelldrake Talisman +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001bbc,
        name: "Flamedrake Talisman +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001bc6,
        name: "Boltdrake Talisman +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001bd0,
        name: "Golden Braid",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001bda,
        name: "Pearldrake Talisman +3",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001be4,
        name: "Crimson Seed Talisman +1",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001bee,
        name: "Cerulean Seed Talisman +1",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001f40,
        name: "Blessed Blue Dew Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001f4a,
        name: "Fine Crucible Feather Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001f54,
        name: "Outer God Heirloom",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001f5e,
        name: "Shattered Stone Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001f68,
        name: "Two-Handed Sword Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001f72,
        name: "Crusade Insignia",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001f7c,
        name: "Aged One's Exultation",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001f86,
        name: "Arrow's Soaring Sting Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001f9a,
        name: "Pearl Shield Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001fa4,
        name: "Dried Bouquet",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001fae,
        name: "Smithing Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001fb8,
        name: "Ailment Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001fc2,
        name: "Retaliatory Crossed-Tree",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001fcc,
        name: "Lacerating Crossed-Tree",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001fd6,
        name: "Sharpshot Talisman",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001fe0,
        name: "St. Trina's Smile",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001fea,
        name: "Talisman of the Dread",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001ff4,
        name: "Enraged Divine Beast",
        dlc: true,
        ..Item::default_talismans()
    },
    Item {
        id: 0x20001ffe,
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
        id: 0x2000201c,
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
