use std::collections::VecDeque;

use beam_logic::level::tree::LevelTree;
use engine::{drawable::text::Text, exports::nalgebra::Vector2, graphics_context::GraphicsContext};
use uuid::{uuid, Uuid};

use crate::assets::UNDEAD_FONT;

#[derive(Default)]
pub struct TreeLayout {
    pub rows: Vec<Vec<TreeItem>>,
}

pub struct TreeItem {
    pub id: Uuid,
    pub text: Text,
    pub total_width: f32,
    pub children: Vec<usize>,

    width: f32,
    offset: f32,
    parent_offset: f32,
    parents: u32,
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

            if rows.len() <= depth {
                rows.push(Vec::new());
            }

            let duplicate = rows[depth].iter().position(|x| x.id == level.id);
            if let (Some(parent), Some(x)) = (parent, duplicate) {
                let parent: &mut TreeItem = &mut rows[depth - 1][parent];
                parent.children.push(x);
                rows[depth][x].parents += 1;
                continue;
            }

            let index = rows[depth].len();
            let text = Text::new(UNDEAD_FONT, &level.name).scale(Vector2::repeat(2.0));
            rows[depth].push(TreeItem {
                id: level.id,
                width: text.size(ctx).x,
                total_width: 0.0,
                offset: 0.0,
                parent_offset: 0.0,
                parents: 0,
                children: Vec::new(),
                text,
            });

            if let Some(parent) = parent {
                let parent: &mut TreeItem = &mut rows[depth - 1][parent];
                parent.children.push(index);
                rows[depth][index].parents += 1;
            }

            if let Some(children) = tree.children(id) {
                for child in children {
                    queue.push_back((*child, Some(index), depth + 1));
                }
            };
        }

        let spacing = 64.0 * ctx.scale_factor;
        propagate_widths(&mut rows, spacing);
        layout_items(&mut rows, spacing);
        propagate_offsets(&mut rows);

        Self { rows }
    }
}

fn propagate_widths(rows: &mut [Vec<TreeItem>], spacing: f32) {
    for depth in (0..rows.len()).rev() {
        if depth + 1 == rows.len() {
            for item in rows[depth].iter_mut() {
                item.total_width = item.width;
            }
            continue;
        }

        let (current_row, next_row) = rows.split_at_mut(depth + 1);
        let (current_row, next_row) = (&mut current_row[depth], &next_row[0]);

        for item in current_row {
            let mut width = 0.0;
            let mut siblings = 0;
            for &child in &item.children {
                let child = &next_row[child];
                siblings = child.parents;
                width += child.total_width;
            }

            width += (item.children.len().saturating_sub(1)) as f32 * spacing;
            width /= siblings as f32;

            item.total_width = width.max(item.width);
        }
    }
}

fn layout_items(rows: &mut [Vec<TreeItem>], spacing: f32) {
    for depth in 0..rows.len() - 1 {
        let (current_row, next_row) = rows.split_at_mut(depth + 1);
        let (current_row, next_row) = (&current_row[depth], &mut next_row[0]);

        for item in current_row {
            let mut acc = -item.total_width / 2.0;

            for &child in &item.children {
                let child = &mut next_row[child];

                child.offset = acc + child.total_width / 2.0;
                acc += child.total_width + spacing;
            }
        }
    }
}

fn propagate_offsets(rows: &mut [Vec<TreeItem>]) {
    for depth in 0..rows.len() - 1 {
        let (current_row, next_row) = rows.split_at_mut(depth + 1);
        let (current_row, next_row) = (&current_row[depth], &mut next_row[0]);

        for item in current_row {
            for &child in &item.children {
                let child = &mut next_row[child];
                child.parent_offset += item.offset + item.parent_offset;
            }
        }
    }
}

impl TreeItem {
    pub fn offset(&self) -> f32 {
        self.offset + self.parent_offset
    }
}
