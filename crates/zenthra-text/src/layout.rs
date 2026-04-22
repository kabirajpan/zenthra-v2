use cosmic_text::{Buffer, CacheKey};

/// A single positioned glyph ready for atlas lookup + GPU upload.
#[derive(Debug, Clone)]
pub struct GlyphPosition {
    pub cache_key: CacheKey,
    pub x: f32,
    pub y: f32,
    pub color: [u8; 4],
}

/// The result of walking a shaped Buffer's layout runs.
#[derive(Debug, Clone, Default)]
pub struct TextLayout {
    pub glyphs: Vec<GlyphPosition>,
    pub width: f32,
    pub height: f32,
}

impl TextLayout {
    pub fn from_buffer(buffer: &Buffer) -> Self {
        let mut glyphs = Vec::new();
        let mut max_width = 0.0f32;
        let mut max_y = 0.0f32;

        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical = glyph.physical((0.0, 0.0), 1.0);
                let color = match glyph.color_opt {
                    Some(c) => [c.r(), c.g(), c.b(), c.a()],
                    None => [255u8, 255, 255, 255],
                };
                glyphs.push(GlyphPosition {
                    cache_key: physical.cache_key,
                    x: glyph.x,
                    y: run.line_y,
                    color,
                });
                max_width = max_width.max(glyph.x + glyph.w);
            }
            max_y = max_y.max(run.line_y);
        }

        Self {
            glyphs,
            width: max_width,
            height: max_y,
        }
    }
}
