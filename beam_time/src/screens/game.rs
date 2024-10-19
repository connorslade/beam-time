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
    game::{beam::BeamState, board::Board, SharedState},
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
}

impl Screen<App> for GameScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        self.shared.update(ctx, state);

        let space_pressed = ctx.input.key_pressed(KeyCode::Space);
        let play_pressed = ctx.input.key_pressed(KeyCode::KeyP);

        self.running |= play_pressed;
        self.running &= !space_pressed;

        if let Some(beam) = &mut self.beam {
            let tick_needed = self.last_tick.elapsed() >= Duration::from_millis(100);
            if space_pressed || (self.running && tick_needed) {
                self.last_tick = Instant::now();
                beam.tick();
            }

            beam.render(ctx, state, &self.shared);
        } else if space_pressed || play_pressed {
            let mut beam = BeamState::new(&self.board);
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
    pub fn new(save_file: PathBuf) -> Self {
        Self {
            shared: SharedState::default(),
            board: Board::load(&save_file).unwrap_or_default(),
            beam: None,

            tile_picker: TilePicker::default(),
            running: false,
            last_tick: Instant::now(),

            save_file,
        }
    }
}
