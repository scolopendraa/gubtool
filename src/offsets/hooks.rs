use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn locked_target_pointer() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x6F0A16,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x6F0A86,
            Version::V1_2_3 => 0x6F0BA6,
            Version::V1_3_0 |
            Version::V1_3_1 => 0x6F1ED6,
            Version::V1_3_2 => 0x6F1EB6,
            Version::V1_4_0 => 0x6F5D86,
            Version::V1_4_1 => 0x6F5C96,
            Version::V1_5_0 => 0x6F6B46,
            Version::V1_6_0 => 0x6F8996,
            Version::V1_7_0 => 0x6FA0E6,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x7078C6,
            Version::V1_9_0 => 0x708966,
            Version::V1_9_1 => 0x7089C6,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x708C56,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x716FA2,
            Version::V2_3_0 => 0x717192,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x7171F2,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x717372,
            Version::Invalid => 0x0,
        }
    }
}

pub fn target_no_stagger() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x474CC5,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x474D35,
            Version::V1_2_3 => 0x474E55,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x475AE5,
            Version::V1_4_0 => 0x478315,
            Version::V1_4_1 => 0x478225,
            Version::V1_5_0 => 0x4785A5,
            Version::V1_6_0 => 0x479605,
            Version::V1_7_0 => 0x479755,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x47B0E5,
            Version::V1_9_0 |
            Version::V1_9_1 => 0x47B225,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x47B3C5,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x47E2A7,
            Version::V2_3_0 => 0x47E3B7,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x47E3F7,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x47E3C7,
            Version::Invalid => 0x0,
        }
    }
}

pub fn no_grab() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x4402FB,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x44036B,
            Version::V1_2_3 => 0x44048B,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x4410EB,
            Version::V1_4_0 => 0x44390B,
            Version::V1_4_1 => 0x44381B,
            Version::V1_5_0 => 0x443C5B,
            Version::V1_6_0 => 0x444CBB,
            Version::V1_7_0 => 0x444E0B,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x44679B,
            Version::V1_9_0 |
            Version::V1_9_1 => 0x4468DB,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x446A7B,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x44986B,
            Version::V2_3_0 => 0x44997B,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x4499BB,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x44998B,
            Version::Invalid => 0x0,
        }
    }
}

pub fn infinite_poise() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x43CA6A,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x43CADA,
            Version::V1_2_3 => 0x43CBFA,
            Version::V1_3_0 |
            Version::V1_3_1 |
            Version::V1_3_2 => 0x43D820,
            Version::V1_4_0 => 0x43FF10,
            Version::V1_4_1 => 0x43FF50,
            Version::V1_5_0 => 0x440390,
            Version::V1_6_0 => 0x4411D0,
            Version::V1_7_0 => 0x441250,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x442BB0,
            Version::V1_9_0 |
            Version::V1_9_1 => 0x442CF0,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x442DC0,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x445B90,
            Version::V2_3_0 => 0x445CA0,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x445CE0,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x445CB0,
            Version::Invalid => 0x0,
        }
    }
}

pub fn warp_coord_write() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x657AFA,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x657B6A,
            Version::V1_2_3 => 0x657C8A,
            Version::V1_3_0 |
            Version::V1_3_1 => 0x658F8A,
            Version::V1_3_2 => 0x658F6A,
            Version::V1_4_0 => 0x65BC3A,
            Version::V1_4_1 => 0x65BB4A,
            Version::V1_5_0 => 0x65C92A,
            Version::V1_6_0 => 0x65E24A,
            Version::V1_7_0 => 0x65F11A,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x66C2AA,
            Version::V1_9_0 => 0x66D20A,
            Version::V1_9_1 => 0x66D26A,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x66D4DA,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x67A75A,
            Version::V2_3_0 => 0x67A8DA,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x67A93A,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x67AABA,
            Version::Invalid => 0x0,
        }
    }
}

pub fn warp_angle_write() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x657ADA,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x657B4A,
            Version::V1_2_3 => 0x657C6A,
            Version::V1_3_0 |
            Version::V1_3_1 => 0x658F6A,
            Version::V1_3_2 => 0x658F4A,
            Version::V1_4_0 => 0x65BC1A,
            Version::V1_4_1 => 0x65BB2A,
            Version::V1_5_0 => 0x65C90A,
            Version::V1_6_0 => 0x65E22A,
            Version::V1_7_0 => 0x65F0FA,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x66C28A,
            Version::V1_9_0 => 0x66D1EA,
            Version::V1_9_1 => 0x66D24A,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x66D4BA,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x67A73A,
            Version::V2_3_0 => 0x67A8BA,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x67A91A,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x67AA9A,
            Version::Invalid => 0x0,
        }
    }
}

pub fn get_force_act_idx() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x527214E,
            Version::V1_2_1 => 0x53F7581,
            Version::V1_2_2 => 0x4F45CB4,
            Version::V1_2_3 => 0x4EDD1F3,
            Version::V1_3_0 => 0x1E42A60,
            Version::V1_3_1 => 0x523C681,
            Version::V1_3_2 => 0x54E7D7A,
            Version::V1_4_0 => 0x4FF4209,
            Version::V1_4_1 => 0x55AC955,
            Version::V1_5_0 => 0x3683DE,
            Version::V1_6_0 => 0x4F22453,
            Version::V1_7_0 => 0xB7F343,
            Version::V1_8_0 => 0x5757732,
            Version::V1_8_1 => 0x56BA77E,
            Version::V1_9_0 => 0x5679CFE,
            Version::V1_9_1 => 0x17B5923,
            Version::V2_0_0 => 0x1D9C6F4,
            Version::V2_0_1 => 0x55611FB,
            Version::V2_2_0 => 0x5AA1B87,
            Version::V2_2_3 => 0x5A454DB,
            Version::V2_3_0 => 0x57D2875,
            Version::V2_4_0 => 0x1F7F940,
            Version::V2_5_0 => 0x123846F,
            Version::V2_6_0 => 0x53C8D9,
            Version::V2_6_1 => 0x5BED8C4,
            Version::Invalid => 0x0,
        }
    }
}
