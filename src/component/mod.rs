mod square;
mod text;

pub use square::*;
use wgpu::util::DeviceExt;

use crate::vertex::Vertex;

use self::text::{TextConfig, TextObject, TextRenderer};

pub trait IntoComponent {
    fn into_component(
        self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Component;
}

pub struct ComponentBuilder {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub text_objects: Vec<TextObject>,
}

impl ComponentBuilder {
    pub fn new(vertices: impl Into<Vec<Vertex>>, indices: impl Into<Vec<u16>>) -> Self {
        Self {
            vertices: vertices.into(),
            indices: indices.into(),
            text_objects: vec![],
        }
    }

    fn create_vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn create_index_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    fn create_render_pipeline(
        &self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }

    pub fn add_text_object(&mut self, text_renderer: TextRenderer, text_config: TextConfig) {
        self.text_objects
            .push(TextObject::new(text_renderer, text_config));
    }

    pub fn build(self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Component {
        let vertex_buffer = self.create_vertex_buffer(device);
        let index_buffer = self.create_index_buffer(device);
        let render_pipeline = self.create_render_pipeline(device, config, &[]);
        let indices_length = self.indices.len() as u32;
        let text_objects = self.text_objects;
        Component {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            indices_length,
            text_objects,
        }
    }
}

pub struct Component {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices_length: u32,
    text_objects: Vec<TextObject>,
}

impl<'a> Component {
    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) {
        self.text_objects
            .iter_mut()
            .for_each(|object| object.prepare(device, queue, config));
    }

    pub fn render(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, slot: u32) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(slot, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices_length, 0, 0..1);
        self.text_objects
            .iter_mut()
            .for_each(|object| object.render(render_pass));
    }

    pub fn clean(&mut self) {
        self.text_objects
            .iter_mut()
            .for_each(|object| object.clean());
    }
}
