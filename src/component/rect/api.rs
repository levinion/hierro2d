use super::Rect;

impl Rect {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.display_config.size = (width * 2., height * 2.);
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.display_config.position = (x * 2. - 1., -(y * 2. - 1.));
        self
    }

    pub fn color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.display_config.color = (r, g, b, a);
        self
    }

    pub fn depth(mut self, depth: i32) -> Self {
        self.depth = depth;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.display_config.radius = radius;
        self
    }
}
