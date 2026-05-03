use std::sync::{LazyLock, Mutex};

use crate::{
    core::{
        attach::{Game, game},
        sys::*,
    },
    ds2::{
        offsets::{self, code_cave},
        utils::is_scholar,
    },
};
use anyhow::{Result, ensure};
use pelite::Pod;

pub static ITEM_SPAWN_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
pub static MASS_SPAWN_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
pub static TRAVEL_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[track_caller]
pub fn read<T: Pod>(address: u64) -> Result<T> {
    ensure!(game() == Game::DarkSoulsII, "Not attached to Dark Souls II");
    read_unsafe(address)
}

#[track_caller]
pub fn write<T: Pod>(address: u64, value: T) -> Result<()> {
    ensure!(game() == Game::DarkSoulsII, "Not attached to Dark Souls II");
    write_unsafe(address, value)
}

#[track_caller]
pub fn write_bytes(address: u64, data: &[u8]) -> Result<()> {
    ensure!(game() == Game::DarkSoulsII, "Not attached to Dark Souls II");
    write_bytes_unsafe(address, data)
}

pub fn run_thread(address: u64) -> Result<()> {
    ensure!(game() == Game::DarkSoulsII, "Not attached to Dark Souls II");
    if is_scholar() {
        run_win_thread_wait(
            address,
            code_cave::base() + code_cave::RUN_THREAD_ASM,
            offsets::kernel32_create_thread(),
            false,
        )
    } else {
        run_win_thread_wait(
            address,
            code_cave::base() + code_cave::RUN_THREAD_ASM,
            offsets::kernel32_create_thread(),
            true,
        )
    }
}

pub fn run_thread_release(address: u64) -> Result<()> {
    ensure!(game() == Game::DarkSoulsII, "Not attached to Dark Souls II");
    if is_scholar() {
        run_win64_thread(
            address,
            code_cave::base() + code_cave::RUN_THREAD_ASM,
            offsets::kernel32_create_thread()
        )
    } else {
        run_win32_thread(
            address,
            code_cave::base() + code_cave::RUN_THREAD_ASM,
            offsets::kernel32_create_thread()
        )
    }
}

pub fn append_flag_setter(address: u64, asm_head: &[u8]) -> Result<Vec<u8>> {
    ensure!(game() == Game::DarkSoulsII, "Not attached to Dark Souls II");
    if is_scholar() {
        append_64bit_flag_setter(address, asm_head)
    } else {
        append_32bit_flag_setter(address, asm_head)
    }
}

pub fn follow_pointers(pointers: &[u64], read_final: bool) -> Result<u64> {
    let mut pointer = 0u64;
    let (last, rest) = pointers.split_last().unwrap();
    if is_scholar() {
        for offset in rest {
            pointer = read::<u64>(pointer + offset)?
        }
        if read_final {
            pointer = read::<u64>(pointer + last)?
        } else {
            pointer = pointer + last
        }
    } else {
        for offset in rest {
            pointer = read::<u32>(pointer + offset)? as u64
        }
        if read_final {
            pointer = read::<u32>(pointer + last)? as u64
        } else {
            pointer = pointer + last
        }
    }
    Ok(pointer)
}