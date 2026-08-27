pub mod attach;
pub mod bonfire;
pub mod chr_ctrl;
pub mod covenant;
mod enemy;
pub mod event;
pub mod game_state;
pub mod item;
pub mod menu;
mod offsets;
pub mod player;
mod pointer_cache;
pub mod resources;
mod speffect;
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
    gubtool_core::declare_mem_functions!(Game::DarkSouls2);
    gubtool_core::declare_x86_specifics!();
}

#[linkme::distributed_slice]
static DARK_SOULS_2_COMMANDS: [CommandRegistration];
pub const COMMAND_REGISTER: CommandRegistry = CommandRegistry::new(&DARK_SOULS_2_COMMANDS);

#[macro_export]
macro_rules! link_command {
    ($struct_path:expr, $struct_name:ident $(, $cli_name:expr)?) => {
        shared::link_command!(DARK_SOULS_2_COMMANDS, $struct_path, $struct_name $(, $cli_name)?);
    };
}

pub fn init() {
    GAME_STATE.init();
    STATE_FLAGS.update();
}

pub fn reset() {
    POINTER_CACHE.reset_pointers();
    player::player().update();
    STATE_FLAGS.reset();
}

pub fn update() {
    STATE_FLAGS.update();
    GAME_STATE.update();
    let _ = enemy::act_logger().update();
    target::target().update();
    player::STATS.write().unwrap().read();
}

pub fn is_player_loaded() -> bool {
    GAME_STATE.loaded.load(Ordering::Relaxed)
}
