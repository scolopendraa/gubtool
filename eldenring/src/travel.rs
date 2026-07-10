use crate::{
    emevd, event,
    mem::{is_bit_set, read, read_bytes, spawn_thread_join, write, write_bytes, install_hook},
    offsets::{
        ChainReadExt,
        code_cave::CaveOffset,
        menu_man,
        module_offsets::{BasePointer, Function, Hook},
    },
    resources::{ASM, bosses::Boss, graces::Grace},
    utils::{dlc_check, player_loaded_check},
};
use gubtool_core::{address::Address, slice_ops::*, sys::error::ProcResult};
use std::{sync::Mutex, time::Duration};

/// Saved original hook bytes, read at install time and restored at cleanup.
static COORD_HOOK_ORIGINAL: Mutex<Option<[u8; 7]>> = Mutex::new(None);
static ANGLE_HOOK_ORIGINAL: Mutex<Option<[u8; 7]>> = Mutex::new(None);

/// Clear saved original hook bytes.
/// Called on game detach to prevent stale bytes from being written
/// if the tool re-attaches to a different game version.
pub fn reset_warp_hook_state() {
    let mut coord_original = COORD_HOOK_ORIGINAL.lock().unwrap();
    let mut angle_original = ANGLE_HOOK_ORIGINAL.lock().unwrap();
    *coord_original = None;
    *angle_original = None;
}

pub fn warp_to_grace(grace_id: i64) -> ProcResult {
    let mut fun = ASM.get_function("warp_to_grace");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("world_chr_man"), BasePointer::WorldChrMan)?;
    write_to_slice::<i64>(&mut asm, fun.reloc("grace_id"), grace_id)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_grace_warp"), Function::GraceWarp)?;

    spawn_thread_join(CaveOffset::GraceWarpAsm, asm)
}

pub async fn warp_to_block_id(block_id: i32, coords: [f32; 3], angle: f32, is_night: bool) -> ProcResult {
    let area: i32 = (block_id >> 24) & 0xFF;
    let block: i32 = (block_id >> 16) & 0xFF;
    let map: i32 = (block_id >> 8) & 0xFF;
    let alt_no: i32 = block_id & 0xFF;

    let mut fun = ASM.get_function("warp_to_block_id");
    let mut asm = fun.take_bytes();

    write_to_slice::<i32>(&mut asm, fun.reloc("area"), area)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("block"), block)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("map"), map)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("alt_no"), alt_no)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_block_warp"), Function::BlockWarp)?;

    spawn_thread_join(CaveOffset::BlockWarpAsm, asm)?;
    hook_warp_coord_writes(coords, angle, is_night).await
}

async fn hook_warp_coord_writes(coords: [f32; 3], angle: f32, is_night: bool) -> ProcResult {
    let mut target_coords: [u8; 16] = [0; 16];
    write_to_slice::<f32>(&mut target_coords, 0, coords[0])?;
    write_to_slice::<f32>(&mut target_coords, 4, coords[1])?;
    write_to_slice::<f32>(&mut target_coords, 8, coords[2])?;
    write_to_slice::<f32>(&mut target_coords, 12, 1.0)?;

    write_bytes(CaveOffset::WarpCoords, &target_coords)?;
    write::<f32>(CaveOffset::WarpAngle.add_offset(4), angle)?;

    let mut fun = ASM.get_function("warp_coord_angle_hook");
    let mut asm = fun.take_bytes();

    let code_loc = CaveOffset::WarpCoordsHook;
    write_rel_i32(&mut asm, code_loc, fun.reloc("new_val"), CaveOffset::WarpCoords, 4)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("property_offset"), 0xAA0)?;
    write_rel_i32(&mut asm, code_loc, fun.reloc("hook_loc"), Hook::WarpCoordWrite.add_offset(7), 4)?;
    // Save original bytes before installing the hook
    {
        let mut coord_original = COORD_HOOK_ORIGINAL.lock().unwrap();
        let bytes = read_bytes(Hook::WarpCoordWrite, 7)?;
        *coord_original = Some(bytes.try_into().map_err(|_| {
            gubtool_core::sys::error::ProcessError::partial_access(
                gubtool_core::sys::error::AccessType::Read("warp_coord_original"),
                Hook::WarpCoordWrite.addr() as usize,
                7,
            )
        })?);
    }
    install_hook(&asm, code_loc, Hook::WarpCoordWrite, 7)?;

    let mut fun = ASM.get_function("warp_coord_angle_hook");
    let mut asm = fun.take_bytes();

    let code_loc = CaveOffset::WarpAngleHook;
    write_rel_i32(&mut asm, code_loc, fun.reloc("new_val"), CaveOffset::WarpAngle, 4)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("property_offset"), 0xAB0)?;
    write_rel_i32(&mut asm, code_loc, fun.reloc("hook_loc"), Hook::WarpAngleWrite.add_offset(7), 4)?;
    // Save original bytes before installing the hook
    {
        let mut angle_original = ANGLE_HOOK_ORIGINAL.lock().unwrap();
        let bytes = read_bytes(Hook::WarpAngleWrite, 7)?;
        *angle_original = Some(bytes.try_into().map_err(|_| {
            gubtool_core::sys::error::ProcessError::partial_access(
                gubtool_core::sys::error::AccessType::Read("warp_angle_original"),
                Hook::WarpAngleWrite.addr() as usize,
                7,
            )
        })?);
    }
    install_hook(&asm, code_loc, Hook::WarpAngleWrite, 7)?;

    wait_to_unhook_warp(is_night).await
}

