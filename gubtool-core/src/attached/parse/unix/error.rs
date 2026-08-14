use {
    crate::{pe::error::ParsePeError, sys::Pid},
    std::fmt::Display,
};

#[derive(Debug, Clone)]
pub enum ParseError {
    ScanMaps {
        pid:        Pid,
        error_kind: Option<std::io::ErrorKind>,
    },
    ExeNotFound {
        pid:        Pid,
        error_kind: Option<std::io::ErrorKind>,
    },
    ParsePe {
        error: ParsePeError,
    },
    MatchProductVersion {
        product_version: (u16, u16, u16),
    },
}

impl std::error::Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScanMaps {
                pid,
                error_kind,
            } => {
                if let Some(error_kind) = error_kind {
                    write!(f, "Could not read /proc/{pid}/maps: {error_kind}")
                } else {
                    write!(f, ".exe not found in /proc/{pid}/maps")
                }
            }
            Self::ExeNotFound {
                pid,
                error_kind,
            } => {
                if let Some(error_kind) = error_kind {
                    write!(f, "Could not read /proc/{pid}/environ: {error_kind}")
                } else {
                    write!(f, "Environment variable PWD not found in /proc/{pid}/environ")
                }
            }
            Self::ParsePe {
                error,
            } => write!(f, "{error}"),
            Self::MatchProductVersion {
                product_version: (major, minor, patch),
            } => {
                write!(f, "Could not match product version ({major}, {minor}, {patch})")
            }
        }
    }
}

impl From<ParsePeError> for ParseError {
    fn from(value: ParsePeError) -> Self {
        Self::ParsePe {
            error: value,
        }
    }
}
