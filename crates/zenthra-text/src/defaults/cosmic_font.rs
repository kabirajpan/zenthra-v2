use std::path::Path;
use std::sync::{Arc, Mutex};
use cosmic_text::{Buffer, FontSystem, Metrics, Shaping};
use crate::primitives::shaped_buffer::ShapedBuffer;
use crate::traits::{FontProvider, FontMetrics};
use crate::types::options::TextOptions;
use crate::types::shaped_glyph::ShapedGlyph;

pub struct CosmicFontProvider {
    pub font_system: Arc<Mutex<FontSystem>>,
    pub buffer: Buffer,
}

impl CosmicFontProvider {
    pub fn new() -> Self {
        let font_system = Arc::new(Mutex::new(FontSystem::new()));
        let buffer = Buffer::new(&mut font_system.lock().unwrap(), Metrics::new(16.0, 20.0));
        Self { font_system, buffer }
    }

    pub fn new_with_system(font_system: Arc<Mutex<FontSystem>>) -> Self {
        let buffer = Buffer::new(&mut font_system.lock().unwrap(), Metrics::new(16.0, 20.0));
        Self { font_system, buffer }
    }

    pub fn set_layout_size(&mut self, width: f32, height: f32) {
        self.buffer.set_size(&mut self.font_system.lock().unwrap(), Some(width), Some(height));
    }

    pub fn shape(&mut self, text: &str, options: &TextOptions) -> ShapedBuffer {
        let mut fs = self.font_system.lock().unwrap();
        
        // Apply metrics and wrap mode from options
        let font_size = options.font_size;
        let line_height = font_size * options.line_height;
        self.buffer.set_metrics(&mut fs, Metrics::new(font_size, line_height));
        
        let wrap = match options.wrap {
            crate::types::options::TextWrap::Word => cosmic_text::Wrap::Word,
            crate::types::options::TextWrap::Character => cosmic_text::Wrap::Glyph,
            crate::types::options::TextWrap::None => cosmic_text::Wrap::None,
        };
        self.buffer.set_wrap(&mut fs, wrap);

        // If max_width is specified in options, override current buffer width
        if let Some(mw) = options.max_width {
            let current_h = self.buffer.size().1.unwrap_or(10000.0);
            self.buffer.set_size(&mut fs, Some(mw), Some(current_h));
        }

        self.buffer.set_text(&mut fs, text, &options.as_attrs(), Shaping::Advanced, None);
        
        // Apply alignment before final layout
        if let Some(alignment) = options.align {
            let align = match alignment {
                crate::types::options::HorizontalAlignment::Left => cosmic_text::Align::Left,
                crate::types::options::HorizontalAlignment::Center => cosmic_text::Align::Center,
                crate::types::options::HorizontalAlignment::Right => cosmic_text::Align::Right,
                crate::types::options::HorizontalAlignment::Justified => cosmic_text::Align::Justified,
            };
            for line in self.buffer.lines.iter_mut() {
                line.set_align(Some(align));
            }
        }

        self.buffer.shape_until_scroll(&mut fs, false);

        let metrics = self.buffer.metrics();
        
        // Use the natural baseline of the first run as our anchor. 
        // This keeps single-line Inputs "perfect".
        let mut anchor_y = metrics.font_size as f32;
        if let Some(first_run) = self.buffer.layout_runs().next() {
            anchor_y = first_run.line_y - (first_run.line_i as f32 * metrics.line_height as f32);
        }

        let mut paragraph_offsets = Vec::new();
        let mut current_offset = 0;
        for line in &self.buffer.lines {
            paragraph_offsets.push(current_offset);
            current_offset += line.text().len() + 1;
        }

        let mut shaped_glyphs = Vec::new();
        let mut lines = Vec::new();
        let mut max_width: f32 = 0.0;
        let mut next_paragraph_i = 0;
        let mut visual_line_i = 0;

        for run in self.buffer.layout_runs() {
            // Fill gaps for empty paragraphs BEFORE the current run's paragraph
            while next_paragraph_i < run.line_i {
                let start_cluster = paragraph_offsets.get(next_paragraph_i).cloned().unwrap_or(0);
                let grid_y = anchor_y + (visual_line_i as f32 * metrics.line_height as f32);

                lines.push(crate::types::line::LineInfo {
                    x: 0.0,
                    y: grid_y,
                    width: 0.0,
                    start_cluster,
                });
                next_paragraph_i += 1;
                visual_line_i += 1;
            }

            // Real visual line (could be the start of a paragraph or a wrapped segment)
            let paragraph_offset = paragraph_offsets.get(run.line_i).cloned().unwrap_or(0);
            let start_cluster = paragraph_offset + run.glyphs.first().map(|g| g.start).unwrap_or(0);
            let grid_y = anchor_y + (visual_line_i as f32 * metrics.line_height as f32);

            lines.push(crate::types::line::LineInfo {
                x: 0.0,
                y: grid_y,
                width: run.line_w,
                start_cluster,
            });

            for glyph in run.glyphs {
                max_width = max_width.max(glyph.x + glyph.w);
                let physical = glyph.physical((0.0, 0.0), 1.0);
                shaped_glyphs.push(ShapedGlyph {
                    key: physical.cache_key,
                    cluster: paragraph_offset + glyph.start,
                    x: glyph.x,
                    // Force the glyph to sit on our absolute grid baseline
                    y: grid_y + glyph.y, 
                    width: glyph.w,
                    height: 0.0,
                });
            }

            visual_line_i += 1;
            if next_paragraph_i <= run.line_i {
                next_paragraph_i = run.line_i + 1;
            }
        }

        // Fill trailing gaps (Empty lines at the end)
        while next_paragraph_i < self.buffer.lines.len() {
            let grid_y = anchor_y + (visual_line_i as f32 * metrics.line_height as f32);
            let start_cluster = paragraph_offsets.get(next_paragraph_i).cloned().unwrap_or(0);
            lines.push(crate::types::line::LineInfo {
                x: 0.0,
                y: grid_y,
                width: 0.0,
                start_cluster,
            });
            next_paragraph_i += 1;
            visual_line_i += 1;
        }

        let max_height = lines.len() as f32 * metrics.line_height as f32;
        ShapedBuffer::new(shaped_glyphs, lines, max_width, max_height)
    }

    pub fn load_font(&mut self, data: Vec<u8>) {
        self.font_system.lock().unwrap().db_mut().load_font_data(data);
    }

    pub fn load_font_path(&mut self, path: &Path) -> std::io::Result<()> {
        self.font_system.lock().unwrap().db_mut().load_font_file(path)
    }
}

impl FontProvider for CosmicFontProvider {
    fn shape(&mut self, text: &str, options: &TextOptions) -> ShapedBuffer {
        self.shape(text, options)
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

    fn load_font(&mut self, data: Vec<u8>) {
        self.load_font(data);
    }

    fn load_font_path(&mut self, path: &Path) -> std::io::Result<()> {
        self.load_font_path(path)
    }

    fn set_layout_size(&mut self, width: f32, height: f32) {
        self.set_layout_size(width, height);
    }

    fn font_system(&self) -> Arc<Mutex<FontSystem>> {
        self.font_system.clone()
    }
}
