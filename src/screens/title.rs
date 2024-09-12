use wgpu::CommandEncoder;

use super::Screen;

pub struct TitleScreen {}

impl Screen for TitleScreen {
    fn render(&mut self, encoder: &mut CommandEncoder) {}
}
