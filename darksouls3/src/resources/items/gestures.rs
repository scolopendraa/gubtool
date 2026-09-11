use super::Item;

impl Item {
    const fn default_gesture() -> Self {
        Self {
            category: super::ItemCategory::Gestures,
            stack_size: 1,
            max_storage: 1,
            ..Item::default()
        }
    }
}

pub static GESTURES: [Item; 33] = [
    Item {
        id: 0x40002344,
        name: "Patches Squat",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002346,
        name: "Path of the Dragon",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002331,
        name: "Point down",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002329,
        name: "Point forward",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002330,
        name: "Point up",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000232f,
        name: "Praise the Sun",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002336,
        name: "Prayer",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002335,
        name: "Proper bow",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002334,
        name: "Prostration",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000233a,
        name: "Rejoice",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000233b,
        name: "Rest",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000232e,
        name: "Wave",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002337,
        name: "Welcome",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002328,
        name: "Beckon",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002341,
        name: "Call over",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000233c,
        name: "By my sword",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000232b,
        name: "Bow",
        ..Item::default_gesture()
    },
    Item {
        name: "Applause",
        id: 0x40002345,
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002343,
        name: "Quiet Resolve",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002338,
        name: "Joy",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000232c,
        name: "Jump for joy",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000232a,
        name: "Hurrah!",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002339,
        name: "My thanks!",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000233d,
        name: "Dignified bow",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000232d,
        name: "Duel bow",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002347,
        name: "Legion Etiquette",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002348,
        name: "Darkmoon Loyalty",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000234b,
        name: "Silent Ally",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002349,
        name: "Sleep",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002340,
        name: "Stretch out",
        ..Item::default_gesture()
    },
    Item {
        id: 0x40002342,
        name: "Collapse",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000234a,
        name: "Toast",
        ..Item::default_gesture()
    },
    Item {
        id: 0x4000233f,
        name: "Curl up",
        ..Item::default_gesture()
    },
];
