use std::{f32::consts::PI, path::PathBuf};

use common::misc::in_bounds;
use engine::{
    color::Rgb,
    drawable::{spacer::Spacer, sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode, window::CursorIcon},
    },
    graphics_context::{Anchor, GraphicsContext},
    layout::{column::ColumnLayout, LayoutElement, LayoutMethods},
    memory::MemoryKey,
    memory_key,
};

use crate::{
    assets::{BACK_BUTTON, CREATE_BUTTON, LEVEL_DROPDOWN_ARROW, UNDEAD_FONT},
    consts::{layer, ERROR_COLOR},
    game::board::{Board, BoardMeta},
    screens::game::GameScreen,
    ui::{
        components::{
            button::Button,
            modal::{modal_buttons, Modal},
            text_input::TextInput,
        },
        misc::{body, font_scale, titled_screen},
    },
    util::load_level_dir,
    App,
};

use super::Screen;

#[derive(Default)]
pub struct SandboxScreen {
    world_dir: PathBuf,
    worlds: Vec<(PathBuf, BoardMeta)>,

    dropdown_angle: f32,
    create: Option<CreateModal>,
}

#[derive(Default)]
pub struct CreateModal {}

impl Screen for SandboxScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        titled_screen(state, ctx, None, "Sandbox");

        self.create_modal(state, ctx);

        if self.worlds.is_empty() {
            ctx.draw(
                Text::new(UNDEAD_FONT, "No worlds...")
                    .position(ctx.center(), Anchor::Center)
                    .scale(Vector2::repeat(4.0)),
            );
        } else {
            const SCALE: f32 = 3.0;
            let (line_height, line_spacing, total_height) =
                font_scale(ctx, UNDEAD_FONT, SCALE, self.worlds.len());

            for (i, (world, meta)) in self.worlds.iter().enumerate() {
                let pos =
                    ctx.center() + Vector2::new(0.0, total_height / 2.0 - line_spacing * i as f32);

                let text = format!("{} . . . . . . . . . . .", meta.name);
                let mut text = Text::new(UNDEAD_FONT, &text)
                    .position(pos, Anchor::Center)
                    .scale(Vector2::repeat(SCALE));

                let size = text.size(ctx);
                let half_size = Vector2::new(size.x / 2.0, line_height / 2.0) * ctx.scale_factor;
                let hovered = in_bounds(ctx.input.mouse, (pos - half_size, pos + half_size));
                if hovered {
                    text = text.color(Rgb::new(0.5, 0.5, 0.5));

                    if ctx.input.mouse_pressed(MouseButton::Left) {
                        state.push_screen(GameScreen::load(world.clone()));
                    }
                }

                ctx.draw(text);

                let dropdown = Sprite::new(LEVEL_DROPDOWN_ARROW)
                    .scale(Vector2::repeat(4.0))
                    .position(
                        pos + Vector2::new(
                            size.x / 2.0 + 16.0 * ctx.scale_factor,
                            -4.0 * ctx.scale_factor,
                        ),
                        Anchor::CenterLeft,
                    )
                    .rotate(-self.dropdown_angle, Anchor::Center);

                self.dropdown_angle =
                    if in_bounds(ctx.input.mouse, (pos - size / 2.0, pos + size / 2.0)) {
                        self.dropdown_angle + (PI / 2.0) * ctx.delta_time * 4.0
                    } else {
                        self.dropdown_angle - (PI / 2.0) * ctx.delta_time * 4.0
                    }
                    .clamp(0.0, PI / 2.0);

                ctx.draw(dropdown);
            }
        }

        let half_width = (35 + 26 + 10) as f32 * ctx.scale_factor;
        let height = (10 + 28) as f32 * ctx.scale_factor;

        let back_button = Button::new(BACK_BUTTON, memory_key!())
            .pos(
                Vector2::new(ctx.center().x + half_width, height),
                Anchor::Center,
            )
            .scale(Vector2::repeat(4.0))
            .set_back();
        back_button.is_clicked(ctx).then(|| state.pop_screen());
        ctx.draw(back_button);

        let create_button = Button::new(CREATE_BUTTON, memory_key!())
            .pos(
                Vector2::new(ctx.center().x - half_width, height),
                Anchor::Center,
            )
            .scale(Vector2::repeat(4.0));
        if create_button.is_clicked(ctx) {
            self.create = Some(CreateModal {});
        }
        ctx.draw(create_button);
    }

    fn on_init(&mut self, state: &mut App) {
        // todo: make async with poll_promise?
        self.world_dir = state.data_dir.join("sandbox");
        if self.world_dir.exists() {
            self.worlds = load_level_dir(&self.world_dir);
        }
    }
}

const NEW_SANDBOX_TEXT: &str =
    "Choose a name for your new sandbox then click 'Create' or press enter.";
const INVALID_NAME_TEXT: &str =
    "Only alphanumeric characters, spaces, dashes, and underscores can be used in sandbox names.";
const NO_NAME_TEXT: &str = "Please enter a name for your new sandbox.";

const NAME_KEY: MemoryKey = memory_key!();

impl SandboxScreen {
    fn create_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let mut exit = ctx.input.consume_key_pressed(KeyCode::Escape);
        let mut enter = ctx.input.consume_key_pressed(KeyCode::Enter);

        if let Some(_create) = &mut self.create {
            ctx.defer(|ctx| ctx.darken(Rgb::repeat(0.5), layer::OVERLAY));

            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(Vector2::new(ctx.center().x, 500.0))
                .margin(margin)
                .layer(layer::OVERLAY);

            let size = modal.inner_size();
            let mut name_error = false;
            modal.draw(ctx, |ctx, root| {
                let body = body(size.x);

                root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                    body("New Sandbox")
                        .scale(Vector2::repeat(4.0))
                        .layout(ctx, layout);
                    body(NEW_SANDBOX_TEXT).layout(ctx, layout);

                    Spacer::new_y(8.0 * ctx.scale_factor).layout(ctx, layout);

                    body("Sandbox Name").layout(ctx, layout);

                    let name = TextInput::new(NAME_KEY)
                        .default_active(true)
                        .width(size.x.min(400.0 * ctx.scale_factor));
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

                    let (back, create) = modal_buttons(ctx, layout, size.x, ("Back", "Create"));
                    let click = ctx.input.mouse_pressed(MouseButton::Left);
                    enter |= create && !name_error && click;
                    exit |= back && click;

                    if create && name_error {
                        ctx.set_cursor(CursorIcon::NotAllowed);
                    }
                });
            });

            if enter && !name_error {
                let name = TextInput::content_for(ctx, NAME_KEY);

                let file_name = name.replace(' ', "_");
                let path = self.world_dir.join(file_name).with_extension("bin");

                let board = Board::new_sandbox(name);
                let screen = GameScreen::new(board, path);
                state.push_screen(screen);
            }
        }

        if self.create.is_some() && (exit || enter) {
            self.create = None;
        }
    }
}
