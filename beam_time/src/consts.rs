use engine::color::{OkLab, Rgb};

use crate::{
    assets::{BUTTON_LEFT_CAP, BUTTON_MIDDLE, BUTTON_RIGHT_CAP, DEFAULT_FONT},
    ui::button::ButtonStyle,
};

pub const DEFAULT_SIZE: (u32, u32) = (800, 600);
pub const BACKGROUND_COLOR: Rgb<f32> = Rgb::new(0.294, 0.184, 0.224);
pub const FOREGROUND_COLOR: Rgb<f32> = Rgb::new(0.859, 0.89, 0.839);
pub const START_COLOR: OkLab<f32> = OkLab::new(0.773, 0.1131, 0.0);

pub const BUTTON_STYLE: ButtonStyle = ButtonStyle {
    left_cap: BUTTON_LEFT_CAP,
    right_cap: BUTTON_RIGHT_CAP,
    segment: BUTTON_MIDDLE,
    font: DEFAULT_FONT,

    default_border_color: Rgb::new(1.0, 1.0, 1.0),
    default_text_color: FOREGROUND_COLOR,
};
