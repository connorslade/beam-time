#![feature(decl_macro)]

use std::mem;

use anyhow::Result;
use engine::{
    application::{Application, ApplicationArgs},
    exports::winit::window::{Icon, WindowAttributes},
};
use env_logger::WriteStyle;
use log::LevelFilter;

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
#[cfg(feature = "debug")]
use screens::debug_overlay::DebugOverlay;
use screens::{Screens, title::TitleScreen};
use util::include_atlas;

fn main() -> Result<()> {
    env_logger::builder()
        .filter(Some("beam_time"), LevelFilter::Trace)
        .filter(Some("beam_logic"), LevelFilter::Trace)
        .write_style(WriteStyle::Always)
        .init();

    let icon = Icon::from_rgba(include_atlas!("textures/icon.png").into_vec(), 30, 30)?;
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default()
            .with_title("Beam Time")
            .with_window_icon(Some(icon)),
        asset_constructor: Box::new(assets::init),
        resumed: Box::new(|| {
            let mut app = App::new();
            let mut screens = Screens::new(vec![
                #[cfg(feature = "debug")]
                Box::new(DebugOverlay::default()),
                Box::new(TitleScreen::default()),
            ]);
            screens.top().on_init(&mut app);

            Box::new(move |ctx| {
                app.on_tick(ctx);
                if let Some(old_size) = ctx.input.resized {
                    screens.on_resize(old_size.map(|x| x as f32), ctx.size(), &mut app);
                }

                screens.render(ctx, &mut app);
                screens.pop_n(mem::take(&mut app.close_screens), &mut app);
                screens.extend(mem::take(&mut app.new_screens), &mut app);
            })
        }),
        ..Default::default()
    })
    .run()
}
