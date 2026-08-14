use {crate::sys::ipc::READ_TIMEOUT, std::fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum IpcError {
    DllInjection,
    NoResponse,
    InvalidResponse(Vec<u8>),
    Io(std::io::ErrorKind),
}

impl Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DllInjection => {
                write!(f, "Could not find port after DLL injection attempt")
            }
            Self::NoResponse => {
                write!(f, "Worker did not return a reponse within {:#?}", READ_TIMEOUT)
            }
            Self::InvalidResponse(response) => {
                write!(f, "Server returned an invalid response: {:#?}", response)
            }
            Self::Io(errorkind) => {
                write!(f, "Ipc IO error: {errorkind}")
            }
        }
    }
}

impl std::error::Error for IpcError {}
