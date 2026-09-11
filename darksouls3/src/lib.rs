use {
    shared::command_registry::{CommandRegistration, CommandRegistry},
    std::sync::atomic::Ordering,
};

mod attach;
pub mod chr_ins;
pub mod esd;
pub mod event;
mod game_state;
pub mod item;
mod obj_refs;
mod offsets;
pub mod player;
mod pointer_cache;
pub mod resources;
pub mod target;
pub mod travel;
pub mod utility;
pub mod utils;

use crate::{game_state::GAME_STATE, obj_refs::{PLAYER, TARGET}, pointer_cache::pointer_cache};
pub use {
    attach::attach,
    offsets::code_cave::pointers as cave_pointers,
    pointer_cache::{global_pointers, load_all_pointers, player_pointers, target_pointers},
};

mod mem {
    gubtool_core::declare_mem_functions!(DarkSouls3);
    gubtool_core::declare_x64_specifics!();
}

#[linkme::distributed_slice]
static DARK_SOULS_3_COMMANDS: [CommandRegistration];
pub const COMMAND_REGISTER: CommandRegistry = CommandRegistry::new(&DARK_SOULS_3_COMMANDS);

#[macro_export]
macro_rules! link_command {
    ($struct_path:expr, $struct_name:ident $(, $cli_name:expr)?) => {
        shared::link_command!(DARK_SOULS_3_COMMANDS, $struct_path, $struct_name $(, $cli_name)?);
    };
}

pub fn init() {
    GAME_STATE.init();
}

pub fn reset() {
    pointer_cache().clear_pointers();
    PLAYER.update();
}

pub fn update() {
    GAME_STATE.update();
    TARGET.update();
    player::player_game_data().read();
}

pub fn is_player_loaded() -> bool {
    GAME_STATE.loaded.load(Ordering::Relaxed)
}
