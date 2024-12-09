use anyhow::Result;
use log::{trace, warn};
use steamworks::{Client, SingleClient};

use crate::consts::STEAM_ID;

pub struct Steam {
    client: Client,
    sync: SingleClient,
}

impl Steam {
    pub fn init() -> Result<Self> {
        let (client, sync) = Client::init_app(STEAM_ID)?;
        Ok(Self { client, sync })
    }

    pub fn on_tick(&mut self) {
        self.sync.run_callbacks();
    }

    pub fn award_achievement(&self, achievement: &str) {
        trace!("Awarding achievement `{achievement}`");
        let stats = self.client.user_stats();

        let result = stats.achievement(achievement).set();
        if result.is_err() {
            warn!("Error granting achievement `{achievement}`");
            return;
        }

        if stats.store_stats().is_err() {
            warn!("Error pushing achievements to server");
        }
    }
}
