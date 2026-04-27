use anyhow::{Result, anyhow, bail};
use nix::unistd::Pid;
use pelite::{
    FileMap,
    image::VS_VERSION,
    pe32::{self, Pe as Pe32},
    pe64::{self, Pe as Pe64},
};
use std::{
    fmt,
    fs::{self, DirEntry},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

pub static mut ATTACHED_PROCESS: GameProcess = GameProcess::detached();

const VALID_COMMS: &[(&str, Game); 4] = &[
    ("eldenring.exe", Game::EldenRing),
    ("start_protected", Game::EldenRing),
    ("start_protected_game.exe", Game::EldenRing),
    ("DarkSoulsII.exe", Game::DarkSoulsII),
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
            version: Version::ERUnknown,
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

            let mut version = Version::ERUnknown;
            if path != PathBuf::default() {
                match get_version(*game, &path) {
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

fn get_version(game: Game, path: &PathBuf) -> Result<Version> {
    let file_map = FileMap::open(path)?;
    if let Ok(pe) = pe64::PeFile::from_bytes(&file_map) {
        let resources = pe.resources()?;
        let version_info = resources.version_info()?;
        let product_version = version_info.fixed().unwrap().dwProductVersion;
        match game {
            Game::EldenRing => match_er_version(product_version),
            Game::DarkSoulsII => match_scholar_version(product_version),
        }
    } else {
        let pe = pe32::PeFile::from_bytes(&file_map)?;
        let resources = pe.resources()?;
        let version_info = resources.version_info()?;
        let product_version = version_info.fixed().unwrap().dwProductVersion;
        match_vanilla_version(product_version)
    }
}

fn match_er_version(product_version: VS_VERSION) -> Result<Version> {
    Ok(match (
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
    ) {
        (1, 2, 0) => Version::ER1_2_0,
        (1, 2, 1) => Version::ER1_2_1,
        (1, 2, 2) => Version::ER1_2_2,
        (1, 2, 3) => Version::ER1_2_3,
        (1, 3, 0) => Version::ER1_3_0,
        (1, 3, 1) => Version::ER1_3_1,
        (1, 3, 2) => Version::ER1_3_2,
        (1, 4, 0) => Version::ER1_4_0,
        (1, 4, 1) => Version::ER1_4_1,
        (1, 5, 0) => Version::ER1_5_0,
        (1, 6, 0) => Version::ER1_6_0,
        (1, 7, 0) => Version::ER1_7_0,
        (1, 8, 0) => Version::ER1_8_0,
        (1, 8, 1) => Version::ER1_8_1,
        (1, 9, 0) => Version::ER1_9_0,
        (1, 9, 1) => Version::ER1_9_1,
        (2, 0, 0) => Version::ER2_0_0,
        (2, 0, 1) => Version::ER2_0_1,
        (2, 2, 0) => Version::ER2_2_0,
        (2, 2, 3) => Version::ER2_2_3,
        (2, 3, 0) => Version::ER2_3_0,
        (2, 4, 0) => Version::ER2_4_0,
        (2, 5, 0) => Version::ER2_5_0,
        (2, 6, 0) => Version::ER2_6_0,
        (2, 6, 1) => Version::ER2_6_1,
        _ => bail!("Could not match product version ({}, {}, {})",
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
        ),
    })
}

fn match_vanilla_version(product_version: VS_VERSION) -> Result<Version> {
    Ok(match (
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
    ) {
        (1, 0, 3) => Version::Vanilla1_0_3,
        (1, 0, 4) => Version::Vanilla1_0_4,
        (1, 0, 5) => Version::Vanilla1_0_5,
        (1, 0, 6) => Version::Vanilla1_0_5,
        (1, 0, 7) => Version::Vanilla1_0_7,
        (1, 0, 10) => Version::Vanilla1_0_10,
        (1, 0, 11) => Version::Vanilla1_0_11,
        (1, 0, 12) => Version::Vanilla1_0_12,
        _ => bail!("Could not match product version ({}, {}, {})",
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
        ),
    })
}

fn match_scholar_version(product_version: VS_VERSION) -> Result<Version> {
    Ok(match (
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
    ) {
        (1, 0, 1) => Version::Scholar1_0_1,
        (1, 0, 2) => Version::Scholar1_0_2,
        (1, 0, 3) => Version::Scholar1_0_3,
        _ => bail!("Could not match product version ({}, {}, {})",
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
        ),
    })
}

fn get_valid_exe_names(game: &Game) -> &'static [&'static str] {
    match game {
        Game::EldenRing => &["eldenring.exe", "start_protected_game.exe"],
        Game::DarkSoulsII => &["DarkSoulsII.exe"],
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

pub fn pid() -> Pid {
    unsafe { ATTACHED_PROCESS.pid }
}

pub fn game() -> Game {
    unsafe { ATTACHED_PROCESS.game }
}

pub fn module_handle() -> u64 {
    unsafe { ATTACHED_PROCESS.module_handle }
}

pub fn version() -> Version {
    unsafe { ATTACHED_PROCESS.version }
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Game {
    EldenRing,
    DarkSoulsII,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Game::EldenRing => "Elden Ring",
            Game::DarkSoulsII => "Dark Souls II",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Copy)]
pub enum Version {
    ER1_2_0, ER1_2_1, ER1_2_2, ER1_2_3,
    ER1_3_0, ER1_3_1, ER1_3_2, ER1_4_0,
    ER1_4_1, ER1_5_0, ER1_6_0, ER1_7_0,
    ER1_8_0, ER1_8_1, ER1_9_0, ER1_9_1,
    ER2_0_0, ER2_0_1, ER2_2_0, ER2_2_3,
    ER2_3_0, ER2_4_0, ER2_5_0, ER2_6_0,
    ER2_6_1, ERUnknown,

    Vanilla1_0_3, Vanilla1_0_4, Vanilla1_0_5, Vanilla1_0_6,
    Vanilla1_0_7, Vanilla1_0_10, Vanilla1_0_11, Vanilla1_0_12,
    Scholar1_0_1, Scholar1_0_2, Scholar1_0_3, VanillaUnknown,
    ScholarUnknown,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Version::ER1_2_0 => "Elden Ring v1.02",
            Version::ER1_2_1 => "Elden Ring v1.02.1",
            Version::ER1_2_2 => "Elden Ring v1.02.2",
            Version::ER1_2_3 => "Elden Ring v1.02.3",
            Version::ER1_3_0 => "Elden Ring v1.03",
            Version::ER1_3_1 => "Elden Ring v1.03.1",
            Version::ER1_3_2 => "Elden Ring v1.03.2",
            Version::ER1_4_0 => "Elden Ring v1.04",
            Version::ER1_4_1 => "Elden Ring v1.04.1",
            Version::ER1_5_0 => "Elden Ring v1.05",
            Version::ER1_6_0 => "Elden Ring v1.06",
            Version::ER1_7_0 => "Elden Ring v1.07",
            Version::ER1_8_0 => "Elden Ring v1.08",
            Version::ER1_8_1 => "Elden Ring v1.08.1",
            Version::ER1_9_0 => "Elden Ring v1.09",
            Version::ER1_9_1 => "Elden Ring v1.09.1",
            Version::ER2_0_0 => "Elden Ring v1.10",
            Version::ER2_0_1 => "Elden Ring v1.10.1",
            Version::ER2_2_0 => "Elden Ring v1.12",
            Version::ER2_2_3 => "Elden Ring v1.12.3",
            Version::ER2_3_0 => "Elden Ring v1.13",
            Version::ER2_4_0 => "Elden Ring v1.14",
            Version::ER2_5_0 => "Elden Ring v1.15",
            Version::ER2_6_0 => "Elden Ring v1.16",
            Version::ER2_6_1 => "Elden Ring v1.16.1",
            Version::ERUnknown => "Unknown",

            Version::Vanilla1_0_3 => "Dark Souls II v1.0.3",
            Version::Vanilla1_0_4 => "Dark Souls II v1.0.4",
            Version::Vanilla1_0_5 => "Dark Souls II v1.0.5",
            Version::Vanilla1_0_6 => "Dark Souls II v1.0.6",
            Version::Vanilla1_0_7 => "Dark Souls II v1.0.7",
            Version::Vanilla1_0_10 => "Dark Souls II v1.0.10",
            Version::Vanilla1_0_11 => "Dark Souls II v1.0.11",
            Version::Vanilla1_0_12 => "Dark Souls II v1.0.12",
            Version::VanillaUnknown => "Unknown",
            Version::Scholar1_0_1 => "Dark Souls II (Scholar) v1.0.1",
            Version::Scholar1_0_2 => "Dark Souls II (Scholar) v1.0.2",
            Version::Scholar1_0_3 => "Dark Souls II (Scholar) v1.0.3",
            Version::ScholarUnknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}
