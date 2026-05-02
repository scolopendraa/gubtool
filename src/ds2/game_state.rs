use anyhow::Result;

use crate::ds2::{
    chr_ctrl::ChrCtrlExt,
    mem::*,
    offsets::{code_cave, game_manager_imp},
    player,
};

pub struct GameStateHandler {
    pub loaded: bool,
    has_invoked_loaded: bool,
    has_invoked_load_delayed: bool,
}

impl GameStateHandler {
    pub fn new() -> Self {
        Self {
            loaded: true,
            has_invoked_loaded: true,
            has_invoked_load_delayed: true,
        }
    }
    pub fn poll(&mut self) -> Result<()> {
        if is_loaded().unwrap_or_default() {
            if !self.has_invoked_load_delayed && !is_loading_screen().unwrap_or_default() {
                self.on_load_delayed()?;
                self.has_invoked_load_delayed = true;
            }
            if !self.loaded {
                self.loaded = true;
                self.on_loaded()?;
                self.has_invoked_loaded = true;
            }
        } else if self.loaded {
            self.on_unloaded()?;
            self.has_invoked_load_delayed = false;
            self.has_invoked_loaded = false;
            self.loaded = false;
        }
        Ok(())
    }
    fn on_loaded(&self) -> Result<()> {
        if get_state_flag(GameStateFlags::PlayerNoDeath) {
            player::player_ctrl().set_no_death(true)?
        }
        if get_state_flag(GameStateFlags::PlayerNoDamage) {
            //player::player_ctrl().set_no_damage(true)?
        }
        Ok(())
    }
    fn on_load_delayed(&self) -> Result<()> {
        Ok(())
    }
    fn on_unloaded(&self) -> Result<()> {
        Ok(())
    }
}

#[repr(u64)]
pub enum GameStateFlags {
    PlayerNoDeath = 0x0,
    PlayerNoDamage = 0x1,
}

pub fn get_state_flag(flag_offset: GameStateFlags) -> bool {
    read::<u8>(code_cave::base() + code_cave::STATE_HANDLER_FLAGS + flag_offset as u64)
        .map(|val| val == 0x1)
        .unwrap_or(false)
}

pub fn set_state_flag(flag_offset: GameStateFlags, state: bool) -> Result<()> {
    write::<u8>(
        code_cave::base() + code_cave::STATE_HANDLER_FLAGS + flag_offset as u64,
        state as u8,
    )
}

pub fn is_loading_screen() -> Result<bool> {
    read::<u64>(game_manager_imp::base())
        .and_then(|addr| read::<u8>(addr + game_manager_imp::loading_flag()))
        .map(|val| val == 1)
}

pub fn is_loaded() -> Result<bool> {
    follow_pointers(
        &[game_manager_imp::base(), game_manager_imp::player_ctrl()],
        true,
    )
    .map(|val| val != 0)
}
