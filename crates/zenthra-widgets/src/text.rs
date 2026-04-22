use crate::ui::{DrawCommand, TextDraw, Ui};
use zenthra_core::Color;
use zenthra_text::shaper::TextFamily;

pub struct TextBuilder<'a> {
    ui: &'a mut Ui,
    content: String,
    font_size: f32,
    color: Color,
    weight: u16,
    italic: bool,
    family: TextFamily,
    x: f32,
    y: f32,
    max_width: Option<f32>,
    padding_top: f32,
    padding_bottom: f32,
    padding_left: f32,
    padding_right: f32,
    margin_top: f32,
    margin_bottom: f32,
    margin_left: f32,
    margin_right: f32,
    bg: Option<Color>,
    bg_radius: f32,
    hover_bg: Option<Color>,
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
        Self {
            ui,
            content: content.to_string(),
            font_size: 16.0,
            color: Color::WHITE,
            weight: 400,
            italic: false,
            family: TextFamily::SansSerif,
            x,
            y,
            max_width: None,
            padding_top: 0.0,
            padding_bottom: 0.0,
            padding_left: 0.0,
            padding_right: 0.0,
            margin_top: 0.0,
            margin_bottom: 0.0,
            margin_left: 0.0,
            margin_right: 0.0,
            bg: None,
            bg_radius: 0.0,
            hover_bg: None,
            cursor: CursorIcon::Default,
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
    pub fn weight(mut self, w: u16) -> Self {
        self.weight = w;
        self
    }
    pub fn bold(mut self) -> Self {
        self.weight = 700;
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn family(mut self, f: TextFamily) -> Self {
        self.family = f;
        self
    }
    pub fn monospace(mut self) -> Self {
        self.family = TextFamily::Monospace;
        self
    }
    pub fn serif(mut self) -> Self {
        self.family = TextFamily::Serif;
        self
    }
    pub fn font(mut self, name: impl Into<String>) -> Self {
        self.family = TextFamily::Named(name.into());
        self
    }
    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn max_width(mut self, w: f32) -> Self {
        self.max_width = Some(w);
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding_top = p;
        self.padding_bottom = p;
        self.padding_left = p;
        self.padding_right = p;
        self
    }
    pub fn padding_x(mut self, p: f32) -> Self {
        self.padding_left = p;
        self.padding_right = p;
        self
    }
    pub fn padding_y(mut self, p: f32) -> Self {
        self.padding_top = p;
        self.padding_bottom = p;
        self
    }
    pub fn padding_top(mut self, p: f32) -> Self {
        self.padding_top = p;
        self
    }
    pub fn padding_bottom(mut self, p: f32) -> Self {
        self.padding_bottom = p;
        self
    }
    pub fn padding_left(mut self, p: f32) -> Self {
        self.padding_left = p;
        self
    }
    pub fn padding_right(mut self, p: f32) -> Self {
        self.padding_right = p;
        self
    }

    pub fn margin(mut self, m: f32) -> Self {
        self.margin_top = m;
        self.margin_bottom = m;
        self.margin_left = m;
        self.margin_right = m;
        self
    }
    pub fn margin_x(mut self, m: f32) -> Self {
        self.margin_left = m;
        self.margin_right = m;
        self
    }
    pub fn margin_y(mut self, m: f32) -> Self {
        self.margin_top = m;
        self.margin_bottom = m;
        self
    }
    pub fn margin_top(mut self, m: f32) -> Self {
        self.margin_top = m;
        self
    }
    pub fn margin_bottom(mut self, m: f32) -> Self {
        self.margin_bottom = m;
        self
    }
    pub fn margin_left(mut self, m: f32) -> Self {
        self.margin_left = m;
        self
    }
    pub fn margin_right(mut self, m: f32) -> Self {
        self.margin_right = m;
        self
    }

    pub fn bg(mut self, c: Color) -> Self {
        self.bg = Some(c);
        self
    }
    pub fn bg_radius(mut self, r: f32) -> Self {
        self.bg_radius = r;
        self
    }
    pub fn hover_bg(mut self, c: Color) -> Self {
        self.hover_bg = Some(c);
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

    pub fn show(self) {
        let draw_x = self.x;
        let draw_y = self.y;

        let est_w = self.content.len() as f32 * self.font_size * 0.6
            + self.padding_left
            + self.padding_right;
        let est_h = self.font_size * 1.4 + self.padding_top + self.padding_bottom;

        let active_bg = self.bg.or(self.hover_bg.filter(|_| {
            let mx = self.ui.mouse_x;
            let my = self.ui.mouse_y;
            mx >= self.x && mx <= self.x + est_w && my >= self.y && my <= self.y + est_h
        }));

        // record draw start BEFORE pushing
        let draw_start = self.ui.draws.len();

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.content,
            x: draw_x,
            y: draw_y,
            font_size: self.font_size,
            color: self.color,
            max_width: self.max_width,
            weight: self.weight,
            italic: self.italic,
            family: self.family,
            bg: active_bg,
            bg_radius: self.bg_radius,
            padding_top: self.padding_top,
            padding_bottom: self.padding_bottom,
            padding_left: self.padding_left,
            padding_right: self.padding_right,
        }));

        self.ui.advance(est_w, est_h, draw_start);
    }
}
