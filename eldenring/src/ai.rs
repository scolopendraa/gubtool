use {
    crate::{
        mem::{read, run_game_function},
        offsets::{
            chr_ins::ai_think_offsets::{self, attack_comp_offsets},
            module_offsets::Function,
        },
    },
    gubtool_core::sys::{ipc::FfiValue, sys_error::SysResult},
};

#[repr(C, packed)]
#[derive(Debug)]
pub struct CoolTime {
    animation_id:            u32,
    time_since_last_applied: f32,
    cooldown:                f32,
}

pub fn get_cool_time_list(ai_think_ptr: u64) -> SysResult<Vec<CoolTime>> {
    let mut items = Vec::new();

    let attack_comp = ai_think_ptr + ai_think_offsets::ai_attack_comp();
    let cool_time_count = read::<i32>(attack_comp + attack_comp_offsets::COOLDOWN_COUNT)?;

    if cool_time_count == 0 {
        return Ok(items);
    }

    let list_start = attack_comp + attack_comp_offsets::COOLDOWN_LIST;

    for i in 0..cool_time_count {
        let off = 0x14 * i as u64;
        let cool_time_entry = read::<CoolTime>(list_start + off)?;
        items.push(cool_time_entry);
    }
    Ok(items)
}

pub fn add_cool_time(ai_think_ptr: u64, animation_id: u32) -> anyhow::Result<()> {
    let args = [
        FfiValue::pointer(ai_think_ptr),
        FfiValue::uint32(animation_id),
    ];

    run_game_function(Function::AddCoolTime, &args)
}
