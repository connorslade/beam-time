use std::{f32::consts::PI, path::PathBuf};

use common::misc::in_bounds;
use engine::{
    color::Rgb,
    drawable::{sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, Drawable, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::{BACK_BUTTON, CREATE_BUTTON, LEVEL_DROPDOWN_ARROW, UNDEAD_FONT},
    consts::layer,
    game::board::BoardMeta,
    screens::game::GameScreen,
    ui::{
        button::{Button, ButtonState},
        layout::column::ColumnLayout,
        misc::{font_scale, titled_screen},
        modal::Modal,
        text_input::{TextInput, TextInputState},
    },
    util::load_level_dir,
    App,
};

#[derive(Default)]
pub struct SandboxScreen {
    worlds: Vec<(PathBuf, BoardMeta)>,

    back_button: ButtonState,
    create_button: ButtonState,
    dropdown_angle: f32,

    create: Option<CreateModal>,
}

pub struct CreateModal {
    name_input: TextInputState,
}

impl Screen<App> for SandboxScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
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
                        ctx.push_screen(GameScreen::load(world.clone()));
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

        let back_button = Button::new(BACK_BUTTON, &mut self.back_button)
            .pos(
                Vector2::new(ctx.center().x + half_width, height),
                Anchor::Center,
            )
            .scale(Vector2::repeat(4.0))
            .set_back();
        if back_button.is_clicked(ctx) {
            ctx.pop_screen();
        }
        ctx.draw(back_button);

        let create_button = Button::new(CREATE_BUTTON, &mut self.create_button)
            .pos(
                Vector2::new(ctx.center().x - half_width, height),
                Anchor::Center,
            )
            .scale(Vector2::repeat(4.0));
        if create_button.is_clicked(ctx) {
            self.create = Some(CreateModal {
                name_input: TextInputState::default(),
            });
        }
        ctx.draw(create_button);
    }

    fn on_init(&mut self, state: &mut App) {
        // todo: make async with poll_promise?
        let world_dir = state.data_dir.join("sandbox");
        if world_dir.exists() {
            self.worlds = load_level_dir(world_dir);
        }
    }
}

impl SandboxScreen {
    fn create_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        const NEW_SANDBOX_TEXT: &str =
            "Choose a name for your new sandbox and press enter to create it!";

        if self.create.is_some() && ctx.input.consume_key_pressed(KeyCode::Escape) {
            self.create = None;
        }

        if let Some(create) = &mut self.create {
            ctx.defer(|ctx| ctx.darken(Rgb::repeat(0.5), layer::OVERLAY));

            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(Vector2::new(ctx.center().x, 500.0))
                .margin(margin)
                .layer(layer::OVERLAY);

            let size = modal.inner_size();
            modal.draw(ctx, |ctx| {
                let body = |text| Text::new(UNDEAD_FONT, text).scale(Vector2::repeat(2.0));

                let mut layout = ColumnLayout::new(padding);
                layout.draw(ctx, body("New Sandbox").scale(Vector2::repeat(4.0)));
                layout.draw(ctx, body(NEW_SANDBOX_TEXT).max_width(size.x));

                layout.space(8.0 * ctx.scale_factor * state.config.ui_scale);
                layout.draw(ctx, body("Sandbox Name"));
                layout.draw(
                    ctx,
                    TextInput::new(&mut create.name_input)
                        .width(size.x.min(400.0 * ctx.scale_factor)),
                );

                layout.space_to(size.y - ctx.scale_factor * 12.0);
                layout.row(ctx, |ctx| {
                    let button_space = ctx.scale_factor * 10.0;

                    Sprite::new(LEVEL_DROPDOWN_ARROW)
                        .scale(Vector2::repeat(2.0))
                        .rotate(PI, Anchor::Center)
                        .draw(ctx);
                    body("Back")
                        .position(Vector2::x() * button_space, Anchor::BottomLeft)
                        .draw(ctx);

                    Sprite::new(LEVEL_DROPDOWN_ARROW)
                        .position(Vector2::x() * size.x, Anchor::BottomRight)
                        .scale(Vector2::repeat(2.0))
                        .draw(ctx);
                    body("Create")
                        .position(Vector2::x() * (size.x - button_space), Anchor::BottomRight)
                        .draw(ctx);
                });
            });
        }
    }
}
