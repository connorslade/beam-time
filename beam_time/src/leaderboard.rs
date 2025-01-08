use std::{io::Cursor, thread, time::Duration};

use ahash::{HashMap, HashMapExt};
use anyhow::{Context, Result};
use bincode::Options;
use clone_macro::clone;
use log::{trace, warn};
use poll_promise::Promise;
use ureq::{Agent, AgentBuilder};
use url::Url;
use uuid::Uuid;

use crate::consts::LEADERBOARD_SERVER;
use beam_logic::{simulation::level_state::LevelResult, tile::Tile};
use common::{consts::BINCODE_OPTIONS, map::Map, user::UserId};
use leaderboard::api::{
    hmac::hash,
    results::{GetResultsResponse, PutResultsRef},
};

type PendingResult<T> = Promise<Result<T>>;

pub struct LeaderboardManager {
    client: Agent,

    requests: Vec<(Uuid, PendingResult<GetResultsResponse>)>,
    cache: HashMap<Uuid, GetResultsResponse>,
}

impl LeaderboardManager {
    pub fn publish_solution(&self, user: &UserId, level: Uuid, board: &Map<Tile>) {
        let path = results_path(level);
        let body = BINCODE_OPTIONS
            .serialize(&PutResultsRef { user, board })
            .unwrap();

        thread::spawn(clone!([{ self.client } as client], move || {
            let auth = hex::encode(hash(&body));
            let resp = client
                .put(path.as_str())
                .set("Authorization", &auth)
                .set("Content-Length", body.len().to_string().as_str())
                .send(Cursor::new(body))
                .with_context(|| format!("Error publishing solution for {level}"))
                .and_then(|x| {
                    x.into_json::<LevelResult>()
                        .context("Error deserializing result")
                });

            match resp {
                Ok(resp) => trace!("Solution published for {level}: {resp:?}"),
                Err(e) => warn!("{e}"),
            }
        }));
    }

    /// Will start a task to fetch the results for that level in the background.
    /// You can retrieve the results later on using the `get_results` method.
    pub fn fetch_results(&mut self, level: Uuid) {
        if self.cache.contains_key(&level) {
            return;
        }

        trace!("Fetching histogram data for {level}");
        let path = results_path(level);

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

fn results_path(level: Uuid) -> Url {
    LEADERBOARD_SERVER
        .join(&format!("{level}/results"))
        .unwrap()
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
