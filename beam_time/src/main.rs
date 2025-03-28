#![feature(decl_macro)]

use anyhow::Result;
use engine::{
    application::{Application, ApplicationArgs},
    exports::winit::{
        dpi::PhysicalSize,
        window::{Icon, WindowAttributes},
    },
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
mod ui;
mod util;

use app::App;
#[cfg(feature = "debug")]
use screens::overlay::debug::DebugOverlay;
use screens::{overlay::ticker::Ticker, title::TitleScreen};
use util::include_atlas;

fn main() -> Result<()> {
    env_logger::builder()
        .filter(Some("beam_time"), LevelFilter::Trace)
        .write_style(WriteStyle::Always)
        .init();

    let icon = Icon::from_rgba(include_atlas!("icon.png").into_vec(), 30, 30)?;
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default()
            .with_title("Beam Time")
            .with_window_icon(Some(icon))
            .with_inner_size(PhysicalSize::new(1920, 1080)),
        app_constructor: Box::new(App::new),
        screen_constructor: Box::new(|| {
            vec![
                Box::new(Ticker),
                #[cfg(feature = "debug")]
                Box::new(DebugOverlay::default()),
                Box::new(TitleScreen::default()),
            ]
        }),
        asset_constructor: Box::new(assets::init),
    })
    .run()
}
