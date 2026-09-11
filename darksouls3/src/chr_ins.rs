use {
    gubtool_core::{
        address::{ChainReadExt, Pointer},
        attached::version,
        game_version::DarkSouls3Version::{self, *},
        sys::{read_pointer, sys_error::SysResult},
    },
    macros::GameObject,
    shared::pointer_cache::{PointerCache, PointerKey},
    strum::EnumIter,
};

#[derive(GameObject)]
pub struct ChrIns {
    pub base:     Pointer,
    pub pointers: PointerCache<ResolvedChrPtr>,
}

impl ChrIns {
    pub fn get_hp(&mut self) -> SysResult<i32> {
        self.get_ptr(ResolvedChrPtr::DataModule)
        .add_offset(0xd8)
        .read::<i32>()
    }

    pub fn set_hp(&mut self, val: i32) -> SysResult {
        self.get_ptr(ResolvedChrPtr::DataModule)
        .add_offset(0xd8)
        .write::<i32>(val)
    }

    pub fn get_max_hp(&mut self) -> SysResult<i32> {
        self.get_ptr(ResolvedChrPtr::DataModule)
        .add_offset(0xdc)
        .read::<i32>()
    }
}

impl ChrIns {
    pub fn new(pointer: Pointer) -> Self {
        ChrIns {
            base:     pointer,
            pointers: PointerCache::new_with_base(pointer),
        }
    }

    pub fn get_ptr(&mut self, pointer: ResolvedChrPtr) -> SysResult<Pointer> {
        self.pointers.lookup(pointer)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, EnumIter)]
pub enum ResolvedChrPtr {
    Modules,
    TimeActModule,
    DataModule,
    ResistModule,
    BehaviorModule,
    SuperArmorModule,
    EventModule,
    PhysicsModule,
    AiThink,
}

impl PointerKey for ResolvedChrPtr {
    fn lookup(
        cache: &mut PointerCache<Self>,
        key: Self,
        base: Option<Pointer>,
    ) -> SysResult<Pointer>
    where
        Self: Sized,
    {
        let base = base.unwrap();

        match key {
            ResolvedChrPtr::Modules => read_pointer(base + modules_offset()),
            ResolvedChrPtr::TimeActModule => cache.lookup(ResolvedChrPtr::Modules).read_ptr(0x10),
            ResolvedChrPtr::DataModule => cache.lookup(ResolvedChrPtr::Modules).read_ptr(0x18),
            ResolvedChrPtr::ResistModule => cache.lookup(ResolvedChrPtr::Modules).read_ptr(0x20),
            ResolvedChrPtr::BehaviorModule => cache.lookup(ResolvedChrPtr::Modules).read_ptr(0x28),
            ResolvedChrPtr::SuperArmorModule => {
                cache.lookup(ResolvedChrPtr::Modules).read_ptr(0x40)
            }
            ResolvedChrPtr::EventModule => cache.lookup(ResolvedChrPtr::Modules).read_ptr(0x58),
            ResolvedChrPtr::PhysicsModule => cache.lookup(ResolvedChrPtr::Modules).read_ptr(0x68),
            ResolvedChrPtr::AiThink => read_pointer(base + 0x58).read_ptr(0x320),
        }
    }
}

pub fn modules_offset() -> i32 {
    match version::<DarkSouls3Version>() {
        Some(v) if v < Version1_12_0 => 0x1f80,
        Some(v) if v == Version1_12_0 => 0x1f88,
        _ => 0x1f90,
    }
}
