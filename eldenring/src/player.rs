use std::ptr;

pub use crate::offsets::{chr_dbg_flags::ChrDbgOffset, game_data_man::PlayerGameDataOffset};
use crate::{
    chr_ins::{self, ChrIns, ChrInsExt},
    mem::*,
    offsets::{
        ChainReadExt,
        code_cave::CaveOffset,
        game_data_man,
        module_offsets::{BasePointer, Data, Function, Hook, Patch},
        world_chr_man,
    },
    resources::ASM,
    travel,
    utils::player_loaded_check,
};
use crate::offsets::chr_ins as chr_ins_offsets;

use gubtool_core::{
    address::Address,
    attached::version,
    game_version::EldenRingVersion::*,
    slice_ops::*,
    sys::error::{PointerType, ProcResult, ProcessError},
};

pub fn player_ins() -> ChrIns {
    match read::<u64>(BasePointer::WorldChrMan).read_offset(world_chr_man::player_ins()) {
        Ok(ptr) if ptr != 0x0 => Ok(ptr),
        Ok(_) | Err(_) => Err(ProcessError::InvalidPointer {
            pointer_type: PointerType::PlayerIns,
        }),
    }
}

pub fn torrent_ins() -> ChrIns {
    let handle = read::<u64>(BasePointer::GameDataMan)
        .read_offset(game_data_man::torrent_handle())?;
    if handle == 0 {
        return Ok(0);
    }
    chr_ins::chr_ins_from_handle(handle)
}

pub fn set_chr_dbg_flag(offset: ChrDbgOffset, state: bool) -> ProcResult {
    write::<u8>(Data::ChrDbgFlags.add_offset(offset as u64), state as u8)
}

pub fn is_chr_dbg_flag(offset: ChrDbgOffset) -> ProcResult<bool> {
    read::<u8>(Data::ChrDbgFlags.add_offset(offset as u64)).map(|val| val == 1)
}

pub fn set_rune_arc(state: bool) -> ProcResult {
        player_game_data()
        .add_offset(PlayerGameDataOffset::RuneArc as u64)
        .write::<u8>(state as u8)
}

pub fn set_rfbs() -> ProcResult {
    let player_ins = player_ins();
    let max_hp = player_ins.get_max_hp()?;
    player_ins.set_hp((max_hp * 20) / 100 - 1)
}

/// Set player HP to exactly 1.
/// Used for challenge runs and one-HP mode.
pub fn set_1hp() -> ProcResult {
    player_ins().set_hp(1)
}

pub fn set_runes(amount: u32) -> ProcResult {
    let current_amount = PlayerGameData::read().rune_count;
    let to_give = amount as i32 - current_amount as i32;
    give_runes(to_give as i64)
}

pub fn give_runes(amount: i64) -> ProcResult {
    player_loaded_check()?;

    let mut fun = ASM.get_function("give_runes");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("player_game_data"), player_game_data()?)?;
    write_to_slice::<i64>(&mut asm, fun.reloc("amount"), amount)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_give_runes"), Function::GiveRunes)?;

    spawn_thread_join(CaveOffset::GiveRunesAsm, asm)
}

pub fn map_coords() -> ProcResult<[f32; 3]> {
    read::<[f32; 3]>(
        player_ins()? + world_chr_man::player_ins_offsets::current_map_coords(),
    )
}

pub fn map_angle() -> ProcResult<f32> {
    read::<f32>(
        player_ins()? + world_chr_man::player_ins_offsets::current_map_angle(),
    )
}

fn install_grab_hook() -> ProcResult {
    let mut fun = ASM.get_function("grab_hook");
    let mut asm = fun.take_bytes();

    let location = CaveOffset::NoGrabHook;
    let skip_grab_jmp_location = Hook::PlayerNoGrab.add_offset(0x95);

    write_addr_to_slice(&mut asm, fun.reloc("world_chr_man"), BasePointer::WorldChrMan)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("player_ins_off"), world_chr_man::player_ins())?;
    write_rel_i32(&mut asm, location, fun.reloc("skip_grab_jmp_location"), skip_grab_jmp_location, 4)?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), Hook::PlayerNoGrab.add_offset(9), 4)?;

    install_hook(&asm, location, Hook::PlayerNoGrab, 9)
}

const GRAB_HOOK_BYTES_ORIGINAL: [u8; 9] = [0x41, 0x8B, 0x56, 0x44, 0x48, 0x8D, 0x4C, 0x24, 0x40];
fn uninstall_grab_hook() -> ProcResult {
    write_bytes(Hook::PlayerNoGrab, &GRAB_HOOK_BYTES_ORIGINAL)
}

