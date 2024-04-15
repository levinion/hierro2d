mod common;
mod container;
mod empty;
mod img;
mod rect;
mod text;

pub use container::Container;
pub use empty::Empty;
pub use img::Img;
pub use rect::Rect;
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

    fn render<'a>(
        &'a mut self,
        _device: &wgpu::Device,
        _config: &wgpu::SurfaceConfiguration,
        _render_pass: &mut wgpu::RenderPass<'a>,
    ) {
    }

    fn clean(&mut self) {}

    fn children(&mut self) -> Option<&mut Components> {
        None
    }

    fn depth(&self) -> i32;

    fn set_depth(&mut self, depth: i32);

    fn get_size(&self) -> (f32, f32);

    fn set_size(&mut self, size: (f32, f32));

    fn get_position(&self) -> (f32, f32);

    fn set_position(&mut self, position: (f32, f32));

    /// inner method
    fn apply_workspace(&mut self, size: (f32, f32), offset: (f32, f32)) {
        let position = self.get_position();
        self.set_position((
            offset.0 + position.0 * size.0,
            offset.1 + position.1 * size.1,
        ));
        let ori_size = self.get_size();
        self.set_size((ori_size.0 * size.0, ori_size.1 * size.1));
        if let Some(children) = self.children() {
            children
                .iter_mut()
                .for_each(|child| child.apply_workspace(size, offset));
        }
    }

    /// inner method
    fn collect_and_init(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Components {
        let mut r = vec![];
        let children = self.take_children();
        for mut child in children {
            child.init(device, queue, config);
            let mut children_components = child.collect_and_init(device, queue, config);
            r.push(child);
            r.append(&mut children_components);
        }
        r
    }

    /// inner method
    fn take_children(&mut self) -> Components {
        match self.children() {
            Some(children) => std::mem::take(children),
            None => vec![],
        }
    }
}

pub type Components = Vec<Box<dyn Component>>;
