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
        let actual_lh = if options.line_height > 3.0 {
            options.line_height
        } else {
            (font_size * options.line_height).max(font_size)
        };
        let half_leading = (actual_lh - font_size).max(0.0) / 2.0;
        let visual_ascent = font_size * 0.85 + half_leading;

        // Calculate a vertical shift to ensure the first line's top starts exactly at pos[1]
        let first_line_y = buffer.lines().first().map(|l| l.y).unwrap_or(visual_ascent);
        let vertical_shift = visual_ascent - first_line_y;

        let clip = options.clip_rect.unwrap_or([0.0, 0.0, 9999.0, 9999.0]);
        let sf = if options.scale_factor > 0.0 { options.scale_factor } else { 1.0 };

        // --- 2. HIGHLIGHT RENDERING ---
        if has_highlight {
            let highlight_color = options.highlight_color.unwrap();
            
            for line in buffer.lines() {
                if line.width > 0.0 {
                    instances.push(crate::types::glyph::GlyphInstance {
                        pos: [
                            ((pos[0] + line.x) * sf).round(), 
                            ((pos[1] + line.y + vertical_shift - visual_ascent) * sf).round()
                        ],
                        size: [(line.width * sf).round(), (actual_lh * sf).round()],
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
        let base_x = (pos[0] * sf).round();
        for glyph in buffer.glyphs() {
            if let Some(entry) = atlas.get(&glyph.key) {
                if entry.pixel_size[0] <= 0.0 || entry.pixel_size[1] <= 0.0 {
                    continue;
                }
                let base_y = ((pos[1] + glyph.y + vertical_shift) * sf).round();
                instances.push(crate::types::glyph::GlyphInstance {
                    pos: [
                        base_x + (glyph.physical_x as f32) + entry.pixel_offset[0],
                        base_y - entry.pixel_offset[1],
                    ],
                    size: entry.pixel_size,
                    uv_pos: entry.uv_pos,
                    uv_size: entry.uv_size,
                    color: color.to_array(),
                    bg_color: [0.0, 0.0, 0.0, if entry.is_color { 1.0 } else { 0.0 }],
                    clip_rect: clip,
                });
            }
        }

        // --- 4. DECORATIONS (UNDERLINE & STRIKETHROUGH) ---
        if options.underline {
            let u_color = options.underline_color.unwrap_or(options.color);
            let u_thickness = (font_size * 0.08 * sf).round().max(1.0);
            let u_offset = font_size * 0.15;
            for line in buffer.lines() {
                if line.width > 0.0 {
                    instances.push(crate::types::glyph::GlyphInstance {
                        pos: [
                            ((pos[0] + line.x) * sf).round(),
                            ((pos[1] + line.y + vertical_shift + u_offset) * sf).round(),
                        ],
                        size: [(line.width * sf).round(), u_thickness],
                        uv_pos: [0.0, 0.0],
                        uv_size: [0.0, 0.0],
                        color: [0.0, 0.0, 0.0, 0.0],
                        bg_color: u_color.to_array(),
                        clip_rect: clip,
                    });
                }
            }
        }

        if options.strikethrough {
            let s_color = options.strikethrough_color.unwrap_or(options.color);
            let s_thickness = (font_size * 0.08 * sf).round().max(1.0);
            let s_offset = -font_size * 0.28;
            for line in buffer.lines() {
                if line.width > 0.0 {
                    instances.push(crate::types::glyph::GlyphInstance {
                        pos: [
                            ((pos[0] + line.x) * sf).round(),
                            ((pos[1] + line.y + vertical_shift + s_offset) * sf).round(),
                        ],
                        size: [(line.width * sf).round(), s_thickness],
                        uv_pos: [0.0, 0.0],
                        uv_size: [0.0, 0.0],
                        color: [0.0, 0.0, 0.0, 0.0],
                        bg_color: s_color.to_array(),
                        clip_rect: clip,
                    });
                }
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
