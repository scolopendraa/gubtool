use anyhow::{Result, anyhow, bail, ensure};
use pelite::Pod;
use std::{
    any::type_name,
    hint::spin_loop,
    io::{IoSlice, IoSliceMut},
    sync::{LazyLock, Mutex},
    time::{Duration, Instant},
    slice, thread,
};
use nix::{
    sys::{
        ptrace::{self, regset::{NT_PRFPREG, NT_PRSTATUS}},
        uio::{RemoteIoVec, process_vm_readv, process_vm_writev},
        wait::waitpid,
    },
    unistd::Pid
};
use crate::{
    core::attach::ATTACHED_PROCESS,
    offsets::{self, code_cave}
};

static PTRACE_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
pub static ITEM_SPAWN_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
pub static EXECUTE_EMEVD_COMMAND_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

fn run_win_thread(address: u64) -> Result<()> {
    let pid = unsafe { ATTACHED_PROCESS.pid };
    let start = Instant::now();
    let timeout = Duration::from_millis(50);

    loop {
        if start.elapsed() > timeout {
            bail!("Failed to spawn thread at {:#X} within {:#?}", address, timeout)
        }
        let handle = PTRACE_MUTEX.lock().unwrap();
        ptrace::attach(pid)?;
        waitpid(pid, None)?;

        let start = unsafe { ATTACHED_PROCESS.module_handle };
        let rip = ptrace::getregs(pid)?.rip;
        if start < rip && rip < start + 0x5E03000 {
            let original_regs = ptrace::getregset::<NT_PRSTATUS>(pid)?;
            let original_fp_regs = ptrace::getregset::<NT_PRFPREG>(pid)?;
            let mut regs = original_regs;

            regs.rcx = 0;                                   // lpThreadAttributes
            regs.rdx = 0;                                   // dwStackSize
            regs.r9 = 0;                                    // lpParameter
            regs.r8 = address;                              // lpStartAddress
            regs.rsp = (regs.rsp - 0x100) & !0xFu64;
            regs.rax = read::<u64>(offsets::kernel32_create_thread())?;
            let asm: [u8; 29] = [
                0x48, 0x83, 0xEC, 0x30,                     // sub rsp,0x30
                0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00,   // mov qword ptr [rsp+20h],0x0 (dwCreationFlags)
                0x00, 0x00,
                0x48, 0xC7, 0x44, 0x24, 0x28, 0x00, 0x00,   // mov qword ptr [rsp+28h],0x0 (lpThreadId)
                0x00, 0x00,
                0xFF, 0xD0,                                 // call
                0x48, 0x83, 0xC4, 0x30,                     // add rsp,0x30
                0xCC,                                       // int3
            ];
            let location = code_cave::base() + code_cave::RUN_THREAD_ASM;
            write_bytes(location, &asm)?;

            regs.rip = location;
            ptrace::setregset::<NT_PRSTATUS>(pid, regs)?;

            ptrace::cont(pid, None)?;
            waitpid(pid, None)?;

            ptrace::setregset::<NT_PRSTATUS>(pid, original_regs)?;
            ptrace::setregset::<NT_PRFPREG>(pid, original_fp_regs)?;
            ptrace::detach(pid, None).map_err(anyhow::Error::msg)?;
            return Ok(())
        } else {
            ptrace::detach(pid, None)?;
            drop(handle);
            thread::sleep(Duration::from_micros(10));
        }
    }
}

pub fn run_win_thread_wait(address: u64) -> Result<()> {
    let running_flag = address.saturating_sub(1);
    write::<u8>(running_flag, 0x1)?;
    run_win_thread(address)?;
    let start = Instant::now();
    let timeout = Duration::from_millis(100);
    loop {
        if read::<u8>(running_flag)? == 0x0 {
            return Ok(())
        }
        if start.elapsed() > timeout {
            bail!("Thread did not return within {:#?}", timeout)
        }
        spin_loop();
    }
}

