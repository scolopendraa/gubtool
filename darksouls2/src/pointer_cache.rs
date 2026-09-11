use {
    crate::{
        event::get_obj_state_act_ctrl,
        obj_refs,
        offsets::{
            game_manager_imp::{self, event_manager_offsets, game_data_manager_offsets},
            module_offsets::BasePointer,
        },
        resources::map_ids::MapId,
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
    GameManagerImp,
    GameDataManager,
    AiManager,
    ClearCountPtr,
    EventManager,
    EventFlagManager,
    WarpManager,
    BonfireManager,
    WindowManager,
    DlBackAllocator,
    BelfryGateStateActCtrl,
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
            ResolvedPtr::GameManagerImp => read_pointer(BasePointer::GameManagerImp),
            ResolvedPtr::GameDataManager => {
                cache
                    .lookup(ResolvedPtr::GameManagerImp)
                    .read_ptr(game_manager_imp::GAME_DATA_MANAGER)
            }
            ResolvedPtr::ClearCountPtr => {
                cache
                    .lookup(ResolvedPtr::GameDataManager)
                    .read_ptr(game_data_manager_offsets::CLEARCOUNT_PTR)
            }
            ResolvedPtr::AiManager => {
                cache
                    .lookup(ResolvedPtr::GameManagerImp)
                    .read_ptr(game_manager_imp::AI_MANAGER)
            }
            ResolvedPtr::EventManager => {
                cache
                    .lookup(ResolvedPtr::GameManagerImp)
                    .read_ptr(game_manager_imp::EVENT_MANAGER)
            }
            ResolvedPtr::EventFlagManager => {
                cache
                    .lookup(ResolvedPtr::EventManager)
                    .read_ptr(event_manager_offsets::EVENT_FLAG_MANAGER)
            }
            ResolvedPtr::WarpManager => {
                cache
                    .lookup(ResolvedPtr::EventManager)
                    .read_ptr(event_manager_offsets::EVENT_WARP_MANAGER)
            }
            ResolvedPtr::BonfireManager => {
                cache
                    .lookup(ResolvedPtr::EventManager)
                    .read_ptr(event_manager_offsets::EVENT_BONFIRE_MANAGER)
            }
            ResolvedPtr::WindowManager => {
                cache
                    .lookup(ResolvedPtr::EventManager)
                    .read_ptr(event_manager_offsets::EVENT_WINDOW_MANAGER)
            }
            ResolvedPtr::DlBackAllocator => {
                cache
                    .lookup(ResolvedPtr::GameManagerImp)
                    .read_ptr(game_manager_imp::DL_BACK_ALLOCATOR)
            }
            ResolvedPtr::BelfryGateStateActCtrl => {
                Ok(get_obj_state_act_ctrl(MapId::TheLostBastille, 10161051).unwrap_or_default())
            }
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
