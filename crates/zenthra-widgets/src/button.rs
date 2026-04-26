// crates/zenthra-widgets/src/button.rs

use crate::ui::{Ui, DrawCommand, RectDraw, TextDraw};
// use crate::text::TextBuilder;
use zenthra_core::{Color, Id, Role, SemanticNode, Rect, EdgeInsets};
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
    stroke_weight: f32,
    stroke_color: Color,
    
    // Shadows
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_color: Color,
    
    // States
    hover_bg: Option<Color>,
    active_bg: Option<Color>,
    
    // Other
    opacity: f32,
    render_mode: Option<zenthra_core::RenderMode>,
    // on_click: Option<Box<dyn FnMut()>>,
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
            padding: EdgeInsets::all(12.0),
            bg: Color::rgb(0.2, 0.2, 0.25),
            text_color: Color::WHITE,
            radius: [4.0; 4],
            font_size: 18.0,
            stroke_weight: 0.0,
            stroke_color: Color::TRANSPARENT,
            shadow_offset: [0.0, 2.0],
            shadow_blur: 4.0,
            shadow_color: Color::rgba(0.0, 0.0, 0.0, 0.3),
            hover_bg: Some(Color::rgb(0.25, 0.25, 0.35)),
            active_bg: Some(Color::rgb(0.15, 0.15, 0.2)),
            opacity: 1.0,
            render_mode: None,
            // on_click: None,
        }
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

    pub fn stroke(mut self, color: Color, weight: f32) -> Self {
        self.stroke_color = color;
        self.stroke_weight = weight;
        self
    }

    pub fn shadow(mut self, color: Color, offset: [f32; 2], blur: f32) -> Self {
        self.shadow_color = color;
        self.shadow_offset = offset;
        self.shadow_blur = blur;
        self
    }

    pub fn padding(mut self, padding: impl Into<EdgeInsets>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = o;
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
        let is_hovered = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            self.ui.mouse_in_rect(rect.origin.x, rect.origin.y, rect.size.width, rect.size.height)
        } else {
            self.ui.mouse_in_rect(x, y, self.width.unwrap_or(100.0), self.height.unwrap_or(40.0))
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
            current_brightness = 1.1;
        }

        // Measure Text
        let mut text_w = 0.0;
        let mut text_h = 0.0;
        if let Some(fs) = self.ui.font_system.as_ref() {
            // use zenthra_text::traits::FontProvider;
            let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let options = zenthra_text::prelude::TextOptions::new().font_size(self.font_size);
            let buffer = adapter.shape(&self.label, &options);
            let (cw, ch) = buffer.content_size();
            text_w = cw;
            text_h = ch;
        }

        let final_w = self.width.unwrap_or(text_w + self.padding.horizontal());
        let final_h = self.height.unwrap_or(text_h + self.padding.vertical());

        let start_draw = self.ui.draws.len();

        // 1. Draw Background
        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [x, y],
                size: [final_w, final_h],
                color: current_bg.to_array(),
                radius: self.radius,
                border_width: self.stroke_weight,
                border_color: self.stroke_color.to_array(),
                shadow_color: self.shadow_color.to_array(),
                shadow_offset: self.shadow_offset,
                shadow_blur: self.shadow_blur,
                clip_rect: [0.0, 0.0, 9999.0, 9999.0],
                grayscale: 0.0,
                brightness: current_brightness,
                opacity: self.opacity,
            }
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
            clip: [x, y, final_w, final_h],
        }));

        // 3. Register Semantic
        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Button, Rect::new(x, y, final_w, final_h))
                .with_label(self.label.clone())
        );

        self.ui.record_layout(self.id, Rect::new(x, y, final_w, final_h));
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
