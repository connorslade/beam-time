use std::{
    mem,
    path::PathBuf,
    time::{Duration, Instant},
};

use engine::{
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::GraphicsContext,
    screens::Screen,
};

use crate::{
    consts::BACKGROUND_COLOR,
    game::{beam::BeamState, board::Board, level::LEVELS, SharedState},
    ui::tile_picker::TilePicker,
    App,
};

pub struct GameScreen {
    save_file: PathBuf,

    shared: SharedState,
    board: Board,
    beam: Option<BeamState>,

    tile_picker: TilePicker,
    running: bool,
    last_tick: Instant,
    needs_init: bool,
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

        if self.beam.is_none() && ctx.input.key_pressed(KeyCode::Escape) {
            ctx.pop_screen()
        }

        let space_pressed = ctx.input.key_pressed(KeyCode::Space);
        let play_pressed = ctx.input.key_pressed(KeyCode::KeyP);
        let test_pressed = ctx.input.key_pressed(KeyCode::KeyT) && self.beam.is_none();

        self.running |= play_pressed || test_pressed;
        self.running &= !space_pressed;

        if let Some(beam) = &mut self.beam {
            let tick_needed = self.last_tick.elapsed() >= Duration::from_millis(50);
            if space_pressed || (self.running && tick_needed) {
                self.last_tick = Instant::now();
                beam.tick();
            }

            let is_complete = beam.level.as_ref().map(|x| x.is_complete());
            if is_complete == Some(true) {
                self.running = false;
                self.beam = None;
            } else {
                beam.render(ctx, state, &self.shared);
            }
        } else if space_pressed || play_pressed || test_pressed {
            let mut beam = BeamState::new(&self.board, test_pressed);
            beam.tick();
            self.beam = Some(beam);
        }

        if ctx.input.key_pressed(KeyCode::Escape) {
            self.beam = None;
        }

        ctx.background(BACKGROUND_COLOR);
        self.board.render(ctx, state, &self.shared, &mut self.beam);
        self.tile_picker
            .render(ctx, self.beam.is_some(), &mut self.board.transient.holding);
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
            beam: None,

            tile_picker: TilePicker::default(),
            running: false,
            last_tick: Instant::now(),

            save_file,
            needs_init: true,
        }
    }

    pub fn load(save_file: PathBuf) -> Self {
        GameScreen::new(Board::load(&save_file).unwrap_or_default(), save_file)
    }
}
