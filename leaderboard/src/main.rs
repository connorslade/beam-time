use afire::{
    extensions::Logger,
    trace::{self, Level},
    Middleware, Server,
};

mod routes;

fn main() {
    trace::set_log_level(Level::Trace);

    let mut server = Server::<()>::new("localhost", 8080)
        .keep_alive(false)
        .workers(16);

    Logger::new().attach(&mut server);
    routes::attach(&mut server);

    server.run().unwrap();
}
