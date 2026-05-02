use crate::core::{
    attach::{ATTACHED_PROCESS, module_handle, pid},
    common::write_to_slice,
};
use anyhow::{Result, anyhow, bail, ensure};
use libc::{NT_PRSTATUS, PTRACE_GETREGSET, PTRACE_SETREGSET};
use nix::{
    sys::{
        ptrace::{
            self,
            regset::{NT_PRFPREG, NT_PRSTATUS},
        },
        uio::{RemoteIoVec, process_vm_readv, process_vm_writev},
        wait::waitpid,
    },
    unistd::Pid,
};
use pelite::Pod;
use std::{
    any::type_name,
    hint::spin_loop,
    io::{IoSlice, IoSliceMut},
    mem::zeroed,
    ptr, slice,
    sync::{LazyLock, Mutex},
    thread,
    time::{Duration, Instant},
};

static PTRACE_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

const CREATE_THREAD_64: [u8; 39] = [
    0x48, 0x83, 0xEC, 0x30,                     // sub rsp,0x30
    0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00,   // mov qword ptr [rsp+20h],0x0 (dwCreationFlags)
    0x00, 0x00,
    0x48, 0xC7, 0x44, 0x24, 0x28, 0x00, 0x00,   // mov qword ptr [rsp+28h],0x0 (lpThreadId)
    0x00, 0x00,
    0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00,   // movabs rax,create_thread_address
    0x00, 0x00, 0x00,
    0xFF, 0xD0,                                 // call
    0x48, 0x83, 0xC4, 0x30,                     // add rsp,0x30
    0xCC,                                       // int3
];

const CREATE_THREAD_32: [u8; 23] = [
    0x6A, 0x00,                                 // push 0x0 (lpThreadId)
    0x6A, 0x00,                                 // push 0x0 (dwCreationFlags)
    0x6A, 0x00,                                 // push 0x0 (lpParameter)
    0x68, 0x00, 0x00, 0x00, 0x00,               // push code_address (lpStartAddress)
    0x6A, 0x00,                                 // push 0x0 (dwStackSize)
    0x6A, 0x00,                                 // push 0x0 (lpThreadAttributes)
    0xB8, 0x00, 0x00, 0x00, 0x00,               // mov eax create_thread_address
    0xFF, 0xD0,                                 // call
    0xCC,                                       // int3
];

const FLAG_SETTER_64: [u8; 14] = [
    0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00,   // movabs rax,flag_loc
    0x00, 0x00, 0x00,
    0xC6, 0x00, 0x00,                           // mov BYTE PTR [rax],0x0
    0xC3,                                       // ret
];

const FLAG_SETTER_32: [u8; 9] = [
    0xB8, 0x00, 0x00, 0x00, 0x00,               // mov eax,flag_loc
    0xC6, 0x00, 0x00,                           // mov BYTE PTR [eax],0x0
    0xC3,                                       // ret
];

#[repr(C)]
#[derive(Debug, Clone)]
struct i386Regs {
    ebx: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    ebp: u32,
    eax: u32,
    ds: u16,
    __ds: u16,
    es: u16,
    __es: u16,
    fs: u16,
    __fs: u16,
    gs: u16,
    __gs: u16,
    orig_eax: u32,
    eip: u32,
    cs: u16,
    __cs: u16,
    eflags: u32,
    esp: u32,
    ss: u16,
    __ss: u16,
}

