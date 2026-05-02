use anyhow::Result;

use crate::{
    core::common::write_to_slice,
    ds2::{
        mem::*,
        offsets::{
            code_cave, functions,
            game_manager_imp::{self, chr_ctrl_offsets, px_world_offsets},
        },
        resources::asm,
        utils::is_scholar,
    },
};

pub fn player_ctrl() -> Result<u64> {
    follow_pointers(&[
        game_manager_imp::base(),
        game_manager_imp::player_ctrl(),
    ], true)
}

pub fn give_souls(amount: i32) -> Result<()> {
    let stats_entity = follow_pointers(&[
        game_manager_imp::base(),
        game_manager_imp::player_ctrl(),
        chr_ctrl_offsets::stats_ptr(),
    ], true)?;
    if is_scholar() {
        give_souls_scholar(amount, stats_entity)
    } else {
        give_souls_vanilla(amount, stats_entity)
    }
}

fn give_souls_scholar(amount: i32, stats_entity: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::SOULS_GIVE_ASM;

    let mut asm = asm::GIVE_SOULS_SCHOLAR;
    write_to_slice::<u64>(&mut asm, 2, stats_entity)?;
    write_to_slice::<i64>(&mut asm, 12, amount)?;
    write_to_slice::<u64>(&mut asm, 22, functions::give_souls())?;
    let asm = append_flag_setter(location, &asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn give_souls_vanilla(amount: i32, stats_entity: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::SOULS_GIVE_ASM;

    let mut asm = asm::GIVE_SOULS_VANILLA;
    write_to_slice::<i32>(&mut asm, 1, amount)?;
    write_to_slice::<u32>(&mut asm, 7, stats_entity)?;
    write_to_slice::<u32>(&mut asm, 12, functions::give_souls())?;
    let asm = append_flag_setter(location, &asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

pub fn position_pointer() -> Result<[f32; 16]> {
    let mut pointers = vec![
        game_manager_imp::base(),
        game_manager_imp::px_world(),
    ];
    pointers.extend_from_slice(&px_world_offsets::player_coords_chain());
    let pointer = follow_pointers(&pointers, false)?;
    read::<[f32; 16]>(pointer)
}