use std::{collections::VecDeque, mem};

use beam_logic::level::{tree::LevelTree, DEFAULT_LEVELS};
use engine::{
    color::Rgb,
    drawable::text::Text,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, Drawable, GraphicsContext},
};
use uuid::uuid;

use crate::{app::App, assets::UNDEAD_FONT, ui::pixel_line::PixelLine};

use super::Screen;

pub struct CampaignScreen {
    tree: LevelTree,
    layout: TreeLayout,
    pan: Vector2<f32>,
}

#[derive(Default)]
struct TreeLayout {
    rows: Vec<Vec<TreeItem>>,
}

struct TreeItem {
    name: String, // make ref
    children: Vec<usize>,
}

impl Screen for CampaignScreen {
    fn render(&mut self, _state: &mut App, ctx: &mut GraphicsContext) {
        self.pan += ctx.input.mouse_delta * ctx.input.mouse_down(MouseButton::Left) as u8 as f32;
        // let spacing = 32.0 * ctx.scale_factor;
        let spacing = 64.0 * ctx.scale_factor;

        let mut next_positions = Vec::new();
        for (i, row) in self.layout.rows.iter().enumerate() {
            let offset = Vector2::y() * i as f32 * spacing;
            let mut total_width = (row.len() - 1) as f32 * spacing;

            let mut drawables = Vec::new();
            for item in row.iter() {
                let text = Text::new(UNDEAD_FONT, &item.name).scale(Vector2::repeat(2.0));
                let size = text.size(ctx);
                total_width += size.x;
                drawables.push((text, size.x));
            }

            let mut acc = 0.0;
            let last_positions = mem::take(&mut next_positions);
            for ((drawable, width), item) in drawables.into_iter().zip(row.iter()) {
                acc += width;
                let offset = offset + Vector2::x() * (total_width / 2.0 - acc);
                let center = offset + Vector2::x() * width / 2.0;
                acc += spacing;
                next_positions.push(center);
                drawable
                    .position(self.pan + offset, Anchor::CenterLeft)
                    .z_index(1)
                    .draw(ctx);
                for line in item.children.iter() {
                    let position = last_positions[*line];
                    PixelLine::new(center, position)
                        .color(Rgb::repeat(0.5))
                        .position(self.pan)
                        .draw(ctx);
                }
                // Text::new(UNDEAD_FONT, format!("{:?}", item.children))
                //     .scale(Vector2::repeat(4.0))
                //     .position(self.pan + offset, Anchor::Center)
                //     .color(Rgb::new(1.0, 0.0, 0.0))
                //     .z_index(1)
                //     .draw(ctx);
            }
        }
    }
}

impl CampaignScreen {
    pub fn new() -> Self {
        let tree = LevelTree::new(&DEFAULT_LEVELS);
        Self {
            layout: TreeLayout::generate(&tree),
            tree,
            pan: Vector2::zeros(),
        }
    }
}

impl TreeLayout {
    pub fn generate(tree: &LevelTree) -> Self {
        let mut rows = Vec::<Vec<TreeItem>>::new();
        let mut queue = VecDeque::new();
        queue.push_back((uuid!("58fc60ca-3831-4f27-a29a-b4878a5dd68a"), Vec::new(), 0));

        while let Some((id, mut parent, depth)) = queue.pop_front() {
            let level = tree.get(id).unwrap();

            if rows.len() <= depth {
                rows.push(Vec::new());
            }

            if let Some(x) = rows[depth].iter().position(|x| x.name == level.name) {
                rows[depth][x].children.extend(parent);
                continue;
            }

            let index = rows[depth].len();
            rows[depth].push(TreeItem {
                name: level.name.clone(),
                children: parent.clone(),
            });

            parent.push(index);
            if let Some(children) = tree.children(id) {
                for child in children {
                    queue.push_back((*child, vec![index], depth + 1));
                }
            };
        }

        Self { rows }
    }
}
