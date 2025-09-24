use anyhow::Result;
use log::{trace, warn};
use steamworks::Client;

use crate::{consts::STEAM_ID, integrations::RichPresence};

pub struct Steam {
    client: Client,
}

impl Steam {
    pub fn init() -> Result<Self> {
        let client = Client::init_app(STEAM_ID)?;
        let user_id = client.user().steam_id().raw();
        client.user_stats().request_user_stats(user_id);
        Ok(Self { client })
    }

    pub fn on_tick(&mut self) {
        self.client.run_callbacks();
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

    pub fn rich_presence(&self, _app: &App, value: &RichPresence) {
        let friends = self.client.friends();

        friends.set_rich_presence("steam_display", value.steam_display());
        match value {
            RichPresence::Campaign(name) => {
                friends.set_rich_presence("name", Some(&name));
            }
            _ => {}
        }
    }
}

impl RichPresence {
    fn steam_display(&self) -> Option<&str> {
        match self {
            RichPresence::None => None,
            RichPresence::Sandbox => Some("#Status_Sandbox"),
            RichPresence::Campaign(_) => Some("#Status_Campaign"),
        }
    }
}