fn install_infinite_poise_hook() -> ProcResult {
    let mut fun = ASM.get_function("infinite_poise_hook");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("world_chr_man"), BasePointer::WorldChrMan)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("player_ins_off"), world_chr_man::player_ins())?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_get_chr_ins"), Function::GetChrInsByEntityId)?;
    write_rel_i32(
        &mut asm,
        CaveOffset::InfinitePoiseHook,
        fun.reloc("hook_loc"),
        Hook::PlayerInfinitePoise.add_offset(7),
        4,
    )?;
    install_hook(&asm, CaveOffset::InfinitePoiseHook, Hook::PlayerInfinitePoise, 7)
}

fn infinite_poise_bytes_original() -> [u8; 7] {
    match version() {
        Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) | Some(Version1_2_3) => {
            [0x4C, 0x8B, 0xC7, 0x41, 0x0F, 0xB6, 0xD6]
        }
        _ => [0x4C, 0x8B, 0xC7, 0x40, 0x0F, 0xB6, 0xD5],
    }
}
fn uninstall_infinite_poise_hook() -> ProcResult {
    write_bytes(Hook::PlayerInfinitePoise, &infinite_poise_bytes_original())
}

pub fn is_infinite_poise() -> ProcResult<bool> {
    read::<[u8; 7]>(Hook::PlayerInfinitePoise)
        .map(|val| val != infinite_poise_bytes_original())
}

pub fn set_infinite_poise(val: bool) -> ProcResult {
    match val {
        true => {
            install_infinite_poise_hook()?;
            install_grab_hook()
        }
        false => {
            uninstall_infinite_poise_hook()?;
            uninstall_grab_hook()
        }
    }
}

pub fn set_torrent_anywhere(state: bool) -> ProcResult {
    match state {
        true => {
            write_bytes(Patch::TorrentDisabledUnderworld, &[0x30, 0xC0, 0x90])?;
            write_bytes(Patch::WhistleDisabled, &[0x30, 0xC0, 0x90])
        }
        false => {
            write_bytes(Patch::TorrentDisabledUnderworld, &[0x0F, 0x95, 0xC0])?;
            write_bytes(Patch::WhistleDisabled, &[0x0F, 0x95, 0xC0])
        }
    }
}

pub fn is_torrent_anywhere() -> ProcResult<bool> {
    read::<[u8; 3]>(Patch::WhistleDisabled)
        .map(|val| val != [0x0F, 0x95, 0xC0])
}

