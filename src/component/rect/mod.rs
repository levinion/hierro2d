mod api;
mod bind_group;
mod util;
mod vertex;

use crate::vertex::Vertex;

use self::{bind_group::create_bind_group_layout, vertex::RectVertex};

use super::{common, container::Container, Component, Components};
pub(crate) use bind_group::RectUniform;

struct DisplayConfig {
    size: (f32, f32),
    position: (f32, f32),
    color: (f32, f32, f32, f32),
    radius: f32,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            size: (1., 1.),
            position: (0., 0.),
            color: (0., 0., 1., 1.),
            radius: 0.,
        }
    }
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
    depth: i32,
}

impl Component for Rect {
    fn init(
        &mut self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) {
        let indices = [0, 1, 2, 2, 3, 0];
        let index_buffer = common::create_index_buffer(device, &indices);
        let bind_group_layout = create_bind_group_layout(device);
        let render_pipeline = common::create_render_pipeline(
            device,
            config,
            &[&bind_group_layout],
            include_str!("rect.wgsl"),
            &[RectVertex::desc()],
        );
        self.index_buffer = Some(index_buffer);
        self.render_pipeline = Some(render_pipeline);
        self.indices_length = Some(indices.len() as _);
        self.bind_group_layout = Some(bind_group_layout);
    }

    fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        let bind_group =
            self.create_bind_group(device, self.bind_group_layout.as_ref().unwrap(), config);
        self.bind_group = Some(bind_group);

        let vertices = self.create_vertices();
        let vertex_buffer = common::create_vertex_buffer(device, &vertices);
        self.vertex_buffer = Some(vertex_buffer);

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

    fn depth(&self) -> i32 {
        self.depth
    }

    fn set_depth(&mut self, depth: i32) {
        self.depth = depth;
    }

    fn get_size(&self) -> (f32, f32) {
        self.display_config.size
    }

    fn set_size(&mut self, size: (f32, f32)) {
        self.display_config.size = size;
    }

    fn get_position(&self) -> (f32, f32) {
        self.display_config.position
    }

    fn set_position(&mut self, position: (f32, f32)) {
        self.display_config.position = position;
    }
}

impl Container for Rect {}
