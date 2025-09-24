use engine::{
    drawable::{Anchor, Drawable},
    drawable::{sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    layout::{Justify, LayoutElement, LayoutMethods, column::ColumnLayout, root::RootLayout},
};

use crate::{
    app::App,
    assets::{DOWN_ARROW, UNDEAD_FONT},
    consts::{layer, spacing::PADDING},
    game::{board::Board, pancam::Pancam},
};

impl Board {
    pub fn render_notes(&mut self, ctx: &mut GraphicsContext, _state: &App, pancam: &Pancam) {
        for note in self.notes.iter() {
            let pos = pancam.world_to_screen_space(note.position);

            let mut root = RootLayout::new(pos, Anchor::BottomCenter);

            root.nest(
                ctx,
                ColumnLayout::new(PADDING).justify(Justify::Center),
                |ctx, layout| {
                    Text::new(UNDEAD_FONT, &note.title)
                        .scale(Vector2::repeat(2.0))
                        .z_index(layer::OVERLAY)
                        .layout(ctx, layout);

                    if pancam.scale >= 6.0 && !note.body.is_empty() {
                        Text::new(UNDEAD_FONT, &note.body)
                            .max_width(16.0 * 20.0)
                            .scale(Vector2::repeat(2.0))
                            .z_index(layer::OVERLAY)
                            .layout(ctx, layout);
                    }

                    Sprite::new(DOWN_ARROW)
                        .scale(Vector2::repeat(2.0))
                        .z_index(layer::OVERLAY)
                        .layout(ctx, layout);
                },
            );

            root.draw(ctx);
        }
    }
}
