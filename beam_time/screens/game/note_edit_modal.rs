use std::mem;

use engine::{
    drawable::Anchor,
    drawable::{spacer::Spacer, sprite::Sprite},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::GraphicsContext,
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::{
    app::App,
    assets::TRASH,
    consts::layer,
    game::board::Note,
    screens::game::ActiveModal,
    ui::{
        components::{button::ButtonExt, modal::Modal, text_input::TextInput},
        misc::body,
    },
};

use super::GameScreen;

const DESCRIPTION: &str = "Notes allow you to place blocks of text in the world. \
All notes have a title and optionally have body text that shows when zoomed in.";
const DEFAULT_NAME: &str = "New Note";
const DEFAULT_BODY: &str =
    "This note doesn't have any content yet, click it to open the edit modal.";

enum Operation {
    None,
    Delete,
    Edit { title: String, body: String },
}

impl GameScreen {
    pub(super) fn note_edit_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        if matches!(self.modal, ActiveModal::None) && ctx.input.consume_key_pressed(KeyCode::KeyN) {
            let position = self.pancam.screen_to_world_space(ctx, ctx.input.mouse());
            let closest = closest_note(&self.board.notes, position);
            let closest_distance = closest.map(|x| x.1).unwrap_or(f32::MAX);

            if closest_distance < 1.0 {
                let index = closest.unwrap().0;
                self.modal = ActiveModal::NoteEdit { index, old: true };
            } else {
                let index = self.board.notes.len();
                self.board.notes.push(Note {
                    position,
                    title: DEFAULT_NAME.into(),
                    body: DEFAULT_BODY.into(),
                });

                self.modal = ActiveModal::NoteEdit { index, old: false };
            }
        }

        if let ActiveModal::NoteEdit { index, old } = &mut self.modal {
            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(state.modal_size(ctx))
                .position(ctx.center(), Anchor::Center)
                .margin(margin)
                .layer(layer::UI_OVERLAY);

            let (mut close, mut operation) = (false, Operation::None);

            let size = modal.inner_size();
            modal.draw(ctx, |ctx, root| {
                let body = body(size.x);

                root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                    layout.nest(
                        ctx,
                        RowLayout::new(padding).justify(Justify::Center),
                        |ctx, layout| {
                            body("Editing Note")
                                .scale(Vector2::repeat(4.0))
                                .layout(ctx, layout);

                            layout.nest(
                                ctx,
                                RowLayout::new(padding).direction(Direction::MaxToMin),
                                |ctx, layout| {
                                    let tracker = LayoutTracker::new(memory_key!());
                                    Sprite::new(TRASH)
                                        .scale(Vector2::repeat(2.0))
                                        .button(memory_key!())
                                        .tracked(tracker)
                                        .layout(ctx, layout);
                                    Spacer::new(Vector2::x() * layout.available().x)
                                        .layout(ctx, layout);

                                    if tracker.clicked(ctx, MouseButton::Left) {
                                        operation = Operation::Delete;
                                        close = true;
                                    }
                                },
                            );
                        },
                    );

                    body(DESCRIPTION).layout(ctx, layout);

                    let title = TextInput::new(memory_key!())
                        .default_active(true)
                        .placeholder("Title")
                        .width(size.x.min(400.0 * ctx.scale_factor))
                        .max_chars(32);

                    let body = TextInput::new(memory_key!())
                        .placeholder("Body")
                        .width(size.x);

                    if mem::take(old) {
                        let note = &self.board.notes[*index];
                        title.with_content(ctx, note.title.to_owned());
                        body.with_content(ctx, note.body.to_owned());
                    }

                    if ctx.input.consume_key_pressed(KeyCode::Escape) {
                        let (title, body) = (title.content(ctx), body.content(ctx));
                        if title.is_empty() && body.is_empty() {
                            operation = Operation::Delete;
                        } else {
                            operation = Operation::Edit { title, body };
                        }
                        close = true;
                    }

                    title.layout(ctx, layout);
                    body.layout(ctx, layout);
                });
            });

            match operation {
                Operation::Delete => {
                    self.board.notes.remove(*index);
                }
                Operation::Edit { title, body } => {
                    let note = &mut self.board.notes[*index];
                    note.title = title;
                    note.body = body;
                }
                Operation::None => {}
            }

            close.then(|| self.modal = ActiveModal::None);
        }
    }
}

fn closest_note(notes: &[Note], pos: Vector2<f32>) -> Option<(usize, f32)> {
    notes
        .iter()
        .enumerate()
        .map(|(idx, x)| (idx, OrderedFloat((x.position - pos).magnitude_squared())))
        .sorted_by_key(|(_x, dist)| *dist)
        .next()
        .map(|(idx, dist)| (idx, *dist))
}