pub fn append_flag_setter(location: u64, asm_head: &[u8]) -> Result<Vec<u8>> {
    let mut asm_tail = [
        0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00,   // movabs rax,running_flag
        0x00, 0x00, 0x00,
        0xC6, 0x00, 0x00,                           // mov BYTE PTR [rax],0x0
        0xC3,                                       // ret
    ];
    write_to_slice::<u64>(&mut asm_tail, 2, location.saturating_sub(1))?;
    let mut asm = asm_head[..asm_head.len().saturating_sub(1)].to_vec();
    asm.extend_from_slice(&asm_tail);
    Ok(asm)
}

#[track_caller]
pub fn read<T: Pod>(address: u64) -> Result<T> {
    unsafe {
        ensure!(ATTACHED_PROCESS.pid != Pid::from_raw(-1), "Game not found");
        let file_location = std::panic::Location::caller();
        let mut value = std::mem::zeroed::<T>();
        let size = std::mem::size_of::<T>();
        let local_iov = IoSliceMut::new(slice::from_raw_parts_mut(&mut value as *mut T as *mut u8, size));
        let remote_iov = RemoteIoVec { base: address as usize, len: size };
        let nread = process_vm_readv(ATTACHED_PROCESS.pid, &mut [local_iov], &[remote_iov])
            .map_err(|err| anyhow!(
                    "{}:{}: failed to read {} at module base + {:#X} ({})",
                    file_location.file(),
                    file_location.line(),
                    type_name::<T>(),
                    address.saturating_sub(ATTACHED_PROCESS.module_handle),
                    err))?;
        if nread != size {
            bail!(
                "{}:{}: partial read at module base + {:#X}. Tried to read {}, read {} bytes",
                file_location.file(),
                file_location.line(),
                address.saturating_sub(ATTACHED_PROCESS.module_handle),
                type_name::<T>(),
                nread);
        }
        Ok(value)
    }
}

#[track_caller]
pub fn write<T: Pod>(address: u64, value: T) -> Result<()> {
    unsafe {
        ensure!(ATTACHED_PROCESS.pid != Pid::from_raw(-1), "Game not found");
        let file_location = std::panic::Location::caller();
        let size = std::mem::size_of::<T>();
        let local_iov = IoSlice::new(slice::from_raw_parts(&value as *const T as *const u8, size));
        let remote_iov = RemoteIoVec { base: address as usize, len: size };
        let nwritten = process_vm_writev(ATTACHED_PROCESS.pid, &[local_iov], &[remote_iov])
            .map_err(|err| anyhow!(
                    "{}:{}: failed to write {} at module base + {:#X} ({})",
                    file_location.file(),
                    file_location.line(),
                    type_name::<T>(),
                    address.saturating_sub(ATTACHED_PROCESS.module_handle),
                    err))?;
        if nwritten != size {
            bail!(
                "{}:{}: partial write at module base + {:#X}. Tried to write {}, wrote {} bytes",
                file_location.file(),
                file_location.line(),
                address.saturating_sub(ATTACHED_PROCESS.module_handle),
                type_name::<T>(),
                nwritten);
        }
        Ok(())
    }
}

