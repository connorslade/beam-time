use std::collections::VecDeque;

use beam_logic::level::tree::LevelTree;
use engine::{drawable::text::Text, exports::nalgebra::Vector2, graphics_context::GraphicsContext};
use uuid::{Uuid, uuid};

use crate::assets::UNDEAD_FONT;

#[derive(Default)]
pub struct TreeLayout {
    pub rows: Vec<Vec<TreeItem>>,
    spacing: f32,
}

pub struct TreeItem {
    pub id: Uuid,
    pub text: Text,
    pub children: Vec<usize>,

    pub height: u32,
    pub width: f32,
    total_width: f32,
    offset: f32,
    parent_offset: f32,
}

impl TreeLayout {
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn generate(tree: &LevelTree, ctx: &mut GraphicsContext) -> Self {
        let mut rows = Vec::<Vec<TreeItem>>::new();
        let mut queue = VecDeque::new();
        queue.push_back((uuid!("58fc60ca-3831-4f27-a29a-b4878a5dd68a"), None, 0));

        while let Some((id, parent, depth)) = queue.pop_front() {
            let level = tree.get(id).unwrap();
            (rows.len() <= depth).then(|| rows.push(Vec::new()));

            let index = rows[depth].len();
            let text = Text::new(UNDEAD_FONT, &level.name).scale(Vector2::repeat(2.0));
            let width = text.size(ctx).x;
            rows[depth].push(TreeItem {
                id: level.id,
                width,
                height: 1,
                total_width: width,
                offset: 0.0,
                parent_offset: 0.0,
                children: Vec::new(),
                text,
            });

            if let Some(parent) = parent {
                let parent: &mut TreeItem = &mut rows[depth - 1][parent];
                parent.children.push(index);
            }

            if let Some(children) = tree.children(id) {
                for child in children {
                    queue.push_back((*child, Some(index), depth + 1));
                }
            };
        }

        let spacing = 64.0 * ctx.scale_factor;
        let mut this = Self { rows, spacing };
        this.naive_layout();
        this.propagate_offsets();

        this
    }

    fn naive_layout(&mut self) {
        for depth in (0..self.rows.len() - 1).rev() {
            let (current_row, next_row) = self.rows.split_at_mut(depth + 1);
            let (current_row, next_row) = (&mut current_row[depth], &mut next_row[0]);

            for item in current_row.iter_mut() {
                let children = item.children.iter().map(|x| &next_row[*x]);
                let child_width =
                    children.fold(-self.spacing, |acc, x| acc + x.total_width + self.spacing);
                item.total_width = item.total_width.max(child_width);

                let mut acc = 0.0;
                for &child in &item.children {
                    let child = &mut next_row[child];
                    child.offset = acc + child.total_width / 2.0;
                    acc += child.total_width + self.spacing;
                }

                let mut max_height = 0;
                for &child in &item.children {
                    let child = &mut next_row[child];
                    child.offset -= (acc - self.spacing) / 2.0;
                    max_height = max_height.max(child.height);
                }

                item.height += max_height;
            }
        }
    }

    fn propagate_offsets(&mut self) {
        for depth in 0..self.rows.len() - 1 {
            let (current_row, next_row) = self.rows.split_at_mut(depth + 1);
            let (current_row, next_row) = (&current_row[depth], &mut next_row[0]);

            for item in current_row {
                for &child in &item.children {
                    let child = &mut next_row[child];
                    child.parent_offset += item.offset + item.parent_offset;
                }
            }
        }
    }
}

impl TreeItem {
    pub fn offset(&self) -> f32 {
        self.offset + self.parent_offset
    }
}
