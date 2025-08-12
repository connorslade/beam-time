use std::{fs, path::PathBuf, time::Instant};

use anyhow::Result;
use common::user::UserId;
use engine::graphics_context::GraphicsContext;
use serde::{Deserialize, Serialize};

#[cfg(feature = "steam")]
use crate::steam::Steam;
use crate::{consts::CONFIG_FILE, leaderboard::LeaderboardManager, screens::Screen};

pub struct App {
    pub id: UserId,
    #[cfg(feature = "steam")]
    pub steam: Steam,
    pub leaderboard: LeaderboardManager,

    pub start: Instant,
    pub debug: Vec<String>,

    pub config: Config,
    pub scale_multiplier: f32,
    pub data_dir: PathBuf,

    pub new_screens: Vec<Box<dyn Screen>>,
    pub close_screens: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub vsync: bool,
    pub show_fps: bool,
    pub debug: bool,

    pub zoom_sensitivity: f32,
    pub movement_speed: f32,

    pub interface_scale: f32,
    pub fullscreen: bool,
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
            .and_then(|s| toml::from_str::<Config>(&s).ok())
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
            debug: Vec::new(),

            scale_multiplier: config.interface_scale,
            config,
            data_dir,

            new_screens: vec![],
            close_screens: 0,
        }
    }

    pub fn debug(&mut self, msg: impl Fn() -> String) {
        self.debug.push(msg());
    }

    pub fn save_config(&self) -> Result<()> {
        fs::write(
            self.data_dir.join(CONFIG_FILE),
            toml::to_string(&self.config).unwrap(),
        )?;
        Ok(())
    }

    pub fn on_tick(&mut self, ctx: &mut GraphicsContext) {
        ctx.scale_factor *= self.scale_multiplier;

        #[cfg(feature = "steam")]
        self.steam.on_tick();
        self.leaderboard.tick();

        ctx.window.vsync(self.config.vsync);
        ctx.window.fullscreen(self.config.fullscreen);
    }

    pub fn frame(&self) -> u8 {
        self.start.elapsed().as_millis() as u8 / 100 % 3
    }
}

impl App {
    pub fn pop_screen(&mut self) {
        self.close_screens += 1;
    }

    pub fn push_screen(&mut self, mut screen: impl Screen + 'static) {
        screen.on_init(self);
        self.new_screens.push(Box::new(screen));
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vsync: true,
            show_fps: false,
            debug: false,

            zoom_sensitivity: 0.08,
            movement_speed: 2000.0,

            fullscreen: false,
            interface_scale: 1.0,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = self.save_config();
    }
}
