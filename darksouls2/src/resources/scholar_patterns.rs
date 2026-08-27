use gubtool_core::aob_scanner::pattern::{AddressingMode, AobScan};

pub const GAME_MANAGER_IMP: AobScan = AobScan {
    name:        "GameManagerImp",
    pattern:     "74 ? 48 8b 1d ? ? ? ? 48 85 db 74 ? 48 8b 9b 60 06 00 00",
    scan_origin: 0x498a34,
    offset:      5,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const KATANA_MAIN_APP: AobScan = AobScan {
    name:        "KatanaMainApp",
    pattern:     "0f 29 74 24 ? 89 83 90 00 00 00 48 8b 05 ? ? ? ? 48 8b 48 40 e8 ? ? ? ?",
    scan_origin: 0x17ea,
    offset:      14,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GIVE_SOULS: AobScan = AobScan {
    name:        "GiveSouls()",
    pattern:     "48 8b 01 48 85 c0 74 ? 48 8b 80 b8 00 00 00 48 85 c0 74 ? 49 b8 00 00 00 00 00 \
                  00 00 08",
    scan_origin: 0x38ab44,
    offset:      -4,
    scan_mode:   AddressingMode::Absolute,
};

pub const WARP: AobScan = AobScan {
    name:        "Warp()",
    pattern:     "40 53 48 83 ec 60 8b 02 48 8b d9 89 01 8b 42 04 89 41 04 8b 42 08 89 41 08",
    scan_origin: 0x184830,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const ITEM_SPAWN: AobScan = AobScan {
    name:        "ItemSpawn()",
    pattern:     "48 89 5c 24 18 56 57 41 56 48 83 ec 30 45 8b f1 41 8b f0 48 8b da 48 8b f9 e8 ? \
                  ? ? ?",
    scan_origin: 0x1a7470,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const BUILD_ITEM_DIALOGUE: AobScan = AobScan {
    name:        "BuildItemDialogue()",
    pattern:     "e8 ? ? ? ? 48 8d 54 24 ? 48 8b cf e8 ? ? ? ? b0 01",
    scan_origin: 0x198cef,
    offset:      1,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const SHOW_ITEM_DIALOGUE: AobScan = AobScan {
    name:        "ShowItemDialogue()",
    pattern:     "e8 ? ? ? ? 48 8d 54 24 ? 48 8b cf e8 ? ? ? ? b0 01",
    scan_origin: 0x198cef,
    offset:      14,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const CURRENT_ITEM_QUANTITY_CHECK: AobScan = AobScan {
    name:        "CurrentItemQuantityCheck()",
    pattern:     "f6 c1 01 74 ? 4c 8d 45 f4 48 8d 55 40 48 8b cf e8 ? ? ? ?",
    scan_origin: 0x1b5369,
    offset:      17,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const SET_EVENT: AobScan = AobScan {
    name:        "SetEvent()",
    pattern:     "57 48 83 ec 20 8b fa 45 0f b6 d8 b8 59 17 b7 d1",
    scan_origin: 0x4750b5,
    offset:      -5,
    scan_mode:   AddressingMode::Absolute,
};

pub const MAP_ENTITY_FROM_MAP_ID_AND_OBJ_ID: AobScan = AobScan {
    name:        "MapEntityFromAreaIdAndObjId()",
    pattern:     "40 57 48 83 ec 20 48 8b 05 ? ? ? ? 8b fa 4c 8b 40 38 4d 85 c0 75 ?",
    scan_origin: 0x3c1b90,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const GET_STATE_ACT_COMPONENT: AobScan = AobScan {
    name:        "GetStateActComponent()",
    pattern:     "40 53 48 83 ec 20 8b da 48 8b d1 48 81 c1 b8 00 00 00 e8 ? ? ? ? 0f b6 d3",
    scan_origin: 0x1c65c0,
    offset:      19,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const MAKE_SOUND: AobScan = AobScan {
    name:        "MakeSound()",
    pattern:     "57 48 83 ec 20 41 0f b6 f9 49 8b f0 48 8b d9 48 85 d2 74 ? 48 83 ba ? ? ? ? 00 \
                  74 ?",
    scan_origin: 0x10e19f,
    offset:      -15,
    scan_mode:   AddressingMode::Absolute,
};

pub const BONFIRE_REST: AobScan = AobScan {
    name:        "BonfireRest()",
    pattern:     "40 53 48 83 ec 20 83 79 38 00 48 8b d9 75 ? 85 d2 74 ? 48 83 79 30 00 75 ? e8 ? \
                  ? ? ?",
    scan_origin: 0x17aa30,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const BONFIRE_UNLOCK: AobScan = AobScan {
    name:        "BonfireUnlock()",
    pattern:     "41 0f b6 f8 48 8b d9 e8 ? ? ? ? 48 85 c0 74 ?",
    scan_origin: 0x17e32a,
    offset:      -10,
    scan_mode:   AddressingMode::Absolute,
};

pub const OPEN_MENU: AobScan = AobScan {
    name:        "OpenMenu()",
    pattern:     "48 89 5c 24 10 48 89 6c 24 18 56 48 83 ec 20 48 83 7a 28 00",
    scan_origin: 0x199020,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const MENU_CHR_STATE: AobScan = AobScan {
    name:        "MenuChrState()",
    pattern:     "4c 63 ca 41 83 f9 14",
    scan_origin: 0x5013e0,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const LEVEL_UP: AobScan = AobScan {
    name:        "LevelUp()",
    pattern:     "48 85 d2 0f 84 ? ? ? ? 48 89 5c 24 18 57 48 81 ec 20 01 00 00 48 8b 05 ? ? ? ? \
                  48 33 c4",
    scan_origin: 0x38b1a0,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const LEVEL_LOOKUP: AobScan = AobScan {
    name:        "LevelLookup()",
    pattern:     "d1 ee 48 8b cd 8b d6 e8 ? ? ? ? 48 8b d8 eb e3 48 8b cb 0f b7 01",
    scan_origin: 0x38d140,
    offset:      -0x5c,
    scan_mode:   AddressingMode::Absolute,
};

pub const CHR_SET_ACTION: AobScan = AobScan {
    name:        "ChrSetAction()",
    pattern:     "48 89 5c 24 08 48 89 6c 24 10 48 89 74 24 18 57 48 83 ec 20 49 8b d8 48 8b fa \
                  48 8b f1",
    scan_origin: 0x424952,
    offset:      0x25,
    scan_mode:   AddressingMode::VfTableRelative {
        table_offset: 0x80,
    },
};

pub const EZ_STATE_EXTERNAL_EVENT_CTOR: AobScan = AobScan {
    name:        "EzStateExternalEventCtor()",
    pattern:     "48 8d 05 ? ? ? ? 48 89 01 c7 41 10 02 00 00 00 89 51 08 48 c7 81 00 02 00 00 00 \
                  00 00 00 48 8b c1 c3",
    scan_origin: 0x9951b0,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const EZ_STATE_EXECUTE_EVENT: AobScan = AobScan {
    name:        "EzStateExecuteEvent()",
    pattern:     "48 89 5c 24 20 55 56 57 41 54 41 55 41 56 41 57 48 8d ac 24 d0 f0 ff ff b8 30 \
                  10 00 00",
    scan_origin: 0x461f20,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const APPLY_SPEFFECT: AobScan = AobScan {
    name:        "ApplySpeffect()",
    pattern:     "88 5d e4 66 c7 45 e5 04 00 e8 ? ? ? ?",
    scan_origin: 0x145ee3,
    offset:      10,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const SET_SHARED_FLAG: AobScan = AobScan {
    name:        "SetSharedFlag",
    pattern:     "83 fa 07 77 ? 48 63 c2 44 38 84 08 ? ? ? ? 74 ? 44 88 84 08 ? ? ? ?",
    scan_origin: 0x41f360,
    offset:      18,
    scan_mode:   AddressingMode::Absolute,
};

pub const LOCKED_TARGET_POINTER: AobScan = AobScan {
    name:        "LockedTargetPointer",
    pattern:     "48 89 bb c0 00 00 00 eb ?",
    scan_origin: 0x49d192,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const CREDITS_SKIP: AobScan = AobScan {
    name:        "CreditsSkip",
    pattern:     "4c 8b dc 53 48 81 ec 20 02 00 00 8b 41 ? 48 8b d9 ff c8 83 f8 06",
    scan_origin: 0x59a60,
    offset:      4,
    scan_mode:   AddressingMode::Absolute,
};

pub const SKIP_LOGOS: AobScan = AobScan {
    name:        "SkipLogos",
    pattern:     "33 c0 38 05 ? ? ? ? c6 05 ? ? ? ? 01 0f 95 c0 89 41 10",
    scan_origin: 0xfd930,
    offset:      4,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const FASTER_MENU: AobScan = AobScan {
    name:        "FasterMenu",
    pattern:     "40 57 48 81 ec 60 01 00 00 48 8b 05 ? ? ? ? 48 33 c4 48 89 84 24 ? ? ? ? 83 79 \
                  ? 00 48 8b f9 7e ?",
    scan_origin: 0x105460,
    offset:      19,
    scan_mode:   AddressingMode::Absolute,
};

pub const EVENT_LOG: AobScan = AobScan {
    name:        "EventLog",
    pattern:     "b8 59 17 b7 d1 44 8b d7 48 8b f1 f7 e2 44 8b ca",
    scan_origin: 0x4750c0,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const PLAYER_NO_DAMAGE: AobScan = AobScan {
    name:        "PlayerNoDamage",
    pattern:     "49 8b 0e e8 ? ? ? ? 84 c0 74 ? 83 bb ? ? ? ? 00 7f ? 83 3f 01",
    scan_origin: 0x16a3a0,
    offset:      -6,
    scan_mode:   AddressingMode::Absolute,
};

pub const INFINITE_POISE: AobScan = AobScan {
    name:        "InfinitePoise",
    pattern:     "ff 90 ? ? ? ? 33 db 44 8d 43 ? 84 c0 0f 84 ? ? ? ? 39 9d ? ? ? ? 0f 95 c1",
    scan_origin: 0x1365c6,
    offset:      20,
    scan_mode:   AddressingMode::Absolute,
};

pub const INFINITE_STAMINA: AobScan = AobScan {
    name:        "InfiniteStamina",
    pattern:     "0f 83 ? ? ? ? 48 8b 0b f3 0f 10 89 ? ? ? ? f3 0f 5c ce 0f 2f ca",
    scan_origin: 0x333639,
    offset:      1,
    scan_mode:   AddressingMode::Absolute,
};

pub const INFINITE_CONSUMABLES: AobScan = AobScan {
    name:        "InfiniteConsumables",
    pattern:     "3b c6 0f 86 ? ? ? ? 44 8b ce 4c 8b c3 48 8b d5 48 8b cf e8 ? ? ? ?",
    scan_origin: 0x1af2b4,
    offset:      25,
    scan_mode:   AddressingMode::Absolute,
};

pub const INFINITE_DURABILITY: AobScan = AobScan {
    name:        "InfiniteDurability",
    pattern:     "4b 8d 0c 76 44 8b c7 41 8b d6 48 03 cf 48 8d 04 c9 48 8b cb f3 0f 11 b4 c3 94 \
                  00 00 00",
    scan_origin: 0x34b669,
    offset:      20,
    scan_mode:   AddressingMode::Absolute,
};

pub const INFINITE_CASTS: AobScan = AobScan {
    name:        "InfiniteCasts",
    pattern:     "88 4d ? 49 8b ce e8 ? ? ? ? 8b c7 48 83 c4 20 41 5e 5f 5e 5d 5b c3",
    scan_origin: 0x1aef00,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_SOUL_GAIN: AobScan = AobScan {
    name:        "NoSoulGain",
    pattern:     "66 41 0f 6e ce 0f 5b c9 f3 0f 59 c1 f3 48 0f 2c d0 e8 ? ? ? ?",
    scan_origin: 0x202489,
    offset:      17,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_HOLLOWING: AobScan = AobScan {
    name:        "NoHollowing",
    pattern:     "48 8d 44 24 ? 4c 8d 44 24 ? 80 fa 20 49 0f 4e c0 0f b6 00 88 81 ? ? ? ? c3",
    scan_origin: 0x38bae5,
    offset:      20,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_SOUL_LOSS: AobScan = AobScan {
    name:        "NoSoulLoss",
    pattern:     "c6 47 ? 01 48 8b 86 ? ? ? ? 8b 88 ? ? ? ? 89 4f ? 48 8b 86 ? ? ? ? 38 90 ? ? ? \
                  ? 75 ? 89 90 ? ? ? ?",
    scan_origin: 0x26afb0,
    offset:      35,
    scan_mode:   AddressingMode::Absolute,
};

pub const PLAYER_HIDDEN: AobScan = AobScan {
    name:        "PlayerHidden",
    pattern:     "49 8b f1 44 0f b6 fa 4d 85 c9 0f 84 ? ? ? ? 48 85 ff 0f 84 ? ? ? ?",
    scan_origin: 0x43bf1f,
    offset:      11,
    scan_mode:   AddressingMode::Absolute,
};

pub const PLAYER_SILENT: AobScan = AobScan {
    name:        "PlayerSilent",
    pattern:     "e8 ? ? ? ? 84 c0 74 ? 48 8b 83 10 14 00 00 48 85 c0 74 ? 48 89 07",
    scan_origin: 0x10e306,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const MENU_TRANSITION: AobScan = AobScan {
    name:        "MenuTransition",
    pattern:     "75 ? 48 8b 4f ? 33 d2 48 8b 01 ff 10 c7 47 ? 04 00 00 00 48 83 c4 40 5f c3",
    scan_origin: 0xef614,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_ROLL: AobScan = AobScan {
    name:        "NoRoll",
    pattern:     "48 8b 48 18 e8 ? ? ? ? 32 c0 48 83 c4 28 c3 b0 01 48 83 c4 28 c3",
    scan_origin: 0x38e97e,
    offset:      16,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_BACKSTEP: AobScan = AobScan {
    name:        "NoBackstep",
    pattern:     "e8 ? ? ? ? 32 c0 48 83 c4 28 c3 e8 ? ? ? ? 84 c0 0f 95 c0 48 83 c4 28 c3",
    scan_origin: 0x38ea22,
    offset:      19,
    scan_mode:   AddressingMode::Absolute,
};

pub const MAP_ID: AobScan = AobScan {
    name:        "MapId",
    pattern:     "8b 15 ? ? ? ? 8b c2 25 00 00 00 ff 3d 00 00 00 32",
    scan_origin: 0x1da784,
    offset:      2,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const KERNEL32_CREATE_THREAD: AobScan = AobScan {
    name:        "KERNEL32.DLL::CreateThread",
    pattern:     "48 8d 44 24 68 48 8b f9 48 89 44 24 28 33 c9 c7 44 24 20 00 00 00 00",
    scan_origin: 0x6dd262,
    offset:      25,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const KERNEL32_CLOSE_HANDLE: AobScan = AobScan {
    name:        "KERNEL32.DLL::CloseHandle",
    pattern:     "48 8b 4d e7 ff 15 ? ? ? ? 8b 45 ? 44 8b 45 ? 48 8b 4d ?",
    scan_origin: 0xc52d2a,
    offset:      6,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const KERNEL32_SLEEP: AobScan = AobScan {
    name:        "KERNEL32.DLL::Sleep",
    pattern:     "f7 e9 c1 fa 06 8b ca c1 e9 1f 03 ca",
    scan_origin: 0x870d3f,
    offset:      15,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const KERNEL32_LOAD_LIBRARY_W: AobScan = AobScan {
    name:        "KERNEL32.DLL::LoadLibraryW",
    pattern:     "48 8d 0d ? ? ? ? ff 15 ? ? ? ? 48 8b f0 48 8d 0d ? ? ? ? ff 15 ? ? ? ?",
    scan_origin: 0xacea90,
    offset:      9,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const DISABLE_AI: AobScan = AobScan {
    name:        "DisableAi",
    pattern:     "48 8b 03 48 8b d7 48 8b cb ff 50 38 48 8b 5b ? 48 85 db 75 ?",
    scan_origin: 0x423394,
    offset:      3,
    scan_mode:   AddressingMode::Absolute,
};

pub const TRIGGER_NG: AobScan = AobScan {
    name:        "TriggerNg()",
    pattern:     "48 83 ec 28 48 8b 0d ? ? ? ? 48 8b 01 ff 90 98 00 00 00 48 8b 0d ? ? ? ? ",
    scan_origin: 0xd8780,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const RESET_ENEMY: AobScan = AobScan {
    name:        "ResetEnemy()",
    pattern:     "48 89 5c 24 08 57 48 83 ec 20 48 8b f9 48 83 e9 80 0f b7 da e8 ? ? ? ? 66 3b c3",
    scan_origin: 0x4129f0,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};
