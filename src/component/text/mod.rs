mod api;
mod config;
mod renderer;

use self::{config::TextConfig, renderer::TextRenderer};

use super::Component;

#[derive(Default)]
pub struct Text {
    renderer: Option<TextRenderer>,
    config: TextConfig,
    depth: i32,
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

    fn render<'a>(
        &'a mut self,
        _device: &wgpu::Device,
        _config: &wgpu::SurfaceConfiguration,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
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
        (0., 0.)
    }

    fn set_size(&mut self, _size: (f32, f32)) {}
}
