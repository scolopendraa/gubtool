use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn give_runes() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x258270,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x2582E0,
            Version::V1_2_3 => 0x258400,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x258B70,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x25AB40,
            Version::V1_5_0 => 0x25ACB0,
            Version::V1_6_0 => 0x25BB30,
            Version::V1_7_0 => 0x25BCC0,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x25C670,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x25C7A0,
            Version::V2_2_0 |
            Version::V2_2_3 |
            Version::V2_3_0 |
            Version::V2_4_0 |
            Version::V2_5_0 |
            Version::V2_6_0 |
            Version::V2_6_1 => 0x25E1B0,
            Version::Invalid => 0x0,
        }
    }
}

pub fn get_player_item_quantity_by_id() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x75ACC0,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x75AD30,
            Version::V1_2_3 => 0x75AE00,
            Version::V1_3_0 |
            Version::V1_3_1 => 0x75C1A0,
            Version::V1_3_2 => 0x75C180,
            Version::V1_4_0 => 0x7603C0,
            Version::V1_4_1 => 0x7602D0,
            Version::V1_5_0 => 0x761810,
            Version::V1_6_0 => 0x7636A0,
            Version::V1_7_0 => 0x764D80,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x773500,
            Version::V1_9_0 => 0x7745A0,
            Version::V1_9_1 => 0x774600,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x774890,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x784CF0,
            Version::V2_3_0 => 0x784EE0,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x784F40,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x7850C0,
            Version::Invalid => 0x0,
        }
    }
}

pub fn item_spawn() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x54E570,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x54E5E0,
            Version::V1_2_3 => 0x54E700,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x54F640,
            Version::V1_4_0 => 0x552330,
            Version::V1_4_1 => 0x552240,
            Version::V1_5_0 => 0x552840,
            Version::V1_6_0 => 0x5539E0,
            Version::V1_7_0 => 0x554850,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x55C1A0,
            Version::V1_9_0 => 0x55C4C0,
            Version::V1_9_1 => 0x55C520,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x55C760,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x5604E0,
            Version::V2_3_0 => 0x560660,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x5606A0,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x560670,
            Version::Invalid => 0x0,
        }
    }
}

pub fn grace_warp() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x5855B0,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x585620,
            Version::V1_2_3 => 0x585740,
            Version::V1_3_0 |
            Version::V1_3_1 => 0x586720,
            Version::V1_3_2 => 0x586710,
            Version::V1_4_0 => 0x589410,
            Version::V1_4_1 => 0x589320,
            Version::V1_5_0 => 0x589930,
            Version::V1_6_0 => 0x58AC00,
            Version::V1_7_0 => 0x58BA70,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x5951E0,
            Version::V1_9_0 => 0x595560,
            Version::V1_9_1 => 0x5955C0,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x595800,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x599B20,
            Version::V2_3_0 => 0x599CA0,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x599D00,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x599CD0,
            Version::Invalid => 0x0,
        }
    }
}

pub fn block_warp() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x5D7DA0,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x5D7E10,
            Version::V1_2_3 => 0x5D7F30,
            Version::V1_3_0 |
            Version::V1_3_1 => 0x5D8FC0,
            Version::V1_3_2 => 0x5D8FA0,
            Version::V1_4_0 => 0x5DBCA0,
            Version::V1_4_1 => 0x5DBBB0,
            Version::V1_5_0 => 0x5DC690,
            Version::V1_6_0 => 0x5DDE30,
            Version::V1_7_0 => 0x5DECB0,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x5EB330,
            Version::V1_9_0 => 0x5EC050,
            Version::V1_9_1 => 0x5EC0B0,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x5EC2F0,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x5F7850,
            Version::V2_3_0 => 0x5F79D0,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x5F7A30,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x5F7BB0,
            Version::Invalid => 0x0,
        }
    }
}

