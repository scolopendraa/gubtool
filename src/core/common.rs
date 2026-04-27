use anyhow::{Result, anyhow, bail};
use pelite::Pod;

#[track_caller]
pub fn read_from_slice<T: Pod>(array: &[u8], offset: u64) -> Result<T> {
    let file_location = std::panic::Location::caller();
    let offset = offset as usize;
    let size = std::mem::size_of::<T>();
    let end = offset.checked_add(size)
        .ok_or_else(|| anyhow::anyhow!("{}:{}: offset overflow",
            file_location.file(),
            file_location.line(),
            ))?;
    let bytes = array
        .get(offset..end)
        .ok_or_else(|| anyhow::anyhow!("{}:{}: out of bounds read",
            file_location.file(),
            file_location.line(),
            ))?;
    Ok(unsafe {
        std::ptr::read_unaligned(bytes.as_ptr() as *const T)
    })
}

#[track_caller]
pub fn write_to_slice<T: Pod>(array: &mut [u8], offset: u64, value: impl TryInto<T>) -> Result<()> {
    let file_location = std::panic::Location::caller();
    let offset = offset as usize;
    let value: T = value.
        try_into()
        .map_err(|_| anyhow!("{}:{}: type conversion failed",
                file_location.file(),
                file_location.line(),
        ))?;
    let size = std::mem::size_of::<T>();
    if offset + size > array.len() {
        bail!("{}:{}: write out of bounds",
            file_location.file(),
            file_location.line(),
        )
    }
    let bytes = unsafe {
        std::slice::from_raw_parts(&value as *const T as *const u8, size)
    };
    array[offset..][..size].copy_from_slice(bytes);
    Ok(())
}

#[track_caller]
pub fn rel_i32(target: u64, source: u64) -> Result<i32> {
    let file_location = std::panic::Location::caller();
    let relative_offset = (target as i128) - (source as i128);
    relative_offset
        .try_into()
        .map_err(|_| anyhow!("{}:{}: relative offset outside i32 range",
                file_location.file(),
                file_location.line(),
        ))
}