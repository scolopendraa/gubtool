use std::{fmt::Display, fs, path::PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum AppDataError {
    Env(std::env::VarError),
    Io(std::io::ErrorKind),
    Serialize(toml::ser::Error),
    Deserialize(toml::de::Error),
}

pub fn app_data_dir() -> Result<PathBuf, AppDataError> {
    #[cfg(windows)]
    let mut dir = PathBuf::from(std::env::var("APPDATA")?);

    #[cfg(unix)]
    let mut dir = PathBuf::from(std::env::var("HOME")?)
        .join(".local")
        .join("state");

    dir.push("gubtool");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

const MAX_LINES: usize = 500;

pub fn log_error(err: &impl Display) -> Result<(), AppDataError> {
    let msg = err.to_string();
    let log_path = app_data_dir()?.join("errors.log");

    let contents = if log_path.exists() { fs::read_to_string(&log_path)? } else { String::new() };

    let mut lines: Vec<&str> = contents.lines().collect();

    if lines.last() == Some(&msg.as_str()) {
        return Ok(());
    }

    lines.push(&msg);

    let lines = if lines.len() > MAX_LINES {
        &lines[lines.len() - MAX_LINES..]
    } else {
        &lines[..]
    };

    fs::write(&log_path, lines.join("\n") + "\n")?;
    Ok(())
}

impl From<std::env::VarError> for AppDataError {
    fn from(err: std::env::VarError) -> Self {
        Self::Env(err)
    }
}

impl From<std::io::Error> for AppDataError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.kind())
    }
}

impl From<toml::ser::Error> for AppDataError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serialize(err)
    }
}

impl From<toml::de::Error> for AppDataError {
    fn from(err: toml::de::Error) -> Self {
        Self::Deserialize(err)
    }
}

impl std::error::Error for AppDataError {}

impl Display for AppDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Env(err) => write!(f, "Env error: {}", err),
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Serialize(err) => write!(f, "Serialize error: {err}"),
            Self::Deserialize(err) => write!(f, "Deserialize error: {err}"),
        }
    }
}
