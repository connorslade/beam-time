use engine::{
    color::Rgb,
    drawable::spacer::Spacer,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::GraphicsContext,
    layout::{
        column::ColumnLayout, row::RowLayout, tracker::LayoutTracker, Direction, Justify, Layout,
        LayoutElement, LayoutMethods,
    },
    memory_key,
};

use crate::{
    app::App,
    assets::TRASH,
    consts::layer,
    ui::{
        components::{button::Button, modal::Modal, text_input::TextInput},
        misc::body,
    },
};

use super::GameScreen;

const NOTE_DESCRIPTION: &str = "Notes allow you to place blocks of text in the world. All notes have a title and optionally have body text that shows when zoomed in.";

#[derive(Clone, Copy)]
pub struct NoteEditModal {
    pub index: usize,
}

impl GameScreen {
    pub(super) fn note_edit_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        if let Some(note) = self.note_edit {
            ctx.defer(|ctx| ctx.darken(Rgb::repeat(0.5), layer::UI_OVERLAY));

            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(Vector2::new(ctx.center().x, 500.0))
                .margin(margin)
                .layer(layer::UI_OVERLAY);

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
                                        self.board.notes.remove(note.index);
                                        self.note_edit = None;
                                    }
                                },
                            );
                        },
                    );

                    body(NOTE_DESCRIPTION).layout(ctx, layout);

                    let (title, body) = (memory_key!(), memory_key!());
                    TextInput::new(title)
                        .default_active(true)
                        .placeholder("Title")
                        .width(size.x.min(400.0 * ctx.scale_factor))
                        .max_chars(32)
                        .layout(ctx, layout);

                    TextInput::new(body)
                        .placeholder("Body")
                        .width(size.x)
                        .layout(ctx, layout);

                    if ctx.input.consume_key_pressed(KeyCode::Escape) {
                        let note = &mut self.board.notes[note.index];
                        note.title = TextInput::content_for(ctx, title);
                        note.body = TextInput::content_for(ctx, body);

                        self.note_edit = None;
                    }
                });
            });
        }
    }
}
