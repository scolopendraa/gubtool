pub mod chr_ctrl;
pub mod code_cave;
pub mod game_manager_imp;
pub mod module_offsets;

use {
    crate::mem::{read, read_address, write},
    gubtool_core::{attached::is_32, sys::sys_error::SysResult},
};

pub struct Offset {
    pub vanilla: u64,
    pub scholar: u64,
}

impl Offset {
    #[inline(always)]
    pub fn resolve(&self) -> u64 {
        if is_32() { self.vanilla } else { self.scholar }
    }
}

pub trait ChainReadExt {
    #[track_caller]
    fn read_offset(self, offset: Offset) -> SysResult<u64>;
    #[track_caller]
    fn add_offset(self, offset: Offset) -> SysResult<u64>;
    #[track_caller]
    fn read<T>(self) -> SysResult<T>;
    #[track_caller]
    fn write<T>(self, val: T) -> SysResult;
}

impl ChainReadExt for SysResult<u64> {
    fn read_offset(self, offset: Offset) -> SysResult<u64> {
        let base = self?;
        read_address(base.saturating_add(offset.resolve()))
    }
    fn add_offset(self, offset: Offset) -> SysResult<u64> {
        let base = self?;
        Ok(base.saturating_add(offset.resolve()))
    }
    fn read<T>(self) -> SysResult<T> {
        let addr = self?;
        read::<T>(addr)
    }
    fn write<T>(self, val: T) -> SysResult {
        let addr = self?;
        write::<T>(addr, val)
    }
}
