use crate::{
    core::common::{read_from_slice, rel_i32, write_to_slice},
    er::{
        chr_ins::{self, ChrIns, ChrInsExt},
        mem::*,
        offsets::{
            chr_dbg_flags::{self, ChrDbgOffsets},
            code_cave, functions,
            game_data_man::{self, player_game_data_offsets},
            hooks::{self},
            patches, world_chr_man,
        },
        resources::asm,
        utils::{character_loaded_check, dlc_check},
    },
};
use anyhow::{Result, anyhow, ensure};

pub fn player_ins() -> ChrIns {
    read::<u64>(world_chr_man::base())
        .and_then(|addr| read::<u64>(addr + world_chr_man::player_ins()))
        .map_err(|_| anyhow!("Character not loaded"))
}

pub fn torrent_ins() -> ChrIns {
    let handle = read::<u64>(game_data_man::base())
        .and_then(|addr| read::<u64>(addr + game_data_man::PLAYER_GAME_DATA))
        .and_then(|addr| read::<u64>(addr + game_data_man::torrent_handle()))?;
    chr_ins::chr_ins_from_handle(handle)
}

pub fn set_chr_dbg_flag(offset: ChrDbgOffsets, state: bool) -> Result<()> {
    write::<u8>(chr_dbg_flags::base() + offset as u64, state as u8)
}

pub fn is_chr_dbg_flag(offset: ChrDbgOffsets) -> Result<bool> {
    read::<u8>(chr_dbg_flags::base() + offset as u64)
        .map(|val| val == 1)
}

pub fn set_rune_arc(state: bool) -> Result<()> {
    read::<u64>(game_data_man::base())
        .and_then(|addr| read::<u64>(addr + game_data_man::PLAYER_GAME_DATA))
        .and_then(|addr| write::<u8>(addr + player_game_data_offsets::RUNE_ARC, state as u8))
}

pub fn set_rfbs() -> Result<()> {
    let player_ins = player_ins();
    let max_hp = player_ins.get_max_hp()?;
    player_ins.set_hp((max_hp * 20) / 100 - 1)
}

pub fn give_runes(amount: i64) -> Result<()> {
    character_loaded_check()?;

    let location = code_cave::base() + code_cave::RUNE_GIVE_ASM;
    let player_game_data = read::<u64>(game_data_man::base())
        .and_then(|addr| read::<u64>(addr + game_data_man::PLAYER_GAME_DATA))?;

    let mut asm = asm::GIVE_RUNES;
    write_to_slice::<u64>(&mut asm, 2, player_game_data)?;
    write_to_slice::<i64>(&mut asm, 12, amount)?;
    write_to_slice::<u64>(&mut asm, 22, functions::give_runes())?;
    let asm = append_flag_setter(location, &asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

pub fn map_coords() -> Result<[f32; 3]> {
    read::<[f32; 3]>(player_ins().chr_ins_pointer()? + world_chr_man::player_ins_offsets::current_map_coords())
}

pub fn map_angle() -> Result<f32> {
    read::<f32>(player_ins().chr_ins_pointer()? + world_chr_man::player_ins_offsets::current_map_angle())
}

fn install_grab_hook() -> Result<()> {
    let mut asm = asm::GRAB_HOOK;
    let location = code_cave::base() + code_cave::NO_GRAB_ASM;
    let skip_grab_jmp_location = hooks::no_grab() + 0x95;

    write_to_slice::<i32>(&mut asm, 4, rel_i32(world_chr_man::base(), location + 8)?)?;
    write_to_slice::<i32>(&mut asm, 11, world_chr_man::player_ins())?;
    write_to_slice::<i32>(&mut asm, 22, rel_i32(skip_grab_jmp_location, location + 26)?)?;
    write_to_slice::<i32>(&mut asm, 36, rel_i32(hooks::no_grab() + 9, location + 40)?)?;

    let mut hookbytes: [u8; 9] = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90, 0x90, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::no_grab() + 5)?)?;

    write_bytes(location, &asm)?;
    write_bytes(hooks::no_grab(), &hookbytes)
}

fn uninstall_grab_hook() -> Result<()> {
    write_bytes(hooks::no_grab(), &asm::GRAB_HOOK_BYTES_ORIGINAL)
}

fn install_infinite_poise_hook() -> Result<()> {
    let mut asm = asm::INFINITE_POISE_HOOK;
    let location = code_cave::base() + code_cave::INFINITE_POISE_ASM;

    write_to_slice::<i32>(&mut asm, 11, rel_i32(world_chr_man::base(), location + 15)?)?;
    write_to_slice::<i32>(&mut asm, 18, world_chr_man::player_ins())?;
    write_to_slice::<i32>(&mut asm, 27, world_chr_man::player_ins())?;
    write_to_slice::<i32>(&mut asm, 64, rel_i32(world_chr_man::base(), location + 68)?)?;
    write_to_slice::<i32>(&mut asm, 84, rel_i32(functions::get_chr_ins_by_entity_id(), location + 88)?)?;
    write_to_slice::<i32>(&mut asm, 107, rel_i32(hooks::infinite_poise() + 7, location + 111)?)?;

    let mut hookbytes: [u8; 7] = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::infinite_poise() + 5)?)?;

    write_bytes(location, &asm)?;
    write_bytes(hooks::infinite_poise(), &hookbytes)
}

fn uninstall_infinite_poise_hook() -> Result<()> {
    write_bytes(hooks::infinite_poise(), &asm::infinite_poise_bytes_original())
}

