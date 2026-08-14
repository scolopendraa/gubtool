use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum ParsePeError {
    IoError(std::io::ErrorKind),
    PeError(pelite::resources::FindError),
}

impl From<std::io::Error> for ParsePeError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.kind())
    }
}

impl From<pelite::resources::FindError> for ParsePeError {
    fn from(value: pelite::resources::FindError) -> Self {
        Self::PeError(value)
    }
}

impl From<pelite::Error> for ParsePeError {
    fn from(value: pelite::Error) -> Self {
        Self::PeError(pelite::resources::FindError::from(value))
    }
}

impl Display for ParsePeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(kind) => {
                write!(f, "Couldn't open filemap: {kind}")
            }
            Self::PeError(err) => {
                write!(f, "Error while parsing PE file: {err}")
            }
        }
    }
}

impl std::error::Error for ParsePeError {}
