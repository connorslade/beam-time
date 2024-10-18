use engine::exports::nalgebra::{Scalar, Vector2};

pub macro include_asset($name:expr) {
    include_bytes!(concat!("../assets/", $name))
}

pub macro include_atlas($name:expr) {
    image::load_from_memory(include_asset!($name))
        .unwrap()
        .to_rgba8()
}

pub fn in_bounds<T: Scalar + PartialOrd>(
    pos: Vector2<T>,
    bounds: (Vector2<T>, Vector2<T>),
) -> bool {
    pos.x >= bounds.0.x && pos.x <= bounds.1.x && pos.y >= bounds.0.y && pos.y <= bounds.1.y
}
