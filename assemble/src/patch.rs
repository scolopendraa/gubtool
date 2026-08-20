use crate::AsmFunction;

pub trait PatchSize {}

pub struct BYTE;
pub struct WORD;
pub struct DWORD;
pub struct QWORD;

impl PatchSize for BYTE {}
impl PatchSize for WORD {}
impl PatchSize for DWORD {}
impl PatchSize for QWORD {}

pub fn patch_asm_fun(fun: &mut AsmFunction, relocation: &'static str, bytes: &[u8]) {
    let reloc = fun.reloc(relocation) as usize;
    fun.bytes[reloc..reloc + bytes.len()].copy_from_slice(bytes);
}

pub trait Patch<P: PatchSize> {
    fn patch(self, fun: &mut AsmFunction, relocation: &'static str);
}

impl AsmFunction {
    pub fn patch<P: PatchSize>(&mut self, relocation: &'static str, data: impl Patch<P>) {
        data.patch(self, relocation);
    }

    pub fn patch_rel32<T, U>(
        &mut self,
        relocation: &'static str,
        source_addr: T,
        target_addr: U,
        bytes_to_next_instr: u8,
    ) where
        T: Into<u64>,
        U: Into<u64>,
    {
        let source = source_addr.into() as i128;
        let target = target_addr.into() as i128;
        let reloc = self.reloc(relocation) as i128;

        let relative_offset = target - source - reloc - bytes_to_next_instr as i128;
        let rel_i32: i32 = relative_offset.try_into().unwrap();
        let bytes = rel_i32.to_le_bytes();

        let offset = reloc as usize;
        self.bytes[offset..offset + bytes.len()].copy_from_slice(&bytes);
    }
}

macro_rules! impl_patch {
    ($patch_size:ident, $ty:ty) => {
        impl Patch<$patch_size> for $ty {
            fn patch(self, fun: &mut AsmFunction, relocation: &'static str) {
                patch_asm_fun(fun, relocation, &self.to_le_bytes());
            }
        }
    };
}

impl_patch!(BYTE, u8);
impl_patch!(BYTE, i8);
impl_patch!(WORD, u16);
impl_patch!(WORD, i16);
impl_patch!(DWORD, u32);
impl_patch!(DWORD, i32);
impl_patch!(DWORD, f32);
impl_patch!(QWORD, u64);
impl_patch!(QWORD, i64);
impl_patch!(QWORD, f64);
