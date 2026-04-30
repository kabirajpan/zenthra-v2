// crates/zenthra-widgets/src/controls/checkbox.rs

use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{Color, Id, Rect, Response, Role, SemanticNode};
use zenthra_render::RectInstance;

pub struct CheckboxBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    value: &'b mut bool,
    label: String,
    id: Id,

    // Layout
    size: f32,
    radius: f32,
    gap: f32,

    // Visuals (Unchecked)
    bg: Color,
    stroke_color: Color,
    stroke_weight: f32,

    // Visuals (Checked)
    check_bg: Color,
    check_color: Color,

    // Label
    label_size: f32,
    label_color: Color,

    // Interaction
    hover_bg: Option<Color>,
    active_bg: Option<Color>,
}

impl<'u, 'a, 'b> CheckboxBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, value: &'b mut bool, label: &str) -> Self {
        let id = ui.id();

        Self {
            ui,
            value,
            label: label.to_string(),
            id,

            // Default Layout
            size: 18.0,
            radius: 4.0,
            gap: 8.0,

            // Default Visuals
            bg: Color::rgb(0.15, 0.15, 0.15),
            stroke_color: Color::rgb(0.3, 0.3, 0.3),
            stroke_weight: 1.0,

            check_bg: Color::rgb(0.2, 0.5, 1.0),
            check_color: Color::WHITE,

            label_size: 14.0,
            label_color: Color::WHITE,

            hover_bg: None,
            active_bg: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    pub fn check_bg(mut self, color: Color) -> Self {
        self.check_bg = color;
        self
    }

    pub fn check_color(mut self, color: Color) -> Self {
        self.check_color = color;
        self
    }

    pub fn label_size(mut self, size: f32) -> Self {
        self.label_size = size;
        self
    }

    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = color;
        self
    }

    pub fn stroke(mut self, color: Color, weight: f32) -> Self {
        self.stroke_color = color;
        self.stroke_weight = weight;
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

    pub fn show(self) -> Response {
        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);

        // 1. Measure Text
        let mut text_w = 0.0;
        let mut text_h = 0.0;
        if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let options = zenthra_text::prelude::TextOptions::new().font_size(self.label_size);
            let buffer = adapter.shape(&self.label, &options);
            let (cw, ch) = buffer.content_size();
            text_w = cw;
            text_h = ch;
        }

        let total_w = self.size + self.gap + text_w;
        let total_h = self.size.max(text_h);

        // 2. Hit-testing (Use recorded layout for accurate positioning in containers)
        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                if rect.size.width > 0.0 { rect.size.width } else { total_w },
                if rect.size.height > 0.0 { rect.size.height } else { total_h }
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, total_w, total_h)
        };

        let is_hovered = self.ui.mouse_in_rect(actual_ox, actual_oy, actual_w, actual_h);
        let is_pressed = is_hovered && self.ui.mouse_down;
        let mut clicked = false;

        if self.ui.clicked && is_hovered {
            *self.value = !*self.value;
            self.ui.needs_redraw = true;
            clicked = true;
        }

        // 3. Determine Colors
        let mut current_bg = if *self.value { self.check_bg } else { self.bg };
        let mut brightness = 1.0;

        if is_pressed {
            if let Some(c) = self.active_bg { current_bg = c; }
            else { brightness = 0.8; }
        } else if is_hovered {
            if let Some(c) = self.hover_bg { current_bg = c; }
            else { brightness = 1.1; }
        }

        let start_draw = self.ui.draws.len();

        // 4. Draw Box
        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [x, y + (total_h - self.size) / 2.0],
                size: [self.size, self.size],
                color: current_bg.to_array(),
                radius: [self.radius; 4],
                border_width: self.stroke_weight,
                border_color: self.stroke_color.to_array(),
                brightness,
                ..Default::default()
            },
        }));

        // 5. Draw Check Indicator (Unicode Checkmark)
        if *self.value {
            let check_str = "✓";
            let mut check_w = 0.0;
            let mut check_h = 0.0;
            
            if let Some(fs) = self.ui.font_system.as_ref() {
                let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
                // Scale the checkmark slightly smaller than the box
                let check_font_size = self.size * 0.8;
                let options = zenthra_text::prelude::TextOptions::new().font_size(check_font_size);
                let buffer = adapter.shape(check_str, &options);
                let (cw, ch) = buffer.content_size();
                check_w = cw;
                check_h = ch;
            }

            let cx = x + (self.size - check_w) / 2.0;
            let cy = y + (total_h - check_h) / 2.0;

            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: check_str.to_string(),
                pos: [cx, cy],
                options: zenthra_text::prelude::TextOptions::new()
                    .font_size(self.size * 0.8)
                    .color(self.check_color),
                clip: [0.0, 0.0, 9999.0, 9999.0],
            }));
        }

        // 6. Draw Label
        let tx = x + self.size + self.gap;
        let ty = y + (total_h - text_h) / 2.0;
        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [tx, ty],
            options: zenthra_text::prelude::TextOptions::new()
                .font_size(self.label_size)
                .color(self.label_color),
            clip: [0.0, 0.0, 9999.0, 9999.0],
        }));

        // 7. Register Semantic & Advance
        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::CheckBox, Rect::new(x, y, total_w, total_h))
                .with_label(self.label.clone())
                .with_value(if *self.value { 1.0 } else { 0.0 }),
        );

        self.ui.record_layout(self.id, Rect::new(x, y, total_w, total_h));
        self.ui.advance(total_w, total_h, start_draw);

        Response {
            clicked,
            hovered: is_hovered,
            pressed: is_pressed,
        }
    }
}
