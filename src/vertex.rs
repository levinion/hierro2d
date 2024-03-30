pub trait Vertex: bytemuck::Pod {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}
