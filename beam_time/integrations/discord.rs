use std::time::{SystemTime, UNIX_EPOCH};

use discord_presence::{
    Client,
    models::{ActivityType, DisplayType},
};
use log::info;

use crate::{consts, integrations::RichPresence};

pub struct Discord {
    discord: Client,
    start: u64,
}

impl Discord {
    pub fn init() -> Self {
        let mut discord = Client::new(1420274195216207933);
        discord
            .on_ready(|_| info!("Discord RPC connected"))
            .persist();
        discord
            .on_error(|ctx| {
                eprintln!("An error occured, {:?}", ctx.event);
            })
            .persist();
        discord.start();

        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self { discord, start }
    }

    pub fn rich_presence(&mut self, value: &RichPresence) {
        self.discord
            .set_activity(|a| {
                a.activity_type(ActivityType::Playing)
                    .details(match value {
                        RichPresence::None => "In Menu",
                        RichPresence::Sandbox => "Sandbox",
                        RichPresence::Campaign(_) => "Campaign",
                    })
                    .state(match value {
                        RichPresence::Campaign(c) => c,
                        _ => "",
                    })
                    .append_buttons(|b| b.label("Get it on Steam!").url(consts::GAME_HOMEPAGE))
                    .status_display(DisplayType::Name)
                    .timestamps(|t| t.start(self.start))
            })
            .unwrap();
    }
}
