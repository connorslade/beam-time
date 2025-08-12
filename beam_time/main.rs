#![feature(decl_macro)]

use std::mem;

use anyhow::Result;
use common::consts::API_TESTING;
use engine::{
    application::{Application, ApplicationArgs},
    exports::winit::window::{Icon, WindowAttributes},
};
use env_logger::WriteStyle;
use log::{LevelFilter, warn};

mod app;
mod assets;
mod consts;
mod game;
mod leaderboard;
mod screens;
#[cfg(feature = "steam")]
mod steam;
pub mod ui;
mod util;

use app::App;
use screens::{Screens, debug_overlay::DebugOverlay, title::TitleScreen};
use util::include_atlas;

fn main() -> Result<()> {
    env_logger::builder()
        .filter(Some("beam_time"), LevelFilter::Trace)
        .filter(Some("beam_logic"), LevelFilter::Trace)
        .write_style(WriteStyle::Always)
        .init();
    API_TESTING.then(|| warn!("Using test API key!"));

    let icon = Icon::from_rgba(include_atlas!("textures/icon.png").into_vec(), 32, 32)?;
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default()
            .with_title(concat!("Beam Time v", env!("CARGO_PKG_VERSION")))
            .with_window_icon(Some(icon))
            .with_maximized(true),
        asset_constructor: Box::new(assets::init),
        resumed: Box::new(|| {
            let mut app = App::new();
            let mut screens = Screens::new(vec![
                Box::new(DebugOverlay::default()),
                Box::new(TitleScreen::default()),
            ]);
            screens.top().unwrap().on_init(&mut app);

            Box::new(move |ctx| {
                app.on_tick(ctx);

                screens.render(ctx, &mut app);
                screens.pop_n(mem::take(&mut app.close_screens), &mut app);
                screens.extend(mem::take(&mut app.new_screens), &mut app);

                ctx.window
                    .close_requested()
                    .then(|| screens.destroy(&mut app));
            })
        }),
        ..Default::default()
    })
    .run()
}
