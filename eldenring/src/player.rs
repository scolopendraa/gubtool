pub use crate::offsets::{chr_dbg_flags::ChrDbgOffset, game_data_man::PlayerGameDataOffset};
use {
    crate::{
        chr_ins::{ChrIns, ResolvedChrPtr},
        emevd,
        game_state::{self, StateFlag},
        is_player_loaded,
        mem::*,
        offsets::{
            ChainReadExt,
            code_cave::CaveAddr,
            game_data_man,
            game_man,
            module_offsets::{BasePointer, Data, Function, Hook, Patch},
            world_chr_man,
        },
        pointer_cache::ResolvedPtr,
        resources::ASM,
        utils::{dlc_check, player_loaded_check},
    },
    assemble::patch::{BYTE, DWORD},
    gubtool_core::{
        address::{Address, POINTER},
        attached::version,
        game_version::EldenRingVersion::*,
        slice_ops::write_to_slice,
        sys::{
            ipc::FfiValue,
            sys_error::{PointerType, SysError, SysResult},
        },
    },
    shared::{
        command::{StatCommand, ToggleCommand, UnitCommand, ValueCommand},
        declare_command,
    },
    std::sync::{LazyLock, Mutex, MutexGuard},
    strum::Display,
};

static PLAYER: LazyLock<Mutex<Player>> = LazyLock::new(|| Mutex::new(Player::new()));

pub fn player() -> MutexGuard<'static, Player> {
    PLAYER.lock().unwrap()
}

pub struct Player {
    chr_ins: Option<ChrIns>,
}

impl Player {
    fn new() -> Self {
        let mut player = Self {
            chr_ins: None,
        };
        player.update();
        player
    }
    pub fn update(&mut self) {
        let ptr = ResolvedPtr::WorldChrMan
            .get()
            .read_offset(world_chr_man::player_ins());
        match ptr {
            Ok(pointer) => {
                if pointer != 0x0 {
                    self.chr_ins = Some(ChrIns::new(pointer));
                } else {
                    self.chr_ins = None;
                }
            }
            Err(_) => {
                self.chr_ins = None;
            }
        }
    }
    pub fn chr_ins(&mut self) -> SysResult<&mut ChrIns> {
        self.chr_ins
            .as_mut()
            .ok_or(SysError::null_pointer(PointerType::Player))
    }
    pub fn pointers(&self) -> Vec<(String, u64)> {
        self.chr_ins
            .as_ref()
            .map(|c| c.pointers())
            .unwrap_or_default()
    }
}

static TORRENT: LazyLock<Mutex<Torrent>> = LazyLock::new(|| Mutex::new(Torrent::new()));

pub fn torrent() -> MutexGuard<'static, Torrent> {
    TORRENT.lock().unwrap()
}

pub struct Torrent {
    chr_ins: Option<ChrIns>,
}

impl Torrent {
    fn new() -> Self {
        let mut torrent = Self {
            chr_ins: None,
        };
        torrent.update();
        torrent
    }
    pub fn update(&mut self) {
        let handle = ResolvedPtr::PlayerGameData
            .get()
            .read_offset(game_data_man::torrent_handle())
            .unwrap_or_default();

        self.chr_ins = ChrIns::from_handle(handle);
    }
    pub fn chr_ins(&mut self) -> SysResult<&mut ChrIns> {
        self.chr_ins
            .as_mut()
            .ok_or(SysError::null_pointer(PointerType::Torrent))
    }
    pub fn pointers(&self) -> Vec<(String, u64)> {
        self.chr_ins
            .as_ref()
            .map(|c| c.pointers())
            .unwrap_or_default()
    }
}

declare_command!(
    NoDeath,
    OneShot,
    InfiniteStamina,
    InfiniteFp,
    InfiniteConsumables,
    InfiniteArrows,
    Silent,
    Hidden,
    NoDamage,
    InfinitePoise,
    SetRfbsOnLoad,
    NoRuneLossOnDeath,
    NoTimePassOnDeath,
    RuneArc,
    TorrentAnywhere,
    TorrentNoDeath,
    Health,
    Die,
    Runes,
    AnimationSpeed,
    Rest,
);

fn is_chr_dbg_flag(offset: ChrDbgOffset) -> SysResult<bool> {
    read::<u8>(Data::ChrDbgFlags.add(offset as u64)).map(|val| val == 1)
}

fn set_chr_dbg_flag(offset: ChrDbgOffset, state: bool) -> anyhow::Result<()> {
    write::<u8>(Data::ChrDbgFlags.add(offset as u64), state as u8)?;
    Ok(())
}

