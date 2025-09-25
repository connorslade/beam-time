use std::time::{SystemTime, UNIX_EPOCH};

use clone_macro::clone;
use crossbeam_channel::Sender;
use discord_presence::{
    Client,
    models::{ActivityType, DisplayType},
};
use log::{error, info};

use crate::{consts, integrations::RichPresence};

pub struct Discord {
    _discord: Client,
    sender: Sender<RichPresence>,
}

impl Discord {
    pub fn init() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();

        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut discord = Client::new(consts::DISCORD_ID);
        discord
            .on_ready(clone!([discord], move |_| {
                info!("Discord RPC connected");

                let mut discord = discord.clone();
                for value in rx.iter() {
                    let activity = match value {
                        RichPresence::None => "In Menu",
                        RichPresence::Sandbox => "Sandbox",
                        RichPresence::Campaign(_) => "Campaign",
                    };
                    let details = match &value {
                        RichPresence::Campaign(c) => c,
                        _ => "",
                    };

                    discord
                        .set_activity(|a| {
                            a.activity_type(ActivityType::Playing)
                                .status_display(DisplayType::Name)
                                .timestamps(|t| t.start(start))
                                .details(activity)
                                .state(details)
                        })
                        .unwrap();
                }
            }))
            .persist();
        discord
            .on_error(|ctx| error!("Discord Error: {:?}", ctx.event))
            .persist();
        discord.start();

        Self {
            _discord: discord,
            sender: tx,
        }
    }

    pub fn rich_presence(&mut self, value: &RichPresence) {
        self.sender.send(value.clone()).unwrap();
    }
}
