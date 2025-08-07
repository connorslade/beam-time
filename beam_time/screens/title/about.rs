use engine::{
    drawable::{spacer::Spacer, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
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
    consts::{SELECTION_COLOR, layer},
    screens::title::{ActiveModal, TitleScreen},
    ui::{
        components::{
            button::ButtonExt,
            modal::{Modal, modal_buttons},
        },
        misc::body,
    },
};

pub const DESCRIPTION: &str = indoc! {"
    Beam time is a logic puzzle game where you redirect and split laser beams to create digital circuits. \
    Through the campaign, you will explore logic gates, oscillators, latches, counters, adders, memory, and more.

    Thank you to everyone that pushed me to actually finish this project ♥. \
    Special thanks to Brandon Li (aspiringLich on GitHub) for creating the tile graphics, you do not want to see what the game looked like before.

    This is not an open source project, however the source code for the custom engine, leaderboard server, and the game itself is available on Github at @connorslade/beam-time.

    Assets Used:
      • Alagard, Font by Hewett Tsoi
      • Undead Pixel Light, Font by Not Jam
      • Universal UI/Menu Soundpack, by Cyrex Studios
"};

const PAGE_KEY: MemoryKey = memory_key!();

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Page {
    General = 0,
    Controls = 1,
}

impl TitleScreen {
    pub fn about_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let (margin, padding) = state.spacing(ctx);
        let desired_size = state.modal_size(ctx);

        let current = *ctx.memory.get_or_insert(PAGE_KEY, Page::General);
        let text = [DESCRIPTION, "todo: controls or smth\n"][current as u8 as usize];

        let description = Text::new(UNDEAD_FONT, text)
            .max_width(desired_size.x - margin * 2.0)
            .scale(Vector2::repeat(2.0));
        let height = description.size(ctx).y;

        let modal = Modal::new(Vector2::new(
            desired_size.x,
            height + 100.0 * ctx.scale_factor,
        ))
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
                                            button.color(SELECTION_COLOR).layout(ctx, layout);
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

                description.layout(ctx, layout);

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
