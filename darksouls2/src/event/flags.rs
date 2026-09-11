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
        address::{Address, POINTER, Pointer},
        attached::is_32,
        game_version::Game::DarkSouls2,
        slice_ops::*,
        sys::{
            install_hook,
            ipc::{FfiValue, X86CallingConvention},
            is_bit_set,
            read,
            read_pointer,
            set_bit,
            sys_error::SysResult,
            write,
            write_bytes,
        },
    },
    shared::{command::ToggleCommand, event_log::EventLog, impl_event_logger, toggle_command},
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
    bitmap_ptr: Pointer,
    size:       u32,
    key:        u32,
    next_node:  Pointer,
}

impl Node {
    fn read_at(address: Pointer) -> SysResult<Self> {
        if is_32() {
            let bytes = read::<[u8; 0x10]>(address)?;
            Ok(Self {
                bitmap_ptr: Pointer::new(
                    read_from_slice::<u32>(&bytes, 0x0)? as u64,
                    Some(DarkSouls2),
                ),
                size:       read_from_slice::<u32>(&bytes, 0x4)?,
                key:        read_from_slice::<u32>(&bytes, 0x8)?,
                next_node:  Pointer::new(
                    read_from_slice::<u32>(&bytes, 0xc)? as u64,
                    Some(DarkSouls2),
                ),
            })
        } else {
            let bytes = read::<[u8; 0x18]>(address)?;
            Ok(Self {
                bitmap_ptr: Pointer::new(read_from_slice::<u64>(&bytes, 0x0)?, Some(DarkSouls2)),
                size:       read_from_slice::<u32>(&bytes, 0x8)?,
                key:        read_from_slice::<u32>(&bytes, 0xc)?,
                next_node:  Pointer::new(read_from_slice::<u64>(&bytes, 0x10)?, Some(DarkSouls2)),
            })
        }
    }
}