fn player_game_data() -> ProcResult<u64> {
    read::<u64>(BasePointer::GameDataMan)
        .read_offset(game_data_man::PLAYER_GAME_DATA)
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct PlayerGameData {
    vftable: usize,
    pub character_event_id: u32,
    pub player_id: u32,
    pub current_hp: u32,
    pub current_max_hp: u32,
    pub base_max_hp: u32,
    pub current_fp: u32,
    pub current_max_fp: u32,
    pub base_max_fp: u32,
    unk28: f32,
    pub current_stamina: u32,
    pub current_max_stamina: u32,
    pub base_max_stamina: u32,
    unk38: f32,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
    pub base_hero_point: f32,
    pub base_hero_point_2: f32,
    pub base_durability: f32,
    pub level: u32,
    pub rune_count: u32,
    pub rune_memory: u32,
    unk74: u32,
    pub poison_resist: u32,
    pub rot_resist: u32,
    pub bleed_resist: u32,
    pub death_resist: u32,
    pub frost_resist: u32,
    pub sleep_resist: u32,
    pub madness_resist: u32,
    pub pending_block_clear_bonus: f32,
    pub chr_type: i32,
    character_name: [u16; 17],
    pub gender: u8,
    pub archetype: u8,
    pub vow_type: u8,
    unkc1: u8,
    pub voice_type: u8,
    pub starting_gift: u8,
    unkc4: u8,
    pub unlocked_magic_slots: u8,
    pub unlocked_talisman_slots: u8,
    pub matchmaking_spirit_ashes_level: u8,
    pub total_summon_count: u32,
    pub coop_success_count: u32,
    pub game_data_man_index: u32,
    unkd4: [u8; 0xb],
    pub furlcalling_finger_remedy_active: bool,
    unke0: u8,
    unke1: u8,
    pub matching_weapon_level: u8,
    pub white_ring_active: u8,
    pub blue_ring_active: u8,
    pub multiplay_role: u8,
    unke6: u8,
    pub is_my_world: bool,
    unke8: [u8; 0x3],
    unke9: bool,
    pub character_id: u32,
    pub invasions_success_count: u32,
    pub solo_breakin_point: u32,
    pub invaders_killed: u32,
    pub scadutree_blessing: u8,
    pub reversed_spirit_ash: u8,
    pub resist_curse_item_count: u8,
    pub rune_arc_active: bool,
    unk100: bool,
    pub max_hp_flask: u8,
    pub max_fp_flask: u8,
}

impl PlayerGameData {
    pub fn read() -> Self {
        if player_loaded_check().is_err() {
            return Self::default()
        }
        let bytes = read::<u64>(BasePointer::GameDataMan)
            .read_offset(game_data_man::PLAYER_GAME_DATA)
            .read::<[u8; std::mem::size_of::<Self>()]>()
            .unwrap_or([0x0; std::mem::size_of::<Self>()]);
        unsafe {
            ptr::read_unaligned(bytes.as_ptr() as *const Self)
        }
    }
}

pub fn set_stat(player_game_data_offset: PlayerGameDataOffset, val: i32) -> anyhow::Result<()> {
    player_loaded_check()?;

    let val = val.clamp(0, 99);

    let game_data = player_game_data()?;
    let current_val = read::<i32>(game_data + player_game_data_offset as u64)?;

    let diff = val - current_val;
    let current_level = read::<i32>(game_data + PlayerGameDataOffset::RuneLevel as u64)?;

    if val > current_val {
        let mut rune_cost = 0;
        for i in 1..=diff {
            rune_cost += level_up_cost(current_level + i);
        }
        let current_rune_mem = read::<u32>(game_data + PlayerGameDataOffset::RuneMemory as u64)?;
        let new_rune_mem = std::cmp::min(current_rune_mem as u64 + rune_cost as u64, 0xFFFFFFFF);
        write::<u32>(
            game_data + PlayerGameDataOffset::RuneMemory as u64,
            new_rune_mem as u32,
        )?;
    }
    write::<i32>(
        game_data + PlayerGameDataOffset::RuneLevel as u64,
        current_level + diff,
    )?;
    write::<i32>(game_data + player_game_data_offset as u64, val)?;
    Ok(())
}

pub fn set_dlc_stat(player_game_data_offset: PlayerGameDataOffset, val: u8) -> anyhow::Result<()> {
    player_loaded_check()?;
    write::<u8>(player_game_data()? + player_game_data_offset as u64, val.clamp(0, 20))?;
    Ok(())
}

fn level_up_cost(next_level: i32) -> i32 {
    let base_level_offset = 80_f32;
    let initial_level_up_cost = 0.1_f32;
    let initial_level_up_offset = 1_f32;
    let level_up_cost_increase = 0.02_f32;
    let level_up_increase_interval = 92_f32;

    let base_level = next_level as f32 + base_level_offset;
    let adjusted_level = 0.0_f32.max(base_level - level_up_increase_interval);
    let cost =
        base_level * base_level * (level_up_cost_increase * adjusted_level + initial_level_up_cost)
            + initial_level_up_offset;
    cost as i32
}

// Position save/load support

/// Threshold for determining if a same-area teleport is "long distance".
/// If the delta between current and saved position exceeds this value,
/// no-gravity is enabled temporarily to prevent fall damage during teleport.
/// 500.0 ≈ 2 × GRID_SIZE, meaning teleports across more than 2 grid cells
/// are considered long distance.
pub const LONG_DISTANCE_THRESHOLD: f32 = 500.0;

/// Size of each overworld grid cell in game units.
pub const GRID_SIZE: f32 = 256.0;
pub const OVERWORLD_AREA_ID: u8 = 0x3C;
pub const DLC_OVERWORLD_AREA_ID: u8 = 0x3D;

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub block_id: u32,
    pub coords: [f32; 3],
    pub angle: f32,
}

impl Position {
    pub fn read_from_cave(index: usize) -> ProcResult<Self> {
        let offset = if index == 0 { CaveOffset::SavedPos1 } else { CaveOffset::SavedPos2 };
        let bytes = read::<[u8; 24]>(offset)?;
        let block_id = read_from_slice::<u32>(&bytes, 0)?;
        let coords = read_from_slice::<[f32; 3]>(&bytes, 4)?;
        let angle = read_from_slice::<f32>(&bytes, 16)?;
        Ok(Self { block_id, coords, angle })
    }

