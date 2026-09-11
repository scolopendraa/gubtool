use gubtool_core::aob_scanner::pattern::{AddressingMode, AobScan};

pub const KERNEL32_CREATE_THREAD: AobScan = AobScan {
    name:        "KERNEL32.DLL::CreateThread",
    pattern:     "48 89 5c 24 08 57 48 83 ec 30 48 8b da 48 63 54 24 68 48 8d 44 24 68 48 8b f9 \
                  48 89 44 24 28 33 c9 c7 44 24 20 00 00 00 00 ff 15 ? ? ? ? 48 89 07 48 85 c0 75 \
                  14 c7 03 01 00 00 00 48 8b c3 48 8b 5c 24 40 48 83 c4 30 5f c3",
    scan_origin: 0x12f0740,
    offset:      43,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const KERNEL32_CLOSE_HANDLE: AobScan = AobScan {
    name:        "KERNEL32.DLL::CloseHandle",
    pattern:     "ff 15 ? ? ? ? 48 8b cf 48 8b d8 ff 15 ? ? ? ? 85 db 48 8b 5c 24 30 0f 95 c0 48 \
                  83 c4 20 5f c3",
    scan_origin: 0x17e80d6,
    offset:      14,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const KERNEL32_LOAD_LIBRARY_W: AobScan = AobScan {
    name:        "KERNEL32.DLL::LoadLibraryW",
    pattern:     "ff 15 ? ? ? ? 48 89 83 78 02 00 00 48 85 c0 0f 84 a1 01 00 00 33 c9 ff 15 ? ? ? \
                  ? 48 8b 8b 78 02 00 00",
    scan_origin: 0x17dca06,
    offset:      2,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const WORLD_CHR_MAN_IMP: AobScan = AobScan {
    name:        "WorldChrManImp",
    pattern:     "0f c6 c1 66 f3 0f 58 c8 0f c6 c0 55 f3 0f 58 c8 0f c6 c9 00 0f 51 c1 0f 2f f0 \
                  76 ? 48 8b 0d ? ? ? ?",
    scan_origin: 0x17dca06,
    offset:      31,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const WORLD_OBJ_MAN_IMPL: AobScan = AobScan {
    name:        "WorldObjManImpl",
    pattern:     "40 53 48 83 ec 20 8b 51 30 48 8b d9 83 fa ff 74 42 48 8b 0d ? ? ? ?",
    scan_origin: 0x17dca06,
    offset:      20,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GAME_MAN: AobScan = AobScan {
    name:        "GameMan",
    pattern:     "48 8b 05 ? ? ? ? 0f 28 80 ? ? ? ? 48 8b c1 66 0f 7f 01 c3",
    scan_origin: 0x626ff0,
    offset:      3,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GAME_DATA_MAN: AobScan = AobScan {
    name:        "GameDataMan",
    pattern:     "48 8d 4c 24 40 e8 ? ? ? ? 8b cb e8 ? ? ? ? 44 8b 40 10 48 8b 05 ? ? ? ? 48 8b \
                  48 10 0f b7 51 10 48 8b ce e8 ? ? ? ?",
    scan_origin: 0x46533d,
    offset:      24,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const MENU_MAN: AobScan = AobScan {
    name:        "MenuMan",
    pattern:     "81 f9 f3 01 00 00 77 ? 48 8b 05 ? ? ? ? 48 63 c9 89 54 88 34",
    scan_origin: 0x765220,
    offset:      11,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const MAP_ITEM_MAN: AobScan = AobScan {
    name:        "MapItemMan",
    pattern:     "8b 94 24 c8 00 00 00 48 8b 0d ? ? ? ? e8 ? ? ? ? ba aa 0f 00 00 41 b8 01 00 00 \
                  00",
    scan_origin: 0x4609c3,
    offset:      10,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const EVENT_FLAG_MAN: AobScan = AobScan {
    name:        "EventFlagMan",
    pattern:     "44 39 6b 08 0f 8c ? ? ? ? 48 8b 0d ? ? ? ? 48 85 c9 75 ? 4c 8d 0d ? ? ? ? 4c 8d \
                  05 ? ? ? ? ba b1 00 00 00",
    scan_origin: 0x6027e5,
    offset:      13,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const WORLD_AI_MANAGER: AobScan = AobScan {
    name:        "WorldAiManager",
    pattern:     "48 89 7c 24 38 48 83 c1 18 e8 ? ? ? ? 48 8b 0d ? ? ? ? 8b 1b 8b 38 48 85 c9 75 ?",
    scan_origin: 0x3ab1ae,
    offset:      17,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const DEBUG_FLAGS: AobScan = AobScan {
    name:        "DebugFlags",
    pattern:     "80 3d ? ? ? ? 00 74 ? c7 83 b0 01 00 00 00 00 00 00",
    scan_origin: 0x9dba6b,
    offset:      2,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 5,
    },
};

pub const SET_EVENT_FLAG: AobScan = AobScan {
    name:        "SetEventFlag()",
    pattern:     "40 55 57 41 54 41 57 48 83 ec 58 80 b9 ? ? ? ? 00 45 0f b6 f9 45 0f b6 e0 8b ea \
                  48 8b f9 0f 84 ? ? ? ? 45 0f b6 c1",
    scan_origin: 0x4c5e20,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const CS_DLC: AobScan = AobScan {
    name:        "CSDlc",
    pattern:     "c6 45 e8 01 48 8b 1d ? ? ? ? 48 85 db 75 ? 4c 8d 0d ? ? ? ? 4c 8d 05 ? ? ? ? ba \
                  b1 00 00 00",
    scan_origin: 0x619a21,
    offset:      7,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const BONFIRE_WARP: AobScan = AobScan {
    name:        "BonfireWarp()",
    pattern:     "44 8b c5 8b d7 48 8b 6c 24 30 48 8b 7c 24 38 48 8b 5c 24 40 48 83 c4 20 5e e9 ? \
                  ? ? ?",
    scan_origin: 0x486a20,
    offset:      26,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const WARP_COORDS: AobScan = AobScan {
    name:        "WarpCoords",
    pattern:     "66 0f 7f 80 40 0a 00 00",
    scan_origin: 0x62873a,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const WARP_ANGLE: AobScan = AobScan {
    name:        "WarpAngle",
    pattern:     "66 0f 7f 80 50 0a 00 00",
    scan_origin: 0x62871a,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const DISABLE_LOGOS: AobScan = AobScan {
    name:        "DisableLogos",
    pattern:     "e8 ? ? ? ? 90 4d 8b c7 49 8b d4 48 8b c8 e8 ? ? ? ? c7 44 24 50 01 00 00 00",
    scan_origin: 0xbf43ef,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const GAME_SPEED: AobScan = AobScan {
    name:        "GameSpeed",
    pattern:     "c7 83 60 09 00 00 00 00 80 3f",
    scan_origin: 0x9a3e72,
    offset:      6,
    scan_mode:   AddressingMode::Absolute,
};

pub const SPRJ_FLIPPER_IMP: AobScan = AobScan {
    name:        "SprjFlipperImp",
    pattern:     "f6 83 ? ? ? ? 02 75 ? 48 8b 0d ? ? ? ? 48 85 c9 75 ? 4c 8d 0d ? ? ? ? 4c 8d 05 \
                  ? ? ? ?",
    scan_origin: 0xcd6065,
    offset:      12,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GET_ITEM_QUANTITY: AobScan = AobScan {
    name:        "GetItemQuantity()",
    pattern:     "ff c5 8b df 74 ? 44 8b c3 49 8b d6 48 8d 4c 24 28",
    scan_origin: 0x58c04a,
    offset:      -0x2a,
    scan_mode:   AddressingMode::Absolute,
};

pub const ITEM_SPAWN: AobScan = AobScan {
    name:        "ItemSpawn()",
    pattern:     "4c 8d 45 f0 48 8d 55 a4 48 8b 4c 24 50 e8 ? ? ? ?",
    scan_origin: 0x7c131b,
    offset:      14,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const CAMERA_RESET: AobScan = AobScan {
    name:        "CameraReset",
    pattern:     "80 be 6d 29 00 00 00 74",
    scan_origin: 0x86623f,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const CAMERA_ROTATION: AobScan = AobScan {
    name:        "CameraRotation",
    pattern:     "66 0f 7f a6 40 01 00 00 80 be b3 02 00 00 00",
    scan_origin: 0x516fb4,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const EZ_STATE_EXTERNAL_EVENT_CTOR: AobScan = AobScan {
    name:        "EzStateExternalEventCtor()",
    pattern:     "48 8d 05 ? ? ? ? 48 89 01 c7 41 10 02 00 00 00 89 51 08 48 c7 81 00 02 00 00 00 \
                  00 00 00",
    scan_origin: 0x199fcf0,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const EXECUTE_TALK_COMMAND: AobScan = AobScan {
    name:        "ExecuteTalkCommand()",
    pattern:     "48 c7 45 88 fe ff ff ff 48 89 58 18 0f 29 70 b8 48 8b 05 ? ? ? ?",
    scan_origin: 0xf035da,
    offset:      -0x1a,
    scan_mode:   AddressingMode::Absolute,
};

pub const FULL_SHOP: AobScan = AobScan {
    name:        "FullShop",
    pattern:     "74 14 48 8d 54 24 38 48 8d 4d 40 e8 ? ? ? ? 90",
    scan_origin: 0xc1d984,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const LOCKED_TARGET_POINTER: AobScan = AobScan {
    name:        "LockedTargetPointer",
    pattern:     "0f 28 00 0f 29 43 50 0f 28 48 10 0f 29 4b 60 e8 ? ? ? ?",
    scan_origin: 0x86306a,
    offset:      16,
    scan_mode:   AddressingMode::Relative { bytes_to_next_instr: 0x1e },
};