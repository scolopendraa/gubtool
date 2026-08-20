use {
    crate::attached,
    assemble::{
        AsmFunction,
        patch::{Patch, PatchSize},
    },
};

pub(crate) const WIN32_PREFFERED_IMAGE_BASE: u64 = 0x400000;
pub(crate) const WIN64_PREFFERED_IMAGE_BASE: u64 = 0x140000000;
// pub(crate) const SHADPS4_BASE: u64 = 0x800000000;

pub trait Address: Copy {
    fn addr(&self) -> u64;

    fn add(&self, offset: u64) -> u64 {
        self.addr().saturating_add(offset)
    }
    fn sub(&self, offset: u64) -> u64 {
        self.addr().saturating_sub(offset)
    }
}

impl Address for u64 {
    fn addr(&self) -> u64 {
        *self
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

#[macro_export]
macro_rules! impl_address_patch {
    ($($type: ident),* $(,)?) => {
        $(
            impl assemble::patch::Patch<gubtool_core::address::POINTER> for $type {
                fn patch(self, fun: &mut assemble::asm_folder::AsmFunction, relocation: &'static str) {
                    let addr = self.addr();
                    let bytes: &[u8] = if gubtool_core::attached::is_32() {
                        &(addr as u32).to_le_bytes()
                    } else {
                        &addr.to_le_bytes()
                    };
                    assemble::patch::patch_asm_fun(fun, relocation, bytes);
                }
            }
            impl From<$type> for u64 {
                fn from(value: $type) -> u64 {
                    value.addr()
                }
            }
        )*
    }
}
