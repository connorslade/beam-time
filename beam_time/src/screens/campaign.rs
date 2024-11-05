use std::{collections::HashMap, path::PathBuf};

use engine::{
    color::Rgb,
    drawable::text::Text,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};
use uuid::Uuid;

use crate::{
    app::App,
    assets::UNDEAD_FONT,
    game::{
        board::{Board, BoardMeta},
        level::LEVELS,
    },
    ui::{
        button::ButtonState,
        misc::{font_scale, titled_screen},
    },
    util::in_bounds,
};

#[derive(Default)]
pub struct CampaignScreen {
    back_button: ButtonState,
    worlds: HashMap<Uuid, (PathBuf, BoardMeta)>,
}

impl Screen<App> for CampaignScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        titled_screen(state, ctx, &mut self.back_button, "Campaign");

        const SCALE: f32 = 3.0;
        let (line_height, line_spacing, total_height) =
            font_scale(ctx, UNDEAD_FONT, SCALE, LEVELS.len());

        for (i, level) in LEVELS.iter().enumerate() {
            let pos =
                ctx.center() + Vector2::new(0.0, total_height / 2.0 - line_spacing * i as f32);

            let mut text = Text::new(UNDEAD_FONT, &level.name)
                .pos(pos, Anchor::Center)
                .scale(Vector2::repeat(SCALE));

            let width = text.width(ctx) * SCALE;
            let half_size = Vector2::new(width / 2.0, line_height / 2.0) * ctx.scale_factor;
            let hovered = in_bounds(ctx.input.mouse, (pos - half_size, pos + half_size));
            if hovered {
                text = text.color(Rgb::new(0.5, 0.5, 0.5));

                if ctx.input.mouse_pressed(MouseButton::Left) {
                    // ctx.push_screen(GameScreen::new(Board{}));
                }
            }

            ctx.draw(text);
        }
    }
}
