use {
    crate::{address::Address, attached::is_32},
    pelite::Pod,
    std::{fmt::Display, panic::Location},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SliceError {
    error_kind: SliceErrorKind,
    location:   &'static Location<'static>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SliceErrorKind {
    OffsetOverflow,
    OutOfBounds,
    TypeConversion,
    RelI32,
}

impl SliceError {
    #[track_caller]
    fn new(error_kind: SliceErrorKind) -> Self {
        let file_location = std::panic::Location::caller();
        Self {
            error_kind,
            location: file_location,
        }
    }
}

impl std::error::Error for SliceError {}

impl Display for SliceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.error_kind {
            SliceErrorKind::OffsetOverflow => "offset overflow",
            SliceErrorKind::OutOfBounds => "out of bounds access",
            SliceErrorKind::TypeConversion => "type conversion failed",
            SliceErrorKind::RelI32 => "relative offset outside i32 range",
        };
        write!(f, "{}:{}: {s}", self.location.file(), self.location.line(),)
    }
}

#[track_caller]
pub fn read_from_slice<T: Pod>(array: &[u8], offset: u64) -> Result<T, SliceError> {
    let offset = offset as usize;
    let size = std::mem::size_of::<T>();
    let end = match offset.checked_add(size) {
        Some(end) => end,
        None => return Err(SliceError::new(SliceErrorKind::OffsetOverflow)),
    };
    let bytes = match array.get(offset..end) {
        Some(end) => end,
        None => return Err(SliceError::new(SliceErrorKind::OutOfBounds)),
    };
    Ok(unsafe { std::ptr::read_unaligned(bytes.as_ptr() as *const T) })
}

#[track_caller]
pub fn write_to_slice<T: Pod>(
    array: &mut [u8],
    offset: u64,
    value: impl TryInto<T>,
) -> Result<(), SliceError> {
    let offset = offset as usize;
    let value: T = match value.try_into() {
        Ok(val) => val,
        Err(_) => return Err(SliceError::new(SliceErrorKind::TypeConversion)),
    };
    let size = std::mem::size_of::<T>();
    if offset + size > array.len() {
        return Err(SliceError::new(SliceErrorKind::OutOfBounds));
    }
    let bytes = unsafe { std::slice::from_raw_parts(&value as *const T as *const u8, size) };
    array[offset..][..size].copy_from_slice(bytes);
    Ok(())
}

#[track_caller]
fn rel_i32(target: u64, source: u64) -> Result<i32, SliceError> {
    let relative_offset = (target as i128) - (source as i128);
    match relative_offset.try_into() {
        Ok(offset) => Ok(offset),
        Err(_) => Err(SliceError::new(SliceErrorKind::RelI32)),
    }
}

#[track_caller]
pub fn write_rel_i32(
    asm: &mut [u8],
    location: impl Address,
    offset: u64,
    target: impl Address,
    bytes_to_next_instr: u64,
) -> Result<(), SliceError> {
    write_to_slice::<i32>(
        asm,
        offset,
        rel_i32(target.addr(), location.addr() + offset + bytes_to_next_instr)?,
    )
}

#[track_caller]
pub fn read_addr_from_slice(array: &mut [u8], offset: u64) -> Result<u64, SliceError> {
    if is_32() {
        read_from_slice::<u32>(array, offset).map(|addr| addr as u64)
    } else {
        read_from_slice::<u64>(array, offset)
    }
}

#[track_caller]
pub fn write_addr_to_slice(
    array: &mut [u8],
    offset: u64,
    addr: impl Address,
) -> Result<(), SliceError> {
    if is_32() {
        write_to_slice::<u32>(array, offset, addr.addr())
    } else {
        write_to_slice::<u64>(array, offset, addr.addr())
    }
}

#[track_caller]
pub fn get_hook_bytes(
    code_location: impl Address,
    hook_location: impl Address,
    original_instruction_size: u64,
) -> Result<Vec<u8>, SliceError> {
    let mut bytes = vec![0xe9, 0x00, 0x00, 0x00, 0x00];
    let nop_num = original_instruction_size.saturating_sub(5);
    let nops = vec![0x90; nop_num as usize];
    bytes.extend_from_slice(&nops);
    write_rel_i32(&mut bytes, hook_location, 1, code_location, 4)?;
    Ok(bytes)
}
