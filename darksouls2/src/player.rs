use {
    crate::{
        chr_ctrl::{ChrCtrl, ResolvedChrPtr},
        game_state::{self, StateFlag},
        mem::*,
        offsets::{
            ChainReadExt,
            Offset,
            chr_ctrl::stats_offsets::{self},
            code_cave::CaveAddr,
            game_manager_imp,
            module_offsets::{BasePointer, Function, Hook, Patch},
        },
        pointer_cache::ResolvedPtr,
        resources::asm_function,
        speffect,
        utils::player_loaded_check,
    },
    gubtool_core::{
        address::{Address, POINTER},
        attached::{is_32, version},
        game_version::DarkSouls2Version,
        slice_ops::*,
        sys::{
            ipc::{FfiValue, X86CallingConvention},
            sys_error::{PointerType, SysError, SysResult},
        },
    },
    shared::{
        command::{StatCommand, ToggleCommand, UnitCommand, ValueCommand},
        toggle_command,
        unit_command,
        value_command,
    },
    std::sync::{LazyLock, Mutex, MutexGuard, RwLock},
    strum::Display,
};

static PLAYER: LazyLock<Mutex<Player>> = LazyLock::new(|| Mutex::new(Player::new()));

pub fn player() -> MutexGuard<'static, Player> {
    PLAYER.lock().unwrap()
}

pub struct Player {
    chr_ctrl: Option<ChrCtrl>,
}

impl Player {
    fn new() -> Self {
        let mut player = Self {
            chr_ctrl: None,
        };
        player.update();
        player
    }
    pub fn update(&mut self) {
        let ptr = ResolvedPtr::GameManagerImp
            .get()
            .read_offset(game_manager_imp::PLAYER_CTRL);
        match ptr {
            Ok(pointer) => {
                if pointer != 0x0 {
                    self.chr_ctrl = Some(ChrCtrl::new(pointer));
                } else {
                    self.chr_ctrl = None;
                }
            }
            Err(_) => {
                self.chr_ctrl = None;
            }
        }
    }
    pub fn chr_ctrl(&mut self) -> SysResult<&mut ChrCtrl> {
        self.chr_ctrl
            .as_mut()
            .ok_or(SysError::null_pointer(PointerType::Player))
    }
    pub fn pointers(&self) -> Vec<(String, u64)> {
        self.chr_ctrl
            .as_ref()
            .map(|c| c.pointers())
            .unwrap_or_default()
    }
}

value_command!(Souls, u32 {
    get: {
        player()
            .chr_ctrl()?
            .get_ptr(ResolvedChrPtr::Stats)
            .add_offset(stats_offsets::SOULS)
            .read::<u32>()
    }

    set(val): {
        let souls_loc = player()
            .chr_ctrl()?
            .get_ptr(ResolvedChrPtr::Stats)
            .add_offset(stats_offsets::SOULS);
        let current = souls_loc.read::<i32>()?;
        let diff = val.min(999999999) as i32 - current;
        if diff < 0 {
            souls_loc.write::<i32>(current + diff)?;
        } else {
            give_souls(diff)?;
        }
        Ok(())
    }
});

fn give_souls(amount: i32) -> anyhow::Result<()> {
    let args = [
        FfiValue::pointer(player().chr_ctrl()?.get_ptr(ResolvedChrPtr::Stats)?),
        FfiValue::sint32(amount),
    ];

    run_game_function(Function::GiveSouls, &args, X86CallingConvention::__fastcall)
}

toggle_command!(NoDeath {
    is: {
        Ok(game_state::is_flag(StateFlag::PlayerNoDeath))
    }

    set(state): {
        game_state::set_flag(StateFlag::PlayerNoDeath, state)?;
        let _ = player().chr_ctrl().and_then(|chr| chr.set_no_death(state));
        Ok(())
    }
});

const VANILLA_NO_DAMAGE_ORIGINAL: [u8; 6] = [0x89, 0x8e, 0xfc, 0x00, 0x00, 0x00];
const SCHOLAR_NO_DAMAGE_ORIGINAL: [u8; 6] = [0x89, 0x83, 0x68, 0x01, 0x00, 0x00];
toggle_command!(NoDamage {
    is: {
        let bytes: &[u8] = match is_32() {
            true => &VANILLA_NO_DAMAGE_ORIGINAL,
            false => &SCHOLAR_NO_DAMAGE_ORIGINAL,
        };
        read::<[u8; 6]>(Hook::PlayerNoDamage).map(|val| val != bytes)
    }

    set(state): {
        let hook_loc = Hook::PlayerNoDamage;
        let cave_loc = CaveAddr::PlayerNoDamageHook;

        if state {
            let mut fun = asm_function("player_no_damage");

            fun.patch::<POINTER>("game_man_imp", BasePointer::GameManagerImp);
            fun.patch_rel32("hook_loc", cave_loc, hook_loc.add(6), 4);

            install_hook(&fun.bytes, cave_loc, hook_loc, 6)?;
        } else {
            let bytes: &[u8] = match is_32() {
                true => &VANILLA_NO_DAMAGE_ORIGINAL,
                false => &SCHOLAR_NO_DAMAGE_ORIGINAL,
            };
            write_bytes(hook_loc, bytes)?;
        }
        Ok(())
    }
});

