use crate::{
    app::App,
    assets::UNDEAD_FONT,
    consts::{
        color,
        keybind::{self, SHIFT},
        layer,
    },
    util::key_events,
};
use ahash::HashSet;
use base64::{Engine, prelude::BASE64_STANDARD};
use beam_logic::{level::Level, simulation::state::BeamState, tile::Tile};
use bincode::Options;
use common::{consts::BINCODE_OPTIONS, direction::Direction, map::Map, misc::in_bounds};
use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable},
    drawable::{shape::rectangle::Rectangle, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
};
use thousands::Separable;

use super::{board::Board, holding::Holding, pancam::Pancam};

#[derive(Default)]
pub struct SelectionState {
    selection: HashSet<Vector2<i32>>,
    pub(super) selection_start: Option<Vector2<i32>>,

    working_selection: Option<(Vector2<i32>, Vector2<i32>)>,
}

impl Board {
    pub(super) fn update_selection(
        &mut self,
        ctx: &mut GraphicsContext,
        state: &mut App,
        pancam: &Pancam,
        sim: &mut Option<BeamState>,
    ) {
        let this = &mut self.transient.selection;

        this.working_selection = this.selection_start.map(|start| {
            let end = pancam
                .screen_to_world_space(ctx.input.mouse())
                .map(|x| x.ceil() as i32);

            (
                Vector2::new(start.x.min(end.x), start.y.min(end.y)),
                Vector2::new(start.x.max(end.x), start.y.max(end.y)),
            )
        });

        let ctrl = ctx.input.key_down(keybind::CTRL);
        let alt = ctx.input.key_down(keybind::OVERWRITE);
        let copy = ctx.input.key_pressed(keybind::COPY);
        let cut = ctx.input.key_pressed(keybind::CUT);
        let paste = ctx.input.key_pressed(keybind::PASTE);

        let in_level = self.transient.level.is_some();
        if let (Some((min, max)), false) = (this.working_selection, ctrl || alt || in_level) {
            let middle = ((min + max).map(|x| x as f32) - Vector2::repeat(1.0)) / 2.0;
            let screen = pancam.world_to_screen_space(middle);
            // todo clip to screen?

            let size = max - min + Vector2::repeat(1);
            let price = (min.x..=max.x)
                .flat_map(|x| (min.y..=max.y).map(move |y| Vector2::new(x, y)))
                .map(|pos| self.tiles.get(pos).price())
                .sum::<u32>();
            let text = format!("{}x{} • ${}", size.x, size.y, price.separate_with_commas());

            Text::new(UNDEAD_FONT, &text)
                .position(screen, Anchor::Center)
                .scale(Vector2::repeat(2.0))
                .color(Rgb::hex(0xe27285))
                .z_index(layer::OVERLAY)
                .draw(ctx);
        }

        if let (Some(selection), false) = (
            this.working_selection,
            ctx.input.mouse_down(MouseButton::Left),
        ) {
            this.selection_start = None;
            let new_selection = (selection.0.x..=selection.1.x)
                .flat_map(|x| (selection.0.y..=selection.1.y).map(move |y| Vector2::new(x, y)))
                .filter(|&pos| valid_tile(pos, self.transient.level, self.meta.size))
                .collect();

            // if ctrl down, add to selection
            // if alt down, remove from selection
            if ctrl {
                this.selection.extend(new_selection);
            } else if ctx.input.key_down(keybind::ALT) {
                // remove new_selection from selection
                this.selection = this.selection.difference(&new_selection).copied().collect();
            } else {
                this.selection = new_selection;
            }
        }

        key_events!(ctx, {
            keybind::DESELECT => {
                this.selection_start = None;
                this.selection.clear();
            },
            keybind::DELETE => {
                let mut old = Vec::new();
                for pos in this.selection.iter() {
                    old.push((*pos, self.tiles.get(*pos)));
                    self.tiles.remove(*pos);
                }
                *sim = None;
                self.transient.history.track_many(old);
                this.selection.clear();
            }
        });

        if ctrl && (copy || cut) && !self.transient.holding.contains_dynamic() {
            let mut list = Vec::new();
            let mut old = Vec::new();

            for pos in this.selection.iter() {
                let mut tile = self.tiles.get(*pos);
                if cut {
                    old.push((*pos, tile));
                    self.tiles.remove(*pos);
                } else {
                    tile = tile.generic();
                }

                (!tile.is_empty()).then(|| list.push((*pos, tile)));
            }

            cut.then(|| self.transient.history.track_many(old));

            let origin = pancam
                .screen_to_world_space(ctx.input.mouse())
                .map(|x| x.ceil() as i32);
            list.iter_mut().for_each(|(pos, _)| *pos -= origin);

            this.selection.clear();
            if ctx.input.key_down(SHIFT) {
                let mut map = Map::default();
                list.into_iter().for_each(|(pos, tile)| map.set(pos, tile));

                let bytes = BINCODE_OPTIONS.serialize(&map).unwrap();
                let b64 = BASE64_STANDARD.encode(&bytes);
                state.system_clipboard.set_text(b64).unwrap()
            } else {
                *sim = None;
                self.transient.holding = Holding::Paste(list.clone());
                state.clipboard = Some(list);
            }
        }

        if ctrl && paste {
            if ctx.input.key_down(SHIFT) {
                if let Ok(b64) = state.system_clipboard.get_text()
                    && let Ok(bytes) = BASE64_STANDARD.decode(b64)
                    && let Ok(tiles) = BINCODE_OPTIONS.deserialize::<Map<Tile>>(&bytes)
                {
                    *sim = None;
                    self.transient.holding = Holding::Paste(tiles.iter().collect())
                }
            } else if let Some(item) = &state.clipboard {
                *sim = None;
                self.transient.holding =
                    Holding::Paste(item.iter().map(|(p, x)| (*p, x.generic())).collect());
            }
        }
    }

