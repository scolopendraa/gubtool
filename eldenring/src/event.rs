use {
    crate::{
        event,
        mem::*,
        offsets::{ChainReadExt, code_cave::CaveAddr, module_offsets::Function},
        player,
        pointer_cache::ResolvedPtr,
        resources::{
            ASM,
            bosses::{BOSSES, Boss},
            graces::{self, Grace},
            talk_commands::TalkCommand,
        },
        utils::{dlc_check, player_loaded_check},
    },
    anyhow::ensure,
    assemble::patch::DWORD,
    gubtool_core::{
        address::{Address, POINTER},
        slice_ops::*,
        sys::{ipc::FfiValue, sys_error::SysResult},
    },
    shared::{
        command::{ToggleCommand, UnitCommand},
        event_log::{EventLog, EventLogger},
        toggle_command,
        unit_command,
    },
    std::fmt::Display,
};

pub fn get_event(event_id: u32) -> SysResult<bool> {
    if let Some((data_ptr, block_offset)) = event_flag_lookup(event_id)? {
        let mask = 1 << (7 - (block_offset & 7));
        is_bit_set(data_ptr + (block_offset >> 3) as u64, mask)
    } else {
        Ok(false)
    }
}

pub fn set_event(event_id: u32, state: bool) -> anyhow::Result<()> {
    player_loaded_check()?;

    let args = [
        FfiValue::pointer(ResolvedPtr::VirtualMemFlag.get()?),
        FfiValue::uint32(event_id),
        FfiValue::uint8(state as u8),
    ];

    run_game_function(Function::SetEvent, &args)
}

struct VirtMemInfo {
    block_size:       u32,
    stride:           u32,
    mem_base:         u64,
    lookup_tree_root: u64,
}

impl VirtMemInfo {
    pub fn read() -> SysResult<Self> {
        let bytes = ResolvedPtr::VirtualMemFlag.get().read::<[u8; 0x40]>()?;
        Ok(Self {
            block_size:       read_from_slice::<u32>(&bytes, 0x1c)?,
            stride:           read_from_slice::<u32>(&bytes, 0x20)?,
            mem_base:         read_from_slice::<u64>(&bytes, 0x28)?,
            lookup_tree_root: read_from_slice::<u64>(&bytes, 0x38)?,
        })
    }
}

#[derive(Clone, Copy)]
struct Node {
    left_child:  u64,
    right_child: u64,
    is_leaf:     bool,
    block_idx:   u32,
    block_type:  u32,
    data_idx:    u32,
}

impl Node {
    fn read_at(address: u64) -> SysResult<Self> {
        let bytes = read::<[u8; 0x34]>(address)?;
        Ok(Self {
            left_child:  read_from_slice::<u64>(&bytes, 0x0)?,
            right_child: read_from_slice::<u64>(&bytes, 0x10)?,
            is_leaf:     read_from_slice::<u8>(&bytes, 0x19)? != 0x0,
            block_idx:   read_from_slice::<u32>(&bytes, 0x20)?,
            block_type:  read_from_slice::<u32>(&bytes, 0x28)?,
            data_idx:    read_from_slice::<u32>(&bytes, 0x30)?,
        })
    }
}

fn event_flag_lookup(event_id: u32) -> SysResult<Option<(u64, u32)>> {
    let virt_mem_info = VirtMemInfo::read()?;

    if virt_mem_info.block_size == 0 {
        return Ok(None)
    }
    let block_idx = event_id / virt_mem_info.block_size;
    let block_offset = event_id % virt_mem_info.block_size;

    let mut last_valid_node: Option<Node> = None;
    let mut current_node_ptr = read::<u64>(virt_mem_info.lookup_tree_root + 0x8)?;

    loop {
        let current_node = Node::read_at(current_node_ptr)?;

        if current_node.is_leaf {
            break;
        }

        if current_node.block_idx < block_idx {
            current_node_ptr = current_node.right_child;
        } else {
            last_valid_node = Some(current_node);
            current_node_ptr = current_node.left_child;
        };
    }
    if let Some(node) = last_valid_node
        && node.block_idx <= block_idx
    {
        let data_ptr = match node.block_type {
            1 => node.data_idx as u64 * virt_mem_info.stride as u64 + virt_mem_info.mem_base,
            2 => node.data_idx as u64,
            _ => return Ok(None),
        };

        if data_ptr == 0x0 {
            return Ok(None);
        }
        return Ok(Some((data_ptr, block_offset)));
    }
    Ok(None)
}

pub fn execute_talk_command(
    command_id: i32,
    params: &'static [i32],
    chr_handle: u64,
) -> anyhow::Result<()> {
    let params: Vec<u8> = params.iter().flat_map(|&x| x.to_le_bytes()).collect();

    let mut fun = ASM.get_function("execute_talk_command");

    fun.patch::<DWORD>("command_id", command_id);
    fun.patch::<POINTER>("fn_external_event_temp_ctor", Function::ExternalEventTempCtor);
    fun.patch::<POINTER>("chr_handle", chr_handle);
    fun.patch::<DWORD>("params_len", params.len() as u32);
    fun.patch::<POINTER>("params_loc", CaveAddr::EzStateParams);
    fun.patch::<POINTER>("fn_execute_talk_command", Function::ExecuteTalkCommand);

    write_bytes(CaveAddr::EzStateParams, &params)?;
    run_custom_function(fun)
}

