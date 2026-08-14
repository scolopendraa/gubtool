use {
    crate::{pe::error::ParsePeError, sys::sys_error::ProcessError},
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Could not parse IDA pattern: {failed_byte}")]
    ParsePattern {
        failed_byte: &'static str,
    },
    #[error("{scan_name} not found")]
    NotFound {
        scan_name: &'static str,
    },
    #[error("Overflow for {scan_name} when adding offset")]
    Overflow {
        scan_name: &'static str,
    },
    #[error("Overflow for {scan_name} when adding relative offset")]
    OverflowRelative {
        scan_name: &'static str,
    },
    #[error("{err}")]
    ProcessError {
        err: ProcessError,
    },
    #[error("IO error: {error_kind}")]
    Io {
        error_kind: std::io::ErrorKind,
    },
    #[error("{err}")]
    Pe {
        err: ParsePeError,
    },
    #[error("Found multiple matches for {scan_name}: {:#X?}", locations)]
    FoundDuplicates {
        scan_name: &'static str,
        locations: Vec<u64>,
    },
}

impl From<ProcessError> for ScanError {
    fn from(err: ProcessError) -> Self {
        Self::ProcessError {
            err,
        }
    }
}

impl From<std::io::Error> for ScanError {
    fn from(value: std::io::Error) -> Self {
        Self::Io {
            error_kind: value.kind(),
        }
    }
}

impl From<ParsePeError> for ScanError {
    fn from(value: ParsePeError) -> Self {
        Self::Pe {
            err: value,
        }
    }
}
