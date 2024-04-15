mod api;
mod bind_group;
mod texture;
mod util;
mod vertex;
use crate::vertex::Vertex;

use self::{
    bind_group::{create_rect_bind_group_layout, create_texture_bind_group_layout},
    texture::Texture,
    vertex::ImgVertex,
};

use super::{common, Component, Components};

#[derive(Default)]
pub struct Img {
    display_config: DisplayConfig,
    render_pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    indices_length: Option<u32>,
    texture_bind_group_layout: Option<wgpu::BindGroupLayout>,
    texture_bind_group: Option<wgpu::BindGroup>,
    rect_bind_group_layout: Option<wgpu::BindGroupLayout>,
    rect_bind_group: Option<wgpu::BindGroup>,
    children: Components,
    depth: i32,
    texture_raw: Vec<u8>,
    texture: Option<Texture>,
}

struct DisplayConfig {
    size: (f32, f32),
    position: (f32, f32),
    radius: f32,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            size: (1., 1.),
            position: (0., 0.),
            radius: 0.,
        }
    }
}

impl Component for Img {
    fn init(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) {
        let texture = Texture::from_bytes(device, queue, &self.texture_raw).unwrap();
        let indices = [0, 1, 2, 2, 3, 0];
        let index_buffer = common::create_index_buffer(device, &indices);
        let texture_bind_group_layout = create_texture_bind_group_layout(device);
        let rect_bind_group_layout = create_rect_bind_group_layout(device);
        let render_pipeline = common::create_render_pipeline(
            device,
            config,
            &[&texture_bind_group_layout, &rect_bind_group_layout],
            include_str!("img.wgsl"),
            &[ImgVertex::desc()],
        );
        self.index_buffer = Some(index_buffer);
        self.render_pipeline = Some(render_pipeline);
        self.indices_length = Some(indices.len() as _);
        self.texture_bind_group_layout = Some(texture_bind_group_layout);
        self.rect_bind_group_layout = Some(rect_bind_group_layout);
        self.texture = Some(texture);
    }

    fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        let texture_bind_group = self.create_texture_bind_group(
            device,
            self.texture_bind_group_layout.as_ref().unwrap(),
            self.texture.as_ref().unwrap(),
        );
        self.texture_bind_group = Some(texture_bind_group);

        let rect_bind_group = self.create_rect_bind_group(
            device,
            self.rect_bind_group_layout.as_ref().unwrap(),
            config,
        );
        self.rect_bind_group = Some(rect_bind_group);

        let vertices = self.create_vertices();
        let vertex_buffer = common::create_vertex_buffer(device, &vertices);
        self.vertex_buffer = Some(vertex_buffer);

        render_pass.set_pipeline(self.render_pipeline.as_ref().unwrap());
        render_pass.set_bind_group(0, self.texture_bind_group.as_ref().unwrap(), &[]);
        render_pass.set_bind_group(1, self.rect_bind_group.as_ref().unwrap(), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(
            self.index_buffer.as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..self.indices_length.unwrap(), 0, 0..1);
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

    fn children(&mut self) -> Option<&mut Components> {
        Some(&mut self.children)
    }
}
