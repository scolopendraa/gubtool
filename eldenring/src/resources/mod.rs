pub mod aow;
pub mod bosses;
pub mod chr_names;
pub(crate) mod entity_ids;
pub mod graces;
pub mod items;
pub(crate) mod scan_patterns;
pub mod talk_commands;

use {
    assemble::AsmFolder,
    std::{env, sync::LazyLock},
};

static ASM_LIB_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/eldenring.bin"));

pub(crate) static ASM: LazyLock<AsmFolder> =
    LazyLock::new(|| bincode::deserialize(ASM_LIB_BYTES).unwrap());
