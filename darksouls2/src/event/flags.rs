use {
    crate::{
        mem::*,
        offsets::{
            code_cave::CaveAddr,
            module_offsets::{Function, Hook},
        },
        pointer_cache::ResolvedPtr,
        resources::{asm_function, bosses::Boss, map_ids::MapId},
        utils::{area_check, player_loaded_check},
    },
    anyhow::anyhow,
    gubtool_core::{
        address::{Address, POINTER},
        attached::is_32,
        slice_ops::*,
        sys::{
            ipc::{FfiValue, X86CallingConvention},
            sys_error::SysResult,
        },
    },
    shared::{
        command::ToggleCommand,
        declare_command,
        event_log::{EventLog, EventLogger},
    },
};

pub fn set_event_flag(flag_id: u32, state: bool) -> anyhow::Result<()> {
    let event_flag_manager = ResolvedPtr::EventFlagManager.get()?;
    let args = [
        FfiValue::pointer(event_flag_manager),
        FfiValue::uint32(flag_id),
        FfiValue::uint8(state as u8),
    ];

    run_game_function(Function::SetEvent, &args, X86CallingConvention::__thiscall)
}

pub fn get_event_flag(flag_id: u32) -> SysResult<bool> {
    if let Some((byte_addr, bit_mask)) = event_flag_lookup(flag_id)? {
        is_bit_set(byte_addr, bit_mask)
    } else {
        Ok(false)
    }
}

#[derive(Debug)]
struct Node {
    bitmap_ptr: u64,
    size:       u32,
    key:        u32,
    next_node:  u64,
}

impl Node {
    fn read_at(address: u64) -> SysResult<Self> {
        if is_32() {
            let bytes = read::<[u8; 0x10]>(address)?;
            Ok(Self {
                bitmap_ptr: read_from_slice::<u32>(&bytes, 0x0)? as u64,
                size:       read_from_slice::<u32>(&bytes, 0x4)?,
                key:        read_from_slice::<u32>(&bytes, 0x8)?,
                next_node:  read_from_slice::<u32>(&bytes, 0xc)? as u64,
            })
        } else {
            let bytes = read::<[u8; 0x18]>(address)?;
            Ok(Self {
                bitmap_ptr: read_from_slice::<u64>(&bytes, 0x0)?,
                size:       read_from_slice::<u32>(&bytes, 0x8)?,
                key:        read_from_slice::<u32>(&bytes, 0xc)?,
                next_node:  read_from_slice::<u64>(&bytes, 0x10)?,
            })
        }
    }
}

fn event_flag_lookup(flag_id: u32) -> SysResult<Option<(u64, u8)>> {
    let event_flag_man = ResolvedPtr::EventFlagManager.get()?;

    let group = flag_id / 10000;
    let hash = group.wrapping_mul(0x89);
    let bit_index = flag_id % 10000;
    let bit_mask = 1u8 << (7 - (bit_index & 7));

    let first_node_offset = if is_32() {
        0x10 + (hash % 0x1f) as u64 * 4
    } else {
        0x20 + (hash % 0x1f) as u64 * 8
    };

    let mut node_ptr = read_address(event_flag_man + first_node_offset)?;

    while node_ptr != 0x0 {
        let node = Node::read_at(node_ptr)?;
        if node.key == group {
            let byte_index = bit_index >> 3;
            if byte_index < node.size {
                return Ok(Some((node.bitmap_ptr + byte_index as u64, bit_mask)));
            }
        }
        node_ptr = node.next_node;
    }
    Ok(None)
}

impl Boss {
    pub fn is_alive(&self) -> bool {
        !get_event_flag(self.death_flag).unwrap_or_default()
    }
}

#[derive(Default)]
pub struct Ds2EventLogger {
    event_log: EventLog,
}

impl EventLogger for Ds2EventLogger {
    fn event_log(&self) -> &EventLog {
        &self.event_log
    }
    fn event_log_mut(&mut self) -> &mut EventLog {
        &mut self.event_log
    }
    fn file_prefix(&self) -> &'static str {
        "darksouls2"
    }
    fn write_idx(&self) -> SysResult<i32> {
        read::<i32>(CaveAddr::EventLogWriteIdx)
    }
    fn read_buffer(&self) -> SysResult<[u8; 0x1000]> {
        read::<[u8; 0x1000]>(CaveAddr::EventLogBuffer)
    }
    fn clear_cave(&self) -> SysResult {
        write::<i32>(CaveAddr::EventLogWriteIdx, 0x0)?;
        write_bytes(CaveAddr::EventLogBuffer, &[0x0; 0x1000])
    }
    fn toggle_hook(&self) -> anyhow::Result<()> {
        StartEventLogger.toggle()
    }
}