macro_rules! impl_chr_dbg {
    ($struct_name:ident, $chr_dbg_offset:path) => {
        impl ToggleCommand for $struct_name {
            fn is(&self) -> SysResult<bool> {
                is_chr_dbg_flag($chr_dbg_offset)
            }
            fn set(&self, state: bool) -> anyhow::Result<()> {
                set_chr_dbg_flag($chr_dbg_offset, state)
            }
        }
    };
}

impl_chr_dbg!(NoDeath, ChrDbgOffset::PlayerNoDeath);
impl_chr_dbg!(OneShot, ChrDbgOffset::OneShot);
impl_chr_dbg!(InfiniteStamina, ChrDbgOffset::InfiniteStamina);
impl_chr_dbg!(InfiniteFp, ChrDbgOffset::InfiniteFp);
impl_chr_dbg!(InfiniteArrows, ChrDbgOffset::InfiniteArrows);
impl_chr_dbg!(InfiniteConsumables, ChrDbgOffset::InfiniteConsumables);
impl_chr_dbg!(Hidden, ChrDbgOffset::Hidden);
impl_chr_dbg!(Silent, ChrDbgOffset::Silent);

impl ToggleCommand for NoDamage {
    fn is(&self) -> SysResult<bool> {
        Ok(game_state::is_flag(StateFlag::PlayerNoDamage))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::PlayerNoDamage, state)?;
        let _ = self.set_in_game(state);
        Ok(())
    }
}
impl NoDamage {
    pub fn set_in_game(&self, state: bool) -> SysResult {
        player().chr_ins()?.set_no_damage(state)
    }
}

impl ValueCommand<i32> for Health {
    fn get(&self) -> SysResult<i32> {
        player().chr_ins()?.get_current_hp()
    }
    fn set(&self, val: i32) -> anyhow::Result<()> {
        player().chr_ins()?.set_hp(val)?;
        Ok(())
    }
}

impl UnitCommand for Die {
    fn execute(&self) -> anyhow::Result<()> {
        player().chr_ins()?.set_hp(0)?;
        Ok(())
    }
}

impl ValueCommand<f32> for AnimationSpeed {
    fn get(&self) -> SysResult<f32> {
        player().chr_ins()?.get_animation_speed()
    }
    fn set(&self, val: f32) -> anyhow::Result<()> {
        player().chr_ins()?.set_animation_speed(val)?;
        Ok(())
    }
}

impl UnitCommand for Rest {
    fn execute(&self) -> anyhow::Result<()> {
        emevd::rest()
    }
}

impl ToggleCommand for RuneArc {
    fn is(&self) -> SysResult<bool> {
        Ok(player_game_data().rune_arc_active || game_state::is_flag(StateFlag::RuneArc))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::RuneArc, state)?;
        let _ = self.set_in_game(state);
        Ok(())
    }
}
impl RuneArc {
    pub fn set_in_game(&self, state: bool) -> SysResult {
        ResolvedPtr::PlayerGameData
            .get()
            .add_offset(PlayerGameDataOffset::RuneArc as u64)
            .write::<u8>(state as u8)
    }
}

impl ToggleCommand for SetRfbsOnLoad {
    fn is(&self) -> SysResult<bool> {
        Ok(game_state::is_flag(StateFlag::Rfbs))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::Rfbs, state)?;
        Ok(())
    }
}
impl SetRfbsOnLoad {
    pub fn apply_in_game(&self) -> SysResult {
        let max_hp = player().chr_ins()?.get_max_hp()?;
        player().chr_ins()?.set_hp((max_hp * 20) / 100 - 1)
    }
}

impl ValueCommand<u32> for Runes {
    fn get(&self) -> SysResult<u32> {
        Ok(player_game_data().rune_count)
    }
    fn set(&self, val: u32) -> anyhow::Result<()> {
        let current_amount = player_game_data().rune_count;
        let to_give = val as i32 - current_amount as i32;
        self.give(to_give as i64)
    }
}
impl Runes {
    pub fn give(&self, amount: i64) -> anyhow::Result<()> {
        player_loaded_check()?;

        let args = [
            FfiValue::pointer(ResolvedPtr::PlayerGameData.get()?),
            FfiValue::sint64(amount),
        ];

        run_game_function(Function::GiveRunes, &args)
    }
}

