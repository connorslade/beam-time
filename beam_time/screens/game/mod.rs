use std::{
    borrow::Cow,
    collections::HashSet,
    mem,
    path::{Path, PathBuf},
    time::Duration,
};

use log::{info, warn};
use rand::{Rng, seq::SliceRandom};

#[cfg(feature = "steam")]
use crate::game::achievements::award_campaign_achievements;
use crate::{
    App,
    consts::color,
    game::{
        board::{Board, LevelStats, unloaded::UnloadedBoard},
        pancam::Pancam,
        render::beam::BeamStateRender,
    },
    ui::{confetti::Confetti, level_panel::LevelPanel, tile_picker::TilePicker},
    util::key_events,
};
use beam_logic::{
    level::default::DEFAULT_LEVELS,
    misc::price,
    simulation::{
        level_state::LevelResult, runtime::asynchronous::AsyncSimulationState, state::BeamState,
    },
};
use engine::{
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::GraphicsContext,
};

use super::Screen;

mod note_edit_modal;
mod paused_modal;
mod solutions_modal;

pub struct GameScreen {
    pancam: Pancam,
    board: Board,
    beam: AsyncSimulationState,
    level_result: Option<LevelResult>,

    tile_picker: TilePicker,
    level_panel: LevelPanel,
    confetti: Confetti,
    modal: ActiveModal,

    save_file: PathBuf,
    solutions: Vec<UnloadedBoard>,

    needs_init: bool,
    tps: f32,
}

#[derive(Debug, Clone, Copy)]
enum ActiveModal {
    None,
    Paused,
    Reset,
    NoteEdit { index: usize, old: bool },

    Solutions,
    SolutionEdit { index: usize },
    SolutionDelete { index: usize },
}

