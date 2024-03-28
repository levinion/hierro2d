use crate::vertex::Vertex;

use super::{
    text::{TextConfig, TextRenderer},
    Component, ComponentBuilder, IntoComponent,
};

struct DisplayConfig {
    size: (f32, f32),
    position: (f32, f32),
    color: (u8, u8, u8),
}

struct NormalizedDisplayConfig {
    size: (f32, f32),
    position: (f32, f32),
    color: (f32, f32, f32),
}

pub struct Square {
    display_config: DisplayConfig,
    text_config: Option<TextConfig>,
}

impl Square {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.display_config.size = (width, height);
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.display_config.position = (x, y);
        self
    }

    pub fn color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.display_config.color = (r, g, b);
        self
    }

    pub fn with_text(mut self, f: impl Fn(&mut TextConfig)) -> Self {
        let (x, y) = self.display_config.position;
        let mut text_config = TextConfig::new(x, y);
        f(&mut text_config);
        self.text_config = Some(text_config);
        self
    }
}

impl Default for Square {
    fn default() -> Self {
        Self {
            display_config: DisplayConfig {
                size: (0., 0.),
                position: (0., 0.),
                color: (0, 0, 255),
            },
            text_config: None,
        }
    }
}

impl IntoComponent for Square {
    fn into_component(
        self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Component {
        let vertices = self.create_vertices();
        let indices = [0, 1, 2, 2, 3, 0];
        let mut builder = ComponentBuilder::new(vertices, indices);
        if let Some(text_config) = self.text_config {
            let text_renderer = TextRenderer::new(device, queue, config, &text_config);
            builder.add_text_object(text_renderer, text_config);
        }
        builder.build(device, config)
    }
}

impl Square {
    fn normalize(&self) -> NormalizedDisplayConfig {
        let (width, height) = self.display_config.size;
        let size = (width * 2., height * 2.);
        let (x, y) = self.display_config.position;
        let position = (x * 2. - 1., -y * 2. + 1.);
        let (r, g, b) = self.display_config.color;
        let color = (r as f32 / 255., g as f32 / 255., b as f32 / 255.);
        NormalizedDisplayConfig {
            size,
            position,
            color,
        }
    }

    fn create_vertices(&self) -> Vec<Vertex> {
        let NormalizedDisplayConfig {
            size: (width, height),
            position: (x, y),
            color: (r, g, b),
        } = self.normalize();
        let vertices = &[
            Vertex {
                position: [x, y - height, 0.],
                color: [r, g, b],
            }, // A
            Vertex {
                position: [x + width, y - height, 0.],
                color: [r, g, b],
            }, // B
            Vertex {
                position: [x + width, y, 0.],
                color: [r, g, b],
            }, // C
            Vertex {
                position: [x, y, 0.],
                color: [r, g, b],
            }, // D
        ];
        vertices.try_into().unwrap()
    }
}
