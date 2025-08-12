use afire::{
    Content, Server,
    extensions::{RedirectResponse, RouteShorthands},
};

use crate::app::App;

const STEAM_PAGE: &str = "https://store.steampowered.com/app/3385920/Beam_Time";

pub fn attach(server: &mut Server<App>) {
    server.get("/", |ctx| {
        ctx.text(STEAM_PAGE)
            .content(Content::TXT)
            .redirect(STEAM_PAGE)
            .send()?;
        Ok(())
    });
}
