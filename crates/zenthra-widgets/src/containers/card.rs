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
    bg: Option<Color>,
    border_color: Option<Color>,
    border_width: f32,
    radius: f32,
    
    // Shadows
    shadow_color: Option<Color>,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,

    // Premium Animations
    hover_scale: f32,
    hover_bg: Option<Color>,
    hover_border_color: Option<Color>,
    
    // Backdrop Filter
    backdrop_filter: Option<zenthra_core::BackdropFilter>,
    opacity: Option<f32>,
    bg_opacity: Option<f32>,
}

impl<'u, 'a> CardBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        let id = ui.id();
        Self {
            ui,
            id,
            width: None,
            height: None,
            padding: 0.0,
            bg: None,
            border_color: None,
            border_width: 0.0,
            radius: 0.0,
            shadow_color: None,
            shadow_offset: [0.0, 0.0],
            shadow_blur: 0.0,
            shadow_opacity: 0.0,
            hover_scale: 1.0,
            hover_bg: None,
            hover_border_color: None,
            backdrop_filter: None,
            opacity: None,
            bg_opacity: None,
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
        self.bg = Some(bg);
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }

    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.shadow_color = Some(color);
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

    pub fn backdrop_filter(mut self, filter: zenthra_core::BackdropFilter) -> Self {
        self.backdrop_filter = Some(filter);
        self
    }

    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = Some(o);
        self
    }

    pub fn bg_opacity(mut self, o: f32) -> Self {
        self.bg_opacity = Some(o);
        self
    }

    pub fn show<F>(self, f: F)
    where F: FnOnce(&mut Ui) {
        let mut container = self.ui.container()
            .id(self.id)
            .column()
            .padding_all(self.padding);

        if let Some(bg) = self.bg {
            container = container.bg(bg);
        }
        if let Some(bgo) = self.bg_opacity {
            container = container.bg_opacity(bgo);
        }
        if let Some(o) = self.opacity {
            container = container.opacity(o);
        }
        if let Some(bc) = self.border_color {
            container = container.border(bc, self.border_width);
        }
        if self.radius != 0.0 {
            container = container.radius_all(self.radius);
        }
        if let Some(sc) = self.shadow_color {
            container = container.shadow(sc, self.shadow_offset[0], self.shadow_offset[1], self.shadow_blur)
                .shadow_opacity(self.shadow_opacity);
        }
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
        if let Some(filter) = self.backdrop_filter {
            container = container.backdrop_filter(filter);
        }

        container.show(|ui: &mut Ui| {
            f(ui);
        });
    }
}
