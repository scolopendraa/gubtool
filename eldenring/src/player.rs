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
            code_cave::CaveAddress,
            game_data_man,
            module_offsets::{BasePointer, Data, Function, Hook, Patch},
            world_chr_man,
        },
        pointer_cache::ResolvedPtr,
        resources::ASM,
        utils::{dlc_check, player_loaded_check},
    },
    gubtool_core::{
        address::Address,
        attached::version,
        game_version::EldenRingVersion::*,
        slice_ops::*,
        sys::{
            ipc::CppValue,
            sys_error::{PointerType, ProcResult, ProcessError},
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
    pub fn chr_ins(&mut self) -> ProcResult<&mut ChrIns> {
        self.chr_ins
            .as_mut()
            .ok_or(ProcessError::null_pointer(PointerType::Player))
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
    pub fn chr_ins(&mut self) -> ProcResult<&mut ChrIns> {
        self.chr_ins
            .as_mut()
            .ok_or(ProcessError::null_pointer(PointerType::Torrent))
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
    RuneArc,
    TorrentAnywhere,
    TorrentNoDeath,
    Health,
    Die,
    Runes,
    AnimationSpeed,
    Rest,
);

fn is_chr_dbg_flag(offset: ChrDbgOffset) -> ProcResult<bool> {
    read::<u8>(Data::ChrDbgFlags.add_offset(offset as u64)).map(|val| val == 1)
}

fn set_chr_dbg_flag(offset: ChrDbgOffset, state: bool) -> anyhow::Result<()> {
    write::<u8>(Data::ChrDbgFlags.add_offset(offset as u64), state as u8)?;
    Ok(())
}

macro_rules! impl_chr_dbg {
    ($struct_name:ident, $chr_dbg_offset:path) => {
        impl ToggleCommand for $struct_name {
            fn is(&self) -> ProcResult<bool> {
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
    fn is(&self) -> ProcResult<bool> {
        Ok(game_state::is_flag(StateFlag::PlayerNoDamage))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::PlayerNoDamage, state)?;
        let _ = self.set_in_game(state);
        Ok(())
    }
}
impl NoDamage {
    pub fn set_in_game(&self, state: bool) -> ProcResult {
        player().chr_ins()?.set_no_damage(state)
    }
}

impl ValueCommand<i32> for Health {
    fn get(&self) -> ProcResult<i32> {
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
    fn get(&self) -> ProcResult<f32> {
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
    fn is(&self) -> ProcResult<bool> {
        Ok(player_game_data().rune_arc_active || game_state::is_flag(StateFlag::RuneArc))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::RuneArc, state)?;
        let _ = self.set_in_game(state);
        Ok(())
    }
}
impl RuneArc {
    pub fn set_in_game(&self, state: bool) -> ProcResult {
        ResolvedPtr::PlayerGameData
            .get()
            .add_offset(PlayerGameDataOffset::RuneArc as u64)
            .write::<u8>(state as u8)
    }
}

impl ToggleCommand for SetRfbsOnLoad {
    fn is(&self) -> ProcResult<bool> {
        Ok(game_state::is_flag(StateFlag::Rfbs))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::Rfbs, state)?;
        Ok(())
    }
}
impl SetRfbsOnLoad {
    pub fn apply_in_game(&self) -> ProcResult {
        let max_hp = player().chr_ins()?.get_max_hp()?;
        player().chr_ins()?.set_hp((max_hp * 20) / 100 - 1)
    }
}

impl ValueCommand<u32> for Runes {
    fn get(&self) -> ProcResult<u32> {
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
            CppValue::uintptr_t(ResolvedPtr::PlayerGameData.get()?),
            CppValue::int64_t(amount),
        ];

        run_game_function(Function::GiveRunes, &args)
    }
}

impl ToggleCommand for TorrentAnywhere {
    fn is(&self) -> ProcResult<bool> {
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

impl ToggleCommand for TorrentNoDeath {
    fn is(&self) -> ProcResult<bool> {
        Ok(game_state::is_flag(StateFlag::TorrentNoDeath))
    }
    fn set(&self, state: bool) -> anyhow::Result<()> {
        game_state::set_flag(StateFlag::TorrentNoDeath, state)?;
        let _ = self.set_in_game(state);
        Ok(())
    }
}
impl TorrentNoDeath {
    pub fn set_in_game(&self, state: bool) -> ProcResult {
        torrent().chr_ins()?.set_no_death(state)
    }
}

impl ToggleCommand for InfinitePoise {
    fn is(&self) -> ProcResult<bool> {
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
    fn set_infinite_poise(&self, state: bool) -> ProcResult {
        if state {
            let mut fun = ASM.get_function("infinite_poise_hook");
            let mut asm = fun.take_bytes();

            write_addr_to_slice(&mut asm, fun.reloc("world_chr_man"), BasePointer::WorldChrMan)?;
            write_to_slice::<i32>(
                &mut asm,
                fun.reloc("player_ins_off"),
                world_chr_man::player_ins(),
            )?;
            write_addr_to_slice(
                &mut asm,
                fun.reloc("fn_get_chr_ins"),
                Function::GetChrInsByEntityId,
            )?;
            write_rel_i32(
                &mut asm,
                CaveAddress::InfinitePoiseHook,
                fun.reloc("hook_loc"),
                Hook::PlayerInfinitePoise.add_offset(7),
                4,
            )?;
            install_hook(&asm, CaveAddress::InfinitePoiseHook, Hook::PlayerInfinitePoise, 7)
        } else {
            write_bytes(Hook::PlayerInfinitePoise, &infinite_poise_bytes_original())
        }
    }
    fn set_no_grab(&self, state: bool) -> ProcResult {
        if state {
            let mut fun = ASM.get_function("grab_hook");
            let mut asm = fun.take_bytes();

            let location = CaveAddress::NoGrabHook;
            let skip_grab_jmp_location = Hook::PlayerNoGrab.add_offset(0x95);

            write_addr_to_slice(&mut asm, fun.reloc("world_chr_man"), BasePointer::WorldChrMan)?;
            write_to_slice::<i32>(
                &mut asm,
                fun.reloc("player_ins_off"),
                world_chr_man::player_ins(),
            )?;
            write_rel_i32(
                &mut asm,
                location,
                fun.reloc("skip_grab_jmp_location"),
                skip_grab_jmp_location,
                4,
            )?;
            write_rel_i32(
                &mut asm,
                location,
                fun.reloc("hook_loc"),
                Hook::PlayerNoGrab.add_offset(9),
                4,
            )?;

            install_hook(&asm, location, Hook::PlayerNoGrab, 9)
        } else {
            write_bytes(Hook::PlayerNoGrab, &GRAB_HOOK_BYTES_ORIGINAL)
        }
    }
}

pub fn map_coords() -> ProcResult<[f32; 3]> {
    player()
        .chr_ins()?
        .get_ptr(ResolvedChrPtr::ChrIns)
        .add_offset(world_chr_man::player_ins_offsets::current_map_coords())
        .read::<[f32; 3]>()
}

pub fn map_angle() -> ProcResult<f32> {
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
            let bytes = ResolvedPtr::PlayerGameData
                .get()
                .read::<[u8; std::mem::size_of::<Self>()]>()
                .unwrap_or([0x0; std::mem::size_of::<Self>()]);
            *self = unsafe { *(bytes.as_ptr() as *const Self) }
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
