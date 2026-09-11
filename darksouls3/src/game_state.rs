use {
    crate::{obj_refs::PLAYER, offsets::module_offsets::BasePointer, pointer_cache::pointer_cache},
    gubtool_core::{address::ChainReadExt, sys::read_pointer},
    std::sync::{
        LazyLock,
        atomic::{AtomicBool, Ordering},
    },
};

pub(crate) static GAME_STATE: LazyLock<GameState> = LazyLock::new(GameState::default);

#[derive(Default)]
pub struct GameState {
    pub loaded: AtomicBool,
}

impl GameState {
    pub fn init(&self) {
        self.loaded.store(is_loaded(), Ordering::Relaxed);
    }
    pub fn update(&self) {
        if is_loaded() {
            if !self.loaded.load(Ordering::Relaxed) {
                self.loaded.store(true, Ordering::Relaxed);
                self.on_loaded();
            }
        } else if self.loaded.load(Ordering::Relaxed) {
            self.on_unloaded();
            self.loaded.store(false, Ordering::Relaxed);
        }
    }
    fn on_loaded(&self) {
        pointer_cache().clear_pointers();
        PLAYER.update();
    }
    fn on_unloaded(&self) {
        pointer_cache().clear_pointers();
        PLAYER.update();
    }
}

pub fn is_loaded() -> bool {
    read_pointer(BasePointer::WorldChrManImp)
        .read_ptr(0x80)
        .map(|val| val != 0x0)
        .unwrap_or_default()
}
