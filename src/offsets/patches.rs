use crate::core::attach::{Version, ATTACHED_PROCESS};

pub fn no_logo() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0xAAAD4A,
            Version::V1_2_1 => 0xAAADCA,
            Version::V1_2_2 => 0xAAAE3A,
            Version::V1_2_3 => 0xAAAF1A,
            Version::V1_3_0 => 0xAB021D,
            Version::V1_3_1 => 0xAB022D,
            Version::V1_3_2 => 0xAB020D,
            Version::V1_4_0 => 0xA8FB6D,
            Version::V1_4_1 => 0xA8FA7D,
            Version::V1_5_0 => 0xA9417D,
            Version::V1_6_0 => 0xA9807D,
            Version::V1_7_0 => 0xA9972D,
            Version::V1_8_0 |
            Version::V1_8_1 => 0xADB0FD,
            Version::V1_9_0 => 0xADDC8D,
            Version::V1_9_1 => 0xADDCED,
            Version::V2_0_0 |
            Version::V2_0_1 => 0xADDF7D,
            Version::V2_2_0 |
            Version::V2_2_3 => 0xB0BD7D,
            Version::V2_3_0 => 0xB0C0ED,
            Version::V2_4_0 |
            Version::V2_5_0 => 0xB0C26D,
            Version::V2_6_0 => 0xB0C3ED,
            Version::V2_6_1 => 0xB0C44D,
            Version::Invalid => 0x0,
        }
    }
}

pub fn fps_cap() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0xDFEF5F,
            Version::V1_2_1 => 0xDFEFAF,
            Version::V1_2_2 => 0xDFF3BF,
            Version::V1_2_3 => 0xDFFE9F,
            Version::V1_3_0 => 0xE081CF,
            Version::V1_3_1 => 0xE07F6F,
            Version::V1_3_2 => 0xE07F4F,
            Version::V1_4_0 => 0xDE8C5F,
            Version::V1_4_1 => 0xDE8B6F,
            Version::V1_5_0 => 0xDF094F,
            Version::V1_6_0 => 0xDF6F7F,
            Version::V1_7_0 => 0xDFA6BF,
            Version::V1_8_0 => 0xE4167F,
            Version::V1_8_1 => 0xE4165F,
            Version::V1_9_0 => 0xE4438F,
            Version::V1_9_1 => 0xE4449F,
            Version::V2_0_0 => 0xE4474F,
            Version::V2_0_1 => 0xE4482F,
            Version::V2_2_0 => 0xE826BD,
            Version::V2_2_3 => 0xE826DD,
            Version::V2_3_0 => 0xE82BCD,
            Version::V2_4_0 |
            Version::V2_5_0 => 0xE82B2D,
            Version::V2_6_0 => 0xE82AFD,
            Version::V2_6_1 => 0xE82B5D,
            Version::Invalid => 0x0,
        }
    }
}

pub fn mute_music() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0xD4A95A,
            Version::V1_2_1 => 0xD4A9DA,
            Version::V1_2_2 => 0xD4AB3A,
            Version::V1_2_3 => 0xD4AC2A,
            Version::V1_3_0 => 0xD531BA,
            Version::V1_3_1 => 0xD531CA,
            Version::V1_3_2 => 0xD531AA,
            Version::V1_4_0 => 0xD3343A,
            Version::V1_4_1 => 0xD3334A,
            Version::V1_5_0 => 0xD38A6A,
            Version::V1_6_0 => 0xD3EDBA,
            Version::V1_7_0 => 0xD424CA,
            Version::V1_8_0 => 0xD88D0A,
            Version::V1_8_1 => 0xD88CEA,
            Version::V1_9_0 => 0xD8BA1A,
            Version::V1_9_1 => 0xD8BB2A,
            Version::V2_0_0 |
            Version::V2_0_1 => 0xD8BDDA,
            Version::V2_2_0 => 0xDC3E7A,
            Version::V2_2_3 => 0xDC3E9A,
            Version::V2_3_0 => 0xDC438A,
            Version::V2_4_0 |
            Version::V2_5_0 => 0xDC475A,
            Version::V2_6_0 => 0xDC48DA,
            Version::V2_6_1 => 0xDC493A,
            Version::Invalid => 0x0,
        }
    }
}

