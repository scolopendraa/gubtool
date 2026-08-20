pub mod attach;
pub mod bonfire;
pub mod chr_ctrl;
pub mod covenant;
pub mod event;
pub mod game_state;
pub mod item;
pub mod menu;
mod offsets;
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

pub fn init() {
    GAME_STATE.init();
}

pub fn reset() {
    POINTER_CACHE.reset_pointers();
    player::player().update();
    STATE_FLAGS.reset();
}

pub fn update() {
    STATE_FLAGS.update();
    GAME_STATE.update();
    let _ = target::act_logger().update();
    target::target().update();
    player::STATS.write().unwrap().read();
}

pub fn is_player_loaded() -> bool {
    GAME_STATE.loaded.load(Ordering::Relaxed)
}
