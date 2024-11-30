use afire::{extensions::RouteShorthands, Server};
use uuid::Uuid;

pub fn attach(server: &mut Server) {
    server.put("/api/{level}/results", |ctx| {
        let level = ctx.param_idx(0).parse::<Uuid>()?;
        ctx.text(format!("Updating results for level {level:?}."))
            .send()?;
        Ok(())
    });
}
