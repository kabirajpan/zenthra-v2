use zenthra_text::prelude::*;
use zenthra_text::traits::FontProvider;
use crate::ui::{DrawCommand, TextDraw, Ui};
use zenthra_core::{Color, EdgeInsets};

pub struct TextBuilder<'a> {
    ui: &'a mut Ui,
    content: String,
    options: TextOptions,
    
    // Layout-specific fields (not in TextOptions)
    margin: EdgeInsets,
    bg_radius: f32, // Not currently in Zentype but used in Zenthra
    cursor: CursorIcon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorIcon {
    Default,
    Text,
    Pointer,
    Crosshair,
}

impl<'a> TextBuilder<'a> {
    pub fn new(ui: &'a mut Ui, content: &str) -> Self {
        let x = ui.cursor_x;
        let y = ui.cursor_y;
        let max_width = (ui.max_x - x).max(0.0);
        
        Self {
            ui,
            content: content.to_string(),
            options: TextOptions::new()
                .at(x, y)
                .max_width(max_width),
            margin: EdgeInsets::ZERO,
            bg_radius: 0.0,
            cursor: CursorIcon::Default,
        }
    }

    pub fn min_width(mut self, w: f32) -> Self {
        self.options = self.options.min_width(w);
        self
    }

    pub fn size(mut self, s: f32) -> Self {
        self.options = self.options.font_size(s);
        self
    }
    pub fn color(mut self, c: Color) -> Self {
        self.options = self.options.color(c);
        self
    }
    pub fn weight(mut self, w: FontWeight) -> Self {
        self.options = self.options.font_weight(w);
        self
    }
    pub fn bold(mut self) -> Self {
        self.options = self.options.font_weight(FontWeight::Bold);
        self
    }
    pub fn italic(mut self) -> Self {
        self.options = self.options.font_style(FontStyle::Italic);
        self
    }
    pub fn family(mut self, f: impl Into<String>) -> Self {
        self.options = self.options.font_family(f);
        self
    }
    pub fn monospace(mut self) -> Self {
        self.options = self.options.font_family("monospace");
        self
    }
    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.options = self.options.at(x, y);
        self
    }
    pub fn max_width(mut self, w: f32) -> Self {
        self.options = self.options.max_width(w);
        self
    }

    pub fn padding(mut self, p: impl Into<Padding>) -> Self {
        self.options = self.options.padding(p.into());
        self
    }
    pub fn padding_x(mut self, p: f32) -> Self {
        self.options = self.options.padding_horizontal(p);
        self
    }
    pub fn padding_y(mut self, p: f32) -> Self {
        self.options = self.options.padding_vertical(p);
        self
    }

    pub fn margin(mut self, m: f32) -> Self {
        self.margin = EdgeInsets::all(m);
        self
    }
    pub fn margin_x(mut self, m: f32) -> Self {
        self.margin.left = m;
        self.margin.right = m;
        self
    }
    pub fn margin_y(mut self, m: f32) -> Self {
        self.margin.top = m;
        self.margin.bottom = m;
        self
    }

    pub fn line_height(mut self, lh: f32) -> Self {
        self.options = self.options.line_height(lh);
        self
    }

    pub fn bg(mut self, c: Color) -> Self {
        self.options = self.options.bg(c);
        self
    }

    pub fn full_width_bg(mut self, b: bool) -> Self {
        self.options = self.options.full_width(b);
        self
    }
    
    pub fn cursor(mut self, c: CursorIcon) -> Self {
        self.cursor = c;
        self
    }
    pub fn cursor_text(mut self) -> Self {
        self.cursor = CursorIcon::Text;
        self
    }
    pub fn cursor_pointer(mut self) -> Self {
        self.cursor = CursorIcon::Pointer;
        self
    }

    pub fn show(mut self) -> Option<ShapedBuffer> {
        let horiz = self.margin.horizontal();
        let vert = self.margin.vertical();
        let (w, h, buffer, start) = self.draw_and_measure();
        self.ui.advance(w + horiz, h + vert, start);
        buffer
    }

    pub fn draw_and_measure(&mut self) -> (f32, f32, Option<ShapedBuffer>, usize) {
        let metrics = self.ui.font_system.as_ref().map(|fs| {
             let adapter = CosmicFontProvider::new_with_system(fs.clone());
             adapter.metrics(&self.options)
        });

        let (cw, ch, buffer) = if let Some(_metrics) = metrics {
             let mut adapter = CosmicFontProvider::new_with_system(self.ui.font_system.clone().unwrap());
             
             let layout_width = self.options.max_width.unwrap_or(self.ui.width - self.options.x) - self.options.padding.left - self.options.padding.right;
             adapter.set_layout_size(layout_width, self.ui.height);
             
             let buffer = adapter.shape(&self.content, &self.options);
             let (cw, ch) = buffer.content_size();
             
             let padding = self.options.padding;
             let mut w = cw + padding.left + padding.right;
             
             if let Some(min_w) = self.options.min_width {
                 if w < min_w {
                     w = min_w;
                 }
             }

             let h = ch + padding.top + padding.bottom;
             (w, h, Some(buffer))
        } else {
            (100.0, 20.0, None) // Fallback
        };

        let start_draw = self.ui.draws.len();
        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.content.clone().into(),
            pos: [self.options.x, self.options.y],
            options: self.options.clone(),
        }));

        (cw, ch, buffer, start_draw)
    }
}
