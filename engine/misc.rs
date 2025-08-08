use std::{mem, ops::Deref};

pub(crate) struct Mutable<T> {
    value: T,
    desired: Option<T>,
}

impl<T: PartialEq> Mutable<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            desired: None,
        }
    }

    pub fn set(&mut self, value: T) {
        self.desired = Some(value);
    }

    pub fn is_set(&self) -> bool {
        matches!(&self.desired, Some(val) if val != &self.value)
    }

    pub fn desired(&mut self) -> Option<&T> {
        if let Some(desired) = mem::take(&mut self.desired)
            && self.value != desired
        {
            self.value = desired;
            return Some(&self.value);
        }

        None
    }
}

impl<T> Deref for Mutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Default + PartialEq> Default for Mutable<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
