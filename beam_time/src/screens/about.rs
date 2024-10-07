use engine::{
    drawable::text::Text,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};
use indoc::indoc;

use crate::{
    assets::{ALAGARD_FONT, UNDEAD_FONT},
    consts::FOREGROUND_COLOR,
    ui::{button::ButtonState, misc::titled_screen},
    App,
};

const DESCRIPTION: &str = indoc! {"
    Beam time is a logic puzzle about redirecting and splitting laser beams to create circuits. \
    It's made with a custom GPU accelerated game engine.

    Special thanks to Brandon Li for all the artwork.

    Source code is available online at https://github.com/connorslade/beam-time.

    Assets Used:

    • Alagard, Font by Hewett Tsoi
    • Sock Puppet Supremacy, Song by FADE
    • Undead Pixel Light, Font by Not Jam
    • Universal UI/Menu Soundpack, by Cyrex Studios
"};

#[derive(Default)]
pub struct AboutScreen {
    back_button: ButtonState,
}

impl Screen<App> for AboutScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        let pos = titled_screen(state, ctx, &mut self.back_button, "About");

        let desc = &ctx.assets.get_font(ALAGARD_FONT).desc;
        let height = (desc.height + desc.leading) * 6.0 * ctx.scale_factor;

        ctx.draw(
            Text::new(UNDEAD_FONT, DESCRIPTION)
                .max_width(ctx.size().x - 20.0)
                .pos(Vector2::new(10.0, pos.y - height), Anchor::TopLeft)
                .color(FOREGROUND_COLOR)
                .scale(Vector2::repeat(3.0)),
        );
    }
}
