use std::{thread, time::Duration};
use anyhow::Result;
use crate::{
    core::common::{rel_i32, write_to_slice},
    ds2::{
        game_state::is_loading_screen, mem::*, offsets::{
            code_cave,
            functions,
            game_manager_imp::{self, event_manager_offsets, hk_hardware_info},
            hooks,
        }, resources::{
            asm,
            warps::Warp,
        }, utils::{character_loaded_check, is_scholar}
    },
};

impl Warp {
    pub fn warp(&self) -> Result<()> {
        character_loaded_check()?;
        let event_warp_entity = follow_pointers(&[
            game_manager_imp::base(),
            game_manager_imp::event_manager(),
            event_manager_offsets::warp_event_entity(),
        ], true)?;
        if let Some(event_object_id) = self.event_object_id {
            event_warp(self.bonfire_id, event_object_id, event_warp_entity)?
        } else {
            bonfire_warp(self.bonfire_id, event_warp_entity)?
        }
        if let Some(coords) = self.coordinates {
            write_coords_hook(coords)?
        }
        Ok(())
    }
}

fn bonfire_warp(bonfire_id: i32, event_warp_entity: u64) -> Result<()> {
    let bonfire_id_location = code_cave::base() + code_cave::BONFIRE_ID;
    write::<i32>(bonfire_id_location, bonfire_id)?;
    if is_scholar() {
        bonfire_warp_scholar(event_warp_entity)
    } else {
        bonfire_warp_vanilla(event_warp_entity)
    }
}

