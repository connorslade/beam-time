use std::ops::Deref;

use native_db::{Key, ToKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
pub struct IdKey {
    inner: Uuid,
}

impl ToKey for IdKey {
    fn to_key(&self) -> Key {
        Key::new(self.inner.as_bytes().to_vec())
    }

    fn key_names() -> Vec<String> {
        vec!["Uuid".into()]
    }
}

impl Deref for IdKey {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<Uuid> for IdKey {
    fn from(inner: Uuid) -> Self {
        Self { inner }
    }
}

impl Default for IdKey {
    fn default() -> Self {
        Self {
            inner: Uuid::new_v4(),
        }
    }
}
