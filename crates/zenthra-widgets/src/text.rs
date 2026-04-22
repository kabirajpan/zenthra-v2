use crate::ui::{TextDraw, Ui};
use zenthra_core::Color;

pub struct TextBuilder<'a> {
    pub ui: &'a mut Ui,
    pub content: String,
    pub font_size: f32,
    pub color: Color,
    pub bold: bool,
    pub italic: bool,
    pub max_width: Option<f32>,
    pub x: f32,
    pub y: f32,
}

impl<'a> TextBuilder<'a> {
    pub fn new(ui: &'a mut Ui, content: &str) -> Self {
        let x = ui.cursor_x;
        let y = ui.cursor_y;
        Self {
            ui,
            content: content.to_string(),
            font_size: 16.0,
            color: Color::WHITE,
            bold: false,
            italic: false,
            max_width: None,
            x,
            y,
        }
    }

    pub fn size(mut self, s: f32) -> Self {
        self.font_size = s;
        self
    }
    pub fn color(mut self, c: Color) -> Self {
        self.color = c;
        self
    }
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn max_width(mut self, w: f32) -> Self {
        self.max_width = Some(w);
        self
    }
    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn show(self) {
        self.ui.text_draws.push(TextDraw {
            text: self.content,
            x: self.x,
            y: self.y,
            font_size: self.font_size,
            color: self.color,
            max_width: self.max_width,
            bold: self.bold,
            italic: self.italic,
        });
    }
}
