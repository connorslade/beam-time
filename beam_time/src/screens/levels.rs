use engine::{
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::GraphicsContext,
    screens::Screen,
};

use crate::{
    consts::BACKGROUND_COLOR,
    game::{beam::BeamState, board::Board, tile::Tile},
    ui::tile_picker::tile_picker,
    App,
};

pub struct LevelsScreen {
    board: Board,
    beam: Option<BeamState>,
    holding: Option<Tile>,
}

impl Screen<App> for LevelsScreen {
    fn render(&mut self, _state: &mut App, ctx: &mut GraphicsContext<App>) {
        let space_pressed = ctx.input.key_pressed(KeyCode::Space);
        if let Some(beam) = &mut self.beam {
            space_pressed.then(|| beam.tick());
            beam.render(ctx);
        } else if space_pressed {
            let mut beam = BeamState::new(&self.board);
            beam.tick();
            self.beam = Some(beam);
        }

        if ctx.input.key_pressed(KeyCode::Escape) {
            self.beam = None;
        }

        ctx.background(BACKGROUND_COLOR);
        self.board.render(ctx, &mut self.beam, &mut self.holding);
        if self.beam.is_none() {
            tile_picker(ctx, &mut self.holding);
        }
    }
}

impl Default for LevelsScreen {
    fn default() -> Self {
        let size = Vector2::repeat(8);
        Self {
            board: Board::new(size),
            beam: None,
            holding: None,
        }
    }
}