const VANILLA_INFINITE_POISE_ORIGINAL: [u8; 7] = [0x83, 0xbb, 0xec, 0x05, 0x00, 0x00, 0x00];
const SCHOLAR_INFINITE_POISE_ORIGINAL: [u8; 6] = [0x39, 0x9d, 0xec, 0x05, 0x00, 0x00];
toggle_command!(InfinitePoise {
    is: {
        if is_32() {
            read::<[u8; 7]>(Hook::InfinitePoise).map(|val| val != VANILLA_INFINITE_POISE_ORIGINAL)
        } else {
            read::<[u8; 6]>(Hook::InfinitePoise).map(|val| val != SCHOLAR_INFINITE_POISE_ORIGINAL)
        }
    }

    set(state): {
        if state {
            let orig_instr_len = if is_32() { 7 } else { 6 };

            let mut fun = asm_function("infinite_poise_hook");

            fun.patch::<POINTER>("game_man_imp", BasePointer::GameManagerImp);
            fun.patch_rel32(
                "hook_loc",
                CaveAddr::InfinitePoiseHook,
                Hook::InfinitePoise.add(orig_instr_len),
                4,
            );

            install_hook(
                &fun.bytes,
                CaveAddr::InfinitePoiseHook,
                Hook::InfinitePoise,
                orig_instr_len,
            )?;
        } else {
            let bytes: &[u8] = match is_32() {
                true => &VANILLA_INFINITE_POISE_ORIGINAL,
                false => &SCHOLAR_INFINITE_POISE_ORIGINAL,
            };
            write_bytes(Hook::InfinitePoise, bytes)?;
        }
        Ok(())
    }
});

toggle_command!(InfiniteStamina {
    is: {
        read::<u8>(Patch::InfiniteStamina)
            .map(|val| val != 0x83)
    }

    set(state): {
        let byte = if state { 0x82 } else { 0x83 };
        Ok(write::<u8>(Patch::InfiniteStamina, byte)?)
    }
});

toggle_command!(InfiniteDurability {
    is: {
        read::<[u8; 5]>(Patch::InfiniteDurability)
            .map(|val| val == [0x90; 5])
    }

    set(state): {
        let bytes: &[u8] = match (state, is_32()) {
            (true, true) => &[0x90; 5],
            (true, false) => &[0x90; 9],
            (false, true) => &[0xf3, 0x0f, 0x11, 0x47, 0x6c],
            (false, false) => &[0xf3, 0x0f, 0x11, 0xb4, 0xc3, 0x94, 0x00, 0x00, 0x00],
        };
        Ok(write_bytes(Patch::InfiniteDurability, bytes)?)
    }
});

toggle_command!(InfiniteConsumables {
    is: {
        read::<[u8; 4]>(Patch::InfiniteConsumables)
            .map(|val| val == [0x90; 4])
    }

    set(state): {
        let bytes: &[u8] = match (state, is_32()) {
            (true, true) | (true, false) => &[0x90; 4],
            (false, true) => &[0x66, 0x29, 0x5e, 0x18],
            (false, false) => &[0x66, 0x29, 0x73, 0x20],
        };
        Ok(write_bytes(Patch::InfiniteConsumables, bytes)?)
    }
});

toggle_command!(NoHollowing {
    is: {
        read::<[u8; 6]>(Patch::NoHollowing)
            .map(|val| val == [0x90; 6])
    }

    set(state): {
        let bytes: &[u8] = match (state, is_32()) {
            (true, true) | (true, false) => &[0x90; 6],
            (false, true) => &[0x88, 0x91, 0xa8, 0x01, 0x00, 0x00],
            (false, false) => &[0x88, 0x81, 0xac, 0x01, 0x00, 0x00],
        };
        Ok(write_bytes(Patch::NoHollowing, bytes)?)
    }
});

