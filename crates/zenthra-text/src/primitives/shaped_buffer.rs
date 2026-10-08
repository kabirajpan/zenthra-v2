use crate::types::shaped_glyph::ShapedGlyph;
use crate::types::line::LineInfo;

/// A buffer of shaped and positioned glyphs.
/// This is the final output of a text shaping operation.
#[derive(Debug, Clone, Default)]
pub struct ShapedBuffer {
    glyphs: Vec<ShapedGlyph>,
    lines: Vec<LineInfo>,
    width: f32,
    height: f32,
}

impl ShapedBuffer {
    /// Creates a new empty ShapedBuffer.
    pub fn new(glyphs: Vec<ShapedGlyph>, lines: Vec<LineInfo>, width: f32, height: f32) -> Self {
        Self { glyphs, lines, width, height }
    }

    /// Returns the list of shaped glyphs in this buffer.
    pub fn glyphs(&self) -> &[ShapedGlyph] {
        &self.glyphs
    }

    /// Returns the tracked line information for this buffer.
    pub fn lines(&self) -> &[LineInfo] {
        &self.lines
    }

    /// Returns the number of glyphs in the buffer.
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Returns true if the buffer contains no glyphs.
    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    /// Returns the logical width and height of the text content itself (no padding).
    pub fn content_size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    /// Returns the visual width and height including the provided padding.
    pub fn outer_size(&self, padding: &crate::types::options::Padding) -> (f32, f32) {
        (
            self.width + padding.left + padding.right,
            self.height + padding.top + padding.bottom,
        )
    }

    /// Returns the logical (width, height) used during the last shaping pass.
    /// This is an alias for `content_size()`.
    pub fn size(&self) -> (f32, f32) {
        self.content_size()
    }

    /// Finds the character byte index closest to the given (x, y) coordinates.
    pub fn index_at(&self, x: f32, y: f32) -> usize {
        if self.lines.is_empty() || self.glyphs.is_empty() {
            return 0;
        }

        // 1. Find the line with the closest Y coordinate
        let mut best_line_idx = 0;
        let mut min_dist_y = f32::MAX;

        for (i, line) in self.lines.iter().enumerate() {
            let dist = (y - line.y).abs();
            if dist < min_dist_y {
                min_dist_y = dist;
                best_line_idx = i;
            }
        }

        let best_line = &self.lines[best_line_idx];

        // 2. Find the glyph on that line with the closest X coordinate using character midpoints
        let line_glyphs: Vec<&crate::types::shaped_glyph::ShapedGlyph> = self.glyphs.iter()
            .filter(|glyph| (glyph.y - best_line.y).abs() < 2.0)
            .collect();

        if line_glyphs.is_empty() {
            return self.glyphs.first().map(|g| g.cluster).unwrap_or(0);
        }

        for (i, glyph) in line_glyphs.iter().enumerate() {
            let midpoint = glyph.x + glyph.width * 0.5;
            if x < midpoint {
                return glyph.cluster;
            }
            let is_last = i + 1 >= line_glyphs.len();
            if is_last {
                return glyph.cluster + 1;
            }
            let next_midpoint = line_glyphs[i + 1].x + line_glyphs[i + 1].width * 0.5;
            if x < next_midpoint {
                return line_glyphs[i + 1].cluster;
            }
        }

        line_glyphs.last().map(|g| g.cluster + 1).unwrap_or(0)
    }

    /// Returns the (x, y) coordinates for a given character byte index.
    pub fn position_at(&self, index: usize) -> Option<(f32, f32)> {
        self.glyphs.iter()
            .find(|g| g.cluster == index)
            .map(|g| (g.x, g.y))
    }
}



