use {
    crate::offsets::{Offset, module_offsets::BasePointer},
    gubtool_core::{address::Address, attached::is_32},
};

pub const CHARACTER_MANAGER: Offset = Offset {
    vanilla: 0x18,
    scholar: 0x18,
};

pub const CAMERA_MANAGER: Offset = Offset {
    vanilla: 0x1c,
    scholar: 0x20,
};

pub const AI_MANAGER: Offset = Offset {
    vanilla: 0x20,
    scholar: 0x28,
};

pub const APP_RESOURCE_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x30,
};

pub const ENEMY_GENERATOR_MANAGER: Offset = Offset {
    vanilla: 0x2c,
    scholar: 0x40,
};

pub const TARGET_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x48,
};

pub const PAD_OWNERSHIP_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x50,
};

pub const EVENT_MANAGER: Offset = Offset {
    vanilla: 0x44,
    scholar: 0x70,
};

pub const FACE_GEN_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x80,
};

pub const RUMBLE_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x88,
};

pub const SIGN_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0x90,
};

pub const STATE_ACT_MANAGER: Offset = Offset {
    vanilla: 0,
    scholar: 0xa0,
};

pub const GAME_DATA_MANAGER: Offset = Offset {
    vanilla: 0x60,
    scholar: 0xa8,
};

pub const SAVE_LOAD_SYSTEM: Offset = Offset {
    vanilla: 0,
    scholar: 0xb8,
};

pub const APP_DLC_CONTENTS_INFO_ACCESSOR: Offset = Offset {
    vanilla: 0,
    scholar: 0xc8,
};

pub const PLAYER_CTRL: Offset = Offset {
    vanilla: 0x74,
    scholar: 0xd0,
};

pub const LOADING_FLAG: Offset = Offset {
    vanilla: 0xdfc,
    scholar: 0x24bc,
};

pub mod event_manager_offsets {
    use crate::offsets::Offset;

    pub const EVENT_FLAG_MANAGER: Offset = Offset {
        vanilla: 0x10,
        scholar: 0x20,
    };

    pub const EVENT_WARP_MANAGER: Offset = Offset {
        vanilla: 0x38,
        scholar: 0x70,
    };

    pub const EVENT_BONFIRE_MANAGER: Offset = Offset {
        vanilla: 0x2c,
        scholar: 0x58,
    };

    pub const RESPAWN_MAP: Offset = Offset {
        vanilla: 0xb4,
        scholar: 0x164,
    };

    pub const RESPAWN_BONFIRE: Offset = Offset {
        vanilla: 0xbc,
        scholar: 0x16c,
    };

    pub const EVENT_WINDOW_MANAGER: Offset = Offset {
        vanilla: 0x28,
        scholar: 0x50,
    };
    pub mod bonfire_manager_offsets {
        use crate::offsets::Offset;

        pub const ARRAY_BASE: Offset = Offset {
            vanilla: 0x10,
            scholar: 0x20,
        };

        pub const COUNT: Offset = Offset {
            vanilla: 0x14,
            scholar: 0x28,
        };
    }
}

pub const QUITOUT: Offset = Offset {
    vanilla: 0xdf1,
    scholar: 0x24b1,
};

pub const PX_WORLD: Offset = Offset {
    vanilla: 0x280,
    scholar: 0x660,
};

pub fn player_coords_chain() -> [u64; 7] {
    match is_32() {
        true => {
            [
                BasePointer::GameManagerImp.addr(),
                PX_WORLD.resolve(),
                0xc,
                0x168,
                0xc,
                0x4,
                0x120,
            ]
        }
        false => {
            [
                BasePointer::GameManagerImp.addr(),
                PX_WORLD.resolve(),
                0x18,
                0x1f8,
                0x18,
                0x8,
                0x1a0,
            ]
        }
    }
}

pub const DL_BACK_ALLOCATOR: Offset = Offset {
    vanilla: 0xcc4,
    scholar: 0x22e0,
};

pub mod dl_back_allocator_offsets {
    use crate::offsets::Offset;

    pub const UNK_FLAG: Offset = Offset {
        vanilla: 0x1a3,
        scholar: 0x30f,
    };

    pub const REF_COUNT: Offset = Offset {
        vanilla: 0x1b0,
        scholar: 0x31c,
    };
}

pub fn fe_item_select_menu_chain() -> [u64; 7] {
    match is_32() {
        true => {
            [
                BasePointer::GameManagerImp.addr(),
                DL_BACK_ALLOCATOR.resolve(),
                0x88,
                0x8,
                0x1c,
                0x18,
                0x18,
            ]
        }
        false => {
            [
                BasePointer::GameManagerImp.addr(),
                DL_BACK_ALLOCATOR.resolve(),
                0x110,
                0x10,
                0x38,
                0x30,
                0x30,
            ]
        }
    }
}

pub mod fe_item_select_menu_offsets {
    use crate::offsets::Offset;

    pub const OPEN_FLAG: Offset = Offset {
        vanilla: 0x12,
        scholar: 0x1e,
    };
}

pub mod player_ctrl_offsets {
    use crate::offsets::Offset;

    pub const PLAYER_OPERATOR: Offset = Offset {
        vanilla: 0xac,
        scholar: 0xe8,
    };
}

pub mod game_data_manager_offsets {
    use crate::offsets::Offset;

    pub const CLEARCOUNT_PTR: Offset = Offset {
        vanilla: 0x60,
        scholar: 0xc0,
    };

    pub mod clearcount_ptr_offsets {
        use crate::offsets::Offset;

        pub const CLEARCOUNT: Offset = Offset {
            vanilla: 0x68,
            scholar: 0x68,
        };
    }
}
