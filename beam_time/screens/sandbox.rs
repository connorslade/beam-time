use std::{cmp::Reverse, path::PathBuf};

use chrono::Utc;
use engine::{
    drawable::{Anchor, Drawable},
    drawable::{
        shape::{rectangle::Rectangle, rectangle_outline::RectangleOutline},
        spacer::Spacer,
        sprite::Sprite,
        text::Text,
    },
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        root::RootLayout, row::RowLayout, tracker::LayoutTracker,
    },
    memory::MemoryKey,
    memory_key,
};

use crate::{
    App,
    assets::{ALAGARD_FONT, DUPLICATE, EDIT, TRASH, UNDEAD_FONT},
    consts::{BACKGROUND_COLOR, ERROR_COLOR, MODAL_BORDER_COLOR, WATERFALL, layer},
    game::board::{Board, BoardMeta},
    screens::game::GameScreen,
    ui::{
        components::{
            button::{ButtonEffects, ButtonExt},
            modal::{Modal, modal_buttons},
            text_input::TextInput,
        },
        misc::body,
        waterfall::Waterfall,
    },
    util::{human_duration, human_duration_minimal, load_level_dir},
};

use super::Screen;

#[derive(Default)]
pub struct SandboxScreen {
    world_dir: PathBuf,
    worlds: Vec<(PathBuf, BoardMeta)>,

    create: bool,
}

impl Screen for SandboxScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let (_, padding) = state.spacing(ctx);

        ctx.background(BACKGROUND_COLOR);
        Waterfall::new(WATERFALL).draw(ctx);

        ctx.input
            .key_pressed(KeyCode::Escape)
            .then(|| state.pop_screen());

        let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);
        Text::new(ALAGARD_FONT, "Sandbox")
            .position(pos, Anchor::TopCenter)
            .scale(Vector2::repeat(6.0))
            .default_shadow()
            .draw(ctx);

        self.create_modal(state, ctx);

        let mut root = RootLayout::new(ctx.center(), Anchor::Center);
        root.nest(
            ctx,
            ColumnLayout::new(32.0 * ctx.scale_factor).justify(Justify::Center),
            |ctx, layout| {
                if self.worlds.is_empty() {
                    Text::new(UNDEAD_FONT, "No worlds...")
                        .position(ctx.center(), Anchor::Center)
                        .scale(Vector2::repeat(4.0))
                        .layout(ctx, layout);
                } else {
                    let width = (ctx.size().x * 0.75)
                        .clamp(400.0 * ctx.scale_factor, 600.0 * ctx.scale_factor);

                    for (i, (world, meta)) in self.worlds.iter().enumerate() {
                        let tracker = LayoutTracker::new(memory_key!(i));
                        if let Some(bounds) = tracker.bounds(ctx) {
                            let offset = Vector2::repeat(padding);
                            let (size, pos) = (bounds.size() + offset * 2.0, bounds.min - offset);

                            RectangleOutline::new(size, 4.0)
                                .position(pos, Anchor::BottomLeft)
                                .relative_inner()
                                .color(MODAL_BORDER_COLOR)
                                .draw(ctx);
                            Rectangle::new(size)
                                .position(pos, Anchor::BottomLeft)
                                .color(BACKGROUND_COLOR)
                                .z_index(-1)
                                .draw(ctx);
                        }

                        let column = ColumnLayout::new(padding).tracked(tracker);
                        column.show(ctx, layout, |ctx, layout| {
                            RowLayout::new(0.0)
                                .justify(Justify::Center)
                                .sized(Vector2::new(width, 0.0))
                                .show(ctx, layout, |ctx, layout| {
                                    Text::new(UNDEAD_FONT, &meta.name)
                                        .scale(Vector2::repeat(3.0))
                                        .button(memory_key!(i))
                                        .effects(ButtonEffects::empty())
                                        .on_click(ctx, || {
                                            state.push_screen(GameScreen::load(world.clone()))
                                        })
                                        .layout(ctx, layout);

                                    let row =
                                        RowLayout::new(padding).direction(Direction::MaxToMin);
                                    row.show(ctx, layout, |ctx, layout| {
                                        let button = |asset| {
                                            Sprite::new(asset)
                                                .scale(Vector2::repeat(2.0))
                                                .button(memory_key!(i, asset))
                                        };

                                        button(TRASH).layout(ctx, layout);
                                        button(DUPLICATE).layout(ctx, layout);
                                        button(EDIT).layout(ctx, layout);

                                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                                    });
                                });

                            let since_last_play = (Utc::now() - meta.last_played).num_seconds();
                            let playtime = format!(
                                "Last played {} ago\nPlayed for {}",
                                human_duration_minimal(since_last_play as u64),
                                human_duration(meta.playtime),
                            );
                            Text::new(UNDEAD_FONT, playtime)
                                .scale(Vector2::repeat(2.0))
                                .layout(ctx, layout);
                        });
                    }
                }

                Text::new(UNDEAD_FONT, "+ New Sandbox +")
                    .scale(Vector2::repeat(2.0))
                    .default_shadow()
                    .button(memory_key!())
                    .on_click(ctx, || self.create = true)
                    .layout(ctx, layout);
            },
        );

        root.draw(ctx);
    }

    fn on_init(&mut self, state: &mut App) {
        // todo: make async with poll_promise?
        self.world_dir = state.data_dir.join("sandbox");
        if self.world_dir.exists() {
            self.worlds = load_level_dir(&self.world_dir);
            self.worlds
                .sort_by_key(|(_, meta)| Reverse(meta.last_played));
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
        if !self.create {
            return;
        }

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

                (create && name_error).then(|| ctx.window.cursor(CursorIcon::NotAllowed));
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

        if exit || enter {
            self.create = false;
        }
    }
}
