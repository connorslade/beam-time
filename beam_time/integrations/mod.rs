use common::user::UserId;

#[cfg(feature = "discord")]
use crate::integrations::discord::Discord;
#[cfg(feature = "steam")]
use crate::integrations::steam::Steam;

#[cfg(feature = "steam")]
mod steam;

#[cfg(feature = "discord")]
mod discord;

#[allow(dead_code)]
pub enum RichPresence {
    None,
    Sandbox,
    Campaign(String),
}

pub struct Integrations {
    #[cfg(feature = "steam")]
    steam: Steam,
    #[cfg(feature = "discord")]
    discord: Discord,
}

impl Integrations {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "steam")]
            steam: Steam::init().unwrap(),
            #[cfg(feature = "discord")]
            discord: Discord::init(),
        }
    }

    pub fn user_id(&self) -> UserId {
        #[cfg(feature = "steam")]
        return UserId::Steam(self.steam.user_id());
        #[cfg(not(feature = "steam"))]
        return UserId::Hardware(crate::util::hwid::get());
    }

    pub fn tick(&mut self) {
        #[cfg(feature = "steam")]
        self.steam.on_tick();
    }
}

impl Integrations {
    pub fn award_achievement(&self, _name: &str) {
        #[cfg(feature = "steam")]
        self.steam.award_achievement(_name);
    }

    pub fn rich_presence(&mut self, _value: RichPresence) {
        #[cfg(feature = "steam")]
        self.steam.rich_presence(&_value);
        #[cfg(feature = "discord")]
        self.discord.rich_presence(&_value);
    }
}
