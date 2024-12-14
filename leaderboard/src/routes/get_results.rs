use afire::{extensions::RouteShorthands, Content, Server};
use serde_json::json;
use uuid::Uuid;

use leaderboard::api::results::{GetResultsResponse, Histogram};

use crate::{
    app::App,
    database::schema::results::{Results, ResultsKey},
};

pub fn attach(server: &mut Server<App>) {
    server.get("/api/{level}/results", |ctx| {
        let level_id = ctx.param_idx(0).parse::<Uuid>()?;

        let app = ctx.app();
        let transaction = app.db.r_transaction()?;
        let data = transaction
            .scan()
            .secondary::<Results>(ResultsKey::level_id)?
            .all()?
            .filter_map(|x| x.ok())
            .filter(|x| *x.level_id == level_id)
            .collect::<Vec<_>>();

        let response = GetResultsResponse {
            cost: generate(&data.iter().map(|x| x.cost).collect::<Vec<_>>()),
            latency: generate(&data.iter().map(|x| x.latency).collect::<Vec<_>>()),
        };

        ctx.text(json!(response)).content(Content::JSON).send()?;
        Ok(())
    });
}

fn generate(data: &[u32]) -> Histogram {
    const BIN_COUNT: usize = 12;

    let max = data.iter().copied().max().unwrap_or_default();
    let bin_width = max as f32 / BIN_COUNT as f32;

    let mut bins = [0; BIN_COUNT];
    for &point in data {
        let bin = (point as f32 / bin_width) as usize;
        bins[bin.min(BIN_COUNT - 1)] += 1;
    }

    Histogram { bins, max }
}
