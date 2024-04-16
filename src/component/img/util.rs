use crate::component::rect::RectUniform;

use super::{
    bind_group::{create_rect_bind_group, create_rect_buffer, create_texture_bind_group},
    texture::Texture,
    vertex::ImgVertex,
    Img,
};

impl Img {
    pub(crate) fn create_vertices(&self) -> Vec<ImgVertex> {
        let (x, y) = self.display_config.position;
        let (width, height) = self.display_config.size;
        let vertices = &[
            ImgVertex {
                position: [x, y - height],
                tex_coords: [0., 1.],
            }, // A
            ImgVertex {
                position: [x + width, y - height],
                tex_coords: [1., 1.],
            }, // B
            ImgVertex {
                position: [x + width, y],
                tex_coords: [1., 0.],
            }, // C
            ImgVertex {
                position: [x, y],
                tex_coords: [0., 0.],
            }, // D
        ];
        vertices.into()
    }

    pub(crate) fn create_texture_bind_group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        texture: &Texture,
    ) -> wgpu::BindGroup {
        create_texture_bind_group(device, layout, &texture.view, &texture.sampler)
    }

    pub(crate) fn create_rect_bind_group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::BindGroup {
        let (x, y) = self.display_config.position;
        let (width, height) = self.display_config.size;
        let radius = self.display_config.radius;
        let resolution = [config.width as f32, config.height as f32];
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
        create_rect_bind_group(device, layout, &buffer)
    }
}
