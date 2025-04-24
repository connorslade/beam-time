use std::mem;

use engine::{
    drawable::spacer::Spacer,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
    layout::{
        column::ColumnLayout, row::RowLayout, tracker::LayoutTracker, Direction, Justify, Layout,
        LayoutElement, LayoutMethods,
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
    ui::{
        components::{button::Button, modal::Modal, text_input::TextInput},
        misc::body,
    },
};

use super::GameScreen;

const DESCRIPTION: &str = "Notes allow you to place blocks of text in the world. \
All notes have a title and optionally have body text that shows when zoomed in.";
const DEFAULT_NAME: &str = "New Note";
const DEFAULT_BODY: &str =
    "This note doesn't have any content yet, click it to open the edit modal.";

pub struct NoteEditModal {
    pub index: usize,
    pub old: bool,
}

enum Operation {
    None,
    Delete,
    Edit { title: String, body: String },
}

impl GameScreen {
    pub(super) fn note_edit_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        if self.note_edit.is_none() && ctx.input.consume_key_pressed(KeyCode::KeyN) {
            let position = self.shared.screen_to_world_space(ctx, ctx.input.mouse);
            let closest = closest_note(&self.board.notes, position);
            let closest_distance = closest.map(|x| x.1).unwrap_or(f32::MAX);

            if closest_distance < 1.0 {
                let index = closest.unwrap().0;
                self.note_edit = Some(NoteEditModal { index, old: true });
            } else {
                let index = self.board.notes.len();
                self.board.notes.push(Note {
                    position,
                    title: DEFAULT_NAME.into(),
                    body: DEFAULT_BODY.into(),
                });

                self.note_edit = Some(NoteEditModal { index, old: false });
            }
        }

        if let Some(note) = &mut self.note_edit {
            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(Vector2::new(ctx.center().x, 500.0))
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
                                    Button::new(TRASH, memory_key!())
                                        .scale(Vector2::repeat(2.0))
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

                    if mem::take(&mut note.old) {
                        let note = &self.board.notes[note.index];
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
                    self.board.notes.remove(note.index);
                }
                Operation::Edit { title, body } => {
                    let note = &mut self.board.notes[note.index];
                    note.title = title;
                    note.body = body;
                }
                Operation::None => {}
            }

            if close {
                self.note_edit = None;
            }
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
