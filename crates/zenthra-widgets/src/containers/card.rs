// crates/zenthra-widgets/src/containers/card.rs

use crate::ui::Ui;
use zenthra_core::{Color, Id};

pub struct CardBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,

    // Styling
    width: Option<f32>,
    height: Option<f32>,
    padding: f32,
    bg: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,
    
    // Shadows
    shadow_color: Color,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,

    // Premium Animations
    hover_scale: f32,
    hover_bg: Option<Color>,
    hover_border_color: Option<Color>,
}

impl<'u, 'a> CardBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        let id = ui.id();
        Self {
            ui,
            id,
            width: None,
            height: None,
            padding: 16.0,
            bg: Color::rgb(0.13, 0.13, 0.16),
            border_color: Color::rgb(0.22, 0.22, 0.26),
            border_width: 1.0,
            radius: 8.0,
            shadow_color: Color::rgba(0.0, 0.0, 0.0, 0.5),
            shadow_offset: [0.0, 4.0],
            shadow_blur: 12.0,
            shadow_opacity: 0.35,
            hover_scale: 1.0,
            hover_bg: None,
            hover_border_color: None,
        }
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = Some(w);
        self.height = Some(h);
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = color;
        self.border_width = width;
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }

    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.shadow_color = color;
        self.shadow_offset = [x, y];
        self.shadow_blur = blur;
        self
    }

    pub fn shadow_opacity(mut self, opacity: f32) -> Self {
        self.shadow_opacity = opacity;
        self
    }

    pub fn hover_scale(mut self, scale: f32) -> Self {
        self.hover_scale = scale;
        self
    }

    pub fn hover_bg(mut self, bg: Color) -> Self {
        self.hover_bg = Some(bg);
        self
    }

    pub fn hover_border_color(mut self, color: Color) -> Self {
        self.hover_border_color = Some(color);
        self
    }

    pub fn show<F>(self, f: F)
    where F: FnOnce(&mut Ui) {
        let mut container = self.ui.container()
            .id(self.id)
            .column()
            .padding_all(self.padding)
            .bg(self.bg)
            .border(self.border_color, self.border_width)
            .radius_all(self.radius)
            .shadow(self.shadow_color, self.shadow_offset[0], self.shadow_offset[1], self.shadow_blur)
            .shadow_opacity(self.shadow_opacity);

        if let Some(w) = self.width {
            container = container.width(w);
        }
        if let Some(h) = self.height {
            container = container.height(h);
        }
        if let Some(hb) = self.hover_bg {
            container = container.hover_bg(hb);
        }
        if let Some(hbc) = self.hover_border_color {
            container = container.hover_border(hbc, self.border_width);
        }
        if self.hover_scale != 1.0 {
            container = container.hover_scale(self.hover_scale);
        }

        container.show(|ui: &mut Ui| {
            f(ui);
        });
    }
}
