use std::{collections::HashMap, time::Duration};

use anyhow::Result;
use clone_macro::clone;
use log::{trace, warn};
use poll_promise::Promise;
use ureq::{Agent, AgentBuilder};
use uuid::Uuid;

use crate::consts::LEADERBOARD_SERVER;
use leaderboard::api::results::GetResultsResponse;

type PendingResult<T> = Promise<Result<T>>;

pub struct LeaderboardManager {
    client: Agent,

    requests: Vec<(Uuid, PendingResult<GetResultsResponse>)>,
    cache: HashMap<Uuid, GetResultsResponse>,
}

impl LeaderboardManager {
    /// Will start a task to fetch the results for that level in the background.
    /// You can retrieve the results later on using the `get_results` method.
    pub fn fetch_results(&mut self, level: Uuid) {
        if self.cache.contains_key(&level) {
            return;
        }

        trace!("Fetching histogram data for {level}");
        let path = LEADERBOARD_SERVER
            .join(&format!("{level}/results"))
            .unwrap();

        let promise = Promise::spawn_thread(
            "histogram_fetch",
            clone!([{ self.client } as client], move || {
                Ok(client.get(path.as_str()).call()?.into_json()?)
            }),
        );

        self.requests.push((level, promise));
    }

    pub fn get_results(&self, level: Uuid) -> Option<&GetResultsResponse> {
        self.cache.get(&level)
    }

    pub fn tick(&mut self) {
        let mut i = 0;
        while i < self.requests.len() {
            if self.requests[i].1.ready().is_none() {
                i += 1;
                continue;
            }

            let (id, req) = self.requests.remove(i);
            let req = match req.block_and_take() {
                Ok(x) => x,
                Err(err) => {
                    warn!("Error fetching histogram data for {id}: {err}");
                    continue;
                }
            };

            self.cache.insert(id, req);
        }
    }
}

impl Default for LeaderboardManager {
    fn default() -> Self {
        Self {
            client: AgentBuilder::new()
                .timeout(Duration::from_secs(15))
                .user_agent(concat!("beam-time/", env!("CARGO_PKG_VERSION")))
                .build(),
            requests: Vec::new(),
            cache: HashMap::new(),
        }
    }
}
