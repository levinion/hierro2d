use crate::component::Component;

use super::Text;

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, r: u8, g: u8, b: u8, a: f32) -> Self {
        self.config.color = glyphon::Color::rgba(r, g, b, (a * 255.) as u8);
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.config.content = content.into();
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.config.left = x;
        self.config.top = y;
        self
    }

    pub fn bound(mut self, x: i32, y: i32) -> Self {
        self.config.text_bounds.bottom = y;
        self.config.text_bounds.right = x;
        self
    }

    pub fn depth(mut self, depth: i32) -> Self {
        self.depth = depth;
        self
    }

    pub fn center(mut self) -> Self {
        let size = self.get_size();
        self.set_position(((1. - size.0) / 2., (1. - size.1) / 2.));
        self
    }

    pub fn center_x(mut self) -> Self {
        let size = self.get_size();
        let position = self.get_position();
        self.set_position(((1. - size.0) / 2., position.1));
        self
    }

    pub fn center_y(mut self) -> Self {
        let size = self.get_size();
        let position = self.get_position();
        self.set_position((position.0, (1. - size.1) / 2.));
        self
    }
}
