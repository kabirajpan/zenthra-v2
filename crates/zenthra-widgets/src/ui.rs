use crate::container::ContainerBuilder;
use crate::text::{CursorIcon, TextBuilder};
use zenthra_core::Color;
use zenthra_render::RectInstance;
use zenthra_text::shaper::TextFamily;

pub struct TextDraw {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub font_size: f32,
    pub color: Color,
    pub max_width: Option<f32>,
    pub weight: u16,
    pub italic: bool,
    pub family: TextFamily,
    // bg
    pub bg: Option<Color>,
    pub bg_radius: f32,
    pub padding_top: f32,
    pub padding_bottom: f32,
    pub padding_left: f32,
    pub padding_right: f32,
}

pub struct RectDraw {
    pub instance: RectInstance,
}

pub enum DrawCommand {
    Rect(RectDraw),
    Text(TextDraw),
}

pub struct Ui {
    pub width: f32,
    pub height: f32,
    pub scale_factor: f32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_down: bool,
    pub cursor_icon: CursorIcon,
    pub draws: Vec<DrawCommand>,
    pub cursor_x: f32,
    pub cursor_y: f32,
}

impl Ui {
    pub fn new(width: u32, height: u32, scale_factor: f64) -> Self {
        Self {
            width: width as f32,
            height: height as f32,
            scale_factor: scale_factor as f32,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_down: false,
            cursor_icon: CursorIcon::Default,
            draws: Vec::new(),
            cursor_x: 0.0,
            cursor_y: 0.0,
        }
    }

    pub fn set_mouse(&mut self, x: f32, y: f32, down: bool) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.mouse_down = down;
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
