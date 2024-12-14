use std::{ops::Deref, path::Path};

use anyhow::Result;
use native_db::Database;
use schema::SCHEMA;

pub mod id_key;
pub mod schema;

pub struct Db {
    inner: Database<'static>,
}

impl Db {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let inner = native_db::Builder::new().create(&SCHEMA, &path)?;
        Ok(Self { inner })
    }
}

impl Deref for Db {
    type Target = Database<'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
