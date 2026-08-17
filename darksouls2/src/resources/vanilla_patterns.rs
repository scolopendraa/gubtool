use gubtool_core::aob_scanner::pattern::{AddressingMode, AobScan};

pub const GAME_MANAGER_IMP: AobScan = AobScan {
    name:        "GameManagerImp",
    pattern:     "0f 57 c0 f3 0f 2a c0 0f 5a c0 f2 0f 5e c8 52 8d ? ? 66 0f 5a c1 f3 0f 11 86 ? ? \
                  ? ?",
    scan_origin: 0xd914b,
    offset:      114,
    scan_mode:   AddressingMode::Direct32,
};

pub const KATANA_MAIN_APP: AobScan = AobScan {
    name:        "KatanaMainApp",
    pattern:     "8b 0d ? ? ? ? 8b 49 ? 89 55 ? f3 0f 11 45 ? 89 4d ?",
    scan_origin: 0x218194,
    offset:      2,
    scan_mode:   AddressingMode::Direct32,
};

pub const GIVE_SOULS: AobScan = AobScan {
    name:        "GiveSouls()",
    pattern:     "e8 ? ? ? ? c6 86 ? ? ? ? 01 a1 ? ? ? ? 8b 48 ? 6a 09 e8 ? ? ? ?",
    scan_origin: 0x2765a8,
    offset:      1,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const WARP: AobScan = AobScan {
    name:        "Warp()",
    pattern:     "8b d9 57 b9 10 00 00 00 8b fb f3 a5 8b 0d ? ? ? ? 8b 81 ? ? ? ? 85 c0 74 ? ba \
                  01 00 00 00",
    scan_origin: 0x20d5e0,
    offset:      -11,
    scan_mode:   AddressingMode::Absolute,
};

pub const ITEM_SPAWN: AobScan = AobScan {
    name:        "ItemSpawn()",
    pattern:     "83 f8 1f 0f 87 ? ? ? ? 53 56 8b cf e8 ? ? ? ?",
    scan_origin: 0x22ad3b,
    offset:      -27,
    scan_mode:   AddressingMode::Absolute,
};

pub const BUILD_ITEM_DIALOGUE: AobScan = AobScan {
    name:        "BuildItemDialogue()",
    pattern:     "89 b0 ? ? ? ? 89 88 ? ? ? ? 3b de 76 ? 57 8b 7d ?",
    scan_origin: 0x11f447,
    offset:      -23,
    scan_mode:   AddressingMode::Absolute,
};

pub const SHOW_ITEM_DIALOGUE: AobScan = AobScan {
    name:        "ShowItemDialogue()",
    pattern:     "83 78 ? 00 74 ? 85 c9 74 ? 85 d2 74 ? 6a 01 52 51",
    scan_origin: 0x21d010,
    offset:      37,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const CURRENT_ITEM_QUANTITY_CHECK: AobScan = AobScan {
    name:        "CurrentItemQuantityCheck()",
    pattern:     "8a 03 8b f1 8a c8 80 e1 03 57",
    scan_origin: 0x2330ff,
    offset:      30,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const SET_EVENT: AobScan = AobScan {
    name:        "SetEvent()",
    pattern:     "b8 59 17 b7 d1 f7 e6 57 8b f9 8b ca c1 e9 0d",
    scan_origin: 0x47faeb,
    offset:      -11,
    scan_mode:   AddressingMode::Absolute,
};

pub const MAP_ENTITY_FROM_MAP_ID_AND_OBJ_ID: AobScan = AobScan {
    name:        "MapEntityFromMapIdAndObjId()",
    pattern:     "24 0f c0 e1 04 0a c8 88 4e ? 8b 57 ? 33 c0 39 42 ?",
    scan_origin: 0x4292c4,
    offset:      43,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GET_STATE_ACT_COMPONENT: AobScan = AobScan {
    name:        "GetStateActComponent()",
    pattern:     "f3 0f 11 45 ? e8 ? ? ? ? 85 c0 74 ? f3 0f 10 05 ? ? ? ?",
    scan_origin: 0x24af9d,
    offset:      6,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const MAKE_SOUND: AobScan = AobScan {
    name:        "MakeSound()",
    pattern:     "E8 ? ? ? ? 84 C0 74 3F 8B 86",
    scan_origin: 0x1a173c,
    offset:      1,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const BONFIRE_REST: AobScan = AobScan {
    name:        "BonfireRest()",
    pattern:     "0f 57 c0 8b ce c7 46 ? 01 00 00 00 f3 0f 11 46 ? c7 46 ? 00 00 00 00 c7 46 ? 00 \
                  00 00 00 e8 ? ? ? ? b0 01 5e 5d c2 04 00",
    scan_origin: 0x208df0,
    offset:      -38,
    scan_mode:   AddressingMode::Absolute,
};

pub const BONFIRE_UNLOCK: AobScan = AobScan {
    name:        "BonfireUnlock()",
    pattern:     "55 8b ec 51 8b 45 ? 53 8b d9 89 45 fc 85 c0 74 ? 3d ff ff 00 00",
    scan_origin: 0x207370,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const OPEN_MENU: AobScan = AobScan {
    name:        "OpenMenu()",
    pattern:     "55 8b ec 53 8b 5d 08 83 7b 20 00 56 8b f1",
    scan_origin: 0x21cca0,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const MENU_CHR_STATE: AobScan = AobScan {
    name:        "MenuChrState()",
    pattern:     "55 8b ec 8b 45 ? 83 f8 14 7d ? 53 8b 5d ? 32 d2 56",
    scan_origin: 0x4fad30,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const LEVEL_UP: AobScan = AobScan {
    name:        "LevelUp()",
    pattern:     "55 8b ec 81 ec e0 00 00 00 a1 ? ? ? ? 33 c5 89 45 fc 53 56",
    scan_origin: 0x3b3fa0,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const LEVEL_LOOKUP: AobScan = AobScan {
    name:        "LevelLookup()",
    pattern:     "75 0B 50 E8 ? ? ? ? 83 C4 04 EB 02",
    scan_origin: 0x3b4a80,
    offset:      4,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const CHR_SET_ACTION: AobScan = AobScan {
    name:        "ChrSetAction()",
    pattern:     "55 8b ec 8b 45 08 83 89 50 02 00 00 01 89 81 5c 02 00 00",
    scan_origin: 0x438120,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const EZ_STATE_EXTERNAL_EVENT_CTOR: AobScan = AobScan {
    name:        "EzStateExternalEventCtor()",
    pattern:     "55 8b ec 8b c1 8b 4d 08 c7 00 ? ? ? ? c7 40 08 02 00 00 00 89 48 04 c7 80 fc 00 \
                  00 00 00 00 00 00 5d c2 04 00",
    scan_origin: 0x627cd0,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const EZ_STATE_EXECUTE_EVENT: AobScan = AobScan {
    name:        "EzStateExecuteEvent()",
    pattern:     "53 8b dc 83 ec 08 83 e4 f0 83 c4 04 55 8b 6b 04 89 6c 24 04 8b ec 81 ec 48 0d \
                  00 00",
    scan_origin: 0x46fb50,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const SET_SHARED_FLAG: AobScan = AobScan {
    name:        "SetSharedFlag",
    pattern:     "55 8b ec 8b 45 08 83 f8 07 77 ? 8a 55 0c",
    scan_origin: 0x7bda70,
    offset:      27,
    scan_mode:   AddressingMode::Absolute,
};

pub const LOCKED_TARGET_POINTER: AobScan = AobScan {
    name:        "LockedTargetPointer",
    pattern:     "89 b7 b8 00 00 00 eb 59",
    scan_origin: 0x4a54f1,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const CREDITS_SKIP: AobScan = AobScan {
    name:        "CreditsSkip",
    pattern:     "81 ec fc 01 00 00 53 8b d9 8b 43 ? 48 83 f8 06",
    scan_origin: 0x11be23,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const FASTER_MENU: AobScan = AobScan {
    name:        "FasterMenu",
    pattern:     "33 c5 89 45 ? 56 8b f1 83 7e ? 00 7e ? 57 e8 ? ? ? ? 8b f8 85 ff 74 ?",
    scan_origin: 0x1999be,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const EVENT_LOG: AobScan = AobScan {
    name:        "EventLog",
    pattern:     "b8 59 17 b7 d1 f7 e6 57 8b f9 8b ca c1 e9 0d 8b c1 69 c0 10 27 00 00",
    scan_origin: 0x47faeb,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const PLAYER_NO_DAMAGE: AobScan = AobScan {
    name:        "PlayerNoDamage",
    pattern:     "89 8e ? ? ? ? 8b 02 50 e8 ? ? ? ? 83 c4 04 84 c0 74 ? 83 be ? ? ? ? 00 7f ?",
    scan_origin: 0x1f5ad1,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const INFINITE_POISE: AobScan = AobScan {
    name:        "InfinitePoise",
    pattern:     "83 ? ? ? ? ? 00 0f 95 45 ? 83 ? ? ? ? ? 00 74 ?",
    scan_origin: 0x1ca4a3,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const INFINITE_STAMINA: AobScan = AobScan {
    name:        "InfiniteStamina",
    pattern:     "0f 5a e2 0f 5a c1 f2 0f 5c c4 66 0f 5a c0 0f 57 e4 f3 0f 5a e0 66 0f 2f e3 0f \
                  57 db",
    scan_origin: 0x35fcfe,
    offset:      -15,
    scan_mode:   AddressingMode::Absolute,
};

pub const INFINITE_CONSUMABLES: AobScan = AobScan {
    name:        "InfiniteConsumables",
    pattern:     "80 fa 02 75 17",
    scan_origin: 0x233ffd,
    offset:      -219,
    scan_mode:   AddressingMode::Absolute,
};

pub const INFINITE_DURABILITY: AobScan = AobScan {
    name:        "InfiniteDurability",
    pattern:     "8b 4d ? f3 0f 10 45 ? 51 53 8b ce f3 0f 11 47 ? e8 ? ? ? ? 5f 5e 5b 5d c2 ? ?",
    scan_origin: 0x37650e,
    offset:      12,
    scan_mode:   AddressingMode::Absolute,
};

pub const INFINITE_CASTS: AobScan = AobScan {
    name:        "InfiniteCasts",
    pattern:     "88 43 ? e8 ? ? ? ? 8b 45 ? 5f 5e 5b 8b e5 5d c2 ? ?",
    scan_origin: 0x22ee19,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_SOUL_GAIN: AobScan = AobScan {
    name:        "NoSoulGain",
    pattern:     "50 57 d9 6d ? e8 ? ? ? ? 83 c4 24 5f 5b 5e 8b e5 5d c3",
    scan_origin: 0x276f6f,
    offset:      5,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_HOLLOWING: AobScan = AobScan {
    name:        "NoHollowing",
    pattern:     "8a 10 88 91 a8 01 00 00 8b e5 5d c2 04 00",
    scan_origin: 0x3b37b5,
    offset:      2,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_SOUL_LOSS: AobScan = AobScan {
    name:        "NoSoulLoss",
    pattern:     "c6 46 ? 01 8b 93 ? ? ? ? 8b 82 ? ? ? ? 89 46 ? 8b 83 ? ? ? ? 80 b8 ? ? ? ? 00 \
                  75 ? c7 80 ? ? ? ? 00 00 00 00",
    scan_origin: 0x2c4d6f,
    offset:      34,
    scan_mode:   AddressingMode::Absolute,
};

pub const PLAYER_HIDDEN: AobScan = AobScan {
    name:        "PlayerHidden",
    pattern:     "8b 06 8b 49 1c 8b 11 8b 92 a4 00 00 00",
    scan_origin: 0x4496ab,
    offset:      -13,
    scan_mode:   AddressingMode::Absolute,
};

pub const PLAYER_SILENT: AobScan = AobScan {
    name:        "PlayerSilent",
    pattern:     "E8 ? ? ? ? 84 C0 74 3F 8B 86",
    scan_origin: 0x1a196e,
    offset:      -10,
    scan_mode:   AddressingMode::Absolute,
};

pub const MENU_TRANSITION: AobScan = AobScan {
    name:        "MenuTransition",
    pattern:     "0f 85 ? ? ? ? 8b 4e ? 8b 11 8b 02 6a 00",
    scan_origin: 0x187e9e,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_ROLL: AobScan = AobScan {
    name:        "NoRoll",
    pattern:     "e8 ? ? ? ? 32 c0 5d c2 04 00 b0 01 5d c2 04 00",
    scan_origin: 0x3b6a0c,
    offset:      11,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_BACKSTEP: AobScan = AobScan {
    name:        "NoBackstep",
    pattern:     "32 c0 5d c2 04 00 e8 ? ? ? ? 84 c0 0f 95 c0 5d c2 04 00",
    scan_origin: 0x3b6d21,
    offset:      13,
    scan_mode:   AddressingMode::Absolute,
};

pub const MAP_ID: AobScan = AobScan {
    name:        "MapId",
    pattern:     "8B 15 ? ? ? ? 8B F2 81 e6 00 00 00 ff 81 fe 00 00 00 32",
    scan_origin: 0x256b4b,
    offset:      2,
    scan_mode:   AddressingMode::Direct32,
};

pub const KERNEL32_CREATE_THREAD: AobScan = AobScan {
    name:        "KERNEL32.DLL::CreateThread",
    pattern:     "8b 4d 10 51 8b 55 0c 52 8b 45 18 50 57 ff 15 ? ? ? ?",
    scan_origin: 0x7d5a4f,
    offset:      15,
    scan_mode:   AddressingMode::Direct32,
};

pub const KERNEL32_CLOSE_HANDLE: AobScan = AobScan {
    name:        "KERNEL32.DLL::CloseHandle",
    pattern:     "e8 ? ? ? ? e8 ? ? ? ? 6a 01 ff 15 ? ? ? ? 8b 85 ? ? ? ? 3b c7 74 ? 50 ff 15 ? ? \
                  ? ?",
    scan_origin: 0x31f1e6,
    offset:      31,
    scan_mode:   AddressingMode::Direct32,
};

pub const KERNEL32_SLEEP: AobScan = AobScan {
    name:        "KERNEL32.DLL::Sleep",
    pattern:     "c7 86 ? ? ? ? 40 4b 4c 00 ff d0 8b b6 ? ? ? ? 85 f6 7e ? b8 ad 8b db 68",
    scan_origin: 0xc5b9e5,
    offset:      42,
    scan_mode:   AddressingMode::Direct32,
};

pub const KERNEL32_LOAD_LIBRARY_W: AobScan = AobScan {
    name:        "KERNEL32.DLL::LoadLibraryW",
    pattern:     "ff 15 ? ? ? ? 8b f8 85 ff 0f 84 ? ? ? ? 56 8b 35 ? ? ? ? 68 ? ? ? ? 57",
    scan_origin: 0xb274e5,
    offset:      2,
    scan_mode:   AddressingMode::Direct32,
};
