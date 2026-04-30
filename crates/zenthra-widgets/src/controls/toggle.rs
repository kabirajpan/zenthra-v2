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
    label_gap: f32,

    // State
    disabled: bool,
    animation_speed: f32,
    is_pill: bool,
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
            label_gap: 10.0,

            disabled: false,
            animation_speed: 15.0,
            is_pill: false,
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
        let mut clicked = false;

        if self.ui.clicked && is_hovered {
            *self.state = !*self.state;
            self.ui.needs_redraw = true;
            clicked = true;
        }

        // 3. Animation
        let target_pos = if *self.state { 1.0 } else { 0.0 };
        let current_pos = *self.ui.interaction_state.entry(self.id).or_insert(target_pos);
        let mut final_pos = current_pos;
        
        let delta = target_pos - current_pos;
        if delta.abs() > 0.001 {
            final_pos += delta * (0.15 * self.animation_speed).min(1.0);
            self.ui.interaction_state.insert(self.id, final_pos);
            self.ui.needs_redraw = true;
        } else {
            self.ui.interaction_state.insert(self.id, target_pos);
            final_pos = target_pos;
        }

        let start_draw = self.ui.draws.len();

        // 4. Calculate Layout & Radius
        let current_radius = if self.is_pill { [self.height / 2.0; 4] } else { self.radius };
        let toggle_x = if let LabelSide::Left = self.label_side { x + label_w + self.label_gap } else { x };
        let toggle_y = y + (total_h - self.height) / 2.0;

        // 5. Draw Track
        let mut track_color = if final_pos > 0.5 { self.on_color } else { self.off_color };
        if self.disabled {
            track_color.a *= 0.5;
        }

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [toggle_x, toggle_y],
                size: [self.width, self.height],
                color: track_color.to_array(),
                radius: current_radius,
                brightness: if is_hovered { 1.1 } else { 1.0 },
                ..Default::default()
            }
        }));

        // 6. Draw Inner Labels (clipped to track)
        let clip = [toggle_x, toggle_y, self.width, self.height];
        if let Some(on_text) = &self.inner_on {
            if let Some(off_text) = &self.inner_off {
                // Determine which text to show and where
                let (text, is_on) = if final_pos > 0.5 { (on_text, true) } else { (off_text, false) };
                
                let mut text_w = 0.0;
                let mut text_h = 0.0;
                if let Some(fs) = self.ui.font_system.as_ref() {
                    let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
                    let options = zenthra_text::prelude::TextOptions::new().font_size(self.inner_font_size);
                    let buffer = adapter.shape(text, &options);
                    let (cw, ch) = buffer.content_size();
                    text_w = cw;
                    text_h = ch;
                }

                let thumb_h = self.height - (self.padding * 2.0);
                let thumb_w = self.thumb_width.unwrap_or(thumb_h);
                let track_w_internal = self.width - thumb_w - (self.padding * 2.0);
                
                // Position text in the space NOT occupied by the thumb
                let tx = if is_on {
                    // ON state: thumb is on the right, text on the left
                    toggle_x + self.padding + (track_w_internal - text_w) / 2.0
                } else {
                    // OFF state: thumb is on the left, text on the right
                    toggle_x + self.padding + thumb_w + (track_w_internal - text_w) / 2.0
                };
                let ty = toggle_y + (self.height - text_h) / 2.0;

                // Adjust opacity based on animation progress to avoid flickering
                let mut t_color = self.inner_text_color;
                let opacity = if is_on { (final_pos - 0.5) * 2.0 } else { (0.5 - final_pos) * 2.0 };
                t_color.a *= opacity.clamp(0.0, 1.0);

                if t_color.a > 0.05 {
                    self.ui.draws.push(DrawCommand::Text(TextDraw {
                        text: text.clone(),
                        pos: [tx, ty],
                        options: zenthra_text::prelude::TextOptions::new()
                            .font_size(self.inner_font_size)
                            .color(t_color),
                        clip,
                    }));
                }
            }
        }

        // 7. Draw Thumb
        let thumb_h = self.height - (self.padding * 2.0);
        let thumb_w = self.thumb_width.unwrap_or(thumb_h);
        let move_range = self.width - thumb_w - (self.padding * 2.0);
        let thumb_x = toggle_x + self.padding + (final_pos * move_range);
        let thumb_y = toggle_y + self.padding;

        let t_radius = self.thumb_radius.unwrap_or_else(|| {
            let r = thumb_h / 2.0;
            if self.is_pill { [r; 4] } else { 
                if self.radius[0] < r { self.radius } else { [r; 4] }
            }
        });

        let mut t_color = self.thumb_color;
        if self.disabled { t_color.a *= 0.8; }

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [thumb_x, thumb_y],
                size: [thumb_w, thumb_h],
                color: t_color.to_array(),
                radius: t_radius,
                ..Default::default()
            }
        }));

        // 8. Draw External Label
        if let Some(label_text) = &self.label {
            let lx = if let LabelSide::Left = self.label_side { x } else { toggle_x + self.width + self.label_gap };
            let ly = y + (total_h - label_h) / 2.0;
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: label_text.clone(),
                pos: [lx, ly],
                options: zenthra_text::prelude::TextOptions::new()
                    .font_size(self.label_size)
                    .color(self.label_color)
                    .at(lx, ly),
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
