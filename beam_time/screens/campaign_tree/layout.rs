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
    id: Uuid,
    pub text: Text,

    pub width: f32, // todo make priv
    pub total_width: f32,

    pub offset: f32,
    pub children: Vec<usize>,
}

impl TreeLayout {
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn generate(tree: &LevelTree, ctx: &mut GraphicsContext) -> Self {
        let mut rows = Vec::<Vec<TreeItem>>::new();
        let mut queue = VecDeque::new();
        queue.push_back((uuid!("58fc60ca-3831-4f27-a29a-b4878a5dd68a"), Vec::new(), 0));

        while let Some((id, mut parent, depth)) = queue.pop_front() {
            let level = tree.get(id).unwrap();

            if rows.len() <= depth {
                rows.push(Vec::new());
            }

            if let Some(x) = rows[depth].iter().position(|x| x.id == level.id) {
                rows[depth][x].children.extend(parent);
                continue;
            }

            let first_child = rows.get(depth + 1).map(|x| x.len()).unwrap_or_default();
            let child_count = tree.children(id).map(|x| x.len()).unwrap_or_default();

            let index = rows[depth].len();
            let text = Text::new(UNDEAD_FONT, &level.name).scale(Vector2::repeat(2.0));
            rows[depth].push(TreeItem {
                id: level.id,
                width: text.size(ctx).x,
                total_width: 0.0,
                offset: 0.0,
                children: (first_child..(first_child + child_count)).collect(),
                text,
            });

            parent.push(index);
            if let Some(children) = tree.children(id) {
                for child in children {
                    queue.push_back((*child, vec![index], depth + 1));
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
            let mut width = (item.children.len().saturating_sub(1)) as f32 * spacing;
            for &child in &item.children {
                width += next_row[child].total_width;
            }

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
                next_row[child].offset += item.offset;
            }
        }
    }
}
