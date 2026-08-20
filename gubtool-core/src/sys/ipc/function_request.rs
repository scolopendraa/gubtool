use crate::{
    address::Address,
    sys::ipc::{WorkerThreadRequest, ipc_error::IpcError, send_request},
};

#[allow(non_camel_case_types)]
pub enum FfiValue {
    pointer(u64),
    uint8(u8),
    uint16(u16),
    uint32(u32),
    uint64(u64),
    sint8(i8),
    sint16(i16),
    sint32(i32),
    sint64(i64),
    float(f32),
    double(f64),
}

#[allow(non_camel_case_types)]
pub enum X86CallingConvention {
    __cdecl,
    __stdcall,
    __fastcall,
    __thiscall,
}

pub fn request_parameterized_function(
    port: u16,
    function_address: impl Address,
    args: &[FfiValue],
    calling_convention: Option<X86CallingConvention>,
) -> Result<(), IpcError> {
    let mut payload = vec![args.len() as u8];

    payload.extend_from_slice(&function_address.addr().to_le_bytes());

    for arg in args {
        payload.extend_from_slice(&arg.protocol_code().to_le_bytes());
        payload.extend_from_slice(&arg.to_zero_extended_bytes());
    }

    if let Some(calling_convention) = calling_convention {
        payload.extend_from_slice(&[calling_convention.protocol_code()]);
    }

    send_request(port, WorkerThreadRequest::ParameterizedFunction, Some(&payload))
}

pub fn request_nullary_function(port: u16, function_address: impl Address) -> Result<(), IpcError> {
    send_request(
        port,
        WorkerThreadRequest::NullaryFunction,
        Some(&function_address.addr().to_le_bytes()),
    )
}

pub fn request_thread_function(port: u16, function_address: impl Address) -> Result<(), IpcError> {
    send_request(
        port,
        WorkerThreadRequest::ThreadFunction,
        Some(&function_address.addr().to_le_bytes()),
    )
}

fn zero_extend<const N: usize>(value: [u8; N]) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    bytes[..N].copy_from_slice(&value);
    bytes
}

impl FfiValue {
    fn to_zero_extended_bytes(&self) -> [u8; 8] {
        match self {
            Self::pointer(val) => zero_extend(val.to_le_bytes()),
            Self::uint8(val) => zero_extend(val.to_le_bytes()),
            Self::uint16(val) => zero_extend(val.to_le_bytes()),
            Self::uint32(val) => zero_extend(val.to_le_bytes()),
            Self::uint64(val) => zero_extend(val.to_le_bytes()),
            Self::sint8(val) => zero_extend(val.to_le_bytes()),
            Self::sint16(val) => zero_extend(val.to_le_bytes()),
            Self::sint32(val) => zero_extend(val.to_le_bytes()),
            Self::sint64(val) => zero_extend(val.to_le_bytes()),
            Self::float(val) => zero_extend(val.to_le_bytes()),
            Self::double(val) => zero_extend(val.to_le_bytes()),
        }
    }

    fn protocol_code(&self) -> u16 {
        match self {
            Self::pointer(_) => 0,
            Self::uint8(_) => 1,
            Self::uint16(_) => 2,
            Self::uint32(_) => 3,
            Self::uint64(_) => 4,
            Self::sint8(_) => 5,
            Self::sint16(_) => 6,
            Self::sint32(_) => 7,
            Self::sint64(_) => 8,
            Self::float(_) => 9,
            Self::double(_) => 10,
        }
    }
}

impl X86CallingConvention {
    fn protocol_code(&self) -> u8 {
        match self {
            Self::__cdecl => 0,
            Self::__stdcall => 1,
            Self::__fastcall => 2,
            Self::__thiscall => 3,
        }
    }
}
