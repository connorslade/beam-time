use afire::{extensions::RouteShorthands, Content, Server};
use serde_json::json;
use uuid::Uuid;

use leaderboard::api::results::{GetResultsResponse, Histogram};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.get("/api/{level}/results", |ctx| {
        let _level = ctx.param_idx(0).parse::<Uuid>()?;

        let hist = Histogram {
            bins: [3, 10, 22, 33, 35, 51, 49, 38, 26, 21, 9, 3],
            max: 100,
        };
        let fake = GetResultsResponse {
            cost: hist.clone(),
            latency: hist,
        };

        ctx.text(json!(fake)).content(Content::JSON).send()?;
        Ok(())
    });
}

// let max = *data.iter().max().unwrap();
// let bin_width = max as f32 / BIN_COUNT as f32;

// let mut bins = [0; BIN_COUNT];
// for point in data {
//     let bin = (point as f32 / bin_width) as usize;
//     bins[bin.min(BIN_COUNT - 1)] += 1;
// }

// let max_count = *bins.iter().max().unwrap();
