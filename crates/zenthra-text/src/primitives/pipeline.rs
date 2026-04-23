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
        let has_bg = options.bg_color.map(|c| c.a > 0.0).unwrap_or(false);

        let bg_count = if has_bg { buffer.lines().len() } else { 0 };
        let mut instances = Vec::with_capacity(buffer.len() + bg_count);

        // --- 1. AUTOMATIC BACKGROUND GENERATION ---
        let font_size = options.font_size;
        let lh = options.line_height;
        let _box_height = font_size * lh;
        let visual_ascent = font_size * (0.8 + (lh - 1.0) / 2.0);
        let padding = options.padding;

        // Calculate a vertical shift to ensure the first line's top starts exactly at pos[1] + padding.top
        // The first line's l.y is its baseline relative to some origin.
        // We want (pos[1] + padding.top + line.y + shift) - visual_ascent == pos[1] + padding.top
        // So: line.y + shift - visual_ascent == 0 => shift = visual_ascent - line.y
        let first_line_y = buffer.lines().first().map(|l| l.y).unwrap_or(visual_ascent);
        let vertical_shift = visual_ascent - first_line_y;

        let clip = options.clip_rect.unwrap_or([0.0, 0.0, 9999.0, 9999.0]);

        if has_bg {
            let bg_color = options.bg_color.unwrap();
            let sf = options.scale_factor;

            // DRAW A SINGLE UNIFIED BACKGROUND FOR THE WHOLE BLOCK
            let (cw, ch) = buffer.content_size();
            let mut width = cw + padding.left + padding.right;

            if options.full_width_bg {
                 width = options.max_width.unwrap_or(width);
            }

            if let Some(min_w) = options.min_width {
                if width < min_w {
                    width = min_w;
                }
            }

            instances.insert(0, crate::types::glyph::GlyphInstance {
                pos: [pos[0] * sf, pos[1] * sf], // Start at widget top
                size: [width * sf, (ch + padding.top + padding.bottom) * sf], // Full multi-line padded height
                uv_pos: [0.0, 0.0],
                uv_size: [0.0, 0.0],
                color: [0.0, 0.0, 0.0, 0.0],
                bg_color: bg_color.to_array(),
                clip_rect: clip,
            });
        }

        // --- 2. GLYPH RENDERING ---
        let color = options.color;
        let sf = options.scale_factor;
        for glyph in buffer.glyphs() {
            if let Some(entry) = atlas.get(&glyph.key) {
                instances.push(crate::types::glyph::GlyphInstance {
                    pos: [
                        (pos[0] + glyph.x + options.padding.left) * sf + entry.pixel_offset[0],
                        (pos[1] + glyph.y + options.padding.top + vertical_shift) * sf - entry.pixel_offset[1],
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
