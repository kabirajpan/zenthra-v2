use crate::ui::{Ui, DrawCommand, RectDraw};
use zenthra_core::{Color, EdgeInsets, Id, Response};
use zenthra_render::RectInstance;

pub struct SliderBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    value: &'b mut f32,
    min: f32,
    max: f32,
    step: Option<f32>,
    id: Id,
    
    // 1. Overall Widget Style
    width: Option<f32>,
    height: Option<f32>,
    padding: EdgeInsets,
    bg: Option<Color>,
    radius: [f32; 4],
    border_color: Option<Color>,
    border_width: f32,
    opacity: f32,
    shadow_color: Option<Color>,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,

    // 2. Track Style
    track_w: Option<f32>,
    track_h: f32,
    track_radius: [f32; 4],
    track_color: Color,
    track_shadow_color: Option<Color>,
    track_shadow_offset: [f32; 2],
    track_shadow_blur: f32,
    track_shadow_opacity: f32,

    // 3. Thumb Style
    thumb_w: f32,
    thumb_h: f32,
    thumb_radius: [f32; 4],
    thumb_color: Color,
    thumb_shadow_color: Option<Color>,
    thumb_shadow_offset: [f32; 2],
    thumb_shadow_blur: f32,
    thumb_shadow_opacity: f32,
}

