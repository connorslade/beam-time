use std::{borrow::Cow, mem, path::PathBuf, time::Duration};

use log::{info, warn};
use rand::{Rng, seq::SliceRandom};

#[cfg(feature = "steam")]
use crate::game::achievements::award_campaign_achievements;
use crate::{
    App,
    consts::BACKGROUND_COLOR,
    game::{board::Board, pancam::Pancam, render::beam::BeamStateRender},
    ui::{confetti::Confetti, level_panel::LevelPanel, tile_picker::TilePicker},
    util::key_events,
};
use beam_logic::{
    level::default::DEFAULT_LEVELS,
    simulation::{
        level_state::LevelResult, runtime::asynchronous::AsyncSimulationState, state::BeamState,
    },
};
use engine::{
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::GraphicsContext,
};

use super::Screen;

use note_edit_modal::NoteEditModal;
use paused_modal::PausedModal;
mod note_edit_modal;
mod paused_modal;

pub struct GameScreen {
    pancam: Pancam,
    board: Board,
    beam: AsyncSimulationState,

    tile_picker: TilePicker,
    level_panel: LevelPanel,
    confetti: Confetti,
    paused: Option<PausedModal>,
    note_edit: Option<NoteEditModal>,

    level_result: Option<LevelResult>,
    save_file: PathBuf,
    needs_init: bool,
    tps: f32,
}

impl Screen for GameScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        self.paused_modal(state, ctx);
        self.note_edit_modal(state, ctx);

        if self.paused.is_none() && self.note_edit.is_none() {
            self.pancam.update(state, ctx);
        }

        if mem::take(&mut self.needs_init) {
            let pan = if let Some(size) = self.board.meta.size {
                let tile_size = 16.0 * self.pancam.scale * ctx.scale_factor;
                let half_board = size.map(|x| x as f32) * tile_size / 2.0;
                ctx.center() + Vector2::repeat(tile_size) - half_board
            } else {
                ctx.center()
            };

            self.pancam.pan = pan;
            self.pancam.pan_goal = pan;
        }

        let shift = ctx.input.key_down(KeyCode::ShiftLeft);
        key_events!(ctx, {
            KeyCode::Digit0 => self.tps = [20.0, f32::MAX][shift as usize],
            KeyCode::Equal => self.tps += 5.0,
            KeyCode::Minus => self.tps -= 5.0
        });

        self.tps = self.tps.max(0.0);

        let mut sim = self.beam.get();
        state.debug(|| format!("Tick: {:.2?}", sim.tick_length));
        sim.runtime.time_per_tick = Duration::from_secs_f32(self.tps.max(1.0).recip());

        if sim.beam.is_none() && ctx.input.key_pressed(KeyCode::Escape) {
            self.paused = self.paused.is_none().then_some(PausedModal {});
        }

        let space_pressed = ctx.input.key_pressed(KeyCode::Space);
        let play_pressed = ctx.input.key_pressed(KeyCode::KeyP);
        let test_pressed = ctx.input.key_pressed(KeyCode::KeyT)
            && sim.beam.is_none()
            && self.board.meta.level.is_some();

        sim.runtime.running |= play_pressed || test_pressed;
        sim.runtime.running &= !space_pressed;

        if let Some(beam_state) = &mut sim.beam {
            // Make async?
            if space_pressed {
                beam_state.tick();
            }

            beam_state.render(ctx, state, &self.pancam);

            let level_result = beam_state.level.as_ref().and_then(|x| x.result);
            if let Some(result) = level_result {
                self.level_result = Some(result);
                sim.runtime.running = false;

                if matches!(result, LevelResult::Success { .. }) {
                    let level = self.board.transient.level.as_ref().unwrap();

                    // Award potential steam achievements
                    #[cfg(feature = "steam")]
                    award_campaign_achievements(state, level);

                    // Upload solution to leaderboard server
                    state
                        .leaderboard
                        .publish_solution(&state.id, level.id, &self.board.tiles);

                    create_confetti(&mut self.confetti, ctx);
                    self.board.meta.level.as_mut().unwrap().solved = true;
                    sim.beam = None;
                }
            }

            self.beam.notify_running();
        } else if space_pressed || play_pressed || test_pressed {
            sim.beam = Some(BeamState::new(
                &self.board.tiles,
                self.board.transient.level.map(Cow::Borrowed),
                test_pressed.then_some(self.level_panel.case),
            ));
            self.level_result = None;
        }

        if ctx.input.key_pressed(KeyCode::Escape) {
            sim.beam = None;
            self.level_result = None;
        }

        ctx.background(BACKGROUND_COLOR);
        self.tile_picker
            .render(ctx, state, sim.beam.is_some(), &mut self.board);
        self.level_panel
            .render(ctx, state, &self.board, &sim, &self.level_result);
        self.confetti.render(ctx);

        self.board.transient.history.mark_clean();
        self.board.render(ctx, state, &self.pancam, &mut sim.beam);

        if self.board.transient.history.is_dirty() {
            self.level_result = None;
            if let Some(level) = &mut self.board.meta.level {
                level.solved = false;
            }
        }
    }

    fn on_init(&mut self, state: &mut App) {
        if let Some(level) = self.board.transient.level {
            state.leaderboard.fetch_results(level.id);
        }
    }

    fn on_resize(&mut self, _state: &mut App, old_size: Vector2<f32>, new_size: Vector2<f32>) {
        self.pancam.on_resize(old_size, new_size);
    }

    fn on_destroy(&mut self) {
        let board = mem::take(&mut self.board);
        let trash = board.transient.trash;
        board.save(&self.save_file).unwrap();

        if trash {
            info!("Moving save to trash.");
            if let Err(err) = trash::delete(&self.save_file) {
                warn!("Failed to trash save. {err}")
            }
        }
    }
}

impl GameScreen {
    pub fn new(mut board: Board, save_file: PathBuf) -> Self {
        board.transient.level = board
            .meta
            .level
            .map(|x| DEFAULT_LEVELS.iter().find(|y| y.id == x.id).unwrap());

        Self {
            pancam: Pancam::default(),
            board,
            beam: AsyncSimulationState::new(),

            tile_picker: TilePicker::default(),
            level_panel: LevelPanel::default(),
            confetti: Confetti::new(),
            paused: None,
            note_edit: None,

            level_result: None,
            save_file,
            needs_init: true,
            tps: 20.0,
        }
    }

    pub fn load(save_file: PathBuf) -> Self {
        GameScreen::new(Board::load(&save_file).unwrap_or_default(), save_file)
    }
}

fn create_confetti(confetti: &mut Confetti, ctx: &mut GraphicsContext) {
    let mut points = [
        Vector2::new(0.25, 0.3),
        Vector2::new(0.5, 0.75),
        Vector2::new(0.75, 0.3),
    ];

    let mut rng = rand::rng();
    points.shuffle(&mut rng);

    let randomness = ctx.size() * 0.25;

    for (i, center) in points.iter().enumerate() {
        let offset_percent = Vector2::new(rng.random(), rng.random()) * 2.0 - Vector2::repeat(1.0);
        let pos = center.component_mul(&ctx.size()) + randomness.component_mul(&offset_percent);
        confetti.emit(pos, 100, 0.2 * i as f32);
    }
}
