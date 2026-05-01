// crates/zenthra-widgets/src/controls/toggle.rs

use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{Color, Id, Rect, Response, Role, SemanticNode};
use zenthra_render::RectInstance;

pub enum LabelSide {
    Left,
    Right,
}

pub struct ToggleBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    state: &'b mut bool,
    id: Id,
    label: Option<String>,
    label_side: LabelSide,

    // Track Style
    width: f32,
    height: f32,
    radius: [f32; 4],
    padding: f32,
    
    // Colors
    on_color: Color,
    off_color: Color,
    
    // Thumb
    thumb_color: Color,
    thumb_radius: Option<[f32; 4]>,
    thumb_width: Option<f32>,

    // Inner Labels
    inner_on: Option<String>,
    inner_off: Option<String>,
    inner_text_color: Color,
    inner_font_size: f32,

    // External Label Style
    label_size: f32,
    label_color: Color,
    active_label_color: Option<Color>,
    label_gap: f32,

    // Premium Effects
    glow: bool,
    shadow_color: Color,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,
    shadow_enabled: bool,
    hover_scale: f32,
    pressed_scale: f32,

    // State
    disabled: bool,
    animation_speed: f32,
    is_pill: bool,
    hover_brightness: f32,
}

impl<'u, 'a, 'b> ToggleBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, state: &'b mut bool, label: Option<&str>) -> Self {
        let id = ui.id();
        Self {
            ui,
            state,
            id,
            label: label.map(|s| s.to_string()),
            label_side: LabelSide::Right,

            width: 48.0,
            height: 24.0,
            radius: [12.0; 4],
            padding: 3.0,

            on_color: Color::rgb(0.0, 0.5, 1.0),
            off_color: Color::rgb(0.2, 0.2, 0.2),
            thumb_color: Color::WHITE,
            thumb_radius: None,
            thumb_width: None,

            inner_on: None,
            inner_off: None,
            inner_text_color: Color::WHITE,
            inner_font_size: 10.0,

            label_size: 14.0,
            label_color: Color::WHITE,
            active_label_color: None,
            label_gap: 10.0,

            glow: false,
            shadow_enabled: false,
            shadow_color: Color::rgb(0.0, 0.0, 0.0),
            shadow_offset: [0.0, 2.0],
            shadow_blur: 8.0,
            shadow_opacity: 0.4,
            hover_scale: 1.0,
            pressed_scale: 1.0,

            disabled: false,
            animation_speed: 15.0,
            is_pill: false,
            hover_brightness: 1.0,
        }
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = w;
        self.height = h;
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        self.radius = [r; 4];
        self.is_pill = false;
        self
    }

    pub fn square(mut self) -> Self {
        self.radius = [0.0; 4];
        self.is_pill = false;
        self
    }

    pub fn pill(mut self) -> Self {
        self.is_pill = true;
        self
    }

    pub fn inner_labels(mut self, on: &str, off: &str) -> Self {
        self.inner_on = Some(on.to_string());
        self.inner_off = Some(off.to_string());
        self
    }

    pub fn label_left(mut self) -> Self {
        self.label_side = LabelSide::Left;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn colors(mut self, on: Color, off: Color, thumb: Color) -> Self {
        self.on_color = on;
        self.off_color = off;
        self.thumb_color = thumb;
        self
    }

    pub fn thumb_radius(mut self, r: f32) -> Self {
        self.thumb_radius = Some([r; 4]);
        self
    }

    pub fn inner_color(mut self, color: Color) -> Self {
        self.inner_text_color = color;
        self
    }

    pub fn inner_size(mut self, size: f32) -> Self {
        self.inner_font_size = size;
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

    pub fn active_label(mut self, color: Color) -> Self {
        self.active_label_color = Some(color);
        self
    }

    pub fn glow(mut self, enabled: bool) -> Self {
        self.glow = enabled;
        self
    }

    pub fn scaling(mut self, hover: f32, pressed: f32) -> Self {
        self.hover_scale = hover;
        self.pressed_scale = pressed;
        self
    }

    pub fn label_gap(mut self, gap: f32) -> Self {
        self.label_gap = gap;
        self
    }

    pub fn label_right(mut self) -> Self {
        self.label_side = LabelSide::Right;
        self
    }

    pub fn animation_speed(mut self, speed: f32) -> Self {
        self.animation_speed = speed;
        self
    }

    pub fn hover_brightness(mut self, b: f32) -> Self {
        self.hover_brightness = b;
        self
    }

    pub fn show(self) -> Response {
        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        
        // 1. Measure External Label
        let mut label_w = 0.0;
        let mut label_h = 0.0;
        if let Some(label_text) = &self.label {
            // Use the internal text measurement helper
            if let Some(fs) = self.ui.font_system.as_ref() {
                let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
                let options = zenthra_text::prelude::TextOptions::new().font_size(self.label_size);
                let buffer = adapter.shape(label_text, &options);
                let (cw, ch) = buffer.content_size();
                label_w = cw;
                label_h = ch;
            }
        }

        let total_w = if label_w > 0.0 { self.width + self.label_gap + label_w } else { self.width };
        let total_h = self.height.max(label_h);

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
            *self.state = !*self.state;
            self.ui.needs_redraw = true;
            clicked = true;
        }

        // 3. Animation (Generic & Selection)
        let sel_target = if *self.state { 1.0 } else { 0.0 };
        let current_sel = *self.ui.interaction_state.entry(self.id).or_insert(sel_target);

        let hover_id = Id::from_u64(self.id.raw().wrapping_add(3000000));
        let hover_target = if is_pressed { self.pressed_scale } else if is_hovered { self.hover_scale } else { 1.0 };
        let current_scale = *self.ui.interaction_state.entry(hover_id).or_insert(1.0);

        let mut final_pos = current_sel;
        let mut final_scale = current_scale;
        
        let s_delta = sel_target - current_sel;
        if s_delta.abs() > 0.001 {
            final_pos += s_delta * (0.15 * self.animation_speed).min(1.0);
            self.ui.interaction_state.insert(self.id, final_pos);
            self.ui.needs_redraw = true;
        } else {
            final_pos = sel_target;
        }

        let h_delta = hover_target - current_scale;
        if h_delta.abs() > 0.001 {
            final_scale += h_delta * 0.3;
            self.ui.interaction_state.insert(hover_id, final_scale);
            self.ui.needs_redraw = true;
        } else {
            final_scale = hover_target;
        }

        let start_draw = self.ui.draws.len();

        // 4. Calculate Layout & Radius
        let base_w = self.width * final_scale;
        let base_h = self.height * final_scale;
        
        let current_radius = if self.is_pill { [base_h / 2.0; 4] } else { 
            let mut r = self.radius;
            for i in 0..4 { r[i] *= final_scale; }
            r
        };
        
        let toggle_x = if let LabelSide::Left = self.label_side { x + label_w + self.label_gap } else { x };
        let toggle_y_orig = y + (total_h - self.height) / 2.0;
        
        // Midpoint of the original toggle track
        let mid_x = toggle_x + self.width / 2.0;
        let mid_y = toggle_y_orig + self.height / 2.0;
        
        // Final position for scaled track
        let tx = mid_x - base_w / 2.0;
        let ty = mid_y - base_h / 2.0;

        // 5. Draw Track
        let mut track_color = if final_pos > 0.5 { self.on_color } else { self.off_color };
        if self.disabled {
            track_color.a *= 0.5;
        }

        let mut shadow_color = Color::TRANSPARENT;
        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [tx, ty],
                size: [base_w, base_h],
                color: track_color.to_array(),
                radius: current_radius,
                brightness: if is_hovered { self.hover_brightness } else { 1.0 },
                shadow_color: if self.shadow_enabled {
                    let mut a = self.shadow_color.to_array();
                    a[3] *= self.shadow_opacity;
                    a
                } else if self.glow {
                    let mut sc = self.on_color.to_array();
                    sc[3] *= 0.4 * final_pos;
                    sc
                } else { [0.0, 0.0, 0.0, 0.0] },
                shadow_offset: if self.shadow_enabled { self.shadow_offset } else { [0.0, 0.0] },
                shadow_blur: if self.shadow_enabled { self.shadow_blur } else if self.glow { 12.0 * final_pos } else { 0.0 },
                ..Default::default()
            }
        }));

        // 6. Draw Inner Labels (clipped to track)
        let clip = [tx, ty, base_w, base_h];
        if let Some(on_text) = &self.inner_on {
            if let Some(off_text) = &self.inner_off {
                // Determine which text to show and where
                let (text, is_on) = if final_pos > 0.5 { (on_text, true) } else { (off_text, false) };
                
                let mut text_w = 0.0;
                let mut text_h = 0.0;
                if let Some(fs) = self.ui.font_system.as_ref() {
                    let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
                    let options = zenthra_text::prelude::TextOptions::new().font_size(self.inner_font_size * final_scale);
                    let buffer = adapter.shape(text, &options);
                    let (cw, ch) = buffer.content_size();
                    text_w = cw;
                    text_h = ch;
                }

                let thumb_h_adj = base_h - (self.padding * 2.0 * final_scale);
                let thumb_w_adj = self.thumb_width.map(|w| w * final_scale).unwrap_or(thumb_h_adj);
                let track_w_internal = base_w - thumb_w_adj - (self.padding * 2.0 * final_scale);
                
                // Position text in the space NOT occupied by the thumb
                let inner_tx = if is_on {
                    // ON state: thumb is on the right, text on the left
                    tx + (self.padding * final_scale) + (track_w_internal - text_w) / 2.0
                } else {
                    // OFF state: thumb is on the left, text on the right
                    tx + (self.padding * final_scale) + thumb_w_adj + (track_w_internal - text_w) / 2.0
                };
                let inner_ty = ty + (base_h - text_h) / 2.0;

                // Adjust opacity based on animation progress to avoid flickering
                let mut t_color = self.inner_text_color;
                let opacity = if is_on { (final_pos - 0.5) * 2.0 } else { (0.5 - final_pos) * 2.0 };
                t_color.a *= opacity.clamp(0.0, 1.0);

                if t_color.a > 0.05 {
                    self.ui.draws.push(DrawCommand::Text(TextDraw {
                        text: text.clone(),
                        pos: [inner_tx, inner_ty],
                        options: zenthra_text::prelude::TextOptions::new()
                            .font_size(self.inner_font_size * final_scale)
                            .color(t_color),
                        clip,
                    }));
                }
            }
        }

        // 7. Draw Thumb
        let thumb_h_adj = base_h - (self.padding * 2.0 * final_scale);
        let thumb_w_adj = self.thumb_width.map(|w| w * final_scale).unwrap_or(thumb_h_adj);
        let move_range = base_w - thumb_w_adj - (self.padding * 2.0 * final_scale);
        let thumb_x = tx + (self.padding * final_scale) + (final_pos * move_range);
        let thumb_y = ty + (self.padding * final_scale);

        let t_radius = self.thumb_radius.map(|mut r| { for i in 0..4 { r[i] *= final_scale; } r }).unwrap_or_else(|| {
            let r = thumb_h_adj / 2.0;
            if self.is_pill { [r; 4] } else { 
                let tr = self.radius[0] * final_scale;
                if tr < r { [tr; 4] } else { [r; 4] }
            }
        });

        let mut t_color = self.thumb_color;
        if self.disabled { t_color.a *= 0.8; }

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [thumb_x, thumb_y],
                size: [thumb_w_adj, thumb_h_adj],
                color: t_color.to_array(),
                radius: t_radius,
                ..Default::default()
            }
        }));

        // 8. Draw External Label (Color Transition)
        if let Some(label_text) = &self.label {
            let lx = if let LabelSide::Left = self.label_side { x } else { toggle_x + self.width + self.label_gap };
            let ly = y + (total_h - label_h) / 2.0;
            
            let mut l_color = self.label_color;
            if let Some(active_c) = self.active_label_color {
                if final_pos > 0.5 {
                    l_color = active_c;
                }
            }

            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: label_text.clone(),
                pos: [lx, ly],
                options: zenthra_text::prelude::TextOptions::new()
                    .font_size(self.label_size)
                    .color(l_color),
                clip: [0.0, 0.0, 9999.0, 9999.0],
            }));
        }

        // 9. Register Semantic & Advance
        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Switch, Rect::new(x, y, total_w, total_h))
                .with_label(self.label.clone().unwrap_or_default())
                .with_value(if *self.state { 1.0 } else { 0.0 }),
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
