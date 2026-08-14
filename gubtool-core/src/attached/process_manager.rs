use {
    crate::{
        attached::{
            AttachError,
            GameProcess,
            ParseState,
            parse::{VALID_COMMS, parse_process},
        },
        sys::Pid,
    },
    std::sync::{Mutex, MutexGuard},
};

static GAME_PROCESSES: Mutex<Vec<GameProcess>> = Mutex::new(Vec::new());

pub fn refresh_processes() {
    let mut valid_processes = GAME_PROCESSES.lock().unwrap();
    for (pid, name) in system_processes() {
        for (valid_comm, game) in VALID_COMMS {
            if name == *valid_comm && valid_processes.iter().all(|p| pid != p.pid.as_u32()) {
                valid_processes.push(parse_process(game, Pid::new(pid), name.clone()));
            }
        }
    }

    #[cfg(unix)]
    valid_processes.retain(|p| p.exists());

    #[cfg(windows)]
    valid_processes.retain(|p| {
        if !p.exists() {
            unsafe {
                let _ = windows::Win32::Foundation::CloseHandle(p.handle);
            }
            false
        } else {
            true
        }
    });
}

pub fn try_auto_attach() -> Option<Result<(), AttachError>> {
    refresh_processes();
    let valid_processes = GAME_PROCESSES.lock().unwrap();
    for process in &*valid_processes {
        match process.parse_state {
            ParseState::Valid => {
                return Some(process.attach());
            }
            ParseState::Invalid(_) => continue,
        }
    }
    None
}

pub fn game_processes() -> MutexGuard<'static, Vec<GameProcess>> {
    GAME_PROCESSES.lock().unwrap()
}

#[cfg(unix)]
fn system_processes() -> impl Iterator<Item = (u32, String)> {
    use std::fs;

    std::fs::read_dir("/proc")
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name();
            let pid_str = file_name.to_string_lossy();

            let pid: u32 = pid_str.parse().ok()?;

            let cmdline_path = format!("/proc/{}/comm", pid);
            let name = fs::read_to_string(cmdline_path).ok()?;

            Some((pid, name.trim().to_string()))
        })
}

#[cfg(windows)]
fn system_processes() -> impl Iterator<Item = (u32, String)> {
    let mut out = Vec::new();

    unsafe {
        use windows::Win32::{
            Foundation::CloseHandle,
            System::Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot,
                PROCESSENTRY32W,
                Process32FirstW,
                Process32NextW,
                TH32CS_SNAPPROCESS,
            },
        };

        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
            .expect("failed to create process snapshot");

        let mut entry = PROCESSENTRY32W::default();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(handle, &mut entry).is_ok() {
            loop {
                let len = entry
                    .szExeFile
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.szExeFile.len());

                let name = String::from_utf16_lossy(&entry.szExeFile[..len]);

                out.push((entry.th32ProcessID, name));

                if Process32NextW(handle, &mut entry).is_err() {
                    break;
                }
            }
        }

        let _ = CloseHandle(handle);
    }

    out.into_iter()
}
