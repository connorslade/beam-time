use std::collections::VecDeque;

use beam_logic::level::tree::LevelTree;
use engine::{drawable::text::Text, exports::nalgebra::Vector2, graphics_context::GraphicsContext};
use ordered_float::OrderedFloat;
use uuid::{Uuid, uuid};

use crate::assets::UNDEAD_FONT;

#[derive(Default)]
pub struct TreeLayout {
    pub rows: Vec<Vec<TreeItem>>,
}

pub struct TreeItem {
    pub id: Uuid,
    pub text: Text,
    pub children: Vec<usize>,

    pub downstream_width: Vec<f32>,
    pub height: u32,

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
                downstream_width: Vec::new(),
                height: 1,
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
    let row_count = rows.len();
    for depth in (0..row_count).rev() {
        if depth + 1 == rows.len() {
            for item in rows[depth].iter_mut() {
                item.downstream_width.push(item.width);
            }
            continue;
        }

        let (current_row, next_row) = rows.split_at_mut(depth + 1);
        let (current_row, next_row) = (&mut current_row[depth], &next_row[0]);

        for item in current_row {
            let height = row_count - depth;
            let mut max_height = 0;
            let mut downstream = vec![0.0; height];
            downstream[0] = item.width;
            downstream[1] = spacing * (item.children.len().saturating_sub(1)) as f32;

            for &child in &item.children {
                let child = &next_row[child];
                max_height = max_height.max(child.height);
                for (idx, val) in child.downstream_width.iter().enumerate() {
                    downstream[idx + 1] += val;
                }
            }

            item.height += max_height;
            item.downstream_width = downstream;
        }
    }
}

fn layout_items(rows: &mut [Vec<TreeItem>], spacing: f32) {
    for depth in 0..rows.len() - 1 {
        let (current_row, next_row) = rows.split_at_mut(depth + 1);
        let (current_row, next_row) = (&current_row[depth], &mut next_row[0]);

        for item in current_row {
            let mut acc = 0.0;

            for (idx, &child) in item.children.iter().enumerate() {
                next_row[child].offset = acc;

                if idx + 1 < item.children.len() {
                    let a = next_row[item.children[idx + 1]].width(next_row[child].height) / 2.0;
                    let b = next_row[child].width(next_row[item.children[idx + 1]].height) / 2.0;
                    acc += a + b + spacing;
                }
            }

            for &child in &item.children {
                let child = &mut next_row[child];
                child.offset -= acc / 2.0;
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

    pub fn width(&self, depth: u32) -> f32 {
        self.downstream_width
            .iter()
            .take(depth as usize)
            .copied()
            .map(OrderedFloat)
            .max()
            .unwrap()
            .0
    }
}