toggle_command!(NoSoulLoss {
    is: {
        read::<[u8; 6]>(Patch::NoSoulLoss)
            .map(|val| val == [0x90; 6])
    }

    set(state): {
        let bytes: &[u8] = match (state, is_32()) {
            (true, true) => &[0x90; 10],
            (true, false) => &[0x90; 6],
            (false, true) => &[0xc7, 0x80, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            (false, false) => &[0x89, 0x90, 0xec, 0x00, 0x00, 0x00],
        };
        Ok(write_bytes(Patch::NoSoulLoss, bytes)?)
    }
});

toggle_command!(NoSoulGain {
    is: {
        read::<[u8; 5]>(Patch::NoSoulGain)
            .map(|val| val == [0x90; 5])
    }

    set(state): {
        let bytes: &[u8] = match (state, is_32()) {
            (true, true) | (true, false) => &[0x90; 5],
            (false, true) => &[0xe8, 0xf7, 0xf5, 0xff, 0xff],
            (false, false) => &[0xe8, 0x71, 0x01, 0x00, 0x00],
        };
        Ok(write_bytes(Patch::NoSoulGain, bytes)?)
    }
});

toggle_command!(Hidden {
    is: {
        read::<u8>(Patch::PlayerHidden).map(|val| val != 0x84)
    }

    set(state): {
        let byte = if state { 0x85 } else { 0x84 };
        Ok(write::<u8>(Patch::PlayerHidden, byte)?)
    }
});

toggle_command!(Silent {
    is: {
        read::<[u8; 5]>(Patch::PlayerSilent)
            .map(|val| val == [0x90; 5])
    }

    set(state): {
        match is_32() {
            true => {
                let push_op_neg_offset = match version() {
                    Some(DarkSouls2Version::Vanilla1_0_12) => 4,
                    _ => 1,
                };
                if state {
                    write_bytes(Patch::PlayerSilent, &[0x90; 15])?;
                    write::<u8>(Patch::PlayerSilent.sub(push_op_neg_offset), 0x90)?;
                } else {
                    let mut bytes = match version() {
                        Some(DarkSouls2Version::Vanilla1_0_12) => {
                            vec![
                                0xf3, 0x0f, 0x11, 0x04, 0x24, 0x51, 0x52, 0x53, 0x8b, 0xcf, 0xe8,
                                0x00, 0x00, 0x00, 0x00,
                            ]
                        }
                        _ => {
                            vec![
                                0xf3, 0x0f, 0x11, 0x04, 0x24, 0x52, 0x50, 0x53, 0x8b, 0xcf, 0xe8,
                                0x00, 0x00, 0x00, 0x00,
                            ]
                        }
                    };
                    write_rel_i32(&mut bytes, Patch::PlayerSilent, 11, Function::MakeSound, 4)?;
                    write_bytes(Patch::PlayerSilent, &bytes)?;
                    write::<u8>(Patch::PlayerSilent.sub(push_op_neg_offset), 0x51)?;
                }
            }
            false => {
                if state {
                    write_bytes(Patch::PlayerSilent, &[0x90; 5])?;
                } else {
                    let mut bytes = vec![0xe8; 5];
                    write_rel_i32(&mut bytes, Patch::PlayerSilent, 1, Function::MakeSound, 4)?;
                    write_bytes(Patch::PlayerSilent, &bytes)?;
                }
            }
        }
        Ok(())
    }
});

value_command!(Health, i32 {
    get: {
        player().chr_ctrl()?.get_hp()
    }

    set(val): {
        Ok(player().chr_ctrl()?.set_hp(val)?)
    }
});

unit_command!(RestoreHumanity {
    let speffect = speffect::RESTORE_HUMANITY;
    player().chr_ctrl()?.apply_speffect(speffect)
});

unit_command!(Rest {
    let speffect = speffect::REST;
    player().chr_ctrl()?.apply_speffect(speffect)
});

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Stats {
    pub vigor:        u16,
    pub endurance:    u16,
    pub vitality:     u16,
    pub attunement:   u16,
    pub strength:     u16,
    pub dexterity:    u16,
    pub intelligence: u16,
    pub faith:        u16,
    pub adaptability: u16,
}

impl Stats {
    pub fn read(&mut self) {
        let val = player()
            .chr_ctrl()
            .and_then(|chr| {
                chr.get_ptr(ResolvedChrPtr::Stats)
                    .add_offset(stats_offsets::STATS)
                    .read::<Self>()
            })
            .unwrap_or_default();

        *self = val
    }
}

pub(crate) static STATS: LazyLock<RwLock<Stats>> = LazyLock::new(|| RwLock::new(Stats::default()));

#[repr(u64)]
#[derive(Debug, Clone, Copy, Display)]
pub enum Stat {
    Vigor        = 0x0,
    Endurance    = 0x2,
    Vitality     = 0x4,
    Attunement   = 0x6,
    Strength     = 0x8,
    Dexterity    = 0xa,
    Intelligence = 0xc,
    Faith        = 0xe,
    Adaptability = 0x10,
}

impl StatCommand for Stat {
    fn get(&self) -> u32 {
        let s = STATS.read().unwrap();
        let val = match self {
            Self::Vigor => s.vigor,
            Self::Endurance => s.endurance,
            Self::Vitality => s.vitality,
            Self::Attunement => s.attunement,
            Self::Strength => s.strength,
            Self::Dexterity => s.dexterity,
            Self::Intelligence => s.intelligence,
            Self::Faith => s.faith,
            Self::Adaptability => s.adaptability,
        };
        val as u32
    }
    fn set(&self, val: u32) -> anyhow::Result<()> {
        player_loaded_check()?;

        let player_stats_entity = player().chr_ctrl()?.get_ptr(ResolvedChrPtr::Stats);

        let stats_base = player_stats_entity.add_offset(stats_offsets::STATS);

        let stat_loc = stats_base.map(|addr| addr + *self as u64);

        let new_stat = val.clamp(0, 99) as u16;
        let current_stat = stat_loc.read::<u16>()?;
        let num_levels = new_stat as i32 - current_stat as i32;

        stat_loc.write::<u16>(new_stat)?;

        let is_negative = num_levels <= 0;

        const NUM_LEVELS_SHORT: u64 = 0xe2;
        const NUM_LEVELS_INT: u64 = 0xe8;
        const CURRENT_LEVEL: u64 = 0xec;
        const NEW_LEVEL: u64 = 0xf0;
        const CURRENT_SOULS: u64 = 0xf4;
        const SOULS_AFTER: u64 = 0xf8;
        const REQUIRED_SOULS: u64 = 0xfc;

        let current_level = player_stats_entity
            .add_offset(stats_offsets::SOUL_LEVEL)
            .read::<i32>()?;

        let current_souls = player_stats_entity
            .add_offset(stats_offsets::SOULS)
            .read::<i32>()?;

        let stat_bytes = stats_base.read::<[u8; 22]>()?;

        let negative_flag_loc = CaveAddr::NegativeFlag;
        let buffer_loc = CaveAddr::LevelUpBuffer.addr();

        let mut buffer = [0x0; 0x100];

        write_to_slice::<[u8; 22]>(&mut buffer, 0, stat_bytes)?;
        write_to_slice::<i32>(&mut buffer, CURRENT_LEVEL, current_level)?;
        write_to_slice::<u16>(&mut buffer, NUM_LEVELS_SHORT, num_levels as u8)?;
        write_to_slice::<i32>(&mut buffer, NUM_LEVELS_INT, num_levels)?;
        write_to_slice::<i32>(&mut buffer, NEW_LEVEL, current_level + num_levels)?;
        write_to_slice::<i32>(&mut buffer, CURRENT_SOULS, current_souls)?;

        write_bytes(buffer_loc, &buffer)?;
        write::<u8>(negative_flag_loc, is_negative as u8)?;

        const NEGATIVE_LEVEL_PATCH: Offset = Offset {
            vanilla: 0x32,
            scholar: 0x39,
        };

        let negative_patch_loc = Function::LevelUp.add(NEGATIVE_LEVEL_PATCH.resolve());
        if is_negative {
            write::<u8>(negative_patch_loc, 0x85)?;
        }

        let mut fun = asm_function("level_up");

        fun.patch::<POINTER>("current_level", buffer_loc + CURRENT_LEVEL);
        fun.patch::<POINTER>("negative_flag", negative_flag_loc);
        fun.patch::<POINTER>("fn_level_lookup", Function::LevelLookup);
        fun.patch::<POINTER>("new_level", buffer_loc + NEW_LEVEL);
        fun.patch::<POINTER>("required_souls", buffer_loc + REQUIRED_SOULS);
        fun.patch::<POINTER>("current_souls", buffer_loc + CURRENT_SOULS);
        fun.patch::<POINTER>("stats_entity", player_stats_entity?);
        fun.patch::<POINTER>("fn_give_souls", Function::GiveSouls);
        fun.patch::<POINTER>("stats_entity", player_stats_entity?);
        fun.patch::<POINTER>("current_souls", buffer_loc + CURRENT_SOULS);
        fun.patch::<POINTER>("required_souls", buffer_loc + REQUIRED_SOULS);
        fun.patch::<POINTER>("souls_after", buffer_loc + SOULS_AFTER);
        fun.patch::<POINTER>("stats_entity", player_stats_entity?);
        fun.patch::<POINTER>("buffer", buffer_loc);
        fun.patch::<POINTER>("fn_level_up", Function::LevelUp);

        run_custom_function(fun)?;

        if is_negative {
            write::<u8>(negative_patch_loc, 0x84)?;
        }

        let new_souls = player_stats_entity
            .add_offset(stats_offsets::SOULS)
            .read::<i32>()?;

        give_souls(current_souls - new_souls)
    }
}

pub fn position() -> SysResult<[f32; 16]> {
    let pointer = follow_pointers(&game_manager_imp::player_coords_chain(), false)?;
    read::<[f32; 16]>(pointer)
}
