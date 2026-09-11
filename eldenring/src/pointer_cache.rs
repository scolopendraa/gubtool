use {
    crate::{
        obj_refs,
        offsets::{game_data_man, module_offsets::BasePointer},
    },
    gubtool_core::{
        address::{ChainReadExt, Pointer},
        sys::{read_pointer, sys_error::SysResult},
    },
    shared::pointer_cache::{PointerCache, PointerKey},
    std::sync::{LazyLock, Mutex, MutexGuard},
    strum::EnumIter,
};

pub(crate) static POINTER_CACHE: LazyLock<Mutex<PointerCache<ResolvedPtr>>> =
    LazyLock::new(|| Mutex::new(PointerCache::new()));

pub fn pointer_cache() -> MutexGuard<'static, PointerCache<ResolvedPtr>> {
    POINTER_CACHE.lock().unwrap()
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

impl PointerKey for ResolvedPtr {
    fn lookup(
        cache: &mut PointerCache<Self>,
        key: Self,
        _base: Option<Pointer>,
    ) -> SysResult<Pointer>
    where
        Self: Sized,
    {
        match key {
            ResolvedPtr::WorldChrMan => read_pointer(BasePointer::WorldChrMan),
            ResolvedPtr::FieldArea => read_pointer(BasePointer::FieldArea),
            ResolvedPtr::GameMan => read_pointer(BasePointer::GameMan),
            ResolvedPtr::GameDataMan => read_pointer(BasePointer::GameDataMan),
            ResolvedPtr::PlayerGameData => {
                cache
                    .lookup(ResolvedPtr::GameDataMan)
                    .read_ptr(game_data_man::PLAYER_GAME_DATA)
            }
            ResolvedPtr::MenuMan => read_pointer(BasePointer::MenuMan),
            ResolvedPtr::CsEmkSystem => read_pointer(BasePointer::CsEmkSystem),
            ResolvedPtr::VirtualMemFlag => read_pointer(BasePointer::VirtualMemFlag),
            ResolvedPtr::DamageManager => read_pointer(BasePointer::DamageManager),
            ResolvedPtr::MapItemManImpl => read_pointer(BasePointer::MapItemManImpl),
            ResolvedPtr::DlUserInputManagerImpl => {
                read_pointer(BasePointer::DlUserInputManagerImpl)
            }
            ResolvedPtr::CsFlipperImp => read_pointer(BasePointer::CsFlipperImp),
            ResolvedPtr::CsDlcImp => read_pointer(BasePointer::CsDlcImp),
            ResolvedPtr::LockTgtMan => read_pointer(BasePointer::LockTgtManImp),
            ResolvedPtr::LuaEventSystem => read_pointer(BasePointer::LuaEventSystem),
        }
    }
}

impl ResolvedPtr {
    pub fn get(self) -> SysResult<Pointer> {
        pointer_cache().lookup(self)
    }
}

pub fn load_all_pointers() {
    pointer_cache().load_all_pointers();
    obj_refs::PLAYER.load_all_pointers();
    obj_refs::TARGET.load_all_pointers();
    obj_refs::TORRENT.load_all_pointers();
}

pub fn global_pointers() -> Vec<(String, u64)> {
    pointer_cache().pointers()
}

pub fn player_pointers() -> Vec<(String, u64)> {
    obj_refs::PLAYER.pointers()
}

pub fn target_pointers() -> Vec<(String, u64)> {
    obj_refs::TARGET.pointers()
}

pub fn torrent_pointers() -> Vec<(String, u64)> {
    obj_refs::TORRENT.pointers()
}