fn event_flag_lookup(flag_id: u32) -> SysResult<Option<(Pointer, u8)>> {
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

    let mut node_ptr = read_pointer(event_flag_man + first_node_offset)?;

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

impl_event_logger!(Ds2EventLogger, "darksouls2");

const EVENT_LOG_HOOK_ORIGINAL: [u8; 5] = [0xb8, 0x59, 0x17, 0xb7, 0xd1];
toggle_command!(StartEventLogger {
    is: {
        read::<[u8; 5]>(Hook::EventLog)
            .map(|bytes| bytes != EVENT_LOG_HOOK_ORIGINAL)
    }

    set(state): {
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
});

const VANILLA_IVORY_SKIP_ORIGINAL: [u8; 6] = [0x55, 0x8b, 0xec, 0x83, 0xec, 0x08];
const SCHOLAR_IVORY_SKIP_ORIGINAL: [u8; 5] = [0x48, 0x89, 0x74, 0x24, 0x10];
toggle_command!(SkipIvoryKingGauntlet {
    is: {
        if is_32() {
            read::<[u8; 6]>(Function::SetEvent)
                .map(|val| val != VANILLA_IVORY_SKIP_ORIGINAL)
        } else {
            read::<[u8; 5]>(Function::SetEvent)
                .map(|val| val != SCHOLAR_IVORY_SKIP_ORIGINAL)
        }
    }

    set(state): {
        if state {
            let orig_instr_len = if is_32() { 6 } else { 5 };
            let mut fun = asm_function("ivory_skip");

            fun.patch::<POINTER>("fn_get_map_entity", Function::MapEntityFromMapIdAndObjId);
            fun.patch::<POINTER>("fn_get_map_object", Function::GetStateActComponent);
            fun.patch::<POINTER>("fn_set_event", Function::SetEvent);
            fun.patch_rel32(
                "hook_loc",
                CaveAddr::IvorySkipHook,
                Function::SetEvent.ptr() + orig_instr_len,
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
});

const VANILLA_LOYCE_SKIP_ORIGINAL: [u8; 7] = [0x88, 0x94, 0x08, 0xa1, 0x02, 0x00, 0x00];
const SCHOLAR_LOYCE_SKIP_ORIGINAL: [u8; 8] = [0x44, 0x88, 0x84, 0x08, 0xa1, 0x03, 0x00, 0x00];
toggle_command!(DisableLoyceKnights {
    is: {
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

    set(state): {
        if state {
            let orig_instr_len = if is_32() { 7 } else { 8 };
            let mut fun = asm_function("ivory_knights");
            fun.patch_rel32(
                "hook_loc",
                CaveAddr::IvoryKnightsHook,
                Hook::SetSharedFlag.ptr() + orig_instr_len,
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
});

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

toggle_command!(KingsRingAcquired (display = "King's Ring Acquired") {
    is: {
        EventFlag::KingsRingAcquired.get()
    }

    set(state): {
        EventFlag::KingsRingAcquired.set(state)
    }
});

toggle_command!(NashandraUnlocked {
    is: {
        EventFlag::GiantLordDefeated.get()
    }

    set(state): {
        EventFlag::GiantLordDefeated.set(state)
    }
});

toggle_command!(AldiaUnlocked {
    is: {
        Ok(EventFlag::VendrickDefeated.get()? && EventFlag::UnlockAldia.get()?)
    }

    set(state): {
        EventFlag::VendrickDefeated.set(state)?;
        EventFlag::UnlockAldia.set(state)
    }
});

toggle_command!(DarkChasmLitShadedWoods (display = "Dark Chasm Lit (Shaded Woods)") {
    is: {
        EventFlag::ShadedWoodsChasmCleared.get()
    }

    set(state): {
        EventFlag::ShadedWoodsChasmCleared.set_area_conditional(state, MapId::DarkChasmOfOld)
    }
});

toggle_command!(DarkChasmLitDrangleicCastle (display = "Dark Chasm Lit (Drangleic Castle)") {
    is: {
        EventFlag::DrangleicCastleChasmCleared.get()
    }

    set(state): {
        EventFlag::DrangleicCastleChasmCleared.set_area_conditional(state, MapId::DarkChasmOfOld)
    }
});

toggle_command!(DarkChasmLitBlackGulch (display = "Dark Chasm Lit (Black Gulch)") {
    is: {
        EventFlag::BlackGulchChasmCleared.get()
    }

    set(state): {
        EventFlag::BlackGulchChasmCleared.set_area_conditional(state, MapId::DarkChasmOfOld)
    }
});

toggle_command!(BrumeTowerActivated {
    is: {
        EventFlag::ActivateBrume.get()
    }

    set(state): {
        EventFlag::ActivateBrume.set_area_conditional(state, MapId::BrumeTower)
    }
});

toggle_command!(AavaVisible {
    is: {
        EventFlag::VisibleAava.get()
    }

    set(state): {
        EventFlag::VisibleAava.set_area_conditional(state, MapId::FrozenEleumLoyce)
    }
});

toggle_command!(UndoAlsanasSeal (display = "Alsana's Seal Undone") {
    is: {
        Ok(EventFlag::EleumLoyceWinds.get()? && EventFlag::EleumLoyceWinds.get()?)
    }

    set(state): {
        EventFlag::EleumLoyceIce.set_area_conditional(state, MapId::FrozenEleumLoyce)?;
        EventFlag::EleumLoyceWinds.set_area_conditional(state, MapId::FrozenEleumLoyce)
    }
});

toggle_command!(FreeLoyceKnightOuterWall (display = "Loyce Knight Freed (Outer Wall)") {
    is: {
        EventFlag::LoyceKnightOuterWall.get()
    }

    set(state): {
        EventFlag::LoyceKnightOuterWall.set_area_conditional(state, MapId::FrozenEleumLoyce)
    }
});

toggle_command!(FreeLoyceKnightAbandonedDwelling (display = "Loyce Knight Freed (Abandoned Dwelling)") {
    is: {
        EventFlag::LoyceKnightAbandonedDwelling.get()
    }

    set(state): {
        EventFlag::LoyceKnightAbandonedDwelling.set_area_conditional(state, MapId::FrozenEleumLoyce)
    }
});

toggle_command!(FreeLoyceKnightLowerGarrison (display = "Loyce Knight Freed (Lower Garrison)") {
    is: {
        EventFlag::LoyceKnightLowerGarrison.get()
    }

    set(state): {
        EventFlag::LoyceKnightLowerGarrison.set_area_conditional(state, MapId::FrozenEleumLoyce)
    }
});

fn _set_event_flag_direct(flag_id: u32, state: bool) -> anyhow::Result<()> {
    player_loaded_check()?;
    if let Some((byte_addr, bit_mask)) = event_flag_lookup(flag_id)? {
        Ok(set_bit(byte_addr, bit_mask, state)?)
    } else {
        Err(anyhow!("Event flag not found"))
    }
}
