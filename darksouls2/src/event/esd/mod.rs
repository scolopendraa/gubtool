mod event_commands;
mod scripts;
pub use scripts::*;
use {
    crate::{
        mem::{read, read_address, run_custom_function, write},
        offsets::{Offset, code_cave::CaveAddr, module_offsets::Function},
        resources::{asm_function, map_ids::MapId},
        utils::{area_check, player_loaded_check},
    },
    assemble::patch::DWORD,
    gubtool_core::{address::POINTER, sys::sys_error::SysResult},
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

    write::<[i32; 3]>(CaveAddr::EzStateParams, event.params)?;

    let mut fun = asm_function("ezstate_execute_event");

    fun.patch::<DWORD>("event_id", event.event_id);
    fun.patch::<POINTER>("fn_event_ctor", Function::EzStateExternalEventCtor);
    fun.patch::<DWORD>("map_id", map_id as u32);
    fun.patch::<DWORD>("params_len", event.params_len);
    fun.patch::<POINTER>("params_loc", CaveAddr::EzStateParams);
    fun.patch::<POINTER>("fn_execute_event", Function::EzStateExecuteEvent);

    run_custom_function(fun)
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

    fun.patch::<DWORD>("obj_id", obj_id);
    fun.patch::<DWORD>("map_id", map_id as u32);
    fun.patch::<POINTER>("fn_map_entity", Function::MapEntityFromMapIdAndObjId);
    fun.patch::<POINTER>("saved_state_act_ctrl", CaveAddr::LookedUpStateActCtrl);

    run_custom_function(fun)?;

    let state_act_ctrl = read_address(CaveAddr::LookedUpStateActCtrl)?;
    Ok(state_act_ctrl)
}

const STATE_ID: Offset = Offset {
    vanilla: 0x12,
    scholar: 0x1e,
};

pub fn get_obj_state(state_act_ctrl: u64) -> SysResult<u8> {
    read::<u8>(state_act_ctrl + STATE_ID.resolve())
}