    pub fn write_to_cave(&self, index: usize) -> ProcResult {
        let offset = if index == 0 { CaveOffset::SavedPos1 } else { CaveOffset::SavedPos2 };
        let mut bytes = [0u8; 24];
        write_to_slice::<u32>(&mut bytes, 0, self.block_id)?;
        write_to_slice::<[f32; 3]>(&mut bytes, 4, self.coords)?;
        write_to_slice::<f32>(&mut bytes, 16, self.angle)?;
        write_bytes(offset, &bytes)
    }

    /// Check if this position slot has valid (saved) data.
    /// An unsaved slot will have block_id == 0.
    /// We also check that at least one coordinate is non-zero to guard against
    /// partial writes (e.g., if a crash occurs during write_to_cave).
    pub fn is_valid(&self) -> bool {
        self.block_id != 0 && (self.coords[0] != 0.0 || self.coords[1] != 0.0 || self.coords[2] != 0.0)
    }
}

pub fn get_player_position() -> anyhow::Result<Position> {
    let player = player_ins()?;
    let block_id = read::<u32>(player + chr_ins_offsets::BLOCK_ID)?;
    let coords = map_coords()?;
    let angle = map_angle()?;
    Ok(Position { block_id, coords, angle })
}

/// Get the player's chr_ins address
fn player_addr() -> ProcResult<u64> {
    player_ins().map(|p| p.clone())
}

/// Get the player's physics pointer (same as ChrInsExt::physics_pointer)
fn player_physics_ptr() -> ProcResult<u64> {
    let player = player_addr()?;
    let modules = read::<u64>(player + chr_ins_offsets::MODULES)?;
    read::<u64>(modules + chr_ins_offsets::CHR_PHYSICS_MODULE)
}

/// Get the player's local coords (same as ChrInsExt::local_coords)
fn player_local_coords() -> ProcResult<[f32; 3]> {
    let physics_ptr = player_physics_ptr()?;
    read::<[f32; 3]>(physics_ptr + chr_ins_offsets::physics_offsets::COORDS)
}

/// Check if the player is in a restricted state where position restore should be blocked.
/// Returns an error with a descriptive message if the player is in a restricted state.
pub fn check_player_state_for_restore() -> anyhow::Result<()> {
    // Check if player is dead
    if let Ok(dead) = crate::emevd::is_player_dead() {
        if dead {
            anyhow::bail!("Cannot restore position: player is dead");
        }
    }
    
    // Check if player is on torrent
    if let Ok(torrent_handle) = torrent_ins() {
        if torrent_handle != 0 {
            anyhow::bail!("Cannot restore position: player is on torrent");
        }
    }
    
    Ok(())
}

/// Save the player's current position to the given slot (0 or 1).
pub fn save_position(index: usize) -> anyhow::Result<()> {
    if index >= 2 {
        anyhow::bail!("Position slot index must be 0 or 1, got {}", index);
    }
    player_loaded_check()?;
    let pos = get_player_position()?;
    pos.write_to_cave(index)?;
    Ok(())
}

/// Restore the player's position from the given slot (0 or 1).
pub async fn restore_position(index: usize) -> anyhow::Result<()> {
    if index >= 2 {
        anyhow::bail!("Position slot index must be 0 or 1, got {}", index);
    }
    player_loaded_check()?;
    
    // Check player state before attempting restore
    check_player_state_for_restore()?;
    
    let saved = Position::read_from_cave(index)?;
    
    // Check if the position slot has been saved (not all zeros)
    if !saved.is_valid() {
        anyhow::bail!("Position slot {} has not been saved yet", index + 1);
    }
    
    let current = get_player_position()?;

    let current_area = (current.block_id >> 24) & 0xFF;
    let saved_area = (saved.block_id >> 24) & 0xFF;

    if current_area == saved_area {
        // Re-check player state immediately before writing coords
        // to prevent TOCTOU race where player could die between
        // the initial check and the actual position write.
        check_player_state_for_restore()?;
        restore_same_area(&current, &saved)?;
    } else {
        restore_different_area(&saved).await?;
    }
    Ok(())
}

