use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
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
use log::error;
use slug::slugify;

use crate::{
    app::App,
    consts::{ERROR_COLOR, layer},
    game::board::Board,
    screens::{
        game::GameScreen,
        sandbox::{ActiveModal, SandboxScreen},
    },
    ui::{
        components::{
            modal::{Modal, modal_buttons},
            text_input::TextInput,
        },
        misc::body,
    },
};

const NEW_SANDBOX_TEXT: &str =
    "Choose a name for your new sandbox then click 'Create' or press enter.";
const EDIT_SANDBOX_TEXT: &str =
    "Change the name of your sandbox, then click 'Rename' or press enter.";

const INVALID_NAME_TEXT: &str =
    "Only alphanumeric characters, spaces, dashes, and underscores can be used in sandbox names.";
const NO_NAME_TEXT: &str = "Please enter a name for your new sandbox.";

const NAME_KEY: MemoryKey = memory_key!();

impl SandboxScreen {
    pub(super) fn create_modal(
        &mut self,
        state: &mut App,
        ctx: &mut GraphicsContext,
        edit: Option<usize>,
    ) {
        let mut exit = ctx.input.consume_key_pressed(KeyCode::Escape);
        let mut enter = ctx.input.consume_key_pressed(KeyCode::Enter);

        let (margin, padding) = state.spacing(ctx);
        let modal = Modal::new(state.modal_size(ctx))
            .position(ctx.center(), Anchor::Center)
            .margin(margin)
            .layer(layer::OVERLAY);

        let size = modal.inner_size();
        let mut name_error = false;
        modal.draw(ctx, |ctx, root| {
            let body = body(size.x);

            root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                let is_edit = edit.is_some() as usize;
                body(["New Sandbox", "Edit Sandbox"][is_edit])
                    .scale(Vector2::repeat(4.0))
                    .layout(ctx, layout);
                body([NEW_SANDBOX_TEXT, EDIT_SANDBOX_TEXT][is_edit]).layout(ctx, layout);
                Spacer::new_y(8.0 * ctx.scale_factor).layout(ctx, layout);

                body("Sandbox Name").layout(ctx, layout);

                let name = TextInput::new(NAME_KEY)
                    .default_active(true)
                    .width(size.x.min(400.0 * ctx.scale_factor));
                if let Some(edit) = edit
                    && !name.is_edited(ctx)
                {
                    let board = &self.worlds[edit];
                    name.with_content(ctx, board.meta.name.clone());
                }
                let content = name.content(ctx);
                name.layout(ctx, layout);

                let no_name = content.is_empty();
                let invalid_name = content
                    .chars()
                    .any(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '-' | '_'));

                let checkers = [(no_name, NO_NAME_TEXT), (invalid_name, INVALID_NAME_TEXT)];
                for (_, error) in checkers.iter().filter(|(predicate, _)| *predicate) {
                    body(error).color(ERROR_COLOR).layout(ctx, layout);
                    name_error = true;
                }

                let (back, create) =
                    modal_buttons(ctx, layout, size.x, ("Back", ["Create", "Rename"][is_edit]));
                let click = ctx.input.mouse_pressed(MouseButton::Left);
                enter |= create && !name_error && click;
                exit |= back && click;

                (create && name_error).then(|| ctx.window.cursor(CursorIcon::NotAllowed));
            });
        });

        if enter && !name_error {
            let name = TextInput::content_for(ctx, NAME_KEY);
            let path = self.world_dir.join(slugify(&name)).with_extension("bin");
            if let Some(edit) = edit {
                let path = &self.worlds[edit].path;
                if let Err(err) = rename_board(path, name) {
                    error!("Error renaming board: {err}");
                }
            } else {
                let board = Board::new_sandbox(name);
                let screen = GameScreen::new(board, path);
                state.push_screen(screen);
            }
        }

        (exit || enter).then(|| self.modal = ActiveModal::None);
    }
}

fn rename_board(world: &PathBuf, name: String) -> Result<()> {
    let mut board = Board::load(world)?;
    board.meta.playtime = 0;
    board.meta.name = name;

    let world_dir = world.parent().context("No parent")?;
    let path = world_dir.join(format!("{}.ron", slugify(&board.meta.name)));
    board.save_exact(&path)?;

    fs::remove_file(world)?;
    Ok(())
}
