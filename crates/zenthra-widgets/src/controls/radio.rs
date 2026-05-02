// crates/zenthra-widgets/src/controls/radio.rs

use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{Color, Id, Rect, Response, Role, SemanticNode};
use zenthra_render::RectInstance;

pub struct RadioBuilder<'u, 'a, 'b, T: PartialEq + Clone> {
    ui: &'u mut Ui<'a>,
    state: &'b mut T,
    value: T,
    label: String,
    id: Id,

    // Layout
    size: f32,
    gap: f32,

    // Visuals
    bg: Color,
    ring_color: Color,
    dot_color: Color,
    border_width: f32,

    // Label Style
    label_size: f32,
    label_color: Color,
    active_label_color: Option<Color>,

    // Premium Effects
    glow: bool,
    shadow_color: Color,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,
    shadow_enabled: bool,
    hover_scale: f32,
    pressed_scale: f32,

    // Interaction
    disabled: bool,
}

impl<'u, 'a, 'b, T: PartialEq + Clone> RadioBuilder<'u, 'a, 'b, T> {
    pub fn new(ui: &'u mut Ui<'a>, state: &'b mut T, value: T, label: &str) -> Self {
        let id = ui.id();
        Self {
            ui,
            state,
            value,
            label: label.to_string(),
            id,

            size: 18.0,
            gap: 10.0,

            bg: Color::TRANSPARENT,
            ring_color: Color::rgb(0.4, 0.4, 0.4),
            dot_color: Color::rgb(0.2, 0.5, 1.0),
            border_width: 1.5,

            label_size: 14.0,
            label_color: Color::WHITE,
            active_label_color: None,

            glow: false,
            shadow_enabled: false,
            shadow_color: Color::rgb(0.0, 0.0, 0.0),
            shadow_offset: [0.0, 2.0],
            shadow_blur: 8.0,
            shadow_opacity: 0.4,
            hover_scale: 1.0,
            pressed_scale: 1.0,

            disabled: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn colors(mut self, ring: Color, dot: Color) -> Self {
        self.ring_color = ring;
        self.dot_color = dot;
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    pub fn border(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    pub fn label_style(mut self, size: f32, color: Color) -> Self {
        self.label_color = color;
        self.label_size = size;
        self
    }

    pub fn active_label(mut self, color: Color) -> Self {
        self.active_label_color = Some(color);
        self
    }

    pub fn glow(mut self, enabled: bool) -> Self {
        self.glow = enabled;
        self
    }

    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.shadow_color = color;
        self.shadow_offset = [x, y];
        self.shadow_blur = blur;
        self.shadow_enabled = true;
        self
    }

    pub fn shadow_opacity(mut self, opacity: f32) -> Self {
        self.shadow_opacity = opacity;
        self
    }

    pub fn scaling(mut self, hover: f32, pressed: f32) -> Self {
        self.hover_scale = hover;
        self.pressed_scale = pressed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    pub fn show(self) -> Response {
        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        let is_active = *self.state == self.value;

        // 1. Measure Label
        let mut label_w = 0.0;
        let mut label_h = 0.0;
        if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let options = zenthra_text::prelude::TextOptions::new().font_size(self.label_size);
            let buffer = adapter.shape(&self.label, &options);
            let (cw, ch) = buffer.content_size();
            label_w = cw;
            label_h = ch;
        }

        let total_w = self.size + self.gap + label_w;
        let total_h = self.size.max(label_h);

        // 2. Hit-testing
        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                rect.size.width.max(total_w),
                rect.size.height.max(total_h)
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, total_w, total_h)
        };

        let is_hovered = self.ui.mouse_in_rect(actual_ox, actual_oy, actual_w, actual_h) && !self.disabled;
        let is_pressed = is_hovered && self.ui.mouse_down;
        let mut clicked = false;

        if self.ui.clicked && is_hovered {
            if !is_active {
                *self.state = self.value.clone();
                self.ui.needs_redraw = true;
                clicked = true;
            }
        }

        // 3. Animation (Generic & Selection)
        let sel_target = if is_active { 1.0 } else { 0.0 };
        let current_sel = *self.ui.interaction_state.entry(self.id).or_insert(sel_target);
        
        // Interaction State for Hover/Press (Id + 1 for unique mapping if needed, but let's use a separate logic)
        let hover_id = Id::from_u64(self.id.raw().wrapping_add(1000000));
        let hover_target = if is_pressed { self.pressed_scale } else if is_hovered { self.hover_scale } else { 1.0 };
        let current_scale = *self.ui.interaction_state.entry(hover_id).or_insert(1.0);

        let mut final_sel = current_sel;
        let mut final_scale = current_scale;
        
        let s_delta = sel_target - current_sel;
        if s_delta.abs() > 0.001 {
            final_sel += s_delta * 0.5;
            self.ui.interaction_state.insert(self.id, final_sel);
            self.ui.needs_redraw = true;
        } else {
            final_sel = sel_target;
        }

        let h_delta = hover_target - current_scale;
        if h_delta.abs() > 0.001 {
            // Snappier, smoother lerp
            final_scale += h_delta * 0.5;
            self.ui.interaction_state.insert(hover_id, final_scale);
            self.ui.needs_redraw = true;
        } else {
            final_scale = hover_target;
        }

        let start_draw = self.ui.draws.len();
        
        // Visual Midpoints
        let mid_x = x + self.size / 2.0;
        let mid_y = y + (total_h / 2.0);
        
        // Scaled dimensions
        let base_size = self.size * final_scale;
        let bx = mid_x - base_size / 2.0;
        let by = mid_y - base_size / 2.0;
        let radius = [base_size / 2.0; 4];

        // 4. Draw Ring
        let ring_color = if is_active { self.dot_color } else { self.ring_color };
        let mut r_color = ring_color;
        if self.disabled { r_color.a *= 0.5; }


        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [bx, by],
                size: [base_size, base_size],
                color: self.bg.to_array(),
                radius,
                border_width: self.border_width,
                border_color: r_color.to_array(),
                brightness: if is_hovered { 1.2 } else { 1.0 },
                shadow_color: if self.shadow_enabled {
                    let mut a = self.shadow_color.to_array();
                    a[3] *= self.shadow_opacity;
                    a
                } else if self.glow && is_active {
                    let mut sc = self.dot_color.to_array();
                    sc[3] *= 0.4 * final_sel;
                    sc
                } else { [0.0, 0.0, 0.0, 0.0] },
                shadow_offset: if self.shadow_enabled { self.shadow_offset } else { [0.0, 0.0] },
                shadow_blur: if self.shadow_enabled { self.shadow_blur } else if self.glow && is_active { 10.0 * final_sel } else { 0.0 },
                ..Default::default()
            }
        }));

        // 5. Draw Dot (Animated Scale)
        if final_sel > 0.01 {
            let dot_size = base_size * 0.5 * final_sel;
            let dx = bx + (base_size - dot_size) / 2.0;
            let dy = by + (base_size - dot_size) / 2.0;
            
            let mut d_color = self.dot_color;
            d_color.a *= final_sel;
            if self.disabled { d_color.a *= 0.5; }

            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [dx, dy],
                    size: [dot_size, dot_size],
                    color: d_color.to_array(),
                    radius: [dot_size / 2.0; 4],
                    ..Default::default()
                }
            }));
        }

        // 6. Draw Label (Color Transition)
        let lx = x + self.size + self.gap;
        let ly = y + (total_h - label_h) / 2.0;
        
        let mut l_color = self.label_color;
        if let Some(active_c) = self.active_label_color {
            // Simple linear interpolation for color would be nice, but for now we just switch based on final_sel
            if final_sel > 0.5 {
                l_color = active_c;
            }
        }

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [lx, ly],
            options: zenthra_text::prelude::TextOptions::new()
                .font_size(self.label_size)
                .color(l_color),
            clip: [0.0, 0.0, 9999.0, 9999.0],
        }));

        // 7. Register Semantic & Advance
        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::RadioButton, Rect::new(x, y, total_w, total_h))
                .with_label(self.label.clone())
                .with_value(if is_active { 1.0 } else { 0.0 }),
        );

        self.ui.record_layout(self.id, Rect::new(x, y, total_w, total_h));
        self.ui.advance(total_w, total_h, start_draw);

        Response {
            clicked,
            hovered: is_hovered,
            pressed: is_hovered && self.ui.mouse_down,
        }
    }
}
