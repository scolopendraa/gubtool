pub mod error;
pub use error::ParseError;
use {
    crate::{
        attached::{GameProcess, parse::parse_pe_for_version_and_address_size},
        game_version::Game,
        sys::Pid,
    },
    std::{
        fs,
        io::{BufRead, BufReader, Read},
        path::PathBuf,
    },
};

pub(crate) fn parse_process(game: &Game, pid: Pid, comm: String) -> GameProcess {
    let mut parse_errors: Vec<ParseError> = Vec::new();

    let (exe_path, module_base) = match scan_maps_for_path(pid, *game) {
        Ok(v) => v,
        Err(err) => {
            parse_errors.push(err);
            Default::default()
        }
    };

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
        port: None,
    }
}

fn _parse_environ_for_path(pid: Pid, game: Game) -> Result<PathBuf, ParseError> {
    let path = format!("/proc/{pid}/environ");
    let target_field = "PWD";
    let mut file = fs::File::open(path).map_err(|err| {
        ParseError::ExeNotFound {
            pid,
            error_kind: Some(err.kind()),
        }
    })?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|err| {
        ParseError::ExeNotFound {
            pid,
            error_kind: Some(err.kind()),
        }
    })?;

    for env_var_bytes in buffer.split(|&b| b == 0) {
        if env_var_bytes.is_empty() {
            continue;
        }

        let env_var_str = String::from_utf8_lossy(env_var_bytes);

        if let Some((field, value)) = env_var_str.split_once('=')
            && field == target_field
        {
            for name in valid_exe_names(game) {
                let exe_path = PathBuf::from(value).join(name);
                if exe_path.exists() {
                    return Ok(exe_path);
                }
            }
        }
    }

    Err(ParseError::ExeNotFound {
        pid,
        error_kind: None,
    })
}

fn scan_maps_for_path(pid: Pid, game: Game) -> Result<(PathBuf, u64), ParseError> {
    let path = format!("/proc/{pid}/maps");
    let file = fs::File::open(path).map_err(|err| {
        ParseError::ScanMaps {
            pid,
            error_kind: Some(err.kind()),
        }
    })?;
    let reader = BufReader::new(file);
    let valid_exe_names = valid_exe_names(game);

    for line in reader.lines() {
        let line = line.unwrap();
        for name in valid_exe_names {
            if line.contains(name) {
                let base = line
                    .split_once('-')
                    .map(|(handle, _)| u64::from_str_radix(handle, 16))
                    .unwrap();

                let pos = line.find('/').unwrap();
                let exe_path = PathBuf::from(&line[pos..]);
                return Ok((exe_path, base.unwrap()));
            }
        }
    }
    Err(ParseError::ScanMaps {
        pid,
        error_kind: None,
    })
}

fn valid_exe_names(game: Game) -> &'static [&'static str] {
    match game {
        Game::EldenRing => &["eldenring.exe", "start_protected_game.exe"],
        Game::DarkSouls2 => &["DarkSoulsII.exe"],
    }
}
