use glyphon::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache, TextArea, TextAtlas,
    TextBounds, TextRenderer as GlTextRenderer,
};
use wgpu::TextureFormat;

use super::Component;

#[derive(Default)]
pub struct TextObject {
    renderer: Option<TextRenderer>,
    config: TextConfig,
}

impl Component for TextObject {
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

    fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
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
        0
    }
}

impl TextObject {
    pub fn new(config: TextConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
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
            color: glyphon::Color::rgb(255, 255, 255),
            text_bounds: TextBounds::default(),
            content: String::new(),
        }
    }
}

impl TextConfig {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Self {
            left: x,
            top: y,
            ..Default::default()
        }
    }

    pub(crate) fn fit_screen(&self, config: &wgpu::SurfaceConfiguration) -> Self {
        let mut new_instance = self.clone();
        new_instance.left = self.left * config.width as f32;
        new_instance.top = self.top * config.height as f32;
        new_instance
    }
    pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.color = glyphon::Color::rgb(r, g, b);
        self
    }

    pub fn set_content(&mut self, content: impl Into<String>) -> &mut Self {
        self.content = content.into();
        self
    }

    pub fn set_bound(&mut self, x: i32, y: i32) -> &mut Self {
        self.text_bounds.bottom = y;
        self.text_bounds.right = x;
        self
    }
}
