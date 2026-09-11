use {
    crate::pointer_cache::{PointerCache, PointerKey},
    gubtool_core::sys::sys_error::{PointerType, SysError, SysResult},
    std::{
        fmt::Display,
        marker::PhantomData,
        ops::{Deref, DerefMut},
        sync::{Mutex, MutexGuard},
    },
};

pub struct ObjRef<T, P: PointerType, U: ObjUpdater<T>> {
    pub obj_ins:   Mutex<Option<T>>,
    _phantomdata: PhantomData<(P, U)>,
}

pub struct ObjGuard<'a, T> {
    guard: MutexGuard<'a, Option<T>>,
}

impl<T> Deref for ObjGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.guard.as_ref().unwrap()
    }
}

impl<T> DerefMut for ObjGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.guard.as_mut().unwrap()
    }
}

impl<T, P, U> ObjRef<T, P, U>
where
    P: PointerType,
    U: ObjUpdater<T>,
{
    pub fn new() -> Self {
        let s = Self {
            obj_ins:       Mutex::new(None),
            _phantomdata: PhantomData,
        };
        s.update();
        s
    }

    pub fn update(&self) {
        U::update(&mut self.obj_ins.lock().unwrap());
    }

    pub fn obj(&self) -> SysResult<ObjGuard<'_, T>> {
        let guard = self.obj_ins.lock().unwrap();

        if guard.is_none() {
            return Err(SysError::null_pointer(P::instance()));
        }

        Ok(ObjGuard {
            guard,
        })
    }

    pub fn pointers(&self) -> Vec<(String, u64)>
    where T: GameObject {
        self.obj_ins
            .lock()
            .unwrap()
            .as_ref()
            .map(|obj| obj.pointers())
            .unwrap_or_default()
    }

    pub fn load_all_pointers(&self)
    where T: GameObject {
        if let Some(obj) = self.obj_ins.lock().unwrap().as_mut() {
            obj.load_all_pointers();
        }
    }
}

pub trait GameObject {
    type PointerKey: PointerKey;
    fn pointer_cache(&self) -> &PointerCache<Self::PointerKey>;
    fn pointer_cache_mut(&mut self) -> &mut PointerCache<Self::PointerKey>;

    fn pointers(&self) -> Vec<(String, u64)> {
        self.pointer_cache().pointers()
    }

    fn load_all_pointers(&mut self) {
        self.pointer_cache_mut().load_all_pointers();
    }
}

pub trait ObjUpdater<T> {
    fn update(obj_ins: &mut Option<T>);
}

#[derive(Debug)]
pub struct PlayerPointerType;

impl PointerType for PlayerPointerType {
    fn instance() -> &'static dyn PointerType
    where Self: Sized {
        &PlayerPointerType
    }
}

impl Display for PlayerPointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player not loaded")
    }
}

#[derive(Debug)]
pub struct TargetPointerType;

impl PointerType for TargetPointerType {
    fn instance() -> &'static dyn PointerType
    where Self: Sized {
        &TargetPointerType
    }
}

impl Display for TargetPointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Target not found")
    }
}
