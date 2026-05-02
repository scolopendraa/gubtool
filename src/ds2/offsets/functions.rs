use crate::core::attach::{Version, module_handle, version};

pub fn give_souls() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x3AD170,
        Version::Vanilla1_0_12 => 0x3B3B10,
        Version::Scholar1_0_2 => 0x3841E0,
        Version::Scholar1_0_3 => 0x38AB40,
        _ => 0x0,
    }
}

pub fn warp_prep() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x20A670,
        Version::Vanilla1_0_12 => 0x20CE40,
        Version::Scholar1_0_2 => 0x1811D0,
        Version::Scholar1_0_3 => 0x1843B0,
        _ => 0x0,
    }
}

pub fn warp() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x20AE10,
        Version::Vanilla1_0_12 => 0x20D5E0,
        Version::Scholar1_0_2 => 0x181650,
        Version::Scholar1_0_3 => 0x184830,
        _ => 0x0,
    }
}

pub fn item_give() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x227F00,
        Version::Vanilla1_0_12 => 0x22AD20,
        Version::Scholar1_0_2 => 0x1A3D00,
        Version::Scholar1_0_3 => 0x1A7470,
        _ => 0x0,
    }
}

pub fn build_item_dialog() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x11F360,
        Version::Vanilla1_0_12 => 0x11F430,
        Version::Scholar1_0_2 => 0x5D8C0,
        Version::Scholar1_0_3 => 0x5D950,
        _ => 0x0,
    }
}

pub fn show_item_dialog() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x4F3730,
        Version::Vanilla1_0_12 => 0x4FA9B0,
        Version::Scholar1_0_2 => 0x4F9E70,
        Version::Scholar1_0_3 => 0x501080,
        _ => 0x0,
    }
}

pub fn current_item_quantity_check() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x22EEF0,
        Version::Vanilla1_0_12 => 0x231D10,
        Version::Scholar1_0_2 => 0x1B1A40,
        Version::Scholar1_0_3 => 0x1B51B0,
        _ => 0x0,
    }
}

pub fn sleep() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x13727BC,
        Version::Vanilla1_0_12 => 0x14547BC,
        _ => 0x0,
    }
}