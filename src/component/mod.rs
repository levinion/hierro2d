mod common;
mod square;
mod text;

pub use square::*;

pub trait Component {
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
}

pub type Components = Vec<Box<dyn Component>>;
