use glyphon::TextBounds;

#[derive(Clone, Debug)]
pub struct TextConfig {
    pub(crate) left: f32,
    pub(crate) top: f32,
    pub(crate) scale: f32,
    pub(crate) color: glyphon::Color,
    pub(crate) text_bounds: TextBounds,
    pub(crate) content: String,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            left: -1.,
            top: 1.,
            scale: 1.,
            color: glyphon::Color::rgba(255, 255, 255, 255),
            text_bounds: TextBounds::default(),
            content: String::new(),
        }
    }
}

impl TextConfig {
    pub(crate) fn fit_screen(&self, config: &wgpu::SurfaceConfiguration) -> Self {
        let mut new_instance = self.clone();
        new_instance.left = (self.left + 1.) / 2. * config.width as f32;
        new_instance.top = (-self.top + 1.) / 2. * config.height as f32;
        new_instance
    }
}
