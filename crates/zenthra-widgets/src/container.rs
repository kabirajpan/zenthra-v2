use crate::ui::{RectDraw, Ui};
use zenthra_core::Color;
use zenthra_render::RectInstance;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
}

pub struct ContainerBuilder<'a> {
    ui: &'a mut Ui,
    width: Option<f32>,
    height: Option<f32>,
    fill_x: bool,
    fill_y: bool,
    direction: Direction,
    gap: f32,
    padding: f32,
    bg: Option<Color>,
    radius: f32,
    border_color: Option<Color>,
    border_width: f32,
    shadow_blur: f32,
    shadow_color: Option<Color>,
    opacity: f32,
    scroll_y: bool,
    clip: bool,
}

impl<'a> ContainerBuilder<'a> {
    pub fn new(ui: &'a mut Ui) -> Self {
        Self {
            ui,
            width: None,
            height: None,
            fill_x: false,
            fill_y: false,
            direction: Direction::Column,
            gap: 0.0,
            padding: 0.0,
            bg: None,
            radius: 0.0,
            border_color: None,
            border_width: 0.0,
            shadow_blur: 0.0,
            shadow_color: None,
            opacity: 1.0,
            scroll_y: false,
            clip: false,
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }
    pub fn fill_x(mut self) -> Self {
        self.fill_x = true;
        self
    }
    pub fn fill_y(mut self) -> Self {
        self.fill_y = true;
        self
    }
    pub fn fill(mut self) -> Self {
        self.fill_x = true;
        self.fill_y = true;
        self
    }
    pub fn center(mut self) -> Self {
        self.fill_x = true;
        self.fill_y = true;
        self
    }
    pub fn direction(mut self, d: Direction) -> Self {
        self.direction = d;
        self
    }
    pub fn gap(mut self, g: f32) -> Self {
        self.gap = g;
        self
    }
    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }
    pub fn bg(mut self, c: Color) -> Self {
        self.bg = Some(c);
        self
    }
    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }
    pub fn border(mut self, c: Color, w: f32) -> Self {
        self.border_color = Some(c);
        self.border_width = w;
        self
    }
    pub fn shadow(mut self, blur: f32) -> Self {
        self.shadow_blur = blur;
        self
    }
    pub fn shadow_color(mut self, c: Color) -> Self {
        self.shadow_color = Some(c);
        self
    }
    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = o;
        self
    }
    pub fn scroll_y(mut self) -> Self {
        self.scroll_y = true;
        self
    }
    pub fn clip(mut self) -> Self {
        self.clip = true;
        self
    }

    pub fn show(self) {
        let w = if self.fill_x {
            self.ui.width
        } else {
            self.width.unwrap_or(100.0)
        };
        let h = if self.fill_y {
            self.ui.height
        } else {
            self.height.unwrap_or(100.0)
        };

        if let Some(bg) = self.bg {
            let bc = self.border_color.unwrap_or(Color::TRANSPARENT);
            let sc = self.shadow_color.unwrap_or(Color::TRANSPARENT);

            self.ui.rect_draws.push(RectDraw {
                instance: RectInstance {
                    pos: [self.ui.cursor_x, self.ui.cursor_y],
                    size: [w, h],
                    color: bg.to_array(),
                    radius: self.radius,
                    border_width: self.border_width,
                    border_color: bc.to_array(),
                    shadow_color: sc.to_array(),
                    shadow_offset: [0.0, 0.0],
                    shadow_blur: self.shadow_blur,
                    clip_rect: [0.0, 0.0, self.ui.width, self.ui.height],
                    grayscale: 0.0,
                    brightness: 1.0,
                    opacity: self.opacity,
                },
            });
        }
    }
}
