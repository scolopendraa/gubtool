mod event_commands;
mod scripts;
pub use scripts::*;
use {
    crate::{
        mem::{read, read_address, run_custom_function, write},
        offsets::{Offset, code_cave::CaveAddress, module_offsets::Function},
        resources::{asm_function, map_ids::MapId},
        utils::{area_check, player_loaded_check},
    },
    gubtool_core::{
        slice_ops::{write_addr_to_slice, write_to_slice},
        sys::sys_error::ProcResult,
    },
};

struct EsdEventScript {
    map_id:    MapId,
    functions: &'static [EventCommand],
}

struct EventCommand {
    event_id:   i32,
    params:     [i32; 3],
    params_len: u32,
}

fn execute_esd_event(event: &EventCommand, map_id: MapId) -> anyhow::Result<()> {
    area_check(map_id)?;

    write::<[i32; 3]>(CaveAddress::EzStateParams, event.params)?;

    let mut fun = asm_function("ezstate_execute_event");
    let mut asm = fun.take_bytes();

    write_to_slice::<u32>(&mut asm, fun.reloc("event_id"), event.event_id)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_event_ctor"), Function::EzStateExternalEventCtor)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("map_id"), map_id as u32)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("params_len"), event.params_len)?;
    write_addr_to_slice(&mut asm, fun.reloc("params_loc"), CaveAddress::EzStateParams)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_execute_event"), Function::EzStateExecuteEvent)?;

    run_custom_function(asm)
}

impl EsdEventScript {
    fn execute(&self) -> anyhow::Result<()> {
        self.functions
            .iter()
            .try_for_each(|fun| execute_esd_event(fun, self.map_id))
    }
}

impl EventCommand {
    const fn new(event_id: i32, params: &[i32]) -> Self {
        match params {
            [] => {
                Self {
                    event_id,
                    params: [0; 3],
                    params_len: 0,
                }
            }
            [a] => {
                Self {
                    event_id,
                    params: [*a, 0, 0],
                    params_len: 1,
                }
            }
            [a, b] => {
                Self {
                    event_id,
                    params: [*a, *b, 0],
                    params_len: 2,
                }
            }
            [a, b, c] => {
                Self {
                    event_id,
                    params: [*a, *b, *c],
                    params_len: 3,
                }
            }
            _ => panic!("too many params"),
        }
    }
}

pub fn get_obj_state_act_ctrl(map_id: MapId, obj_id: u32) -> anyhow::Result<u64> {
    player_loaded_check()?;
    area_check(map_id)?;

    let mut fun = asm_function("state_act_ctrl");
    let mut asm = fun.take_bytes();

    write_to_slice::<u32>(&mut asm, fun.reloc("obj_id"), obj_id)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("map_id"), map_id as u32)?;
    write_addr_to_slice(
        &mut asm,
        fun.reloc("fn_map_entity"),
        Function::MapEntityFromMapIdAndObjId,
    )?;
    write_addr_to_slice(
        &mut asm,
        fun.reloc("saved_state_act_ctrl"),
        CaveAddress::LookedUpStateActCtrl,
    )?;

    run_custom_function(asm)?;

    let state_act_ctrl = read_address(CaveAddress::LookedUpStateActCtrl)?;
    Ok(state_act_ctrl)
}

const STATE_ID: Offset = Offset {
    vanilla: 0x12,
    scholar: 0x1e,
};

pub fn get_obj_state(state_act_ctrl: u64) -> ProcResult<u8> {
    read::<u8>(state_act_ctrl + STATE_ID.resolve())
}
