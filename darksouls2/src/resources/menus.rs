use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum MenuType {
    Incense          = 0x1,
    Shop             = 0x3,
    Repair           = 0x4,
    Reinforcement    = 0x5,
    Infusion         = 0x6,
    Sell             = 0x9,
    Trading          = 0xa,
    GiveEquipment    = 0xb,
    Covenants        = 0x12,
    ReinforcePyro    = 0x15,
    ChampionsTablet  = 0x17,
    DeathCount       = 0x18,
    LevelUp          = 0x19,
    ReallocatePoints = 0x1a,
}

pub const MENUS: [MenuType; 12] = [
    MenuType::LevelUp,
    MenuType::Sell,
    MenuType::Reinforcement,
    MenuType::ReallocatePoints,
    MenuType::GiveEquipment,
    MenuType::Infusion,
    MenuType::ReallocatePoints,
    MenuType::Incense,
    MenuType::Covenants,
    MenuType::ReinforcePyro,
    MenuType::DeathCount,
    MenuType::ChampionsTablet,
];

impl Display for MenuType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Incense => "Incense",
            Self::Shop => "Shop",
            Self::Repair => "Repair",
            Self::Reinforcement => "Reinforcement",
            Self::Infusion => "Infusion",
            Self::Sell => "Sell",
            Self::Trading => "Trading",
            Self::GiveEquipment => "Give Equipment",
            Self::Covenants => "Covenants",
            Self::ReinforcePyro => "Reinforce Pyromancy Flame",
            Self::ChampionsTablet => "Champion's Tablet (Online)",
            Self::DeathCount => "Death Count",
            Self::LevelUp => "Level Up",
            Self::ReallocatePoints => "Reallocate Points",
        };
        write!(f, "{}", text)
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Shop {
    Melentia   = 75400000,
    Gilligan   = 70400000,
    Wellager   = 72110000,
    Grandahl   = 72500000,
    Gavlan     = 72600000,
    RatKing    = 75600000,
    Maughlin   = 76100000,
    Chloanne   = 76200000,
    Rosabeth   = 76300000,
    Lenigrast  = 76400000,
    McDuff     = 76430000,
    Carhillion = 76600000,
    Straid     = 76800000,
    Licia      = 76900000,
    Felkin     = 77000000,
    Navlaan    = 77100000,
    Magerold   = 77200000,
    Ornifex    = 77600000,
    Shalquoir  = 77700000,
    TitchyGren = 78300000,
    Cromwell   = 78400000,
    Targray    = 78500000,
    Vengarl    = 30700000,
    Agdayne    = 50600000,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Trade {
    Straid  = 76801000,
    Ornifex = 77601000,
}

pub const SHOPS: [Shop; 24] = [
    Shop::Melentia,
    Shop::Gilligan,
    Shop::Wellager,
    Shop::Grandahl,
    Shop::Gavlan,
    Shop::RatKing,
    Shop::Maughlin,
    Shop::Chloanne,
    Shop::Rosabeth,
    Shop::Lenigrast,
    Shop::McDuff,
    Shop::Carhillion,
    Shop::Straid,
    Shop::Licia,
    Shop::Felkin,
    Shop::Navlaan,
    Shop::Magerold,
    Shop::Ornifex,
    Shop::Shalquoir,
    Shop::TitchyGren,
    Shop::Cromwell,
    Shop::Targray,
    Shop::Vengarl,
    Shop::Agdayne,
];

pub const TRADES: [Trade; 2] = [Trade::Straid, Trade::Ornifex];

impl Display for Shop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Melentia => "Merchant Hag Melentia",
            Self::Gilligan => "Laddersmith Gilligan",
            Self::Wellager => "Chancellor Wellager",
            Self::Grandahl => "Darkdiver Grandahl",
            Self::Gavlan => "Lonesome Gavlan",
            Self::RatKing => "The Rat King",
            Self::Maughlin => "Maughlin the Armorer",
            Self::Chloanne => "Stone Trader Chloanne",
            Self::Rosabeth => "Rosabeth of Melfia",
            Self::Lenigrast => "Blacksmith Lenigrast",
            Self::McDuff => "Steady Hand McDuff",
            Self::Carhillion => "Carhillion of the Fold",
            Self::Straid => "Straid of Olaphis",
            Self::Licia => "Licia of Lindeldt",
            Self::Felkin => "Felkin the Outcast",
            Self::Navlaan => "Royal Sorcerer Navlaan",
            Self::Magerold => "Magerold of Lanafir",
            Self::Ornifex => "Weaponsmith Ornifex",
            Self::Shalquoir => "Sweet Shalquoir",
            Self::TitchyGren => "Titchy Gren",
            Self::Cromwell => "Cromwell the Pardoner",
            Self::Targray => "Blue Sentinel Targray",
            Self::Vengarl => "Head of Vengarl",
            Self::Agdayne => "Grave Warden Agdayne",
        };
        write!(f, "{}", text)
    }
}

impl Display for Trade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Straid => "Straid of Olaphis",
            Self::Ornifex => "Weaponsmith Ornifex",
        };
        write!(f, "{}", text)
    }
}
