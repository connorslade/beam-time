#[macro_export]
macro_rules! include_atlas {
    ($name:expr) => {
        image::load_from_memory(include_bytes!(concat!("../assets/", $name)))
            .unwrap()
            .to_rgba8()
    };
}
