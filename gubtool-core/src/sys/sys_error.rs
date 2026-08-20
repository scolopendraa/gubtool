use {
    crate::{game_version::Game, slice_ops::SliceError},
    std::{fmt::Display, io::ErrorKind, panic::Location},
    strum::Display,
};

pub type SysResult<T = ()> = Result<T, SysError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SysError {
    Io {
        access_type: AccessType,
        address:     u64,
        error_kind:  std::io::ErrorKind,
        os_error:    Option<i32>,
        location:    &'static Location<'static>,
    },
    PartialAccess {
        access_type:    AccessType,
        bytes_accessed: usize,
        address:        u64,
        location:       &'static Location<'static>,
    },
    RemoteThreadCreate {
        os_error: i32,
    },
    RemoteThreadReturn {
        timeout: std::time::Duration,
    },
    NotAttached,
    NullPointer {
        pointer_type: PointerType,
    },
    Slice {
        slice_error: SliceError,
    },
    InvalidGame {
        expected: Game,
    },

    #[cfg(unix)]
    Ptrace {
        ptrace_action: PtraceAction,
        error_kind:    std::io::ErrorKind,
        os_error:      Option<i32>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AccessType {
    Read(&'static str),
    Write(WriteType),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WriteType {
    Type(&'static str),
    Bytes(usize),
}

#[derive(Debug, Copy, Clone, PartialEq, Display)]
#[strum(serialize_all = "title_case")]
pub enum PointerType {
    Player,
    Target,
    Torrent,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PtraceAction {
    Attach,
    Detach,
    Wait,
    Cont,
    GetRegs,
    SetRegs,
}

impl SysError {
    pub fn error_kind(&self) -> Option<std::io::ErrorKind> {
        match self {
            Self::Io {
                error_kind,
                ..
            } => Some(*error_kind),
            #[cfg(unix)]
            Self::Ptrace {
                error_kind,
                ..
            } => Some(*error_kind),
            _ => None,
        }
    }

    pub fn os_code(&self) -> Option<i32> {
        match self {
            Self::Io {
                os_error,
                ..
            } => *os_error,
            #[cfg(unix)]
            Self::Ptrace {
                os_error,
                ..
            } => *os_error,
            Self::RemoteThreadCreate {
                os_error,
                ..
            } => Some(*os_error),
            _ => None,
        }
    }

    pub fn format_os_error(&self) -> String {
        self.os_code()
            .map(|e| format!("os error {e}"))
            .unwrap_or_else(|| "os error unknown".to_string())
    }

    #[track_caller]
    pub fn io(access_type: AccessType, address: u64, error: std::io::Error) -> Self {
        Self::Io {
            access_type,
            address,
            error_kind: error.kind(),
            os_error: error.raw_os_error(),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn partial_access(access_type: AccessType, bytes_accessed: usize, address: u64) -> Self {
        Self::PartialAccess {
            access_type,
            address,
            bytes_accessed,
            location: std::panic::Location::caller(),
        }
    }

    pub fn null_pointer(pointer_type: PointerType) -> Self {
        Self::NullPointer {
            pointer_type,
        }
    }

    #[cfg(unix)]
    pub fn ptrace(ptrace_action: PtraceAction, e: nix::errno::Errno) -> Self {
        let error = std::io::Error::from(e);
        Self::Ptrace {
            ptrace_action,
            error_kind: error.kind(),
            os_error: error.raw_os_error(),
        }
    }
}

impl Display for SysError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.error_kind() == Some(ErrorKind::PermissionDenied) {
            return match self {
                #[cfg(unix)]
                Self::Ptrace {
                    ..
                } => {
                    write!(f, "Permission denied. Is another debugger attached?")
                }
                _ => {
                    write!(f, "Permission denied. Consult the README for more information")
                }
            };
        }
        if self.error_kind() == Some(ErrorKind::NotFound) {
            return write!(f, "Process not found");
        }

        match &self {
            Self::PartialAccess {
                access_type,
                bytes_accessed,
                address,
                location,
            } => {
                let s = match access_type {
                    AccessType::Read(type_name) => {
                        format!(
                            "tried to read {type_name} at {:#X}, read {bytes_accessed} bytes",
                            address
                        )
                    }
                    AccessType::Write(WriteType::Type(type_name)) => {
                        format!(
                            "tried to write {type_name} at {:#X}, wrote {bytes_accessed} bytes",
                            address
                        )
                    }
                    AccessType::Write(WriteType::Bytes(num_written)) => {
                        format!(
                            "tried to write {num_written} bytes at {:#X}, wrote {bytes_accessed} \
                             bytes",
                            address
                        )
                    }
                };
                write!(f, "{}:{}: {s}", location.file(), location.line(),)
            }
            Self::Io {
                access_type,
                address,
                location,
                ..
            } => {
                let s = match access_type {
                    AccessType::Read(type_name) => {
                        format!(
                            "failed to read {type_name} at {:#X} ({})",
                            address,
                            self.format_os_error()
                        )
                    }
                    AccessType::Write(WriteType::Type(type_name)) => {
                        format!(
                            "failed to write {type_name} at {:#X} ({})",
                            address,
                            self.format_os_error()
                        )
                    }
                    AccessType::Write(WriteType::Bytes(num_written)) => {
                        format!(
                            "failed to write {num_written} bytes at {:#X} ({})",
                            address,
                            self.format_os_error()
                        )
                    }
                };
                write!(f, "{}:{}: {s}", location.file(), location.line(),)
            }
            #[cfg(unix)]
            Self::Ptrace {
                ptrace_action,
                ..
            } => {
                write!(f, "Ptrace {ptrace_action} failed ({})", self.format_os_error())
            }
            Self::RemoteThreadCreate {
                ..
            } => {
                write!(f, "CreateThread failed ({})", self.format_os_error())
            }
            Self::RemoteThreadReturn {
                timeout,
            } => {
                write!(f, "Remote thread did not return within {:#?}", timeout)
            }
            Self::NullPointer {
                pointer_type,
            } => {
                write!(f, "{pointer_type} not found")
            }
            Self::Slice {
                slice_error,
            } => {
                write!(f, "{slice_error}")
            }
            Self::InvalidGame {
                expected,
            } => {
                write!(f, "Not attached to {expected}")
            }
            Self::NotAttached => {
                write!(f, "No attached process")
            }
        }
    }
}

impl std::error::Error for SysError {}

impl Display for PtraceAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Attach => "attach",
            Self::Detach => "detach",
            Self::Cont => "cont",
            Self::Wait => "wait",
            Self::GetRegs => "get regs",
            Self::SetRegs => "set regs",
        };
        write!(f, "{s}")
    }
}

impl From<SliceError> for SysError {
    fn from(slice_error: SliceError) -> Self {
        Self::Slice {
            slice_error,
        }
    }
}
