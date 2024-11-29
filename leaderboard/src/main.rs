use afire::{extensions::RouteShorthands, trace, trace::Level, Server};
use uuid::Uuid;

fn main() {
    trace::set_log_level(Level::Trace);

    let mut server = Server::<()>::new("localhost", 8080);

    server.get("/api/{level}/results", |ctx| {
        let level = ctx.param_idx(0).parse::<Uuid>()?;
        ctx.text(format!("Returning results for level {level:?}."))
            .send()?;
        Ok(())
    });

    server.put("/api/{level}/results", |ctx| {
        let level = ctx.param_idx(0).parse::<Uuid>()?;
        ctx.text(format!("Updating results for level {level:?}."))
            .send()?;
        Ok(())
    });

    server.run().unwrap();
}
