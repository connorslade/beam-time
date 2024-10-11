use engine::{
    exports::winit::keyboard::KeyCode, graphics_context::GraphicsContext, screens::Screen,
};

use crate::{
    consts::BACKGROUND_COLOR,
    game::{beam::BeamState, board::Board, tile::Tile, SharedState},
    ui::tile_picker::TilePicker,
    App,
};

pub struct GameScreen {
    shared: SharedState,
    board: Board,
    beam: Option<BeamState>,

    tile_picker: TilePicker,
    holding: Option<Tile>,
}

impl Screen<App> for GameScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        self.shared.update(ctx);

        let space_pressed = ctx.input.key_pressed(KeyCode::Space);
        if let Some(beam) = &mut self.beam {
            space_pressed.then(|| beam.tick());
            beam.render(ctx, state, &self.shared);
        } else if space_pressed {
            let mut beam = BeamState::new(&self.board);
            beam.tick();
            self.beam = Some(beam);
        }

        if ctx.input.key_pressed(KeyCode::Escape) {
            self.beam = None;
        }

        ctx.background(BACKGROUND_COLOR);
        self.board
            .render(ctx, state, &self.shared, &mut self.beam, &mut self.holding);
        self.tile_picker
            .render(ctx, &self.shared, self.beam.is_some(), &mut self.holding);
    }
}

impl Default for GameScreen {
    fn default() -> Self {
        Self {
            shared: SharedState::default(),
            board: Board::new(),
            beam: None,

            tile_picker: TilePicker::default(),
            holding: None,
        }
    }
}
