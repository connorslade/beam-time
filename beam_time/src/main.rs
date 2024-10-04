#![feature(decl_macro)]

use anyhow::Result;
use engine::{
    application::{Application, ApplicationArgs},
    exports::winit::{
        dpi::PhysicalSize,
        window::{Icon, WindowAttributes, WindowButtons},
    },
};

mod app;
mod assets;
mod consts;
mod game;
mod screens;
mod ui;
mod util;

use app::App;
use consts::DEFAULT_SIZE;
use screens::{overlay::debug::DebugOverlay, title::TitleScreen};
use util::include_atlas;

fn main() -> Result<()> {
    let icon = Icon::from_rgba(include_atlas!("icon.png").into_vec(), 16, 16)?;
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default()
            .with_title("Beam Time")
            .with_window_icon(Some(icon))
            .with_inner_size(PhysicalSize::new(DEFAULT_SIZE.0, DEFAULT_SIZE.1))
            .with_resizable(false)
            .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE),
        app_constructor: Box::new(App::default),
        screen_constructor: Box::new(|| {
            vec![
                Box::new(DebugOverlay::default()),
                Box::new(TitleScreen::default()),
            ]
        }),
        asset_constructor: Box::new(assets::init),
    })
    .run()
}