pub fn pause_world() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0xA98175,
            Version::V1_2_1 => 0xA981F5,
            Version::V1_2_2 => 0xA98265,
            Version::V1_2_3 => 0xA98345,
            Version::V1_3_0 => 0xA9D555,
            Version::V1_3_1 => 0xA9D565,
            Version::V1_3_2 => 0xA9D545,
            Version::V1_4_0 => 0xA7CDC5,
            Version::V1_4_1 => 0xA7CCD5,
            Version::V1_5_0 => 0xA81335,
            Version::V1_6_0 => 0xA851E5,
            Version::V1_7_0 => 0xA868F5,
            Version::V1_8_0 |
            Version::V1_8_1 => 0xAC78E5,
            Version::V1_9_0 => 0xACA475,
            Version::V1_9_1 => 0xACA4D5,
            Version::V2_0_0 |
            Version::V2_0_1 => 0xACA765,
            Version::V2_2_0 |
            Version::V2_2_3 => 0xAF7E05,
            Version::V2_3_0 => 0xAF8175,
            Version::V2_4_0 |
            Version::V2_5_0 => 0xAF8335,
            Version::V2_6_0 => 0xAF84B5,
            Version::V2_6_1 => 0xAF8515,
            Version::Invalid => 0x0,
        }
    }
}

pub fn torrent_disabled_underworld() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0xC81F8A,
            Version::V1_2_1 => 0xC8200A,
            Version::V1_2_2 => 0xC8216A,
            Version::V1_2_3 => 0xC8225A,
            Version::V1_3_0 => 0xC891FA,
            Version::V1_3_1 => 0xC8920A,
            Version::V1_3_2 => 0xC891EA,
            Version::V1_4_0 => 0xC69A7A,
            Version::V1_4_1 => 0xC6998A,
            Version::V1_5_0 => 0xC6E77A,
            Version::V1_6_0 => 0xC730EA,
            Version::V1_7_0 => 0xC74DFA,
            Version::V1_8_0 => 0xCBA2FA,
            Version::V1_8_1 => 0xCBA2DA,
            Version::V1_9_0 => 0xCBCF1A,
            Version::V1_9_1 => 0xCBCF7A,
            Version::V2_0_0 |
            Version::V2_0_1 => 0xCBD20A,
            Version::V2_2_0 |
            Version::V2_2_3 => 0xCEF0EA,
            Version::V2_3_0 => 0xCEF5DA,
            Version::V2_4_0 |
            Version::V2_5_0 => 0xCEF9AA,
            Version::V2_6_0 => 0xCEFB2A,
            Version::V2_6_1 => 0xCEFB8A,
            Version::Invalid => 0x0,
        }
    }
}

pub fn whistle_disabled() -> u64 {
    unsafe {
        ATTACHED_PROCESS.module_handle + match ATTACHED_PROCESS.version {
            Version::V1_2_0 => 0x6DFF7F,
            Version::V1_2_1 |
            Version::V1_2_2 => 0x6DFFEF,
            Version::V1_2_3 => 0x6E010F,
            Version::V1_3_0 |
            Version::V1_3_1 => 0x6E150F,
            Version::V1_3_2 => 0x6E14EF,
            Version::V1_4_0 => 0x6E51AF,
            Version::V1_4_1 => 0x6E50BF,
            Version::V1_5_0 => 0x6E5F6F,
            Version::V1_6_0 => 0xC730EA,
            Version::V1_7_0 => 0x6E92DF,
            Version::V1_8_0 |
            Version::V1_8_1 => 0x6F6ABF,
            Version::V1_9_0 => 0x6F7B5F,
            Version::V1_9_1 => 0x6F7BBF,
            Version::V2_0_0 |
            Version::V2_0_1 => 0x6F7E4F,
            Version::V2_2_0 |
            Version::V2_2_3 => 0x705E9F,
            Version::V2_3_0 => 0x70608F,
            Version::V2_4_0 |
            Version::V2_5_0 => 0x7060EF,
            Version::V2_6_0 |
            Version::V2_6_1 => 0x70626F,
            Version::Invalid => 0x0,
        }
    }
}
