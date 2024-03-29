use crate::vertex::Vertex;

use super::{
    common,
    text::{TextConfig, TextObject},
    Component, Components,
};

struct DisplayConfig {
    size: (f32, f32),
    position: (f32, f32),
    color: (u8, u8, u8),
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            size: (0., 0.),
            position: (0., 0.),
            color: (0, 0, 255),
        }
    }
}

struct NormalizedDisplayConfig {
    size: (f32, f32),
    position: (f32, f32),
    color: (f32, f32, f32),
}

#[derive(Default)]
pub struct Square {
    display_config: DisplayConfig,
    text_config: Option<TextConfig>,
    render_pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    indices_length: Option<u32>,
    text_objects: Components,
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

impl Component for Square {
    fn init(
        &mut self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) {
        let vertices = self.create_vertices();
        let vertex_buffer = common::create_vertex_buffer(device, &vertices);
        let indices = [0, 1, 2, 2, 3, 0];
        let index_buffer = common::create_index_buffer(device, &indices);
        let render_pipeline = common::create_render_pipeline(
            device,
            config,
            &[],
            include_str!("../shader.wgsl"),
            &[Vertex::desc()],
        );
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.render_pipeline = Some(render_pipeline);
        self.indices_length = Some(indices.len() as _);

        if let Some(text_config) = self.text_config.clone() {
            let text_object = TextObject::new(text_config);
            self.text_objects.push(Box::new(text_object));
        }
    }

    fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(self.render_pipeline.as_ref().unwrap());
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(
            self.index_buffer.as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..self.indices_length.unwrap(), 0, 0..1);
    }

    fn sub_components(&mut self) -> Components {
        std::mem::take(&mut self.text_objects)
    }

    fn depth(&self) -> u8 {
        1
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
