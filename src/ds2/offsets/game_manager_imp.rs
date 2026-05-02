use crate::core::attach::{Version, module_handle, version};

pub fn base() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x11493F4,
        Version::Vanilla1_0_12 => 0x1150414,
        Version::Scholar1_0_2 => 0x160B8D0,
        Version::Scholar1_0_3 => 0x16148F0,
        _ => 0x0,
    }
}

pub fn event_manager() -> u64 {
    match version() {
        Version::Vanilla1_0_11 |
        Version::Vanilla1_0_12 => 0x44,
        Version::Scholar1_0_2 |
        Version::Scholar1_0_3 => 0x70,
        _ => 0x0,
    }
}

pub fn player_ctrl() -> u64 {
    match version() {
        Version::Vanilla1_0_11 |
        Version::Vanilla1_0_12 => 0x74,
        Version::Scholar1_0_2 |
        Version::Scholar1_0_3 => 0xD0,
        _ => 0x0,
    }
}

pub fn loading_flag() -> u64 {
    match version() {
        Version::Vanilla1_0_11 |
        Version::Vanilla1_0_12 => 0xDFC,
        Version::Scholar1_0_2 |
        Version::Scholar1_0_3 => 0x24BC,
        _ => 0x0,
    }
}

pub mod event_manager_offsets {
use crate::core::attach::{Version, version};
    pub fn warp_event_entity() -> u64 {
        match version() {
            Version::Vanilla1_0_11 |
            Version::Vanilla1_0_12 => 0x38,
            Version::Scholar1_0_2 |
            Version::Scholar1_0_3 => 0x70,
            _ => 0x0,
        }
    }
}

pub fn quitout() -> u64 {
    match version() {
        Version::Vanilla1_0_11 |
        Version::Vanilla1_0_12 => 0xDF1,
        Version::Scholar1_0_2 |
        Version::Scholar1_0_3 => 0x24B1,
        _ => 0x0,
    }
}

pub mod chr_ctrl_offsets {
use crate::core::attach::{Version, version};
    pub fn stats_ptr() -> u64 {
        match version() {
            Version::Vanilla1_0_11 |
            Version::Vanilla1_0_12 => 0x378,
            Version::Scholar1_0_2 |
            Version::Scholar1_0_3 => 0x490,
            _ => 0x0,
        }
    }
    pub fn coords() -> u64 {
        match version() {
            Version::Vanilla1_0_11 |
            Version::Vanilla1_0_12 => 0x80,
            Version::Scholar1_0_2 |
            Version::Scholar1_0_3 => 0x90,
            _ => 0x0,
        }
    }
    pub fn min_hp() -> u64 {
        match version() {
            Version::Vanilla1_0_11 |
            Version::Vanilla1_0_12 => 0x100,
            Version::Scholar1_0_2 |
            Version::Scholar1_0_3 => 0x16C,
            _ => 0x0,
        }
    }
}

pub mod hk_hardware_info {
use crate::core::attach::{Version, version, module_handle};
    pub fn base() -> u64 {
        module_handle() + match version() {
            Version::Scholar1_0_1 => 0x0,
            Version::Scholar1_0_2 => 0x16114C0,
            Version::Scholar1_0_3 => 0x161A4E0,
            _ => 0x0,
        }
    }
}

pub fn px_world() -> u64 {
    match version() {
        Version::Vanilla1_0_11 |
        Version::Vanilla1_0_12 => 0x280,
        Version::Scholar1_0_2 |
        Version::Scholar1_0_3 => 0x660,
        _ => 0x0,
    }
}

pub mod px_world_offsets {
use crate::core::attach::{Version, version};
    pub fn player_coords_chain() -> [u64; 5] {
        match version() {
            Version::Vanilla1_0_11 |
            Version::Vanilla1_0_12 => [0xC, 0x168, 0xC, 0x4, 0x120],
            Version::Scholar1_0_2 |
            Version::Scholar1_0_3 => [0x18, 0x1F8, 0x18, 0x8, 0x1A0],
            _ => [0x0,0x0,0x0,0x0,0x0],
        }
    }
}