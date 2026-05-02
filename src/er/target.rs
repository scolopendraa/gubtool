use crate::{
    core::common::{rel_i32, write_to_slice},
    er::{
        chr_ins::ChrIns,
        mem::*,
        offsets::{code_cave, field_area, hooks},
        resources::asm,
    },
};
use anyhow::{Result, anyhow, bail, ensure};

pub fn target_ins() -> ChrIns {
    let addr = read::<u64>(code_cave::base() + code_cave::TARGET_POINTER)
        .map_err(|_| anyhow!("Could not read target pointer"))?;
    ensure!(addr != 0x0, "Target not found");
    Ok(addr)
}

pub fn install_target_hook() -> Result<()> {
    let location = code_cave::base() + code_cave::TARGET_POINTER_HOOK;
    let saved_pointer = code_cave::base() + code_cave::TARGET_POINTER;

    let mut asm = asm::TARGET_HOOK;
    write_to_slice::<i32>(&mut asm, 10, rel_i32(saved_pointer, location + 14)?)?;
    write_to_slice::<i32>(&mut asm, 15, rel_i32(hooks::locked_target_pointer() + 7, location + 19)?)?;

    let mut hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::locked_target_pointer() + 5)?)?;

    write_bytes(location, &asm)?;
    write_bytes(hooks::locked_target_pointer(), &hookbytes)
}

pub fn uninstall_target_hook() -> Result<()> {
    write_bytes(hooks::locked_target_pointer(), &asm::TARGET_HOOK_BYTES_ORIGINAL)
}

pub fn is_target_hook_active() -> Result<bool> {
    read::<[u8; 7]>(hooks::locked_target_pointer())
        .map(|val| val != asm::TARGET_HOOK_BYTES_ORIGINAL)
}

pub fn force_act_sequence(act_sequence: Vec<i32>, npc_think_param_id: i32) -> Result<()> {
    ensure!(act_sequence.len() <= 10, "Max number of acts is 10");

    let location = code_cave::base() + code_cave::FORCE_ACT_SEQUENCE_HOOK;
    let current_idx_location = code_cave::base() + code_cave::CURRENT_IDX;
    let should_run_flag_location = code_cave::base() + code_cave::SHOULD_RUN;
    let act_array_location = code_cave::base() + code_cave::ACT_ARRAY;

    let mut act_array = act_sequence;
    act_array.resize(10, 0);
    let act_array: Vec<u8> = act_array.iter().flat_map(|&x| x.to_le_bytes()).collect();

    let mut asm = asm::FORCE_ACT_SEQUENCE;
    write_to_slice::<i32>(&mut asm, 2, rel_i32(should_run_flag_location, location + 7)?)?;
    write_to_slice::<i32>(&mut asm, 12, npc_think_param_id)?;
    write_to_slice::<i32>(&mut asm, 23, rel_i32(current_idx_location, location + 27)?)?;
    write_to_slice::<i32>(&mut asm, 30, rel_i32(act_array_location, location + 34)?)?;
    write_to_slice::<i32>(&mut asm, 42, rel_i32(current_idx_location, location + 46)?)?;
    write_to_slice::<i32>(&mut asm, 53, rel_i32(should_run_flag_location, location + 58)?)?;
    write_to_slice::<i32>(&mut asm, 62, rel_i32(hooks::get_force_act_idx() + 7, location + 66)?)?;
    write_to_slice::<[u8; 7]>(&mut asm, 66, asm::get_force_act_idx_original_bytes())?;
    write_to_slice::<i32>(&mut asm, 74, rel_i32(hooks::get_force_act_idx() + 7, location + 78)?)?;

    let mut hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::get_force_act_idx() + 5)?)?;

    write::<i32>(current_idx_location, 0x0)?;
    write_bytes(act_array_location, &act_array)?;

    write_bytes(location, &asm)?;
    write::<u8>(should_run_flag_location, 0x1)?;
    write_bytes(hooks::get_force_act_idx(), &hookbytes)
}

pub fn install_stagger_hook() -> Result<()> {
    let location = code_cave::base() + code_cave::TARGET_NO_STAGGER_HOOK;
    let target_ptr_location = code_cave::base() + code_cave::TARGET_POINTER;

    let mut asm = asm::TARGET_STAGGER_HOOK;
    write_to_slice::<i32>(&mut asm, 8, rel_i32(target_ptr_location, location + 12)?)?;
    write_to_slice::<i32>(&mut asm, 24, rel_i32(hooks::target_no_stagger() + 8, location + 28)?)?;

    let mut hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::target_no_stagger() + 5)?)?;

    write_bytes(location, &asm)?;
    write_bytes(hooks::target_no_stagger(), &hookbytes)
}

pub fn uninstall_stagger_hook() -> Result<()> {
    write_bytes(hooks::target_no_stagger(), &asm::TARGET_STAGGER_HOOK_BYTES_ORIGINAL)
}

pub fn is_stagger_hook_active() -> Result<bool> {
    read::<[u8; 8]>(hooks::target_no_stagger())
        .map(|val| val != asm::TARGET_STAGGER_HOOK_BYTES_ORIGINAL)
}

pub fn toggle_stagger_hook() -> Result<()> {
    match is_stagger_hook_active()? {
        true => uninstall_stagger_hook(),
        false => install_stagger_hook(),
    }
}

pub fn world_block_info_from_block_id(block_id: u32) -> Result<u64> {
    let target_area = (block_id >> 24) & 0xFF;
    let world_info_owner = read::<u64>(field_area::base())
        .and_then(|addr| read::<u64>(addr + field_area::WORLD_INFO_OWNER))?;
    let area_count = read::<i32>(world_info_owner + field_area::world_info_owner_offsets::AREA_COUNT)?;

    for i in 0..area_count as u64 {
        let area_ptr = read::<u64>(world_info_owner + field_area::world_info_owner_offsets::AREA_ARRAY_BASE + (i * 8))?;
        let area_id = read::<u32>(area_ptr + 0xC)?;

        if area_id == target_area {
            let block_count = read::<i32>(area_ptr + 0x40)?;
            let blocks_ptr = read::<u64>(area_ptr + 0x48)?;

            for j in 0..block_count as u64 {
                let block_info_ptr = blocks_ptr + (j * 0xE0);
                let stored_block_id = read::<u32>(block_info_ptr + 0x8)?;

                if stored_block_id == block_id {
                    return Ok(block_info_ptr);
                }
            }
        }
    }
    bail!("Could not find world block info")
}
