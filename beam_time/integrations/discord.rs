use std::time::{SystemTime, UNIX_EPOCH};

use clone_macro::clone;
use crossbeam_channel::Sender;
use discord_presence::{
    Client,
    models::{ActivityType, DisplayType},
};
use log::info;

use crate::{
    consts::{self, GAME_HOMEPAGE},
    integrations::RichPresence,
};

pub struct Discord {
    _discord: Client,
    sender: Sender<RichPresence>,
}

impl Discord {
    pub fn init() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        tx.send(RichPresence::None).unwrap();

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
                    let (activity, details) = match &value {
                        RichPresence::None => ("In Menu", None),
                        RichPresence::Sandbox => ("Sandbox", None),
                        RichPresence::Campaign(c) => ("Campaign", Some(c.as_str())),
                    };

                    discord
                        .set_activity(|a| {
                            let mut a = a
                                .activity_type(ActivityType::Playing)
                                .status_display(DisplayType::Name)
                                .append_buttons(|b| b.label("Get it on Steam").url(GAME_HOMEPAGE))
                                .timestamps(|t| t.start(start))
                                .details(activity);
                            if let Some(details) = details {
                                a = a.state(details);
                            }

                            a
                        })
                        .unwrap();
                }
            }))
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
