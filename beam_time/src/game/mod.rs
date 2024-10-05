use engine::{exports::nalgebra::Vector2, graphics_context::GraphicsContext};

pub mod beam;
pub mod board;
pub mod tile;

// todo: dont recompute tile_size and size up to 8*8*3 (192) times per frame
fn tile_pos<App>(
    ctx: &GraphicsContext<App>,
    size: Vector2<usize>,
    pos: Vector2<usize>,
) -> Vector2<f32> {
    let tile_size = 16.0 * 4.0 * ctx.scale_factor;
    let size = size.map(|x| x as f32) * tile_size;

    ctx.center() - pos.map(|x| x as f32) * tile_size + size / 2.0 - Vector2::repeat(tile_size / 2.0)
}
