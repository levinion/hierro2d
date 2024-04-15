use super::{
    bind_group::{create_bind_group, create_rect_buffer, RectUniform},
    vertex::RectVertex,
    Rect,
};

#[derive(Debug)]
struct NormalizedDisplayConfig {
    size: (f32, f32),
    position: (f32, f32),
    color: (f32, f32, f32, f32),
    radius: f32,
}

impl Rect {
    fn normalize(&self) -> NormalizedDisplayConfig {
        let (width, height) = self.display_config.size;
        let size = (width * 2., height * 2.);
        let (x, y) = self.display_config.position;
        let position = (x * 2. - 1., -y * 2. + 1.);
        let (r, g, b, a) = self.display_config.color;
        let color = (r, g, b, a);
        NormalizedDisplayConfig {
            size,
            position,
            color,
            radius: self.display_config.radius,
        }
    }

    pub(crate) fn create_vertices(&self) -> Vec<RectVertex> {
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

    pub(crate) fn create_bind_group(
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
        create_bind_group(device, layout, &buffer)
    }
}
