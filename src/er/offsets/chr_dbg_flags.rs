use crate::core::attach::{Version, module_handle, version};

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x3C50480,
        Version::ER1_2_1 => 0x3C504A0,
        Version::ER1_2_2 => 0x3C504C0,
        Version::ER1_2_3 => 0x3C534DE,
        Version::ER1_3_0 |
        Version::ER1_3_1 |
        Version::ER1_3_2 => 0x3C65050,
        Version::ER1_4_0 |
        Version::ER1_4_1 => 0x3C082E0,
        Version::ER1_5_0 => 0x3C2008A,
        Version::ER1_6_0 => 0x3C312B0,
        Version::ER1_7_0 => 0x3C4BC70,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x3CD9B97,
        Version::ER1_9_0 |
        Version::ER1_9_1 |
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x3CDCFE8,
        Version::ER2_2_0 => 0x3D661A0,
        Version::ER2_2_3 |
        Version::ER2_3_0 => 0x3D661C0,
        Version::ER2_4_0 |
        Version::ER2_5_0 |
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x3D661A0,
        Version::ERUnknown => 0x0,
        _ => 0x0,
    }
}

#[repr(u64)]
pub enum ChrDbgOffsets {
    PlayerNoDeath = 0x0,
    OneShot = 0x2,
    InfiniteGoods = 0x3,
    InfiniteStam = 0x4,
    InfiniteFp = 0x5,
    InfiniteArrows = 0x6,
    Hidden = 0x8,
    Silent = 0x9,
    AllNoDeath = 0xA,
    AllNoDamage = 0xB,
    AllNoHit = 0xC,
    AllNoAttack = 0xD,
    AllNoMove = 0xE,
    AllDisableAi = 0xF,
}
