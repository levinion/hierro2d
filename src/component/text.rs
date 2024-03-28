use glyphon::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache, TextArea, TextAtlas,
    TextBounds, TextRenderer as GlTextRenderer,
};
use wgpu::TextureFormat;

pub struct TextObject {
    renderer: TextRenderer,
    config: TextConfig,
}

impl<'a> TextObject {
    pub fn new(renderer: TextRenderer, config: TextConfig) -> Self {
        Self { renderer, config }
    }

    pub fn prepare(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) {
        self.renderer.prepare(device, queue, config, &self.config);
    }

    pub fn render(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.renderer
            .renderer
            .render(&self.renderer.atlas, render_pass)
            .unwrap();
    }

    pub fn clean(&mut self) {
        self.renderer.atlas.trim();
    }
}

impl Drop for TextObject {
    fn drop(&mut self) {
        self.renderer.atlas.trim();
    }
}

pub struct TextRenderer {
    cache: SwashCache,
    renderer: GlTextRenderer,
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
        let renderer =
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
            renderer,
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
        self.renderer
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

#[derive(Clone)]
pub struct TextConfig {
    pub(crate) left: f32,
    pub(crate) top: f32,
    pub(crate) scale: f32,
    pub(crate) color: glyphon::Color,
    pub(crate) text_bounds: TextBounds,
    pub(crate) content: String,
}

impl TextConfig {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Self {
            left: x,
            top: y,
            scale: 1.,
            color: glyphon::Color::rgb(255, 255, 255),
            text_bounds: TextBounds::default(),
            content: String::new(),
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
