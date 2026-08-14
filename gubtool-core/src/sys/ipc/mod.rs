pub mod function_request;
pub mod ipc_error;

pub use function_request::*;
use {
    crate::{
        address::Address,
        attached::{self, AddressSize},
        slice_ops::write_addr_to_slice,
        sys::{ASM32, ASM64, dll::Dll, ipc::ipc_error::IpcError, write_bytes_unsafe},
    },
    std::{net::UdpSocket, time::Duration},
};

const WORKER_THREAD_32: Dll = Dll {
    name: "WinWorkerThread32",
    data: include_bytes!("../resources/WinWorkerThread32.dll"),
};

const WORKER_THREAD_64: Dll = Dll {
    name: "WinWorkerThread64",
    data: include_bytes!("../resources/WinWorkerThread64.dll"),
};

const READ_TIMEOUT: Duration = Duration::from_millis(100);

enum WorkerThreadRequest {
    Handshake,
    NullaryFunction,
    ParameterizedFunction,
    LoadLibrary,
}

impl WorkerThreadRequest {
    fn protocol_code(&self) -> u8 {
        match self {
            Self::Handshake => 0,
            Self::NullaryFunction => 1,
            Self::ParameterizedFunction => 2,
            Self::LoadLibrary => 3,
        }
    }
}

pub fn worker_thread_dll_load_code(
    load_library_w_pointer_loc: impl Address,
    path_loc: impl Address,
) -> anyhow::Result<Vec<u8>> {
    let dll = match attached::address_size()? {
        AddressSize::Bits32 => WORKER_THREAD_32,
        AddressSize::Bits64 => WORKER_THREAD_64,
    };

    if !dll.exists_on_disk()? {
        dll.write_to_disk()?;
    }

    let path_bytes = dll.get_win_path_bytes()?;
    write_bytes_unsafe(path_loc, &path_bytes)?;

    let mut fun = match attached::address_size()? {
        AddressSize::Bits32 => ASM32.get_function("load_library"),
        AddressSize::Bits64 => ASM64.get_function("load_library"),
    };

    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("path_loc"), path_loc)?;
    write_addr_to_slice(&mut asm, fun.reloc("load_library_w_loc"), load_library_w_pointer_loc)?;

    Ok(asm)
}

fn send_request(
    port: u16,
    request: WorkerThreadRequest,
    payload: Option<&[u8]>,
) -> Result<(), IpcError> {
    let mut request: Vec<u8> = vec![request.protocol_code()];

    if let Some(payload) = payload {
        request.extend_from_slice(payload);
    }

    let mut response = [0u8; 0x10];

    let socket = UdpSocket::bind(("127.0.0.1", 0)).unwrap();
    socket.set_read_timeout(Some(READ_TIMEOUT)).unwrap();
    socket.send_to(&request, ("127.0.0.1", port)).unwrap();

    match socket.recv_from(&mut response) {
        Ok((1, _)) if response[0] == 0x0 => Ok(()),
        Ok((len, _)) => Err(IpcError::InvalidResponse(response[..len].to_vec())),
        Err(err)
            if err.kind() == std::io::ErrorKind::TimedOut
                || err.kind() == std::io::ErrorKind::WouldBlock =>
        {
            Err(IpcError::NoResponse)
        }
        Err(err) => Err(IpcError::Io(err.kind())),
    }
}
