use native_db::Models;

pub mod results;

pub fn get() -> Models {
    let mut models = Models::new();
    models.define::<results::Results>().unwrap();
    models
}