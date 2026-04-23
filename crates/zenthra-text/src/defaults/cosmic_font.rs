use crate::primitives::shaped_buffer::ShapedBuffer;
use crate::traits::font_provider::{FontMetrics, FontProvider};
use crate::types::options::TextOptions;
use crate::types::shaped_glyph::ShapedGlyph;
use cosmic_text::{Align, Buffer, FontSystem, Metrics, Shaping};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// A FontProvider that uses the `cosmic-text` library for shaping and layout.
pub struct CosmicFontProvider {
    font_system: Arc<Mutex<FontSystem>>,
    buffer: Buffer,
}

impl CosmicFontProvider {
    /// Creates a new CosmicFontProvider with system fonts loaded.
    pub fn new() -> Self {
        let mut font_system = FontSystem::new();
        let buffer = Buffer::new(&mut font_system, Metrics::new(16.0, 24.0));
        Self {
            font_system: Arc::new(Mutex::new(font_system)),
            buffer,
        }
    }

    /// Creates a new CosmicFontProvider using an existing font system.
    pub fn new_with_system(font_system: Arc<Mutex<FontSystem>>) -> Self {
        let mut fs = font_system.lock().unwrap();
        let buffer = Buffer::new(&mut fs, Metrics::new(16.0, 24.0));
        drop(fs);
        
        Self {
            font_system,
            buffer,
        }
    }

    /// Performs shaping using an external font system lock.
    /// This is useful when the caller already holds a lock on the FontSystem.
    pub fn shape_with_system(&mut self, font_system: &mut FontSystem, text: &str, options: &TextOptions) -> ShapedBuffer {
        // 1. Set metrics
        let metrics = Metrics::new(options.font_size, options.font_size * options.line_height);
        self.buffer.set_metrics(font_system, metrics);

        // 2. Set text
        self.buffer.set_text(
            font_system,
            text,
            &options.as_attrs(),
            Shaping::Advanced,
            None,
        );

        // 3. Apply alignment
        if let Some(alignment) = options.align {
            let align: Align = alignment.into();
            for line in self.buffer.lines.iter_mut() {
                line.set_align(Some(align));
            }
        }

        // 4. Shape
        self.buffer.shape_until_scroll(font_system, false);

        // 5. Convert to ShapedBuffer
        let mut shaped_glyphs = Vec::new();
        let mut lines = Vec::new();
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        let descent = -(metrics.line_height - metrics.font_size);
        for run in self.buffer.layout_runs() {
            max_height = max_height.max(run.line_y - descent);
            let layout_width = self.buffer.size().0.unwrap_or(run.line_w);
            let alignment_offset = match options.align {
                Some(crate::types::HorizontalAlignment::Center) => (layout_width - run.line_w) / 2.0,
                Some(crate::types::HorizontalAlignment::Right) => layout_width - run.line_w,
                _ => 0.0,
            };

            lines.push(crate::types::line::LineInfo { 
                x: alignment_offset, 
                y: run.line_y, 
                width: run.line_w,
                start_cluster: run.glyphs.first().map(|g| g.start).unwrap_or(text.len()),
            });
            for glyph in run.glyphs {
                max_width = max_width.max(glyph.x + glyph.w);
                let physical = glyph.physical((0.0, 0.0), 1.0);
                shaped_glyphs.push(ShapedGlyph {
                    key: physical.cache_key,
                    cluster: glyph.start,
                    x: glyph.x,
                    y: run.line_y + glyph.y,
                    width: glyph.w,
                    height: 0.0,
                });
            }
        }
        
        let mut final_height = max_height.max(metrics.line_height);
        
        // Ensure trailing newlines create a reachable line
        if text.ends_with('\n') {
            let last_y = lines.last().map(|l| l.y).unwrap_or(metrics.font_size);
            let next_y = last_y + metrics.line_height;
            lines.push(crate::types::line::LineInfo { 
                x: 0.0, 
                y: next_y, 
                width: 0.0,
                start_cluster: text.len(),
            });
            final_height += metrics.line_height;
        }

        ShapedBuffer::new(shaped_glyphs, lines, max_width, final_height)
    }
}

