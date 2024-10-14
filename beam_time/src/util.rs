use engine::exports::nalgebra::Vector2;

pub macro include_asset($name:expr) {
    include_bytes!(concat!("../assets/", $name))
}

pub macro include_atlas($name:expr) {
    image::load_from_memory(include_asset!($name))
        .unwrap()
        .to_rgba8()
}

pub fn in_bounds(pos: Vector2<f32>, bounds: (Vector2<f32>, Vector2<f32>)) -> bool {
    pos.x >= bounds.0.x && pos.x <= bounds.1.x && pos.y >= bounds.0.y && pos.y <= bounds.1.y
}