pub fn is_infinite_poise() -> Result<bool> {
    read::<[u8; 7]>(hooks::infinite_poise())
        .map(|val| val != asm::infinite_poise_bytes_original())
}

pub fn set_infinite_poise(val: bool) -> Result<()> {
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

pub fn set_torrent_anywhere(state: bool) -> Result<()> {
    match state {
        true => {
            write_bytes(patches::torrent_disabled_underworld(), &[0x30, 0xC0, 0x90])?;
            write_bytes(patches::whistle_disabled(), &[0x30, 0xC0, 0x90])
        }
        false => {
            write_bytes(patches::torrent_disabled_underworld(), &[0x0F, 0x95, 0xC0])?;
            write_bytes(patches::whistle_disabled(), &[0x0F, 0x95, 0xC0])
        }
    }
}

pub fn is_torrent_anywhere() -> Result<bool> {
    read::<[u8; 3]>(patches::whistle_disabled())
        .map(|val| val == [0x30, 0xC0, 0x90])
}

fn player_game_data() -> Result<u64> {
    read::<u64>(game_data_man::base())
        .and_then(|addr| read::<u64>(addr + game_data_man::PLAYER_GAME_DATA))
}

#[derive(Clone, Debug, Default)]
pub struct PlayerStats {
    pub vigor: i32,
    pub mind: i32,
    pub endurance: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub faith: i32,
    pub arcane: i32,
    pub rune_level: i32,
    pub runes: i32,
    pub rune_memory: u32,
    pub scadutree: u8,
    pub spirit_ash: u8,
    pub rune_arc: bool,
}

impl PlayerStats {
    pub fn new() -> PlayerStats {
        let mut stats = PlayerStats::default();
        stats.update().ok();
        stats
    }
    pub fn update(&mut self) -> Result<()> {
        character_loaded_check()?;
        let array = read::<[u8; 0x100]>(player_game_data()?)?;
        self.vigor = read_from_slice::<i32>(&array, player_game_data_offsets::VIGOR)?;
        self.mind = read_from_slice::<i32>(&array, player_game_data_offsets::MIND)?;
        self.endurance = read_from_slice::<i32>(&array, player_game_data_offsets::ENDURANCE)?;
        self.strength = read_from_slice::<i32>(&array, player_game_data_offsets::STRENGTH)?;
        self.dexterity = read_from_slice::<i32>(&array, player_game_data_offsets::DEXTERITY)?;
        self.intelligence = read_from_slice::<i32>(&array, player_game_data_offsets::INTELLIGENCE)?;
        self.faith = read_from_slice::<i32>(&array, player_game_data_offsets::FAITH)?;
        self.arcane = read_from_slice::<i32>(&array, player_game_data_offsets::ARCANE)?;
        self.rune_level = read_from_slice::<i32>(&array, player_game_data_offsets::RUNE_LEVEL)?;
        self.runes = read_from_slice::<i32>(&array, player_game_data_offsets::RUNES)?;
        self.rune_memory = read_from_slice::<u32>(&array, player_game_data_offsets::RUNE_MEMORY)?;
        self.scadutree = array[player_game_data_offsets::SCADUTREE as usize];
        self.spirit_ash = array[player_game_data_offsets::SPIRIT_ASH as usize];
        self.rune_arc = array[player_game_data_offsets::RUNE_ARC as usize] != 0;
        Ok(())
    }
}

pub fn set_stat(player_game_data_offset: u64, val: i32) -> Result<()> {
    character_loaded_check()?;
    ensure!((0..=99).contains(&val), "Valid range: [0,99]");

    let game_data = player_game_data()?;
    let current_val = read::<i32>(game_data + player_game_data_offset)?;

    let diff = val - current_val;
    let current_level = read::<i32>(game_data + player_game_data_offsets::RUNE_LEVEL)?;

    if val > current_val {
        let mut rune_cost = 0;
        for i in 1..=diff {
            rune_cost += level_up_cost(current_level + i);
        }
        let current_rune_mem = read::<u32>(game_data + player_game_data_offsets::RUNE_MEMORY)?;
        let new_rune_mem = std::cmp::min(current_rune_mem as u64 + rune_cost as u64, 0xFFFFFFFF);
        write::<u32>(game_data + player_game_data_offsets::RUNE_MEMORY, new_rune_mem as u32)?;
    }
    write::<i32>(game_data + player_game_data_offsets::RUNE_LEVEL, current_level + diff)?;
    write::<i32>(game_data + player_game_data_offset, val)
}

pub fn set_dlc_stat(player_game_data_offset: u64, val: u8) -> Result<()> {
    dlc_check()?;
    ensure!((0..=20).contains(&val), "Valid range: [0,20]");
    write::<u8>(player_game_data()? + player_game_data_offset, val)
}

fn level_up_cost(next_level: i32) -> i32 {
    let base_level_offset = 80_f32;
    let initial_level_up_cost = 0.1_f32;
    let initial_level_up_offset= 1_f32;
    let level_up_cost_increase = 0.02_f32;
    let level_up_increase_interval = 92_f32;

    let base_level = next_level as f32 + base_level_offset;
    let adjusted_level = 0.0_f32.max(base_level - level_up_increase_interval);
    let cost = base_level * base_level * (level_up_cost_increase * adjusted_level + initial_level_up_cost) + initial_level_up_offset;
    cost as i32
}