pub fn get_chr_ins_by_entity_id() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x4F7630,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x4F76A0,
            Version::V1_2_3 => 0x4F77C0,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x4F86D0,
            Version::V1_4_0 => 0x4FB4E0,
            Version::V1_4_1 => 0x4FB3F0,
            Version::V1_5_0 => 0x4FB780,
            Version::V1_6_0 => 0x4FC8F0,
            Version::V1_7_0 => 0x4FC8A0,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x503C30,
            Version::V1_9_0 => 0x503F50,
            Version::V1_9_1 => 0x503FB0,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x5041F0,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x507C70,
            Version::V2_3_0 => 0x507DF0,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x507E30,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x507E00,
            Version::Invalid => 0x0,
        }
    }
}

pub fn set_event() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x5D9E40,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x5D9EB0,
            Version::V1_2_3 => 0x5D9FD0,
            Version::V1_3_0 |
            Version::V1_3_1 => 0x5DB060,
            Version::V1_3_2 => 0x5DB040,
            Version::V1_4_0 => 0x5DDD40,
            Version::V1_4_1 => 0x5DDC50,
            Version::V1_5_0 => 0x5DE730,
            Version::V1_6_0 => 0x5DFED0,
            Version::V1_7_0 => 0x5E0D50,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x5ED450,
            Version::V1_9_0 => 0x5EE170,
            Version::V1_9_1 => 0x5EE1D0,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x5EE410,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x5F9970,
            Version::V2_3_0 => 0x5F9AF0,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x5F9B50,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x5F9CD0,
            Version::Invalid => 0x0,
        }
    }
}

pub fn get_event() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x5D9650,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x5D96C0,
            Version::V1_2_3 => 0x5D97E0,
            Version::V1_3_0 |
            Version::V1_3_1 => 0x5DA870,
            Version::V1_3_2 => 0x5DA850,
            Version::V1_4_0 => 0x5DD550,
            Version::V1_4_1 => 0x5DD460,
            Version::V1_5_0 => 0x5DDF40,
            Version::V1_6_0 => 0x5DF6E0,
            Version::V1_7_0 => 0x5E0560,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x5ECC60,
            Version::V1_9_0 => 0x5ED980,
            Version::V1_9_1 => 0x5ED9E0,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x5EDC20,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x5F9180,
            Version::V2_3_0 => 0x5F9300,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x5F9360,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x5F94E0,
            Version::Invalid => 0x0,
        }
    }
}

pub fn external_event_temporary_constructor() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x1F934E0,
            Version::V1_2_1 => 0x1F93530,
            Version::V1_2_2 => 0x1F939D0,
            Version::V1_2_3 => 0x1F94D10,
            Version::V1_3_0 => 0x1F9E470,
            Version::V1_3_1 => 0x1F9E210,
            Version::V1_3_2 => 0x1F9E1F0,
            Version::V1_4_0 => 0x1F85E60,
            Version::V1_4_1 => 0x1F85D70,
            Version::V1_5_0 => 0x1F93FD0,
            Version::V1_6_0 => 0x1F9E130,
            Version::V1_7_0 => 0x1FAB700,
            Version::V1_8_0 => 0x1FF7D00,
            Version::V1_8_1 => 0x1FF7CE0,
            Version::V1_9_0 => 0x1FFA9D0,
            Version::V1_9_1 => 0x1FFAAE0,
            Version::V2_0_0 => 0x1FFAD90,
            Version::V2_0_1 => 0x1FFAE70,
            Version::V2_2_0 => 0x20406F0,
            Version::V2_2_3 => 0x2040710,
            Version::V2_3_0 => 0x2040FB0,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x2040FF0,
            Version::V2_6_0 => 0x2040FC0,
            Version::V2_6_1 => 0x2041020,
            Version::Invalid => 0x0,
        }
    }
}

