use anyhow::{Result, anyhow, bail};
use nix::unistd::Pid;
use std::{
    fmt,
    fs::{self, DirEntry},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};
use pelite::{
    pe64::{Pe, PeFile},
    FileMap,
};

pub static mut ATTACHED_PROCESS: GameProcess = GameProcess::detached();

const VALID_COMMS: &[(&str, Game); 3] = &[
    ("eldenring.exe", Game::EldenRing),
    ("start_protected", Game::EldenRing),
    ("start_protected_game.exe", Game::EldenRing),
];

pub struct GameProcess {
    pub pid: Pid,
    pub comm: &'static str,
    pub path: PathBuf,
    pub game: Game,
    pub version: Version,
    pub module_handle: u64,
    pub attach_result: Result<()>,
}

impl GameProcess {
    pub const fn detached() -> Self {
        Self {
            pid: Pid::from_raw(-1),
            comm: "",
            path: PathBuf::new(),
            game: Game::EldenRing,
            version: Version::Invalid,
            module_handle: 0,
            attach_result: Ok(()),
        }
    }
}

pub fn auto_attach() -> Result<bool> {
    let mut processes = get_processes();
    if let Some(process) = processes.pop() {
        attach_to_process(process)?;
        return Ok(true);
    }
    Ok(false)
}

pub fn attach_to_process(process: GameProcess) -> Result<()> {
    unsafe {
        ATTACHED_PROCESS = process;
        let process = &raw const ATTACHED_PROCESS;
        (*process)
            .attach_result
            .as_ref()
            .map_err(|e| anyhow!(e.to_string()))?;
        Ok(())
    }
}

pub fn get_processes() -> Vec<GameProcess> {
    let mut processes = Vec::new();
    let entries = fs::read_dir("/proc").unwrap();
    for process in entries.flatten() {
        if let Some(valid_process) = parse_process(process) {
            processes.push(valid_process);
        }
    }
    processes
}

fn parse_process(process: DirEntry) -> Option<GameProcess> {
    let pid = process.file_name().into_string().unwrap();
    if !pid.chars().all(|c| c.is_numeric()) {
        return None;
    }
    let comm_path = Path::new("/proc").join(&pid).join("comm");
    let Ok(comm) = fs::read_to_string(comm_path) else {
        return None;
    };

    for (valid_comm, game) in VALID_COMMS {
        if comm.trim() == *valid_comm {
            let mut attach_result = Ok(());
            let pid = Pid::from_raw(pid.parse::<i32>().unwrap());

            let (path, module_handle) = get_path_and_handle(pid, game).unwrap_or_else(|err| {
                attach_result = Err(err);
                (PathBuf::default(), 0x140000000)
            });

            let mut version = Version::Invalid;
            if path != PathBuf::default() {
                match get_version(&path) {
                    Ok(v) => version = v,
                    Err(err) => {
                        if attach_result.is_ok() {
                            attach_result = Err(err)
                        }
                    }
                }
            }
            return Some(GameProcess {
                pid,
                comm: valid_comm,
                game: *game,
                version,
                path,
                module_handle,
                attach_result
            });
        }
    }
    None
}

fn get_path_and_handle(pid: Pid, game: &Game) -> Result<(PathBuf, u64)> {
    let maps_path = Path::new("/proc").join(pid.to_string()).join("maps");
    let maps_file = fs::File::open(maps_path).map_err(|err| anyhow!("Could not read /proc/{}/maps ({})", pid, err))?;
    let reader = BufReader::new(maps_file);
    let valid_exe_names = get_valid_exe_names(game);

    for line in reader.lines() {
        let line = line.unwrap();
        for name in valid_exe_names {
            if line.contains(name) {
                let handle = line.split_once('-')
                    .map(|(handle, _)| u64::from_str_radix(handle, 16))
                    .unwrap();

                let pos = line.find('/').unwrap();
                let exe_path = PathBuf::from(&line[pos..]);
                return Ok((exe_path, handle?))
            }
        }
    }
    bail!("exe not found in memory maps")
}

