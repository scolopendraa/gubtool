use {
    crate::{
        chr_ins::ChrIns,
        emevd,
        mem::*,
        offsets::{
            ChainReadExt,
            code_cave::CaveAddr,
            cs_dlc_imp,
            game_data_man,
            menu_man,
            module_offsets::BasePointer,
            world_chr_man,
        },
        player,
        pointer_cache::POINTER_CACHE,
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
    pub loaded:               AtomicBool,
    has_invoked_load_delayed: AtomicBool,
    pub dlc:                  AtomicBool,
}

#[derive(Default)]
pub struct StateFlags {
    buffer: Mutex<[u8; 0x20]>,
}

impl GameState {
    pub fn init(&self) {
        let loaded = is_loaded();
        self.loaded.store(loaded, Ordering::Relaxed);
        self.has_invoked_load_delayed
            .store(loaded, Ordering::Relaxed);
        self.dlc.store(is_dlc_available(), Ordering::Relaxed);
    }

    pub fn update(&self) {
        if is_loaded() {
            if !self.has_invoked_load_delayed.load(Ordering::Relaxed) && is_faded_in() {
                self.on_load_delayed();
                self.has_invoked_load_delayed.store(true, Ordering::Relaxed);
            }
            if !self.loaded.load(Ordering::Relaxed) {
                self.loaded.store(true, Ordering::Relaxed);
                self.on_loaded();

                if is_new_game() {
                    self.on_new_game();
                }
            }
        } else if self.loaded.load(Ordering::Relaxed) {
            self.on_unloaded();
            self.has_invoked_load_delayed
                .store(false, Ordering::Relaxed);
            self.loaded.store(false, Ordering::Relaxed);
        }
    }

    fn on_loaded(&self) {
        self.dlc.store(is_dlc_available(), Ordering::Relaxed);
        STATE_FLAGS.on_loaded();
        POINTER_CACHE.reset_pointers();
        player::player().update();
        player::torrent().update();
        let _ = restore_target();
    }

    fn on_load_delayed(&self) {
        STATE_FLAGS.on_load_delayed();
    }

    fn on_unloaded(&self) {
        POINTER_CACHE.reset_pointers();
        player::player().update();
        player::torrent().update();
        let _ = save_target();
    }

    fn on_new_game(&self) {}
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
        if self.is_flag(StateFlag::PlayerNoDamage) {
            let _ = player::NoDamage.set_in_game(true);
        }
        if self.is_flag(StateFlag::TitleCards) {
            let _ = emevd::disable_title_card();
        }
        if self.is_flag(StateFlag::RuneArc) {
            let _ = player::RuneArc.set_in_game(true);
        }
        if self.is_flag(StateFlag::StutterFix) {
            let _ = utility::StutterFix.set_in_game(true);
        }
        if self.is_flag(StateFlag::Hitboxes) {
            let _ = utility::DrawHitboxes.set_in_game(true);
        }
    }

    pub fn on_load_delayed(&self) {
        if self.is_flag(StateFlag::Rfbs) {
            let _ = player::SetRfbsOnLoad.apply_in_game();
        }
        if self.is_flag(StateFlag::TorrentNoDeath) {
            let _ = player::TorrentNoDeath.set_in_game(true);
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

fn save_target() -> SysResult {
    let handle = target().chr_ins()?.handle()?;
    write::<u64>(CaveAddr::SavedHandle, handle)
}

fn restore_target() -> anyhow::Result<()> {
    let handle = read::<u64>(CaveAddr::SavedHandle)?;
    if let Some(chr_ins) = ChrIns::from_handle(handle) {
        chr_ins.set_as_target(&mut target())?;
    }
    Ok(())
}

#[repr(u64)]
pub enum StateFlag {
    PlayerNoDamage = 0x0,
    Rfbs           = 0x1,
    TitleCards     = 0x2,
    RuneArc        = 0x3,
    TorrentNoDeath = 0x4,
    StutterFix     = 0x5,
    Hitboxes       = 0x6,
}

fn is_loaded() -> bool {
    read::<u64>(BasePointer::WorldChrMan)
        .read_offset(world_chr_man::player_ins())
        .map(|val| val != 0)
        .unwrap_or_default()
}

fn is_faded_in() -> bool {
    read::<u64>(BasePointer::MenuMan)
        .add_offset(menu_man::is_fading())
        .read::<u8>()
        .map(|val| val == 0x0)
        .unwrap_or_default()
}

fn is_new_game() -> bool {
    read::<u64>(BasePointer::GameDataMan)
        .add_offset(game_data_man::IGT)
        .read::<u64>()
        .map(|val| val < 5000)
        .unwrap_or_default()
}

fn is_dlc_available() -> bool {
    read::<u64>(BasePointer::CsDlcImp)
        .add_offset(cs_dlc_imp::BYTE_FLAGS)
        .add_offset(cs_dlc_imp::flags::DLC_CHECK)
        .read::<u8>()
        .map(|val| val == 1)
        .unwrap_or_default()
}
