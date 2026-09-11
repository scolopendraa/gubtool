pub mod dll;
pub mod import_macro;
pub mod ipc;
pub mod pid;
pub mod sys_error;

pub use pid::*;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::*;
use {
    crate::{
        address::{Address, Pointer},
        attached::is_32,
        slice_ops::get_hook_bytes,
        sys::sys_error::SysResult,
    },
    assemble::AsmFolder,
    std::sync::LazyLock,
};

static ASM_LIB32_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sys32.bin"));
static ASM_LIB64_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sys64.bin"));
static ASM32: LazyLock<AsmFolder> =
    LazyLock::new(|| bincode::deserialize(ASM_LIB32_BYTES).unwrap());
static ASM64: LazyLock<AsmFolder> =
    LazyLock::new(|| bincode::deserialize(ASM_LIB64_BYTES).unwrap());

#[track_caller]
pub fn read_pointer(pointer: impl Address) -> SysResult<Pointer> {
    let address = if is_32() {
        read::<u32>(pointer).map(|addr| addr as u64)
    } else {
        read::<u64>(pointer)
    }?;

    Ok(Pointer::new(address, pointer.game()))
}

#[track_caller]
pub fn write_address(location: impl Address, address: impl Address) -> SysResult {
    if is_32() {
        write::<u32>(location, address.u64_address() as u32)
    } else {
        write::<u64>(location, address.u64_address())
    }
}

#[track_caller]
pub fn is_bit_set(pointer: impl Address, mask: u8) -> SysResult<bool> {
    read::<u8>(pointer).map(|byte| byte & mask != 0)
}

#[track_caller]
pub fn set_bit(pointer: impl Address, mask: u8, state: bool) -> SysResult {
    let current_byte = read::<u8>(pointer)?;

    let new_byte = match state {
        true => current_byte | mask,
        false => current_byte & !mask,
    };

    write::<u8>(pointer, new_byte)
}

#[track_caller]
pub fn install_hook(
    code: &[u8],
    code_location: impl Address,
    hook_location: impl Address,
    original_instruction_size: u64,
) -> SysResult {
    hook_location.validate_game()?;

    let hookbytes = get_hook_bytes(code_location, hook_location, original_instruction_size)?;
    write_bytes(code_location, &code)?;
    write_bytes(hook_location, &hookbytes)
}

#[track_caller]
pub fn install_hook_without_code(
    code_location: impl Address,
    hook_location: impl Address,
    original_instruction_size: u64,
) -> SysResult {
    hook_location.validate_game()?;

    let hookbytes = get_hook_bytes(code_location, hook_location, original_instruction_size)?;
    write_bytes(hook_location, &hookbytes)
}

pub struct PointerChain<A: Address> {
    pub start:      A,
    pub offsets:    &'static [i32],
    pub read_final: bool,
}

impl<A: Address> PointerChain<A> {
    #[track_caller]
    pub fn follow_pointers(&self) -> SysResult<Pointer> {
        let mut pointer = read_pointer(self.start)?;

        let (last, rest) = self.offsets.split_last().unwrap();
        for offset in rest {
            pointer = read_pointer(pointer + *offset)?
        }

        if self.read_final {
            pointer = read_pointer(pointer + *last)?
        } else {
            pointer = pointer + *last
        }

        Ok(pointer)
    }
}

pub fn print_asm_sizes() {
    println!("Core");
    ASM64.print_function_sizes();
    println!("\n");
}