impl Screen for GameScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        self.modal(state, ctx);
        match self.modal {
            ActiveModal::None => self.pancam.update(state, ctx),
            _ => self.pancam.only_animate(ctx),
        }

        if mem::take(&mut self.needs_init) {
            let pan = if let Some(size) = self.board.meta.size {
                let tile_size = 16.0 * self.pancam.scale;
                let half_board = size.map(|x| x as f32) * tile_size / 2.0;
                ctx.center() + Vector2::repeat(tile_size) - half_board
            } else {
                ctx.center()
            };

            self.pancam.pan = pan;
            self.pancam.pan_goal = pan;
        }

        if let Some(old_size) = ctx.window.size_changed() {
            self.pancam.on_resize(old_size, ctx.size());
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

        let space_pressed = ctx.input.key_pressed(KeyCode::Space);
        let play_pressed = ctx.input.key_pressed(KeyCode::KeyF);
        let test_pressed = ctx.input.key_pressed(KeyCode::KeyT) && self.board.meta.level.is_some();
        let escape = ctx.input.key_pressed(KeyCode::Escape);
        let shift = ctx.input.key_down(KeyCode::ShiftLeft);

        let mut stop_simulation = sim.beam.is_some()
            && ((escape && !shift && matches!(self.modal, ActiveModal::None))
                || (sim.is_playing() && test_pressed));
        sim.runtime.running &= !space_pressed;
        sim.runtime.running |= play_pressed || test_pressed;

        if ctx.input.key_pressed(KeyCode::Escape) {
            self.modal = match self.modal {
                ActiveModal::Paused => ActiveModal::None,
                _ if sim.beam.is_none() || shift => ActiveModal::Paused,
                x => x,
            }
        }

        if let Some(beam_state) = &mut sim.beam
            && !stop_simulation
        {
            // Make async?
            space_pressed.then(|| beam_state.tick());
            beam_state.render(ctx, state, &self.pancam);

            let level_result = beam_state.level.as_ref().and_then(|x| x.result);
            if let Some(result) = level_result {
                self.level_result = Some(result);
                sim.runtime.running = false;

                if let LevelResult::Success { latency } = result
                    && let Some(level_meta) = &mut self.board.meta.level
                {
                    let level = self.board.transient.level.as_ref().unwrap();
                    let (cost, _count) = price(&self.board.tiles, level);

                    // Award potential steam achievements
                    #[cfg(feature = "steam")]
                    award_campaign_achievements(state, level_meta.id);

                    state.mark_level_complete(level.id);
                    // Upload solution to leaderboard server
                    state
                        .leaderboard
                        .publish_solution(&state.id, level.id, &self.board.tiles);

                    create_confetti(&mut self.confetti, ctx);
                    level_meta.solved = Some(LevelStats { cost, latency });
                    sim.beam = None;
                }
            }

            self.beam.notify_running();
        } else if space_pressed
            || (play_pressed && !sim.is_playing())
            || (test_pressed && !sim.is_testing())
        {
            stop_simulation = false;

            sim.beam = Some(BeamState::new(
                &self.board.tiles,
                self.board.transient.level.map(Cow::Borrowed),
                test_pressed.then(|| {
                    let tests = &self.board.transient.level.unwrap().tests;
                    tests.true_index(self.level_panel.case) * tests.variable_start as usize
                }),
            ));
            self.level_result = None;
        }

        stop_simulation.then(|| sim.beam = None);

        ctx.background(color::BACKGROUND);
        self.tile_picker
            .render(ctx, state, sim.beam.is_some(), &mut self.board);
        self.level_panel
            .render(ctx, state, &self.board, &sim, &self.level_result);
        self.confetti.render(ctx);

        self.board.transient.history.mark_clean();
        self.board.render(ctx, state, &self.pancam, &mut sim.beam);
        self.board.tick_input(ctx, &self.pancam, &mut sim.beam);

        if self.board.transient.history.is_dirty() {
            self.level_result = None;
            if let Some(level) = &mut self.board.meta.level {
                level.solved = None;
            }
        }
    }

    fn on_init(&mut self, state: &mut App) {
        #[cfg(feature = "steam")]
        if let Some(level) = self.board.transient.level {
            let text = format!("In a level: {}.", level.name);
            state.steam.rich_presence(Some(&text));
        } else {
            state.steam.rich_presence(Some("In a sandbox world."));
        }

        if let Some(level) = self.board.transient.level {
            state.leaderboard.fetch_results(level.id);
        }
    }

    fn on_destroy(&mut self, _state: &mut App) {
        #[cfg(feature = "steam")]
        _state.steam.rich_presence(None);

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
        let level_meta = board.meta.level.as_ref();
        board.transient.level =
            level_meta.map(|x| DEFAULT_LEVELS.iter().find(|y| y.id == x.id).unwrap());

        if let Some(level) = board.transient.level {
            let mut seen_ids = HashSet::new();
            for (_pos, tile) in board.tiles.iter() {
                if let Some(id) = tile.id() {
                    seen_ids.insert(id);
                }
            }

            for (pos, tile) in level.tiles.iter() {
                if let Some(id) = tile.id()
                    && !seen_ids.contains(&id)
                {
                    board.tiles.set(pos, tile);
                }
            }
        }

        Self {
            pancam: Pancam::default(),
            board,
            beam: AsyncSimulationState::new(),
            level_result: None,

            tile_picker: TilePicker::default(),
            level_panel: LevelPanel::default(),
            confetti: Confetti::new(),
            modal: ActiveModal::None,

            save_file,
            solutions: Vec::new(),

            needs_init: true,
            tps: 20.0,
        }
    }

    pub fn load(save_file: impl AsRef<Path>) -> Self {
        let save_file = save_file.as_ref().to_path_buf();
        GameScreen::new(Board::load(&save_file).unwrap_or_default(), save_file)
    }

    pub fn with_solutions(mut self, solutions: impl Iterator<Item = UnloadedBoard>) -> Self {
        self.solutions
            .extend(solutions.filter(|x| x.path != self.save_file));
        self
    }

    fn modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        self.note_edit_modal(state, ctx);
        match self.modal {
            ActiveModal::Paused => self.paused_modal(state, ctx),
            ActiveModal::Solutions => self.solutions_modal(state, ctx),
            ActiveModal::SolutionEdit { index } => self.solutions_rename_modal(ctx, index),
            ActiveModal::SolutionDelete { index } => self.solutions_delete_modal(state, ctx, index),
            ActiveModal::Reset => self.solutions_reset_modal(ctx),
            _ => {}
        }
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