#[track_caller]
pub fn read_unsafe<T: Pod>(address: u64) -> Result<T> {
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
pub fn write_unsafe<T: Pod>(address: u64, value: T) -> Result<()> {
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
pub fn write_bytes_unsafe(address: u64, data: &[u8]) -> Result<()> {
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

pub fn append_64bit_flag_setter(location: u64, asm_head: &[u8]) -> Result<Vec<u8>> {
    let mut asm_tail = FLAG_SETTER_64;
    write_to_slice::<u64>(&mut asm_tail, 2, location.saturating_sub(1))?;
    let mut asm = asm_head[..asm_head.len().saturating_sub(1)].to_vec();
    asm.extend_from_slice(&asm_tail);
    Ok(asm)
}

pub fn append_32bit_flag_setter(location: u64, asm_head: &[u8]) -> Result<Vec<u8>> {
    let mut asm_tail = FLAG_SETTER_32;
    write_to_slice::<u32>(&mut asm_tail, 1, location.saturating_sub(1))?;
    let mut asm = asm_head[..asm_head.len().saturating_sub(1)].to_vec();
    asm.extend_from_slice(&asm_tail);
    Ok(asm)
}

pub fn run_win_thread_wait(
    code_address: u64,
    cave_address: u64,
    create_thread_pointer_location: u64,
    is_32: bool,
) -> Result<()> {
    let running_flag = code_address.saturating_sub(1);
    write_unsafe::<u8>(running_flag, 0x1)?;

    if is_32 {
        run_win32_thread(code_address, cave_address, create_thread_pointer_location)?;
    } else {
        run_win64_thread(code_address, cave_address, create_thread_pointer_location)?;
    }

    let start = Instant::now();
    let timeout = Duration::from_millis(50);
    loop {
        if read_unsafe::<u8>(running_flag)? == 0x0 {
            return Ok(())
        }
        if start.elapsed() > timeout {
            bail!("Thread did not return within {:#?}", timeout)
        }
        spin_loop();
    }
}

pub fn run_win64_thread(
    code_address: u64,
    cave_address: u64,
    create_thread_pointer_location: u64,
) -> Result<()> {
    let start = Instant::now();
    let timeout = Duration::from_millis(50);

    loop {
        if start.elapsed() > timeout {
            bail!("Failed to spawn thread at {:#X} within {:#?}", code_address, timeout)
        }
        let handle = PTRACE_MUTEX.lock().unwrap();
        ptrace::attach(pid())?;
        waitpid(pid(), None)?;

        let start = module_handle();
        let rip = ptrace::getregs(pid())?.rip;
        if start < rip && rip < start + 0x5E03000 {
            let original_regs = ptrace::getregset::<NT_PRSTATUS>(pid())?;
            let original_fp_regs = ptrace::getregset::<NT_PRFPREG>(pid())?;
            let mut regs = original_regs;

            regs.rcx = 0;                                   // lpThreadAttributes
            regs.rdx = 0;                                   // dwStackSize
            regs.r9 = 0;                                    // lpParameter
            regs.r8 = code_address;                         // lpStartAddress
            regs.rip = cave_address;
            regs.rsp = (regs.rsp - 0x100) & !0xFu64;

            let create_thread_address = read_unsafe::<u64>(create_thread_pointer_location)?;
            let mut asm = CREATE_THREAD_64;
            write_to_slice::<u64>(&mut asm, 24, create_thread_address)?;
            write_bytes_unsafe(cave_address, &asm)?;

            ptrace::setregset::<NT_PRSTATUS>(pid(), regs)?;

            ptrace::cont(pid(), None)?;
            waitpid(pid(), None)?;

            ptrace::setregset::<NT_PRSTATUS>(pid(), original_regs)?;
            ptrace::setregset::<NT_PRFPREG>(pid(), original_fp_regs)?;
            ptrace::detach(pid(), None).map_err(anyhow::Error::msg)?;
            return Ok(())
        } else {
            ptrace::detach(pid(), None)?;
            drop(handle);
            thread::sleep(Duration::from_micros(10));
        }
    }
}

pub fn run_win32_thread(
    code_address: u64,
    cave_address: u64,
    create_thread_pointer_location: u64,
) -> Result<()> {
    let start = Instant::now();
    let timeout = Duration::from_millis(50);

    loop {
        if start.elapsed() > timeout {
            bail!(
                "Failed to spawn thread at {:#X} within {:#?}",
                code_address,
                timeout
            )
        }
        let handle = PTRACE_MUTEX.lock().unwrap();

        unsafe {
            ptrace::attach(pid())?;
            waitpid(pid(), None)?;

            let mut regs_buf: [u8; 512] = zeroed();
            let mut iov = libc::iovec {
                iov_base: regs_buf.as_mut_ptr() as *mut libc::c_void,
                iov_len: regs_buf.len(),
            };

            libc::ptrace(
                PTRACE_GETREGSET,
                pid(),
                NT_PRSTATUS as *mut libc::c_void,
                &mut iov as *mut _ as *mut libc::c_void,
            );
            let regs_ptr = regs_buf.as_mut_ptr() as *mut i386Regs;
            let original_regs = ptr::read_unaligned(regs_ptr);
            let eip = original_regs.eip as u64;

            if module_handle() < eip && eip < module_handle() + 0x5E03000 {
                let mut regs = original_regs.clone();

                let create_thread_address = read_unsafe::<u32>(create_thread_pointer_location)?;
                let mut asm = CREATE_THREAD_32;
                write_to_slice::<u32>(&mut asm, 7, code_address as u32)?;
                write_to_slice::<u32>(&mut asm, 16, create_thread_address)?;
                write_bytes_unsafe(cave_address, &asm)?;

                regs.eip = cave_address.try_into()?;

                ptr::write_unaligned(regs_ptr, regs);
                libc::ptrace(
                    PTRACE_SETREGSET,
                    pid(),
                    libc::NT_PRSTATUS as *mut libc::c_void,
                    &mut iov as *mut _ as *mut libc::c_void
                );

                ptrace::cont(pid(), None)?;
                waitpid(pid(), None)?;

                ptr::write_unaligned(regs_ptr, original_regs);

                libc::ptrace(PTRACE_SETREGSET, pid(), 1, &mut iov as *mut _ as *mut libc::c_void);
                ptrace::detach(pid(), None)?;
                return Ok(());
            } else {
                ptrace::detach(pid(), None)?;
                drop(handle);
                thread::sleep(Duration::from_micros(10));
            }
        }
    }
}