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
        id: 0x400024b8,
        name: "Ancient Dragon Apostle's Cookbook [1]",
        event_id: Some(68000),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024b9,
        name: "Ancient Dragon Apostle's Cookbook [2]",
        event_id: Some(68010),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024bb,
        name: "Ancient Dragon Apostle's Cookbook [3]",
        event_id: Some(68030),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024ba,
        name: "Ancient Dragon Apostle's Cookbook [4]",
        event_id: Some(68020),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8ec,
        name: "Ancient Dragon Knight's Cookbook [1]",
        event_id: Some(68740),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8f0,
        name: "Ancient Dragon Knight's Cookbook [2]",
        event_id: Some(68780),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8fd,
        name: "Antiquity Scholar's Cookbook [1]",
        event_id: Some(68910),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8f8,
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
        id: 0x4000246e,
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
        id: 0x4000246d,
        name: "Armorer's Cookbook [7]",
        event_id: Some(67250),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8f2,
        name: "Battlefield Priest's Cookbook [1]",
        event_id: Some(68800),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8f4,
        name: "Battlefield Priest's Cookbook [2]",
        event_id: Some(68820),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8fb,
        name: "Battlefield Priest's Cookbook [3]",
        event_id: Some(68890),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8ff,
        name: "Battlefield Priest's Cookbook [4]",
        event_id: Some(68930),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024cc,
        name: "Fevor's Cookbook [1]",
        event_id: Some(68200),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024ce,
        name: "Fevor's Cookbook [2]",
        event_id: Some(68220),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024cd,
        name: "Fevor's Cookbook [3]",
        event_id: Some(68210),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8fe,
        name: "Finger-Weaver's Cookbook [1]",
        event_id: Some(68920),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8dc,
        name: "Finger-Weaver's Cookbook [2]",
        event_id: Some(68580),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8ef,
        name: "Fire Knight's Cookbook [1]",
        event_id: Some(68770),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8fc,
        name: "Fire Knight's Cookbook [2]",
        event_id: Some(68900),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8d6,
        name: "Forager Brood Cookbook [1]",
        event_id: Some(68520),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8d7,
        name: "Forager Brood Cookbook [2]",
        event_id: Some(68530),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8d8,
        name: "Forager Brood Cookbook [3]",
        event_id: Some(68540),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8d9,
        name: "Forager Brood Cookbook [4]",
        event_id: Some(68550),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8da,
        name: "Forager Brood Cookbook [5]",
        event_id: Some(68560),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8d5,
        name: "Forager Brood Cookbook [6]",
        event_id: Some(68510),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8f5,
        name: "Forager Brood Cookbook [7]",
        event_id: Some(68830),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024e0,
        name: "Frenzied's Cookbook [1]",
        event_id: Some(68400),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024e1,
        name: "Frenzied's Cookbook [2]",
        event_id: Some(68410),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000247d,
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
        id: 0x4000247c,
        name: "Glintstone Craftsman's Cookbook [4]",
        event_id: Some(67400),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000247e,
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
        id: 0x401ea900,
        name: "Grave Keeper's Cookbook [1]",
        event_id: Some(68940),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8f7,
        name: "Grave Keeper's Cookbook [2]",
        event_id: Some(68850),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8dd,
        name: "Greater Potentate's Cookbook [1]",
        event_id: Some(68590),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8e3,
        name: "Greater Potentate's Cookbook [10]",
        event_id: Some(68650),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8e4,
        name: "Greater Potentate's Cookbook [11]",
        event_id: Some(68660),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8e0,
        name: "Greater Potentate's Cookbook [12]",
        event_id: Some(68620),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8e8,
        name: "Greater Potentate's Cookbook [13]",
        event_id: Some(68700),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8e9,
        name: "Greater Potentate's Cookbook [14]",
        event_id: Some(68710),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8eb,
        name: "Greater Potentate's Cookbook [2]",
        event_id: Some(68730),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8e7,
        name: "Greater Potentate's Cookbook [3]",
        event_id: Some(68690),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8de,
        name: "Greater Potentate's Cookbook [4]",
        event_id: Some(68600),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8df,
        name: "Greater Potentate's Cookbook [5]",
        event_id: Some(68610),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8ea,
        name: "Greater Potentate's Cookbook [6]",
        event_id: Some(68720),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8e1,
        name: "Greater Potentate's Cookbook [7]",
        event_id: Some(68630),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8e6,
        name: "Greater Potentate's Cookbook [8]",
        event_id: Some(68680),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8e2,
        name: "Greater Potentate's Cookbook [9]",
        event_id: Some(68640),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8f3,
        name: "Igon's Cookbook [1]",
        event_id: Some(68810),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8db,
        name: "Igon's Cookbook [2]",
        event_id: Some(68570),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8f1,
        name: "Loyal Knight's Cookbook",
        event_id: Some(68790),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8ed,
        name: "Mad Craftsman's Cookbook [1]",
        event_id: Some(68750),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8e5,
        name: "Mad Craftsman's Cookbook [2]",
        event_id: Some(68670),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8fa,
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
        id: 0x400024cf,
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
        id: 0x4000246a,
        name: "Nomadic Warrior's Cookbook [11]",
        event_id: Some(67220),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245a,
        name: "Nomadic Warrior's Cookbook [12]",
        event_id: Some(67060),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245c,
        name: "Nomadic Warrior's Cookbook [13]",
        event_id: Some(67080),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024ab,
        name: "Nomadic Warrior's Cookbook [14]",
        event_id: Some(67870),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024ae,
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
        id: 0x4000245e,
        name: "Nomadic Warrior's Cookbook [17]",
        event_id: Some(67100),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000246f,
        name: "Nomadic Warrior's Cookbook [18]",
        event_id: Some(67270),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245b,
        name: "Nomadic Warrior's Cookbook [19]",
        event_id: Some(67070),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245f,
        name: "Nomadic Warrior's Cookbook [2]",
        event_id: Some(67110),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000246b,
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
        id: 0x400024ad,
        name: "Nomadic Warrior's Cookbook [22]",
        event_id: Some(67890),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000245d,
        name: "Nomadic Warrior's Cookbook [23]",
        event_id: Some(67090),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024af,
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
        id: 0x400024a4,
        name: "Nomadic Warrior's Cookbook [4]",
        event_id: Some(67800),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024a7,
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
        id: 0x400024ac,
        name: "Nomadic Warrior's Cookbook [8]",
        event_id: Some(67880),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x4000247f,
        name: "Nomadic Warrior's Cookbook [9]",
        event_id: Some(67430),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024a8,
        name: "Perfumer's Cookbook [1]",
        event_id: Some(67840),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024a9,
        name: "Perfumer's Cookbook [2]",
        event_id: Some(67850),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024aa,
        name: "Perfumer's Cookbook [3]",
        event_id: Some(67860),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x400024b0,
        name: "Perfumer's Cookbook [4]",
        event_id: Some(67920),
        dlc: false,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8ee,
        name: "St. Trina Disciple's Cookbook [1]",
        event_id: Some(68760),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea901,
        name: "St. Trina Disciple's Cookbook [2]",
        event_id: Some(68950),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8f6,
        name: "St. Trina Disciple's Cookbook [3]",
        event_id: Some(68840),
        dlc: true,
        ..Item::default_cookbook()
    },
    Item {
        id: 0x401ea8f9,
        name: "Tibia's Cookbook",
        event_id: Some(68870),
        dlc: true,
        ..Item::default_cookbook()
    },
];
