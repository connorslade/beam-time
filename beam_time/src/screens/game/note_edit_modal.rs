use engine::{
    color::Rgb,
    drawable::text::Text,
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    layout::{column::ColumnLayout, LayoutElement, LayoutMethods},
    memory_key,
};

use crate::{
    app::App,
    assets::UNDEAD_FONT,
    consts::layer,
    ui::components::{modal::Modal, text_input::TextInput},
};

use super::GameScreen;

pub struct NoteEditModal {}

impl GameScreen {
    pub(super) fn note_edit_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        if let Some(_note) = &mut self.note_edit {
            ctx.defer(|ctx| ctx.darken(Rgb::repeat(0.5), layer::UI_OVERLAY));

            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(Vector2::new(ctx.center().x, 500.0))
                .margin(margin)
                .layer(layer::UI_OVERLAY);

            let size = modal.inner_size();
            modal.draw(ctx, |ctx, root| {
                let body = |text| {
                    Text::new(UNDEAD_FONT, text)
                        .scale(Vector2::repeat(2.0))
                        .max_width(size.x)
                };

                root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                    body("Editing Note")
                        .scale(Vector2::repeat(4.0))
                        .layout(ctx, layout);

                    TextInput::new(memory_key!())
                        .width(size.x.min(400.0 * ctx.scale_factor))
                        .layout(ctx, layout);
                });
            });
        }
    }
}
