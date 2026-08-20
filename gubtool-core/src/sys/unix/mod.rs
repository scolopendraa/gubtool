use {
    crate::{
        address::{Address, POINTER},
        attached,
        slice_ops::{SliceError, write_to_slice},
        sys::{
            ASM32,
            ASM64,
            sys_error::{AccessType, PtraceAction, SysError, SysResult, WriteType},
        },
    },
    assemble::patch::QWORD,
    libc::{NT_PRSTATUS, PTRACE_GETREGSET, PTRACE_SETREGSET},
    nix::sys::{
        ptrace::{
            self,
            regset::{NT_PRFPREG, NT_PRSTATUS},
        },
        uio::{RemoteIoVec, process_vm_readv, process_vm_writev},
        wait::waitpid,
    },
    std::{
        any::type_name,
        io::{IoSlice, IoSliceMut},
        mem::zeroed,
        ptr,
        slice,
        sync::{LazyLock, Mutex},
        thread,
        time::{Duration, Instant},
    },
};

static PTRACE_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[track_caller]
pub fn read_unsafe<T>(address: impl Address) -> SysResult<T> {
    unsafe {
        let pid = pid()?;
        let mut value = std::mem::zeroed::<T>();
        let size = std::mem::size_of::<T>();
        let local_iov =
            IoSliceMut::new(slice::from_raw_parts_mut(&mut value as *mut T as *mut u8, size));
        let remote_iov = RemoteIoVec {
            base: address.addr() as usize,
            len:  size,
        };

        let nread = match process_vm_readv(pid, &mut [local_iov], &[remote_iov]) {
            Ok(n) => n,
            Err(err) => {
                return Err(SysError::io(
                    AccessType::Read(type_name::<T>()),
                    address.addr(),
                    std::io::Error::from(err),
                ));
            }
        };
        if nread != size {
            return Err(SysError::partial_access(
                AccessType::Read(type_name::<T>()),
                nread,
                address.addr(),
            ));
        }
        Ok(value)
    }
}

#[track_caller]
pub fn write_unsafe<T>(address: impl Address, value: T) -> SysResult {
    unsafe {
        let pid = pid()?;
        let size = std::mem::size_of::<T>();
        let local_iov = IoSlice::new(slice::from_raw_parts(&value as *const T as *const u8, size));
        let remote_iov = RemoteIoVec {
            base: address.addr() as usize,
            len:  size,
        };

        let nwritten = match process_vm_writev(pid, &[local_iov], &[remote_iov]) {
            Ok(n) => n,
            Err(err) => {
                return Err(SysError::io(
                    AccessType::Write(WriteType::Type(type_name::<T>())),
                    address.addr(),
                    std::io::Error::from(err),
                ));
            }
        };
        if nwritten != size {
            return Err(SysError::partial_access(
                AccessType::Write(WriteType::Type(type_name::<T>())),
                nwritten,
                address.addr(),
            ));
        }
        Ok(())
    }
}

#[track_caller]
pub fn write_bytes_unsafe(address: impl Address, data: &[u8]) -> SysResult {
    let pid = pid()?;
    let size = data.len();
    let local_iov = IoSlice::new(data);
    let remote_iov = RemoteIoVec {
        base: address.addr() as usize,
        len:  size,
    };

    let nwritten = match process_vm_writev(pid, &[local_iov], &[remote_iov]) {
        Ok(n) => n,
        Err(err) => {
            return Err(SysError::io(
                AccessType::Write(WriteType::Bytes(size)),
                address.addr(),
                std::io::Error::from(err),
            ));
        }
    };
    if nwritten != size {
        return Err(SysError::partial_access(
            AccessType::Write(WriteType::Bytes(size)),
            nwritten,
            address.addr(),
        ));
    }
    Ok(())
}

pub fn spawn_thread_release(
    spawn_code_address: impl Address,
    thread_start_address: impl Address,
    thread_code: Vec<u8>,
    create_thread_ptr_loc: impl Address,
    close_handle_ptr_loc: impl Address,
) -> SysResult {
    write_bytes_unsafe(thread_start_address, &thread_code)?;
    if attached::is_32() {
        run_win32_thread(
            spawn_code_address,
            thread_start_address,
            create_thread_ptr_loc,
            close_handle_ptr_loc,
        )
    } else {
        run_win64_thread(
            spawn_code_address,
            thread_start_address,
            create_thread_ptr_loc,
            close_handle_ptr_loc,
        )
    }
}

