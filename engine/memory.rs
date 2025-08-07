use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::{DefaultHasher, Hash, Hasher},
};

type Key = (MemoryKey, TypeId);

#[derive(Default)]
pub struct Memory {
    entries: HashMap<Key, Box<dyn Any>>,
    accessed: RefCell<HashSet<Key>>,
}

impl Memory {
    fn key<T: 'static>(key: MemoryKey) -> Key {
        (key, TypeId::of::<T>())
    }

    pub fn insert<T: 'static>(&mut self, key: MemoryKey, value: T) {
        let key = Self::key::<T>(key);
        self.accessed.borrow_mut().insert(key);
        self.entries.insert(key, Box::new(value));
    }

    pub fn get<T: 'static>(&self, key: MemoryKey) -> Option<&T> {
        let key = Self::key::<T>(key);
        self.accessed.borrow_mut().insert(key);
        self.entries.get(&key).map(|x| x.downcast_ref().unwrap())
    }

    pub fn get_mut<T: 'static>(&mut self, key: MemoryKey) -> Option<&mut T> {
        let key = Self::key::<T>(key);
        self.accessed.borrow_mut().insert(key);
        self.entries
            .get_mut(&key)
            .map(|x| x.downcast_mut().unwrap())
    }

    pub fn get_or_insert<T: 'static>(&mut self, key: MemoryKey, fallback: T) -> &mut T {
        let key = Self::key::<T>(key);
        self.accessed.borrow_mut().insert(key);
        self.entries
            .entry(key)
            .or_insert(Box::new(fallback))
            .downcast_mut()
            .unwrap()
    }

    pub fn get_or_insert_with<T: 'static>(
        &mut self,
        key: MemoryKey,
        fallback: impl FnOnce() -> T,
    ) -> &mut T {
        self.get_or_insert(key, fallback())
    }

    pub(crate) fn garbage_collect(&mut self) {
        let mut accessed = self.accessed.borrow_mut();
        self.entries.retain(|k, _v| accessed.contains(k));
        accessed.clear();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryKey(u32);

impl MemoryKey {
    pub const fn new(name: &str) -> Self {
        Self(const_fnv1a_hash::fnv1a_hash_str_32(name))
    }

    pub const fn from_raw(hash: u32) -> Self {
        Self(hash)
    }

    pub fn context(self, context: impl Hash) -> Self {
        let mut hasher = DefaultHasher::new();
        context.hash(&mut hasher);
        self.0.hash(&mut hasher);
        Self(hasher.finish() as u32)
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
        $crate::memory::MemoryKey::new(concat!(file!(), line!(), column!()))
    };
    ($($ctx:expr),+) => {
        {
            let mut key = memory_key!();
            $(key = key.context($ctx);)+
            key
        }
    };
}
