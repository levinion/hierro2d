mod api;
mod config;
mod renderer;

use crate::context::Context;

use self::{config::TextConfig, renderer::TextRenderer};

use super::{Component, IntoComponent};

#[derive(Default)]
pub struct Text {
    renderer: Option<TextRenderer>,
    config: TextConfig,
    depth: i32,
    on_click: Option<fn(Context)>,
    id: isize,
}

impl Component for Text {
    fn init(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) {
        let renderer = TextRenderer::new(device, queue, config, &self.config);
        self.renderer = Some(renderer);
    }

    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) {
        self.renderer
            .as_mut()
            .unwrap()
            .prepare(device, queue, config, &self.config);
    }

    fn render<'a, 'b>(
        &'a mut self,
        _device: &wgpu::Device,
        _config: &wgpu::SurfaceConfiguration,
        render_pass: &mut wgpu::RenderPass<'b>,
    ) where
        'a: 'b,
    {
        self.renderer
            .as_ref()
            .unwrap()
            .gl_renderer
            .render(&self.renderer.as_ref().unwrap().atlas, render_pass)
            .unwrap();
    }

    fn clean(&mut self) {
        self.renderer.as_mut().unwrap().atlas.trim();
    }

    fn depth(&self) -> i32 {
        self.depth
    }

    fn set_depth(&mut self, depth: i32) {
        self.depth = depth;
    }

    fn get_position(&self) -> (f32, f32) {
        (self.config.left, self.config.top)
    }

    fn set_position(&mut self, position: (f32, f32)) {
        self.config.left = position.0;
        self.config.top = position.1;
    }

    fn get_size(&self) -> (f32, f32) {
        (2., 2.)
    }

    fn set_size(&mut self, _size: (f32, f32)) {}

    fn click_handler(&self) -> Option<fn(Context)> {
        self.on_click.clone()
    }

    fn get_id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id;
    }
}

impl IntoComponent for Text {
    fn into_comp(self) -> super::Comp {
        super::Comp::Text(self)
    }
}
