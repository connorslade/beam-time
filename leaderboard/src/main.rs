use std::process;

use afire::{extensions::Logger, trace, trace::Level, Middleware, Server};
use anyhow::Result;

use app::App;
mod app;
mod config;
mod database;
mod routes;

fn main() -> Result<()> {
    trace::set_log_level(Level::Trace);

    let app = App::new()?;
    let mut server = Server::<App>::new(&app.config.server.host, app.config.server.port)
        .workers(app.config.server.threads)
        .state(app);

    Logger::new().attach(&mut server);
    routes::attach(&mut server);

    let app = server.app();
    ctrlc::set_handler(move || {
        trace!("Exiting");
        app.db.cleanup().unwrap();
        process::exit(0);
    })
    .unwrap();

    server.run()?;
    Ok(())
}
