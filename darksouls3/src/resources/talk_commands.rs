use crate::esd::TalkCommand;

pub const MENUS: [TalkCommand; 4] = [
    TalkCommand {
        name:   "Level Up",
        id:     31,
        params: &[],
    },
    TalkCommand {
        name:   "Repair",
        id:     23,
        params: &[],
    },
    TalkCommand {
        name:   "Allot Estus",
        id:     105,
        params: &[],
    },
    TalkCommand {
        name:   "Warp",
        id:     41,
        params: &[],
    },
];

pub const SHOPS: [TalkCommand; 10] = [
    TalkCommand {
        name:   "Shrine Handmaid",
        id:     22,
        params: &[110000, 119999],
    },
    TalkCommand {
        name:   "Ludleth of Courland",
        id:     22,
        params: &[30000, 31000],
    },
    TalkCommand {
        name:   "Greirat of the Undead Settlement",
        id:     22,
        params: &[120000, 129999],
    },
    TalkCommand {
        name:   "Yoel of Londor",
        id:     22,
        params: &[50000, 59999],
    },
    TalkCommand {
        name:   "Yuria of Londor",
        id:     22,
        params: &[40000, 49999],
    },
    TalkCommand {
        name:   "Irina of Carim",
        id:     22,
        params: &[160000, 169999],
    },
    TalkCommand {
        name:   "Unbreakable Patches",
        id:     22,
        params: &[200000, 209999],
    },
    TalkCommand {
        name:   "Cornyx of the Great Swamp",
        id:     22,
        params: &[140000, 149999],
    },
    TalkCommand {
        name:   "Orbeck of Vinheim",
        id:     22,
        params: &[130000, 139999],
    },
    TalkCommand {
        name:   "Karla",
        id:     22,
        params: &[150000, 159999],
    },
];
