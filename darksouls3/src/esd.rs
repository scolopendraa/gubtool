use {
    crate::{
        mem::run_custom_function_in_thread,
        offsets::{code_cave::CaveAddr, module_offsets::Function},
        resources::ASM,
    },
    assemble::patch::DWORD,
    gubtool_core::{address::POINTER, sys::write_bytes},
};

pub struct TalkCommand {
    pub name:   &'static str,
    pub id:     u32,
    pub params: &'static [i32],
}

impl TalkCommand {
    pub fn execute(&self) -> anyhow::Result<()> {
        execute_talk_command(self.id, self.params)
    }
}

fn execute_talk_command(command_id: u32, params: &'static [i32]) -> anyhow::Result<()> {
    let arg_bytes: Vec<u8> = params.iter().flat_map(|arg| arg.to_le_bytes()).collect();

    write_bytes(CaveAddr::TalkCommandArgs, &arg_bytes)?;

    let mut fun = ASM.get_function("execute_talk_command");

    fun.patch::<DWORD>("command_id", command_id);
    fun.patch::<POINTER>("fn_event_ctor", Function::EzStateExternalEventCtor);
    fun.patch::<DWORD>("params_len", params.len() as u32);
    fun.patch::<POINTER>("params_loc", CaveAddr::TalkCommandArgs);
    fun.patch::<POINTER>("fn_execute_event", Function::ExecuteTalkCommand);

    run_custom_function_in_thread(fun)
}
