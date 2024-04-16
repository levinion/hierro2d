use std::path::PathBuf;

use anyhow::Result;

use super::Img;

impl Img {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, path: impl Into<PathBuf>) -> Result<Self> {
        let path: PathBuf = path.into();
        self.texture_raw = std::fs::read(path)?;
        Ok(self)
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.display_config.size = (width * 2., height * 2.);
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.display_config.position = (x * 2. - 1., -(y * 2. - 1.));
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