impl Default for CosmicFontProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FontProvider for CosmicFontProvider {
    fn shape(&mut self, text: &str, options: &TextOptions) -> ShapedBuffer {
        let mut font_system = self.font_system.lock().unwrap();
        
        let metrics = Metrics::new(options.font_size, options.font_size * options.line_height);
        self.buffer.set_metrics(&mut font_system, metrics);

        self.buffer.set_text(
            &mut font_system,
            text,
            &options.as_attrs(),
            Shaping::Advanced,
            None,
        );

        if let Some(alignment) = options.align {
            let align: Align = alignment.into();
            for line in self.buffer.lines.iter_mut() {
                line.set_align(Some(align));
            }
        }

        self.buffer.shape_until_scroll(&mut font_system, false);

        // Pre-calculate paragraph start offsets
        let mut paragraph_offsets = Vec::new();
        let mut current_offset = 0;
        for line in &self.buffer.lines {
            paragraph_offsets.push(current_offset);
            current_offset += line.text().len() + 1; // +1 for the newline
        }

        let mut shaped_glyphs = Vec::new();
        let mut lines = Vec::new();
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        let layout_width = self.buffer.size().0.unwrap_or(0.0);

        let mut next_line_i = 0;
        let mut last_line_y = 0.0;

        for run in self.buffer.layout_runs() {
            // 1. Fill in gaps for empty logical lines before this run
            while next_line_i < run.line_i {
                let missed_y = if lines.is_empty() {
                    metrics.font_size
                } else {
                    last_line_y + metrics.line_height
                };
                
                let start_cluster = paragraph_offsets.get(next_line_i).cloned().unwrap_or(0);
                lines.push(crate::types::line::LineInfo {
                    x: 0.0,
                    y: missed_y,
                    width: 0.0,
                    start_cluster,
                });
                
                last_line_y = missed_y;
                next_line_i += 1;
            }

            // 2. Process this visual run
            let alignment_offset = match options.align {
                Some(crate::types::HorizontalAlignment::Center) => (layout_width - run.line_w) / 2.0,
                Some(crate::types::HorizontalAlignment::Right) => layout_width - run.line_w,
                _ => 0.0,
            };

            let paragraph_offset = paragraph_offsets.get(run.line_i).cloned().unwrap_or(0);
            lines.push(crate::types::line::LineInfo {
                x: alignment_offset,
                y: run.line_y,
                width: run.line_w,
                start_cluster: paragraph_offset + run.glyphs.first().map(|g| g.start).unwrap_or(0),
            });

            for glyph in run.glyphs {
                max_width = max_width.max(glyph.x + glyph.w);
                let physical = glyph.physical((0.0, 0.0), 1.0);
                
                shaped_glyphs.push(ShapedGlyph {
                    key: physical.cache_key,
                    cluster: paragraph_offset + glyph.start,
                    x: glyph.x,
                    y: run.line_y + glyph.y,
                    width: glyph.w,
                    height: 0.0,
                });
            }

            last_line_y = run.line_y;
            max_height = max_height.max(run.line_y + metrics.line_height - metrics.font_size);
            
            // Advance next_line_i up to current run index
            if next_line_i <= run.line_i {
                next_line_i = run.line_i + 1;
            }
        }

        // 3. Final cleanup for trailing empty lines
        while next_line_i < self.buffer.lines.len() {
             let missed_y = if lines.is_empty() {
                metrics.font_size
            } else {
                last_line_y + metrics.line_height
            };
            
            let start_cluster = paragraph_offsets.get(next_line_i).cloned().unwrap_or(0);
            lines.push(crate::types::line::LineInfo {
                x: 0.0,
                y: missed_y,
                width: 0.0,
                start_cluster,
            });
            
            last_line_y = missed_y;
            next_line_i += 1;
            max_height = max_height.max(last_line_y + metrics.line_height - metrics.font_size);
        }

        ShapedBuffer::new(shaped_glyphs, lines, max_width, max_height)
    }
    fn load_font(&mut self, data: Vec<u8>) {
        self.font_system.lock().unwrap().db_mut().load_font_data(data);
    }

    fn load_font_path(&mut self, path: &Path) -> std::io::Result<()> {
        self.font_system.lock().unwrap().db_mut().load_font_file(path)
    }

    fn metrics(&self, options: &TextOptions) -> FontMetrics {
        let font_size = options.font_size;
        let line_height = font_size * options.line_height;

        FontMetrics {
            ascent: font_size,
            descent: -(line_height - font_size),
            line_gap: 0.0,
        }
    }

    fn set_layout_size(&mut self, width: f32, height: f32) {
         self.buffer.set_size(&mut self.font_system.lock().unwrap(), Some(width), Some(height));
    }

    fn font_system(&self) -> Arc<Mutex<FontSystem>> {
        self.font_system.clone()
    }
}
