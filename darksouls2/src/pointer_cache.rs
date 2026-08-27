use {
    crate::{
        event::get_obj_state_act_ctrl,
        mem::read_address,
        offsets::{
            ChainReadExt,
            game_manager_imp::{self, event_manager_offsets, game_data_manager_offsets},
            module_offsets::BasePointer,
        },
        resources::map_ids::MapId,
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

impl PointerCache {
    pub fn lookup(&self, pointer: ResolvedPtr) -> SysResult<u64> {
        {
            let cache = self.map.lock().unwrap();
            if let Some(&val) = cache.get(&pointer) {
                return Ok(val);
            }
        }

        let resolved_pointer = match pointer {
            ResolvedPtr::GameManagerImp => read_address(BasePointer::GameManagerImp),
            ResolvedPtr::GameDataManager => {
                self.lookup(ResolvedPtr::GameManagerImp)
                    .read_offset(game_manager_imp::GAME_DATA_MANAGER)
            }
            ResolvedPtr::ClearCountPtr => {
                self.lookup(ResolvedPtr::GameDataManager)
                    .read_offset(game_data_manager_offsets::CLEARCOUNT_PTR)
            }
            ResolvedPtr::AiManager => {
                self.lookup(ResolvedPtr::GameManagerImp)
                    .read_offset(game_manager_imp::AI_MANAGER)
            }
            ResolvedPtr::EventManager => {
                self.lookup(ResolvedPtr::GameManagerImp)
                    .read_offset(game_manager_imp::EVENT_MANAGER)
            }
            ResolvedPtr::EventFlagManager => {
                self.lookup(ResolvedPtr::EventManager)
                    .read_offset(event_manager_offsets::EVENT_FLAG_MANAGER)
            }
            ResolvedPtr::WarpManager => {
                self.lookup(ResolvedPtr::EventManager)
                    .read_offset(event_manager_offsets::EVENT_WARP_MANAGER)
            }
            ResolvedPtr::BonfireManager => {
                self.lookup(ResolvedPtr::EventManager)
                    .read_offset(event_manager_offsets::EVENT_BONFIRE_MANAGER)
            }
            ResolvedPtr::WindowManager => {
                self.lookup(ResolvedPtr::EventManager)
                    .read_offset(event_manager_offsets::EVENT_WINDOW_MANAGER)
            }
            ResolvedPtr::DlBackAllocator => {
                self.lookup(ResolvedPtr::GameManagerImp)
                    .read_offset(game_manager_imp::DL_BACK_ALLOCATOR)
            }
            ResolvedPtr::BelfryGateStateActCtrl => {
                Ok(get_obj_state_act_ctrl(MapId::TheLostBastille, 10161051).unwrap_or_default())
            }
        }?;

        let mut cache = self.map.lock().unwrap();
        // if resolved_pointer != 0x0 {
        cache.insert(pointer, resolved_pointer);
        // }
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
