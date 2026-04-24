use anyhow::Result;
use crate::{
    core::{
        event,
        mem::*,
        utils::character_loaded_check,
    },
    offsets::{cs_flipper_imp, damage_manager, game_data_man, game_man, map_debug_flags, patches, user_input_manager},
};

pub fn quitout() -> Result<()> {
    character_loaded_check()?;
    read::<u64>(game_man::base())
        .and_then(|addr| write::<u8>(addr + 0x10, 0x1))
}

pub fn get_ng_cycle() -> Result<i32> {
    read::<u64>(game_data_man::base())
        .and_then(|addr| read::<i32>(addr + game_data_man::NEW_GAME))
}

const EVENT_IDS: [u32; 8] = [50, 51, 52, 53, 54, 55, 56, 57];
pub fn set_ng_cycle(val: i32) -> Result<()> {
    read::<u64>(game_data_man::base())
        .and_then(|addr| write::<i32>(addr + game_data_man::NEW_GAME, val))?;

    let current_ng = get_ng_cycle()?.clamp(0, 7);
    EVENT_IDS.iter().enumerate()
        .try_for_each(|(i, &id)| event::set_event(id, i == current_ng as usize))
}

pub fn trigger_new_game() -> Result<()> {
    character_loaded_check()?;
    read::<u64>(game_man::base())
        .and_then(|addr| write::<u8>(addr + game_man::start_new_game(), 0x1))
}

pub fn set_fps_cap(value: f32) -> Result<()> {
    write::<f32>(patches::fps_cap() + 0x3, 1_f32 / value)
}

pub fn get_fps_cap() -> Result<f32> {
    read::<f32>(patches::fps_cap() + 0x3)
        .map(|val| (1_f32 / val).round())
}

pub fn set_logo_patch(state: bool) -> Result<()> {
    match state {
        false => write_bytes(patches::no_logo(), &[0x74, 0x53]),
        true => write_bytes(patches::no_logo(), &[0x90, 0x90]),
    }
}

pub fn is_logo_patch() -> Result<bool> {
    read::<[u8; 2]>(patches::no_logo())
        .map(|val| val == [0x90, 0x90])
}

pub fn set_freeze_world(state: bool) -> Result<()> {
    match state {
        false => write_bytes(patches::pause_world(), &[0x0F, 0x84]),
        true => write_bytes(patches::pause_world(), &[0x0F, 0x85]),
    }
}

pub fn is_freeze_world_on() -> Result<bool> {
    read::<[u8; 2]>(patches::pause_world())
        .map(|val| val == [0x0F, 0x85])
}

pub fn mute_music(state: bool) -> Result<()> {
    match state {
        false => write_bytes(patches::mute_music(), &[0x0F, 0xB6, 0x48, 0x04]),
        true => write_bytes(patches::mute_music(), &[0x31, 0xC9, 0x90, 0x90]),
    }
}

pub fn is_music_muted() -> Result<bool> {
    read::<[u8; 4]>(patches::mute_music())
        .map(|val| val == [0x31, 0xC9, 0x90, 0x90])
}

pub fn draw_hitboxes(val: bool, is_view_b: bool) -> Result<()> {
    let offset = if is_view_b { damage_manager::HITBOXVIEW_B } else { damage_manager::HITBOXVIEW_A };
    read::<u64>(damage_manager::base())
        .map(|addr| write::<i64>(addr + offset, val as i64))?
}

pub fn is_hitboxes(is_view_b: bool) -> Result<bool> {
    let offset = if is_view_b { damage_manager::HITBOXVIEW_B } else { damage_manager::HITBOXVIEW_A };
    read::<u64>(damage_manager::base())
        .and_then(|addr| read::<i64>(addr + offset))
        .map(|val| val == 1)
}

pub fn show_all_graces(val: bool) -> Result<()> {
    write::<u8>(map_debug_flags::base() + map_debug_flags::SHOW_ALL_GRACES, val as u8)
}

pub fn is_show_all_graces_on() -> Result<bool> {
    read::<u8>(map_debug_flags::base() + map_debug_flags::SHOW_ALL_GRACES)
        .map(|val| val == 1)
}

pub fn show_all_maps(val: bool) -> Result<()> {
    write::<u8>(map_debug_flags::base() + map_debug_flags::SHOW_ALL_MAPS, val as u8)
}

pub fn is_show_all_maps_on() -> Result<bool> {
    read::<u8>(map_debug_flags::base() + map_debug_flags::SHOW_ALL_MAPS)
        .map(|val| val == 1)
}

pub fn set_stutter_fix(val: bool) -> Result<()> {
    read::<u64>(user_input_manager::base())
        .and_then(|addr| write::<u8>(addr + user_input_manager::STEAM_INPUT, val as u8))
}

pub fn is_stutter_fix_on() -> Result<bool> {
    read::<u64>(user_input_manager::base())
        .and_then(|addr| read::<u8>(addr + user_input_manager::STEAM_INPUT))
        .map(|val| val == 1)
}

pub fn set_game_speed(val: f32) -> Result<()> {
    read::<u64>(cs_flipper_imp::base())
        .and_then(|addr| write::<f32>(addr + cs_flipper_imp::game_speed(), val))
}

pub fn get_game_speed() -> Result<f32> {
    read::<u64>(cs_flipper_imp::base())
        .and_then(|addr| read::<f32>(addr + cs_flipper_imp::game_speed()))
}

/*
pub fn set_music(val: u8) -> Result<()> {
    read::<u64>(game_data_man::base())
        .and_then(|addr| read::<u64>(addr + game_data_man::OPTIONS))
        .and_then(|addr| write::<u8>(addr + game_data_man::options_offsets::MUSIC, val))
}
 */
