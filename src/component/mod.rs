mod common;
mod container;
mod img;
mod rect;
mod text;

pub use container::Container;
use enum_dispatch::enum_dispatch;
pub use img::Img;
pub use rect::Rect;
pub use text::Text;

use crate::context::Context;

pub trait IntoComponent {
    fn into_comp(self) -> Comp;
}

#[enum_dispatch(Component)]
pub enum Comp {
    Rect(Rect),
    Img(Img),
    Text(Text),
}

#[enum_dispatch]
pub trait Component: 'static + Sized {
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

    fn render<'a, 'b>(
        &'a mut self,
        _device: &wgpu::Device,
        _config: &wgpu::SurfaceConfiguration,
        _render_pass: &mut wgpu::RenderPass<'b>,
    ) where
        'a: 'b,
    {
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

    fn get_id(&self) -> isize;

    fn set_id(&mut self, id: isize);

    #[allow(unused_mut)]
    fn center(mut self) -> Self {
        let size = self.get_size();
        self.set_position(((2. - size.0) / 2. - 1., 1. - (2. - size.1) / 2.));
        self
    }

    #[allow(unused_mut)]
    fn center_x(mut self) -> Self {
        let size = self.get_size();
        let position = self.get_position();
        self.set_position(((2. - size.0) / 2. - 1., position.1));
        self
    }

    #[allow(unused_mut)]
    fn center_y(mut self) -> Self {
        let size = self.get_size();
        let position = self.get_position();
        self.set_position((position.0, 1. - (2. - size.1) / 2.));
        self
    }

    /// inner method
    fn apply_workspace(&mut self, size: (f32, f32), offset: (f32, f32)) {
        let position = self.get_position();
        self.set_position((
            offset.0 + (position.0 + 1.) * size.0 / 2.,
            offset.1 + (position.1 - 1.) * size.1 / 2.,
        ));
        let ori_size = self.get_size();
        self.set_size((ori_size.0 * size.0 / 2., ori_size.1 * size.1 / 2.));
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

    /// inner method
    fn click_handler(&self) -> Option<fn(Context)> {
        None
    }
}

pub type Components = Vec<Comp>;
