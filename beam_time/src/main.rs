#![feature(decl_macro)]

use anyhow::Result;
#[cfg(target_arch = "wasm32")]
use engine::exports::winit::platform::web::WindowAttributesExtWebSys;
use engine::{
    application::{Application, ApplicationArgs},
    exports::winit::{
        dpi::PhysicalSize,
        window::{Icon, WindowAttributes},
    },
};

mod app;
mod assets;
mod consts;
mod game;
mod misc;
mod screens;
mod ui;
mod util;

use app::App;
use consts::DEFAULT_SIZE;
use env_logger::WriteStyle;
use log::LevelFilter;
use screens::{overlay::debug::DebugOverlay, title::TitleScreen};
use util::include_atlas;
use wasm_bindgen::prelude::wasm_bindgen;

async fn start() -> Result<()> {
    // env_logger::builder()
    //     .filter(Some("beam_time"), LevelFilter::Trace)
    //     .write_style(WriteStyle::Always)
    //     .init();

    let icon = Icon::from_rgba(include_atlas!("icon.png").into_vec(), 30, 30)?;
    Application::new(ApplicationArgs {
        window_attributes: {
            let mut attr = WindowAttributes::default()
                .with_title("Beam Time")
                .with_window_icon(Some(icon)); //.with_inner_size(PhysicalSize::new(DEFAULT_SIZE.0, DEFAULT_SIZE.1));
            #[cfg(target_arch = "wasm32")]
            {
                attr = attr.with_append(true)
            }
            attr
        },
        app_constructor: Box::new(App::new),
        screen_constructor: Box::new(|| {
            vec![
                Box::new(DebugOverlay::default()),
                Box::new(TitleScreen::default()),
            ]
        }),
        asset_constructor: Box::new(assets::init),
    })
    .await?;

    // .run()

    Ok(())
}

#[wasm_bindgen]
pub async fn wasm_main() {
    if let Err(err) = start().await {
        wasm_bindgen::throw_str(&format!("{:?}", err));
    }
}

#[pollster::main]
async fn main() -> Result<()> {
    start().await
}
