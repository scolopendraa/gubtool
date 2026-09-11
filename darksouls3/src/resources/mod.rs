pub mod bonfires;
pub mod bosses;
pub(crate) mod chr_names;
pub mod items;
pub(crate) mod scan_patterns;
pub mod talk_commands;

use {
    assemble::AsmFolder,
    std::{env, sync::LazyLock},
};

static ASM_LIB_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/darksouls3.bin"));

pub(crate) static ASM: LazyLock<AsmFolder> =
    LazyLock::new(|| bincode::deserialize(ASM_LIB_BYTES).unwrap());
