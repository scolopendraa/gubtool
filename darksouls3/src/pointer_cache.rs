use {
    crate::{obj_refs, offsets::module_offsets::BasePointer},
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
    WorldObjManImpl,
    WorldChrManImp,
    GameMan,
    GameDataMan,
    MenuMan,
    MapItemMan,
    EventFlagMan,
    WorldAiManager,
    CSDlc,
    PlayerGameData,
    SprjFlipperImp,
    PlayerIns,
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
            ResolvedPtr::WorldObjManImpl => read_pointer(BasePointer::WorldObjManImpl),
            ResolvedPtr::WorldChrManImp => read_pointer(BasePointer::WorldChrManImp),
            ResolvedPtr::GameMan => read_pointer(BasePointer::GameMan),
            ResolvedPtr::GameDataMan => read_pointer(BasePointer::GameDataMan),
            ResolvedPtr::MenuMan => read_pointer(BasePointer::MenuMan),
            ResolvedPtr::MapItemMan => read_pointer(BasePointer::MapItemMan),
            ResolvedPtr::EventFlagMan => read_pointer(BasePointer::EventFlagMan),
            ResolvedPtr::WorldAiManager => read_pointer(BasePointer::WorldAiManager),
            ResolvedPtr::SprjFlipperImp => read_pointer(BasePointer::SprjFlipperImp),
            ResolvedPtr::CSDlc => read_pointer(BasePointer::CsDlc),
            ResolvedPtr::PlayerGameData => cache.lookup(ResolvedPtr::GameDataMan).read_ptr(0x10),
            ResolvedPtr::PlayerIns => cache.lookup(ResolvedPtr::WorldChrManImp).read_ptr(0x80),
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
