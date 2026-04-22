use zenthra_platform::app::{App as PlatformApp, Frame};
use zenthra_render::{RectPipeline, TextPipeline};
use zenthra_text::{FontSystem, GlyphAtlas, ShapedText, TextLayout, TextProperties};
use zenthra_widgets::Ui;

pub struct App {
    platform: PlatformApp,
}

impl App {
    pub fn new() -> Self {
        Self {
            platform: PlatformApp::new(),
        }
    }

    pub fn title(mut self, t: &str) -> Self {
        self.platform = self.platform.title(t);
        self
    }

    pub fn size(mut self, w: u32, h: u32) -> Self {
        self.platform = self.platform.size(w, h);
        self
    }

    pub fn with_ui<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Ui) + 'static,
    {
        let mut font_system: FontSystem = FontSystem::new();
        let mut rect_pipeline: Option<RectPipeline> = None;
        let mut text_pipeline: Option<TextPipeline> = None;
        let mut glyph_atlas: Option<GlyphAtlas> = None;

        self.platform = self.platform.with_ui(move |frame: &mut Frame| {
            let device = &frame.window.gpu.device;
            let queue = &frame.window.gpu.queue;
            let format = frame.window.gpu.config.format;
            let width = frame.window.width();
            let height = frame.window.height();

            // Init pipelines once
            let rp = rect_pipeline.get_or_insert_with(|| RectPipeline::new(device, format));
            let tp = text_pipeline.get_or_insert_with(|| TextPipeline::new(device, format));
            let ga = glyph_atlas.get_or_insert_with(|| {
                let atlas = GlyphAtlas::new(device);
                tp.set_atlas(device, &atlas.texture_view);
                atlas
            });

            // Build UI
            let mut ui = Ui::new(width, height, frame.scale_factor());
            f(&mut ui);

            // ── Rects ─────────────────────────────────────────────────────
            let rects: Vec<_> = ui.rect_draws.iter().map(|r| r.instance).collect();
            rp.prepare(device, queue, width, height, &rects);

            // ── Text ──────────────────────────────────────────────────────
            let mut glyph_instances = Vec::new();

            for td in &ui.text_draws {
                let props = TextProperties {
                    text: td.text.clone(),
                    font_size: td.font_size,
                    color: td.color,
                    weight: if td.bold { 700 } else { 400 },
                    italic: td.italic,
                    ..Default::default()
                };

                let shaped = ShapedText::shape(&mut font_system.inner, &props, td.max_width);
                let layout = TextLayout::from_buffer(&shaped.buffer);

                for glyph in &layout.glyphs {
                    if let Some(ag) = ga.get_or_insert(&mut font_system.inner, glyph.cache_key) {
                        let x = td.x + glyph.x + ag.left as f32;
                        let y = td.y + glyph.y - ag.top as f32;

                        glyph_instances.push(zenthra_render::GlyphInstance {
                            pos: [x, y],
                            size: [ag.width as f32, ag.height as f32],
                            uv0: [ag.u0, ag.v0],
                            uv1: [ag.u1, ag.v1],
                            color: [
                                glyph.color[0] as f32 / 255.0,
                                glyph.color[1] as f32 / 255.0,
                                glyph.color[2] as f32 / 255.0,
                                glyph.color[3] as f32 / 255.0,
                            ],
                        });
                    }
                }
            }

            ga.flush(queue);
            tp.set_atlas(device, &ga.texture_view);
            tp.prepare(device, queue, width, height, &glyph_instances);

            // ── Render ────────────────────────────────────────────────────
            let surface_texture = match frame.window.gpu.surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(t) => t,
                wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
                _ => return,
            };

            let view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Zenthra Frame"),
            });

            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Main Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.05,
                                g: 0.05,
                                b: 0.07,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });

                rp.draw(&mut pass);
                tp.draw(&mut pass);
            }

            queue.submit(std::iter::once(encoder.finish()));
            surface_texture.present();
        });

        self
    }

    pub fn run(self) {
        self.platform.run();
    }
}