fn restore_same_area(current: &Position, saved: &Position) -> anyhow::Result<()> {
    let current_abs = to_absolute(current.coords, current.block_id);
    let saved_abs = to_absolute(saved.coords, saved.block_id);
    let delta = [
        saved_abs[0] - current_abs[0],
        saved_abs[1] - current_abs[1],
        saved_abs[2] - current_abs[2],
    ];

    let player = player_addr()?;
    let player_coords = player_local_coords()?;
    let new_coords = [
        player_coords[0] + delta[0],
        player_coords[1] + delta[1],
        player_coords[2] + delta[2],
    ];

    // Read physics pointer once and reuse for all writes.
    // The pointer chain (player -> modules -> chr_physics_module) is stable
    // once the player is loaded and doesn't change during synchronous execution.
    let physics_ptr = player_physics_ptr()?;

    let is_long_distance = delta[0].hypot(delta[1].hypot(delta[2])) > LONG_DISTANCE_THRESHOLD;

    if is_long_distance {
        // Enable no gravity for long distance teleports to prevent death
        write::<u8>(physics_ptr + chr_ins_offsets::physics_offsets::NO_GRAVITY, 1)?;
    }

    // Write new local coords
    write::<[f32; 3]>(physics_ptr + chr_ins_offsets::physics_offsets::COORDS, new_coords)?;

    // Also update map angle
    write::<f32>(
        player + world_chr_man::player_ins_offsets::current_map_angle(),
        saved.angle,
    )?;

    if is_long_distance {
        // Disable no gravity after a delay.
        // We verify the player is still loaded before writing to avoid writing to freed memory.
        // Use tokio::spawn to stay consistent with the rest of the codebase (see restore_different_area).
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            // Verify player is still loaded before writing
            if let Ok(verified_ptr) = {
                let player_check = match read::<u64>(BasePointer::WorldChrMan)
                    .read_offset(world_chr_man::player_ins())
                {
                    Ok(ptr) if ptr != 0 => Ok(ptr),
                    _ => Err(gubtool_core::sys::error::ProcessError::InvalidPointer {
                        pointer_type: gubtool_core::sys::error::PointerType::PlayerIns,
                    }),
                };
                player_check.and_then(|p| {
                    let modules = read::<u64>(p + chr_ins_offsets::MODULES)?;
                    read::<u64>(modules + chr_ins_offsets::CHR_PHYSICS_MODULE)
                })
            } {
                let _ = write::<u8>(
                    verified_ptr + chr_ins_offsets::physics_offsets::NO_GRAVITY,
                    0,
                );
            }
        });
    }

    Ok(())
}

async fn restore_different_area(saved: &Position) -> anyhow::Result<()> {
    // For different map areas, spawn a tokio task to warp to the saved position.
    // We use tokio::spawn since we're already on a tokio runtime (from spawn_task! in the TUI).
    // This avoids creating a new Runtime per warp, which would accumulate thread pools.
    let block_id = saved.block_id as i32;
    let coords = saved.coords;
    let angle = saved.angle;
    let handle = tokio::spawn(async move {
        let _ = travel::warp_to_block_id(block_id, coords, angle, false).await;
    });
    handle.await.map_err(|e| anyhow::anyhow!("Warp task failed: {}", e))?;
    Ok(())
}

/// Convert map coordinates to absolute world coordinates.
/// For overworld areas, the block_id encodes grid position (grid_x, grid_z)
/// which is used to offset the local map coordinates into world space.
/// Y coordinate is left unchanged because in Elden Ring, Y (height) is global
/// and not affected by grid position - this matches TarnishedTool's implementation.
/// For non-overworld areas (dungeons, interiors), coordinates are already absolute.
fn to_absolute(map_coords: [f32; 3], block_id: u32) -> [f32; 3] {
    let area = ((block_id >> 24) & 0xFF) as u8;
    if is_overworld_area(area) {
        let grid_x = ((block_id >> 16) & 0xFF) as f32;
        let grid_z = ((block_id >> 8) & 0xFF) as f32;
        [
            map_coords[0] + GRID_SIZE * grid_x,
            map_coords[1],
            map_coords[2] + GRID_SIZE * grid_z,
        ]
    } else {
        map_coords
    }
}

fn is_overworld_area(area: u8) -> bool {
    area == OVERWORLD_AREA_ID || area == DLC_OVERWORLD_AREA_ID
}

pub fn format_position(pos: &Position) -> String {
    let area = (pos.block_id >> 24) & 0xFF;
    let grid_x = (pos.block_id >> 16) & 0xFF;
    let map = (pos.block_id >> 8) & 0xFF;
    format!("[A{:02}][G{:03},M{:03}] ({:.1}, {:.1}, {:.1}) a:{:.2}",
        area, grid_x, map,
        pos.coords[0], pos.coords[1], pos.coords[2],
        pos.angle,
    )
}
