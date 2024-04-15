use glyphon::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache, TextArea, TextAtlas,
    TextRenderer as GlTextRenderer,
};
use wgpu::TextureFormat;

use super::config::TextConfig;

pub struct TextRenderer {
    pub cache: SwashCache,
    pub gl_renderer: GlTextRenderer,
    pub font_system: FontSystem,
    pub atlas: TextAtlas,
    pub buffer: Buffer,
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
