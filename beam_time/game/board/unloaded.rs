use std::path::{Path, PathBuf};

use log::warn;

use crate::game::board::{Board, BoardMeta};

pub struct UnloadedBoard {
    pub path: PathBuf,
    pub meta: BoardMeta,
}

pub fn load_level_dir(dir: &Path) -> Vec<UnloadedBoard> {
    let mut out = Vec::new();

    for world in dir.read_dir().unwrap().filter_map(Result::ok) {
        let path = world.path();
        if !path.is_file() {
            continue;
        }

        let meta = match Board::load_meta(&path) {
            Ok(meta) => meta,
            Err(err) => {
                warn!("Failed to load meta for {path:?}: {err}");
                continue;
            }
        };

        out.push(UnloadedBoard { path, meta });
    }

    out
}
