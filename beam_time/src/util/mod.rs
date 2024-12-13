use std::path::PathBuf;

use log::warn;

use crate::game::board::{Board, BoardMeta};

pub mod hwid;

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
    $(
        if $ctx.input.key_pressed($key) {
            $action;
        }
    )*
}

pub fn load_level_dir(dir: PathBuf) -> Vec<(PathBuf, BoardMeta)> {
    let mut out = Vec::new();

    for world in dir.read_dir().unwrap().filter_map(Result::ok) {
        let path = world.path();
        let meta = match Board::load_meta(&path) {
            Ok(meta) => meta,
            Err(err) => {
                warn!("Failed to load meta for {:?}: {}", path, err);
                continue;
            }
        };

        out.push((path, meta));
    }

    out
}
