use crate::{
    address::Address,
    sys::ipc::{WorkerThreadRequest, ipc_error::IpcError, send_request},
};

#[allow(non_camel_case_types)]
pub enum CppValue {
    uintptr_t(u64),
    uint8_t(u8),
    uint16_t(u16),
    uint32_t(u32),
    uint64_t(u64),
    int8_t(i8),
    int16_t(i16),
    int32_t(i32),
    int64_t(i64),
    float_t(f32),
    double_t(f64),
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
    args: &[CppValue],
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

fn zero_extend<const N: usize>(value: [u8; N]) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    bytes[..N].copy_from_slice(&value);
    bytes
}

impl CppValue {
    fn to_zero_extended_bytes(&self) -> [u8; 8] {
        match self {
            Self::uintptr_t(val) => zero_extend(val.to_le_bytes()),
            Self::uint8_t(val) => zero_extend(val.to_le_bytes()),
            Self::uint16_t(val) => zero_extend(val.to_le_bytes()),
            Self::uint32_t(val) => zero_extend(val.to_le_bytes()),
            Self::uint64_t(val) => zero_extend(val.to_le_bytes()),
            Self::int8_t(val) => zero_extend(val.to_le_bytes()),
            Self::int16_t(val) => zero_extend(val.to_le_bytes()),
            Self::int32_t(val) => zero_extend(val.to_le_bytes()),
            Self::int64_t(val) => zero_extend(val.to_le_bytes()),
            Self::float_t(val) => zero_extend(val.to_le_bytes()),
            Self::double_t(val) => zero_extend(val.to_le_bytes()),
        }
    }

    fn protocol_code(&self) -> u16 {
        match self {
            Self::uintptr_t(_) => 0,
            Self::uint8_t(_) => 1,
            Self::uint16_t(_) => 2,
            Self::uint32_t(_) => 3,
            Self::uint64_t(_) => 4,
            Self::int8_t(_) => 5,
            Self::int16_t(_) => 6,
            Self::int32_t(_) => 7,
            Self::int64_t(_) => 8,
            Self::float_t(_) => 9,
            Self::double_t(_) => 10,
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
