pub mod hwid;
pub mod time;

pub macro include_asset($name:expr) {
    include_bytes!(concat!("../assets/", $name))
}

pub macro include_atlas($name:expr) {
    image::load_from_memory(include_asset!($name))
        .unwrap()
        .to_rgba8()
}

pub macro key_events(
    $ctx:expr, { $($key:expr => $action:expr),* }
) {
    $($ctx.input.key_pressed($key).then(|| $action);)*
}
