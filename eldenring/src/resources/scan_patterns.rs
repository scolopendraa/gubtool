use gubtool_core::aob_scanner::pattern::{AddressingMode, AobScan};

pub const WORLD_CHR_MAN: AobScan = AobScan {
    name:        "WorldChrMan",
    pattern:     "41 8b f0 48 8b da 48 8b f9 4c 8b b5 e8 00 00 00 4c 8b bd f0 00 00 00 45 33 e4",
    scan_origin: 0x6d016f,
    offset:      29,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const FIELD_AREA: AobScan = AobScan {
    name:        "FieldArea",
    pattern:     "48 8b 3d ? ? ? ? 49 8b d8 48 8b f2 48 8b e9 48 85 ff 0f 84 b3 00 00 00",
    scan_origin: 0x61e36b,
    offset:      3,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GAME_MAN: AobScan = AobScan {
    name:        "GameMan",
    pattern:     "0f 28 01 0f 11 80 b0 0a 00 00 c3",
    scan_origin: 0x67aa97,
    offset:      -4,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GAME_DATA_MAN: AobScan = AobScan {
    name:        "GameDataMan",
    pattern:     "0f 84 27 01 00 00 f6 41",
    scan_origin: 0x55fab3,
    offset:      -24,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const MENU_MAN: AobScan = AobScan {
    name:        "MenuMan",
    pattern:     "0f 45 f8 48 8b 0d ? ? ? ? 48 85 c9 75 2e",
    scan_origin: 0x7adc94,
    offset:      6,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const CHR_DBG_FLAGS: AobScan = AobScan {
    name:        "ChrDbgFlags",
    pattern:     "80 3d ? ? ? ? 00 0f 85 6c",
    scan_origin: 0x437619,
    offset:      2,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 5,
    },
};

pub const CS_EMK_SYSTEM: AobScan = AobScan {
    name:        "CSEmkSystem",
    pattern:     "0f 11 80 8c",
    scan_origin: 0x583056,
    offset:      10,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const VIRTUAL_MEMORY_FLAG: AobScan = AobScan {
    name:        "CSFD4VirtualMemoryFlag",
    pattern:     "48 8b 3d ? ? ? ? 48 85 ff 74 53",
    scan_origin: 0xd441a6,
    offset:      3,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const DAMAGE_MANAGER: AobScan = AobScan {
    name:        "DamageManager",
    pattern:     "48 3b df 0f 84 87 01",
    scan_origin: 0x445e69,
    offset:      42,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const MAP_ITEM_MAN_IMPL: AobScan = AobScan {
    name:        "MapItemManImpl",
    pattern:     "44 8b 4f ? 48 8d 57 ? 44 8b 47 ? 48 8b 49 ? e8 ? ? ? ? 0f b6 d8 84 c0 74 49 48 \
                  8b 0d ? ? ? ?",
    scan_origin: 0x57bc27,
    offset:      31,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const DL_USER_INPUT_MANAGER_IMPL: AobScan = AobScan {
    name:        "DLUserInputManagerImpl",
    pattern:     "0f b6 80 8e 08 00 00 b9",
    scan_origin: 0x1f264bb,
    offset:      -14,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const CS_FLIPPER_IMP: AobScan = AobScan {
    name:        "CSFlipperImp",
    pattern:     "f3 0f 10 b7 14 01 00 00 85",
    scan_origin: 0x2631e6,
    offset:      15,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const CS_DLC_IMP: AobScan = AobScan {
    name:        "CSDlcImp",
    pattern:     "c6 80 f1 00 00 00 00 48 8b 0d ? ? ? ? 48 85 c9",
    scan_origin: 0x255b9a,
    offset:      10,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const MAP_DBG_FLAGS: AobScan = AobScan {
    name:        "MapDbgFlags",
    pattern:     "48 81 c2 d8 02 00 00 44 0f b6 ? ? ? ? 03",
    scan_origin: 0x9bb4b1,
    offset:      11,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 3,
    },
};

pub const WORLD_CHR_MAN_DBG: AobScan = AobScan {
    name:        "WorldChrManDbg",
    pattern:     "80 78 67 00 0f 84 6b 02 00 00 f6 87 c8 01 00 00 01 75 10 48 8b cf",
    scan_origin: 0x6d016f,
    offset:      -19,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GRACE_WARP: AobScan = AobScan {
    name:        "GraceWarp()",
    pattern:     "c7 44 24 30 10 27 00 00 48 8b cf",
    scan_origin: 0x599ce4,
    offset:      -20,
    scan_mode:   AddressingMode::Absolute,
};

pub const BLOCK_WARP: AobScan = AobScan {
    name:        "BlockWarp()",
    pattern:     "48 83 ec 48 48 c7 44 24 28 fe ff ff ff e8 ? ? ? ?",
    scan_origin: 0x5f7bb0,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const GET_PLAYER_ITEM_QUANTITY_BY_ID: AobScan = AobScan {
    name:        "GetPlayerItemQuantityById()",
    pattern:     "40 57 48 83 ec 30 48 c7 44 24 20 fe ff ff ff 48 89 5c 24 58 48 8b f9 48 8d 44 \
                  24 48 48 89 44 24 50",
    scan_origin: 0x774890,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const ITEM_SPAWN: AobScan = AobScan {
    name:        "ItemSpawn()",
    pattern:     "4c 8d 45 34",
    scan_origin: 0x55c774,
    offset:      12,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GIVE_RUNES: AobScan = AobScan {
    name:        "GiveRunes()",
    pattern:     "74 12 8b 53 6c",
    scan_origin: 0xa376af,
    offset:      11,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GET_EVENT: AobScan = AobScan {
    name:        "GetEvent()",
    pattern:     "e8 ? ? ? ? 85 c0 40 0f 95 c6 40 84 ed 48 8b 6c 24 30",
    scan_origin: 0x5c7072,
    offset:      1,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const SET_EVENT: AobScan = AobScan {
    name:        "SetEvent()",
    pattern:     "49 8b 50 08 45 2b d1 80 7a 19 00 75",
    scan_origin: 0x5ee435,
    offset:      -37,
    scan_mode:   AddressingMode::Absolute,
};

pub const SET_SPEFFECT: AobScan = AobScan {
    name:        "SetSpeffect()",
    pattern:     "85 d2 78 09 48 8b",
    scan_origin: 0x4fb080,
    offset:      9,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const REMOVE_SPEFFECT: AobScan = AobScan {
    name:        "RemoveSpeffect()",
    pattern:     "0f b6 d0 42",
    scan_origin: 0x5605b9,
    offset:      13,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const GET_CHR_INS_BY_ENTITY_ID: AobScan = AobScan {
    name:        "GetChrInsByEntityId()",
    pattern:     "48 8d 93 34 02",
    scan_origin: 0x6e9982,
    offset:      8,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const EMEVD_SWITCH: AobScan = AobScan {
    name:        "EmevdSwitch()",
    pattern:     "8b 8b c0 00 00 00 89 4b",
    scan_origin: 0x57ed60,
    offset:      -29,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const EMK_EVENT_INS_CTOR: AobScan = AobScan {
    name:        "EmkEventInsCtor()",
    pattern:     "75 0c 0f b7 47",
    scan_origin: 0x583915,
    offset:      -45,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const EXTERNAL_EVENT_TEMP_CTOR: AobScan = AobScan {
    name:        "ExternalEventTempCtor()",
    pattern:     "c7 41 10 02 00 00 00 89 51",
    scan_origin: 0x1ffad9d,
    offset:      -13,
    scan_mode:   AddressingMode::Absolute,
};

pub const EXECUTE_TALK_COMMAND: AobScan = AobScan {
    name:        "ExecuteTalkCommand()",
    pattern:     "89 7d 80 48 8b 02 48 8b ca",
    scan_origin: 0xe6576f,
    offset:      -79,
    scan_mode:   AddressingMode::Absolute,
};

pub const LOCKED_TARGET_POINTER: AobScan = AobScan {
    name:        "LockedTargetPointer",
    pattern:     "74 17 48 8b 8f 88",
    scan_origin: 0x708c54,
    offset:      2,
    scan_mode:   AddressingMode::Absolute,
};

pub const TARGET_NO_STAGGER: AobScan = AobScan {
    name:        "TargetNoStagger",
    pattern:     "48 85 c0 74 1a 48 8b 00 48 85 c0 74 12",
    scan_origin: 0x47b397,
    offset:      46,
    scan_mode:   AddressingMode::Absolute,
};

pub const PLAYER_NO_GRAB: AobScan = AobScan {
    name:        "PlayerNoGrab",
    pattern:     "41 8b 56 44 48 8d 4c",
    scan_origin: 0x446a7b,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const PLAYER_INFINITE_POISE: AobScan = AobScan {
    name:        "PlayerInfinitePoise",
    pattern:     "eb 03 ? ? ? 48 8b ce e8 ? ? ? ? 48 8b c8 4c 8b",
    scan_origin: 0x442db0,
    offset:      16,
    scan_mode:   AddressingMode::Absolute,
};

pub const WARP_COORD_WRITE: AobScan = AobScan {
    name:        "WarpCoordWrite",
    pattern:     "0f 11 80 a0 0a",
    scan_origin: 0x66d4da,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const WARP_ANGLE_WRITE: AobScan = AobScan {
    name:        "WarpAngleWrite",
    pattern:     "0f 11 80 b0 0a 00 00 c3",
    scan_origin: 0x66d4ba,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const GET_FORCE_ACT_IDX: AobScan = AobScan {
    name:        "GetForceActIdx",
    pattern:     "48 8b 41 08 0f be 80 ? ? 00 00 48 8d 64 24 08",
    scan_origin: 0x56ba77e,
    offset:      4,
    scan_mode:   AddressingMode::Absolute,
};

pub const SET_REQUESTED_ACTION: AobScan = AobScan {
    name:        "SetRequestedAction",
    pattern:     "74 05 49 09 41 10",
    scan_origin: 0x4050c0,
    offset:      2,
    scan_mode:   AddressingMode::Absolute,
};

pub const NO_LOGO: AobScan = AobScan {
    name:        "NoLogo",
    pattern:     "48 85 d2 74 07 c6 82",
    scan_origin: 0xaddf65,
    offset:      24,
    scan_mode:   AddressingMode::Absolute,
};

pub const FPS_CAP: AobScan = AobScan {
    name:        "FpsCap",
    pattern:     "c7 43 ? 89 88 88 3c",
    scan_origin: 0xe4474f,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const MUTE_MUSIC: AobScan = AobScan {
    name:        "MuteMusic",
    pattern:     "48 8b f8 48 85 c0 0f 84 ? ? ? ? 0f b6 48 04",
    scan_origin: 0xd8bdce,
    offset:      12,
    scan_mode:   AddressingMode::Absolute,
};

pub const PAUSE_WORLD: AobScan = AobScan {
    name:        "PauseWorld",
    pattern:     "0f 84 87 00 00 00 c6 83",
    scan_origin: 0xaca765,
    offset:      0,
    scan_mode:   AddressingMode::Absolute,
};

pub const TORRENT_DISABLED_UNDERWOLD: AobScan = AobScan {
    name:        "TorrentDisabledUnderworld",
    pattern:     "80 78 36 00 0f",
    scan_origin: 0xcbd206,
    offset:      4,
    scan_mode:   AddressingMode::Absolute,
};

pub const WHISTLE_DISABLED: AobScan = AobScan {
    name:        "WhistleDisabled",
    pattern:     "80 79 36 00 0f 95 c0 48 83 c4 28 c3",
    scan_origin: 0x6f7e4b,
    offset:      4,
    scan_mode:   AddressingMode::Absolute,
};

pub const OPEN_MAP: AobScan = AobScan {
    name:        "OpenMap",
    pattern:     "84 c0 74 2e c7",
    scan_origin: 0x7dd2a8,
    offset:      2,
    scan_mode:   AddressingMode::Absolute,
};

pub const CLOSE_MAP: AobScan = AobScan {
    name:        "CloseMap",
    pattern:     "48 8b 03 48 8b cb 48 8b 94 24 ? 00 00 00 ff 50 ? 48 8d 8b ? 27 00 00",
    scan_origin: 0x9a382c,
    offset:      14,
    scan_mode:   AddressingMode::Absolute,
};

pub const CAN_FAST_TRAVEL: AobScan = AobScan {
    name:        "CanFastTravel",
    pattern:     "74 14 ba 16",
    scan_origin: 0x7b3494,
    offset:      12,
    scan_mode:   AddressingMode::Absolute,
};

pub const KERNEL32_CREATE_THREAD: AobScan = AobScan {
    name:        "KERNEL32.DLL::CreateThread",
    pattern:     "ba 00 00 02 00 89 74 24 20 33 c9",
    scan_origin: 0x21ef794,
    offset:      13,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const KERNEL32_CLOSE_HANDLE: AobScan = AobScan {
    name:        "KERNEL32.DLL::CloseHandle",
    pattern:     "e8 ? ? ? ? 48 8b 8f 70 2b 00 00 33 d2 ff 15",
    scan_origin: 0x2235b61,
    offset:      33,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};

pub const KERNEL32_LOAD_LIBRARY_W: AobScan = AobScan {
    name:        "KERNEL32.DLL::LoadLibraryW",
    pattern:     "ff 15 ? ? ? ? 48 89 43 30 48 85 c0 75 0d",
    scan_origin: 0x170b1ea,
    offset:      2,
    scan_mode:   AddressingMode::Relative {
        bytes_to_next_instr: 4,
    },
};
