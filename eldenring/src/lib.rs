mod ai;
pub mod attach;
pub mod chr_ins;
pub mod emevd;
pub mod event;
pub mod game_state;
pub mod item;
mod offsets;
mod phase_transition;
pub mod player;
mod pointer_cache;
pub mod resources;
pub mod target;
pub mod travel;
pub mod utility;
pub mod utils;

use {
    crate::{
        game_state::{GAME_STATE, STATE_FLAGS},
        pointer_cache::POINTER_CACHE,
    },
    shared::command_registry::{CommandRegistration, CommandRegistry},
    std::sync::atomic::Ordering,
};
pub use {
    attach::attach,
    offsets::code_cave::pointers as cave_pointers,
    pointer_cache::{get_pointers, load_all_pointers},
};

mod mem {
    gubtool_core::declare_mem_functions!(Game::EldenRing);
    gubtool_core::declare_x64_specifics!();
}

#[linkme::distributed_slice]
static ELDEN_RING_COMMANDS: [CommandRegistration];
pub const COMMAND_REGISTER: CommandRegistry = CommandRegistry::new(&ELDEN_RING_COMMANDS);

#[macro_export]
macro_rules! link_command {
    ($struct_path:expr, $struct_name:ident $(, $cli_name:expr)?) => {
        shared::link_command!(ELDEN_RING_COMMANDS, $struct_path, $struct_name $(, $cli_name)?);
    };
}

pub fn init() {
    GAME_STATE.init();
    STATE_FLAGS.update();
}

pub fn reset() {
    POINTER_CACHE.reset_pointers();
    player::player().update();
    player::torrent().update();
    STATE_FLAGS.reset();
}

pub fn update() {
    STATE_FLAGS.update();
    GAME_STATE.update();
    target::target().update();
    player::player_game_data().read();
}

pub fn is_player_loaded() -> bool {
    GAME_STATE.loaded.load(Ordering::Relaxed)
}

pub fn is_dlc_available() -> bool {
    GAME_STATE.dlc.load(Ordering::Relaxed)
}
