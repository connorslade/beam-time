pub macro include_assets($name:expr) {
    include_bytes!(concat!("../assets/", $name))
}

pub macro include_atlas($name:expr) {
    image::load_from_memory(include_assets!($name))
        .unwrap()
        .to_rgba8()
}