pub fn execute_talk_command() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0xE1E320,
            Version::V1_2_1 => 0xE1E370,
            Version::V1_2_2 => 0xE1E810,
            Version::V1_2_3 => 0xE1F2F0,
            Version::V1_3_0 => 0xE27650,
            Version::V1_3_1 => 0xE273F0,
            Version::V1_3_2 => 0xE273D0,
            Version::V1_4_0 => 0xE08220,
            Version::V1_4_1 => 0xE08130,
            Version::V1_5_0 => 0xE10150,
            Version::V1_6_0 => 0xE183A0,
            Version::V1_7_0 => 0xE1BB00,
            Version::V1_8_0 => 0xE628C0,
            Version::V1_8_1 => 0xE628A0,
            Version::V1_9_0 => 0xE65360,
            Version::V1_9_1 => 0xE65470,
            Version::V2_0_0 => 0xE65720,
            Version::V2_0_1 => 0xE65800,
            Version::V2_2_0 => 0xEA4E10,
            Version::V2_2_3 => 0xEA4E30,
            Version::V2_3_0 => 0xEA5320,
            Version::V2_4_0 |
            Version::V2_5_0 => 0xEA5280,
            Version::V2_6_0 => 0xEA5250,
            Version::V2_6_1 => 0xEA52B0,
            Version::Invalid => 0x0,
        }
    }
}

pub fn emevd_switch() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x555D00,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x555D70,
            Version::V1_2_3 => 0x555E90,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x556DA0,
            Version::V1_4_0 => 0x559AC0,
            Version::V1_4_1 => 0x5599D0,
            Version::V1_5_0 => 0x559FD0,
            Version::V1_6_0 => 0x55B170,
            Version::V1_7_0 => 0x55BFE0,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x563930,
            Version::V1_9_0 => 0x563C50,
            Version::V1_9_1 => 0x563CB0,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x563EF0,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x567C70,
            Version::V2_3_0 => 0x567DF0,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x567E30,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x567E00,
            Version::Invalid => 0x0,
        }
    }
}

pub fn emk_event_ins_constructor() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x56ECA0,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x56ED10,
            Version::V1_2_3 => 0x56EE30,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x56FDA0,
            Version::V1_4_0 => 0x572AC0,
            Version::V1_4_1 => 0x5729D0,
            Version::V1_5_0 => 0x572FE0,
            Version::V1_6_0 => 0x574180,
            Version::V1_7_0 => 0x574FF0,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x57DDC0,
            Version::V1_9_0 => 0x57E0E0,
            Version::V1_9_1 => 0x57E140,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x57E380,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x582570,
            Version::V2_3_0 => 0x5826F0,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x582730,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x582700,
            Version::Invalid => 0x0,
        }
    }
}

pub fn set_speffect() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x3E17E0,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x3E1850,
            Version::V1_2_3 => 0x3E1970,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x3E2090,
            Version::V1_4_0 |
            Version::V1_4_1 => 0x3E4550,
            Version::V1_5_0 => 0x3E4920,
            Version::V1_6_0 => 0x3E5700,
            Version::V1_7_0 => 0x3E5730,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x3E69C0,
            Version::V1_9_0 |
            Version::V1_9_1 |
            Version::V2_0_0 |
            Version::V2_0_1 => 0x3E6AF0,
            Version::V2_2_0 |
            Version::V2_2_3 |
            Version::V2_3_0 => 0x3E9100,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x3E9120,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x3E90F0,
            Version::Invalid => 0x0,
        }
    }
}

pub fn remove_speffect() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x4E6970,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x4E69E0,
            Version::V1_2_3 => 0x4E6B00,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x4E79C0,
            Version::V1_4_0 => 0x4EA730,
            Version::V1_4_1 => 0x4EA640,
            Version::V1_5_0 => 0x4EA9D0,
            Version::V1_6_0 => 0x4EBB40,
            Version::V1_7_0 => 0x4EBB50,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x4F2E00,
            Version::V1_9_0 => 0x4F3060,
            Version::V1_9_1 => 0x4F3070,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x4F32B0,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x4F67F0,
            Version::V2_3_0 => 0x4F6970,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x4F69B0,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x4F6980,
            Version::Invalid => 0x0,
        }
    }
}