impl ToggleCommand for TorrentAnywhere {
    fn is(&self) -> SysResult<bool> {
        read::<[u8; 3]>(Patch::WhistleDisabled).map(|val| val != [0x0f, 0x95, 0xc0])
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        match state {
            true => {
                write_bytes(Patch::TorrentDisabledUnderworld, &[0x30, 0xc0, 0x90])?;
                write_bytes(Patch::WhistleDisabled, &[0x30, 0xc0, 0x90])?;
            }
            false => {
                write_bytes(Patch::TorrentDisabledUnderworld, &[0x0f, 0x95, 0xc0])?;
                write_bytes(Patch::WhistleDisabled, &[0x0f, 0x95, 0xc0])?;
            }
        }
        Ok(())
    }
}

impl ToggleCommand for NoRuneLossOnDeath {
    fn is(&self) -> SysResult<bool> {
        read::<u8>(Patch::NoRuneLossOnDeath).map(|val| val != 0xf)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        let orig_bytes = match version() {
            Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) | Some(Version1_2_3)
            | Some(Version1_3_0) | Some(Version1_3_1) | Some(Version1_3_2) | Some(Version1_4_0)
            | Some(Version1_4_1) | Some(Version1_5_0) | Some(Version1_6_0) | Some(Version1_7_0)
            | Some(Version1_8_0) | Some(Version1_8_1) | Some(Version1_9_0) | Some(Version1_9_1)
            | Some(Version2_0_0) | Some(Version2_0_1) => [0xf, 0x84, 0xdc, 0x1, 0x0, 0x0],
            _ => [0xf, 0x84, 0xe4, 0x1, 0x0, 0x0],
        };
        match state {
            true => {
                let mut bytes = [0xe9, 0x0, 0x0, 0x0, 0x0, 0x90];
                let orig_jump = i32::from_le_bytes([
                    orig_bytes[2],
                    orig_bytes[3],
                    orig_bytes[4],
                    orig_bytes[5],
                ]);
                write_to_slice::<i32>(&mut bytes, 1, orig_jump + 1)?;
                write_bytes(Patch::NoRuneLossOnDeath, &bytes)?
            }
            false => write_bytes(Patch::NoRuneLossOnDeath, &orig_bytes)?,
        }
        Ok(())
    }
}

const NO_TIME_PASS_ORIGINAL: [u8; 5] = [0x4c, 0x8b, 0x74, 0x24, 0x70];
impl ToggleCommand for NoTimePassOnDeath {
    fn is(&self) -> SysResult<bool> {
        read::<[u8; 5]>(Hook::NoTimePassOnDeath).map(|val| val != NO_TIME_PASS_ORIGINAL)
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        match state {
            true => {
                let mut fun = ASM.get_function("no_time_pass_on_death");

                fun.patch::<POINTER>("world_area_time_impl", BasePointer::WorldAreaTimeImpl);
                fun.patch::<POINTER>("game_man", BasePointer::GameMan);
                fun.patch::<BYTE>("stored_time_off", game_man::stored_time() as u8);
                fun.patch_rel32(
                    "hook_loc",
                    CaveAddr::NoTimePassOnDeathHook,
                    Hook::NoTimePassOnDeath.add(5),
                    4,
                );

                install_hook(
                    &fun.bytes,
                    CaveAddr::NoTimePassOnDeathHook,
                    Hook::NoTimePassOnDeath,
                    5,
                )?
            }
            false => write::<[u8; 5]>(Hook::NoTimePassOnDeath, NO_TIME_PASS_ORIGINAL)?,
        }
        Ok(())
    }
}

impl ToggleCommand for TorrentNoDeath {
    fn is(&self) -> SysResult<bool> {
        Ok(game_state::is_flag(StateFlag::TorrentNoDeath))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::TorrentNoDeath, state)?;
        let _ = self.set_in_game(state);
        Ok(())
    }
}
impl TorrentNoDeath {
    pub fn set_in_game(&self, state: bool) -> SysResult {
        torrent().chr_ins()?.set_no_death(state)
    }
}

impl ToggleCommand for InfinitePoise {
    fn is(&self) -> SysResult<bool> {
        read::<[u8; 7]>(Hook::PlayerInfinitePoise).map(|val| val != infinite_poise_bytes_original())
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        self.set_infinite_poise(state)?;
        self.set_no_grab(state)?;
        Ok(())
    }
}

