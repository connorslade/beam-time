use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Default)]
pub struct Memory {
    entries: HashMap<(MemoryKey, TypeId), Box<dyn Any>>,
}

impl Memory {
    fn key<T: 'static>(key: MemoryKey) -> (MemoryKey, TypeId) {
        (key, TypeId::of::<T>())
    }

    pub fn insert<T: 'static>(&mut self, key: MemoryKey, value: T) {
        self.entries.insert(Self::key::<T>(key), Box::new(value));
    }

    pub fn get<T: 'static>(&self, key: MemoryKey) -> Option<&T> {
        self.entries
            .get(&Self::key::<T>(key))
            .map(|x| x.downcast_ref().unwrap())
    }

    pub fn get_or_insert<T: 'static>(&mut self, key: MemoryKey, fallback: T) -> &mut T {
        let key = Self::key::<T>(key);
        self.entries
            .entry(key)
            .or_insert(Box::new(fallback))
            .downcast_mut()
            .unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryKey(u32);

impl MemoryKey {
    pub const fn new(name: &str) -> Self {
        Self(const_fnv1a_hash::fnv1a_hash_str_32(name))
    }

    pub const fn context(self, name: &str) -> Self {
        let name_hash = const_fnv1a_hash::fnv1a_hash_str_32(name);
        Self(self.0.wrapping_add(name_hash))
    }
}

impl Hash for MemoryKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}

#[macro_export]
macro_rules! memory_key {
    () => {
        $crate::memory::MemoryKey::new(concat!(file!(), line!()))
    };
}
