use crate::{
    chr_ins::ChrInsExt,
    mem::{EXECUTE_EMEVD_COMMAND_MUTEX, read, spawn_thread_join, write, write_bytes},
    offsets::{
        code_cave::CaveOffset,
        module_offsets::{BasePointer, Function},
    },
    player::player_ins,
    resources::ASM,
    utils::player_loaded_check,
};
use gubtool_core::{slice_ops::*, sys::error::ProcResult};
use crate::offsets::game_data_man;

fn execute_emevd_command(group_id: i32, command_id: i32, args: &[u8]) -> ProcResult {
    let mut fun = ASM.get_function("execute_emevd_command");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("fn_emk_event_ins_ctor"), Function::EmkEventInsCtor)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("group_id"), group_id)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("command_id"), command_id)?;
    write_addr_to_slice(&mut asm, fun.reloc("args_location"), CaveOffset::EmevdArgs)?;
    write_addr_to_slice(&mut asm, fun.reloc("cs_emk_system_base"), BasePointer::CsEmkSystem)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_emevd_switch"), Function::EmevdSwitch)?;

    // Only hold the mutex for the write_bytes call, not spawn_thread_join.
    // spawn_thread_join is non-blocking and returns immediately, so holding
    // the mutex across the spawn would unnecessarily serialize concurrent
    // emevd command execution and could cause deadlocks if the spawned
    // thread tries to acquire the same mutex.
    {
        let _handle = EXECUTE_EMEVD_COMMAND_MUTEX.lock().unwrap();
        write_bytes(CaveOffset::EmevdArgs, args)?;
    }
    spawn_thread_join(CaveOffset::EmevdAsm, asm)
}

pub fn set_night() -> ProcResult {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u8>(&mut param_data, 0, 20)?;
    write_to_slice::<u8>(&mut param_data, 5, 1)?;
    write_to_slice::<f32>(&mut param_data, 8, 0.75)?;
    write_to_slice::<f32>(&mut param_data, 12, 2.0)?;
    write_to_slice::<f32>(&mut param_data, 16, 0.0)?;
    execute_emevd_command(2001, 4, &param_data)
}

pub fn rest() -> ProcResult {
    player_loaded_check()?;
    execute_emevd_command(2004, 47, &[])
}


pub fn disable_title_card() -> ProcResult {
    execute_emevd_command(2012, 8, &[])
}

pub fn reset_character_position(entity_id: u32) -> ProcResult {
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
) -> ProcResult {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u32>(&mut param_data, 0, entity_id)?;
    write_to_slice::<u32>(&mut param_data, 4, animation_id)?;
    write_to_slice::<u8>(&mut param_data, 8, should_loop as u8)?;
    write_to_slice::<u8>(&mut param_data, 9, should_wait_for_completion as u8)?;
    write_to_slice::<u8>(&mut param_data, 10, ignore_wait_for_transition as u8)?;
    execute_emevd_command(2003, 18, &param_data)
}

/// Set the game's time of day.
/// `hour` should be between 0.0 and 24.0 (exclusive), where 0.0 = midnight,
/// 6.0 = 6am, 12.0 = noon, 18.0 = 6pm.
pub fn set_time_of_day(hour: f32) -> ProcResult {
    player_loaded_check()?;
    
    // Reject NaN inputs — f32::min/max return NaN for NaN inputs,
    // which would corrupt the emevd parameter structure.
    if hour.is_nan() {
        return Err(gubtool_core::sys::error::ProcessError::partial_access(
            gubtool_core::sys::error::AccessType::Write(
                gubtool_core::sys::error::WriteType::Type("set_time_of_day"),
            ),
            0,
            0,
        ));
    }

    // Clamp hour to valid range [0.0, 24.0)
    let hour = hour.min(23.999).max(0.0);
    
    // Build parameter data for emevd command 2001/4 (set game cycle time)
    // Structure based on set_night():
    // byte[0]: 20 (group 2001 high byte)
    // byte[5]: 1 (command sub-type)
    // f32 at offset 8: hour value (0.0-24.0)
    // f32 at offset 12: multiplier (1.0)
    // f32 at offset 16: unknown (0.0)
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u8>(&mut param_data, 0, 20)?;
    write_to_slice::<u8>(&mut param_data, 5, 1)?;
    write_to_slice::<f32>(&mut param_data, 8, hour)?;
    write_to_slice::<f32>(&mut param_data, 12, 1.0)?;
    write_to_slice::<f32>(&mut param_data, 16, 0.0)?;
    
    execute_emevd_command(2001, 4, &param_data)
}

