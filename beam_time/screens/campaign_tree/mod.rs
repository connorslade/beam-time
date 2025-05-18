use beam_logic::level::{tree::LevelTree, DEFAULT_LEVELS};
use engine::{
    color::Rgb,
    drawable::shape::rectangle::Rectangle,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, Drawable, GraphicsContext},
};

use crate::{app::App, ui::pixel_line::PixelLine};

use super::Screen;

mod layout;
use layout::TreeLayout;

pub struct CampaignScreen {
    tree: LevelTree,
    layout: TreeLayout,
    pan: Vector2<f32>,
}

impl Screen for CampaignScreen {
    fn render(&mut self, _state: &mut App, ctx: &mut GraphicsContext) {
        self.pan += ctx.input.mouse_delta * ctx.input.mouse_down(MouseButton::Left) as u8 as f32;
        let spacing = 64.0 * ctx.scale_factor;

        if self.layout.is_empty() || ctx.input.dpi_changed() {
            self.layout = TreeLayout::generate(&self.tree, ctx);
        }

        for (i, row) in self.layout.rows.iter().enumerate() {
            let offset = Vector2::y() * i as f32 * spacing;

            for item in row {
                let center = offset + Vector2::x() * item.offset();

                item.text
                    .clone()
                    .position(self.pan + center, Anchor::Center)
                    .z_index(1)
                    .draw(ctx);
                Rectangle::new(Vector2::new(item.total_width, 4.0))
                    .position(self.pan + center, Anchor::Center)
                    .color(Rgb::new(1.0, 0.0, 0.0))
                    .draw(ctx);

                for child in item.children.iter() {
                    let offset = self.layout.rows[i + 1][*child].offset();
                    PixelLine::new(center, Vector2::new(offset, (i + 1) as f32 * spacing))
                        .color(Rgb::repeat(0.5))
                        .position(self.pan)
                        .draw(ctx);
                }
            }
        }
    }
}

impl CampaignScreen {
    pub fn new() -> Self {
        let tree = LevelTree::new(&DEFAULT_LEVELS);
        Self {
            tree,
            layout: TreeLayout::default(),
            pan: Vector2::zeros(),
        }
    }
}
