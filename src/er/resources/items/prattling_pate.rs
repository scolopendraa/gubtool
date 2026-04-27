use super::Item;

impl Item {
    const fn default_prattling_pate() -> Self {
        Self {
            category: super::Categories::PrattlingPate,
            max_storage: 600,
            ..Item::default()
        }
    }
}

pub static PRATTLING_PATE: [Item; 9] = [
    Item {
        id: 0x4000089A,
        name: "Prattling Pate \"Apologies\"",
        ..Item::default_prattling_pate()
    },
    Item {
        id: 0x40000898,
        name: "Prattling Pate \"Hello\"",
        ..Item::default_prattling_pate()
    },
    Item {
        id: 0x401E8CE6,
        name: "Prattling Pate \"Lamentation\"",
        dlc: true,
        ..Item::default_prattling_pate()
    },
    Item {
        id: 0x4000089E,
        name: "Prattling Pate \"Let's get to it\"",
        ..Item::default_prattling_pate()
    },
    Item {
        id: 0x4000089D,
        name: "Prattling Pate \"My beloved\"",
        ..Item::default_prattling_pate()
    },
    Item {
        id: 0x4000089C,
        name: "Prattling Pate \"Please help\"",
        ..Item::default_prattling_pate()
    },
    Item {
        id: 0x40000899,
        name: "Prattling Pate \"Thank you\"",
        ..Item::default_prattling_pate()
    },
    Item {
        id: 0x4000089B,
        name: "Prattling Pate \"Wonderful\"",
        ..Item::default_prattling_pate()
    },
    Item {
        id: 0x4000089F,
        name: "Prattling Pate \"You're beautiful\"",
        ..Item::default_prattling_pate()
    },
];
