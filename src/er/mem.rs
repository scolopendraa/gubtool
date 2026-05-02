use crate::{
    core::{
        attach::{Game, game},
        sys::*,
    },
    er::offsets::{self, code_cave},
};
use anyhow::{Result, ensure};
use pelite::Pod;
use std::sync::{LazyLock, Mutex};

pub static ITEM_SPAWN_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
pub static EXECUTE_EMEVD_COMMAND_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[track_caller]
pub fn read<T: Pod>(address: u64) -> Result<T> {
    ensure!(game() == Game::EldenRing, "Not attached to Elden Ring");
    read_unsafe(address)
}

#[track_caller]
pub fn write<T: Pod>(address: u64, value: T) -> Result<()> {
    ensure!(game() == Game::EldenRing, "Not attached to Elden Ring");
    write_unsafe(address, value)
}

#[track_caller]
pub fn write_bytes(address: u64, data: &[u8]) -> Result<()> {
    ensure!(game() == Game::EldenRing, "Not attached to Elden Ring");
    write_bytes_unsafe(address, data)
}

pub fn run_thread(address: u64) -> Result<()> {
    ensure!(game() == Game::EldenRing, "Not attached to Elden Ring");
    run_win_thread_wait(
        address,
        code_cave::base() + code_cave::RUN_THREAD_ASM,
        offsets::kernel32_create_thread(),
        false,
    )
}

pub fn append_flag_setter(location: u64, asm_head: &[u8]) -> Result<Vec<u8>> {
    append_64bit_flag_setter(location, asm_head)
}

pub fn is_bit_set(address: u64, mask: u8) -> Result<bool> {
    read::<u8>(address)
        .map(|byte| byte & mask != 0)
}

pub fn set_bit(address: u64, mask: u8, value: bool) -> Result<()> {
    let current_byte = read::<u8>(address)?;
    let new_byte = match value {
        true => current_byte | mask,
        false => current_byte & !mask,
    };
    write::<u8>(address, new_byte)
}