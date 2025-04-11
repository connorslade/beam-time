use std::f32::consts::PI;

use bitflags::bitflags;
use engine::{
    color::Rgb,
    drawable::{shape::rectangle::Rectangle, spacer::Spacer, sprite::Sprite},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{
        column::ColumnLayout, root::RootLayout, row::RowLayout, tracker::LayoutTracker, Direction,
        Layout, LayoutElement, LayoutMethods,
    },
    memory_key,
};

use crate::{
    assets::LEVEL_DROPDOWN_ARROW,
    consts::{MODAL_BORDER_COLOR, MODAL_COLOR},
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

        for sprite in sprites {
            sprite.z_index = sprite.z_index.max(self.layer + 1);
            sprite.clip = [
                pos - Vector2::new(0.0, self.size.y),
                pos + self.size - shift,
            ];
        }

        for vert in shapes {
            vert.z_index = vert.z_index.max(self.layer + 1);
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

    let button = |ctx: &mut GraphicsContext, layout: &mut RowLayout, text, rotation| {
        let key = memory_key!(rotation);
        let tracker = LayoutTracker::new(key);
        let hover = tracker.hovered(ctx);

        let t = ctx.memory.get_or_insert(key, 0.0);
        *t += ctx.delta_time * if hover { 1.0 } else { -1.0 };
        *t = t.clamp(0.0, 0.1);
        let scale = Vector2::repeat(2.0 + *t);

        let direction = if rotation {
            Direction::MinToMax
        } else {
            Direction::MaxToMin
        };

        layout.nest(
            ctx,
            RowLayout::new(button_space)
                .direction(direction)
                .tracked(tracker),
            |ctx, layout| {
                let anchors = [Anchor::CenterLeft, Anchor::CenterRight];
                Sprite::new(LEVEL_DROPDOWN_ARROW)
                    .scale(Vector2::repeat(2.0))
                    .dynamic_scale(scale, anchors[rotation as usize])
                    .rotate(PI * rotation as u8 as f32, Anchor::Center)
                    .layout(ctx, layout);
                body(text)
                    .dynamic_scale(scale, anchors[(1 + rotation as usize) % 2])
                    .layout(ctx, layout);
            },
        );

        hover
    };

    let mut hovered = (false, false);
    layout.nest(
        ctx,
        ColumnLayout::new(0.0).direction(Direction::MaxToMin),
        |ctx, layout| {
            layout.nest(ctx, RowLayout::new(button_space), |ctx, layout| {
                hovered.0 = button(ctx, layout, left, true);

                layout.nest(
                    ctx,
                    RowLayout::new(button_space).direction(Direction::MaxToMin),
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

        let (t, b, l, r) = (
            ModalSides::TOP,
            ModalSides::BOTTOM,
            ModalSides::LEFT,
            ModalSides::RIGHT,
        );

        let size = self.size;
        for (_parts, size, offset) in [
            (t, tb_size, Vector2::new(px, px)),
            (b, tb_size, Vector2::new(px, -size.y)),
            (l, lr_size, Vector2::new(-px, -px)),
            (r, lr_size, Vector2::new(size.x, -px)),
            (t | l, c_size, Vector2::new(0.0, 0.0)),
            (t | r, c_size, Vector2::new(size.x - px, 0.0)),
            (b | l, c_size, Vector2::new(0.0, px - size.y)),
            (b | r, c_size, Vector2::new(size.x - px, px - size.y)),
        ]
        .into_iter()
        .filter(|(parts, _, _)| self.sides.contains(*parts))
        {
            Rectangle::new(size)
                .color(MODAL_BORDER_COLOR)
                .position(pos + offset, Anchor::TopLeft)
                .z_index(self.layer)
                .draw(ctx);
        }
    }
}