#[track_caller]
pub fn write_bytes(address: u64, data: &[u8]) -> Result<()> {
    unsafe {
        if ATTACHED_PROCESS.pid == Pid::from_raw(-1) {
            bail!("Game not found")
        }
    }
    let file_location = std::panic::Location::caller();
    let local_iov = IoSlice::new(data);
    let remote_iov = RemoteIoVec { base: address.try_into()? , len: data.len() };
    let nwritten = process_vm_writev(unsafe { ATTACHED_PROCESS.pid }, &[local_iov], &[remote_iov])
        .map_err(|err| anyhow!(
                "{}:{}: failed to write {} bytes at module base + {:#X} ({})",
                file_location.file(),
                file_location.line(),
                data.len(),
                address.saturating_sub(unsafe{ ATTACHED_PROCESS.module_handle }),
                err))?;
    if nwritten != data.len() {
        bail!(
            "{}:{}: partial write at module base + {:#X}. Tried to write {}, wrote {} bytes",
            file_location.file(),
            file_location.line(),
            address.saturating_sub(unsafe{ ATTACHED_PROCESS.module_handle }),
            data.len(),
            nwritten);
    }
    Ok(())
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

#[track_caller]
pub fn read_from_slice<T: Pod>(array: &[u8], offset: u64) -> Result<T> {
    let file_location = std::panic::Location::caller();
    let offset = offset as usize;
    let size = std::mem::size_of::<T>();
    let end = offset.checked_add(size)
        .ok_or_else(|| anyhow::anyhow!("{}:{}: offset overflow",
            file_location.file(),
            file_location.line(),
            ))?;
    let bytes = array
        .get(offset..end)
        .ok_or_else(|| anyhow::anyhow!("{}:{}: out of bounds read",
            file_location.file(),
            file_location.line(),
            ))?;
    Ok(unsafe {
        std::ptr::read_unaligned(bytes.as_ptr() as *const T)
    })
}

#[track_caller]
pub fn write_to_slice<T: Pod>(array: &mut [u8], offset: u64, value: impl TryInto<T>) -> Result<()> {
    let file_location = std::panic::Location::caller();
    let offset = offset as usize;
    let value: T = value.
        try_into()
        .map_err(|_| anyhow!("{}:{}: type conversion failed",
                file_location.file(),
                file_location.line(),
        ))?;
    let size = std::mem::size_of::<T>();
    if offset + size > array.len() {
        bail!("{}:{}: write out of bounds",
            file_location.file(),
            file_location.line(),
        )
    }
    let bytes = unsafe {
        std::slice::from_raw_parts(&value as *const T as *const u8, size)
    };
    array[offset..][..size].copy_from_slice(bytes);
    Ok(())
}

#[track_caller]
pub fn rel_i32(target: u64, source: u64) -> Result<i32> {
    let file_location = std::panic::Location::caller();
    let relative_offset = (target as i128) - (source as i128);
    relative_offset
        .try_into()
        .map_err(|_| anyhow!("{}:{}: relative offset outside i32 range",
                file_location.file(),
                file_location.line(),
        ))
}

pub fn _print_flow(start_address: u64, stop_address: u64) -> Result<()> {
    let pid = unsafe { ATTACHED_PROCESS.pid };
    let start = Instant::now();
    let timeout = Duration::from_millis(10);

    loop {
        if start.elapsed() > timeout {
            bail!("er")
        }
        let handle = PTRACE_MUTEX.lock().unwrap();
        ptrace::attach(pid)?;
        waitpid(pid, None)?;

        let start = unsafe { ATTACHED_PROCESS.module_handle };
        let rip = ptrace::getregs(pid)?.rip;
        if start < rip && rip < start + 0x5E03000 {
            let original_regs = ptrace::getregset::<NT_PRSTATUS>(pid)?;
            let original_fp_regs = ptrace::getregset::<NT_PRFPREG>(pid)?;
            let mut regs = original_regs;

            regs.rsp -= 128;
            regs.rsp &= !0xF;
            regs.rsp -= 8;
            regs.rip = start_address;
            ptrace::setregset::<NT_PRSTATUS>(pid, regs)?;

            let mut counter = 0;
            let mut rip = start_address;
            while rip != stop_address && counter < 50000 {
                ptrace::step(pid, None)?;
                waitpid(pid, None)?;
                rip = ptrace::getregs(pid)?.rip;
                println!("{:#X}", rip);
                counter += 1;
            }
            if counter >= 50000 {
                println!("Did not reach end");
            }
            ptrace::setregset::<NT_PRSTATUS>(pid, original_regs)?;
            ptrace::setregset::<NT_PRFPREG>(pid, original_fp_regs)?;
            ptrace::detach(pid, None).map_err(anyhow::Error::msg)?;
            return Ok(())
        } else {
            ptrace::detach(pid, None)?;
            drop(handle);
            thread::sleep(Duration::from_micros(10));
        }
    }
}
