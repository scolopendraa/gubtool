use super::Item;

impl Item {
    const fn default_arrow() -> Self {
        Self {
            category: super::Categories::Arrows,
            stack_size: 999,
            ..Item::default()
        }
    }
}

pub static ARROWS: &[Item; 18] = &[
    Item {
        id: 60760000,
        name: "Wood Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60770000,
        name: "Iron Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60780000,
        name: "Magic Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60790000,
        name: "Lightning Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60800000,
        name: "Fire Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60810000,
        name: "Dark Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60820000,
        name: "Poison Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60830000,
        name: "Lacerating Arrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60850000,
        name: "Iron Greatarrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60870000,
        name: "Lightning Greatarrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60880000,
        name: "Fire Greatarrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60900000,
        name: "Destructive Greatarrow",
        ..Item::default_arrow()
    },
    Item {
        id: 60910000,
        name: "Wood Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 60920000,
        name: "Heavy Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 60930000,
        name: "Magic Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 60940000,
        name: "Lightning Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 60950000,
        name: "Fire Bolt",
        ..Item::default_arrow()
    },
    Item {
        id: 60960000,
        name: "Dark Bolt",
        ..Item::default_arrow()
    },
];