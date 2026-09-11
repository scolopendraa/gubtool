use {
    gubtool_core::{address::Pointer, sys::sys_error::SysResult},
    std::{collections::HashMap, fmt::Debug, hash::Hash},
    strum::IntoEnumIterator,
};

pub struct PointerCache<K> {
    pub map:  HashMap<K, Pointer>,
    pub base: Option<Pointer>,
}

impl<K> PointerCache<K>
where K: PointerKey
{
    pub fn new() -> Self {
        Self {
            map:  HashMap::new(),
            base: None,
        }
    }

    pub fn new_with_base(base: Pointer) -> Self {
        Self {
            map:  HashMap::new(),
            base: Some(base),
        }
    }

    pub fn pointers(&self) -> Vec<(String, u64)> {
        let mut pointers = Vec::with_capacity(self.map.len() + self.base.is_some() as usize);

        if let Some(pointer) = self.base {
            pointers.push((format!("Base Pointer"), pointer.address));
        }

        pointers.extend(
            self.map
                .iter()
                .map(|(name, pointer)| (format!("{:?}", name), pointer.address)),
        );

        pointers
    }

    pub fn load_all_pointers(&mut self) {
        self.clear_pointers();

        for key in K::iter() {
            let _ = self.lookup(key);
        }
    }

    pub fn lookup(&mut self, key: K) -> SysResult<Pointer> {
        if let Some(&pointer) = self.map.get(&key) {
            return Ok(pointer);
        }

        let pointer = K::lookup(self, key, self.base)?;

        self.map.insert(key, pointer);

        Ok(pointer)
    }

    pub fn clear_pointers(&mut self) {
        self.map.clear();
    }
}

pub trait PointerKey: IntoEnumIterator + Debug + Eq + Hash + Copy {
    fn lookup(
        cache: &mut PointerCache<Self>,
        key: Self,
        base: Option<Pointer>,
    ) -> SysResult<Pointer>
    where
        Self: Sized;
}
