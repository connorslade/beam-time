use std::borrow::Cow;

use afire::{extensions::RouteShorthands, route::RouteContext, HeaderName, Server};
use beam_logic::{level::DEFAULT_LEVELS, simulation::runtime::testing::TestingSimulationState};
use leaderboard::api::{hmac::verify, results::PutResults};
use serde_json::json;
use uuid::Uuid;

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.put("/api/{level}/results", |ctx| {
        let level = ctx.param_idx(0).parse::<Uuid>()?;

        let hash = ctx
            .req
            .headers
            .get(HeaderName::Authorization)
            .context("Authorization not provided")?;

        let hash = hex::decode(hash.as_bytes())?;
        verify(&ctx.req.body, &hash).context("Invalid authorization")?;

        let app = ctx.app();

        let body = serde_json::from_slice::<PutResults>(&ctx.req.body)?;

        let results = TestingSimulationState::new(
            &body.board,
            Cow::Borrowed(&DEFAULT_LEVELS[0]),
            app.config.simulation.max_ticks,
        )
        .run();

        ctx.text(json!(results)).send()?;

        Ok(())
    });
}
