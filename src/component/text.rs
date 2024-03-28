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
                    left: 10.0,
                    top: 10.0,
                    scale: 1.0,
                    bounds: text_config.text_bounds,
                    default_color: text_config.color,
                }],
                &mut self.cache,
            )
            .unwrap();
    }
}

pub struct TextConfig {
    pub color: glyphon::Color,
    pub text_bounds: TextBounds,
    pub content: String,
}
