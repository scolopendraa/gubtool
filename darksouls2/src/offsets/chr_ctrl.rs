use crate::offsets::Offset;

pub const CHR_ID: Offset = Offset {
    vanilla: 0x0,
    scholar: 0x0,
};

pub const ROTATION: Offset = Offset {
    vanilla: 0x40,
    scholar: 0x60,
};

pub const ORIENTATION: Offset = Offset {
    vanilla: 0x60,
    scholar: 0x80,
};

pub const STATS_PTR: Offset = Offset {
    vanilla: 0x378,
    scholar: 0x490,
};

pub const HANDLE: Offset = Offset {
    vanilla: 0,
    scholar: 0x270,
};

pub const PARAMS_PTR: Offset = Offset {
    vanilla: 0x20,
    scholar: 0x38,
};

pub const COORDS: Offset = Offset {
    vanilla: 0x80,
    scholar: 0x90,
};

pub const HEALTH: Offset = Offset {
    vanilla: 0xfc,
    scholar: 0x168,
};

pub const MIN_HEALTH: Offset = Offset {
    vanilla: 0x100,
    scholar: 0x16c,
};

pub const MAX_HEALTH: Offset = Offset {
    vanilla: 0x104,
    scholar: 0x170,
};

pub const POISE: Offset = Offset {
    vanilla: 0x1ac,
    scholar: 0x218,
};

pub const MIN_POISE: Offset = Offset {
    vanilla: 0x1b0,
    scholar: 0x21c,
};

pub const MAX_POISE: Offset = Offset {
    vanilla: 0x1b4,
    scholar: 0x220,
};

pub const POSTURE: Offset = Offset {
    vanilla: 0x14c,
    scholar: 0x1b8,
};

pub const MIN_POSTURE: Offset = Offset {
    vanilla: 0x150,
    scholar: 0x1bc,
};

pub const MAX_POSTURE: Offset = Offset {
    vanilla: 0x154,
    scholar: 0x1c0,
};

pub const CHR_SPEFFECT_CTRL: Offset = Offset {
    vanilla: 0x308,
    scholar: 0x3e0,
};

pub mod stats_offsets {
    use crate::offsets::Offset;

    pub const STATS: Offset = Offset {
        vanilla: 0x4,
        scholar: 0x8,
    };

    pub const SOUL_LEVEL: Offset = Offset {
        vanilla: 0xcc,
        scholar: 0xd0,
    };

    pub const SOULS: Offset = Offset {
        vanilla: 0xe8,
        scholar: 0xec,
    };

    pub const SOUL_MEMORY: Offset = Offset {
        vanilla: 0xf0,
        scholar: 0xf4,
    };

    pub const COVENANT: Offset = Offset {
        vanilla: 0x1a9,
        scholar: 0x1ad,
    };
}

pub const BOSS_OPERATOR: Offset = Offset {
    vanilla: 0xac,
    scholar: 0xe8,
};

pub mod boss_operator_offsets {
    use crate::offsets::Offset;
    pub const CHR_AI_MANIPULATOR: Offset = Offset {
        vanilla: 0xc,
        scholar: 0x18,
    };
}

pub mod chr_ai_manipulator_offsets {
    use crate::offsets::Offset;
    pub const CHR_AI: Offset = Offset {
        vanilla: 0x10,
        scholar: 0x20,
    };
}
