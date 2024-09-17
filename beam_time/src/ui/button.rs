use engine::{
    assets::AssetRef,
    color::Rgb,
    drawable::{sprites::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
};

pub struct Button<'a, 'b> {
    text: &'a str,
    style: &'b ButtonStyle,

    pos: Vector2<f32>,
    scale: Vector2<f32>,

    text_color: Rgb<f32>,
    border_color: Rgb<f32>,
}

pub struct ButtonStyle {
    pub left_cap: AssetRef,
    pub right_cap: AssetRef,
    pub segment: AssetRef,
    pub font: AssetRef,

    pub default_text_color: Rgb<f32>,
    pub default_border_color: Rgb<f32>,
}

impl<'a, 'b> Button<'a, 'b> {
    pub fn new(style: &'b ButtonStyle, text: &'a str) -> Self {
        Self {
            text,
            style,
            pos: Vector2::zeros(),
            scale: Vector2::repeat(1.0),
            border_color: style.default_border_color,
            text_color: style.default_text_color,
        }
    }

    pub fn pos(mut self, pos: Vector2<f32>) -> Self {
        self.pos = pos;
        self
    }

    pub fn scale(mut self, scale: Vector2<f32>) -> Self {
        self.scale = scale;
        self
    }
}

impl<'a, 'b> Drawable for Button<'a, 'b> {
    fn draw(self, ctx: &mut GraphicsContext) {
        let font = ctx.get_asset(self.style.font).as_font().unwrap();

        let text_pos = self.pos - Vector2::new(0.0, font.desc.height / 2.0 * self.scale.y);
        let text = Text::new(self.style.font, self.text)
            .pos(text_pos, Anchor::Center)
            .scale(self.scale)
            .color(self.text_color);

        let middle_sprite = ctx.get_asset(self.style.segment).as_sprite().unwrap();
        let middle_width = middle_sprite.size.x as f32 * self.scale.x;
        let middle_count = (text.width(ctx) / middle_width).ceil();
        let text_width = middle_count * middle_width;

        ctx.draw(text);

        let left_pos = self.pos - Vector2::new(text_width / 2.0, 0.0);
        ctx.draw(
            Sprite::new(self.style.left_cap)
                .scale(self.scale)
                .pos(left_pos, Anchor::CenterRight)
                .color(self.border_color),
        );

        let right_pos = self.pos + Vector2::new(text_width / 2.0, 0.0);
        ctx.draw(
            Sprite::new(self.style.right_cap)
                .scale(self.scale)
                .pos(right_pos, Anchor::CenterLeft)
                .color(self.border_color),
        );

        for i in 0..(middle_count as i32) {
            let x = i as f32 * 16.0 * self.scale.x - text_width / 2.0;
            let pos = self.pos + Vector2::new(x, 0.0);
            ctx.draw(
                Sprite::new(self.style.segment)
                    .scale(self.scale)
                    .pos(pos, Anchor::CenterLeft)
                    .color(self.border_color),
            );
        }
    }
}
