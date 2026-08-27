use {
    crate::{
        POINTER_CACHE,
        enemy,
        mem::*,
        offsets::{
            ChainReadExt,
            code_cave::CaveAddr,
            game_manager_imp,
            module_offsets::BasePointer,
        },
        player::{self, player},
        target::target,
        utility,
    },
    gubtool_core::{address::Address, slice_ops::*, sys::sys_error::SysResult},
    std::sync::{
        LazyLock,
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

pub(crate) static GAME_STATE: LazyLock<GameState> = LazyLock::new(GameState::default);

pub(crate) static STATE_FLAGS: LazyLock<StateFlags> = LazyLock::new(StateFlags::default);

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
        POINTER_CACHE.reset_pointers();
        player().update();
        STATE_FLAGS.on_loaded();
        let _ = enemy::clear_disabled_targets();
    }
    fn on_unloaded(&self) {
        POINTER_CACHE.reset_pointers();
        player().update();
        target().clear();
        enemy::act_logger().clear();
        STATE_FLAGS.on_unloaded();
    }
}

#[derive(Default)]
pub struct StateFlags {
    buffer: Mutex<[u8; 0x20]>,
}

impl StateFlags {
    pub fn update(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        *buffer = read::<[u8; 0x20]>(CaveAddr::StateHandlerFlags).unwrap_or_default();
    }

    pub fn is_flag(&self, flag_offset: StateFlag) -> bool {
        read_from_slice::<u8>(&*self.buffer.lock().unwrap(), flag_offset as u64).unwrap_or_default()
            != 0x0
    }

    pub fn on_loaded(&self) {
        if self.is_flag(StateFlag::PlayerNoDeath) {
            let _ = player::player()
                .chr_ctrl()
                .and_then(|chr| chr.set_no_death(true));
        }
    }

    pub fn on_unloaded(&self) {
        if self.is_flag(StateFlag::FastQuitout) {
            let _ = utility::FastQuitout.set_hook(true);
        } else if utility::FastQuitout.is_hook().unwrap_or(false) {
            let _ = utility::FastQuitout.set_hook(false);
        }
    }

    pub fn reset(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        *buffer = [0x0; 0x20];
    }
}

pub fn is_flag(flag_offset: StateFlag) -> bool {
    STATE_FLAGS.is_flag(flag_offset)
}

pub fn set_flag(flag_offset: StateFlag, state: bool) -> SysResult {
    write::<u8>(CaveAddr::StateHandlerFlags.add(flag_offset as u64), state as u8)
}

#[repr(u64)]
pub enum StateFlag {
    PlayerNoDeath = 0x0,
    FastQuitout   = 0x1,
}

fn is_loading_screen() -> bool {
    read_address(BasePointer::GameManagerImp)
        .add_offset(game_manager_imp::LOADING_FLAG)
        .read::<u8>()
        .map(|val| val == 0x1)
        .unwrap_or_default()
}

fn is_loaded() -> bool {
    read_address(BasePointer::GameManagerImp)
        .read_offset(game_manager_imp::PLAYER_CTRL)
        .map(|val| val != 0x0)
        .unwrap_or_default()
}