declare_command!(
    StartEventLogger,
    KingsRingAquired => "King's Ring Aquired",
    NashandraUnlocked,
    AldiaUnlocked,
    DarkChasmLitShadedWoods => "Dark Chasm Lit (Shaded Woods)",
    DarkChasmLitDrangleicCastle => "Dark Chasm Lit (Drangleic Castle)",
    DarkChasmLitBlackGulch => "Dark Chasm Lit (Black Gulch)",
    BrumeTowerActivated,
    AavaVisible,
    UndoAlsanasSeal => "Alsana's Seal Undone",
    SkipIvoryKingGauntlet,
    DisableLoyceKnights,
    FreeLoyceKnightOuterWall => "Loyce Knight Freed (Outer Wall)",
    FreeLoyceKnightAbandonedDwelling => "Loyce Knight Freed (Abandoned Dwelling)",
    FreeLoyceKnightLowerGarrison => "Loyce Knight Freed (Lower Garrison)",
);

const EVENT_LOG_HOOK_ORIGINAL: [u8; 5] = [0xb8, 0x59, 0x17, 0xb7, 0xd1];
impl ToggleCommand for StartEventLogger {
    fn is(&self) -> SysResult<bool> {
        read::<[u8; 5]>(Hook::EventLog).map(|bytes| bytes != EVENT_LOG_HOOK_ORIGINAL)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        if state {
            let mut fun = asm_function("event_log");

            fun.patch::<POINTER>("write_index", CaveAddr::EventLogWriteIdx);
            fun.patch::<POINTER>("buffer", CaveAddr::EventLogBuffer);
            fun.patch_rel32("hook_loc", CaveAddr::EventLogHook, Hook::EventLog.add(5), 4);

            install_hook(&fun.bytes, CaveAddr::EventLogHook, Hook::EventLog, 5)?;
        } else {
            write_bytes(Hook::EventLog, &EVENT_LOG_HOOK_ORIGINAL)?;
        }
        Ok(())
    }
}

const VANILLA_IVORY_SKIP_ORIGINAL: [u8; 6] = [0x55, 0x8b, 0xec, 0x83, 0xec, 0x08];
const SCHOLAR_IVORY_SKIP_ORIGINAL: [u8; 5] = [0x48, 0x89, 0x74, 0x24, 0x10];
impl ToggleCommand for SkipIvoryKingGauntlet {
    fn is(&self) -> SysResult<bool> {
        if is_32() {
            read::<[u8; 6]>(Function::SetEvent).map(|val| val != VANILLA_IVORY_SKIP_ORIGINAL)
        } else {
            read::<[u8; 5]>(Function::SetEvent).map(|val| val != SCHOLAR_IVORY_SKIP_ORIGINAL)
        }
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        if state {
            let orig_instr_len = if is_32() { 6 } else { 5 };
            let mut fun = asm_function("ivory_skip");

            fun.patch::<POINTER>("fn_get_map_entity", Function::MapEntityFromMapIdAndObjId);
            fun.patch::<POINTER>("fn_get_map_object", Function::GetStateActComponent);
            fun.patch::<POINTER>("fn_set_event", Function::SetEvent);
            fun.patch_rel32(
                "hook_loc",
                CaveAddr::IvorySkipHook,
                Function::SetEvent.add(orig_instr_len),
                4,
            );

            install_hook(&fun.bytes, CaveAddr::IvorySkipHook, Function::SetEvent, orig_instr_len)?;
        } else {
            let bytes: &[u8] = if is_32() {
                &VANILLA_IVORY_SKIP_ORIGINAL
            } else {
                &SCHOLAR_IVORY_SKIP_ORIGINAL
            };
            write_bytes(Function::SetEvent, bytes)?;
        }
        Ok(())
    }
}

