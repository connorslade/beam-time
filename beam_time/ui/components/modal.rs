use std::{
    f32::consts::{FRAC_PI_2, PI},
    mem,
};

use bitflags::bitflags;
use common::direction::Direction;
use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable},
    drawable::{shape::rectangle::Rectangle, spacer::Spacer, sprite::Sprite},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    layout::{
        Direction as LayoutDirection, Layout, LayoutElement, LayoutMethods, bounds::Bounds2D,
        column::ColumnLayout, root::RootLayout, row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};

use crate::{
    assets::HISTOGRAM_MARKER,
    consts::color,
    ui::{components::manual_button::ManualButton, misc::body},
};

pub struct Modal {
    size: Vector2<f32>,
    position: Vector2<f32>,
    anchor: Anchor,

    margin: f32,
    layer: i16,
    sides: ModalSides,
    popup: bool,
    colors: (Rgb<f32>, Rgb<f32>),
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
            colors: (color::MODAL, color::MODAL_BORDER),
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

    pub fn color(self, background: Rgb<f32>, border: Rgb<f32>) -> Self {
        Self {
            colors: (background, border),
            ..self
        }
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

        let bounds = Bounds2D::new(pos - Vector2::new(0.0, self.size.y), pos + self.size);

        for sprite in sprites {
            sprite.z_index += self.layer + 1;
            let [min, max] = &mut sprite.clip;
            min.x = min.x.max(bounds.min.x);
            min.y = min.y.max(bounds.min.y);
            max.x = max.x.min(bounds.max.x);
            max.y = max.y.min(bounds.max.y);
        }

        for vert in shapes {
            vert.z_index = vert.z_index.max(self.layer + 1);
            *vert = vert.clip([bounds.min, bounds.max]);
        }

        if self.popup {
            ctx.defer(move |ctx| ctx.darken(Rgb::repeat(0.5), self.layer));

            ctx.input.cancel_hover();
            ctx.input.cancel_clicks();
        } else if bounds.contains(ctx.input.mouse()) {
            ctx.input.cancel_clicks();
        }
    }
}

pub fn modal_buttons<L: Layout + 'static>(
    ctx: &mut GraphicsContext,
    layout: &mut L,
    width: f32,
    (left, right): (&str, &str),
) -> (bool, bool) {
    let button_space = 10.0;
    let body = body(width);

    let button = |ctx: &mut GraphicsContext, layout: &mut RowLayout, text: &str, rotation| {
        if text.is_empty() {
            return false;
        }

        let key = memory_key!(rotation);
        let tracker = LayoutTracker::new(key);
        let button = ManualButton::new(key).tracker(ctx, tracker);
        let color = Rgb::repeat(1.0).lerp(color::ACCENT, button.hover_time(ctx));

        let direction = [LayoutDirection::MaxToMin, LayoutDirection::MinToMax][rotation as usize];
        layout.nest(
            ctx,
            RowLayout::new(10.0).direction(direction).tracked(tracker),
            |ctx, layout| {
                Sprite::new(HISTOGRAM_MARKER)
                    .scale(Vector2::repeat(2.0))
                    .color(color)
                    .rotate(PI * rotation as u8 as f32 + FRAC_PI_2, Anchor::Center)
                    .layout(ctx, layout);
                body(text).color(color).layout(ctx, layout);
            },
        );

        button.is_hovered()
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
        let (background, border) = self.colors;
        Rectangle::new(self.size)
            .color(background)
            .position(pos, Anchor::TopLeft)
            .z_index(self.layer)
            .draw(ctx);

        let px = 4.0;
        let tb_size = Vector2::new(self.size.x - px * 2.0, px);
        let lr_size = Vector2::new(px, self.size.y - px * 2.0);
        let c_size = Vector2::repeat(px);
        let size = self.size;

        let mut border = |size, pos| {
            Rectangle::new(size)
                .color(border)
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
