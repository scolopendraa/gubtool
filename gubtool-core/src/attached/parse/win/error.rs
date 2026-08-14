use {crate::pe::error::ParsePeError, std::fmt::Display};

#[derive(Debug, Clone)]
pub enum ParseError {
    OpenProcess {
        err: windows::core::Error,
    },
    ModuleBase {
        err: windows::core::Error,
    },
    PathNotFound {
        err: windows::core::Error,
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
            Self::OpenProcess {
                err,
            } => {
                write!(f, "Could not open process: {err}")
            }
            Self::ModuleBase {
                err,
            } => {
                write!(f, "Could not determine base pointer: {err}")
            }
            Self::PathNotFound {
                err,
            } => {
                write!(f, "Could not find executable path: {err}")
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
