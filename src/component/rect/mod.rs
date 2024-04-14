mod bind_group;
mod vertex;

use std::sync::Arc;

use winit::window::Window;

use crate::vertex::Vertex;

use self::{
    bind_group::{create_rect_buffer, RectUniform},
    vertex::RectVertex,
};

use super::{
    common::{self, create_bind_group},
    container::Container,
    Component, Components,
};

struct DisplayConfig {
    size: (f32, f32),
    position: (f32, f32),
    color: (u8, u8, u8, f32),
    radius: f32,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            size: (1., 1.),
            position: (0., 0.),
            color: (0, 0, 255, 1.),
            radius: 0.1,
        }
    }
}

#[derive(Debug)]
struct NormalizedDisplayConfig {
    size: (f32, f32),
    position: (f32, f32),
    color: (f32, f32, f32, f32),
    radius: f32,
}

#[derive(Default)]
pub struct Rect {
    display_config: DisplayConfig,
    render_pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    indices_length: Option<u32>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    bind_group: Option<wgpu::BindGroup>,
    children: Components,
    depth: u8,
}

impl Rect {
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

    pub fn color(mut self, r: u8, g: u8, b: u8, a: f32) -> Self {
        self.display_config.color = (r, g, b, a);
        self
    }

    pub fn depth(mut self, depth: u8) -> Self {
        self.depth = depth;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.display_config.radius = radius;
        self
    }

    pub fn center(mut self) -> Self {
        if let Some(size) = self.size_mut() {
            let size = (*size.0, *size.1);
            if let Some(position) = self.position_mut() {
                (*position.0, *position.1) = ((1. - size.0) / 2., (1. - size.1) / 2.);
            }
        }
        self
    }

    pub fn center_x(mut self) -> Self {
        if let Some(size) = self.size_mut() {
            let size = (*size.0, *size.1);
            if let Some(position) = self.position_mut() {
                *position.0 = (1. - size.0) / 2.;
            }
        }
        self
    }

    pub fn center_y(mut self) -> Self {
        if let Some(size) = self.size_mut() {
            let size = (*size.0, *size.1);
            if let Some(position) = self.position_mut() {
                *position.1 = (1. - size.1) / 2.;
            }
        }
        self
    }
}

impl Component for Rect {
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
        let bind_group_layout = common::create_bind_group_layout(device);
        let render_pipeline = common::create_render_pipeline(
            device,
            config,
            &[&bind_group_layout],
            include_str!("rect.wgsl"),
            &[RectVertex::desc()],
        );
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.render_pipeline = Some(render_pipeline);
        self.indices_length = Some(indices.len() as _);
        self.bind_group_layout = Some(bind_group_layout);
    }

    fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        window: Arc<Window>,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        let bind_group =
            self.create_bind_group(device, self.bind_group_layout.as_ref().unwrap(), window);
        self.bind_group = Some(bind_group);
        render_pass.set_pipeline(self.render_pipeline.as_ref().unwrap());
        render_pass.set_bind_group(0, self.bind_group.as_ref().unwrap(), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(
            self.index_buffer.as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..self.indices_length.unwrap(), 0, 0..1);
    }

    fn children(&mut self) -> Option<&mut Components> {
        Some(&mut self.children)
    }

    fn depth(&self) -> u8 {
        self.depth
    }

    fn position_mut(&mut self) -> Option<(&mut f32, &mut f32)> {
        Some((
            &mut self.display_config.position.0,
            &mut self.display_config.position.1,
        ))
    }

    fn size_mut(&mut self) -> Option<(&mut f32, &mut f32)> {
        Some((
            &mut self.display_config.size.0,
            &mut self.display_config.size.1,
        ))
    }
}

impl Container for Rect {}

impl Rect {
    fn normalize(&self) -> NormalizedDisplayConfig {
        let (width, height) = self.display_config.size;
        let size = (width * 2., height * 2.);
        let (x, y) = self.display_config.position;
        let position = (x * 2. - 1., -y * 2. + 1.);
        let (r, g, b, a) = self.display_config.color;
        let color = (r as f32 / 255., g as f32 / 255., b as f32 / 255., a);
        NormalizedDisplayConfig {
            size,
            position,
            color,
            radius: self.display_config.radius,
        }
    }

    fn create_vertices(&self) -> Vec<RectVertex> {
        let NormalizedDisplayConfig {
            size: (width, height),
            position: (x, y),
            color: (r, g, b, a),
            ..
        } = self.normalize();
        let vertices = &[
            RectVertex {
                position: [x, y - height],
                color: [r, g, b, a],
            }, // A
            RectVertex {
                position: [x + width, y - height],
                color: [r, g, b, a],
            }, // B
            RectVertex {
                position: [x + width, y],
                color: [r, g, b, a],
            }, // C
            RectVertex {
                position: [x, y],
                color: [r, g, b, a],
            }, // D
        ];
        vertices.into()
    }

    fn create_bind_group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        window: Arc<Window>,
    ) -> wgpu::BindGroup {
        let NormalizedDisplayConfig {
            size: (width, height),
            position: (x, y),
            radius,
            ..
        } = self.normalize();
        let size = window.inner_size();
        let resolution = [size.width as f32, size.height as f32];
        let buffer = create_rect_buffer(
            device,
            RectUniform {
                position: [x, y],
                size: [width, height],
                radius,
                _padding: 0.,
                resolution,
            },
        );
        create_bind_group(device, layout, &buffer)
    }
}
