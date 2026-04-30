use crate::ui::{Ui, DrawCommand, RectDraw};
use zenthra_core::{Color, Id, Response};
use zenthra_render::RectInstance;

pub struct SwitchBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    state: &'b mut bool,
    id: Id,
    
    // 1. Overall Style (The Track)
    width: f32,
    height: f32,
    radius: [f32; 4],
    padding: f32,
    opacity: f32,
    
    // 2. Thumb Style (The Small Button)
    thumb_w: Option<f32>,
    thumb_h: Option<f32>,
    thumb_radius: [f32; 4],
    thumb_color: Color,
    
    // 3. Track Colors
    on_color: Color,
    off_color: Color,
    
    // 4. Shadows
    shadow_color: Option<Color>,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,

    thumb_shadow_color: Option<Color>,
    thumb_shadow_offset: [f32; 2],
    thumb_shadow_blur: f32,
    thumb_shadow_opacity: f32,

    // 5. Animation
    animation_speed: f32,
}

impl<'u, 'a, 'b> SwitchBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, state: &'b mut bool, id: Id) -> Self {
        Self {
            ui,
            state,
            id,
            
            width: 44.0,
            height: 24.0,
            radius: [0.0; 4],
            padding: 4.0,
            opacity: 1.0,
            
            thumb_w: None,
            thumb_h: None,
            thumb_radius: [0.0; 4],
            thumb_color: Color::WHITE,
            
            on_color: Color::rgb(0.0, 0.8, 0.4),
            off_color: Color::rgb(0.2, 0.2, 0.2),
            
            shadow_color: None,
            shadow_offset: [0.0, 2.0],
            shadow_blur: 5.0,
            shadow_opacity: 0.3,

            thumb_shadow_color: None,
            thumb_shadow_offset: [0.0, 1.0],
            thumb_shadow_blur: 3.0,
            thumb_shadow_opacity: 0.4,

            animation_speed: 15.0,
        }
    }

    // --- 1. Overall Style Methods ---
    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = w;
        self.height = h;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.radius = [tl, tr, br, bl];
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = o;
        self
    }

    // --- 2. Thumb Style Methods ---
    pub fn thumb_size(mut self, w: f32, h: f32) -> Self {
        self.thumb_w = Some(w);
        self.thumb_h = Some(h);
        self
    }

    pub fn thumb_radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.thumb_radius = [tl, tr, br, bl];
        self
    }

    pub fn thumb_radius_full(mut self) -> Self {
        let h = self.thumb_h.unwrap_or(self.height - (self.padding * 2.0));
        let r = h / 2.0;
        self.thumb_radius = [r, r, r, r];
        self
    }

    pub fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = color;
        self
    }

    // --- 3. Color Methods ---
    pub fn colors(mut self, on: Color, off: Color, thumb: Color) -> Self {
        self.on_color = on;
        self.off_color = off;
        self.thumb_color = thumb;
        self
    }

    // --- 4. Shadow Methods ---
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

    pub fn thumb_shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.thumb_shadow_color = Some(color);
        self.thumb_shadow_offset = [x, y];
        self.thumb_shadow_blur = blur;
        self
    }

    pub fn thumb_shadow_opacity(mut self, opacity: f32) -> Self {
        self.thumb_shadow_opacity = opacity;
        self
    }

    pub fn show(self) -> Response {
        let x = self.ui.cursor_x;
        let y = self.ui.cursor_y;
        let w = self.width;
        let h = self.height;

        // Use recorded layout for accurate hit-testing (handles centering/alignment)
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

        // --- Interaction ---
        if self.ui.clicked && is_hovered {
            *self.state = !*self.state;
            clicked = true;
            self.ui.needs_redraw = true;
        }

        // --- Animation Logic ---
        let target_pos = if *self.state { 1.0 } else { 0.0 };
        let current_pos = *self.ui.interaction_state.entry(self.id).or_insert(target_pos);
        
        let delta = target_pos - current_pos;
        let mut final_pos = current_pos;
        
        if delta.abs() > 0.001 {
            final_pos += delta * (0.15 * self.animation_speed).min(1.0);
            self.ui.interaction_state.insert(self.id, final_pos);
            self.ui.needs_redraw = true;
        } else {
            self.ui.interaction_state.insert(self.id, target_pos);
            final_pos = target_pos;
        }

        // --- Rendering ---
        let start_draw = self.ui.draws.len();

        // 1. Draw Track
        let track_color = if final_pos > 0.5 {
            self.on_color
        } else {
            self.off_color
        };

        let mut track_shadow_color = [0.0; 4];
        if let Some(c) = self.shadow_color {
            track_shadow_color = c.to_array();
            track_shadow_color[3] *= self.shadow_opacity;
        }

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [x, y],
                size: [w, h],
                color: track_color.to_array(),
                radius: self.radius,
                opacity: self.opacity,
                shadow_color: track_shadow_color,
                shadow_offset: self.shadow_offset,
                shadow_blur: self.shadow_blur,
                ..Default::default()
            }
        }));

        // 2. Draw Thumb
        let thumb_h = self.thumb_h.unwrap_or(h - (self.padding * 2.0));
        let thumb_w = self.thumb_w.unwrap_or(thumb_h); // Default to square thumb if only H is known
        
        let track_w_internal = w - thumb_w - (self.padding * 2.0);
        let thumb_x = x + self.padding + (final_pos * track_w_internal);
        let thumb_y = y + (h - thumb_h) / 2.0;

        // Clamp radius to prevent SDF distortion for perfect circles
        let max_r = (thumb_w.min(thumb_h) / 2.0).max(0.0);
        let clamped_radius = [
            self.thumb_radius[0].min(max_r),
            self.thumb_radius[1].min(max_r),
            self.thumb_radius[2].min(max_r),
            self.thumb_radius[3].min(max_r),
        ];

        let mut thumb_shadow_color = [0.0; 4];
        if let Some(c) = self.thumb_shadow_color {
            thumb_shadow_color = c.to_array();
            thumb_shadow_color[3] *= self.thumb_shadow_opacity;
        }

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [thumb_x, thumb_y],
                size: [thumb_w, thumb_h],
                color: self.thumb_color.to_array(),
                radius: clamped_radius, 
                opacity: self.opacity,
                brightness: if is_hovered { 1.1 } else { 1.0 },
                shadow_color: thumb_shadow_color,
                shadow_offset: self.thumb_shadow_offset,
                shadow_blur: self.thumb_shadow_blur,
                ..Default::default()
            }
        }));

        self.ui.record_layout(self.id, zenthra_core::Rect::new(x, y, w, h));
        self.ui.advance(w, h, start_draw);

        Response {
            clicked,
            hovered: is_hovered,
            pressed: self.ui.mouse_down && is_hovered,
        }
    }
}