/// Get the current game cycle time (hour).
/// Returns 0.0-24.0.
pub fn get_time_of_day() -> f32 {
    // The game cycle time is stored in game_data_man
    // Read pointer first, then add offset, then read f32
    read::<u64>(BasePointer::GameDataMan)
        .and_then(|ptr| read::<f32>(ptr + game_data_man::IGT))
        .map(|v| v * 24.0) // Convert from normalized [0,1] to [0,24]
        .unwrap_or(12.0)
}

/// Save the current time of day to a cave offset.
/// Used for the "no time change on death" feature.
pub fn save_time_of_day() -> ProcResult {
    let time = get_time_of_day();
    write::<f32>(CaveOffset::SavedTimeOfDay, time)
}

/// Restore the saved time of day.
/// Used when the player dies to prevent time advancement.
pub fn restore_time_of_day() -> ProcResult {
    let saved_time = read::<f32>(CaveOffset::SavedTimeOfDay)?;
    set_time_of_day(saved_time)
}

/// Check if the player has died (HP is 0 or very low).
/// Returns true if the player appears to be dead.
pub fn is_player_dead() -> ProcResult<bool> {
    let hp = player_ins().get_current_hp()?;
    Ok(hp <= 0)
}

/// Detect if the player has just died and set the death flag.
/// This should be called on every poll when `no_time_change_death` is enabled.
/// It sets DeathFlag when HP transitions from > 0 to <= 0.
/// Also clears the flag if the player is alive (handles rapid-death edge case
/// where the player dies and revives very quickly through game mechanics).
pub fn detect_death_and_set_flag() -> ProcResult<()> {
    let was_dead = read::<u8>(CaveOffset::DeathFlag)?;
    
    // Check if player is currently dead
    let is_dead = is_player_dead()?;
    if is_dead {
        // Player is dead - set the flag if not already set
        if was_dead == 0 {
            write::<u8>(CaveOffset::DeathFlag, 1)?;
        }
    } else {
        // Player is alive - clear the death flag to handle rapid-death edge case
        // where the player dies and revives very quickly through game mechanics.
        // Without this, the flag could remain set and cause time restoration
        // to happen at the wrong time.
        if was_dead != 0 {
            write::<u8>(CaveOffset::DeathFlag, 0)?;
        }
    }
    Ok(())
}

/// Check if death was detected and time needs to be restored.
/// After restoring, clears the death flag.
pub fn check_and_restore_time_on_death() -> ProcResult<bool> {
    // Check the death flag
    let was_dead = read::<u8>(CaveOffset::DeathFlag)?;
    if was_dead == 0 {
        return Ok(false);
    }
    
    // Death flag is set - check if player is now alive again
    let is_dead = is_player_dead()?;
    if !is_dead {
        // Player is alive now, restore the saved time
        restore_time_of_day()?;
        // Clear the death flag
        write::<u8>(CaveOffset::DeathFlag, 0)?;
        return Ok(true);
    }
    
    // Still dead, keep the flag set
    Ok(false)
}

/// Called when the game loads to save the initial time of day.
pub fn init_time_of_day() -> ProcResult {
    save_time_of_day()?;
    write::<u8>(CaveOffset::DeathFlag, 0)?;
    Ok(())
}