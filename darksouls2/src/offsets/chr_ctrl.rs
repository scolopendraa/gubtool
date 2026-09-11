use gubtool_core::address::Offset;

pub const CHR_ID: Offset = Offset {
    x86: 0x0,
    x64: 0x0,
};

pub const ROTATION: Offset = Offset {
    x86: 0x40,
    x64: 0x60,
};

pub const ORIENTATION: Offset = Offset {
    x86: 0x60,
    x64: 0x80,
};

pub const STATS_PTR: Offset = Offset {
    x86: 0x378,
    x64: 0x490,
};

pub const HANDLE: Offset = Offset {
    x86: 0,
    x64: 0x270,
};

pub const PARAMS_PTR: Offset = Offset {
    x86: 0x20,
    x64: 0x38,
};

pub const COORDS: Offset = Offset {
    x86: 0x80,
    x64: 0x90,
};

pub const HEALTH: Offset = Offset {
    x86: 0xfc,
    x64: 0x168,
};

pub const MIN_HEALTH: Offset = Offset {
    x86: 0x100,
    x64: 0x16c,
};

pub const MAX_HEALTH: Offset = Offset {
    x86: 0x104,
    x64: 0x170,
};

pub const POISE: Offset = Offset {
    x86: 0x1ac,
    x64: 0x218,
};

pub const MIN_POISE: Offset = Offset {
    x86: 0x1b0,
    x64: 0x21c,
};

pub const MAX_POISE: Offset = Offset {
    x86: 0x1b4,
    x64: 0x220,
};

pub const POSTURE: Offset = Offset {
    x86: 0x14c,
    x64: 0x1b8,
};

pub const MIN_POSTURE: Offset = Offset {
    x86: 0x150,
    x64: 0x1bc,
};

pub const MAX_POSTURE: Offset = Offset {
    x86: 0x154,
    x64: 0x1c0,
};

pub const CHR_SPEFFECT_CTRL: Offset = Offset {
    x86: 0x308,
    x64: 0x3e0,
};

pub mod stats_offsets {
    use gubtool_core::address::Offset;

    pub const STATS: Offset = Offset {
        x86: 0x4,
        x64: 0x8,
    };

    pub const SOUL_LEVEL: Offset = Offset {
        x86: 0xcc,
        x64: 0xd0,
    };

    pub const SOULS: Offset = Offset {
        x86: 0xe8,
        x64: 0xec,
    };

    pub const SOUL_MEMORY: Offset = Offset {
        x86: 0xf0,
        x64: 0xf4,
    };

    pub const COVENANT: Offset = Offset {
        x86: 0x1a9,
        x64: 0x1ad,
    };
}

pub const BOSS_OPERATOR: Offset = Offset {
    x86: 0xac,
    x64: 0xe8,
};

pub mod boss_operator_offsets {
    use gubtool_core::address::Offset;

    pub const CHR_AI_MANIPULATOR: Offset = Offset {
        x86: 0xc,
        x64: 0x18,
    };
}

pub mod chr_ai_manipulator_offsets {
    use gubtool_core::address::Offset;

    pub const CHR_AI: Offset = Offset {
        x86: 0x10,
        x64: 0x20,
    };
}
