use {
    crate::offsets::module_offsets::BasePointer,
    gubtool_core::{address::Offset, attached::is_32, sys::PointerChain},
};

pub const CHARACTER_MANAGER: Offset = Offset {
    x86: 0x18,
    x64: 0x18,
};

pub const CAMERA_MANAGER: Offset = Offset {
    x86: 0x1c,
    x64: 0x20,
};

pub const AI_MANAGER: Offset = Offset {
    x86: 0x20,
    x64: 0x28,
};

pub const APP_RESOURCE_MANAGER: Offset = Offset {
    x86: 0,
    x64: 0x30,
};

pub const ENEMY_GENERATOR_MANAGER: Offset = Offset {
    x86: 0x2c,
    x64: 0x40,
};

pub const TARGET_MANAGER: Offset = Offset {
    x86: 0,
    x64: 0x48,
};

pub const PAD_OWNERSHIP_MANAGER: Offset = Offset {
    x86: 0,
    x64: 0x50,
};

pub const EVENT_MANAGER: Offset = Offset {
    x86: 0x44,
    x64: 0x70,
};

pub const FACE_GEN_MANAGER: Offset = Offset {
    x86: 0,
    x64: 0x80,
};

pub const RUMBLE_MANAGER: Offset = Offset {
    x86: 0,
    x64: 0x88,
};

pub const SIGN_MANAGER: Offset = Offset {
    x86: 0,
    x64: 0x90,
};

pub const STATE_ACT_MANAGER: Offset = Offset {
    x86: 0,
    x64: 0xa0,
};

pub const GAME_DATA_MANAGER: Offset = Offset {
    x86: 0x60,
    x64: 0xa8,
};

pub const SAVE_LOAD_SYSTEM: Offset = Offset {
    x86: 0,
    x64: 0xb8,
};

pub const APP_DLC_CONTENTS_INFO_ACCESSOR: Offset = Offset {
    x86: 0,
    x64: 0xc8,
};

pub const PLAYER_CTRL: Offset = Offset {
    x86: 0x74,
    x64: 0xd0,
};

pub const LOADING_FLAG: Offset = Offset {
    x86: 0xdfc,
    x64: 0x24bc,
};

pub mod event_manager_offsets {
    use gubtool_core::address::Offset;

    pub const EVENT_FLAG_MANAGER: Offset = Offset {
        x86: 0x10,
        x64: 0x20,
    };

    pub const EVENT_WARP_MANAGER: Offset = Offset {
        x86: 0x38,
        x64: 0x70,
    };

    pub const EVENT_BONFIRE_MANAGER: Offset = Offset {
        x86: 0x2c,
        x64: 0x58,
    };

    pub const RESPAWN_MAP: Offset = Offset {
        x86: 0xb4,
        x64: 0x164,
    };

    pub const RESPAWN_BONFIRE: Offset = Offset {
        x86: 0xbc,
        x64: 0x16c,
    };

    pub const EVENT_WINDOW_MANAGER: Offset = Offset {
        x86: 0x28,
        x64: 0x50,
    };
    pub mod bonfire_manager_offsets {
        use gubtool_core::address::Offset;

        pub const ARRAY_BASE: Offset = Offset {
            x86: 0x10,
            x64: 0x20,
        };

        pub const COUNT: Offset = Offset {
            x86: 0x14,
            x64: 0x28,
        };
    }
}

pub const QUITOUT: Offset = Offset {
    x86: 0xdf1,
    x64: 0x24b1,
};

pub const PX_WORLD: Offset = Offset {
    x86: 0x280,
    x64: 0x660,
};

pub fn player_coords_chain() -> PointerChain<BasePointer> {
    match is_32() {
        true => {
            PointerChain {
                start:      BasePointer::GameManagerImp,
                offsets:    &[0x280, 0xc, 0x168, 0xc, 0x4, 0x120],
                read_final: false,
            }
        }
        false => {
            PointerChain {
                start:      BasePointer::GameManagerImp,
                offsets:    &[0x660, 0x18, 0x1f8, 0x18, 0x8, 0x1a0],
                read_final: false,
            }
        }
    }
}

pub const DL_BACK_ALLOCATOR: Offset = Offset {
    x86: 0xcc4,
    x64: 0x22e0,
};

pub mod dl_back_allocator_offsets {
    use gubtool_core::address::Offset;

    pub const UNK_FLAG: Offset = Offset {
        x86: 0x1a3,
        x64: 0x30f,
    };

    pub const REF_COUNT: Offset = Offset {
        x86: 0x1b0,
        x64: 0x31c,
    };
}

pub fn fe_item_select_menu_chain() -> PointerChain<BasePointer> {
    match is_32() {
        true => {
            PointerChain {
                start:      BasePointer::GameManagerImp,
                offsets:    &[0xcc4, 0x88, 0x8, 0x1c, 0x18, 0x18],
                read_final: true,
            }
        }
        false => {
            PointerChain {
                start:      BasePointer::GameManagerImp,
                offsets:    &[0x22e0, 0x110, 0x10, 0x38, 0x30, 0x30],
                read_final: true,
            }
        }
    }
}

pub mod fe_item_select_menu_offsets {
    use gubtool_core::address::Offset;

    pub const OPEN_FLAG: Offset = Offset {
        x86: 0x12,
        x64: 0x1e,
    };
}

pub mod player_ctrl_offsets {
    use gubtool_core::address::Offset;

    pub const PLAYER_OPERATOR: Offset = Offset {
        x86: 0xac,
        x64: 0xe8,
    };
}

pub mod game_data_manager_offsets {
    use gubtool_core::address::Offset;

    pub const CLEARCOUNT_PTR: Offset = Offset {
        x86: 0x60,
        x64: 0xc0,
    };

    pub mod clearcount_ptr_offsets {
        use gubtool_core::address::Offset;

        pub const CLEARCOUNT: Offset = Offset {
            x86: 0x68,
            x64: 0x68,
        };
    }
}
