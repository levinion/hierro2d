mod common;
mod container;
mod empty;
mod rect;
mod text;

pub use container::Container;
pub use empty::Empty;
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

    fn render<'a>(&'a mut self, _render_pass: &mut wgpu::RenderPass<'a>) {}

    fn clean(&mut self) {}

    fn children(&mut self) -> Option<&mut Components> {
        None
    }

    fn depth(&self) -> u8 {
        0
    }

    fn size_mut(&mut self) -> Option<(&mut f32, &mut f32)>;

    fn position_mut(&mut self) -> Option<(&mut f32, &mut f32)>;

    // inner methods

    fn apply_workspace(&mut self, size: (f32, f32), offset: (f32, f32)) {
        if let Some((x, y)) = self.position_mut() {
            *x = offset.0 + *x * size.0;
            *y = offset.1 + *y * size.1;
        }
        if let Some((width, height)) = self.size_mut() {
            *width *= size.0;
            *height *= size.1;
        }
        if let Some(children) = self.children() {
            children
                .iter_mut()
                .for_each(|child| child.apply_workspace(size, offset));
        }
    }

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

    fn take_children(&mut self) -> Components {
        match self.children() {
            Some(children) => std::mem::take(children),
            None => vec![],
        }
    }
}

pub type Components = Vec<Box<dyn Component>>;