const GRAB_HOOK_BYTES_ORIGINAL: [u8; 9] = [0x41, 0x8b, 0x56, 0x44, 0x48, 0x8d, 0x4c, 0x24, 0x40];
fn infinite_poise_bytes_original() -> [u8; 7] {
    match version() {
        Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) | Some(Version1_2_3) => {
            [0x4c, 0x8b, 0xc7, 0x41, 0x0f, 0xb6, 0xd6]
        }
        _ => [0x4c, 0x8b, 0xc7, 0x40, 0x0f, 0xb6, 0xd5],
    }
}
impl InfinitePoise {
    fn set_infinite_poise(&self, state: bool) -> SysResult {
        if state {
            let mut fun = ASM.get_function("infinite_poise_hook");

            fun.patch::<POINTER>("world_chr_man", BasePointer::WorldChrMan);
            fun.patch::<DWORD>("player_ins_off", world_chr_man::player_ins() as u32);
            fun.patch::<POINTER>("fn_get_chr_ins", Function::GetChrInsByEntityId);
            fun.patch_rel32(
                "hook_loc",
                CaveAddr::InfinitePoiseHook,
                Hook::PlayerInfinitePoise.add(7),
                4,
            );

            install_hook(&fun.bytes, CaveAddr::InfinitePoiseHook, Hook::PlayerInfinitePoise, 7)
        } else {
            write_bytes(Hook::PlayerInfinitePoise, &infinite_poise_bytes_original())
        }
    }
    fn set_no_grab(&self, state: bool) -> SysResult {
        if state {
            let mut fun = ASM.get_function("grab_hook");

            let location = CaveAddr::NoGrabHook;
            let skip_grab_jmp_location = Hook::PlayerNoGrab.add(0x95);

            fun.patch::<POINTER>("world_chr_man", BasePointer::WorldChrMan);
            fun.patch::<DWORD>("player_ins_off", world_chr_man::player_ins() as u32);
            fun.patch_rel32("skip_grab_jmp_location", location, skip_grab_jmp_location, 4);
            fun.patch_rel32("hook_loc", location, Hook::PlayerNoGrab.add(9), 4);

            install_hook(&fun.bytes, location, Hook::PlayerNoGrab, 9)
        } else {
            write_bytes(Hook::PlayerNoGrab, &GRAB_HOOK_BYTES_ORIGINAL)
        }
    }
}

pub fn map_coords() -> SysResult<[f32; 3]> {
    player()
        .chr_ins()?
        .get_ptr(ResolvedChrPtr::ChrIns)
        .add_offset(world_chr_man::player_ins_offsets::current_map_coords())
        .read::<[f32; 3]>()
}

pub fn map_angle() -> SysResult<f32> {
    player()
        .chr_ins()?
        .get_ptr(ResolvedChrPtr::ChrIns)
        .add_offset(world_chr_man::player_ins_offsets::current_map_angle())
        .read::<f32>()
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct PlayerGameData {
    vftable:                              usize,
    pub character_event_id:               u32,
    pub player_id:                        u32,
    pub current_hp:                       u32,
    pub current_max_hp:                   u32,
    pub base_max_hp:                      u32,
    pub current_fp:                       u32,
    pub current_max_fp:                   u32,
    pub base_max_fp:                      u32,
    unk28:                                f32,
    pub current_stamina:                  u32,
    pub current_max_stamina:              u32,
    pub base_max_stamina:                 u32,
    unk38:                                f32,
    pub vigor:                            u32,
    pub mind:                             u32,
    pub endurance:                        u32,
    pub strength:                         u32,
    pub dexterity:                        u32,
    pub intelligence:                     u32,
    pub faith:                            u32,
    pub arcane:                           u32,
    pub base_hero_point:                  f32,
    pub base_hero_point_2:                f32,
    pub base_durability:                  f32,
    pub level:                            u32,
    pub rune_count:                       u32,
    pub rune_memory:                      u32,
    unk74:                                u32,
    pub poison_resist:                    u32,
    pub rot_resist:                       u32,
    pub bleed_resist:                     u32,
    pub death_resist:                     u32,
    pub frost_resist:                     u32,
    pub sleep_resist:                     u32,
    pub madness_resist:                   u32,
    pub pending_block_clear_bonus:        f32,
    pub chr_type:                         i32,
    character_name:                       [u16; 17],
    pub gender:                           u8,
    pub archetype:                        u8,
    pub vow_type:                         u8,
    unkc1:                                u8,
    pub voice_type:                       u8,
    pub starting_gift:                    u8,
    unkc4:                                u8,
    pub unlocked_magic_slots:             u8,
    pub unlocked_talisman_slots:          u8,
    pub matchmaking_spirit_ashes_level:   u8,
    pub total_summon_count:               u32,
    pub coop_success_count:               u32,
    pub game_data_man_index:              u32,
    unkd4:                                [u8; 0xb],
    pub furlcalling_finger_remedy_active: bool,
    unke0:                                u8,
    unke1:                                u8,
    pub matching_weapon_level:            u8,
    pub white_ring_active:                u8,
    pub blue_ring_active:                 u8,
    pub multiplay_role:                   u8,
    unke6:                                u8,
    pub is_my_world:                      bool,
    unke8:                                [u8; 0x3],
    unke9:                                bool,
    pub character_id:                     u32,
    pub invasions_success_count:          u32,
    pub solo_breakin_point:               u32,
    pub invaders_killed:                  u32,
    pub scadutree_blessing:               u8,
    pub revered_spirit_ash:               u8,
    pub resist_curse_item_count:          u8,
    pub rune_arc_active:                  bool,
    unk100:                               bool,
    pub max_hp_flask:                     u8,
    pub max_fp_flask:                     u8,
}

static PLAYER_GAME_DATA: LazyLock<Mutex<PlayerGameData>> =
    LazyLock::new(|| Mutex::new(PlayerGameData::default()));

pub fn player_game_data() -> MutexGuard<'static, PlayerGameData> {
    PLAYER_GAME_DATA.lock().unwrap()
}

