use std::{fs, path::PathBuf, time::Instant};

use anyhow::Result;
use log::{trace, warn};
use serde::{Deserialize, Serialize};
use steamworks::{Client, ClientManager, SingleClient};

use crate::{
    consts::{CONFIG_FILE, STEAM_ID},
    leaderboard::LeaderboardManager,
    ui::waterfall::WaterfallState,
};

pub struct App {
    pub steam: Client<ClientManager>,
    pub steam_sync: SingleClient<ClientManager>,

    pub leaderboard: LeaderboardManager,

    pub start: Instant,
    pub waterfall: WaterfallState,

    pub config: Config,
    pub data_dir: PathBuf,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub zoom_sensitivity: f32,
    pub movement_speed: f32,
    pub ui_scale: f32,
}

impl App {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir().unwrap().join("BeamTime");
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).unwrap();
        }

        let config = fs::read_to_string(data_dir.join(CONFIG_FILE))
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default();

        // todo: handle this case without unwrap
        let (steam, steam_sync) = steamworks::Client::init_app(STEAM_ID).unwrap();

        Self {
            steam,
            steam_sync,
            leaderboard: LeaderboardManager::default(),

            start: Instant::now(),
            waterfall: WaterfallState::default(),

            config,
            data_dir,
        }
    }

    pub fn award_achievement(&self, achievement: &str) {
        trace!("Awarding achievement `{achievement}`");
        let stats = self.steam.user_stats();

        let result = stats.achievement(achievement).set();
        if result.is_err() {
            warn!("Error granting achievement `{achievement}`");
            return;
        }

        if stats.store_stats().is_err() {
            warn!("Error pushing achievements to server");
        }
    }

    pub fn save_config(&self) -> Result<()> {
        fs::write(
            self.data_dir.join(CONFIG_FILE),
            toml::to_string(&self.config).unwrap(),
        )?;
        Ok(())
    }

    pub fn on_tick(&mut self) {
        self.steam_sync.run_callbacks();
        self.leaderboard.tick();
    }

    pub fn frame(&self) -> u8 {
        self.start.elapsed().as_millis() as u8 / 100 % 3
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            zoom_sensitivity: 1.0,
            movement_speed: 500.0,
            ui_scale: 1.0,
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = self.save_config();
    }
}
