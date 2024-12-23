use afire::{extensions::RouteShorthands, Content, Server};
use serde_json::json;
use uuid::Uuid;

use leaderboard::api::results::GetResultsResponse;

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.get("/api/{level}/results", |ctx| {
        let level_id = ctx.param_idx(0).parse::<Uuid>()?;

        let app = ctx.app();

        // TODO: Check if level exists?
        let (cost, latency) = app.db.get_histogram(level_id)?;
        ctx.text(json!(GetResultsResponse { cost, latency }))
            .content(Content::JSON)
            .send()?;
        Ok(())
    });
}

// fn generate(data: &[u32]) -> Histogram {
//     const BIN_COUNT: usize = 12;

//     let max = data.iter().copied().max().unwrap_or_default();
//     let bin_width = max as f32 / BIN_COUNT as f32;

//     let mut bins = [0; BIN_COUNT];
//     for &point in data {
//         let bin = (point as f32 / bin_width) as usize;
//         bins[bin.min(BIN_COUNT - 1)] += 1;
//     }

//     Histogram { bins, max }
// }
