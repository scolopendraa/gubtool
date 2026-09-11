use {
    crate::{
        address::Address,
        attached,
        sys::sys_error::{AccessType, SysError, SysResult, WriteType},
    },
    std::{any::type_name, mem::MaybeUninit, time::Duration},
    windows::Win32::{
        Foundation::{CloseHandle, WAIT_OBJECT_0},
        System::{
            Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},
            Threading::{CreateRemoteThread, LPTHREAD_START_ROUTINE, WaitForSingleObject},
        },
    },
};

#[track_caller]
pub fn read<T>(address: impl Address) -> SysResult<T> {
    address.validate_game()?;

    unsafe {
        let mut value = MaybeUninit::<T>::uninit();
        let size = std::mem::size_of::<T>();
        let mut nread = 0;

        let result = ReadProcessMemory(
            attached::handle()?,
            address.u64_address() as *const _,
            value.as_mut_ptr() as *mut _,
            size,
            Some(&mut nread),
        );

        if result.is_err() {
            return Err(SysError::io(
                AccessType::Read(type_name::<T>()),
                address.u64_address(),
                std::io::Error::last_os_error(),
            ));
        }

        if nread != size {
            return Err(SysError::partial_access(
                AccessType::Read(type_name::<T>()),
                nread,
                address.u64_address(),
            ));
        }

        Ok(value.assume_init())
    }
}

#[track_caller]
pub fn write<T>(address: impl Address, value: T) -> SysResult {
    address.validate_game()?;

    unsafe {
        let size = std::mem::size_of::<T>();
        let mut nwritten = 0;

        let result = WriteProcessMemory(
            attached::handle()?,
            address.u64_address() as *mut _,
            &value as *const T as *const _,
            size,
            Some(&mut nwritten),
        );

        if result.is_err() {
            return Err(SysError::io(
                AccessType::Write(WriteType::Type(type_name::<T>())),
                address.u64_address(),
                std::io::Error::last_os_error(),
            ));
        }

        if nwritten != size {
            return Err(SysError::partial_access(
                AccessType::Write(WriteType::Type(type_name::<T>())),
                nwritten,
                address.u64_address(),
            ));
        }
    }
    Ok(())
}

#[track_caller]
pub fn write_bytes(address: impl Address, data: &[u8]) -> SysResult {
    address.validate_game()?;

    unsafe {
        let size = data.len();
        let mut nwritten = 0;

        let result = WriteProcessMemory(
            attached::handle()?,
            address.u64_address() as *mut _,
            data.as_ptr() as *const _,
            size,
            Some(&mut nwritten),
        );

        if result.is_err() {
            return Err(SysError::io(
                AccessType::Write(WriteType::Bytes(size)),
                address.u64_address(),
                std::io::Error::last_os_error(),
            ));
        }

        if nwritten != size {
            return Err(SysError::partial_access(
                AccessType::Write(WriteType::Bytes(size)),
                nwritten,
                address.u64_address(),
            ));
        }
    }
    Ok(())
}

pub fn spawn_thread_release(thread_start_address: impl Address, thread_code: Vec<u8>) -> SysResult {
    unsafe {
        write_bytes(thread_start_address, &thread_code)?;
        let start: LPTHREAD_START_ROUTINE =
            Some(std::mem::transmute(thread_start_address.u64_address()));
        let thread_handle = CreateRemoteThread(attached::handle()?, None, 0, start, None, 0, None)
            .map_err(|err| {
                SysError::RemoteThreadCreate {
                    os_error: err.code().0,
                }
            })?;
        let _ = CloseHandle(thread_handle);
        Ok(())
    }
}

pub fn spawn_thread_join(thread_start_address: impl Address, thread_code: Vec<u8>) -> SysResult {
    unsafe {
        write_bytes(thread_start_address, &thread_code)?;
        let start: LPTHREAD_START_ROUTINE =
            Some(std::mem::transmute(thread_start_address.u64_address()));
        let thread_handle = CreateRemoteThread(attached::handle()?, None, 0, start, None, 0, None)
            .map_err(|err| {
                SysError::RemoteThreadCreate {
                    os_error: err.code().0,
                }
            })?;

        let timeout = 50;
        let wait_result = WaitForSingleObject(thread_handle, 50);

        let _ = CloseHandle(thread_handle);

        match wait_result {
            WAIT_OBJECT_0 => Ok(()),
            _ => {
                Err(SysError::RemoteThreadReturn {
                    timeout: Duration::from_millis(timeout as u64),
                })
            }
        }
    }
}