impl PlayerGameData {
    pub fn read(&mut self) {
        if is_player_loaded() {
            let new = ResolvedPtr::PlayerGameData
                .get()
                .read::<Self>()
                .unwrap_or_default();
            *self = new
        } else {
            *self = Self::default()
        }
    }
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, Display)]
#[strum(serialize_all = "title_case")]
pub enum Stat {
    Vigor             = 0x3c,
    Mind              = 0x40,
    Endurance         = 0x44,
    Strength          = 0x48,
    Dexterity         = 0x4c,
    Intelligence      = 0x50,
    Faith             = 0x54,
    Arcane            = 0x58,
    ScadutreeBlessing = 0xfc,
    ReveredSpiritAsh  = 0xfd,
}

impl StatCommand for Stat {
    fn get(&self) -> u32 {
        let s = player_game_data();
        match self {
            Self::Vigor => s.vigor,
            Self::Mind => s.mind,
            Self::Endurance => s.endurance,
            Self::Strength => s.strength,
            Self::Dexterity => s.dexterity,
            Self::Intelligence => s.intelligence,
            Self::Faith => s.faith,
            Self::Arcane => s.arcane,
            Self::ScadutreeBlessing => s.scadutree_blessing as u32,
            Self::ReveredSpiritAsh => s.revered_spirit_ash as u32,
        }
    }
    fn set(&self, val: u32) -> anyhow::Result<()> {
        set_stat(*self, val as i32)
    }
}

fn set_stat(stat: Stat, val: i32) -> anyhow::Result<()> {
    player_loaded_check()?;

    match stat {
        Stat::ScadutreeBlessing | Stat::ReveredSpiritAsh => {
            dlc_check()?;

            ResolvedPtr::PlayerGameData
                .get()
                .add_offset(stat as u64)
                .write::<u8>((val as u8).clamp(0, 20))?;
        }
        _ => {
            let val = val.clamp(0, 99);

            let game_data = ResolvedPtr::PlayerGameData.get()?;
            let current_val = read::<i32>(game_data + stat as u64)?;

            let diff = val - current_val;
            let current_level = read::<i32>(game_data + PlayerGameDataOffset::RuneLevel as u64)?;

            if val > current_val {
                let mut rune_cost = 0;
                for i in 1..=diff {
                    rune_cost += level_up_cost(current_level + i);
                }
                let current_rune_mem =
                    read::<u32>(game_data + PlayerGameDataOffset::RuneMemory as u64)?;
                let new_rune_mem =
                    std::cmp::min(current_rune_mem as u64 + rune_cost as u64, 0xffffffff);
                write::<u32>(
                    game_data + PlayerGameDataOffset::RuneMemory as u64,
                    new_rune_mem as u32,
                )?;
            }
            write::<i32>(game_data + PlayerGameDataOffset::RuneLevel as u64, current_level + diff)?;
            write::<i32>(game_data + stat as u64, val)?;
        }
    }
    Ok(())
}

fn level_up_cost(next_level: i32) -> i32 {
    let base_level_offset = 80.0_f32;
    let initial_level_up_cost = 0.1_f32;
    let initial_level_up_offset = 1.0_f32;
    let level_up_cost_increase = 0.02_f32;
    let level_up_increase_interval = 92.0_f32;

    let base_level = next_level as f32 + base_level_offset;
    let adjusted_level = 0.0_f32.max(base_level - level_up_increase_interval);
    let cost =
        base_level * base_level * (level_up_cost_increase * adjusted_level + initial_level_up_cost)
            + initial_level_up_offset;
    cost as i32
}
