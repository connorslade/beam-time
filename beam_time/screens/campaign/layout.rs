use std::collections::VecDeque;

use beam_logic::level::tree::LevelTree;
use engine::{drawable::text::Text, exports::nalgebra::Vector2, graphics_context::GraphicsContext};
use log::warn;
use uuid::{Uuid, uuid};

use crate::{assets::UNDEAD_FONT, screens::campaign::SPACING};

#[derive(Default)]
pub struct TreeLayout {
    pub rows: Vec<Vec<TreeItem>>,
}

pub struct TreeItem {
    pub id: Uuid,
    pub text: Text,
    pub children: Vec<usize>,

    pub height: u32,
    pub width: f32,
    contours: Vec<(f32, f32)>,
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
            let Some(level) = tree.get(id) else {
                warn!("Child level `{id}` not found.");
                continue;
            };

            (rows.len() <= depth).then(|| rows.push(Vec::new()));

            let index = rows[depth].len();
            let text = Text::new(UNDEAD_FONT, &level.name).scale(Vector2::repeat(2.0));
            let width = text.size(ctx).x;
            rows[depth].push(TreeItem {
                id: level.id,
                width,
                contours: vec![(-width / 2.0, width / 2.0)],
                height: 1,
                offset: 0.0,
                parent_offset: 0.0,
                children: Vec::new(),
                text,
            });

            if let Some(parent) = parent {
                let parent: &mut TreeItem = &mut rows[depth - 1][parent];
                parent.children.push(index);
            }

            for child in &level.children {
                queue.push_back((*child, Some(index), depth + 1));
            }
        }

        let mut this = Self { rows };
        this.compact_layout();
        this.propagate_offsets();

        this
    }

    fn compact_layout(&mut self) {
        for depth in (0..self.rows.len() - 1).rev() {
            let (current_row, next_row) = self.rows.split_at_mut(depth + 1);
            let (current_row, next_row) = (&mut current_row[depth], &mut next_row[0]);

            for item in current_row.iter_mut().filter(|x| !x.children.is_empty()) {
                for i in 1..item.children.len() {
                    let child = &next_row[item.children[i]];
                    let mut shift = 0.0_f32;

                    for &prev_child in &item.children[0..i] {
                        shift = shift.max(item_separation(&next_row[prev_child], child));
                    }

                    for j in i..item.children.len() {
                        next_row[item.children[j]].offset += shift + SPACING;
                    }
                }

                let first_child = &next_row[item.children[0]];
                let last_child = &next_row[*item.children.last().unwrap()];
                let shift = -((first_child.offset - first_child.width / 2.0)
                    + (last_child.offset + last_child.width / 2.0))
                    / 2.0;

                for &child_idx in &item.children {
                    let child = &mut next_row[child_idx];
                    child.offset += shift;

                    if child.contours.len() >= item.contours.len() {
                        item.contours
                            .resize(child.contours.len() + 1, (f32::INFINITY, f32::NEG_INFINITY));
                    }

                    for i in 0..child.contours.len() {
                        let contours = item.contours[i + 1];
                        item.contours[i + 1] = (
                            contours.0.min(child.offset + child.contours[i].0),
                            contours.1.max(child.offset + child.contours[i].1),
                        );
                    }
                }

                let max_height = item.children.iter().map(|&x| next_row[x].height).max();
                item.height += max_height.unwrap_or_default();
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

fn item_separation(a: &TreeItem, b: &TreeItem) -> f32 {
    let max_depth = b.contours.len().max(a.contours.len());
    let mut separation = 0_f32;

    for l in 0..max_depth {
        let left = b.contours.get(l).map(|x| x.0).unwrap_or(f32::INFINITY);
        let right = a.contours.get(l).map(|x| x.1).unwrap_or(f32::NEG_INFINITY);

        separation = separation.max((right - left) - (b.offset - a.offset));
    }

    separation
}
