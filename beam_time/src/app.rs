use std::{fs, path::PathBuf, time::Instant};

use anyhow::Result;
use common::user::UserId;
use engine::graphics_context::GraphicsContext;
use serde::{Deserialize, Serialize};

#[cfg(feature = "steam")]
use crate::steam::Steam;
use crate::{consts::CONFIG_FILE, leaderboard::LeaderboardManager, ui::waterfall::WaterfallState};

pub struct App {
    pub id: UserId,
    #[cfg(feature = "steam")]
    pub steam: Steam,
    pub leaderboard: LeaderboardManager,

    pub start: Instant,
    pub waterfall: WaterfallState,
    #[cfg(feature = "debug")]
    pub debug: Vec<String>,

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

        let _ = fs::create_dir(data_dir.join("levels"));

        let config = fs::read_to_string(data_dir.join(CONFIG_FILE))
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default();

        #[cfg(feature = "steam")]
        let steam = Steam::init().unwrap();

        Self {
            #[cfg(feature = "steam")]
            id: UserId::Steam(steam.user_id()),
            #[cfg(not(feature = "steam"))]
            id: UserId::Hardware(crate::util::hwid::get()),

            #[cfg(feature = "steam")]
            steam,
            leaderboard: LeaderboardManager::default(),

            start: Instant::now(),
            waterfall: WaterfallState::default(),
            #[cfg(feature = "debug")]
            debug: Vec::new(),

            config,
            data_dir,
        }
    }

    // => (Margin, Padding)
    pub fn spacing<App>(&self, ctx: &mut GraphicsContext<App>) -> (f32, f32) {
        let margin = 16.0 * self.config.ui_scale * ctx.scale_factor;
        let padding = 10.0 * self.config.ui_scale * ctx.scale_factor;

        (margin, padding)
    }

    pub fn debug(&mut self, _msg: impl Fn() -> String) {
        #[cfg(feature = "debug")]
        self.debug.push(_msg());
    }

    pub fn save_config(&self) -> Result<()> {
        fs::write(
            self.data_dir.join(CONFIG_FILE),
            toml::to_string(&self.config).unwrap(),
        )?;
        Ok(())
    }

    pub fn on_tick(&mut self) {
        #[cfg(feature = "steam")]
        self.steam.on_tick();
        self.leaderboard.tick();
    }

    pub fn frame(&self) -> u8 {
        self.start.elapsed().as_millis() as u8 / 100 % 3
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            zoom_sensitivity: 0.08,
            movement_speed: 2000.0,
            ui_scale: 1.0,
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = self.save_config();
    }
}
