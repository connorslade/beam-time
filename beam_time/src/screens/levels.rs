use engine::{exports::nalgebra::Vector2, graphics_context::GraphicsContext, screens::Screen};

use crate::{
    consts::BACKGROUND_COLOR,
    game::{board::Board, tile::Tile},
    ui::tile_picker::tile_picker,
    App,
};

pub struct LevelsScreen {
    board: Board,
    holding: Option<Tile>,
}

impl Screen<App> for LevelsScreen {
    fn render(&mut self, _state: &mut App, ctx: &mut GraphicsContext<App>) {
        ctx.background(BACKGROUND_COLOR);

        tile_picker(ctx, &mut self.holding);
        self.board.render(ctx, &mut self.holding);
    }
}

impl Default for LevelsScreen {
    fn default() -> Self {
        Self {
            board: Board::new(Vector2::repeat(8)),
            holding: None,
        }
    }
}
