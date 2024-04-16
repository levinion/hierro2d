use crate::context::Context;

use super::Text;

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = (width * 2., height * 2.);
        self
    }

    pub fn color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.config.color = glyphon::Color::rgba(
            (r * 255.) as u8,
            (g * 255.) as u8,
            (b * 255.) as u8,
            (a * 255.) as u8,
        );
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.config.content = content.into();
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.config.left = x * 2. - 1.;
        self.config.top = -(y * 2. - 1.);
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

    pub fn on_click(mut self, f: fn(Context)) -> Self {
        self.on_click = Some(f);
        self
    }
}