    pub(super) fn tile_selection(
        &mut self,
        ctx: &mut GraphicsContext,
        pancam: &Pancam,
        pos: Vector2<i32>,
        render_pos: Vector2<f32>,
    ) {
        let this = &mut self.transient.selection;

        // Return quickly if there is not currently a selection.
        if this.working_selection.is_none() && this.selection.is_empty() {
            return;
        }

        let ctrl = ctx.input.key_down(keybind::CTRL);
        let alt = ctx.input.key_down(keybind::ALT);
        let shift = ctx.input.key_down(keybind::SHIFT);

        let in_selection = |pos| {
            if !valid_tile(pos, self.transient.level, self.meta.size) {
                return false;
            }

            let selection = this.selection.contains(&pos);
            let working = this
                .working_selection
                .is_some_and(|bound| in_bounds(pos, bound));

            if ctrl {
                working || selection
            } else if alt {
                selection && !working
            } else if shift && this.working_selection.is_some() {
                working
            } else {
                selection
            }
        };

        // Draw overlay_selection if the tile is in the selection and the direction is not
        if in_selection(pos) {
            for dir in Direction::ALL {
                let offset_point = dir.offset(pos);
                if !in_selection(offset_point) {
                    let px = pancam.scale;
                    let size = match dir {
                        Direction::Up | Direction::Down => Vector2::new(16.0, 1.0),
                        _ => Vector2::new(1.0, 16.0),
                    } * px;

                    let shift = match dir {
                        Direction::Up => Vector2::new(-9.0 * px, 7.0 * px),
                        Direction::Down => Vector2::new(-8.0 * px, -9.0 * px),
                        Direction::Left => Vector2::new(-9.0 * px, -9.0 * px),
                        Direction::Right => Vector2::new(7.0 * px, -8.0 * px),
                    };

                    Rectangle::new(size)
                        .color(color::SELECTION)
                        .position(render_pos + shift, Anchor::BottomLeft)
                        .z_index(layer::TILE_BACKGROUND_OVERLAY)
                        .draw(ctx);
                }
            }
        }
    }
}

fn valid_tile(pos: Vector2<i32>, level: Option<&Level>, size: Option<Vector2<u32>>) -> bool {
    let moveable = level.map(|x| x.permanent.contains(&pos)) != Some(true);
    let in_bounds = size
        .map(|size| in_bounds(pos, (Vector2::repeat(0), size.map(|x| x as i32 - 1))))
        .unwrap_or(true);

    moveable && in_bounds
}
