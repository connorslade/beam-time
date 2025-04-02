use std::{borrow::Cow, mem, path::PathBuf, time::Duration};

use rand::{seq::SliceRandom, thread_rng, Rng};

#[cfg(feature = "steam")]
use crate::game::achievements::award_campaign_achievements;
use crate::{
    assets::UNDEAD_FONT,
    consts::{layer, BACKGROUND_COLOR},
    game::{board::Board, render::beam::BeamStateRender, shared_state::SharedState},
    ui::{
        confetti::Confetti, layout::column::ColumnLayout, level_panel::LevelPanel,
        misc::modal_buttons, modal::Modal, tile_picker::TilePicker,
    },
    util::{human_duration, key_events},
    App,
};
use beam_logic::{
    level::DEFAULT_LEVELS,
    simulation::{
        level_state::LevelResult, runtime::asynchronous::AsyncSimulationState, state::BeamState,
    },
};
use engine::{
    color::Rgb,
    drawable::text::Text,
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::GraphicsContext,
};

use super::Screen;

pub struct GameScreen {
    shared: SharedState,
    board: Board,
    beam: AsyncSimulationState,

    tile_picker: TilePicker,
    level_panel: LevelPanel,
    confetti: Confetti,
    paused: Option<PausedModal>,

    level_result: Option<LevelResult>,
    save_file: PathBuf,
    needs_init: bool,
    tps: f32,
}

struct PausedModal {}

impl Screen for GameScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        self.paused_modal(state, ctx);
        if self.paused.is_none() {
            self.shared.update(state, ctx);
        }

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
        state.debug(|| format!("Tick: {:.2?}", sim.tick_length));
        sim.runtime.time_per_tick = Duration::from_secs_f32(self.tps.max(1.0).recip());

        if sim.beam.is_none() && ctx.input.key_pressed(KeyCode::Escape) {
            self.paused = self.paused.is_none().then_some(PausedModal {});
            // ctx.pop_screen();
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

            beam_state.render(ctx, state, &self.shared);

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
                test_pressed,
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
        self.board.render(ctx, state, &self.shared, &mut sim.beam);

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
            .map(|x| DEFAULT_LEVELS.iter().find(|y| y.id == x.id).unwrap());

        Self {
            shared: SharedState::default(),
            board,
            beam: AsyncSimulationState::new(),

            tile_picker: TilePicker::default(),
            level_panel: LevelPanel::default(),
            confetti: Confetti::new(),
            paused: None,

            level_result: None,
            save_file,
            needs_init: true,
            tps: 20.0,
        }
    }

    pub fn load(save_file: PathBuf) -> Self {
        GameScreen::new(Board::load(&save_file).unwrap_or_default(), save_file)
    }

    fn paused_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        if let Some(pause) = &mut self.paused {
            ctx.defer(|ctx| ctx.darken(Rgb::repeat(0.5), layer::UI_OVERLAY));

            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(Vector2::new(ctx.center().x, 500.0))
                .margin(margin)
                .layer(layer::UI_OVERLAY);

            let size = modal.inner_size();
            modal.draw(ctx, |ctx| {
                let body = |text| {
                    Text::new(UNDEAD_FONT, text)
                        .scale(Vector2::repeat(2.0))
                        .max_width(size.x)
                };

                let mut layout = ColumnLayout::new(padding);
                let name = match self.board.transient.level {
                    Some(level) => format!("Campaign: {}", level.name),
                    None => format!("Sandbox: {}", self.board.meta.name),
                };

                layout.draw(ctx, body(&name).scale(Vector2::repeat(4.0)));

                let playtime = self.board.meta.playtime
                    + self.board.transient.open_timestamp.elapsed().as_secs();
                let playtime = format!("Playtime: {}", human_duration(playtime));
                layout.draw(ctx, body(&playtime));

                layout.space_to(size.y - ctx.scale_factor * 12.0);
                layout.row(ctx, |ctx| {
                    modal_buttons(ctx, size.x, ("Exit", "Resume"));
                });
            });
        }
    }
}

fn create_confetti(confetti: &mut Confetti, ctx: &mut GraphicsContext) {
    let mut points = [
        Vector2::new(0.25, 0.3),
        Vector2::new(0.5, 0.75),
        Vector2::new(0.75, 0.3),
    ];

    let mut rng = thread_rng();
    points.shuffle(&mut rng);

    let randomness = ctx.size() * 0.25;

    for (i, center) in points.iter().enumerate() {
        let offset_percent = Vector2::new(rng.gen(), rng.gen()) * 2.0 - Vector2::repeat(1.0);
        let pos = center.component_mul(&ctx.size()) + randomness.component_mul(&offset_percent);
        confetti.emit(pos, 100, 0.2 * i as f32);
    }
}
