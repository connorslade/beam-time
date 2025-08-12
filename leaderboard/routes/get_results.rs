use afire::{Content, Server, extensions::RouteShorthands};
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
