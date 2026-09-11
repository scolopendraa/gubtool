use {
    crate::{
        mem::run_game_function,
        offsets::{code_cave::CaveAddr, module_offsets::Function},
        pointer_cache::ResolvedPtr,
        resources::ASM,
    },
    gubtool_core::{
        address::{Address, POINTER},
        sys::{install_hook, ipc::FfiValue, read, sys_error::SysResult, write, write_bytes},
    },
    shared::{
        command::ToggleCommand,
        event_log::EventLog,
        impl_event_logger,
        toggle_command,
        unit_command,
    },
};

impl_event_logger!(Ds3EventLogger, "darksouls3");

const EVENT_LOG_HOOK_ORIGINAL: [u8; 5] = [0x40, 0x55, 0x57, 0x41, 0x54];
toggle_command!(StartEventLogger {
    is: {
        read::<[u8; 5]>(Function::SetEventFlag)
            .map(|bytes| bytes != EVENT_LOG_HOOK_ORIGINAL)
    }

    set(state): {
        match state {
            true => {
                let mut fun = ASM.get_function("event_log");

                fun.patch::<POINTER>("write_index", CaveAddr::EventLogWriteIdx);
                fun.patch::<POINTER>("buffer", CaveAddr::EventLogBuffer);
                fun.patch_rel32("hook_loc", CaveAddr::EventLogHook, Function::SetEventFlag.add(5), 4);

                install_hook(&fun.bytes, CaveAddr::EventLogHook, Function::SetEventFlag, 5)?;
            }
            false => write_bytes(Function::SetEventFlag, &EVENT_LOG_HOOK_ORIGINAL)?,
        }
        Ok(())
    }
});

fn set_event_flag(flag_id: u32, state: bool) -> anyhow::Result<()> {
    let args = [
        FfiValue::pointer(ResolvedPtr::EventFlagMan.get()?),
        FfiValue::uint32(flag_id),
        FfiValue::uint8(state as u8),
        FfiValue::uint8(0),
    ];

    run_game_function(Function::SetEventFlag, &args)
}

unit_command!(UnlockMidir {
    set_event_flag(0xE66928, true)
});
