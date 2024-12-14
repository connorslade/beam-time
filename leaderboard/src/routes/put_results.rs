use std::{
    borrow::Cow,
    time::{SystemTime, UNIX_EPOCH},
};

use afire::{
    extensions::{RealIp, RouteShorthands},
    route::RouteContext,
    HeaderName, Server,
};
use beam_logic::{
    level::DEFAULT_LEVELS,
    misc::price,
    simulation::{level_state::LevelResult, runtime::testing::TestingSimulationState},
};
use leaderboard::api::{hmac::verify, results::PutResults};
use serde_json::json;
use uuid::Uuid;

use crate::{app::App, database::schema::results::Results};

pub fn attach(server: &mut Server<App>) {
    server.put("/api/{level}/results", |ctx| {
        let level_id = ctx.param_idx(0).parse::<Uuid>()?;

        let hash = ctx
            .req
            .headers
            .get(HeaderName::Authorization)
            .context("Authorization not provided")?;

        let hash = hex::decode(hash.as_bytes())?;
        verify(&ctx.req.body, &hash).context("Invalid authorization")?;

        let app = ctx.app();

        let body = serde_json::from_slice::<PutResults>(&ctx.req.body)?;

        let level = &DEFAULT_LEVELS[0];
        let results = TestingSimulationState::new(
            &body.board,
            Cow::Borrowed(level),
            app.config.simulation.max_ticks,
        )
        .run();

        ctx.text(json!(results)).send()?;

        if let LevelResult::Success { latency } = results {
            let cost = price(&body.board, Some(level));
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let transaction = app.db.rw_transaction()?;
            transaction.insert(Results {
                id: Uuid::new_v4().into(),

                user_id: body.user,
                ip_address: ctx.req.real_ip(),
                timestamp,

                level_id: level_id.into(),
                cost,
                latency,

                solution: body.board,
            })?;
            transaction.commit()?;
        }

        Ok(())
    });
}
