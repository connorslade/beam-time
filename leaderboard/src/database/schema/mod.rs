use native_db::Models;
use once_cell::sync::Lazy;

pub mod results;

pub static SCHEMA: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<results::Results>().unwrap();
    models
});
