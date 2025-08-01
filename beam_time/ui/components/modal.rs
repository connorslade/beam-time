use std::{f32::consts::PI, mem};

use bitflags::bitflags;
use common::direction::Direction;
use engine::{
    color::Rgb,
    drawable::{shape::rectangle::Rectangle, spacer::Spacer, sprite::Sprite},
    exports::{nalgebra::Vector2, winit::window::CursorIcon},
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{
        Direction as LayoutDirection, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        root::RootLayout, row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};

use crate::{
    assets::LEVEL_DROPDOWN_ARROW,
    consts::{ACCENT_COLOR, MODAL_BORDER_COLOR, MODAL_COLOR},
    ui::misc::body,
};

pub struct Modal {
    size: Vector2<f32>,
    position: Vector2<f32>,
    anchor: Anchor,

    margin: f32,
    layer: i16,
    sides: ModalSides,
    popup: bool,
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct ModalSides: u8 {
        const TOP = 1 << 0;
        const RIGHT = 1 << 1;
        const BOTTOM = 1 << 2;
        const LEFT = 1 << 3;
    }
}

impl Modal {
    pub fn new(size: Vector2<f32>) -> Self {
        Self {
            size,
            position: Vector2::zeros(),
            anchor: Anchor::BottomLeft,
            layer: 0,
            margin: 0.0,
            sides: ModalSides::all(),
            popup: true,
        }
    }

    pub fn sides(self, sides: ModalSides) -> Self {
        Self { sides, ..self }
    }

    pub fn margin(self, margin: f32) -> Self {
        Self { margin, ..self }
    }

    pub fn layer(self, layer: i16) -> Self {
        Self { layer, ..self }
    }

    pub fn position(self, position: Vector2<f32>, anchor: Anchor) -> Self {
        Self {
            position,
            anchor,
            ..self
        }
    }

    pub fn popup(self, popup: bool) -> Self {
        Self { popup, ..self }
    }

    pub fn inner_size(&self) -> Vector2<f32> {
        Vector2::new(
            self.size.x - 2.0 * self.margin,
            self.size.y - 2.0 * self.margin,
        )
    }

    pub fn origin(&self) -> Vector2<f32> {
        self.position + self.anchor.offset(self.size) + Vector2::y() * self.size.y
    }

    pub fn draw_empty(self, ctx: &mut GraphicsContext) {
        self.background(ctx, self.origin());
    }

    pub fn draw(
        self,
        ctx: &mut GraphicsContext,
        ui: impl FnOnce(&mut GraphicsContext, &mut RootLayout),
    ) {
        let pos = self.origin();
        let shift = Vector2::new(self.margin, -self.margin);

        self.background(ctx, pos);
        let (sprites, shapes) = ctx.draw_callback(|ctx| {
            let mut root = RootLayout::new(pos + shift, Anchor::TopLeft).sized(self.inner_size());
            (ui)(ctx, &mut root);
            root.draw(ctx);
        });

        let clip_bounds = [pos - Vector2::new(0.0, self.size.y), pos + self.size];

        for sprite in sprites {
            sprite.z_index += self.layer + 1;
            let [min, max] = &mut sprite.clip;
            min.x = min.x.max(clip_bounds[0].x);
            min.y = min.y.max(clip_bounds[0].y);
            max.x = max.x.min(clip_bounds[1].x);
            max.y = max.y.min(clip_bounds[1].y);
        }

        for vert in shapes {
            vert.z_index = vert.z_index.max(self.layer + 1);
            *vert = vert.clip(clip_bounds);
        }

        if self.popup {
            ctx.defer(move |ctx| ctx.darken(Rgb::repeat(0.5), self.layer));

            ctx.input.cancel_hover();
            ctx.input.cancel_clicks();
        }
    }
}

pub fn modal_buttons(
    ctx: &mut GraphicsContext,
    layout: &mut ColumnLayout,
    width: f32,
    (left, right): (&str, &str),
) -> (bool, bool) {
    let button_space = ctx.scale_factor * 10.0;
    let body = body(width);

    let button = |ctx: &mut GraphicsContext, layout: &mut RowLayout, text: &str, rotation| {
        if text.is_empty() {
            return false;
        }

        let key = memory_key!(rotation);
        let tracker = LayoutTracker::new(key);
        let hover = tracker.hovered(ctx);
        hover.then(|| ctx.set_cursor(CursorIcon::Pointer));

        let t = ctx.memory.get_or_insert(key, 0.0);
        *t += ctx.delta_time * if hover { 1.0 } else { -1.0 };
        *t = t.clamp(0.0, 0.1);
        let color = Rgb::hex(0xFFFFFF).lerp(ACCENT_COLOR, *t / 0.1);

        let direction = [LayoutDirection::MaxToMin, LayoutDirection::MinToMax][rotation as usize];
        layout.nest(
            ctx,
            RowLayout::new(button_space)
                .direction(direction)
                .tracked(tracker),
            |ctx, layout| {
                Sprite::new(LEVEL_DROPDOWN_ARROW)
                    .scale(Vector2::repeat(2.0))
                    .color(color)
                    .rotate(PI * rotation as u8 as f32, Anchor::Center)
                    .layout(ctx, layout);
                body(text).color(color).layout(ctx, layout);
            },
        );

        hover
    };

    let mut hovered = (false, false);
    layout.nest(
        ctx,
        ColumnLayout::new(0.0).direction(LayoutDirection::MaxToMin),
        |ctx, layout| {
            layout.nest(ctx, RowLayout::new(button_space), |ctx, layout| {
                hovered.0 = button(ctx, layout, left, true);

                layout.nest(
                    ctx,
                    RowLayout::new(button_space).direction(LayoutDirection::MaxToMin),
                    |ctx, layout| {
                        hovered.1 = button(ctx, layout, right, false);
                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                    },
                );
            });

            Spacer::new_y(layout.available().y).layout(ctx, layout);
        },
    );

    hovered
}

impl Modal {
    // it's like whatever
    fn background(&self, ctx: &mut GraphicsContext, pos: Vector2<f32>) {
        Rectangle::new(self.size)
            .color(MODAL_COLOR)
            .position(pos, Anchor::TopLeft)
            .z_index(self.layer)
            .draw(ctx);

        let px = 4.0 * ctx.scale_factor;
        let tb_size = Vector2::new(self.size.x - px * 2.0, px);
        let lr_size = Vector2::new(px, self.size.y - px * 2.0);
        let c_size = Vector2::repeat(px);
        let size = self.size;

        let mut border = |size, pos| {
            Rectangle::new(size)
                .color(MODAL_BORDER_COLOR)
                .position(pos, Anchor::TopLeft)
                .z_index(self.layer)
                .draw(ctx)
        };

        for (dir, mut size, mut offset) in [
            (Direction::Up, tb_size, Vector2::new(px, px)),
            (Direction::Down, tb_size, Vector2::new(px, -size.y)),
            (Direction::Left, lr_size, Vector2::new(-px, -px)),
            (Direction::Right, lr_size, Vector2::new(size.x, -px)),
        ]
        .into_iter()
        .filter(|(parts, _, _)| self.sides.contains((*parts).into()))
        {
            let mut left = !self.sides.contains(dir.rotate_reverse().into());
            let mut right = !self.sides.contains(dir.rotate().into());

            if matches!(dir, Direction::Left | Direction::Down) {
                mem::swap(&mut left, &mut right);
            }

            if left {
                let idx = dir.is_horizontal() as usize;
                size[idx] += px;
                offset[idx] += px * if dir.is_vertical() { -1.0 } else { 1.0 };
            }

            if right {
                let idx = dir.is_horizontal() as usize;
                size[idx] += px;
            }

            border(size, pos + offset);
        }

        let (t, b, l, r) = (
            ModalSides::TOP,
            ModalSides::BOTTOM,
            ModalSides::LEFT,
            ModalSides::RIGHT,
        );
        for (parts, size, offset) in [
            (t | l, c_size, Vector2::zeros()),
            (t | r, c_size, Vector2::x() * (size.x - px)),
            (b | l, c_size, Vector2::y() * (px - size.y)),
            (b | r, c_size, Vector2::new(size.x - px, px - size.y)),
        ] {
            if self.sides.contains(parts) {
                border(size, pos + offset);
            }
        }
    }
}

impl From<Direction> for ModalSides {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => ModalSides::TOP,
            Direction::Right => ModalSides::RIGHT,
            Direction::Down => ModalSides::BOTTOM,
            Direction::Left => ModalSides::LEFT,
        }
    }
}
