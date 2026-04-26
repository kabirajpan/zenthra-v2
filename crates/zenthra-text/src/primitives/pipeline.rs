use crate::primitives::atlas::GlyphAtlas;

pub struct ZentypePipeline {
    pub(crate) inner: crate::gpu::pipeline::TextPipeline,
}

impl ZentypePipeline {
    pub fn new(
        device: &wgpu::Device,
        atlas: &GlyphAtlas,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        Self {
            inner: crate::gpu::pipeline::TextPipeline::new(device, &atlas.inner, config),
        }
    }

    pub fn update_screen_size(&self, queue: &wgpu::Queue, width: f32, height: f32) {
        self.inner.update_screen_size(queue, width, height);
    }

    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.inner.pipeline
    }

    pub fn atlas_bind_group(&self) -> &wgpu::BindGroup {
        &self.inner.atlas_bind_group
    }

    pub fn uniform_bind_group(&self) -> &wgpu::BindGroup {
        &self.inner.uniform_bind_group
    }

    /// Generates raw GPU instance data for a shaped buffer at a specific position.
    pub fn generate_instances(
        &self,
        buffer: &crate::primitives::shaped_buffer::ShapedBuffer,
        atlas: &GlyphAtlas,
        pos: [f32; 2],
        options: &crate::types::options::TextOptions,
    ) -> Vec<crate::types::glyph::GlyphInstance> {
        // We pre-allocate space for backgrounds and glyphs
        let has_highlight = options.highlight_color.map(|c| c.a > 0.0).unwrap_or(false);

        let bg_count = if has_highlight { 1 } else { 0 };
        let mut instances = Vec::with_capacity(buffer.len() + bg_count);

        // --- 1. METRICS & ALIGNMENT ---
        let font_size = options.font_size;
        let lh = options.line_height;
        let visual_ascent = font_size * (0.8 + (lh - 1.0) / 2.0);

        // Calculate a vertical shift to ensure the first line's top starts exactly at pos[1]
        let first_line_y = buffer.lines().first().map(|l| l.y).unwrap_or(visual_ascent);
        let vertical_shift = visual_ascent - first_line_y;

        let clip = options.clip_rect.unwrap_or([0.0, 0.0, 9999.0, 9999.0]);
        let sf = options.scale_factor;

        // --- 2. HIGHLIGHT RENDERING ---
        if has_highlight {
            let highlight_color = options.highlight_color.unwrap();
            
            for line in buffer.lines() {
                if line.width > 0.0 {
                    instances.push(crate::types::glyph::GlyphInstance {
                        pos: [
                            (pos[0] + line.x) * sf, 
                            (pos[1] + line.y + vertical_shift - visual_ascent) * sf
                        ],
                        size: [line.width * sf, (font_size * lh) * sf],
                        uv_pos: [0.0, 0.0],
                        uv_size: [0.0, 0.0],
                        color: [0.0, 0.0, 0.0, 0.0],
                        bg_color: highlight_color.to_array(),
                        clip_rect: clip,
                    });
                }
            }
        }

        // --- 3. GLYPH RENDERING ---
        let color = options.color;
        for glyph in buffer.glyphs() {
            if let Some(entry) = atlas.get(&glyph.key) {
                instances.push(crate::types::glyph::GlyphInstance {
                    pos: [
                        (pos[0] + glyph.x) * sf + entry.pixel_offset[0],
                        (pos[1] + glyph.y + vertical_shift) * sf - entry.pixel_offset[1],
                    ],
                    size: entry.pixel_size,
                    uv_pos: entry.uv_pos,
                    uv_size: entry.uv_size,
                    color: color.to_array(),
                    bg_color: [0.0, 0.0, 0.0, 0.0],
                    clip_rect: clip,
                });
            }
        }

        instances
    }

    /// Records the commands to draw a shaped buffer using the provided atlas and instance buffer.
    pub fn draw_buffer<'a>(
        &'a self,
        rpass: &mut wgpu::RenderPass<'a>,
        buffer: &crate::primitives::shaped_buffer::ShapedBuffer,
        _atlas: &GlyphAtlas, // Atlas is already bound inside the pipeline, but we keep it in the API for safety/consistency
        instance_buffer: &'a wgpu::Buffer,
    ) {
        self.inner.draw(rpass, instance_buffer, buffer.len() as u32);
    }
}
