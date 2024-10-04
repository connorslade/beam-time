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
    beam: BeamState,
    holding: Option<Tile>,
}

impl Screen<App> for LevelsScreen {
    fn render(&mut self, _state: &mut App, ctx: &mut GraphicsContext<App>) {
        ctx.input
            .key_pressed(KeyCode::Space)
            .then(|| self.beam.tick());

        ctx.background(BACKGROUND_COLOR);

        tile_picker(ctx, &mut self.holding);
        self.board.render(ctx, &mut self.holding);
        self.beam.render(ctx);
    }
}

impl Default for LevelsScreen {
    fn default() -> Self {
        let size = Vector2::repeat(8);
        Self {
            board: Board::new(size),
            beam: BeamState::new(size),
            holding: None,
        }
    }
}
