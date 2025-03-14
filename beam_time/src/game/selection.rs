use crate::{
    app::App,
    assets::{OVERLAY_SELECTION, UNDEAD_FONT},
    consts::layer,
    util::key_events,
};
use ahash::HashSet;
use beam_logic::{simulation::state::BeamState, tile::Tile};
use common::{direction::Direction, map::Map, misc::in_bounds};
use engine::{
    color::Rgb,
    drawable::{sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};
use thousands::Separable;

use super::{history::History, holding::Holding, shared_state::SharedState};

#[derive(Default)]
pub struct SelectionState {
    selection: HashSet<Vector2<i32>>,
    selection_start: Option<Vector2<i32>>,

    working_selection: Option<(Vector2<i32>, Vector2<i32>)>,
    last_holding: Holding,
}

impl SelectionState {
    pub fn update(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        shared: &SharedState,
        sim: &mut Option<BeamState>,
        tiles: &mut Map<Tile>,
        history: &mut History,
        holding: &mut Holding,
    ) {
        self.working_selection = self.selection_start.map(|start| {
            let end = shared
                .screen_to_world_space(ctx, ctx.input.mouse)
                .map(|x| x.ceil() as i32);

            (
                Vector2::new(start.x.min(end.x), start.y.min(end.y)),
                Vector2::new(start.x.max(end.x), start.y.max(end.y)),
            )
        });

        let ctrl = ctx.input.key_down(KeyCode::ControlLeft);
        let alt = ctx.input.key_down(KeyCode::AltLeft);
        let copy = ctx.input.key_pressed(KeyCode::KeyC);
        let cut = ctx.input.key_pressed(KeyCode::KeyX);
        let paste = ctx.input.key_pressed(KeyCode::KeyV);

        if let (Some((min, max)), false) = (self.working_selection, ctrl || alt) {
            let middle = ((min + max).map(|x| x as f32) - Vector2::repeat(1.0)) / 2.0;
            let screen = shared.world_to_screen_space(ctx, middle);
            // todo clip to screen

            let size = max - min + Vector2::repeat(1);
            let price = (min.x..=max.x)
                .flat_map(|x| (min.y..=max.y).map(move |y| Vector2::new(x, y)))
                .map(|pos| tiles.get(pos).price())
                .sum::<u32>();
            let text = format!("{}x{} • ${}", size.x, size.y, price.separate_with_commas());
            ctx.draw(
                Text::new(UNDEAD_FONT, &text)
                    .position(screen, Anchor::Center)
                    .scale(Vector2::repeat(2.0))
                    .color(Rgb::hex(0xe27285)),
            );
        }

        if let (Some(selection), false) = (
            self.working_selection,
            ctx.input.mouse_down(MouseButton::Left),
        ) {
            self.selection_start = None;
            let new_selection = (selection.0.x..=selection.1.x)
                .flat_map(|x| (selection.0.y..=selection.1.y).map(move |y| Vector2::new(x, y)))
                .collect();

            // if ctrl down, add to selection
            // if alt down, remove from selection
            if ctx.input.key_down(KeyCode::ControlLeft) {
                self.selection.extend(new_selection);
            } else if ctx.input.key_down(KeyCode::AltLeft) {
                // remove new_selection from selection
                self.selection = self.selection.difference(&new_selection).copied().collect();
            } else {
                self.selection = new_selection;
            }
        }

        key_events!(ctx, {
            KeyCode::KeyU => {
                self.selection_start = None;
                self.selection.clear();
            },
            KeyCode::Delete => {
                let mut old = Vec::new();
                for pos in self.selection.iter() {
                    old.push((*pos, tiles.get(*pos)));
                    tiles.remove(*pos);
                }
                *sim = None;
                history.track_many(old);
                self.selection.clear();
            }
        });

        if ctrl && (copy || cut) {
            let mut list = Vec::new();
            let mut old = Vec::new();

            for pos in self.selection.iter() {
                let tile = tiles.get(*pos);
                old.push((*pos, tile));

                if !tile.is_empty() {
                    list.push((*pos, tile));
                }

                cut.then(|| tiles.remove(*pos));
            }

            history.track_many(old);

            let origin = shared
                .screen_to_world_space(ctx, ctx.input.mouse)
                .map(|x| x.ceil() as i32);
            list.iter_mut().for_each(|(pos, _)| *pos -= origin);

            *sim = None;
            *holding = Holding::Paste(list);
            self.last_holding = holding.clone();
            self.selection.clear();
        }

        if ctrl && paste {
            *sim = None;
            *holding = self.last_holding.clone();
        }
    }

    pub fn update_tile(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        shared: &SharedState,
        hovered: bool,
        pos: Vector2<i32>,
        render_pos: Vector2<f32>,
    ) {
        let ctrl = ctx.input.key_down(KeyCode::ControlLeft);
        let alt = ctx.input.key_down(KeyCode::AltLeft);
        let shift = ctx.input.key_down(KeyCode::ShiftLeft);

        let in_selection = |pos| {
            let selection = self.selection.contains(&pos);
            let working = self
                .working_selection
                .is_some_and(|bound| in_bounds(pos, bound));

            if ctrl {
                working || selection
            } else if alt {
                selection && !working
            } else if shift && self.working_selection.is_some() {
                working
            } else {
                selection
            }
        };

        // draw overlay_selection if the tile is in the selection and the direction is not
        if in_selection(pos) {
            for dir in Direction::ALL {
                let offset_point = dir.offset(pos);
                if !in_selection(offset_point) {
                    let selection_overlay = Sprite::new(OVERLAY_SELECTION)
                        .scale(Vector2::repeat(shared.scale))
                        .position(render_pos, Anchor::Center)
                        .rotate(dir.to_angle(), Anchor::Center)
                        .z_index(layer::TILE_BACKGROUND_OVERLAY);
                    ctx.draw(selection_overlay);
                }
            }
        }

        if hovered
            && ctx.input.key_down(KeyCode::ShiftLeft)
            && ctx.input.mouse_pressed(MouseButton::Left)
        {
            self.selection_start = Some(pos);
        }
    }
}
