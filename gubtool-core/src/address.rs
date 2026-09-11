use {
    crate::{
        attached,
        game_version::Game,
        sys::{
            read_pointer,
            sys_error::{SysError, SysResult},
        },
    },
    assemble::{
        AsmFunction,
        patch::{Patch, PatchSize},
    },
    std::ops::Add,
};

pub(crate) const WIN32_PREFFERED_IMAGE_BASE: u64 = 0x400000;
pub(crate) const WIN64_PREFFERED_IMAGE_BASE: u64 = 0x140000000;
// pub(crate) const SHADPS4_BASE: u64 = 0x800000000;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pointer {
    pub address: u64,
    pub game:    Option<Game>,
}

impl Pointer {
    pub const fn new(address: u64, game: Option<Game>) -> Self {
        Self {
            address,
            game,
        }
    }

    pub fn is_null(&self) -> bool {
        self.address == 0
    }
}

pub trait PointerOffset {
    fn apply(self, address: u64) -> u64;
}

impl PointerOffset for u64 {
    fn apply(self, address: u64) -> u64 {
        address.saturating_add(self)
    }
}

impl PointerOffset for i32 {
    fn apply(self, address: u64) -> u64 {
        address.saturating_add_signed(self as i64)
    }
}

impl PointerOffset for Offset {
    fn apply(self, address: u64) -> u64 {
        address.saturating_add_signed(self.resolve() as i64)
    }
}

impl<T: PointerOffset> Add<T> for Pointer {
    type Output = Pointer;

    fn add(self, rhs: T) -> Self::Output {
        Pointer {
            address: rhs.apply(self.address),
            game:    self.game,
        }
    }
}

impl PartialEq<u64> for Pointer {
    fn eq(&self, other: &u64) -> bool {
        self.address == *other
    }
}

impl PartialEq<Pointer> for u64 {
    fn eq(&self, other: &Pointer) -> bool {
        *self == other.address
    }
}

impl From<Pointer> for u64 {
    fn from(value: Pointer) -> Self {
        value.address
    }
}

impl From<u64> for Pointer {
    fn from(value: u64) -> Self {
        Pointer {
            address: value,
            game:    None,
        }
    }
}

pub trait ChainReadExt {
    fn read_ptr<T: PointerOffset>(self, offset: T) -> SysResult<Pointer>;
    fn add_offset<T: PointerOffset>(self, offset: T) -> SysResult<Pointer>;
    fn read<T>(self) -> SysResult<T>;
    fn write<T>(self, val: T) -> SysResult;
    fn is_bit_set(self, mask: u8) -> SysResult<bool>;
    fn set_bit(self, mask: u8, state: bool) -> SysResult;
}

impl ChainReadExt for SysResult<Pointer> {
    #[track_caller]
    fn read_ptr<T: PointerOffset>(self, offset: T) -> SysResult<Pointer> {
        let pointer = self.add_offset(offset)?;
        read_pointer(pointer)
    }

    fn add_offset<T: PointerOffset>(self, offset: T) -> SysResult<Pointer> {
        let pointer = self?;
        Ok(pointer + offset)
    }

    #[track_caller]
    fn read<T>(self) -> SysResult<T> {
        crate::sys::read::<T>(self?)
    }

    #[track_caller]
    fn write<T>(self, value: T) -> SysResult {
        crate::sys::write::<T>(self?, value)
    }

    #[track_caller]
    fn is_bit_set(self, mask: u8) -> SysResult<bool> {
        crate::sys::is_bit_set(self?, mask)
    }

    #[track_caller]
    fn set_bit(self, mask: u8, state: bool) -> SysResult {
        crate::sys::set_bit(self?, mask, state)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Offset {
    pub x86: i32,
    pub x64: i32,
}

impl Offset {
    pub fn resolve(&self) -> i32 {
        match attached::is_32() {
            true => self.x86,
            false => self.x64,
        }
    }
}

pub trait Address: Copy + Sized {
    fn u64_address(&self) -> u64;
    fn game(&self) -> Option<Game>;

    fn validate_game(&self) -> SysResult {
        let attached = attached::game()?;

        if let Some(game) = self.game()
            && game != attached
        {
            return Err(SysError::InvalidGame {
                expected: game,
            });
        }
        Ok(())
    }

    fn ptr(&self) -> Pointer {
        Pointer::new(self.u64_address(), self.game())
    }

    fn add(&self, offset: impl Into<i32>) -> Pointer {
        let addr = self
            .u64_address()
            .saturating_add_signed(offset.into() as i64);
        Pointer::new(addr, self.game())
    }
}

impl Address for u64 {
    fn u64_address(&self) -> u64 {
        *self
    }
    fn game(&self) -> Option<Game> {
        None
    }
}

impl Address for Pointer {
    fn u64_address(&self) -> u64 {
        self.address
    }
    fn game(&self) -> Option<Game> {
        self.game
    }
}

pub struct POINTER;

impl PatchSize for POINTER {}

impl Patch<POINTER> for u64 {
    fn patch(self, fun: &mut AsmFunction, relocation: &'static str) {
        let bytes: &[u8] = if attached::is_32() {
            &(self as u32).to_le_bytes()
        } else {
            &self.to_le_bytes()
        };
        assemble::patch::patch_asm_fun(fun, relocation, bytes);
    }
}

impl Patch<POINTER> for Pointer {
    fn patch(self, fun: &mut AsmFunction, relocation: &'static str) {
        let bytes: &[u8] = if attached::is_32() {
            &(self.address as u32).to_le_bytes()
        } else {
            &self.address.to_le_bytes()
        };
        assemble::patch::patch_asm_fun(fun, relocation, bytes);
    }
}
