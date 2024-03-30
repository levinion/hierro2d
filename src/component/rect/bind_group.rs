use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct RectUniform {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub radius: f32,
    pub _padding: f32,
}

pub fn create_rect_buffer(device: &wgpu::Device, rect: RectUniform) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("rect buffer"),
        contents: bytemuck::cast_slice(&[rect]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}
