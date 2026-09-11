use super::Item;

impl Item {
    const fn default_arrow() -> Self {
        Self {
            category: super::ItemCategory::Arrows,
            stack_size: 99,
            max_storage: 600,
            ..Item::default()
        }
    }
}

pub static ARROWS: [Item; 19] = [
    Item {
        name: "Dark Arrow",
        id: 0x61d3c,
        ..Item::default_arrow()
    },
    Item {
        name: "Feather Arrow",
        id: 0x61c10,
        ..Item::default_arrow()
    },
    Item {
        name: "Fire Arrow",
        id: 0x61ae4,
        ..Item::default_arrow()
    },
    Item {
        name: "Large Arrow",
        id: 0x61bac,
        ..Item::default_arrow()
    },
    Item {
        name: "Moonlight Arrow",
        id: 0x61c74,
        ..Item::default_arrow()
    },
    Item {
        name: "Poison Arrow",
        id: 0x61b48,
        ..Item::default_arrow()
    },
    Item {
        name: "Standard Arrow",
        id: 0x61a80,
        ..Item::default_arrow()
    },
    Item {
        name: "Wood Arrow",
        id: 0x61cd8,
        ..Item::default_arrow()
    },
    Item {
        name: "Dragonslayer Greatarrow",
        id: 0x62250,
        stack_size: 50,
        ..Item::default_arrow()
    },
    Item {
        name: "Dragonslayer Lightning Arrow",
        id: 0x62318,
        stack_size: 50,
        ..Item::default_arrow()
    },
    Item {
        name: "Millwood Greatarrow",
        id: 0x623e0,
        stack_size: 30,
        dlc: 1,
        ..Item::default_arrow()
    },
    Item {
        name: "Onislayer Greatarrow",
        id: 0x6237c,
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        name: "Exploding Bolt",
        id: 0x62c78,
        stack_size: 30,
        ..Item::default_arrow()
    },
    Item {
        name: "Heavy Bolt",
        id: 0x62a84,
        ..Item::default_arrow()
    },
    Item {
        name: "Lightning Bolt",
        id: 0x62bb0,
        ..Item::default_arrow()
    },
    Item {
        name: "Sniper Bolt",
        id: 0x62ae8,
        ..Item::default_arrow()
    },
    Item {
        name: "Splintering Bolt",
        id: 0x62c14,
        stack_size: 60,
        ..Item::default_arrow()
    },
    Item {
        name: "Standard Bolt",
        id: 0x62a20,
        ..Item::default_arrow()
    },
    Item {
        name: "Wood Bolt",
        id: 0x62b4c,
        ..Item::default_arrow()
    },
];
