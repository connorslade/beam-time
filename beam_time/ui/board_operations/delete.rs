use engine::{
    drawable::{Anchor, spacer::Spacer},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::GraphicsContext,
    layout::{LayoutElement, LayoutMethods, column::ColumnLayout},
};

use crate::{
    consts::layer,
    ui::{
        board_operations::BoardType,
        components::modal::{Modal, modal_buttons},
        misc::{body, modal_size, spacing},
    },
};

pub enum Result {
    Nothing,
    Cancled,
    Deleted,
}

pub fn delete_modal(ctx: &mut GraphicsContext, mode: BoardType, name: &str) -> Result {
    let (margin, padding) = spacing(ctx);
    let modal = Modal::new(modal_size(ctx))
        .position(ctx.center(), Anchor::Center)
        .margin(margin)
        .layer(layer::UI_OVERLAY);
    let size = modal.inner_size();

    let mut out = Result::Nothing;
    modal.draw(ctx, |ctx, root| {
        let body = body(size.x);

        root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
            body("Deletion Confirmation")
                .scale(Vector2::repeat(4.0))
                .layout(ctx, layout);
            Spacer::new_y(4.0 * ctx.scale_factor).layout(ctx, layout);

            let text = format!(
                "Are you sure you want to delete the {} world '{name}'?",
                mode.type_name_lower()
            );
            body(&text).layout(ctx, layout);
            body("If so, it will be moved to your system trash.").layout(ctx, layout);

            let click = ctx.input.mouse_pressed(MouseButton::Left);
            let (back, delete) = modal_buttons(ctx, layout, size.x, ("Back", "Delete"));
            (ctx.input.consume_key_pressed(KeyCode::Escape) || back && click)
                .then(|| out = Result::Cancled);
            (delete && click).then(|| out = Result::Deleted);
        });
    });

    out
}

impl BoardType {
    fn type_name_lower(&self) -> &str {
        match self {
            BoardType::Sandbox => "sandbox",
            BoardType::Solution => "solution",
        }
    }
}
