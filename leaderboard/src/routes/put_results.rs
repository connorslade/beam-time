use std::borrow::Cow;

use afire::{extensions::RouteShorthands, route::RouteContext, HeaderName, Server};
use beam_logic::{level::DEFAULT_LEVELS, simulation::runtime::testing::TestingSimulationState};
use leaderboard::api::{hmac::verify, results::PutResults};
use serde_json::json;
use uuid::Uuid;

pub fn attach(server: &mut Server) {
    server.put("/api/{level}/results", |ctx| {
        let level = ctx.param_idx(0).parse::<Uuid>()?;

        let hash = ctx
            .req
            .headers
            .get(HeaderName::Authorization)
            .context("Authorization not provided")?;

        let hash = hex::decode(hash.as_bytes())?;
        verify(&ctx.req.body, &hash).context("Invalid authorization")?;

        let body = serde_json::from_slice::<PutResults>(&ctx.req.body)?;

        let results =
            TestingSimulationState::new(&body.board, Cow::Borrowed(&DEFAULT_LEVELS[0]), 500).run();

        ctx.text(json!(results)).send()?;

        Ok(())
    });
}
