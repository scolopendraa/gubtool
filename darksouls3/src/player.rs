use {
    crate::{is_player_loaded, offsets::module_offsets::Data, pointer_cache::ResolvedPtr},
    gubtool_core::{
        address::{Address, ChainReadExt},
        sys::{read, sys_error::SysResult, write},
    },
    shared::toggle_command,
    std::sync::{LazyLock, Mutex, MutexGuard},
};

#[repr(u64)]
enum DebugFlag {
    NoDeath         = 0x0,
    OneShot         = 0x1,
    InfiniteStamina = 0x2,
    InfiniteFp      = 0x3,
    InfiniteArrows  = 0x4,
    Hidden          = 0x6,
    Silent          = 0x7,
    AllNoDeath      = 0x8,
    AllNoDamage     = 0x9,
    AllNoAttack     = 0xb,
    AllNoMove       = 0xc,
    AllNoUpdate     = 0xd,
}

fn is_dbg_flag(offset: DebugFlag) -> SysResult<bool> {
    read::<u8>(Data::DebugFlags.ptr() + offset as u64).map(|val| val == 1)
}

fn set_dbg_flag(offset: DebugFlag, state: bool) -> anyhow::Result<()> {
    Ok(write::<u8>(Data::DebugFlags.ptr() + offset as u64, state as u8)?)
}

macro_rules! impl_debug_command {
    ($struct_name:ident, $chr_dbg_offset:path) => {
        toggle_command!($struct_name {
            is: {
                is_dbg_flag($chr_dbg_offset)
            }

            set(state): {
                set_dbg_flag($chr_dbg_offset, state)
            }
        });
    };
}

impl_debug_command!(NoDeath, DebugFlag::NoDeath);
impl_debug_command!(OneShot, DebugFlag::OneShot);
impl_debug_command!(InfiniteStamina, DebugFlag::InfiniteStamina);
impl_debug_command!(InfiniteFp, DebugFlag::InfiniteFp);
impl_debug_command!(InfiniteArrows, DebugFlag::InfiniteArrows);
impl_debug_command!(Hidden, DebugFlag::Hidden);
impl_debug_command!(Silent, DebugFlag::Silent);
impl_debug_command!(AllNoDeath, DebugFlag::AllNoDeath);
impl_debug_command!(AllNoDamage, DebugFlag::AllNoDamage);
impl_debug_command!(AllNoAttack, DebugFlag::AllNoAttack);
impl_debug_command!(AllNoMove, DebugFlag::AllNoMove);
impl_debug_command!(AllNoUpdate, DebugFlag::AllNoUpdate);

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

#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct PlayerGameData {
    _vftable:             u64,
    _unk08:               u64,
    pub id:               u32,
    _unk14:               u32,
    pub hp:               u32,
    pub max_hp:           u32,
    pub base_max_hp:      u32,
    pub mp:               u32,
    pub max_mp:           u32,
    pub base_max_mp:      u32,
    _unk30:               u32,
    pub stamina:          u32,
    pub max_stamina:      u32,
    pub base_max_stamina: u32,
    _unk40:               u32,
    pub vigor:            u32,
    pub attunement:       u32,
    pub endurance:        u32,
    pub strength:         u32,
    pub dexterity:        u32,
    pub intelligence:     u32,
    pub faith:            u32,
    pub luck:             u32,
    _unk64:               u32,
    _unk68:               u32,
    pub vitality:         u32,
    pub soul_level:       u32,
    pub souls:            u32,
    pub total_souls:      u64,
    _unk80:               u64,
    pub character_name:   [u16; 17],
    _unkaa:               [u8; 0x16],
    _unkc0:               [u8; 0x20],
    _unke0:               [u8; 0x20],
    pub ember:            u8,
}

// toggle_command!(Ember {
//     is: {
//         Ok(player_game_data().ember != 0)
//     }
//     set(state): {
//         ResolvedPtr::PlayerGameData
//             .get()
//             .add_offset(0x100)
//             .write::<u8>(state as u8)?;
//         Ok(())
//     }
// });
