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
        client.user_stats().request_current_stats();
        Ok(Self { client, sync })
    }

    pub fn on_tick(&mut self) {
        self.sync.run_callbacks();
    }

    pub fn user_id(&self) -> u64 {
        self.client.user().steam_id().raw()
    }

    pub fn award_achievement(&self, name: &str) {
        trace!("Awarding achievement `{name}`");
        let stats = self.client.user_stats();

        let achievement = stats.achievement(name);
        let was_set = achievement.get().unwrap_or_default();
        if achievement.set().is_err() {
            warn!("Error granting achievement `{name}`");
            return;
        }

        if !was_set && stats.store_stats().is_err() {
            warn!("Error pushing achievements to server");
        }
    }
}
