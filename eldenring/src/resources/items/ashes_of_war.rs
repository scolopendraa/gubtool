use super::Item;

impl Item {
    const fn default_aow() -> Self {
        Self {
            category: super::Categories::AshesOfWar,
            max_storage: 0,
            ..Item::default()
        }
    }
}

pub static ASHES_OF_WAR: [Item; 116] = [
    Item {
        id: 0x80030da4,
        name: "Aspects of the Crucible: Wings",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x8000eb28,
        name: "Assassin's Gambit",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000fde8,
        name: "Barbaric Roar",
        ..Item::default_aow()
    },
    Item {
        id: 0x80009ca4,
        name: "Barrage",
        ..Item::default_aow()
    },
    Item {
        id: 0x80007594,
        name: "Barricade Shield",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000feb0,
        name: "Beast's Roar",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005654,
        name: "Black Flame Tornado",
        ..Item::default_aow()
    },
    Item {
        id: 0x80063da8,
        name: "Blind Spot",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80064d48,
        name: "Blinkbolt",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80005780,
        name: "Blood Blade",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002a30,
        name: "Blood Tax",
        ..Item::default_aow()
    },
    Item {
        id: 0x800138e4,
        name: "Bloodhound's Step",
        ..Item::default_aow()
    },
    Item {
        id: 0x80004fb0,
        name: "Bloody Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000ff78,
        name: "Braggart's Roar",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005528,
        name: "Carian Grandeur",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000558c,
        name: "Carian Greatsword",
        ..Item::default_aow()
    },
    Item {
        id: 0x80007724,
        name: "Carian Retaliation",
        ..Item::default_aow()
    },
    Item {
        id: 0x800660d0,
        name: "Carian Sovereignty",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80002904,
        name: "Charge Forth",
        ..Item::default_aow()
    },
    Item {
        id: 0x800058ac,
        name: "Chilling Mist",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000ed1c,
        name: "Cragblade",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000ea60,
        name: "Determination",
        ..Item::default_aow()
    },
    Item {
        id: 0x80065900,
        name: "Divine Beast Frost Stomp",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80002bc0,
        name: "Double Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x80030d40,
        name: "Dryleaf Whirlwind",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x800052d0,
        name: "Earthshaker",
        ..Item::default_aow()
    },
    Item {
        id: 0x80009dd0,
        name: "Enchanted Shot",
        ..Item::default_aow()
    },
    Item {
        id: 0x80011170,
        name: "Endure",
        ..Item::default_aow()
    },
    Item {
        id: 0x800050dc,
        name: "Eruption",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000c544,
        name: "Flame of the Redmanes",
        ..Item::default_aow()
    },
    Item {
        id: 0x80065130,
        name: "Flame Skewer",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80065ce8,
        name: "Flame Spear",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80005398,
        name: "Flaming Strike",
        ..Item::default_aow()
    },
    Item {
        id: 0x80067070,
        name: "Ghostflame Call",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80002d50,
        name: "Giant Hunt",
        ..Item::default_aow()
    },
    Item {
        id: 0x80004e20,
        name: "Glintblade Phalanx",
        ..Item::default_aow()
    },
    Item {
        id: 0x80004f4c,
        name: "Glintstone Pebble",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005334,
        name: "Golden Land",
        ..Item::default_aow()
    },
    Item {
        id: 0x800077ec,
        name: "Golden Parry",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000c60c,
        name: "Golden Slam",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000eb8c,
        name: "Golden Vow",
        ..Item::default_aow()
    },
    Item {
        id: 0x800051a4,
        name: "Gravitas",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000c5a8,
        name: "Ground Slam",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000c6d4,
        name: "Hoarah Loux's Earthshaker",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000c3b4,
        name: "Hoarfrost Stomp",
        ..Item::default_aow()
    },
    Item {
        id: 0x80011238,
        name: "Holy Ground",
        ..Item::default_aow()
    },
    Item {
        id: 0x80004ee8,
        name: "Ice Spear",
        ..Item::default_aow()
    },
    Item {
        id: 0x80085ca0,
        name: "Igon's Drake Hunt",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80002774,
        name: "Impaling Thrust",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000c47c,
        name: "Kick",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005014,
        name: "Lifesteal Fist",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000c4e0,
        name: "Lightning Ram",
        ..Item::default_aow()
    },
    Item {
        id: 0x800054c4,
        name: "Lightning Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002710,
        name: "Lion's Claw",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002e18,
        name: "Loretta's Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x80009d08,
        name: "Mighty Shot",
        ..Item::default_aow()
    },
    Item {
        id: 0x800078b4,
        name: "No Skill",
        ..Item::default_aow()
    },
    Item {
        id: 0x80064578,
        name: "Overhead Stance",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80061e68,
        name: "Palm Blast",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x800075f8,
        name: "Parry",
        ..Item::default_aow()
    },
    Item {
        id: 0x800057e4,
        name: "Phantom Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x800027d8,
        name: "Piercing Fang",
        ..Item::default_aow()
    },
    Item {
        id: 0x80062250,
        name: "Piercing Throw",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80002e7c,
        name: "Poison Moth Flight",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005910,
        name: "Poisonous Mist",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005140,
        name: "Prayerful Strike",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002c24,
        name: "Prelate's Charge",
        ..Item::default_aow()
    },
    Item {
        id: 0x80013880,
        name: "Quickstep",
        ..Item::default_aow()
    },
    Item {
        id: 0x800631f0,
        name: "Raging Beast",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80009e98,
        name: "Rain of Arrows",
        ..Item::default_aow()
    },
    Item {
        id: 0x80013948,
        name: "Raptor of the Mists",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002a94,
        name: "Repeating Thrust",
        ..Item::default_aow()
    },
    Item {
        id: 0x80062e08,
        name: "Rolling Sparks",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x8000eac4,
        name: "Royal Knight's Resolve",
        ..Item::default_aow()
    },
    Item {
        id: 0x80004e84,
        name: "Sacred Blade",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000ebf0,
        name: "Sacred Order",
        ..Item::default_aow()
    },
    Item {
        id: 0x800056b8,
        name: "Sacred Ring of Light",
        ..Item::default_aow()
    },
    Item {
        id: 0x800635d8,
        name: "Savage Claws",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80065518,
        name: "Savage Lion's Claw",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80062638,
        name: "Scattershot Throw",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x8000ecb8,
        name: "Seppuku",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000ec54,
        name: "Shared Order",
        ..Item::default_aow()
    },
    Item {
        id: 0x80007530,
        name: "Shield Bash",
        ..Item::default_aow()
    },
    Item {
        id: 0x80007850,
        name: "Shield Crash",
        ..Item::default_aow()
    },
    Item {
        id: 0x800c3500,
        name: "Shield Strike",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x800664b8,
        name: "Shriek of Sorrow",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80009e34,
        name: "Sky Shot",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005848,
        name: "Spectral Lance",
        ..Item::default_aow()
    },
    Item {
        id: 0x80061a80,
        name: "Spinning Gravity Thrust",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x8000283c,
        name: "Spinning Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002b5c,
        name: "Spinning Strikes",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002ee0,
        name: "Spinning Weapon",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002cec,
        name: "Square Off",
        ..Item::default_aow()
    },
    Item {
        id: 0x800029cc,
        name: "Stamp (Sweep)",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002968,
        name: "Stamp (Upward Cut)",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002fa8,
        name: "Storm Assault",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005208,
        name: "Storm Blade",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000c418,
        name: "Storm Stomp",
        ..Item::default_aow()
    },
    Item {
        id: 0x80007788,
        name: "Storm Wall",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000300c,
        name: "Stormcaller",
        ..Item::default_aow()
    },
    Item {
        id: 0x80064190,
        name: "Swift Slash",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80003070,
        name: "Sword Dance",
        ..Item::default_aow()
    },
    Item {
        id: 0x8007b4a8,
        name: "The Poison Flower Blooms Twice",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80007918,
        name: "Thops's Barrier",
        ..Item::default_aow()
    },
    Item {
        id: 0x80009c40,
        name: "Through and Through",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005460,
        name: "Thunderbolt",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000ff14,
        name: "Troll's Roar",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002c88,
        name: "Unsheathe",
        ..Item::default_aow()
    },
    Item {
        id: 0x800055f0,
        name: "Vacuum Slice",
        ..Item::default_aow()
    },
    Item {
        id: 0x800111d4,
        name: "Vow of the Indomitable",
        ..Item::default_aow()
    },
    Item {
        id: 0x80062a20,
        name: "Wall of Sparks",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x8000fe4c,
        name: "War Cry",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000c670,
        name: "Waves of Darkness",
        ..Item::default_aow()
    },
    Item {
        id: 0x80014c08,
        name: "White Shadow's Lure",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002af8,
        name: "Wild Strikes",
        ..Item::default_aow()
    },
    Item {
        id: 0x80064960,
        name: "Wing Stance",
        dlc: true,
        ..Item::default_aow()
    },
];