pub fn spawn_thread_join(
    spawn_code_address: impl Address,
    thread_start_address: impl Address,
    mut thread_code: Vec<u8>,
    create_thread_ptr_loc: impl Address,
    close_handle_ptr_loc: impl Address,
) -> SysResult {
    let running_flag = thread_start_address.addr().saturating_sub(1);
    write_unsafe::<u8>(running_flag, 0x1)?;

    if attached::is_32() {
        append_32bit_flag_setter(thread_start_address.addr(), &mut thread_code)?;
        write_bytes_unsafe(thread_start_address, &thread_code)?;
        run_win32_thread(
            spawn_code_address,
            thread_start_address,
            create_thread_ptr_loc,
            close_handle_ptr_loc,
        )?;
    } else {
        append_64bit_flag_setter(thread_start_address.addr(), &mut thread_code)?;
        write_bytes_unsafe(thread_start_address, &thread_code)?;
        run_win64_thread(
            spawn_code_address,
            thread_start_address,
            create_thread_ptr_loc,
            close_handle_ptr_loc,
        )?;
    }

    let start = Instant::now();
    let timeout = Duration::from_millis(100);
    loop {
        if read_unsafe::<u8>(running_flag)? == 0x0 {
            return Ok(());
        }
        if start.elapsed() > timeout {
            return Err(SysError::RemoteThreadReturn {
                timeout,
            });
        }
        thread::sleep(Duration::from_micros(200))
    }
}

