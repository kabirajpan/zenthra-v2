use cosmic_text::{Buffer, CacheKey};

#[derive(Debug, Clone)]
pub struct GlyphPosition {
    pub cache_key: CacheKey,
    pub x: f32,
    pub y: f32,
    pub color: [u8; 4],
}

/// Per-line metrics — used for bg highlight rect generation.
#[derive(Debug, Clone)]
pub struct LineInfo {
    pub x: f32,
    pub y: f32,
    pub width: f32,
}

#[derive(Debug, Clone, Default)]
pub struct TextLayout {
    pub glyphs: Vec<GlyphPosition>,
    pub lines: Vec<LineInfo>,
    pub width: f32,
    pub height: f32,
}

impl TextLayout {
    pub fn from_buffer(buffer: &Buffer) -> Self {
        let mut glyphs = Vec::new();
        let mut lines = Vec::new();
        let mut max_width = 0.0f32;
        let mut max_y = 0.0f32;

        for run in buffer.layout_runs() {
            let mut line_width = 0.0f32;
            let mut line_x = f32::MAX;

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
                let right = glyph.x + glyph.w;
                line_width = line_width.max(right);
                line_x = line_x.min(glyph.x);
                max_width = max_width.max(right);
            }

            if line_x == f32::MAX {
                line_x = 0.0;
            }

            lines.push(LineInfo {
                x: line_x,
                y: run.line_y,
                width: line_width - line_x,
            });

            max_y = max_y.max(run.line_y);
        }

        Self {
            glyphs,
            lines,
            width: max_width,
            height: max_y,
        }
    }
}
