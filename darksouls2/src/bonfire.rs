use {
    crate::{
        mem::*,
        offsets::{
            ChainReadExt,
            game_manager_imp::event_manager_offsets::{self, bonfire_manager_offsets},
            module_offsets::Function,
        },
        pointer_cache::ResolvedPtr,
        resources::{asm_function, bonfires::Bonfire},
    },
    gubtool_core::{
        attached::is_32,
        slice_ops::*,
        sys::{
            ipc::{CppValue, X86CallingConvention},
            sys_error::ProcResult,
        },
    },
};

impl Bonfire {
    pub fn unlock(&self) -> anyhow::Result<()> {
        light_bonfire(self.bonfire_id)
    }
    pub fn light(&self) -> anyhow::Result<()> {
        light_bonfire(self.bonfire_id)
    }
    pub fn rest(&self) -> anyhow::Result<()> {
        rest_at_bonfire(self)
    }
    pub fn is_lit(&self) -> ProcResult<bool> {
        is_bonfire_lit(self.bonfire_id)
    }
}

pub fn get_last_bonfire_id() -> ProcResult<u32> {
    ResolvedPtr::EventManager
        .get()
        .add_offset(event_manager_offsets::RESPAWN_BONFIRE)
        .read::<u32>()
}

pub fn light_all_bonfires() -> anyhow::Result<()> {
    let mut fun = asm_function("bonfire_unlock_all");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(
        &mut asm,
        fun.reloc("bonfire_manager"),
        ResolvedPtr::BonfireManager.get()?,
    )?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_bonfire_unlock"), Function::BonfireUnlock)?;

    run_custom_function(asm)
}

fn light_bonfire(bonfire_id: u32) -> anyhow::Result<()> {
    let bonfire_manager = ResolvedPtr::BonfireManager.get()?;

    let args = [
        CppValue::uintptr_t(bonfire_manager),
        CppValue::uint16_t(bonfire_id as u16),
        CppValue::uint8_t(0x1), // show popup
    ];

    run_game_function(Function::BonfireUnlock, &args, X86CallingConvention::__thiscall)
}

fn is_bonfire_lit(bonfire_id: u32) -> ProcResult<bool> {
    let Some(addr) = bonfire_handle_from_id(bonfire_id)? else {
        return Ok(false)
    };
    read::<u8>(addr + 0x2).map(|val| val != 0)
}

fn bonfire_handle_from_id(bonfire_id: u32) -> ProcResult<Option<u64>> {
    let bonfire_manager = ResolvedPtr::BonfireManager.get()?;
    let size = if is_32() {
        0x10
    } else {
        0x18
    };

    let array_ptr = read_address(bonfire_manager + bonfire_manager_offsets::ARRAY_BASE.resolve())?;
    let mut high = read::<i32>(bonfire_manager + bonfire_manager_offsets::COUNT.resolve())? - 1;
    let mut low = 0;

    while low <= high {
        let mid = low + ((high - low) >> 1);
        let entry_id = read::<u16>(array_ptr + (mid as u64) * size)? as u32;

        if bonfire_id < entry_id {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        } else if bonfire_id > entry_id {
            low = mid + 1;
        } else {
            return Ok(Some(array_ptr + (mid as u64) * size));
        }
    }
    Ok(None)
}

fn rest_at_bonfire(bonfire: &Bonfire) -> anyhow::Result<()> {
    let bonfire_manager = ResolvedPtr::BonfireManager.get()?;
    let respawn_map_loc = ResolvedPtr::EventManager
        .get()
        .add_offset(event_manager_offsets::RESPAWN_MAP)?;

    let args = [
        CppValue::uintptr_t(bonfire_manager),
        CppValue::uint32_t(bonfire.bonfire_id),
    ];

    let has_rested = 0x0;

    write::<[u32; 3]>(respawn_map_loc, [bonfire.map_id as u32, has_rested, bonfire.bonfire_id])?;

    run_game_function(Function::BonfireRest, &args, X86CallingConvention::__thiscall)
}