impl<'u, 'a, 'b> SliderBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, value: &'b mut f32) -> Self {
        let id = ui.id();
        Self {
            ui,
            value,
            min: 0.0,
            max: 1.0,
            step: None,
            id,
            
            // Default Widget Style
            width: None,
            height: None,
            padding: EdgeInsets::symmetric(0.0, 10.0),
            bg: None,
            radius: [0.0; 4],
            border_color: None,
            border_width: 0.0,
            opacity: 1.0,
            shadow_color: None,
            shadow_offset: [0.0; 2],
            shadow_blur: 0.0,
            shadow_opacity: 1.0,

            // Default Track Style
            track_w: None,
            track_h: 4.0,
            track_radius: [2.0; 4],
            track_color: Color::rgb(0.2, 0.2, 0.2),
            track_shadow_color: None,
            track_shadow_offset: [0.0; 2],
            track_shadow_blur: 0.0,
            track_shadow_opacity: 1.0,

            // Default Thumb Style
            thumb_w: 16.0,
            thumb_h: 16.0,
            thumb_radius: [8.0; 4],
            thumb_color: Color::rgb(0.4, 0.6, 1.0),
            thumb_shadow_color: None,
            thumb_shadow_offset: [0.0; 2],
            thumb_shadow_blur: 0.0,
            thumb_shadow_opacity: 1.0,
        }
    }

    // --- Core Logic ---
    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    // --- 1. Overall Widget Methods ---
    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = Some(w);
        self.height = Some(h);
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

    pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.radius = [tl, tr, br, bl];
        self
    }

    pub fn radius_all(mut self, r: f32) -> Self {
        self.radius = [r; 4];
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

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self
    }

    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = o;
        self
    }

    pub fn shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self {
        self.shadow_color = Some(color);
        self.shadow_offset = [ox, oy];
        self.shadow_blur = blur;
        self
    }

    pub fn shadow_opacity(mut self, o: f32) -> Self {
        self.shadow_opacity = o;
        self
    }

    pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.padding = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }

    // --- 2. Track Methods ---
    pub fn track_size(mut self, w: f32, h: f32) -> Self {
        self.track_w = Some(w);
        self.track_h = h;
        self
    }

    pub fn track_radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.track_radius = [tl, tr, br, bl];
        self
    }

    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    pub fn track_shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self {
        self.track_shadow_color = Some(color);
        self.track_shadow_offset = [ox, oy];
        self.track_shadow_blur = blur;
        self
    }

    pub fn track_shadow_opacity(mut self, o: f32) -> Self {
        self.track_shadow_opacity = o;
        self
    }

    // --- 3. Thumb Methods ---
    pub fn thumb_size(mut self, w: f32, h: f32) -> Self {
        self.thumb_w = w;
        self.thumb_h = h;
        self
    }

    pub fn thumb_radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.thumb_radius = [tl, tr, br, bl];
        self
    }

    pub fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = color;
        self
    }

    pub fn thumb_shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self {
        self.thumb_shadow_color = Some(color);
        self.thumb_shadow_offset = [ox, oy];
        self.thumb_shadow_blur = blur;
        self
    }

    pub fn thumb_shadow_opacity(mut self, o: f32) -> Self {
        self.thumb_shadow_opacity = o;
        self
    }

    pub fn show(self) -> Response {
        let x = self.ui.cursor_x;
        let y = self.ui.cursor_y;
        
        // 1. Resolve Layout Sizes
        let w = self.width.or(self.track_w).unwrap_or(self.ui.available_width.max(100.0));
        let h = self.height.unwrap_or_else(|| {
            self.thumb_h.max(self.track_h).max(20.0) + self.padding.vertical()
        });

        // 2. Hit-testing Bounds
        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                if rect.size.width > 0.0 { rect.size.width } else { w },
                if rect.size.height > 0.0 { rect.size.height } else { h }
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
        };

        let is_hovered = self.ui.mouse_in_rect(actual_ox, actual_oy, actual_w, actual_h);
        let mut clicked = false;

        // --- Interaction Logic ---
        let track_w_hit = actual_w - self.padding.horizontal();
        let track_x_start_hit = actual_ox + self.padding.left;
        
        let is_active = self.ui.active_drag.as_ref().map(|d| d.id == self.id).unwrap_or(false);

        if (self.ui.clicked && is_hovered) || is_active {
            if track_w_hit > 0.0 {
                let relative_mouse_x = self.ui.mouse_x - track_x_start_hit;
                let new_percent = (relative_mouse_x / track_w_hit).clamp(0.0, 1.0);
                
                let mut new_val = self.min + new_percent * (self.max - self.min);
                if let Some(s) = self.step {
                    new_val = (new_val / s).round() * s;
                }
                
                if (*self.value - new_val).abs() > 0.0001 {
                    *self.value = new_val;
                    self.ui.needs_redraw = true;
                }
            }

            if self.ui.clicked && is_hovered {
                clicked = true;
                self.ui.active_drag = Some(crate::ui::ScrollDrag {
                    id: self.id,
                    start_mouse: self.ui.mouse_x,
                    start_scroll: 0.0, 
                });
            }
        }
        
        if is_active && !self.ui.mouse_down {
            self.ui.active_drag = None;
        }

        // --- Rendering ---
        let start_draw = self.ui.draws.len();
        let track_x_start = x + self.padding.left;
        let track_w = w - self.padding.horizontal();

        // 0. Background
        if let Some(bg) = self.bg {
            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [x, y],
                    size: [w, h],
                    color: bg.to_array(),
                    radius: self.radius,
                    border_width: self.border_width,
                    border_color: self.border_color.map(|c| c.to_array()).unwrap_or([0.0; 4]),
                    shadow_color: self.shadow_color.map(|mut c| { c.a *= self.shadow_opacity; c.to_array() }).unwrap_or([0.0; 4]),
                    shadow_offset: self.shadow_offset,
                    shadow_blur: self.shadow_blur,
                    opacity: self.opacity,
                    ..Default::default()
                }
            }));
        }

        // 1. Track
        let ty = y + (h - self.track_h) / 2.0;
        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [track_x_start, ty],
                size: [track_w, self.track_h],
                color: self.track_color.to_array(),
                radius: self.track_radius,
                shadow_color: self.track_shadow_color.map(|mut c| { c.a *= self.track_shadow_opacity; c.to_array() }).unwrap_or([0.0; 4]),
                shadow_offset: self.track_shadow_offset,
                shadow_blur: self.track_shadow_blur,
                opacity: self.opacity,
                ..Default::default()
            }
        }));

        // 2. Thumb
        let percent = ((*self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0);
        let thumb_x = track_x_start + percent * track_w - self.thumb_w / 2.0;
        let thumb_y = y + (h - self.thumb_h) / 2.0;
        
        let mut brightness = 1.0;
        if is_active {
            brightness = 1.3;
        } else if is_hovered {
            brightness = 1.1;
        }

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [thumb_x, thumb_y],
                size: [self.thumb_w, self.thumb_h],
                color: self.thumb_color.to_array(),
                radius: self.thumb_radius,
                brightness,
                shadow_color: self.thumb_shadow_color.map(|mut c| { c.a *= self.thumb_shadow_opacity; c.to_array() }).unwrap_or([0.0; 4]),
                shadow_offset: self.thumb_shadow_offset,
                shadow_blur: self.thumb_shadow_blur,
                opacity: self.opacity,
                ..Default::default()
            }
        }));

        // 3. Metadata
        self.ui.register_semantic(
            zenthra_core::SemanticNode::new(self.id, zenthra_core::Role::Slider, zenthra_core::Rect::new(x, y, w, h))
                .with_value(*self.value)
                .with_min_value(self.min)
                .with_max_value(self.max)
        );

        self.ui.record_layout(self.id, zenthra_core::Rect::new(x, y, w, h));
        self.ui.advance(w, h, start_draw);
        
        Response {
            clicked,
            hovered: is_hovered,
            pressed: is_active,
        }
    }
}
