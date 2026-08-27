use {
    crate::{
        mem::{run_custom_function_in_thread, write_bytes},
        offsets::{
            code_cave::CaveAddr,
            module_offsets::{BasePointer, Function},
        },
        resources::ASM,
        utils::player_loaded_check,
    },
    assemble::patch::DWORD,
    gubtool_core::{address::POINTER, slice_ops::*},
};

fn execute_emevd_command(group_id: i32, command_id: i32, args: &[u8]) -> anyhow::Result<()> {
    let mut fun = ASM.get_function("execute_emevd_command");

    fun.patch::<POINTER>("fn_emk_event_ins_ctor", Function::EmkEventInsCtor);
    fun.patch::<DWORD>("group_id", group_id);
    fun.patch::<DWORD>("command_id", command_id);
    fun.patch::<POINTER>("args_loc", CaveAddr::EmevdArgs);
    fun.patch::<POINTER>("cs_emk_system", BasePointer::CsEmkSystem);
    fun.patch::<POINTER>("fn_emevd_switch", Function::EmevdSwitch);

    write_bytes(CaveAddr::EmevdArgs, args)?;
    run_custom_function_in_thread(fun)
}

pub fn set_night() -> anyhow::Result<()> {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u8>(&mut param_data, 0, 20)?;
    write_to_slice::<u8>(&mut param_data, 5, 1)?;
    write_to_slice::<f32>(&mut param_data, 8, 0.75_f32)?;
    write_to_slice::<f32>(&mut param_data, 12, 2.0_f32)?;
    write_to_slice::<f32>(&mut param_data, 16, 0.0_f32)?;
    execute_emevd_command(2001, 4, &param_data)
}

pub fn rest() -> anyhow::Result<()> {
    player_loaded_check()?;
    execute_emevd_command(2004, 47, &[])
}

fn disable_title_card() -> anyhow::Result<()> {
    execute_emevd_command(2012, 8, &[])
}

pub fn reset_character_position(entity_id: u32) -> anyhow::Result<()> {
    player_loaded_check()?;
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u32>(&mut param_data, 0, entity_id)?;
    execute_emevd_command(2004, 81, &param_data)
}

pub fn force_animation_playback(
    entity_id: u32,
    animation_id: u32,
    should_loop: bool,
    should_wait_for_completion: bool,
    ignore_wait_for_transition: bool,
) -> anyhow::Result<()> {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u32>(&mut param_data, 0, entity_id)?;
    write_to_slice::<u32>(&mut param_data, 4, animation_id)?;
    write_to_slice::<u8>(&mut param_data, 8, should_loop as u8)?;
    write_to_slice::<u8>(&mut param_data, 9, should_wait_for_completion as u8)?;
    write_to_slice::<u8>(&mut param_data, 10, ignore_wait_for_transition as u8)?;
    execute_emevd_command(2003, 18, &param_data)
}
