use engine::{
    drawable::text::Text,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    memory_key,
};
use indoc::indoc;

use crate::{
    App,
    assets::{ALAGARD_FONT, UNDEAD_FONT},
    ui::misc::titled_screen,
};

use super::Screen;

const DESCRIPTION: &str = indoc! {"
    Beam time is a logic puzzle about redirecting and splitting laser beams to create circuits. \
    It's made with a custom GPU accelerated game engine, just because why not.

    Special thanks to Brandon Li for creating the tile graphics. (aspiringLich on GitHub)

    Source code is available online on my Github at https://github.com/connorslade/beam-time.

    Assets Used:

    • Alagard, Font by Hewett Tsoi
    • Undead Pixel Light, Font by Not Jam
    • Universal UI/Menu Soundpack, by Cyrex Studios
"};

#[derive(Default)]
pub struct AboutScreen {}

impl Screen for AboutScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let pos = titled_screen(state, ctx, Some(memory_key!()), "About");

        let desc = &ctx.assets.get_font(ALAGARD_FONT).desc;
        let height = (desc.height + desc.leading) * 6.0 * ctx.scale_factor;

        let width = (ctx.size().x - 20.0).min(800.0 * ctx.scale_factor);
        let pos = Vector2::new(ctx.center().x, pos.y - height - 20.0);

        Text::new(UNDEAD_FONT, DESCRIPTION)
            .max_width(width)
            .position(pos, Anchor::TopCenter)
            .scale(Vector2::repeat(3.0))
            .draw(ctx);
    }
}