const VANILLA_LOYCE_SKIP_ORIGINAL: [u8; 7] = [0x88, 0x94, 0x08, 0xa1, 0x02, 0x00, 0x00];
const SCHOLAR_LOYCE_SKIP_ORIGINAL: [u8; 8] = [0x44, 0x88, 0x84, 0x08, 0xa1, 0x03, 0x00, 0x00];
impl ToggleCommand for DisableLoyceKnights {
    fn is(&self) -> SysResult<bool> {
        match is_32() {
            true => {
                read::<[u8; 7]>(Hook::SetSharedFlag)
                    .map(|val| val != [0x88, 0x94, 0x08, 0xa1, 0x02, 0x00, 0x00])
            }
            false => {
                read::<[u8; 8]>(Hook::SetSharedFlag)
                    .map(|val| val != [0x44, 0x88, 0x84, 0x08, 0xa1, 0x03, 0x00, 0x00])
            }
        }
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        if state {
            let orig_instr_len = if is_32() { 7 } else { 8 };
            let mut fun = asm_function("ivory_knights");
            fun.patch_rel32(
                "hook_loc",
                CaveAddr::IvoryKnightsHook,
                Hook::SetSharedFlag.add(orig_instr_len),
                4,
            );
            install_hook(
                &fun.bytes,
                CaveAddr::IvoryKnightsHook,
                Hook::SetSharedFlag,
                orig_instr_len,
            )?;
        } else {
            let bytes: &[u8] = if is_32() {
                &VANILLA_LOYCE_SKIP_ORIGINAL
            } else {
                &SCHOLAR_LOYCE_SKIP_ORIGINAL
            };
            write_bytes(Hook::SetSharedFlag, bytes)?;
        }
        Ok(())
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum EventFlag {
    GiantLordDefeated            = 100972,
    ThroneDuoDefeated            = 100974,
    NashandraDefeated            = 100973,
    VendrickDefeated             = 100978,
    UnlockAldia                  = 100747,
    KingsRingAcquired            = 100804,
    VisibleAava                  = 537000012,
    FridgidSnowstorm             = 537010014,
    ShadedWoodsChasmCleared      = 403000001,
    DrangleicCastleChasmCleared  = 403000002,
    BlackGulchChasmCleared       = 403000003,
    ActivateBrume                = 536000010,
    EleumLoyceWinds              = 537000001,
    EleumLoyceIce                = 537000011,
    LoyceKnightOuterWall         = 537000020,
    LoyceKnightAbandonedDwelling = 537000021,
    LoyceKnightLowerGarrison     = 537000022,
    EarthenPeakWindmillBurned    = 117000055,
}

impl EventFlag {
    pub fn get(&self) -> SysResult<bool> {
        get_event_flag(*self as u32)
    }

    pub fn set(&self, state: bool) -> anyhow::Result<()> {
        set_event_flag(*self as u32, state)
    }

    pub fn set_area_conditional(&self, state: bool, area_id: MapId) -> anyhow::Result<()> {
        area_check(area_id)?;
        set_event_flag(*self as u32, state)
    }
}

impl ToggleCommand for KingsRingAquired {
    fn is(&self) -> SysResult<bool> {
        EventFlag::KingsRingAcquired.get()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::KingsRingAcquired.set(state)
    }
}
impl ToggleCommand for NashandraUnlocked {
    fn is(&self) -> SysResult<bool> {
        EventFlag::GiantLordDefeated.get()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::GiantLordDefeated.set(state)
    }
}
impl ToggleCommand for AldiaUnlocked {
    fn is(&self) -> SysResult<bool> {
        Ok(EventFlag::VendrickDefeated.get()? && EventFlag::UnlockAldia.get()?)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::VendrickDefeated.set(state)?;
        EventFlag::UnlockAldia.set(state)
    }
}
impl ToggleCommand for DarkChasmLitShadedWoods {
    fn is(&self) -> SysResult<bool> {
        EventFlag::ShadedWoodsChasmCleared.get()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::ShadedWoodsChasmCleared.set_area_conditional(state, MapId::DarkChasmOfOld)
    }
}
impl ToggleCommand for DarkChasmLitDrangleicCastle {
    fn is(&self) -> SysResult<bool> {
        EventFlag::DrangleicCastleChasmCleared.get()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::DrangleicCastleChasmCleared.set_area_conditional(state, MapId::DarkChasmOfOld)
    }
}
impl ToggleCommand for DarkChasmLitBlackGulch {
    fn is(&self) -> SysResult<bool> {
        EventFlag::BlackGulchChasmCleared.get()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::BlackGulchChasmCleared.set_area_conditional(state, MapId::DarkChasmOfOld)
    }
}
impl ToggleCommand for BrumeTowerActivated {
    fn is(&self) -> SysResult<bool> {
        EventFlag::ActivateBrume.get()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::ActivateBrume.set_area_conditional(state, MapId::BrumeTower)
    }
}
impl ToggleCommand for AavaVisible {
    fn is(&self) -> SysResult<bool> {
        EventFlag::VisibleAava.get()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::VisibleAava.set_area_conditional(state, MapId::FrozenEleumLoyce)
    }
}
impl ToggleCommand for UndoAlsanasSeal {
    fn is(&self) -> SysResult<bool> {
        Ok(EventFlag::EleumLoyceWinds.get()? && EventFlag::EleumLoyceWinds.get()?)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::EleumLoyceIce.set_area_conditional(state, MapId::FrozenEleumLoyce)?;
        EventFlag::EleumLoyceWinds.set_area_conditional(state, MapId::FrozenEleumLoyce)
    }
}
impl ToggleCommand for FreeLoyceKnightOuterWall {
    fn is(&self) -> SysResult<bool> {
        EventFlag::LoyceKnightOuterWall.get()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::LoyceKnightOuterWall.set_area_conditional(state, MapId::FrozenEleumLoyce)
    }
}
impl ToggleCommand for FreeLoyceKnightAbandonedDwelling {
    fn is(&self) -> SysResult<bool> {
        EventFlag::LoyceKnightAbandonedDwelling.get()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::LoyceKnightAbandonedDwelling.set_area_conditional(state, MapId::FrozenEleumLoyce)
    }
}

impl ToggleCommand for FreeLoyceKnightLowerGarrison {
    fn is(&self) -> SysResult<bool> {
        EventFlag::LoyceKnightLowerGarrison.get()
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        EventFlag::LoyceKnightLowerGarrison.set_area_conditional(state, MapId::FrozenEleumLoyce)
    }
}

fn _set_event_flag_direct(flag_id: u32, state: bool) -> anyhow::Result<()> {
    player_loaded_check()?;
    if let Some((byte_addr, bit_mask)) = event_flag_lookup(flag_id)? {
        Ok(set_bit(byte_addr, bit_mask, state)?)
    } else {
        Err(anyhow!("Event flag not found"))
    }
}
