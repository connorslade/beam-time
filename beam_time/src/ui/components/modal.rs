use std::f32::consts::PI;

use engine::{
    drawable::{spacer::Spacer, sprite::Sprite},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{
        column::ColumnLayout, root::RootLayout, row::RowLayout, tracker::LayoutTracker, Direction,
        Layout, LayoutElement, LayoutMethods,
    },
    memory_key,
};

use crate::{
    assets::{LEVEL_DROPDOWN_ARROW, PANEL},
    ui::misc::body,
};

pub struct Modal {
    size: Vector2<f32>,
    margin: f32,
    layer: i16,
}

impl Modal {
    pub fn new(size: Vector2<f32>) -> Self {
        Self {
            size,
            layer: 0,
            margin: 0.0,
        }
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    pub fn layer(mut self, layer: i16) -> Self {
        self.layer = layer;
        self
    }

    pub fn inner_size(&self) -> Vector2<f32> {
        Vector2::new(
            self.size.x - 2.0 * self.margin,
            self.size.y - 2.0 * self.margin,
        )
    }

    pub fn origin(&self, ctx: &mut GraphicsContext) -> Vector2<f32> {
        ctx.center() + Vector2::new(-self.size.x, self.size.y) / 2.0
    }

    pub fn draw(
        self,
        ctx: &mut GraphicsContext,
        ui: impl FnOnce(&mut GraphicsContext, &mut RootLayout),
    ) {
        let pos = ctx.center() + Vector2::new(-self.size.x, self.size.y) / 2.0;
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

        ctx.input.cancel_hover();
        ctx.input.cancel_clicks();
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
        let scale = 4.0;
        let tile_size = 16.0 * scale * ctx.scale_factor;

        let y_scale = scale * (self.size.y / tile_size - 2.0);
        let x_scale = scale * (self.size.x / tile_size - 2.0);

        let base = Sprite::new(PANEL)
            .z_index(self.layer)
            .scale(Vector2::repeat(scale));

        // god this is awful...
        ctx.draw([
            // Top
            base.clone()
                .scale(Vector2::repeat(scale))
                .position(pos, Anchor::TopLeft)
                .uv_offset(Vector2::new(-16, -16)),
            base.clone()
                .scale(Vector2::new(x_scale, scale))
                .position(pos + Vector2::x() * tile_size, Anchor::TopLeft)
                .uv_offset(Vector2::new(0, -16)),
            base.clone()
                .scale(Vector2::repeat(scale))
                .position(pos + Vector2::x() * self.size.x, Anchor::TopRight)
                .uv_offset(Vector2::new(16, -16)),
            // Sides
            base.clone()
                .scale(Vector2::new(scale, y_scale))
                .position(pos - Vector2::y() * tile_size, Anchor::TopLeft)
                .uv_offset(Vector2::new(-16, 0)),
            base.clone()
                .scale(Vector2::new(scale, y_scale))
                .position(
                    pos + Vector2::new(self.size.x, -tile_size),
                    Anchor::TopRight,
                )
                .uv_offset(Vector2::new(16, 0)),
            // Bottom
            base.clone()
                .scale(Vector2::repeat(scale))
                .position(pos - Vector2::y() * self.size.y, Anchor::BottomLeft)
                .uv_offset(Vector2::new(-16, 16)),
            base.clone()
                .scale(Vector2::new(x_scale, scale))
                .position(
                    pos + Vector2::new(tile_size, -self.size.y),
                    Anchor::BottomLeft,
                )
                .uv_offset(Vector2::new(0, 16)),
            base.clone()
                .scale(Vector2::repeat(scale))
                .position(
                    pos + Vector2::new(self.size.x, -self.size.y),
                    Anchor::BottomRight,
                )
                .uv_offset(Vector2::new(16, 16)),
            // Middle
            base.scale(Vector2::new(x_scale, y_scale))
                .position(pos + Vector2::new(tile_size, -tile_size), Anchor::TopLeft),
        ]);
    }
}
