use std::{cmp::Reverse, path::PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use engine::{
    drawable::{
        Anchor, Drawable,
        shape::{rectangle::Rectangle, rectangle_outline::RectangleOutline},
        spacer::Spacer,
        sprite::Sprite,
        text::Text,
    },
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::GraphicsContext,
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        root::RootLayout, row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};
use log::error;
use slug::slugify;

use crate::{
    App,
    assets::{ALAGARD_FONT, DUPLICATE, EDIT, TRASH, UNDEAD_FONT},
    consts::{WATERFALL, color, layer},
    game::board::{
        Board,
        unloaded::{UnloadedBoard, load_level_dir},
    },
    screens::game::GameScreen,
    ui::{
        components::{
            button::{ButtonEffects, ButtonExt},
            modal::{Modal, modal_buttons},
        },
        misc::{body, modal_size, spacing},
        waterfall::Waterfall,
    },
    util::time::{human_duration, human_duration_minimal},
};

use super::Screen;

mod create_modal;

#[derive(Default)]
pub struct SandboxScreen {
    world_dir: PathBuf,
    worlds: Vec<UnloadedBoard>,

    modal: ActiveModal,
}

#[derive(Default)]
enum ActiveModal {
    #[default]
    None,
    Create,
    Delete(usize),
    Edit(usize),
}

impl Screen for SandboxScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let (_, padding) = spacing(ctx);

        ctx.background(color::BACKGROUND);
        Waterfall::new(WATERFALL).draw(ctx);
        self.modals(state, ctx);

        ctx.input
            .key_pressed(KeyCode::Escape)
            .then(|| state.pop_screen());

        let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);
        Text::new(ALAGARD_FONT, "Sandbox")
            .position(pos, Anchor::TopCenter)
            .scale(Vector2::repeat(6.0))
            .default_shadow()
            .draw(ctx);

        let mut load_worlds = false;
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

                    for (i, board) in self.worlds.iter().enumerate() {
                        let tracker = LayoutTracker::new(memory_key!(i));
                        if let Some(bounds) = tracker.bounds(ctx) {
                            let offset = Vector2::repeat(padding);
                            let (size, pos) = (bounds.size() + offset * 2.0, bounds.min - offset);

                            RectangleOutline::new(size, 4.0)
                                .position(pos, Anchor::BottomLeft)
                                .relative_inner()
                                .color(color::MODAL_BORDER)
                                .draw(ctx);
                            Rectangle::new(size)
                                .position(pos, Anchor::BottomLeft)
                                .color(color::BACKGROUND)
                                .z_index(-1)
                                .draw(ctx);
                        }

                        let column = ColumnLayout::new(padding).tracked(tracker);
                        column.show(ctx, layout, |ctx, layout| {
                            RowLayout::new(0.0)
                                .justify(Justify::Center)
                                .sized(Vector2::new(width, 0.0))
                                .show(ctx, layout, |ctx, layout| {
                                    Text::new(UNDEAD_FONT, &board.meta.name)
                                        .scale(Vector2::repeat(3.0))
                                        .button(memory_key!(i))
                                        .effects(ButtonEffects::empty())
                                        .on_click(ctx, || {
                                            state.push_screen(GameScreen::load(&board.path))
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

                                        button(TRASH)
                                            .on_click(ctx, || self.modal = ActiveModal::Delete(i))
                                            .layout(ctx, layout);
                                        button(DUPLICATE)
                                            .on_click(ctx, || {
                                                load_worlds = true;
                                                if let Err(err) = duplicate_board(&board.path) {
                                                    error!("Failed to duplicate board: {err}");
                                                }
                                            })
                                            .layout(ctx, layout);
                                        button(EDIT)
                                            .on_click(ctx, || self.modal = ActiveModal::Edit(i))
                                            .layout(ctx, layout);

                                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                                    });
                                });

                            let since_last_play =
                                (Utc::now() - board.meta.last_played).num_seconds();
                            let playtime = format!(
                                "Last played {} ago\nPlayed for {}",
                                human_duration_minimal(since_last_play as u64),
                                human_duration(board.meta.playtime),
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
                    .on_click(ctx, || self.modal = ActiveModal::Create)
                    .layout(ctx, layout);
            },
        );

        root.draw(ctx);

        load_worlds.then(|| self.load_worlds());
    }

    fn on_init(&mut self, state: &mut App) {
        self.world_dir = state.data_dir.join("sandbox");
        self.load_worlds();
    }
}

impl SandboxScreen {
    fn load_worlds(&mut self) {
        if self.world_dir.exists() {
            self.worlds = load_level_dir(&self.world_dir);
            self.worlds.sort_by_key(|x| Reverse(x.meta.last_played));
        }
    }
}

fn duplicate_board(world: &PathBuf) -> Result<()> {
    let mut board = Board::load(world)?;
    board.meta.playtime = 0;
    board.meta.name = format!("{} copy", board.meta.name);

    let world_dir = world.parent().context("No parent")?;
    let path = world_dir.join(format!("{}.ron", slugify(&board.meta.name)));
    board.save_exact(&path)?;
    Ok(())
}

impl SandboxScreen {
    fn modals(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        match self.modal {
            ActiveModal::None => {}
            ActiveModal::Delete(board) => self.delete_modal(state, ctx, board),
            ActiveModal::Create => self.create_modal(state, ctx, None),
            ActiveModal::Edit(board) => self.create_modal(state, ctx, Some(board)),
        }
    }

    fn delete_modal(&mut self, _state: &mut App, ctx: &mut GraphicsContext, board: usize) {
        let mut exit = ctx.input.consume_key_pressed(KeyCode::Escape);

        let (margin, padding) = spacing(ctx);
        let modal = Modal::new(modal_size(ctx))
            .position(ctx.center(), Anchor::Center)
            .margin(margin)
            .layer(layer::OVERLAY);

        let size = modal.inner_size();
        modal.draw(ctx, |ctx, root| {
            let body = body(size.x);

            root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                body("Deletion Confirmation")
                    .scale(Vector2::repeat(4.0))
                    .layout(ctx, layout);
                Spacer::new_y(4.0 * ctx.scale_factor).layout(ctx, layout);

                let world = &self.worlds[board];
                let text = format!(
                    "Are you sure you want to delete the sandbox world '{}'?",
                    world.meta.name
                );
                body(&text).layout(ctx, layout);
                body("If so, it will be moved to your system trash.").layout(ctx, layout);

                let click = ctx.input.mouse_pressed(MouseButton::Left);
                let (back, delete) = modal_buttons(ctx, layout, size.x, ("Back", "Delete"));
                exit |= back && click;

                if delete && click {
                    self.modal = ActiveModal::None;
                    if let Err(err) = trash::delete(&world.path) {
                        error!("Failed to delete world: {err}");
                    }
                    self.load_worlds();
                }
            });
        });

        exit.then(|| self.modal = ActiveModal::None);
    }
}
