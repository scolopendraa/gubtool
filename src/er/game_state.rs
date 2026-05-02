use crate::er::{
    chr_ins::{ChrInsExt, chr_ins_from_handle},
    event,
    mem::*,
    offsets::{code_cave, game_data_man, menu_man, world_chr_man},
    player::{self, player_ins, torrent_ins},
    target::target_ins,
    utils::{is_dlc_available, is_version_dlc_compat},
};
use anyhow::Result;

#[derive(Default)]
pub struct GameStateHandler {
    pub loaded: bool,
    has_invoked_load_delayed: bool,
    has_invoked_loaded: bool,
    pub dlc: bool,
}

impl GameStateHandler {
    pub fn new() -> Self {
        Self {
            loaded: true,
            has_invoked_load_delayed: true,
            has_invoked_loaded: true,
            dlc: is_version_dlc_compat(),
        }
    }

    pub fn poll(&mut self) -> Result<()> {
        if is_loaded().unwrap_or_default() {
            if !self.has_invoked_load_delayed && self.has_invoked_loaded && is_faded_in()? {
                self.on_load_delayed()?;
                self.has_invoked_load_delayed = true;
            }
            if !self.loaded {
                self.loaded = true;
                self.on_loaded()?;
                self.has_invoked_loaded = true;

                if is_new_game().unwrap_or_default() {
                    self.on_new_game()?;
                }
            }
        } else if self.loaded {
            self.on_unloaded();
            self.has_invoked_load_delayed = false;
            self.has_invoked_loaded = false;
            self.loaded = false;
        }
        Ok(())
    }
    fn on_loaded(&mut self) -> Result<()> {
        if get_state_flag(GameStateFlags::PlayerNoDamage) {
            player_ins().set_no_damage(true)?;
        }
        if get_state_flag(GameStateFlags::TitleCards) {
            event::disable_title_card()?;
        }
        if get_state_flag(GameStateFlags::RuneArc) {
            player::set_rune_arc(true)?;
        }

        let handle = read::<u64>(code_cave::base() + code_cave::TARGET_HANDLE)?;
        write::<u64>(code_cave::base() + code_cave::TARGET_POINTER, chr_ins_from_handle(handle).unwrap_or_default())?;

        self.dlc = is_dlc_available()?;
        Ok(())
    }

    fn on_load_delayed(&self) -> Result<()> {
        if get_state_flag(GameStateFlags::Rfbs) {
            player::set_rfbs()?;
        }
        if get_state_flag(GameStateFlags::TorrentNoDeath) {
            torrent_ins().set_no_death(true)?;
        }
        Ok(())
    }

    fn on_unloaded(&self) {
        write::<u64>(code_cave::base() + code_cave::TARGET_HANDLE, target_ins().handle().unwrap_or_default()).ok();
    }

    fn on_new_game(&self) -> Result<()> {
        Ok(())
    }
}

pub fn is_loaded() -> Result<bool> {
    read::<u64>(world_chr_man::base())
        .and_then(|addr| read::<u64>(addr + world_chr_man::player_ins()))
        .map(|val| val != 0)
}

fn is_faded_in() -> Result<bool> {
    read::<u64>(menu_man::base())
        .and_then(|addr| read::<u8>(addr + menu_man::is_fading()))
        .map(|val| val == 0x0)
}

fn is_new_game() -> Result<bool> {
    read::<u64>(game_data_man::base())
        .and_then(|addr| read::<u64>(addr + game_data_man::IGT))
        .map(|val| val < 5000)
}

#[repr(u64)]
pub enum GameStateFlags {
    PlayerNoDamage = 0x0,
    Rfbs = 0x1,
    TitleCards = 0x2,
    RuneArc = 0x3,
    TorrentNoDeath = 0x4,
}

pub fn get_state_flag(flag_offset: GameStateFlags) -> bool {
    read::<u8>(code_cave::base() + code_cave::STATE_HANDLER_FLAGS + flag_offset as u64)
        .map(|val| val == 0x1)
        .unwrap_or_default()
}

pub fn set_state_flag(flag_offset: GameStateFlags, state: bool) -> Result<()> {
    write::<u8>(code_cave::base() + code_cave::STATE_HANDLER_FLAGS + flag_offset as u64, state as u8)
}
