use anyhow::{Result, bail, ensure};
use crate::{
    core::{
        chr_ins::ChrInsExt,
        event,
        mem::*,
        player::player_ins,
        utils::{character_loaded_check, dlc_check},
    },
    offsets::{
        self, code_cave, functions, virtual_memory_flag::{self, node_offsets}
    },
    resources::{
        asm,
        bosses::{Boss, bosses_array},
        talk_commands::TalkCommand,
    },
};

pub fn get_event(event_id: u32) -> Result<bool> {
    let (data_ptr, block_offset) = traverse_event_tree(event_id)?;
    let mask = 1 << (7 - (block_offset & 7));
    is_bit_set(data_ptr + (block_offset >> 3) as u64, mask)
}

pub fn set_event(event_id: u32, state: bool) -> Result<()> {
    let (data_ptr, block_offset) = traverse_event_tree(event_id)?;
    let mask = 1 << (7 - (block_offset & 7));
    set_bit(data_ptr + (block_offset >> 3) as u64, mask, state)
}

fn traverse_event_tree(event_id: u32) -> Result<(u64, u32)> {
    let virt_flag_info = read::<u64>(virtual_memory_flag::base())
        .and_then(|addr| read::<[u8; 0x50]>(addr))?;
    let block_size = read_from_slice::<u32>(&virt_flag_info, virtual_memory_flag::BLOCK_SIZE)?;
    let block_index = event_id / block_size;
    let block_offset = event_id % block_size;
    let root = read_from_slice::<u64>(&virt_flag_info, virtual_memory_flag::EVENT_TREE_ROOT)?;

    let mut last_valid_node = root;
    let mut last_valid_node_data = read::<[u8; 0x38]>(root)?;
    let mut current = read_from_slice::<u64>(&last_valid_node_data, 0x8)?;
    let mut current_data: [u8; 0x38];

    loop {
        current_data = read::<[u8; 0x38]>(current)?;
        if current_data[node_offsets::IS_LEAF as usize] != 0 {
            break;
        }
        let current_block_index = read_from_slice::<u32>(&current_data, node_offsets::BLOCK_INDEX)?;

        if current_block_index < block_index {
            current = read_from_slice::<u64>(&current_data, node_offsets::RIGHT_CHILD)?
        } else {
            last_valid_node = current;
            last_valid_node_data = current_data;
            current = read_from_slice::<u64>(&current_data, node_offsets::LEFT_CHILD)?
        };
    }
    if last_valid_node == root ||
        block_index < read_from_slice::<u32>(&last_valid_node_data, node_offsets::BLOCK_INDEX)? {
        bail!("Node is root")
    }

    let node_type = read_from_slice::<i32>(&last_valid_node_data, node_offsets::TYPE)?;
    let index = read_from_slice::<i32>(&last_valid_node_data, node_offsets::DATA_INDEX)? as u64;
    let data_ptr = if node_type == 1 {
        let stride = read_from_slice::<i32>(&virt_flag_info, virtual_memory_flag::STRIDE)? as u64;
        let base = read_from_slice::<u64>(&virt_flag_info, virtual_memory_flag::EVENT_TREE_BASE)?;
        index * stride + base
    } else if node_type == 2 {
        index
    } else {
        bail!("Node type invalid")
    };
    ensure!(data_ptr != 0, "Block pointer is null after bitmap lookup");
    Ok((data_ptr, block_offset))
}

pub fn execute_emevd_command(group_id: i32, command_id: i32, args: &[u8]) -> Result<()> {
    let location = code_cave::base() + code_cave::EMEVD_ASM;
    let args_location = code_cave::base() + code_cave::EMEVD_ARGS;

    let mut asm = asm::EXECUTE_EMEVD_COMMAND;
    write_to_slice::<u64>(&mut asm, 105, functions::emk_event_ins_constructor())?;
    write_to_slice::<i32>(&mut asm, 124, group_id)?;
    write_to_slice::<i32>(&mut asm, 131, command_id)?;
    write_to_slice::<u64>(&mut asm, 151, args_location)?;
    write_to_slice::<u64>(&mut asm, 171, offsets::cs_emk_system::base())?;
    write_to_slice::<u64>(&mut asm, 194, functions::emevd_switch())?;
    let asm = append_flag_setter(location, &asm)?;

    let _handle = EXECUTE_EMEVD_COMMAND_MUTEX.lock().unwrap();

    write_bytes(args_location, args)?;
    write_bytes(location, &asm)?;
    run_win_thread_wait(location)
}

pub fn execute_talk_command(command_id: i32, params: &'static [i32], handle: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::EZ_STATE_TALK_ASM;
    let params_location = code_cave::base() + code_cave::EZ_STATE_TALK_PARAMS;
    let params: Vec<u8> = params.iter().flat_map(|&x| x.to_le_bytes()).collect();

    let mut asm = asm::EXECUTE_TALK_COMMAND;
    write_to_slice::<i32>(&mut asm, 18, command_id)?;
    write_to_slice::<i32>(&mut asm, 23, rel_i32(functions::external_event_temporary_constructor(), location + 27)?)?;
    write_to_slice::<u64>(&mut asm, 65, handle)?;
    write_to_slice::<i32>(&mut asm, 78, params.len())?;
    write_to_slice::<i32>(&mut asm, 93, rel_i32(params_location, location + 97)?)?;
    write_to_slice::<i32>(&mut asm, 155, rel_i32(functions::execute_talk_command(), location + 159)?)?;
    let asm = append_flag_setter(location, &asm)?;

    write_bytes(params_location, &params)?;
    write_bytes(location, &asm)?;
    run_win_thread_wait(location)
}

