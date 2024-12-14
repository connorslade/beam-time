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

impl Default for IdKey {
    fn default() -> Self {
        Self {
            inner: Uuid::new_v4(),
        }
    }
}
