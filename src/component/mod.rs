mod common;
mod container;
mod square;
mod text;

pub use container::Container;
pub use square::Square;
pub use text::Text;

pub trait Component: 'static {
    fn init(
        &mut self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _config: &wgpu::SurfaceConfiguration,
    ) {
    }

    fn prepare(
        &mut self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _config: &wgpu::SurfaceConfiguration,
    ) {
    }

    fn render<'a>(&'a mut self, _render_pass: &mut wgpu::RenderPass<'a>) {}

    fn clean(&mut self) {}

    fn sub_components(&mut self) -> Components {
        vec![]
    }

    fn depth(&self) -> u8 {
        0
    }

    // inner methods

    fn collect_and_init(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Components {
        let mut r = vec![];
        let sub_components = self.sub_components();
        for mut component in sub_components {
            component.init(device, queue, config);
            let mut children_components = component.collect_and_init(device, queue, config);
            r.push(component);
            r.append(&mut children_components);
        }
        r
    }

    fn apply_workspace(&mut self, _size: (f32, f32), _offset: (f32, f32)) {}
}

pub type Components = Vec<Box<dyn Component>>;
