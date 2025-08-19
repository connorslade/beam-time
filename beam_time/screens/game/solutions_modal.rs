use std::{cmp::Reverse, iter, mem, path::PathBuf};

use anyhow::Result;
use beam_logic::level::Level;
use engine::{
    drawable::{Anchor, dummy::DummyDrawable, spacer::Spacer, sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};
use log::error;
use slug::slugify;
use thousands::Separable;
use uuid::Uuid;

use crate::{
    app::App,
    assets::{DUPLICATE, EDIT, TRASH, UNDEAD_FONT},
    consts::{
        layer, paths,
        spacing::{MARGIN, PADDING},
    },
    game::board::{Board, BoardMeta, LevelMeta, LevelStats, unloaded::UnloadedBoard},
    screens::game::{ActiveModal, GameScreen},
    ui::{
        board_operations::{
            BoardType,
            create::{self, create_modal},
            delete::{self, delete_modal, reset_modal},
        },
        components::{
            button::{ButtonEffects, ButtonExt},
            horizontal_rule::Rule,
            modal::{Modal, modal_buttons},
        },
        misc::{body, modal_size},
    },
};

impl GameScreen {
    pub(super) fn solutions_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        // todo: maybe don't sort every frame? just a thought
        self.solutions.sort_by_key(|x| Reverse(x.meta.last_played));
        let level = self.board.transient.level.unwrap();

        let tracker = LayoutTracker::new(memory_key!());
        let height = tracker.bounds(ctx).map(|x| x.height()).unwrap_or_default();
        let size = modal_size(ctx);

        let modal = Modal::new(Vector2::new(size.x, size.y.max(height)))
            .position(ctx.center(), Anchor::Center)
            .margin(MARGIN)
            .layer(layer::UI_OVERLAY);
        modal.draw(ctx, |ctx, root| {
            let size = root.available();
            let body = body(size.x);

            ColumnLayout::new(PADDING)
                .justify(Justify::Center)
                .tracked(tracker)
                .show(ctx, root, |ctx, layout| {
                    RowLayout::new(0.0).show(ctx, layout, |ctx, layout| {
                        body(&format!("{} Solutions", level.name))
                            .scale(Vector2::repeat(4.0))
                            .layout(ctx, layout);
                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                    });
                    DummyDrawable::new().layout(ctx, layout);

                    Rule::horizontal(layout.available().x).layout(ctx, layout);
                    self.solution(state, ctx, layout, 0);
                    Rule::horizontal(layout.available().x).layout(ctx, layout);
                    for i in 0..self.solutions.len() {
                        self.solution(state, ctx, layout, i + 1);
                        Rule::horizontal(layout.available().x).layout(ctx, layout);
                    }

                    Text::new(UNDEAD_FONT, "+ New Solution +")
                        .scale(Vector2::repeat(2.0))
                        .button(memory_key!())
                        .on_click(ctx, || {
                            // make a function for this
                            let board = Board {
                                meta: BoardMeta {
                                    name: format!("New Solution {}", self.solutions.len() + 2),
                                    level: Some(LevelMeta {
                                        id: level.id,
                                        solved: None,
                                    }),
                                    size: level.size,
                                    ..Default::default()
                                },
                                tiles: level.tiles.clone(),
                                ..Default::default()
                            };
                            let meta = board.meta.clone();

                            let id = Uuid::new_v4();
                            let path = state
                                .data_dir
                                .join(paths::CAMPAIGN)
                                .join(format!("{}_{id}.bin", slugify(&level.name)));
                            if let Err(err) = board.save_exact(&path) {
                                error!("Failed to create new solution: {err}");
                            } else {
                                self.solutions.push(UnloadedBoard { path, meta });
                            }
                        })
                        .layout(ctx, layout);

                    let (back, _) = modal_buttons(ctx, layout, size.x, ("Back", ""));
                    let click = ctx.input.mouse_pressed(MouseButton::Left);
                    if click && back {
                        self.modal = ActiveModal::Paused;
                        ctx.input.cancel_clicks()
                    }
                });
        });
    }

    fn name(&self, idx: usize) -> &String {
        if idx == 0 {
            &self.board.meta.name
        } else {
            &self.solutions[idx - 1].meta.name
        }
    }

    pub(super) fn solutions_rename_modal(&mut self, ctx: &mut GraphicsContext, index: usize) {
        let name = self.name(index);
        match create_modal(ctx, BoardType::Solution, Some(name)) {
            create::Result::Nothing => {}
            create::Result::Cancled => self.modal = ActiveModal::Solutions,
            create::Result::Finished(new) => {
                if index == 0 {
                    self.board.meta.name = new;
                } else {
                    let path = &self.solutions[index - 1].path;
                    if let Err(err) = rename_unloaded_solution(path, new) {
                        error!("Failed to rename solution: {err}");
                    }
                }

                self.modal = ActiveModal::Solutions;
            }
        }
    }

    pub(super) fn solutions_delete_modal(
        &mut self,
        state: &mut App,
        ctx: &mut GraphicsContext,
        index: usize,
    ) {
        let name = self.name(index);
        match delete_modal(ctx, BoardType::Solution, name) {
            delete::Result::Nothing => {}
            delete::Result::Cancled => self.modal = ActiveModal::Solutions,
            delete::Result::Deleted => {
                self.modal = ActiveModal::Solutions;

                if index == 0 {
                    self.board.transient.trash = true;
                    state.pop_screen();
                    // todo: auto load next solution if there is another
                } else {
                    let path = self.solutions.remove(index - 1).path;
                    if let Err(err) = trash::delete(path) {
                        error!("Failed to trash solution: {err}");
                    }
                }
            }
        }
    }

    pub(super) fn solutions_reset_modal(&mut self, ctx: &mut GraphicsContext) {
        match reset_modal(ctx, &self.board.meta.name) {
            delete::Result::Nothing => {}
            delete::Result::Cancled => self.modal = ActiveModal::Paused,
            delete::Result::Deleted => {
                self.modal = ActiveModal::Paused;
                self.beam.get().beam = None;
                self.board.reset();
            }
        }
    }

    fn solution<L: Layout + 'static>(
        &mut self,
        state: &mut App,
        ctx: &mut GraphicsContext,
        layout: &mut L,
        index: usize,
    ) {
        let (path, meta) = if index == 0 {
            (&self.save_file, &self.board.meta)
        } else {
            let solution = &self.solutions[index - 1];
            (&solution.path, &solution.meta)
        };

        let mut new_board = None;
        let mut load = None;
        ColumnLayout::new(PADDING).show(ctx, layout, |ctx, layout| {
            let level = meta.level.as_ref().unwrap();
            let text = if let Some(LevelStats { cost, latency }) = level.solved {
                let cost = cost.separate_with_commas();
                format!("Costs ${cost} • Latency of {latency} ticks")
            } else {
                "Unsolved".into()
            };

            RowLayout::new(0.0)
                .justify(Justify::Center)
                .sized(Vector2::new(layout.available().x, 0.0))
                .show(ctx, layout, |ctx, layout| {
                    let current = if index == 0 { " (Current)" } else { "" };
                    let title = Text::new(UNDEAD_FONT, format!("{}{current}", meta.name))
                        .max_width(layout.available().x - MARGIN)
                        .scale(Vector2::repeat(3.0));

                    if index != 0 {
                        title
                            .button(memory_key!(path))
                            .effects(ButtonEffects::empty())
                            .on_click(ctx, || load = Some(index))
                            .layout(ctx, layout);
                    } else {
                        title.layout(ctx, layout);
                    }

                    let row = RowLayout::new(PADDING).direction(Direction::MaxToMin);
                    row.show(ctx, layout, |ctx, layout| {
                        let button = |asset| {
                            Sprite::new(asset)
                                .scale(Vector2::repeat(2.0))
                                .button(memory_key!(path, asset))
                        };

                        button(TRASH)
                            .on_click(ctx, || {
                                self.modal = ActiveModal::SolutionDelete { index };
                            })
                            .layout(ctx, layout);
                        button(DUPLICATE)
                            .on_click(ctx, || {
                                let level = self.board.transient.level.unwrap();
                                let path = path.to_path_buf();
                                match duplicate_solution(path, level) {
                                    Ok(board) => new_board = Some(board),
                                    Err(err) => error!("Failed to duplicate solution: {err}"),
                                }
                            })
                            .layout(ctx, layout);
                        button(EDIT)
                            .on_click(ctx, || self.modal = ActiveModal::SolutionEdit { index })
                            .layout(ctx, layout);

                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                    });
                });

            Text::new(UNDEAD_FONT, &text)
                .scale(Vector2::repeat(2.0))
                .layout(ctx, layout);
        });

        if let Some(board) = new_board {
            self.solutions.push(board);
        }

        if let Some(index) = load {
            let solution = self.solutions.remove(index - 1);
            state.pop_screen();

            state.push_screen(
                GameScreen::load(solution.path)
                    .with_solutions(mem::take(&mut self.solutions).into_iter())
                    .with_solutions(iter::once(UnloadedBoard {
                        path: self.save_file.clone(),
                        meta: self.board.meta.clone(),
                    })),
            );
        }
    }
}

fn duplicate_solution(solution: PathBuf, level: &Level) -> Result<UnloadedBoard> {
    let mut board = Board::load(&solution)?;
    board.meta.playtime = 0;
    board.meta.name += " copy";

    let id = Uuid::new_v4();
    let parent = solution.parent().unwrap();
    let path = parent.join(format!("{}_{id}.bin", slugify(&level.name)));

    let meta = board.meta.clone();
    board.save_exact(&path)?;

    Ok(UnloadedBoard { path, meta })
}

fn rename_unloaded_solution(path: &PathBuf, name: String) -> Result<()> {
    let mut board = Board::load(path)?;
    board.meta.name = name;
    board.save_exact(path)?;
    Ok(())
}