/// Cleanup warp hooks by restoring original bytes saved at install time.
/// This should be called when the tool detaches or when a warp fails.
/// Uses dynamically-saved original bytes rather than hardcoded constants
/// to handle cases where the game or another tool modified the bytes
/// between install and cleanup.
pub fn cleanup_warp_hooks() -> ProcResult {
    let mut coord_original = COORD_HOOK_ORIGINAL.lock().unwrap();
    let mut angle_original = ANGLE_HOOK_ORIGINAL.lock().unwrap();
    
    if let Some(bytes) = *coord_original {
        write_bytes(Hook::WarpCoordWrite, &bytes)?;
    }
    if let Some(bytes) = *angle_original {
        write_bytes(Hook::WarpAngleWrite, &bytes)?;
    }
    
    // Clear saved bytes after cleanup
    *coord_original = None;
    drop(coord_original);
    *angle_original = None;
    
    Ok(())
}

async fn wait_to_unhook_warp(is_night: bool) -> ProcResult {
    let is_faded_ptr = read::<u64>(BasePointer::MenuMan)
        .add_offset(menu_man::is_fading())?;

    while !is_bit_set(is_faded_ptr, menu_man::fade_bit_flags::IS_FADE_SCREEN)? {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    while is_bit_set(is_faded_ptr, menu_man::fade_bit_flags::IS_FADE_SCREEN)? {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    if is_night {
        emevd::set_night()?;
    }
    // Restore original bytes saved at install time
    let coord_original = COORD_HOOK_ORIGINAL.lock().unwrap();
    let angle_original = ANGLE_HOOK_ORIGINAL.lock().unwrap();
    if let Some(bytes) = *coord_original {
        write_bytes(Hook::WarpCoordWrite, &bytes)?;
    }
    if let Some(bytes) = *angle_original {
        write_bytes(Hook::WarpAngleWrite, &bytes)?;
    }
    Ok(())
}

impl Boss {
    pub async fn warp(&self) -> anyhow::Result<()> {
        player_loaded_check()?;
        if self.dlc {
            dlc_check()?;
        }
        if self.name == "Grafted Scion" && !event::get_event(10010801)? {
            warp_to_block_id(self.block_id, [-33.27, 21.37, -87.86], 2.92, self.is_night).await?;
        } else {
            warp_to_block_id(self.block_id, self.coords, self.angle, self.is_night).await?;
        }
        Ok(())
    }
}

impl Grace {
    pub fn warp(&self) -> anyhow::Result<()> {
        player_loaded_check()?;
        if self.dlc {
            dlc_check()?;
        }
        warp_to_grace(self.grace_entity_id)?;
        Ok(())
    }
}