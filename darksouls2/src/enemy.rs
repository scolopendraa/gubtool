use {
    crate::{
        event::set_event_flag,
        mem::{install_hook, read, read_address, run_custom_function, write},
        offsets::{
            ChainReadExt,
            code_cave::CaveAddr,
            game_manager_imp,
            module_offsets::{BasePointer, Function, Hook},
        },
        pointer_cache::ResolvedPtr,
        resources::{asm_function, bosses::Boss},
    },
    assemble::patch::{DWORD, WORD},
    gubtool_core::{
        address::{Address, POINTER},
        attached::is_32,
        slice_ops::{read_from_slice, write_to_slice},
        sys::sys_error::SysResult,
    },
    std::{
        collections::HashMap,
        sync::{LazyLock, Mutex, MutexGuard},
        time::Duration,
    },
};

static ACT_LOGGER: LazyLock<Mutex<ActLogger>> = LazyLock::new(|| Mutex::new(ActLogger::default()));

pub fn act_logger() -> MutexGuard<'static, ActLogger> {
    ACT_LOGGER.lock().unwrap()
}

#[derive(Default, Debug)]
pub struct ActLogger {
    map:      HashMap<u64, i32>,
    read_idx: i32,
}

impl ActLogger {
    pub fn update(&mut self) -> SysResult {
        let buffer = read::<[u8; 0x50]>(CaveAddr::SavedActBuffer)?;
        let (write_idx, buffer) = buffer.split_at(4);
        let write_idx = read_from_slice::<i32>(write_idx, 0)?;

        let num_to_read = ((write_idx - self.read_idx).rem_euclid(6)) as usize;

        for i in 0..num_to_read {
            let idx = (self.read_idx + i as i32) % 6;
            let read_offset = idx * 12;

            let chr_ai_pointer = read_from_slice::<u64>(buffer, read_offset as u64)?;
            let act_id = read_from_slice::<i32>(buffer, (read_offset + 8) as u64)?;

            let _ = self.map.insert(chr_ai_pointer, act_id);
        }

        self.read_idx = write_idx;
        Ok(())
    }

    pub fn get(&self, chr_ai: u64) -> Option<i32> {
        self.map.get(&chr_ai).copied()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

pub fn install_act_logger_hook() -> SysResult {
    let mut fun = asm_function("target_action_hook");

    fun.patch::<POINTER>("force_act_flag", CaveAddr::ForceActFlag);
    fun.patch::<POINTER>("repeating_chr_ai", CaveAddr::ForceActChrAi);
    fun.patch::<POINTER>("force_act_id", CaveAddr::ForceActId);
    fun.patch::<POINTER>("buffer", CaveAddr::SavedActBuffer);

    install_hook(&fun.bytes, CaveAddr::TargetActHook, Function::ChrSetAction, 7)
}

pub fn is_chr_ai_disabled(chr_ai: u64) -> SysResult<bool> {
    let buffer = read::<[u8; 0x104]>(CaveAddr::DisabledEnemies)?;
    let count = read_from_slice::<u32>(&buffer, 0)?;

    for i in 0..count {
        let current_chr_ai = read_from_slice::<u64>(&buffer, 4 + 8 * i as u64)?;
        if current_chr_ai == chr_ai {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn set_disable_chr_ai(chr_ai: u64, state: bool) -> SysResult {
    let mut buffer = read::<[u8; 0x104]>(CaveAddr::DisabledEnemies)?;
    let mut count = read_from_slice::<u32>(&buffer, 0)?;

    const MAX_ENTRIES: u32 = 20;

    match state {
        true => {
            install_disable_ai_hook()?;

            if count >= MAX_ENTRIES {
                buffer.copy_within(12.., 4);
            } else {
                count += 1;
                write_to_slice::<u32>(&mut buffer, 0, count)?;
            }
            let index = 4 + 8 * (count - 1);
            write_to_slice::<u64>(&mut buffer, index as u64, chr_ai)?;
        }
        false => {
            for i in 0..count {
                let current_idx = 4 + 8 * i as u64;
                let current_chr_ai = read_from_slice::<u64>(&buffer, current_idx)?;
                if current_chr_ai == chr_ai {
                    write_to_slice::<u32>(&mut buffer, 0, count.saturating_sub(1))?;
                    buffer.copy_within(current_idx as usize + 8.., current_idx as usize);
                    break;
                }
            }
        }
    }

    write::<[u8; 0x104]>(CaveAddr::DisabledEnemies, buffer)
}

pub fn clear_disabled_targets() -> SysResult {
    write::<[u8; 0x104]>(CaveAddr::DisabledEnemies, [0u8; 0x104])
}

pub fn install_disable_ai_hook() -> SysResult {
    let code_loc = CaveAddr::DisableAiHook;
    let hook_loc = Hook::DisableAi;
    let normal_jump = if is_32() { 5 } else { 6 };
    let disable_jump = if is_32() { 10 } else { 9 };
    let mut fun = asm_function("disable_ai_hook");

    fun.patch::<POINTER>("disable_all_flag", CaveAddr::DisableAllExceptTarget);
    fun.patch::<POINTER>("target_loc", CaveAddr::SavedTargetPointer);
    fun.patch::<POINTER>("buffer_loc", CaveAddr::DisabledEnemies);
    fun.patch_rel32("hook_loc_skip_call", code_loc, hook_loc.add(disable_jump), 4);
    fun.patch_rel32("hook_loc_normal", code_loc, hook_loc.add(normal_jump), 4);

    install_hook(&fun.bytes, code_loc, hook_loc, 5)
}

pub fn reset_enemy(obj_id: u16, bonfire_id: u32) -> anyhow::Result<()> {
    let mut fun = asm_function("reset_enemy");

    fun.patch::<POINTER>("game_manager_imp", BasePointer::GameManagerImp);
    fun.patch::<WORD>("obj_id", obj_id);
    fun.patch::<DWORD>("bonfire_id", bonfire_id);
    fun.patch::<POINTER>("fn_reset_enemy", Function::ResetEnemy);

    run_custom_function(fun)
}

impl Boss {
    pub async fn revive(&self) -> anyhow::Result<()> {
        set_event_flag(self.death_flag, false)?;

        self.warp()?;

        let player_ctrl = ResolvedPtr::GameManagerImp
            .get()
            .add_offset(game_manager_imp::PLAYER_CTRL)?;

        while read_address(player_ctrl)? != 0x0 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }

        while read_address(player_ctrl)? == 0x0 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }

        for flag in self.revive_flags {
            set_event_flag(*flag, false)?;
        }

        for id in self.obj_ids {
            reset_enemy(*id, self.bonfire_id)?;
        }

        Ok(())
    }
}