impl TalkCommand {
    pub fn execute(&self) -> Result<()> {
        let handle = match self.handle {
            Some(function) => function()?,
            None => 0,
        };
        if self.command_id == 24 {
            execute_talk_command(49, &[6001, 232], 0)?;
            execute_talk_command(49, &[6001, 233], 0)?;
            execute_talk_command(49, &[6001, 234], 0)?;
            execute_talk_command(49, &[6001, 235], 0)?;
        }
        execute_talk_command(self.command_id, self.params, handle)
    }
}

pub fn set_night() -> Result<()> {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u8>(&mut param_data, 0, 20)?;
    write_to_slice::<u8>(&mut param_data, 5, 1)?;
    write_to_slice::<f32>(&mut param_data, 8, 0.75)?;
    write_to_slice::<f32>(&mut param_data, 12, 2.0)?;
    write_to_slice::<f32>(&mut param_data, 16, 0.0)?;
    execute_emevd_command(2001, 4, &param_data)
}

pub fn rest() -> Result<()> {
    execute_emevd_command(2004, 47, &[])
}

pub fn fight_fortissax() -> Result<()> {
    ensure!(player_ins().block_id()? == 201523200, "Must be in general area");
    set_event(12032859, true)
}

pub fn fight_elden_beast() -> Result<()> {
    ensure!(player_ins().block_id()? == 318767104, "Must be in general area");
    set_event(19002802, true)?;
    set_event(19002805, true)
}

pub fn disable_title_card() -> Result<()> {
    execute_emevd_command(2012, 8, &[])
}

pub fn reset_character_position(entity_id: u32) -> Result<()> {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u32>(&mut param_data, 0, entity_id)?;
    execute_emevd_command(2004, 81, &param_data)
}

pub fn set_dlc_clear(state: bool) -> Result<()> {
    character_loaded_check()?;
    dlc_check()?;
    set_event(70, state)
}

pub fn get_dlc_clear() -> Result<bool> {
    dlc_check()?;
    get_event(70)
}

pub fn unlock_metyr() -> Result<()> {
    character_loaded_check()?;
    dlc_check()?;
    let events = [
        2050400600, 2053460600, 2051459226, 2051459228, 2051459229, 2051459230, 2051455023,
        2051459249, 2051452717, 2050407000, 400662, 4856, 4855, 4854, 4849, 2051452718, 2051459213,
        2051450715, 9440, 2051450180,
    ];
    events.iter().try_for_each(|&i| set_event(i, true))
}

pub const DEAD: &str = "Dead";
pub const ALIVE: &str = "Alive";
pub const ALIVE_SE: &str = "Alive (Second Encounter)";

impl Boss {
    pub fn revive(&self, first_encounter: bool, warp: bool) -> Result<()> {
        character_loaded_check()?;
        if self.dlc {
            dlc_check()?;
        }
        if first_encounter {
            self.fe_flags.iter().try_for_each(|(id, state)| set_event(*id, *state))?;
        }
        self.flags.iter().try_for_each(|(id, state)| set_event(*id, *state))?;
        if warp {
            self.warp()?
        }
        Ok(())
    }
    pub fn revive_status(&self) -> &str {
        if event::get_event(self.flags[0].0).unwrap_or_default() {
            return DEAD;
        }
        if self.fe_flags.iter().all(|x| event::get_event(x.0).unwrap_or_default() == x.1) {
            ALIVE
        } else {
            ALIVE_SE
        }
    }
}

pub fn mass_revive(dlc: bool, first_encounter: bool) -> Result<()> {
    bosses_array(dlc).iter().try_for_each(|boss| boss.revive(first_encounter, false))
}

pub fn _set_event(event_id: u32, value: bool) -> Result<()> {
    let location = code_cave::base() + code_cave::SET_EVENT_ASM;
    let virt_flag_ptr = read::<u64>(virtual_memory_flag::base())?;

    let mut asm = asm::SET_EVENT;
    asm[2..10].copy_from_slice(&(virt_flag_ptr).to_le_bytes());
    asm[12..20].copy_from_slice(&(event_id as u64).to_le_bytes());
    asm[22..30].copy_from_slice(&(value as i64).to_le_bytes());
    asm[32..40].copy_from_slice(&(functions::set_event()).to_le_bytes());
    let asm = append_flag_setter(location, &asm)?;

    write_bytes(location, &asm)?;
    run_win_thread_wait(location)
}

pub fn _get_event(event_id: u32) -> Result<bool> {
    let location = code_cave::base() + code_cave::GET_EVENT_ASM;
    let virt_flag_ptr = read::<u64>(virtual_memory_flag::base())?;
    let result_location = code_cave::base() + code_cave::EVENT_RESULT;

    let mut asm = asm::GET_EVENT;
    asm[2..10].copy_from_slice(&(virt_flag_ptr).to_le_bytes());
    asm[12..20].copy_from_slice(&(event_id as u64).to_le_bytes());
    asm[22..30].copy_from_slice(&(functions::get_event()).to_le_bytes());
    asm[42..50].copy_from_slice(&(result_location).to_le_bytes());
    let asm = append_flag_setter(location, &asm)?;

    write::<u8>(result_location, 0xFF)?;
    write_bytes(location, &asm)?;
    run_win_thread_wait(location)?;
    Ok(read::<u8>(result_location)? != 0)
}
