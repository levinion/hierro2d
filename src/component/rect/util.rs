use super::{
    bind_group::{create_bind_group, create_rect_buffer, RectUniform},
    vertex::RectVertex,
    Rect,
};

impl Rect {
    pub(crate) fn create_vertices(&self) -> Vec<RectVertex> {
        let (x, y) = self.display_config.position;
        let (width, height) = self.display_config.size;
        let (r, g, b, a) = self.display_config.color;
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
        create_bind_group(device, layout, &buffer)
    }
}
