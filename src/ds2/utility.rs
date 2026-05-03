use anyhow::Result;
use crate::{
    core::common::{rel_i32, write_to_slice},
    ds2::{
        mem::*,
        offsets::{code_cave, functions, game_manager_imp, hooks},
        resources::asm,
        utils::is_scholar,
    },
};

pub fn quitout() -> Result<()> {
    read::<u64>(game_manager_imp::base())
        .and_then(|addr| write::<u8>(addr + game_manager_imp::quitout(), 6))
}

pub fn set_ivory_skip(state: bool) -> Result<()> {
    match state {
        true => {
            if is_scholar() {
                install_ivory_hooks_scholar()
            } else {
                install_ivory_hooks_vanilla()
            }
        }
        false => {
            if is_scholar() {
                write_bytes(functions::set_event(), &[0x48, 0x89, 0x74, 0x24, 0x10])?;
                write_bytes(hooks::set_shared_flag(), &[0x44, 0x88, 0x84, 0x08, 0xA1, 0x03, 0x00, 0x00])
            } else {
                write_bytes(functions::set_event(), &[0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x08])?;
                write_bytes(hooks::set_shared_flag(), &[0x88, 0x94, 0x08, 0xA1, 0x02, 0x00, 0x00])
            }
        }
    }
}

fn install_ivory_hooks_scholar() -> Result<()> {
    let skip_location = code_cave::base() + code_cave::IVORY_SKIP_HOOK;
    let knights_location = code_cave::base() + code_cave::IVORY_KNIGHTS_HOOK;

    let mut skip_asm = asm::IVORY_SKIP_SCHOLAR;
    write_to_slice::<u64>(&mut skip_asm, 94, functions::get_map_entity_with_area_id_and_obj_id())?;
    write_to_slice::<u64>(&mut skip_asm, 104, functions::get_map_obj_state_act_component())?;
    write_to_slice::<u64>(&mut skip_asm, 114, functions::set_event())?;
    write_to_slice::<i32>(&mut skip_asm, 215, rel_i32(functions::set_event() + 5, skip_location + 219)?)?;

    let mut knights_asm = asm::IVORY_KNIGHTS_SCHOLAR;
    write_to_slice::<i32>(&mut knights_asm, 32, rel_i32(hooks::set_shared_flag() + 8, knights_location + 36)?)?;

    let mut skip_hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00];
    write_to_slice::<i32>(&mut skip_hookbytes, 1, rel_i32(skip_location, functions::set_event() + 5)?)?;

    let mut knights_hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90, 0x90];
    write_to_slice::<i32>(&mut knights_hookbytes, 1, rel_i32(knights_location, hooks::set_shared_flag() + 5)?)?;

    write_bytes(skip_location, &skip_asm)?;
    write_bytes(knights_location, &knights_asm)?;
    write_bytes(functions::set_event(), &skip_hookbytes)?;
    write_bytes(hooks::set_shared_flag(), &knights_hookbytes)
}

fn install_ivory_hooks_vanilla() -> Result<()> {
    let skip_location = code_cave::base() + code_cave::IVORY_SKIP_HOOK;
    let knights_location = code_cave::base() + code_cave::IVORY_KNIGHTS_HOOK;

    let mut skip_asm = asm::IVORY_SKIP_VANILLA;
    write_to_slice::<u32>(&mut skip_asm, 38, functions::set_event())?;
    write_to_slice::<u32>(&mut skip_asm, 73, functions::get_map_entity_with_area_id_and_obj_id())?;
    write_to_slice::<u32>(&mut skip_asm, 80, functions::get_map_obj_state_act_component())?;
    write_to_slice::<i32>(&mut skip_asm, 162, rel_i32(functions::set_event() + 6, skip_location + 167)?)?;

    let mut knights_asm = asm::IVORY_KNIGHTS_VANILLA;
    write_to_slice::<i32>(&mut knights_asm, 28, rel_i32(hooks::set_shared_flag() + 7, knights_location + 33)?)?;

    let mut skip_hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90];
    write_to_slice::<i32>(&mut skip_hookbytes, 1, rel_i32(skip_location, functions::set_event() + 5)?)?;

    let mut knights_hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90];
    write_to_slice::<i32>(&mut knights_hookbytes, 1, rel_i32(knights_location, hooks::set_shared_flag() + 5)?)?;

    write_bytes(skip_location, &skip_asm)?;
    write_bytes(knights_location, &knights_asm)?;
    write_bytes(functions::set_event(), &skip_hookbytes)?;
    write_bytes(hooks::set_shared_flag(), &knights_hookbytes)
}

pub fn is_ivory_skip() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 5]>(functions::set_event())
            .map(|val| val != [0x48, 0x89, 0x74, 0x24, 0x10])
    } else {
        read::<[u8; 6]>(functions::set_event())
            .map(|val| val != [0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x08])
    }
}