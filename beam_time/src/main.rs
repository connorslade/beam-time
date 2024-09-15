use std::time::Instant;

use anyhow::Result;
use screens::title::TitleScreen;

use engine::{
    application::{Application, ApplicationArgs},
    exports::winit::{
        dpi::PhysicalSize,
        window::{Icon, WindowAttributes},
    },
};

mod assets;
mod consts;
mod screens;
mod util;
use consts::DEFAULT_SIZE;

fn main() -> Result<()> {
    let icon = Icon::from_rgba(include_atlas!("icon.png").into_vec(), 16, 16)?;
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default()
            .with_title("Beam Time")
            .with_window_icon(Some(icon))
            .with_inner_size(PhysicalSize::new(DEFAULT_SIZE.0, DEFAULT_SIZE.1))
            .with_resizable(false),
        screen_constructor: Box::new(|| {
            Box::new(TitleScreen {
                last_update: Instant::now(),
                frames: 0,
                last_frames: 0,
            })
        }),
        asset_constructor: Box::new(assets::init),
    })
    .run()
}