impl TalkCommand {
    pub fn execute(&self) -> anyhow::Result<()> {
        player_loaded_check()?;
        if self.dlc {
            dlc_check()?;
        }
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

#[derive(Default)]
pub struct ErEventLogger {
    event_log: EventLog,
}

impl EventLogger for ErEventLogger {
    fn event_log(&self) -> &EventLog {
        &self.event_log
    }
    fn event_log_mut(&mut self) -> &mut EventLog {
        &mut self.event_log
    }
    fn file_prefix(&self) -> &'static str {
        "eldenring"
    }
    fn write_idx(&self) -> SysResult<i32> {
        read::<i32>(CaveAddr::EventLogWriteIdx.addr())
    }
    fn read_buffer(&self) -> SysResult<[u8; 0x1000]> {
        read::<[u8; 0x1000]>(CaveAddr::EventLogBuffer.addr())
    }
    fn clear_cave(&self) -> SysResult {
        write::<i32>(CaveAddr::EventLogWriteIdx.addr(), 0x0)?;
        write_bytes(CaveAddr::EventLogBuffer.addr(), &[0x0; 0x1000])
    }
    fn toggle_hook(&self) -> anyhow::Result<()> {
        StartEventLogger.toggle()
    }
}

const EVENT_LOG_HOOK_ORIGINAL: [u8; 5] = [0x48, 0x89, 0x5c, 0x24, 0x08];
toggle_command!(StartEventLogger {
    is: {
        read::<[u8; 5]>(Function::SetEvent)
            .map(|bytes| bytes != EVENT_LOG_HOOK_ORIGINAL)
    }

    set(state): {
        match state {
            true => {
                let mut fun = ASM.get_function("event_log");

                fun.patch::<POINTER>("write_index", CaveAddr::EventLogWriteIdx);
                fun.patch::<POINTER>("buffer", CaveAddr::EventLogBuffer);
                fun.patch_rel32("hook_loc", CaveAddr::EventLogHook, Function::SetEvent.add(5), 4);

                install_hook(&fun.bytes, CaveAddr::EventLogHook, Function::SetEvent, 5)?;
            }
            false => write_bytes(Function::SetEvent, &EVENT_LOG_HOOK_ORIGINAL)?,
        }
        Ok(())
    }
});

unit_command!(FightFortissax {
    general_area_check(201523200)?;
    set_event(12032859, true)
});

unit_command!(FightEldenBeast {
    general_area_check(318767104)?;
    set_event(19002802, true)?;
    set_event(19002805, true)
});

toggle_command!(DlcClear {
    is: {
        get_event(70)
    }

    set(state): {
        dlc_check()?;
        set_event(70, state)
    }
});

unit_command!(UnlockMetyr {
        dlc_check()?;
        let events = [
            2050400600,
            2053460600,
            2051459226,
            2051459228,
            2051459229,
            2051459230,
            2051455023,
            2051459249,
            2051452717,
            2050407000,
            400662,
            4856,
            4855,
            4854,
            4849,
            2051452718,
            2051459213,
            2051450715,
            9440,
            2051450180,
        ];
        events.iter().try_for_each(|&i| set_event(i, true))
});

unit_command!(MassReviveBosses {
    BOSSES
        .iter()
        .try_for_each(|boss| boss.revive(true))
});

pub enum AliveStatus {
    Dead,
    Alive,
    AliveSecondEncounter,
}

impl Boss {
    pub fn revive(&self, first_encounter: bool) -> anyhow::Result<()> {
        player_loaded_check()?;
        if self.dlc {
            dlc_check()?;
        }
        if first_encounter {
            self.fe_flags
                .iter()
                .try_for_each(|(id, state)| set_event(*id, *state))?;
        }
        self.flags
            .iter()
            .try_for_each(|(id, state)| set_event(*id, *state))?;
        Ok(())
    }
    pub fn revive_status(&self) -> AliveStatus {
        if event::get_event(self.flags[0].0).unwrap_or_default() {
            return AliveStatus::Dead;
        }
        if self
            .fe_flags
            .iter()
            .all(|x| event::get_event(x.0).unwrap_or_default() == x.1)
        {
            AliveStatus::Alive
        } else {
            AliveStatus::AliveSecondEncounter
        }
    }
}

impl Display for AliveStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Alive => "Alive",
            Self::AliveSecondEncounter => "Alive (Second Encounter)",
            Self::Dead => "Dead",
        };
        write!(f, "{s}")
    }
}

impl Grace {
    pub fn is_unlocked(&self) -> SysResult<bool> {
        get_event(self.flag_id)
    }
    pub fn unlock(&self) -> anyhow::Result<()> {
        set_event(self.flag_id, true)
    }
}

unit_command!(UnlockAllGraces {
    graces::GRACES.iter().try_for_each(|grace| grace.unlock())
});

fn general_area_check(area_id: u32) -> anyhow::Result<()> {
    player_loaded_check()?;
    ensure!(
        matches!(player::player().chr_ins()?.block_id(), Ok(id) if id == area_id),
        "Must be in general area"
    );
    Ok(())
}

fn _set_event_direct(event_id: u32, state: bool) -> SysResult {
    if let Some((data_ptr, block_offset)) = event_flag_lookup(event_id)? {
        let mask = 1 << (7 - (block_offset & 7));
        set_bit(data_ptr + (block_offset >> 3) as u64, mask, state)
    } else {
        Ok(())
    }
}
