use super::Item;

impl Item {
    const fn default_gesture() -> Self {
        Self {
            category: super::Categories::Gestures,
            stack_size: 1,
            ..Item::default()
        }
    }
}

pub static GESTURES: &[Item; 20] = &[
    Item {
        id: 63000000,
        name: "Point Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63001000,
        name: "I won't bite Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63003000,
        name: "Bow Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63004000,
        name: "Welcome Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63005000,
        name: "Duel bow Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63006000,
        name: "Wave Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63007000,
        name: "Pumped up Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63008000,
        name: "Joy Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63009000,
        name: "Warcry Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63010000,
        name: "Warmup Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63011000,
        name: "Hurrah! Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63012000,
        name: "Righty-ho! Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63013000,
        name: "No way Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63014000,
        name: "This one's me Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63015000,
        name: "Have mercy! Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63016000,
        name: "Prostration Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63017000,
        name: "Decapitate Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63018000,
        name: "Fist pump Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63019000,
        name: "Mock Gesture",
        ..Item::default_gesture()
    },
    Item {
        id: 63021000,
        name: "Praise the Sun Gesture",
        ..Item::default_gesture()
    },
];