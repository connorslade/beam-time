use engine::{
    drawable::{Anchor, spacer::Spacer, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        row::RowLayout,
    },
    memory::MemoryKey,
    memory_key,
};
use indoc::indoc;

use crate::{
    App,
    assets::UNDEAD_FONT,
    consts::{color, layer},
    screens::title::{ActiveModal, TitleScreen},
    ui::{
        components::{
            button::ButtonExt,
            horizontal_rule::Rule,
            modal::{Modal, modal_buttons},
        },
        misc::{body, modal_size, spacing},
    },
};

pub const GENERAL_DESCRIPTION: &str = indoc! {"
    Beam time is a logic puzzle game where you redirect and split laser beams to create digital circuits. \
    Through the campaign, you will explore logic gates, oscillators, latches, counters, adders, memory, and more.

    Thank you to everyone that pushed me to actually finish this project ♥. \
    Special thanks to Brandon Li (aspiringLich on GitHub) for creating the tile graphics, you do not want to see what the game looked like before.

    The source code for this game, the custom engine, and leaderboard server is available on Github at @connorslade/beam-time, although it is not open source and unauthorized distribution is not allowed.

    Assets Used:
      • Alagard, Font by Hewett Tsoi
      • Undead Pixel Light, Font by Not Jam
      • Universal UI/Menu Soundpack, by Cyrex Studios
"};

pub const CONTROLS_DESCRIPTION: &[&str] = &[
    indoc! {"
        In addition to the keybinds on the pause screen, here are some useful, but non-essential ones that can significantly speed up the construction of large circuits.
    "},
    indoc! {"
        • CTRL+Z - Undo
        • 1-7 - Picks up the corisponding tile from panel
        • N - Creates a stickey note at the mouse position
        • +/-/0 - Increses/decreses/rests TPS
        • SHIFT+0 - ∞ TPS
        • SHIFT+R - Rotates counterclockwise
    "},
    indoc! {"
        Selections are a powerful way to modify multiple tiles at once. \
        Create one by holding SHIFT and dragging to select a rectangular area of tiles. \
        If you've already made a selection, dragging with CTRL+SHIFT will add to it and ALT+SHIFT will subtract from it.

        You can either deselect (U), delete (BACKSPACE), cut (CTRL+X), or copy (CTRL+C) the selection. \
        If you cut/copy it, you will pick up the selection like you would a tile, and just like with a tile you can rotate it (R/SHIFT+R). \
        You can also flip it, either horizontally (H) or vertically (V).
    "},
];

const PAGE_KEY: MemoryKey = memory_key!();

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
enum Page {
    General,
    Controls,
}

fn about_general(ctx: &mut GraphicsContext, width: f32) -> Box<dyn LayoutElement> {
    let (margin, _) = spacing(ctx);
    let text = Text::new(UNDEAD_FONT, GENERAL_DESCRIPTION.trim_end())
        .max_width(width - margin * 2.0)
        .scale(Vector2::repeat(2.0));
    Box::new(text)
}

fn about_controls(ctx: &mut GraphicsContext, width: f32) -> Box<dyn LayoutElement> {
    let body = body(width);

    let mut layout = ColumnLayout::new(12.0 * ctx.scale_factor);
    for (i, desc) in CONTROLS_DESCRIPTION.iter().enumerate() {
        body(desc.trim_end()).layout(ctx, &mut layout);
        if i + 1 != CONTROLS_DESCRIPTION.len() {
            Rule::horizontal(width).layout(ctx, &mut layout);
        }
    }

    Box::new(layout)
}

impl TitleScreen {
    pub fn about_modal(&mut self, _state: &mut App, ctx: &mut GraphicsContext) {
        let (margin, padding) = spacing(ctx);
        let width = modal_size(ctx).x;
        let inner_width = width - 2.0 * margin;

        let current = *ctx.memory.get_or_insert(PAGE_KEY, Page::General);
        let element = match current {
            Page::General => about_general(ctx, inner_width),
            Page::Controls => about_controls(ctx, inner_width),
        };

        let height = element.bounds(ctx).height();
        let modal = Modal::new(Vector2::new(width, height + 120.0 * ctx.scale_factor))
            .position(ctx.center(), Anchor::Center)
            .margin(margin)
            .layer(layer::OVERLAY);

        let size = modal.inner_size();
        modal.draw(ctx, |ctx, root| {
            let body = body(size.x);

            root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                layout.nest(
                    ctx,
                    RowLayout::new(0.0).justify(Justify::Center),
                    |ctx, layout| {
                        body("About")
                            .scale(Vector2::repeat(4.0))
                            .layout(ctx, layout);
                        layout.nest(
                            ctx,
                            RowLayout::new(0.0).direction(Direction::MaxToMin),
                            |ctx, layout| {
                                layout.nest(ctx, RowLayout::new(padding), |ctx, layout| {
                                    for (idx, page) in Page::ALL.into_iter().enumerate() {
                                        let button = body(page.name());

                                        if page == current {
                                            button.color(color::SELECTION).layout(ctx, layout);
                                        } else {
                                            let button = button.button(memory_key!(page));
                                            if button.is_clicked(ctx) {
                                                let active = ctx.memory.get_mut(PAGE_KEY).unwrap();
                                                *active = page;
                                            }
                                            button.layout(ctx, layout);
                                        }

                                        (idx + 1 != Page::ALL.len())
                                            .then(|| body("•").layout(ctx, layout));
                                    }
                                });
                                Spacer::new_x(layout.available().x).layout(ctx, layout);
                            },
                        );
                    },
                );
                Spacer::new_y(4.0 * ctx.scale_factor).layout(ctx, layout);

                layout.layout(ctx, element);

                let clicking = ctx.input.mouse_down(MouseButton::Left);
                let (back, _) = modal_buttons(ctx, layout, size.x, ("Back", ""));
                (back && clicking).then(|| self.modal = ActiveModal::None);
            });
        });
    }
}

impl Page {
    const ALL: [Page; 2] = [Page::General, Page::Controls];

    pub fn name(&self) -> &'static str {
        match self {
            Page::General => "General",
            Page::Controls => "Controls",
        }
    }
}