fn bonfire_warp_scholar(event_warp_entity: u64) -> Result<()> {
    let empty_space = code_cave::base() + code_cave::EMPTY_SPACE;
    let bonfire_id_location = code_cave::base() + code_cave::BONFIRE_ID;
    let location = code_cave::base() + code_cave::BONFIRE_WARP_ASM;

    let mut asm = asm::BONFIRE_WARP_SCHOLAR;
    write_to_slice::<i32>(&mut asm, 7, rel_i32(empty_space, location + 11)?)?;
    write_to_slice::<i32>(&mut asm, 14, rel_i32(bonfire_id_location, location + 18)?)?;
    write_to_slice::<i32>(&mut asm, 25, rel_i32(functions::warp_prep(), location + 29)?)?;
    write_to_slice::<u64>(&mut asm, 31, event_warp_entity)?;
    write_to_slice::<i32>(&mut asm, 42, rel_i32(empty_space, location + 46)?)?;
    write_to_slice::<i32>(&mut asm, 47, rel_i32(functions::warp(), location + 51)?)?;
    let asm = append_flag_setter(location, &asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn bonfire_warp_vanilla(event_warp_entity: u64) -> Result<()> {
    let empty_space = code_cave::base() + code_cave::EMPTY_SPACE;
    let bonfire_id_location = code_cave::base() + code_cave::BONFIRE_ID;
    let location = code_cave::base() + code_cave::BONFIRE_WARP_ASM;

    let mut asm = asm::BONFIRE_WARP_VANILLA;
    write_to_slice::<u32>(&mut asm, 7, bonfire_id_location)?;
    write_to_slice::<u32>(&mut asm, 13, empty_space)?;
    write_to_slice::<u32>(&mut asm, 19, functions::warp_prep())?;
    write_to_slice::<u32>(&mut asm, 30, empty_space)?;
    write_to_slice::<u32>(&mut asm, 36, event_warp_entity)?;
    write_to_slice::<u32>(&mut asm, 41, functions::warp())?;
    let asm = append_flag_setter(location, &asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn event_warp(bonfire_id: i32, event_object_id: i32, event_warp_entity: u64) -> Result<()> {
    let params_location = code_cave::base() + code_cave::BONFIRE_ID;
    let mut params: [u8; 28] = [0; 28];
    write_to_slice::<i32>(&mut params, 0, 4)?;
    write_to_slice::<i32>(&mut params, 4, 5)?;
    write_to_slice::<i32>(&mut params, 8, bonfire_id)?;
    write_to_slice::<i32>(&mut params, 12, -1)?;
    write_to_slice::<i32>(&mut params, 24, event_object_id)?;
    write_bytes(params_location, &params)?;

    if is_scholar() {
        event_warp_scholar(event_warp_entity, params_location)
    } else {
        event_warp_vanilla(event_warp_entity, params_location)
    }
}

fn event_warp_scholar(event_warp_entity: u64, params_location: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::EVENT_WARP_ASM;

    let mut asm = asm::EVENT_WARP_SCHOLAR;
    write_to_slice::<u64>(&mut asm, 9, event_warp_entity)?;
    write_to_slice::<i32>(&mut asm, 20, rel_i32(params_location, location + 24)?)?;
    write_to_slice::<i32>(&mut asm, 25, rel_i32(functions::warp(), location + 29)?)?;
    let asm = append_flag_setter(location, &asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn event_warp_vanilla(event_warp_entity: u64, params_location: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::EVENT_WARP_ASM;

    let mut asm = asm::EVENT_WARP_VANILLA;
    write_to_slice::<u32>(&mut asm, 1, event_warp_entity)?;
    write_to_slice::<u32>(&mut asm, 7, params_location)?;
    write_to_slice::<u32>(&mut asm, 13, functions::warp())?;
    let asm = append_flag_setter(location, &asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn write_coords_hook(coords: &[f32; 16]) -> Result<()> {
    let coords_location = code_cave::base() + code_cave::WARP_COORDS;

    write::<[f32; 16]>(coords_location, *coords)?;

    for (idx, coord) in coords.iter().enumerate() {
        write::<f32>(coords_location + idx as u64 * 4, *coord)?;
    }

    if is_scholar() {
        write_coords_hook_scholar(coords_location)?
    } else {
        write_coords_hook_vanilla(coords_location)?
    }
    install_and_uninstall_hook()
}

fn write_coords_hook_scholar(coords_location: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::WARP_COORDS_HOOK;

    let mut asm = asm::WARP_COORD_WRITE_SCHOLAR;
    write_to_slice::<u64>(&mut asm, 10, hk_hardware_info::base())?;
    write_to_slice::<i32>(&mut asm, 55, rel_i32(coords_location, location + 59)?)?;
    write_to_slice::<i32>(&mut asm, 68, rel_i32(coords_location + 0x10, location + 72)?)?;
    write_to_slice::<i32>(&mut asm, 81, rel_i32(coords_location + 0x20, location + 85)?)?;
    write_to_slice::<i32>(&mut asm, 94, rel_i32(coords_location + 0x30, location + 98)?)?;
    write_to_slice::<i32>(&mut asm, 120, rel_i32(hooks::warp_coord_write() + 7, location + 124)?)?;

    write_bytes(location, &asm)
}

fn write_coords_hook_vanilla(coords_location: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::WARP_COORDS_HOOK;

    let mut asm = asm::WARP_COORD_WRITE_VANILLA;
    write_to_slice::<u32>(&mut asm, 9, game_manager_imp::base())?;
    write_to_slice::<u32>(&mut asm, 53, coords_location)?;
    write_to_slice::<u32>(&mut asm, 64, coords_location + 0x10)?;
    write_to_slice::<u32>(&mut asm, 75, coords_location + 0x20)?;
    write_to_slice::<u32>(&mut asm, 86, coords_location + 0x30)?;
    write_to_slice::<i32>(&mut asm, 103, rel_i32(hooks::warp_coord_write() + 7, location + 107)?)?;

    write_bytes(location, &asm)
}

fn install_and_uninstall_hook() -> Result<()> {
    while !is_loading_screen()? {
        thread::sleep(Duration::from_millis(100));
    }
    thread::sleep(Duration::from_millis(200));

    let location = code_cave::base() + code_cave::WARP_COORDS_HOOK;
    let mut hookbytes: [u8; 7] = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::warp_coord_write() + 5)?)?;
    write_bytes(hooks::warp_coord_write(), &hookbytes)?;

    while is_loading_screen()? {
        thread::sleep(Duration::from_millis(100));
    }
    thread::sleep(Duration::from_millis(200));

    if is_scholar() {
        write_bytes(hooks::warp_coord_write(), &[0x0F, 0x5C, 0xC2, 0x0F, 0x29, 0x47, 0x50])
    } else {
        write_bytes(hooks::warp_coord_write(), &[0x0F, 0x5C, 0xC1, 0x0F, 0x29, 0x46, 0x40])
    }
}