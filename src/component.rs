use wgpu::util::DeviceExt;

use crate::vertex::Vertex;

pub struct Component {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl<'a> Component {
    pub fn render(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        slot: u32,
        vertex_buffer: &'a wgpu::Buffer,
        index_buffer: &'a wgpu::Buffer,
    ) {
        render_pass.set_vertex_buffer(slot, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    }

    pub fn create_vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn create_index_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        })
    }
}

pub struct Square {
    size: (f32, f32),
    position: (f32, f32),
    color: (u16, u16, u16),
}

impl Square {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = (width, height);
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);
        self
    }

    pub fn color(mut self, r: u16, g: u16, b: u16) -> Self {
        self.color = (r, g, b);
        self
    }
}

impl Default for Square {
    fn default() -> Self {
        Self {
            size: (0., 0.),
            position: (0., 0.),
            color: (0, 0, 255),
        }
    }
}

impl From<Square> for Component {
    fn from(value: Square) -> Self {
        let (width, height) = value.size;
        let (width, height) = (width * 2., height * 2.);
        let (x, y) = value.position;
        let (x, y) = (x * 2. - 1., -y * 2. + 1.);
        let (r, g, b) = value.color;
        let (r, g, b) = (r as f32 / 255., g as f32 / 255., b as f32 / 255.);
        let vertices = [
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
        ]
        .to_vec();

        let indices = [0, 1, 2, 2, 3, 0].to_vec();
        Self { vertices, indices }
    }
}
