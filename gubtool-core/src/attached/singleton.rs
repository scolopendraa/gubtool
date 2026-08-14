use {
    crate::{
        attached::{AddressSize, GameProcess},
        game_version::{Game, GameVersion, Version},
        sys::sys_error::{ProcResult, ProcessError},
    },
    std::path::PathBuf,
};

static mut ATTACHED_PROCESS: Option<GameProcess> = None;

#[expect(static_mut_refs)]
#[inline(always)]
fn attached_process() -> &'static Option<GameProcess> {
    unsafe { &ATTACHED_PROCESS }
}

pub fn pid() -> ProcResult<crate::sys::Pid> {
    attached_process()
        .as_ref()
        .map(|process| process.pid)
        .ok_or(ProcessError::NotAttached)
}

pub fn game_version() -> ProcResult<GameVersion> {
    attached_process()
        .as_ref()
        .map(|process| process.game_version)
        .ok_or(ProcessError::NotAttached)
}

pub fn game() -> ProcResult<Game> {
    attached_process()
        .as_ref()
        .map(|process| process.game_version.game())
        .ok_or(ProcessError::NotAttached)
}

pub fn comm() -> ProcResult<&'static str> {
    attached_process()
        .as_ref()
        .map(|process| process.comm.as_str())
        .ok_or(ProcessError::NotAttached)
}

pub fn path() -> ProcResult<&'static PathBuf> {
    attached_process()
        .as_ref()
        .map(|process| &process.exe_path)
        .ok_or(ProcessError::NotAttached)
}

pub fn module_base() -> u64 {
    attached_process()
        .as_ref()
        .map(|process| process.module_base)
        .unwrap_or(0x0)
}

pub fn address_size() -> ProcResult<AddressSize> {
    attached_process()
        .as_ref()
        .map(|process| process.address_size)
        .ok_or(ProcessError::NotAttached)
}

pub fn version<T: Version>() -> Option<T> {
    attached_process()
        .as_ref()
        .and_then(|process| T::from_game_version(&process.game_version))
}

pub fn is_32() -> bool {
    attached_process()
        .as_ref()
        .map(|process| process.address_size == AddressSize::Bits32)
        .unwrap_or(false)
}

pub fn port() -> ProcResult<Option<u16>> {
    attached_process()
        .as_ref()
        .map(|process| process.port)
        .ok_or(ProcessError::NotAttached)
}

#[cfg(windows)]
pub(crate) fn handle() -> ProcResult<windows::Win32::Foundation::HANDLE> {
    attached_process()
        .as_ref()
        .map(|process| process.handle)
        .ok_or(ProcessError::NotAttached)
}

pub fn uptime() -> f64 {
    attached_process()
        .as_ref()
        .map(|process| process.uptime())
        .unwrap_or(0.0)
}

pub(crate) fn attach_to_process(process: GameProcess) {
    unsafe { ATTACHED_PROCESS = Some(process) }
}

#[expect(static_mut_refs)]
pub fn set_port(port: u16) {
    unsafe {
        if let Some(p) = &mut ATTACHED_PROCESS {
            p.port = Some(port)
        }
    }
}

pub fn detach() {
    unsafe { ATTACHED_PROCESS = None }
}

pub fn is_attached() -> bool {
    attached_process().as_ref().is_some()
}

fn process_exists() -> ProcResult<bool> {
    attached_process()
        .as_ref()
        .map(|process| process.exists())
        .ok_or(ProcessError::NotAttached)
}

pub fn detach_if_invalid() {
    if let Ok(exists) = process_exists()
        && !exists
    {
        detach();
    }
}
