use afire::Server;

use crate::app::App;

mod get_results;
mod get_root;
mod put_results;

pub fn attach(server: &mut Server<App>) {
    get_results::attach(server);
    put_results::attach(server);
    get_root::attach(server);
}
