use afire::Server;

mod get_results;
mod put_results;

pub fn attach(server: &mut Server) {
    get_results::attach(server);
    put_results::attach(server);
}
