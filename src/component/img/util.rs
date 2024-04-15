use crate::component::rect::RectUniform;

use super::{
    bind_group::{create_rect_bind_group, create_rect_buffer, create_texture_bind_group},
    texture::Texture,
    vertex::ImgVertex,
    Img,
};

#[derive(Debug)]
struct NormalizedDisplayConfig {
    size: (f32, f32),
    position: (f32, f32),
    radius: f32,
}

impl Img {
    fn normalize(&self) -> NormalizedDisplayConfig {
        let (width, height) = self.display_config.size;
        let size = (width * 2., height * 2.);
        let (x, y) = self.display_config.position;
        let position = (x * 2. - 1., -y * 2. + 1.);
        NormalizedDisplayConfig {
            size,
            position,
            radius: self.display_config.radius,
        }
    }

    pub(crate) fn create_vertices(&self) -> Vec<ImgVertex> {
        let NormalizedDisplayConfig {
            size: (width, height),
            position: (x, y),
            ..
        } = self.normalize();
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
        let NormalizedDisplayConfig {
            size: (width, height),
            position: (x, y),
            radius,
            ..
        } = self.normalize();
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
