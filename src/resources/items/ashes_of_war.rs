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
        id: 0x80030DA4,
        name: "Aspects of the Crucible: Wings",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x8000EB28,
        name: "Assassin's Gambit",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000FDE8,
        name: "Barbaric Roar",
        ..Item::default_aow()
    },
    Item {
        id: 0x80009CA4,
        name: "Barrage",
        ..Item::default_aow()
    },
    Item {
        id: 0x80007594,
        name: "Barricade Shield",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000FEB0,
        name: "Beast's Roar",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005654,
        name: "Black Flame Tornado",
        ..Item::default_aow()
    },
    Item {
        id: 0x80063DA8,
        name: "Blind Spot",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80064D48,
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
        id: 0x80002A30,
        name: "Blood Tax",
        ..Item::default_aow()
    },
    Item {
        id: 0x800138E4,
        name: "Bloodhound's Step",
        ..Item::default_aow()
    },
    Item {
        id: 0x80004FB0,
        name: "Bloody Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000FF78,
        name: "Braggart's Roar",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005528,
        name: "Carian Grandeur",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000558C,
        name: "Carian Greatsword",
        ..Item::default_aow()
    },
    Item {
        id: 0x80007724,
        name: "Carian Retaliation",
        ..Item::default_aow()
    },
    Item {
        id: 0x800660D0,
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
        id: 0x800058AC,
        name: "Chilling Mist",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000ED1C,
        name: "Cragblade",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000EA60,
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
        id: 0x80002BC0,
        name: "Double Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x80030D40,
        name: "Dryleaf Whirlwind",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x800052D0,
        name: "Earthshaker",
        ..Item::default_aow()
    },
    Item {
        id: 0x80009DD0,
        name: "Enchanted Shot",
        ..Item::default_aow()
    },
    Item {
        id: 0x80011170,
        name: "Endure",
        ..Item::default_aow()
    },
    Item {
        id: 0x800050DC,
        name: "Eruption",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000C544,
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
        id: 0x80065CE8,
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
        id: 0x80002D50,
        name: "Giant Hunt",
        ..Item::default_aow()
    },
    Item {
        id: 0x80004E20,
        name: "Glintblade Phalanx",
        ..Item::default_aow()
    },
    Item {
        id: 0x80004F4C,
        name: "Glintstone Pebble",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005334,
        name: "Golden Land",
        ..Item::default_aow()
    },
    Item {
        id: 0x800077EC,
        name: "Golden Parry",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000C60C,
        name: "Golden Slam",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000EB8C,
        name: "Golden Vow",
        ..Item::default_aow()
    },
    Item {
        id: 0x800051A4,
        name: "Gravitas",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000C5A8,
        name: "Ground Slam",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000C6D4,
        name: "Hoarah Loux's Earthshaker",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000C3B4,
        name: "Hoarfrost Stomp",
        ..Item::default_aow()
    },
    Item {
        id: 0x80011238,
        name: "Holy Ground",
        ..Item::default_aow()
    },
    Item {
        id: 0x80004EE8,
        name: "Ice Spear",
        ..Item::default_aow()
    },
    Item {
        id: 0x80085CA0,
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
        id: 0x8000C47C,
        name: "Kick",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005014,
        name: "Lifesteal Fist",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000C4E0,
        name: "Lightning Ram",
        ..Item::default_aow()
    },
    Item {
        id: 0x800054C4,
        name: "Lightning Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002710,
        name: "Lion's Claw",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002E18,
        name: "Loretta's Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x80009D08,
        name: "Mighty Shot",
        ..Item::default_aow()
    },
    Item {
        id: 0x800078B4,
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
        id: 0x80061E68,
        name: "Palm Blast",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x800075F8,
        name: "Parry",
        ..Item::default_aow()
    },
    Item {
        id: 0x800057E4,
        name: "Phantom Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x800027D8,
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
        id: 0x80002E7C,
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
        id: 0x80002C24,
        name: "Prelate's Charge",
        ..Item::default_aow()
    },
    Item {
        id: 0x80013880,
        name: "Quickstep",
        ..Item::default_aow()
    },
    Item {
        id: 0x800631F0,
        name: "Raging Beast",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80009E98,
        name: "Rain of Arrows",
        ..Item::default_aow()
    },
    Item {
        id: 0x80013948,
        name: "Raptor of the Mists",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002A94,
        name: "Repeating Thrust",
        ..Item::default_aow()
    },
    Item {
        id: 0x80062E08,
        name: "Rolling Sparks",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x8000EAC4,
        name: "Royal Knight's Resolve",
        ..Item::default_aow()
    },
    Item {
        id: 0x80004E84,
        name: "Sacred Blade",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000EBF0,
        name: "Sacred Order",
        ..Item::default_aow()
    },
    Item {
        id: 0x800056B8,
        name: "Sacred Ring of Light",
        ..Item::default_aow()
    },
    Item {
        id: 0x800635D8,
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
        id: 0x8000ECB8,
        name: "Seppuku",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000EC54,
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
        id: 0x800C3500,
        name: "Shield Strike",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x800664B8,
        name: "Shriek of Sorrow",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x80009E34,
        name: "Sky Shot",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005848,
        name: "Spectral Lance",
        ..Item::default_aow()
    },
    Item {
        id: 0x80061A80,
        name: "Spinning Gravity Thrust",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x8000283C,
        name: "Spinning Slash",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002B5C,
        name: "Spinning Strikes",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002EE0,
        name: "Spinning Weapon",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002CEC,
        name: "Square Off",
        ..Item::default_aow()
    },
    Item {
        id: 0x800029CC,
        name: "Stamp (Sweep)",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002968,
        name: "Stamp (Upward Cut)",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002FA8,
        name: "Storm Assault",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005208,
        name: "Storm Blade",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000C418,
        name: "Storm Stomp",
        ..Item::default_aow()
    },
    Item {
        id: 0x80007788,
        name: "Storm Wall",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000300C,
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
        id: 0x8007B4A8,
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
        id: 0x80009C40,
        name: "Through and Through",
        ..Item::default_aow()
    },
    Item {
        id: 0x80005460,
        name: "Thunderbolt",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000FF14,
        name: "Troll's Roar",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002C88,
        name: "Unsheathe",
        ..Item::default_aow()
    },
    Item {
        id: 0x800055F0,
        name: "Vacuum Slice",
        ..Item::default_aow()
    },
    Item {
        id: 0x800111D4,
        name: "Vow of the Indomitable",
        ..Item::default_aow()
    },
    Item {
        id: 0x80062A20,
        name: "Wall of Sparks",
        dlc: true,
        ..Item::default_aow()
    },
    Item {
        id: 0x8000FE4C,
        name: "War Cry",
        ..Item::default_aow()
    },
    Item {
        id: 0x8000C670,
        name: "Waves of Darkness",
        ..Item::default_aow()
    },
    Item {
        id: 0x80014C08,
        name: "White Shadow's Lure",
        ..Item::default_aow()
    },
    Item {
        id: 0x80002AF8,
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
