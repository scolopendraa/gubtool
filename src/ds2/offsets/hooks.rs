use crate::core::attach::{Version, module_handle, version};

pub fn warp_coord_write() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x7F9FB0,
        Version::Vanilla1_0_12 => 0x8015B0,
        Version::Scholar1_0_2 => 0x711939,
        Version::Scholar1_0_3 => 0x718E99,
        _ => 0x0,
    }
}