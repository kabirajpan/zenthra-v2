use crate::container::ContainerBuilder;
use crate::text::TextBuilder;
use zenthra_core::Color;
use zenthra_render::RectInstance;

/// A queued text draw call.
pub struct TextDraw {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub font_size: f32,
    pub color: Color,
    pub max_width: Option<f32>,
    pub bold: bool,
    pub italic: bool,
}

/// A queued rect draw call.
pub struct RectDraw {
    pub instance: RectInstance,
}

/// The context the user calls widget methods on every frame.
pub struct Ui {
    pub width: f32,
    pub height: f32,
    pub scale_factor: f32,

    // Draw queues — collected during ui building, flushed by renderer
    pub text_draws: Vec<TextDraw>,
    pub rect_draws: Vec<RectDraw>,

    // Cursor — tracks current draw position
    pub cursor_x: f32,
    pub cursor_y: f32,
}

impl Ui {
    pub fn new(width: u32, height: u32, scale_factor: f64) -> Self {
        Self {
            width: width as f32,
            height: height as f32,
            scale_factor: scale_factor as f32,
            text_draws: Vec::new(),
            rect_draws: Vec::new(),
            cursor_x: 0.0,
            cursor_y: 0.0,
        }
    }

    pub fn text<'a>(&'a mut self, content: &'a str) -> TextBuilder<'a> {
        TextBuilder::new(self, content)
    }

    pub fn container<'a, F>(&'a mut self, f: F) -> ContainerBuilder<'a>
    where
        F: FnOnce(&mut Ui),
    {
        f(self);
        ContainerBuilder::new(self)
    }
}
