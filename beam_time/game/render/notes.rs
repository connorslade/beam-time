use engine::{
    drawable::{sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{column::ColumnLayout, root::RootLayout, Justify, LayoutElement, LayoutMethods},
};

use crate::{
    app::App,
    assets::{HISTOGRAM_MARKER, UNDEAD_FONT},
    consts::layer,
    game::{board::Board, pancam::Pancam},
};

impl Board {
    pub fn render_notes(&mut self, ctx: &mut GraphicsContext, state: &App, pancam: &Pancam) {
        for note in self.notes.iter() {
            let pos = pancam.world_to_screen_space(ctx, note.position);

            let (_, padding) = state.spacing(ctx);
            let mut root = RootLayout::new(pos, Anchor::BottomCenter);

            root.nest(
                ctx,
                ColumnLayout::new(padding).justify(Justify::Center),
                |ctx, layout| {
                    Text::new(UNDEAD_FONT, &note.title)
                        .scale(Vector2::repeat(2.0))
                        .z_index(layer::OVERLAY)
                        .layout(ctx, layout);

                    if pancam.scale >= 6.0 && !note.body.is_empty() {
                        Text::new(UNDEAD_FONT, &note.body)
                            .max_width(16.0 * 20.0 * ctx.scale_factor)
                            .scale(Vector2::repeat(2.0))
                            .z_index(layer::OVERLAY)
                            .layout(ctx, layout);
                    }

                    Sprite::new(HISTOGRAM_MARKER)
                        .scale(Vector2::repeat(2.0))
                        .z_index(layer::OVERLAY)
                        .layout(ctx, layout);
                },
            );

            root.draw(ctx);
        }
    }
}
