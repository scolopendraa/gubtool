use super::Item;

impl Item {
    const fn default_cookbook() -> Self {
        Self {
            category: super::Categories::Cookbooks,
            ..Item::default()
        }
    }
}

pub static COOKBOOKS: [Item; 104] = [
    Item {
        id: 0x400024B8,
        name: "Ancient Dragon Apostle's Cookbook [1]",
        event_id: Some(68000),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024B9,
        name: "Ancient Dragon Apostle's Cookbook [2]",
        event_id: Some(68010),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024BB,
        name: "Ancient Dragon Apostle's Cookbook [3]",
        event_id: Some(68030),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024BA,
        name: "Ancient Dragon Apostle's Cookbook [4]",
        event_id: Some(68020),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8EC,
        name: "Ancient Dragon Knight's Cookbook [1]",
        event_id: Some(68740),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8F0,
        name: "Ancient Dragon Knight's Cookbook [2]",
        event_id: Some(68780),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8FD,
        name: "Antiquity Scholar's Cookbook [1]",
        event_id: Some(68910),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8F8,
        name: "Antiquity Scholar's Cookbook [2]",
        event_id: Some(68860),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002468,
        name: "Armorer's Cookbook [1]",
        event_id: Some(67200),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002469,
        name: "Armorer's Cookbook [2]",
        event_id: Some(67210),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002470,
        name: "Armorer's Cookbook [3]",
        event_id: Some(67280),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000246E,
        name: "Armorer's Cookbook [4]",
        event_id: Some(67260),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002473,
        name: "Armorer's Cookbook [5]",
        event_id: Some(67310),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002472,
        name: "Armorer's Cookbook [6]",
        event_id: Some(67300),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000246D,
        name: "Armorer's Cookbook [7]",
        event_id: Some(67250),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8F2,
        name: "Battlefield Priest's Cookbook [1]",
        event_id: Some(68800),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8F4,
        name: "Battlefield Priest's Cookbook [2]",
        event_id: Some(68820),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8FB,
        name: "Battlefield Priest's Cookbook [3]",
        event_id: Some(68890),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8FF,
        name: "Battlefield Priest's Cookbook [4]",
        event_id: Some(68930),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024CC,
        name: "Fevor's Cookbook [1]",
        event_id: Some(68200),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024CE,
        name: "Fevor's Cookbook [2]",
        event_id: Some(68220),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024CD,
        name: "Fevor's Cookbook [3]",
        event_id: Some(68210),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8FE,
        name: "Finger-Weaver's Cookbook [1]",
        event_id: Some(68920),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8DC,
        name: "Finger-Weaver's Cookbook [2]",
        event_id: Some(68580),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8EF,
        name: "Fire Knight's Cookbook [1]",
        event_id: Some(68770),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8FC,
        name: "Fire Knight's Cookbook [2]",
        event_id: Some(68900),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8D6,
        name: "Forager Brood Cookbook [1]",
        event_id: Some(68520),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8D7,
        name: "Forager Brood Cookbook [2]",
        event_id: Some(68530),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8D8,
        name: "Forager Brood Cookbook [3]",
        event_id: Some(68540),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8D9,
        name: "Forager Brood Cookbook [4]",
        event_id: Some(68550),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8DA,
        name: "Forager Brood Cookbook [5]",
        event_id: Some(68560),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8D5,
        name: "Forager Brood Cookbook [6]",
        event_id: Some(68510),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8F5,
        name: "Forager Brood Cookbook [7]",
        event_id: Some(68830),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024E0,
        name: "Frenzied's Cookbook [1]",
        event_id: Some(68400),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024E1,
        name: "Frenzied's Cookbook [2]",
        event_id: Some(68410),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000247D,
        name: "Glintstone Craftsman's Cookbook [1]",
        event_id: Some(67410),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002481,
        name: "Glintstone Craftsman's Cookbook [2]",
        event_id: Some(67450),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002484,
        name: "Glintstone Craftsman's Cookbook [3]",
        event_id: Some(67480),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000247C,
        name: "Glintstone Craftsman's Cookbook [4]",
        event_id: Some(67400),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000247E,
        name: "Glintstone Craftsman's Cookbook [5]",
        event_id: Some(67420),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002482,
        name: "Glintstone Craftsman's Cookbook [6]",
        event_id: Some(67460),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002483,
        name: "Glintstone Craftsman's Cookbook [7]",
        event_id: Some(67470),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002480,
        name: "Glintstone Craftsman's Cookbook [8]",
        event_id: Some(67440),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA900,
        name: "Grave Keeper's Cookbook [1]",
        event_id: Some(68940),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8F7,
        name: "Grave Keeper's Cookbook [2]",
        event_id: Some(68850),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8DD,
        name: "Greater Potentate's Cookbook [1]",
        event_id: Some(68590),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8E3,
        name: "Greater Potentate's Cookbook [10]",
        event_id: Some(68650),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8E4,
        name: "Greater Potentate's Cookbook [11]",
        event_id: Some(68660),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8E0,
        name: "Greater Potentate's Cookbook [12]",
        event_id: Some(68620),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8E8,
        name: "Greater Potentate's Cookbook [13]",
        event_id: Some(68700),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8E9,
        name: "Greater Potentate's Cookbook [14]",
        event_id: Some(68710),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8EB,
        name: "Greater Potentate's Cookbook [2]",
        event_id: Some(68730),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8E7,
        name: "Greater Potentate's Cookbook [3]",
        event_id: Some(68690),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8DE,
        name: "Greater Potentate's Cookbook [4]",
        event_id: Some(68600),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8DF,
        name: "Greater Potentate's Cookbook [5]",
        event_id: Some(68610),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8EA,
        name: "Greater Potentate's Cookbook [6]",
        event_id: Some(68720),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8E1,
        name: "Greater Potentate's Cookbook [7]",
        event_id: Some(68630),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8E6,
        name: "Greater Potentate's Cookbook [8]",
        event_id: Some(68680),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8E2,
        name: "Greater Potentate's Cookbook [9]",
        event_id: Some(68640),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8F3,
        name: "Igon's Cookbook [1]",
        event_id: Some(68810),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8DB,
        name: "Igon's Cookbook [2]",
        event_id: Some(68570),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8F1,
        name: "Loyal Knight's Cookbook",
        event_id: Some(68790),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8ED,
        name: "Mad Craftsman's Cookbook [1]",
        event_id: Some(68750),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8E5,
        name: "Mad Craftsman's Cookbook [2]",
        event_id: Some(68670),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8FA,
        name: "Mad Craftsman's Cookbook [3]",
        event_id: Some(68880),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002491,
        name: "Missionary's Cookbook [1]",
        event_id: Some(67610),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002490,
        name: "Missionary's Cookbook [2]",
        event_id: Some(67600),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002495,
        name: "Missionary's Cookbook [3]",
        event_id: Some(67650),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002494,
        name: "Missionary's Cookbook [4]",
        event_id: Some(67640),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002493,
        name: "Missionary's Cookbook [5]",
        event_id: Some(67630),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002461,
        name: "Missionary's Cookbook [6]",
        event_id: Some(67130),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024CF,
        name: "Missionary's Cookbook [7]",
        event_id: Some(68230),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002454,
        name: "Nomadic Warrior's Cookbook [1]",
        event_id: Some(67000),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002457,
        name: "Nomadic Warrior's Cookbook [10]",
        event_id: Some(67030),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000246A,
        name: "Nomadic Warrior's Cookbook [11]",
        event_id: Some(67220),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245A,
        name: "Nomadic Warrior's Cookbook [12]",
        event_id: Some(67060),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245C,
        name: "Nomadic Warrior's Cookbook [13]",
        event_id: Some(67080),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024AB,
        name: "Nomadic Warrior's Cookbook [14]",
        event_id: Some(67870),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024AE,
        name: "Nomadic Warrior's Cookbook [15]",
        event_id: Some(67900),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002471,
        name: "Nomadic Warrior's Cookbook [16]",
        event_id: Some(67290),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245E,
        name: "Nomadic Warrior's Cookbook [17]",
        event_id: Some(67100),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000246F,
        name: "Nomadic Warrior's Cookbook [18]",
        event_id: Some(67270),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245B,
        name: "Nomadic Warrior's Cookbook [19]",
        event_id: Some(67070),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245F,
        name: "Nomadic Warrior's Cookbook [2]",
        event_id: Some(67110),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000246B,
        name: "Nomadic Warrior's Cookbook [20]",
        event_id: Some(67230),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002460,
        name: "Nomadic Warrior's Cookbook [21]",
        event_id: Some(67120),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024AD,
        name: "Nomadic Warrior's Cookbook [22]",
        event_id: Some(67890),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245D,
        name: "Nomadic Warrior's Cookbook [23]",
        event_id: Some(67090),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024AF,
        name: "Nomadic Warrior's Cookbook [24]",
        event_id: Some(67910),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002455,
        name: "Nomadic Warrior's Cookbook [3]",
        event_id: Some(67010),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024A4,
        name: "Nomadic Warrior's Cookbook [4]",
        event_id: Some(67800),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024A7,
        name: "Nomadic Warrior's Cookbook [5]",
        event_id: Some(67830),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002456,
        name: "Nomadic Warrior's Cookbook [6]",
        event_id: Some(67020),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x40002459,
        name: "Nomadic Warrior's Cookbook [7]",
        event_id: Some(67050),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024AC,
        name: "Nomadic Warrior's Cookbook [8]",
        event_id: Some(67880),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000247F,
        name: "Nomadic Warrior's Cookbook [9]",
        event_id: Some(67430),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024A8,
        name: "Perfumer's Cookbook [1]",
        event_id: Some(67840),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024A9,
        name: "Perfumer's Cookbook [2]",
        event_id: Some(67850),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024AA,
        name: "Perfumer's Cookbook [3]",
        event_id: Some(67860),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024B0,
        name: "Perfumer's Cookbook [4]",
        event_id: Some(67920),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8EE,
        name: "St. Trina Disciple's Cookbook [1]",
        event_id: Some(68760),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA901,
        name: "St. Trina Disciple's Cookbook [2]",
        event_id: Some(68950),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8F6,
        name: "St. Trina Disciple's Cookbook [3]",
        event_id: Some(68840),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401EA8F9,
        name: "Tibia's Cookbook",
        event_id: Some(68870),
        dlc: true,
        ..Item::default_cookbook()
    },
];
