use std::path::PathBuf;

use anyhow::Result;
use native_db::{Database, Models};
use once_cell::sync::Lazy;

pub mod id_key;
pub mod schema;

static SCHEMA: Lazy<Models> = Lazy::new(|| schema::get());

pub struct Db {
    inner: Database<'static>,
}

impl Db {
    pub fn open(path: PathBuf) -> Result<Self> {
        let  inner = native_db::Builder::new().create(&SCHEMA, &path)?;
        Ok(Self { inner })
    }
}
