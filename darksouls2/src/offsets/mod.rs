pub mod chr_ctrl;
pub mod code_cave;
pub mod game_manager_imp;
pub mod module_offsets;

use {
    crate::mem::{read, read_address, write},
    gubtool_core::{attached::is_32, sys::sys_error::ProcResult},
    pelite::Pod,
};

pub struct Offset {
    pub vanilla: u64,
    pub scholar: u64,
}

impl Offset {
    #[inline(always)]
    pub fn resolve(&self) -> u64 {
        if is_32() {
            self.vanilla
        } else {
            self.scholar
        }
    }
}

pub trait ChainReadExt {
    #[track_caller]
    fn read_offset(self, offset: Offset) -> ProcResult<u64>;
    #[track_caller]
    fn add_offset(self, offset: Offset) -> ProcResult<u64>;
    #[track_caller]
    fn read<T: Pod>(self) -> ProcResult<T>;
    #[track_caller]
    fn write<T: Pod>(self, val: T) -> ProcResult;
}

impl ChainReadExt for ProcResult<u64> {
    fn read_offset(self, offset: Offset) -> ProcResult<u64> {
        let base = self?;
        read_address(base.saturating_add(offset.resolve()))
    }
    fn add_offset(self, offset: Offset) -> ProcResult<u64> {
        let base = self?;
        Ok(base.saturating_add(offset.resolve()))
    }
    fn read<T: Pod>(self) -> ProcResult<T> {
        let addr = self?;
        read::<T>(addr)
    }
    fn write<T: Pod>(self, val: T) -> ProcResult {
        let addr = self?;
        write::<T>(addr, val)
    }
}
