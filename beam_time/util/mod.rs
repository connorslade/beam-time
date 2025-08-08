use std::path::{Path, PathBuf};

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
    $($ctx.input.key_pressed($key).then(|| $action);)*
}

pub fn load_level_dir(dir: &Path) -> Vec<(PathBuf, BoardMeta)> {
    let mut out = Vec::new();

    for world in dir.read_dir().unwrap().filter_map(Result::ok) {
        let path = world.path();
        let meta = match Board::load_meta(&path) {
            Ok(meta) => meta,
            Err(err) => {
                warn!("Failed to load meta for {path:?}: {err}");
                continue;
            }
        };

        out.push((path, meta));
    }

    out
}

const TIME_UNITS: &[(u64, &str)] = &[(86400, "d"), (3600, "h"), (60, "m"), (1, "s")];

pub fn human_duration(mut secs: u64) -> String {
    let mut out = String::new();

    for &(unit, label) in TIME_UNITS {
        if secs >= unit {
            out.push_str(&format!("{}{} ", secs / unit, label));
            secs %= unit;
        }
    }

    out.trim_end().to_string()
}

pub fn human_duration_minimal(secs: u64) -> String {
    for &(unit, label) in TIME_UNITS {
        if secs >= unit {
            return format!("{}{}", secs / unit, label);
        }
    }

    let (unit, label) = TIME_UNITS[0];
    format!("{}{}", secs / unit, label)
}