fn run_win64_thread(
    spawn_code_address: impl Address,
    thread_start_address: impl Address,
    create_thread_ptr_loc: impl Address,
    close_handle_ptr_loc: impl Address,
) -> SysResult {
    let pid = pid()?;
    let start = Instant::now();
    let timeout = Duration::from_millis(50);

    loop {
        if start.elapsed() > timeout {
            return Err(SysError::RemoteThreadCreate {
                os_error: libc::ETIMEDOUT,
            });
        }

        let handle = PTRACE_MUTEX.lock().unwrap();
        ptrace::attach(pid).map_err(|e| SysError::ptrace(PtraceAction::Attach, e))?;
        waitpid(pid, None).map_err(|e| SysError::ptrace(PtraceAction::Wait, e))?;

        let start = attached::module_base();
        let original_regs = ptrace::getregset::<NT_PRSTATUS>(pid)
            .map_err(|e| SysError::ptrace(PtraceAction::GetRegs, e))?;

        if start < original_regs.rip && original_regs.rip < start + 0x5e03000 {
            let original_fp_regs = ptrace::getregset::<NT_PRFPREG>(pid)
                .map_err(|e| SysError::ptrace(PtraceAction::GetRegs, e))?;

            let mut regs = original_regs;

            regs.rip = spawn_code_address.addr();
            regs.rsp = regs.rsp.strict_sub(0x100) & !0xfu64;

            let flag_loc = spawn_code_address.addr().strict_sub(1);

            let mut fun = ASM64.get_function("run_thread");

            fun.patch::<QWORD>("code_address", thread_start_address.addr());
            fun.patch::<QWORD>("create_thread", create_thread_ptr_loc.addr());
            fun.patch::<QWORD>("close_handle", close_handle_ptr_loc.addr());
            fun.patch::<QWORD>("flag_loc", flag_loc);

            write_unsafe::<u8>(flag_loc, 0x0)?;
            write_bytes_unsafe(spawn_code_address, &fun.bytes)?;

            ptrace::setregset::<NT_PRSTATUS>(pid, regs)
                .map_err(|e| SysError::ptrace(PtraceAction::SetRegs, e))?;

            ptrace::cont(pid, None).map_err(|e| SysError::ptrace(PtraceAction::Cont, e))?;
            waitpid(pid, None).map_err(|e| SysError::ptrace(PtraceAction::Wait, e))?;

            ptrace::setregset::<NT_PRSTATUS>(pid, original_regs)
                .map_err(|e| SysError::ptrace(PtraceAction::SetRegs, e))?;
            ptrace::setregset::<NT_PRFPREG>(pid, original_fp_regs)
                .map_err(|e| SysError::ptrace(PtraceAction::SetRegs, e))?;
            ptrace::detach(pid, None).map_err(|e| SysError::ptrace(PtraceAction::Detach, e))?;

            return check_success_flag(flag_loc);
        } else {
            ptrace::detach(pid, None).map_err(|e| SysError::ptrace(PtraceAction::Detach, e))?;
            drop(handle);
            thread::sleep(Duration::from_micros(10));
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
struct i386Regs {
    ebx:      u32,
    ecx:      u32,
    edx:      u32,
    esi:      u32,
    edi:      u32,
    ebp:      u32,
    eax:      u32,
    ds:       u16,
    __ds:     u16,
    es:       u16,
    __es:     u16,
    fs:       u16,
    __fs:     u16,
    gs:       u16,
    __gs:     u16,
    orig_eax: u32,
    eip:      u32,
    cs:       u16,
    __cs:     u16,
    eflags:   u32,
    esp:      u32,
    ss:       u16,
    __ss:     u16,
}

fn run_win32_thread(
    spawn_code_address: impl Address,
    thread_start_address: impl Address,
    create_thread_ptr_loc: impl Address,
    close_handle_ptr_loc: impl Address,
) -> Result<(), SysError> {
    let pid = pid()?;
    let start = Instant::now();
    let timeout = Duration::from_millis(50);

    loop {
        if start.elapsed() > timeout {
            return Err(SysError::RemoteThreadCreate {
                os_error: libc::ETIMEDOUT,
            });
        }

        let handle = PTRACE_MUTEX.lock().unwrap();

        unsafe {
            ptrace::attach(pid).map_err(|e| SysError::ptrace(PtraceAction::Attach, e))?;
            waitpid(pid, None).map_err(|e| SysError::ptrace(PtraceAction::Wait, e))?;

            let mut regs_buf: [u8; size_of::<i386Regs>()] = zeroed();
            let mut iov = libc::iovec {
                iov_base: regs_buf.as_mut_ptr() as *mut libc::c_void,
                iov_len:  regs_buf.len(),
            };

            libc::ptrace(
                PTRACE_GETREGSET,
                pid,
                NT_PRSTATUS as *mut libc::c_void,
                &mut iov as *mut _ as *mut libc::c_void,
            );

            let regs_ptr = regs_buf.as_mut_ptr() as *mut i386Regs;
            let original_regs = ptr::read_unaligned(regs_ptr);
            let eip = original_regs.eip as u64;
            let start = attached::module_base();

            if start < eip && eip < start + 0x5e03000 {
                let mut regs = original_regs.clone();

                let flag_loc = spawn_code_address.addr().saturating_sub(1);

                let mut fun = ASM32.get_function("run_thread");

                fun.patch::<POINTER>("code_address", thread_start_address.addr());
                fun.patch::<POINTER>("create_thread", create_thread_ptr_loc.addr());
                fun.patch::<POINTER>("close_handle", close_handle_ptr_loc.addr());
                fun.patch::<POINTER>("flag_loc", flag_loc);

                write_unsafe::<u8>(flag_loc, 0x0)?;
                write_bytes_unsafe(spawn_code_address, &fun.bytes)?;

                regs.eip = spawn_code_address.addr() as u32;

                ptr::write_unaligned(regs_ptr, regs);

                libc::ptrace(
                    PTRACE_SETREGSET,
                    pid,
                    libc::NT_PRSTATUS as *mut libc::c_void,
                    &mut iov as *mut _ as *mut libc::c_void,
                );

                ptrace::cont(pid, None).map_err(|e| SysError::ptrace(PtraceAction::Cont, e))?;
                waitpid(pid, None).map_err(|e| SysError::ptrace(PtraceAction::Wait, e))?;

                ptr::write_unaligned(regs_ptr, original_regs);

                libc::ptrace(PTRACE_SETREGSET, pid, 1, &mut iov as *mut _ as *mut libc::c_void);
                ptrace::detach(pid, None).map_err(|e| SysError::ptrace(PtraceAction::Detach, e))?;

                return check_success_flag(flag_loc);
            } else {
                ptrace::detach(pid, None).map_err(|e| SysError::ptrace(PtraceAction::Detach, e))?;
                drop(handle);
                thread::sleep(Duration::from_micros(10));
            }
        }
    }
}

fn check_success_flag(flag_loc: u64) -> SysResult {
    let flag = read_unsafe::<u8>(flag_loc)?;
    if flag != 0x0 {
        Ok(())
    } else {
        // check CreateThread error code todo
        Err(SysError::RemoteThreadCreate {
            os_error: 0,
        })
    }
}

const FLAG_SETTER_64: [u8; 14] = [
    0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, // movabs rax, flag_loc
    0x00, 0x00, 0x00, 0xc6, 0x00, 0x00, // mov BYTE PTR [rax], 0x0
    0xc3, // ret
];

const FLAG_SETTER_32: [u8; 9] = [
    0xb8, 0x00, 0x00, 0x00, 0x00, // mov eax, flag_loc
    0xc6, 0x00, 0x00, // mov BYTE PTR [eax], 0x0
    0xc3, // ret
];

fn append_64bit_flag_setter(location: u64, asm_head: &mut Vec<u8>) -> Result<(), SliceError> {
    let mut asm_tail = FLAG_SETTER_64;
    write_to_slice::<u64>(&mut asm_tail, 2, location.saturating_sub(1))?;
    asm_head.pop();
    asm_head.extend_from_slice(&asm_tail);
    Ok(())
}

fn append_32bit_flag_setter(location: u64, asm_head: &mut Vec<u8>) -> Result<(), SliceError> {
    let mut asm_tail = FLAG_SETTER_32;
    write_to_slice::<u32>(&mut asm_tail, 1, location.saturating_sub(1))?;
    asm_head.pop();
    asm_head.extend_from_slice(&asm_tail);
    Ok(())
}

fn pid() -> SysResult<nix::unistd::Pid> {
    Ok(attached::pid()?.as_nix())
}
