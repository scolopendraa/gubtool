use crate::core::attach::{Version, module_handle, version};

pub fn locked_target_pointer() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x6F0A16,
        Version::ER1_2_1 |
        Version::ER1_2_2 => 0x6F0A86,
        Version::ER1_2_3 => 0x6F0BA6,
        Version::ER1_3_0 |
        Version::ER1_3_1 => 0x6F1ED6,
        Version::ER1_3_2 => 0x6F1EB6,
        Version::ER1_4_0 => 0x6F5D86,
        Version::ER1_4_1 => 0x6F5C96,
        Version::ER1_5_0 => 0x6F6B46,
        Version::ER1_6_0 => 0x6F8996,
        Version::ER1_7_0 => 0x6FA0E6,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x7078C6,
        Version::ER1_9_0 => 0x708966,
        Version::ER1_9_1 => 0x7089C6,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x708C56,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0x716FA2,
        Version::ER2_3_0 => 0x717192,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0x7171F2,
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x717372,
        _ => 0x0,
    }
}

pub fn target_no_stagger() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x474CC5,
        Version::ER1_2_1 |
        Version::ER1_2_2 => 0x474D35,
        Version::ER1_2_3 => 0x474E55,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x475AE5,
        Version::ER1_4_0 => 0x478315,
        Version::ER1_4_1 => 0x478225,
        Version::ER1_5_0 => 0x4785A5,
        Version::ER1_6_0 => 0x479605,
        Version::ER1_7_0 => 0x479755,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x47B0E5,
        Version::ER1_9_0 |
        Version::ER1_9_1 => 0x47B225,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x47B3C5,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0x47E2A7,
        Version::ER2_3_0 => 0x47E3B7,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0x47E3F7,
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x47E3C7,
        _ => 0x0,
    }
}

pub fn no_grab() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x4402FB,
        Version::ER1_2_1 |
        Version::ER1_2_2 => 0x44036B,
        Version::ER1_2_3 => 0x44048B,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x4410EB,
        Version::ER1_4_0 => 0x44390B,
        Version::ER1_4_1 => 0x44381B,
        Version::ER1_5_0 => 0x443C5B,
        Version::ER1_6_0 => 0x444CBB,
        Version::ER1_7_0 => 0x444E0B,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x44679B,
        Version::ER1_9_0 |
        Version::ER1_9_1 => 0x4468DB,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x446A7B,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0x44986B,
        Version::ER2_3_0 => 0x44997B,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0x4499BB,
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x44998B,
        _ => 0x0,
    }
}

pub fn infinite_poise() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x43CA6A,
        Version::ER1_2_1 |
        Version::ER1_2_2 => 0x43CADA,
        Version::ER1_2_3 => 0x43CBFA,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x43D820,
        Version::ER1_4_0 => 0x43FF10,
        Version::ER1_4_1 => 0x43FF50,
        Version::ER1_5_0 => 0x440390,
        Version::ER1_6_0 => 0x4411D0,
        Version::ER1_7_0 => 0x441250,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x442BB0,
        Version::ER1_9_0 |
        Version::ER1_9_1 => 0x442CF0,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x442DC0,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0x445B90,
        Version::ER2_3_0 => 0x445CA0,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0x445CE0,
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x445CB0,
        _ => 0x0,
    }
}

pub fn warp_coord_write() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x657AFA,
        Version::ER1_2_1 |
        Version::ER1_2_2 => 0x657B6A,
        Version::ER1_2_3 => 0x657C8A,
        Version::ER1_3_0 |
        Version::ER1_3_1 => 0x658F8A,
        Version::ER1_3_2 => 0x658F6A,
        Version::ER1_4_0 => 0x65BC3A,
        Version::ER1_4_1 => 0x65BB4A,
        Version::ER1_5_0 => 0x65C92A,
        Version::ER1_6_0 => 0x65E24A,
        Version::ER1_7_0 => 0x65F11A,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x66C2AA,
        Version::ER1_9_0 => 0x66D20A,
        Version::ER1_9_1 => 0x66D26A,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x66D4DA,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0x67A75A,
        Version::ER2_3_0 => 0x67A8DA,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0x67A93A,
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x67AABA,
        _ => 0x0,
    }
}

pub fn warp_angle_write() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x657ADA,
        Version::ER1_2_1 |
        Version::ER1_2_2 => 0x657B4A,
        Version::ER1_2_3 => 0x657C6A,
        Version::ER1_3_0 |
        Version::ER1_3_1 => 0x658F6A,
        Version::ER1_3_2 => 0x658F4A,
        Version::ER1_4_0 => 0x65BC1A,
        Version::ER1_4_1 => 0x65BB2A,
        Version::ER1_5_0 => 0x65C90A,
        Version::ER1_6_0 => 0x65E22A,
        Version::ER1_7_0 => 0x65F0FA,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x66C28A,
        Version::ER1_9_0 => 0x66D1EA,
        Version::ER1_9_1 => 0x66D24A,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x66D4BA,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0x67A73A,
        Version::ER2_3_0 => 0x67A8BA,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0x67A91A,
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x67AA9A,
        _ => 0x0,
    }
}

pub fn get_force_act_idx() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x527214E,
        Version::ER1_2_1 => 0x53F7581,
        Version::ER1_2_2 => 0x4F45CB4,
        Version::ER1_2_3 => 0x4EDD1F3,
        Version::ER1_3_0 => 0x1E42A60,
        Version::ER1_3_1 => 0x523C681,
        Version::ER1_3_2 => 0x54E7D7A,
        Version::ER1_4_0 => 0x4FF4209,
        Version::ER1_4_1 => 0x55AC955,
        Version::ER1_5_0 => 0x3683DE,
        Version::ER1_6_0 => 0x4F22453,
        Version::ER1_7_0 => 0xB7F343,
        Version::ER1_8_0 => 0x5757732,
        Version::ER1_8_1 => 0x56BA77E,
        Version::ER1_9_0 => 0x5679CFE,
        Version::ER1_9_1 => 0x17B5923,
        Version::ER2_0_0 => 0x1D9C6F4,
        Version::ER2_0_1 => 0x55611FB,
        Version::ER2_2_0 => 0x5AA1B87,
        Version::ER2_2_3 => 0x5A454DB,
        Version::ER2_3_0 => 0x57D2875,
        Version::ER2_4_0 => 0x1F7F940,
        Version::ER2_5_0 => 0x123846F,
        Version::ER2_6_0 => 0x53C8D9,
        Version::ER2_6_1 => 0x5BED8C4,
        _ => 0x0,
    }
}