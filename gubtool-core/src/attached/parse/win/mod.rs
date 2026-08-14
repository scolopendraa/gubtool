pub mod error;
pub use error::ParseError;
use {
    crate::{
        attached::{GameProcess, parse::parse_pe_for_version_and_address_size},
        game_version::Game,
        sys::Pid,
    },
    std::{ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf},
    windows::{
        Win32::{
            Foundation::{HANDLE, HMODULE},
            System::{
                ProcessStatus::{K32EnumProcessModulesEx, LIST_MODULES_ALL},
                Threading::{
                    OpenProcess,
                    PROCESS_NAME_FORMAT,
                    PROCESS_QUERY_INFORMATION,
                    PROCESS_TERMINATE,
                    PROCESS_VM_OPERATION,
                    PROCESS_VM_READ,
                    PROCESS_VM_WRITE,
                    QueryFullProcessImageNameW,
                },
            },
        },
        core::PWSTR,
    },
};

pub(crate) fn parse_process(game: &Game, pid: Pid, comm: String) -> GameProcess {
    let mut parse_errors: Vec<ParseError> = Vec::new();

    let handle = get_handle(pid).unwrap_or_else(|err| {
        parse_errors.push(err);
        Default::default()
    });

    let module_base = get_module_base(handle).unwrap_or_else(|err| {
        parse_errors.push(err);
        Default::default()
    });

    let exe_path = get_exe_path(handle).unwrap_or_else(|err| {
        parse_errors.push(err);
        Default::default()
    });

    let (address_size, game_version, parse_state) =
        parse_pe_for_version_and_address_size(game, &exe_path, parse_errors);

    GameProcess {
        pid,
        game_version,
        comm,
        exe_path,
        module_base,
        address_size,
        parse_state,
        handle,
        port: None,
    }
}

fn get_handle(pid: Pid) -> Result<HANDLE, ParseError> {
    let flags = PROCESS_VM_READ
        | PROCESS_VM_WRITE
        | PROCESS_VM_OPERATION
        | PROCESS_QUERY_INFORMATION
        | PROCESS_TERMINATE;

    unsafe {
        OpenProcess(flags, false, pid.as_u32()).map_err(|err| {
            ParseError::OpenProcess {
                err,
            }
        })
    }
}

fn get_module_base(handle: HANDLE) -> Result<u64, ParseError> {
    let mut module_base = HMODULE::default();
    let mut needed = 0u32;

    unsafe {
        K32EnumProcessModulesEx(
            handle,
            &mut module_base,
            std::mem::size_of::<HMODULE>() as u32,
            &mut needed,
            LIST_MODULES_ALL.0,
        )
        .ok()
        .map_err(|err| {
            ParseError::ModuleBase {
                err,
            }
        })?;
    }
    Ok(module_base.0 as u64)
}

fn get_exe_path(handle: HANDLE) -> Result<PathBuf, ParseError> {
    let mut path_buf = [0u16; 260];
    let mut size = path_buf.len() as u32;

    unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            PWSTR(path_buf.as_mut_ptr()),
            &mut size,
        )
        .map_err(|err| {
            ParseError::PathNotFound {
                err,
            }
        })?;
    }

    Ok(PathBuf::from(OsString::from_wide(&path_buf[..size as usize])))
}
