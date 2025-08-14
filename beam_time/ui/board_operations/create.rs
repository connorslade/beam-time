use engine::{
    drawable::{Anchor, spacer::Spacer},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
    layout::{LayoutElement, LayoutMethods, column::ColumnLayout},
    memory::MemoryKey,
    memory_key,
};

use crate::{
    consts::{
        color, layer,
        spacing::{MARGIN, PADDING},
    },
    ui::{
        board_operations::BoardType,
        components::{
            modal::{Modal, modal_buttons},
            text_input::TextInput,
        },
        misc::{body, modal_size},
    },
};

const NAME_KEY: MemoryKey = memory_key!();

#[derive(Clone)]
pub enum Result {
    Nothing,
    Cancled,
    Finished(String),
}

pub fn create_modal(ctx: &mut GraphicsContext, mode: BoardType, start: Option<&str>) -> Result {
    let mut exit = ctx.input.consume_key_pressed(KeyCode::Escape);
    let mut enter = ctx.input.consume_key_pressed(KeyCode::Enter);

    let edit = start.is_some();
    let mut name_error = false;

    let modal = Modal::new(modal_size(ctx))
        .position(ctx.center(), Anchor::Center)
        .margin(MARGIN)
        .layer(layer::UI_OVERLAY);

    let size = modal.inner_size();
    modal.draw(ctx, |ctx, root| {
        let body = body(size.x);

        root.nest(ctx, ColumnLayout::new(PADDING), |ctx, layout| {
            body(mode.title(edit))
                .scale(Vector2::repeat(4.0))
                .layout(ctx, layout);
            body(mode.body(edit)).layout(ctx, layout);
            Spacer::new_y(8.0).layout(ctx, layout);

            body(&format!("{} Name", mode.type_name())).layout(ctx, layout);

            let name = TextInput::new(NAME_KEY)
                .default_active(true)
                .width(size.x.min(400.0));
            if let Some(start) = start
                && !name.is_edited(ctx)
            {
                name.with_content(ctx, start.to_owned());
            }

            let content = name.content(ctx);
            name.layout(ctx, layout);

            let no_name = content.is_empty();
            let invalid_name = content
                .chars()
                .any(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '-' | '_'));

            let checkers = [
                (no_name, mode.no_name_error(edit)),
                (invalid_name, mode.invalid_name_error()),
            ];
            for (_, error) in checkers.iter().filter(|(predicate, _)| *predicate) {
                body(error).color(color::ERROR).layout(ctx, layout);
                name_error = true;
            }

            let action = ["Create", "Rename"][edit as usize];
            let (back, create) = modal_buttons(ctx, layout, size.x, ("Back", action));
            let click = ctx.input.mouse_pressed(MouseButton::Left);
            enter |= create && !name_error && click;
            exit |= back && click;

            (create && name_error).then(|| ctx.window.cursor(CursorIcon::NotAllowed));
        });
    });

    if enter && !name_error {
        let name = TextInput::content_for(ctx, NAME_KEY);
        return Result::Finished(name);
    }

    if exit {
        Result::Cancled
    } else {
        Result::Nothing
    }
}

impl BoardType {
    fn type_name(&self) -> &str {
        match self {
            BoardType::Sandbox => "Sandbox",
            BoardType::Solution => "Solution",
        }
    }

    fn title(&self, edit: bool) -> &str {
        match (self, edit) {
            (BoardType::Sandbox, false) => "New Sandbox",
            (BoardType::Sandbox, true) => "Edit Sandbox",
            (BoardType::Solution, true) => "Edit Solution",
            _ => unreachable!(),
        }
    }

    fn body(&self, edit: bool) -> &str {
        match (self, edit) {
            (BoardType::Sandbox, false) => {
                "Choose a name for your new sandbox then click 'Create' or press enter."
            }
            (BoardType::Sandbox, true) => {
                "Change the name of your sandbox, then click 'Rename' or press enter."
            }
            (BoardType::Solution, true) => {
                "Change the name of your solution, then click 'Rename' or press enter."
            }
            _ => unreachable!(),
        }
    }

    fn no_name_error(&self, edit: bool) -> &str {
        match (self, edit) {
            (BoardType::Sandbox, false) => "Please enter a name for your new sandbox.",
            (BoardType::Sandbox, true) => "Please enter a name for your sandbox.",
            (BoardType::Solution, true) => "Please enter a name for your solution.",
            _ => unreachable!(),
        }
    }

    fn invalid_name_error(&self) -> &str {
        match self {
            BoardType::Sandbox => {
                "Only alphanumeric characters, spaces, dashes, and underscores can be used in sandbox names."
            }
            BoardType::Solution => {
                "Only alphanumeric characters, spaces, dashes, and underscores can be used in solution names."
            }
        }
    }
}
