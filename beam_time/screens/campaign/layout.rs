// The amount of time I've spent on this awful little tree layout system is
// kinda sad. It took me like three revisions to get to a point where it doesn't
// lay out nodes with overlapping text. I could have saved like 10 hours of work
// and just positioned the levels manually. It would have been faster and would
// probably look nicer.

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
    parents: Vec<usize>,
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
                rows[depth][x].parents.push(parent);
                let parent: &mut TreeItem = &mut rows[depth - 1][parent];
                parent.children.push(x);
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
                parents: Vec::new(),
                children: Vec::new(),
                text,
            });

            if let Some(parent) = parent {
                rows[depth][index].parents.push(parent);
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
        propagate_height(&mut rows);
        propagate_widths(&mut rows, spacing);
        layout_items(&mut rows, spacing);
        propagate_offsets(&mut rows);

        Self { rows }
    }
}

fn propagate_height(rows: &mut [Vec<TreeItem>]) {
    for depth in (0..rows.len() - 1).rev() {
        let (current_row, next_row) = rows.split_at_mut(depth + 1);
        let (current_row, next_row) = (&mut current_row[depth], &mut next_row[0]);

        for item in current_row.iter_mut() {
            let mut max_height = 0;
            for &child in &item.children {
                let child = &next_row[child];
                max_height = max_height.max(child.height);
            }

            item.height += max_height;
        }
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
        let (current_row, next_row) = (&mut current_row[depth], &mut next_row[0]);

        for idx_current in 0..current_row.len() {
            let item = &current_row[idx_current];

            let height = row_count - depth;
            let mut max_height = 0;
            let mut downstream = vec![0.0; height];
            downstream[0] = item.width;
            downstream[1] = spacing * (item.children.len().saturating_sub(1)) as f32;

            for (idx_child, &child) in item.children.iter().enumerate() {
                let next_height = next_height(item, current_row, next_row, idx_current, idx_child);
                next_row[child].downstream_width[0] = next_row[child].width(next_height);

                let child = &next_row[child];
                max_height = max_height.max(child.height);
                for (idx, val) in child.downstream_width.iter().enumerate() {
                    downstream[idx + 1] += val;
                }
            }

            let item = &mut current_row[idx_current];
            item.downstream_width = downstream;
        }
    }
}

fn layout_items(rows: &mut [Vec<TreeItem>], spacing: f32) {
    for depth in 0..rows.len() - 1 {
        let (current_row, next_row) = rows.split_at_mut(depth + 1);
        let (current_row, next_row) = (&current_row[depth], &mut next_row[0]);

        for (idx_row, item) in current_row.iter().enumerate() {
            let mut acc = 0.0;

            for (idx, &child) in item.children.iter().enumerate() {
                next_row[child].offset = acc;

                if idx + 1 < item.children.len() {
                    let height = next_row[child + 1].height;

                    let mut width = 0_f32;
                    for start in 0..idx {
                        let mut this_width = next_row[item.children[start]].width(height);
                        for i in start..idx {
                            let next_height = next_height(item, current_row, next_row, idx_row, i);
                            this_width -= next_row[item.children[i]].width(next_height) + spacing;
                        }

                        width = width.max(this_width);
                    }

                    let prev_width = next_row[child].width(height).max(width);
                    acc += prev_width + spacing;
                }
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
        let max = self
            .downstream_width
            .iter()
            .take(depth as usize)
            .copied()
            .map(OrderedFloat)
            .max()
            .unwrap_or_default();
        self.width.max(*max)
    }
}

fn next_height(
    item: &TreeItem,
    row: &[TreeItem],
    next_row: &[TreeItem],
    row_idx: usize,
    item_idx: usize,
) -> u32 {
    if item_idx + 1 < item.children.len() {
        next_row[item.children[item_idx + 1]].height
    } else if row_idx + 1 < row.len() {
        row[row_idx + 1].height - 1
    } else {
        0
    }
}
