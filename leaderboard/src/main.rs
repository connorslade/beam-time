use std::process;

use afire::{trace, trace::Level, Middleware, Server};
use anyhow::Result;

mod logger;
use app::App;
use env_logger::WriteStyle;
use log::{info, LevelFilter};
use logger::{AfireLogger, RequestLogger};
mod app;
mod config;
mod database;
mod routes;

fn main() -> Result<()> {
    trace::set_log_level(Level::Trace);
    trace::set_log_formatter(AfireLogger);

    env_logger::builder()
        .filter(None, LevelFilter::Trace)
        .write_style(WriteStyle::Always)
        .init();

    let app = App::new()?;
    let mut server = Server::<App>::new(&app.config.server.host, app.config.server.port)
        .workers(app.config.server.threads)
        .state(app);

    RequestLogger.attach(&mut server);
    routes::attach(&mut server);

    let app = server.app();
    ctrlc::set_handler(move || {
        info!("Exiting");
        app.db.cleanup().unwrap();
        process::exit(0);
    })
    .unwrap();

    server.run()?;
    Ok(())
}
