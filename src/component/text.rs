use glyphon::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache, TextArea, TextAtlas,
    TextBounds, TextRenderer as GlTextRenderer,
};
use wgpu::TextureFormat;

use super::Component;

#[derive(Default)]
pub struct Text {
    renderer: Option<TextRenderer>,
    config: TextConfig,
    depth: u8,
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
        _window: std::sync::Arc<winit::window::Window>,
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

    fn depth(&self) -> u8 {
        self.depth
    }

    fn position_mut(&mut self) -> Option<(&mut f32, &mut f32)> {
        Some((&mut self.config.left, &mut self.config.top))
    }

    fn size_mut(&mut self) -> Option<(&mut f32, &mut f32)> {
        None
    }
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, r: u8, g: u8, b: u8, a: f32) -> Self {
        self.config.color = glyphon::Color::rgba(r, g, b, (a * 255.) as u8);
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.config.content = content.into();
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.config.left = x;
        self.config.top = y;
        self
    }

    pub fn bound(mut self, x: i32, y: i32) -> Self {
        self.config.text_bounds.bottom = y;
        self.config.text_bounds.right = x;
        self
    }

    pub fn depth(mut self, depth: u8) -> Self {
        self.depth = depth;
        self
    }

    pub fn center(mut self) -> Self {
        if let Some(size) = self.size_mut() {
            let size = (*size.0, *size.1);
            if let Some(position) = self.position_mut() {
                (*position.0, *position.1) = ((1. - size.0) / 2., (1. - size.1) / 2.);
            }
        }
        self
    }

    pub fn center_x(mut self) -> Self {
        if let Some(size) = self.size_mut() {
            let size = (*size.0, *size.1);
            if let Some(position) = self.position_mut() {
                *position.0 = (1. - size.0) / 2.;
            }
        }
        self
    }

    pub fn center_y(mut self) -> Self {
        if let Some(size) = self.size_mut() {
            let size = (*size.0, *size.1);
            if let Some(position) = self.position_mut() {
                *position.1 = (1. - size.1) / 2.;
            }
        }
        self
    }
}

pub struct TextRenderer {
    cache: SwashCache,
    gl_renderer: GlTextRenderer,
    font_system: FontSystem,
    atlas: TextAtlas,
    buffer: Buffer,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        text_config: &TextConfig,
    ) -> Self {
        let format = TextureFormat::Bgra8UnormSrgb;
        let mut font_system = FontSystem::new();
        let cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, format);
        let gl_renderer =
            GlTextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None);

        let mut buffer = Buffer::new(&mut font_system, Metrics::new(30.0, 42.0));
        let physical_width = config.width as f32;
        let physical_height = config.height as f32;
        buffer.set_size(&mut font_system, physical_width, physical_height);
        buffer.set_text(
            &mut font_system,
            &text_config.content,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut font_system);

        Self {
            gl_renderer,
            cache,
            font_system,
            atlas,
            buffer,
        }
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        text_config: &TextConfig,
    ) {
        let text_config = text_config.fit_screen(config);
        self.gl_renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                glyphon::Resolution {
                    width: config.width,
                    height: config.height,
                },
                [TextArea {
                    buffer: &self.buffer,
                    left: text_config.left,
                    top: text_config.top,
                    scale: text_config.scale,
                    bounds: text_config.text_bounds,
                    default_color: text_config.color,
                }],
                &mut self.cache,
            )
            .unwrap();
    }
}

#[derive(Clone, Debug)]
pub struct TextConfig {
    pub(crate) left: f32,
    pub(crate) top: f32,
    pub(crate) scale: f32,
    pub(crate) color: glyphon::Color,
    pub(crate) text_bounds: TextBounds,
    pub(crate) content: String,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            left: 0.,
            top: 0.,
            scale: 1.,
            color: glyphon::Color::rgba(255, 255, 255, 255),
            text_bounds: TextBounds::default(),
            content: String::new(),
        }
    }
}

impl TextConfig {
    pub(crate) fn fit_screen(&self, config: &wgpu::SurfaceConfiguration) -> Self {
        let mut new_instance = self.clone();
        new_instance.left = self.left * config.width as f32;
        new_instance.top = self.top * config.height as f32;
        new_instance
    }
}
