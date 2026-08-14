#[cfg(windows)]
use windows::Win32::{
    Foundation::{HANDLE, STILL_ACTIVE},
    System::Threading::GetExitCodeProcess,
};
use {
    crate::{
        attached::{self, parse::ParseError},
        game_version::GameVersion,
    },
    std::path::PathBuf,
    thiserror::Error,
};

#[derive(Debug, Error)]
#[error("{error_count} error(s) occurred during parsing. Check the log for details.")]
pub struct AttachError {
    pub error_count: usize,
}

#[derive(Debug, Clone)]
pub struct GameProcess {
    pub pid:          crate::sys::Pid,
    pub game_version: GameVersion,
    pub comm:         String,
    pub exe_path:     PathBuf,
    pub module_base:  u64,
    pub address_size: AddressSize,
    pub parse_state:  ParseState,
    pub port:         Option<u16>,
    #[cfg(windows)]
    pub handle:       HANDLE,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressSize {
    Bits32,
    Bits64,
}

#[derive(Debug, Clone)]
pub enum ParseState {
    Valid,
    Invalid(Vec<ParseError>),
}

impl GameProcess {
    pub fn attach(&self) -> Result<(), AttachError> {
        match &self.parse_state {
            ParseState::Valid => {
                attached::attach_to_process(self.clone());
                Ok(())
            }
            ParseState::Invalid(errors) => {
                let len = errors.len();
                for err in errors {
                    let _ = crate::appdata::log_error(&err);
                }
                Err(AttachError {
                    error_count: len,
                })
            }
        }
    }

    #[cfg(unix)]
    pub fn kill(&self) {
        use nix::sys::signal::{Signal, kill};
        let _ = kill(self.pid.as_nix(), Signal::SIGKILL);
    }

    #[cfg(windows)]
    pub fn kill(&self) {
        unsafe {
            let _ = windows::Win32::System::Threading::TerminateProcess(self.handle, 1);
        }
    }

    #[cfg(unix)]
    pub fn exists(&self) -> bool {
        use {
            crate::attached::parse::VALID_COMMS,
            std::{fs, path::Path},
        };

        let pid_path = Path::new("/proc").join(format!("{}", self.pid));
        let comm_path = Path::new(&pid_path).join("comm");
        if Path::exists(&pid_path)
            && let Ok(comm) = fs::read_to_string(comm_path)
            && VALID_COMMS.iter().any(|(name, _)| &comm.trim() == name)
        {
            return true;
        }
        false
    }

    #[cfg(windows)]
    pub fn exists(&self) -> bool {
        if self.handle.is_invalid() || self.handle.0.is_null() {
            return false;
        }
        unsafe {
            let mut exit_code: u32 = 0;
            let _ = GetExitCodeProcess(self.handle, &mut exit_code);
            exit_code == STILL_ACTIVE.0 as u32
        }
    }

    #[cfg(unix)]
    pub fn uptime(&self) -> f64 {
        (|| -> Option<f64> {
            use std::fs;

            let stat = fs::read_to_string(format!("/proc/{}/stat", self.pid)).ok()?;
            let start_ticks: f64 = stat.split_whitespace().nth(21)?.parse().ok()?;

            let system_uptime_str = fs::read_to_string("/proc/uptime").ok()?;
            let system_uptime: f64 = system_uptime_str.split_whitespace().next()?.parse().ok()?;

            let system_tick_rate = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as f64;

            Some(system_uptime - start_ticks / system_tick_rate)
        })()
        .unwrap_or(0.0)
    }

    #[cfg(windows)]
    pub fn uptime(&self) -> f64 {
        unsafe {
            use windows::Win32::{
                Foundation::FILETIME,
                System::{SystemInformation::GetSystemTimeAsFileTime, Threading::GetProcessTimes},
            };

            let mut creation = FILETIME::default();
            let mut exit = FILETIME::default();
            let mut kernel = FILETIME::default();
            let mut user = FILETIME::default();

            let _ = GetProcessTimes(self.handle, &mut creation, &mut exit, &mut kernel, &mut user);
            let now = GetSystemTimeAsFileTime();

            let creation = ((creation.dwHighDateTime as u64) << 32) | creation.dwLowDateTime as u64;
            let now = ((now.dwHighDateTime as u64) << 32) | now.dwLowDateTime as u64;

            (now - creation) as f64 / 10_000_000.0
        }
    }
}

#[cfg(windows)]
unsafe impl Send for GameProcess {}
