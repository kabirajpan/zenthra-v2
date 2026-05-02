// crates/zenthra-widgets/src/button.rs

use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
// use crate::text::TextBuilder;
use zenthra_core::{Color, EdgeInsets, Id, Rect, Role, SemanticNode};
use zenthra_render::RectInstance;

pub struct ButtonBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,
    label: String,

    // Position & Layout
    pos: Option<(f32, f32)>,
    width: Option<f32>,
    height: Option<f32>,
    padding: EdgeInsets,

    // Visuals (Idle)
    bg: Color,
    text_color: Color,
    radius: [f32; 4],
    font_size: f32,
    border_width: f32,
    border_color: Color,

    // Shadows
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_color: Color,
    shadow_opacity: f32,

    // States
    hover_bg: Option<Color>,
    active_bg: Option<Color>,

    // Other
    opacity: f32,
    render_mode: Option<zenthra_core::RenderMode>,
    hover_brightness: f32,
    hover_scale: f32,
    hover_border_color: Option<Color>,
    hover_border_width: Option<f32>,
    active_border_color: Option<Color>,
    active_border_width: Option<f32>,
}

impl<'u, 'a> ButtonBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, label: &str) -> Self {
        let id = ui.id();
        Self {
            ui,
            id,
            label: label.to_string(),
            pos: None,
            width: None,
            height: None,
            padding: EdgeInsets::symmetric(6.0, 12.0),
            bg: Color::rgb(0.2, 0.2, 0.25),
            text_color: Color::WHITE,
            radius: [4.0; 4],
            font_size: 14.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: [0.0, 0.0],
            shadow_blur: 0.0,
            shadow_color: Color::TRANSPARENT,
            shadow_opacity: 0.0,
            hover_bg: None,
            active_bg: None,
            opacity: 1.0,
            render_mode: None,
            hover_brightness: 1.0,
            hover_scale: 1.0,
            hover_border_color: None,
            hover_border_width: None,
            active_border_color: None,
            active_border_width: None,
        }
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.pos = Some((x, y));
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

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn hover_bg(mut self, color: Color) -> Self {
        self.hover_bg = Some(color);
        self
    }

    pub fn active_bg(mut self, color: Color) -> Self {
        self.active_bg = Some(color);
        self
    }

    pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.radius = [tl, tr, br, bl];
        self
    }

    pub fn radius_all(mut self, r: f32) -> Self {
        self.radius = [r, r, r, r];
        self
    }

    pub fn radius_top(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self.radius[1] = r;
        self
    }

    pub fn radius_bottom(mut self, r: f32) -> Self {
        self.radius[2] = r;
        self.radius[3] = r;
        self
    }

    pub fn radius_top_left(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self
    }

    pub fn radius_top_right(mut self, r: f32) -> Self {
        self.radius[1] = r;
        self
    }

    pub fn radius_bottom_right(mut self, r: f32) -> Self {
        self.radius[2] = r;
        self
    }

    pub fn radius_bottom_left(mut self, r: f32) -> Self {
        self.radius[3] = r;
        self
    }

    pub fn radius_left(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self.radius[3] = r;
        self
    }

    pub fn radius_right(mut self, r: f32) -> Self {
        self.radius[1] = r;
        self.radius[2] = r;
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = color;
        self.border_width = width;
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

    pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.padding = zenthra_core::EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }

    pub fn padding_x(mut self, x: f32) -> Self {
        self.padding.left = x;
        self.padding.right = x;
        self
    }

    pub fn padding_y(mut self, y: f32) -> Self {
        self.padding.top = y;
        self.padding.bottom = y;
        self
    }

    pub fn padding_top(mut self, t: f32) -> Self {
        self.padding.top = t;
        self
    }

    pub fn padding_bottom(mut self, b: f32) -> Self {
        self.padding.bottom = b;
        self
    }

    pub fn padding_left(mut self, l: f32) -> Self {
        self.padding.left = l;
        self
    }

    pub fn padding_right(mut self, r: f32) -> Self {
        self.padding.right = r;
        self
    }

    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = o;
        self
    }

    pub fn hover_brightness(mut self, b: f32) -> Self {
        self.hover_brightness = b;
        self
    }

    pub fn hover_scale(mut self, s: f32) -> Self {
        self.hover_scale = s;
        self
    }

    pub fn hover_border(mut self, color: Color, weight: f32) -> Self {
        self.hover_border_color = Some(color);
        self.hover_border_width = Some(weight);
        self
    }

    pub fn active_border(mut self, color: Color, weight: f32) -> Self {
        self.active_border_color = Some(color);
        self.active_border_width = Some(weight);
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn render_mode(mut self, mode: zenthra_core::RenderMode) -> Self {
        self.render_mode = Some(mode);
        self
    }

    pub fn show(self) -> zenthra_core::Response {
        if let Some(mode) = self.render_mode {
            self.ui.render_mode_stack.push(mode);
        }

        let (x, y) = self.pos.unwrap_or((self.ui.cursor_x, self.ui.cursor_y));

        let mut clicked = false;
        let ox = x + self.ui.offset_x;
        let oy = y + self.ui.offset_y;

        let is_hovered = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            self.ui.mouse_in_rect(
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                rect.size.width,
                rect.size.height,
            )
        } else {
            self.ui.mouse_in_rect(
                ox,
                oy,
                self.width.unwrap_or(100.0),
                self.height.unwrap_or(40.0),
            )
        };

        let is_pressed = is_hovered && self.ui.mouse_down;

        if self.ui.clicked && is_hovered {
            clicked = true;
        }

        // Determine effective colors based on state
        let mut current_bg = self.bg;
        let current_text = self.text_color;
        let mut current_brightness = 1.0;

        if is_pressed {
            current_bg = self.active_bg.unwrap_or(self.bg);
            current_brightness = 0.8;
        } else if is_hovered {
            current_bg = self.hover_bg.unwrap_or(self.bg);
            current_brightness = self.hover_brightness;
        }

        let mut final_border_w = self.border_width;
        let mut final_border_c = self.border_color;

        if is_pressed {
            if let Some(c) = self.active_border_color { final_border_c = c; }
            if let Some(w) = self.active_border_width { final_border_w = w; }
        } else if is_hovered {
            if let Some(c) = self.hover_border_color { final_border_c = c; }
            if let Some(w) = self.hover_border_width { final_border_w = w; }
        }

        // Measure Text
        let mut text_w = 0.0;
        let mut text_h = 0.0;
        if let Some(fs) = self.ui.font_system.as_ref() {
            // use zenthra_text::traits::FontProvider;
            let mut adapter =
                zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let options = zenthra_text::prelude::TextOptions::new().font_size(self.font_size);
            let buffer = adapter.shape(&self.label, &options);
            let (cw, ch) = buffer.content_size();
            text_w = cw;
            text_h = ch;
        }

        let mut final_w = self.width.unwrap_or(text_w + self.padding.horizontal());
        let mut final_h = self.height.unwrap_or(text_h + self.padding.vertical());

        let final_border_w = final_border_w;
        let final_border_c = final_border_c;

        if is_hovered {
            final_w *= self.hover_scale;
            final_h *= self.hover_scale;
        }

        let start_draw = self.ui.draws.len();

        // 1. Draw Background
        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [x, y],
                size: [final_w, final_h],
                color: current_bg.to_array(),
                radius: [
                    self.radius[3],
                    self.radius[2],
                    self.radius[1],
                    self.radius[0],
                ],
                border_width: final_border_w,
                border_color: final_border_c.to_array(),
                shadow_color: {
                    let mut c = self.shadow_color;
                    c.a *= self.shadow_opacity;
                    c.to_array()
                },
                shadow_offset: self.shadow_offset,
                shadow_blur: self.shadow_blur,
                clip_rect: [0.0, 0.0, 9999.0, 9999.0],
                grayscale: 0.0,
                brightness: current_brightness,
                opacity: self.opacity,
                ..Default::default()
            },
        }));

        // 2. Draw Text (Centered)
        let tx = x + (final_w - text_w) / 2.0;
        let ty = y + (final_h - text_h) / 2.0;

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [tx, ty],
            options: zenthra_text::prelude::TextOptions::new()
                .font_size(self.font_size)
                .color(current_text)
                .at(tx, ty),
            clip: [0.0, 0.0, 9999.0, 9999.0],
        }));

        // 3. Register Semantic
        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Button, Rect::new(x, y, final_w, final_h))
                .with_label(self.label.clone()),
        );

        self.ui
            .record_layout(self.id, Rect::new(x, y, final_w, final_h));
        self.ui.advance(final_w, final_h, start_draw);

        if self.render_mode.is_some() {
            self.ui.render_mode_stack.pop();
        }

        zenthra_core::Response {
            clicked,
            hovered: is_hovered,
            pressed: is_pressed,
        }
    }
}