fn get_version(path: &PathBuf) -> Result<Version> {
    let file_map = FileMap::open(path)?;
    let pe = PeFile::from_bytes(&file_map)?;
    let resources = pe.resources()?;
    let version_info = resources.version_info()?;
    let product_version = version_info.fixed().unwrap().dwProductVersion;
    let version = match (
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
    ) {
        (1, 2, 0) => Version::V1_2_0,
        (1, 2, 1) => Version::V1_2_1,
        (1, 2, 2) => Version::V1_2_2,
        (1, 2, 3) => Version::V1_2_3,
        (1, 3, 0) => Version::V1_3_0,
        (1, 3, 1) => Version::V1_3_1,
        (1, 3, 2) => Version::V1_3_2,
        (1, 4, 0) => Version::V1_4_0,
        (1, 4, 1) => Version::V1_4_1,
        (1, 5, 0) => Version::V1_5_0,
        (1, 6, 0) => Version::V1_6_0,
        (1, 7, 0) => Version::V1_7_0,
        (1, 8, 0) => Version::V1_8_0,
        (1, 8, 1) => Version::V1_8_1,
        (1, 9, 0) => Version::V1_9_0,
        (1, 9, 1) => Version::V1_9_1,
        (2, 0, 0) => Version::V2_0_0,
        (2, 0, 1) => Version::V2_0_1,
        (2, 2, 0) => Version::V2_2_0,
        (2, 2, 3) => Version::V2_2_3,
        (2, 3, 0) => Version::V2_3_0,
        (2, 4, 0) => Version::V2_4_0,
        (2, 5, 0) => Version::V2_5_0,
        (2, 6, 0) => Version::V2_6_0,
        (2, 6, 1) => Version::V2_6_1,
        _ => bail!("Could not match product version ({}, {}, {})",
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
        ),
    };
    Ok(version)
}

fn get_valid_exe_names(game: &Game) -> &'static [&'static str] {
    match game {
        Game::EldenRing => &["eldenring.exe", "start_protected_game.exe"]
    }
}

pub fn is_pid_valid() -> bool {
    let pid_path = Path::new("/proc").join(unsafe { ATTACHED_PROCESS.pid }.to_string());
    let comm_path = Path::new(&pid_path).join("comm");
    if Path::exists(&pid_path)
        && let Ok(comm) = fs::read_to_string(comm_path)
        && (comm.trim() == unsafe { ATTACHED_PROCESS.comm }) {
        return true;
    }
    false
}

#[derive(Clone, Copy)]
pub enum Game {
    EldenRing,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Game::EldenRing => "Elden Ring",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Copy)]
pub enum Version {
    V1_2_0,
    V1_2_1,
    V1_2_2,
    V1_2_3,
    V1_3_0,
    V1_3_1,
    V1_3_2,
    V1_4_0,
    V1_4_1,
    V1_5_0,
    V1_6_0,
    V1_7_0,
    V1_8_0,
    V1_8_1,
    V1_9_0,
    V1_9_1,
    V2_0_0,
    V2_0_1,
    V2_2_0,
    V2_2_3,
    V2_3_0,
    V2_4_0,
    V2_5_0,
    V2_6_0,
    V2_6_1,
    Invalid,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Version::V1_2_0 => "1.02",
            Version::V1_2_1 => "1.02.1",
            Version::V1_2_2 => "1.02.2",
            Version::V1_2_3 => "1.02.3",
            Version::V1_3_0 => "1.03",
            Version::V1_3_1 => "1.03.1",
            Version::V1_3_2 => "1.03.2",
            Version::V1_4_0 => "1.04",
            Version::V1_4_1 => "1.04.1",
            Version::V1_5_0 => "1.05",
            Version::V1_6_0 => "1.06",
            Version::V1_7_0 => "1.07",
            Version::V1_8_0 => "1.08",
            Version::V1_8_1 => "1.08.1",
            Version::V1_9_0 => "1.09",
            Version::V1_9_1 => "1.09.1",
            Version::V2_0_0 => "1.10",
            Version::V2_0_1 => "1.10.1",
            Version::V2_2_0 => "1.12",
            Version::V2_2_3 => "1.12.3",
            Version::V2_3_0 => "1.13",
            Version::V2_4_0 => "1.14",
            Version::V2_5_0 => "1.15",
            Version::V2_6_0 => "1.16",
            Version::V2_6_1 => "1.16.1",
            Version::Invalid => "Unknown",
        };
        write!(f, "{}", name)
    }
}
