use gubtool_core::{
    address::Address,
    attached::game,
    game_version::Game,
    slice_ops::*,
    sys::{
        error::{ProcResult, ProcessError},
        *,
    },
};
use pelite::Pod;
use std::sync::{LazyLock, Mutex};

pub static ITEM_SPAWN_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
pub static EXECUTE_EMEVD_COMMAND_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[track_caller]
pub fn read<T: Pod>(address: impl Address) -> ProcResult<T> {
    ensure_eldenring()?;
    read_unsafe(address)
}

#[track_caller]
pub fn write<T: Pod>(address: impl Address, value: T) -> ProcResult {
    ensure_eldenring()?;
    write_unsafe(address, value)
}

#[track_caller]
pub fn write_bytes(address: impl Address, data: &[u8]) -> ProcResult {
    ensure_eldenring()?;
    write_bytes_unsafe(address, data)
}

#[track_caller]
pub fn read_bytes(address: impl Address, len: usize) -> ProcResult<Vec<u8>> {
    ensure_eldenring()?;
    let mut buf = vec![0u8; len];
    for i in 0..len {
        buf[i] = read_unsafe::<u8>(address.add_offset(i as u64))?;
    }
    Ok(buf)
}

pub fn spawn_thread_join(thread_start_address: impl Address, thread_code: Vec<u8>) -> ProcResult {
    ensure_eldenring()?;
    #[cfg(unix)]
    gubtool_core::sys::spawn_thread_join(
        crate::offsets::code_cave::CaveOffset::RunThreadAsm.addr(),
        thread_start_address,
        thread_code,
        crate::offsets::module_offsets::ExternalFunctionPointer::Kernel32CreateThread,
        crate::offsets::module_offsets::ExternalFunctionPointer::Kernel32CloseHandle,
    )?;
    #[cfg(windows)]
    gubtool_core::sys::spawn_thread_join(
        thread_start_address,
        thread_code,
    )?;
    Ok(())
}

pub fn is_bit_set(address: impl Address, mask: u8) -> ProcResult<bool> {
    read::<u8>(address)
        .map(|byte| byte & mask != 0)
}

pub fn set_bit(address: impl Address, mask: u8, value: bool) -> ProcResult {
    let current_byte = read::<u8>(address)?;
    let new_byte = match value {
        true => current_byte | mask,
        false => current_byte & !mask,
    };
    write::<u8>(address, new_byte)
}

pub fn install_hook(code: &[u8], code_location: impl Address, hook_location: impl Address, original_instruction_size: u64) -> ProcResult {
    let hookbytes = get_hook_bytes(code_location, hook_location, original_instruction_size)?;
    write_bytes(code_location, &code)?;
    write_bytes(hook_location, &hookbytes)
}

fn ensure_eldenring() -> ProcResult {
    if game() != Some(Game::EldenRing) {
        Err(ProcessError::InvalidGame { expected: Game::EldenRing })
    } else {
        Ok(())
    }
}