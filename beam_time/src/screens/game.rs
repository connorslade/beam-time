use std::{mem, path::PathBuf, time::Duration};

use engine::{
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::GraphicsContext,
    screens::Screen,
};
use rand::{thread_rng, Rng};

use crate::{
    consts::BACKGROUND_COLOR,
    game::{
        beam::{state::BeamState, SimulationState},
        board::Board,
        level::LEVELS,
        SharedState,
    },
    ui::{confetti::Confetti, level_panel::LevelPanel, tile_picker::TilePicker},
    util::key_events,
    App,
};

pub struct GameScreen {
    shared: SharedState,
    board: Board,
    beam: SimulationState,

    tile_picker: TilePicker,
    level_panel: LevelPanel,
    confetti: Confetti,

    save_file: PathBuf,
    needs_init: bool,
    tps: f32,
}

impl Screen<App> for GameScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        self.shared.update(ctx, state);

        if self.needs_init {
            if let Some(size) = self.board.meta.size {
                let tile_size = 16.0 * self.shared.scale * ctx.scale_factor;
                let half_board = size.map(|x| x as f32) * tile_size / 2.0;
                let pan = ctx.center() + Vector2::repeat(tile_size) - half_board;
                self.shared.pan = pan;
                self.shared.pan_goal = pan;
            }

            self.needs_init = false;
        }

        let shift = ctx.input.key_down(KeyCode::ShiftLeft);
        key_events!(ctx, {
            KeyCode::Digit0 => {
                if shift {
                    self.tps = f32::MAX;
                } else {
                    self.tps = 20.0;
                }
            },
            KeyCode::Equal => self.tps += 5.0,
            KeyCode::Minus => self.tps -= 5.0
        });

        self.tps = self.tps.max(0.0);

        let mut sim = self.beam.get();
        sim.runtime.time_per_tick = Duration::from_secs_f32(self.tps.max(1.0).recip());

        if sim.beam.is_none() && ctx.input.key_pressed(KeyCode::Escape) {
            ctx.pop_screen()
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

            let is_complete = beam_state.level.as_ref().map(|x| x.is_complete());
            if is_complete == Some(true) {
                sim.runtime.running = false;
                sim.beam = None;
                create_fireworks(&mut self.confetti, ctx);
            } else {
                beam_state.render(ctx, state, &self.shared);
            }

            self.beam.notify_running();
        } else if space_pressed || play_pressed || test_pressed {
            sim.beam = Some(BeamState::new(&self.board, test_pressed));
        }

        if ctx.input.key_pressed(KeyCode::Escape) {
            sim.beam = None;
        }

        ctx.background(BACKGROUND_COLOR);
        self.board.render(ctx, state, &self.shared, &mut sim.beam);
        self.tile_picker.render(
            ctx,
            state,
            sim.beam.is_some(),
            &mut self.board.transient.holding,
        );
        self.level_panel.render(ctx, state, &self.board);
        self.confetti.render(ctx);
    }

    fn on_destroy(&mut self, _state: &mut App) {
        let board = mem::take(&mut self.board);
        board.save(&self.save_file).unwrap();
    }

    fn on_resize(&mut self, _state: &mut App, old_size: Vector2<f32>, new_size: Vector2<f32>) {
        self.shared.on_resize(old_size, new_size);
    }
}

impl GameScreen {
    pub fn new(mut board: Board, save_file: PathBuf) -> Self {
        board.transient.level = board
            .meta
            .level
            .map(|x| LEVELS.iter().find(|y| y.id == x.id).unwrap());

        Self {
            shared: SharedState::default(),
            board,
            beam: SimulationState::new(),

            tile_picker: TilePicker::default(),
            level_panel: LevelPanel::default(),
            confetti: Confetti::new(),

            save_file,
            needs_init: true,
            tps: 20.0,
        }
    }

    pub fn load(save_file: PathBuf) -> Self {
        GameScreen::new(Board::load(&save_file).unwrap_or_default(), save_file)
    }
}

fn create_fireworks<App>(confetti: &mut Confetti, ctx: &mut GraphicsContext<App>) {
    const POINTS: &[Vector2<f32>] = &[
        Vector2::new(0.25, 0.3),
        Vector2::new(0.5, 0.75),
        Vector2::new(0.75, 0.3),
    ];

    let mut rng = thread_rng();
    let randomness = ctx.size() * 0.25;

    for (i, center) in POINTS.iter().enumerate() {
        let offset_percent = Vector2::new(rng.gen(), rng.gen()) * 2.0 - Vector2::repeat(1.0);
        let pos = center.component_mul(&ctx.size()) + randomness.component_mul(&offset_percent);
        confetti.emit(pos, 100, 0.2 * i as f32);
    }
}
