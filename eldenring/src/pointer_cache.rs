use {
    crate::{
        mem::read,
        offsets::{ChainReadExt, game_data_man, module_offsets::BasePointer},
    },
    gubtool_core::sys::sys_error::SysResult,
    std::{
        collections::HashMap,
        sync::{LazyLock, Mutex},
    },
    strum::{EnumIter, IntoEnumIterator},
};

pub(crate) static POINTER_CACHE: LazyLock<PointerCache> = LazyLock::new(PointerCache::default);

#[derive(Default)]
pub struct PointerCache {
    map: Mutex<HashMap<ResolvedPtr, u64>>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, EnumIter)]
pub(crate) enum ResolvedPtr {
    WorldChrMan,
    FieldArea,
    GameMan,
    GameDataMan,
    PlayerGameData,
    MenuMan,
    CsEmkSystem,
    VirtualMemFlag,
    DamageManager,
    MapItemManImpl,
    DlUserInputManagerImpl,
    CsFlipperImp,
    CsDlcImp,
    LockTgtMan,
    LuaEventSystem,
}

impl PointerCache {
    pub fn lookup(&self, pointer: ResolvedPtr) -> SysResult<u64> {
        {
            let cache = self.map.lock().unwrap();
            if let Some(&val) = cache.get(&pointer) {
                return Ok(val);
            }
        }

        let resolved_pointer = match pointer {
            ResolvedPtr::WorldChrMan => read::<u64>(BasePointer::WorldChrMan),
            ResolvedPtr::FieldArea => read::<u64>(BasePointer::FieldArea),
            ResolvedPtr::GameMan => read::<u64>(BasePointer::GameMan),
            ResolvedPtr::GameDataMan => read::<u64>(BasePointer::GameDataMan),
            ResolvedPtr::PlayerGameData => {
                self.lookup(ResolvedPtr::GameDataMan)
                    .read_offset(game_data_man::PLAYER_GAME_DATA)
            }
            ResolvedPtr::MenuMan => read::<u64>(BasePointer::MenuMan),
            ResolvedPtr::CsEmkSystem => read::<u64>(BasePointer::CsEmkSystem),
            ResolvedPtr::VirtualMemFlag => read::<u64>(BasePointer::VirtualMemFlag),
            ResolvedPtr::DamageManager => read::<u64>(BasePointer::DamageManager),
            ResolvedPtr::MapItemManImpl => read::<u64>(BasePointer::MapItemManImpl),
            ResolvedPtr::DlUserInputManagerImpl => read::<u64>(BasePointer::DlUserInputManagerImpl),
            ResolvedPtr::CsFlipperImp => read::<u64>(BasePointer::CsFlipperImp),
            ResolvedPtr::CsDlcImp => read::<u64>(BasePointer::CsDlcImp),
            ResolvedPtr::LockTgtMan => read::<u64>(BasePointer::LockTgtManImp),
            ResolvedPtr::LuaEventSystem => read::<u64>(BasePointer::LuaEventSystem),
        }?;

        let mut cache = self.map.lock().unwrap();
        cache.insert(pointer, resolved_pointer);
        Ok(resolved_pointer)
    }

    pub fn reset_pointers(&self) {
        let mut cache = self.map.lock().unwrap();
        cache.clear();
    }
}

impl ResolvedPtr {
    pub fn get(self) -> SysResult<u64> {
        POINTER_CACHE.lookup(self)
    }
}

pub fn load_all_pointers() {
    POINTER_CACHE.reset_pointers();
    for ptr in ResolvedPtr::iter() {
        let _ = POINTER_CACHE.lookup(ptr);
    }
}

pub fn get_pointers() -> Vec<(String, u64)> {
    let map = POINTER_CACHE.map.lock().unwrap();
    map.iter()
        .map(|(name, addr)| (format!("{:?}", name), *addr))
        .collect()
}
