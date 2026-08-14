use {
    assemble::{AsmFolder, AsmFunction},
    gubtool_core::attached::is_32,
    std::sync::LazyLock,
};

pub mod bonfires;
pub mod bosses;
pub mod chr_names;
pub mod items;
pub mod map_ids;
pub mod menus;
pub mod scholar_patterns;
pub mod vanilla_patterns;

static VANILLA_ASM_LIB_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vanilla.bin"));
static SCHOLAR_ASM_LIB_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/scholar.bin"));

static SCHOLAR_ASM: LazyLock<AsmFolder> =
    LazyLock::new(|| bincode::deserialize(SCHOLAR_ASM_LIB_BYTES).unwrap());
static VANILLA_ASM: LazyLock<AsmFolder> =
    LazyLock::new(|| bincode::deserialize(VANILLA_ASM_LIB_BYTES).unwrap());

pub fn asm_function(name: &'static str) -> AsmFunction {
    if is_32() {
        VANILLA_ASM.get_function(name)
    } else {
        SCHOLAR_ASM.get_function(name)
    }
}

pub fn print_asm_sizes() {
    println!("Dark Souls II");
    SCHOLAR_ASM.print_function_sizes();
    println!("\n");
}
