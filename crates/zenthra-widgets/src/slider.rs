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
    bg: Option<zenthra_core::Background>,
    radius: [f32; 4],
    border_color: Option<Color>,
    border_width: f32,
    shadow_color: Option<Color>,
    shadow_offset: [f32; 2],
    shadow_blur: f32,

    // 2. Track Style
    track_w: Option<f32>,
    track_h: f32,
    track_radius: [f32; 4],
    track_color: Color,
    track_shadow_color: Option<Color>,
    track_shadow_offset: [f32; 2],
    track_shadow_blur: f32,

    // 3. Thumb Style
    thumb_w: f32,
    thumb_h: f32,
    thumb_radius: [f32; 4],
    thumb_color: Color,
    thumb_shadow_color: Option<Color>,
    thumb_shadow_offset: [f32; 2],
    thumb_shadow_blur: f32,

    track_gradient: Option<(Color, Color, zenthra_core::GradientDirection)>,
    thumb_gradient: Option<(Color, Color, zenthra_core::GradientDirection)>,
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
            shadow_color: None,
            shadow_offset: [0.0; 2],
            shadow_blur: 0.0,

            // Default Track Style
            track_w: None,
            track_h: 4.0,
            track_radius: [2.0; 4],
            track_color: Color::rgb(0.2, 0.2, 0.2),
            track_shadow_color: None,
            track_shadow_offset: [0.0; 2],
            track_shadow_blur: 0.0,

            // Default Thumb Style
            thumb_w: 16.0,
            thumb_h: 16.0,
            thumb_radius: [8.0; 4],
            thumb_color: Color::rgb(0.4, 0.6, 1.0),
            thumb_shadow_color: None,
            thumb_shadow_offset: [0.0; 2],
            thumb_shadow_blur: 0.0,
            track_gradient: None,
            thumb_gradient: None,
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

    pub fn fill_x(mut self) -> Self {
        self.width = Some(self.ui.available_width);
        self
    }

    pub fn fill_y(mut self) -> Self {
        self.height = Some((self.ui.max_y - self.ui.cursor_y).max(20.0));
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

    pub fn bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self {
        match bg.into() {
            zenthra_core::Background::None => self.bg = None,
            other => self.bg = Some(other),
        }
        self
    }

    pub fn border(mut self, border: impl Into<zenthra_core::Border>) -> Self {
        match border.into() {
            zenthra_core::Border::None => {
                self.border_color = None;
                self.border_width = 0.0;
            }
            zenthra_core::Border::Solid(c, w) => {
                self.border_color = Some(c);
                self.border_width = w;
            }
        }
        self
    }

    pub fn shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self {
        self.shadow_color = Some(color);
        self.shadow_offset = [ox, oy];
        self.shadow_blur = blur;
        self
    }

    pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.padding = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }

    pub fn padding_all(mut self, p: f32) -> Self {
        self.padding = EdgeInsets::all(p);
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

    // --- 2. Track Methods ---
    pub fn track_size(mut self, w: f32, h: f32) -> Self {
        self.track_w = Some(w);
        self.track_h = h;
        self
    }

    pub fn track_height(mut self, h: f32) -> Self {
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

    pub fn track_gradient(mut self, start: Color, end: Color, direction: zenthra_core::GradientDirection) -> Self {
        self.track_gradient = Some((start, end, direction));
        self
    }

    pub fn track_shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self {
        self.track_shadow_color = Some(color);
        self.track_shadow_offset = [ox, oy];
        self.track_shadow_blur = blur;
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

    pub fn thumb_gradient(mut self, start: Color, end: Color, direction: zenthra_core::GradientDirection) -> Self {
        self.thumb_gradient = Some((start, end, direction));
        self
    }

    pub fn thumb_shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self {
        self.thumb_shadow_color = Some(color);
        self.thumb_shadow_offset = [ox, oy];
        self.thumb_shadow_blur = blur;
        self
    }

    pub fn on_change<F>(self, mut f: F) -> Self
    where
        F: FnMut(f32) + 'a,
    {
        self.ui.add_listener(self.id, crate::ui::EventPhase::Bubble, move |_, event| {
            if let crate::ui::WidgetEvent::Change(crate::ui::EventValue::Float(val)) = event {
                f(*val);
            }
        });
        self
    }

    pub fn on_hover<F>(self, mut f: F) -> Self
    where
        F: FnMut(bool) + 'a,
    {
        self.ui.add_listener(self.id, crate::ui::EventPhase::Bubble, move |_, event| {
            if let crate::ui::WidgetEvent::Hover(hovered) = event {
                f(*hovered);
            }
        });
        self
    }

    pub fn on_event<F>(self, phase: crate::ui::EventPhase, f: F) -> Self
    where
        F: FnMut(&mut crate::ui::EventContext, &crate::ui::WidgetEvent) + 'a,
    {
        self.ui.add_listener(self.id, phase, f);
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
        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some(screen_rect) = self.ui.screen_layout_cache.get(&self.id) {
            (
                screen_rect.origin.x,
                screen_rect.origin.y,
                if screen_rect.size.width > 0.0 { screen_rect.size.width } else { w },
                if screen_rect.size.height > 0.0 { screen_rect.size.height } else { h }
            )
        } else if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                if rect.size.width > 0.0 { rect.size.width } else { w },
                if rect.size.height > 0.0 { rect.size.height } else { h }
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
        };

        let is_hovered = self.ui.is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);
        let mut clicked = false;
        let mut changed = false;

        self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Hover(is_hovered));

        // --- Interaction Logic ---
        let track_w_hit = actual_w - self.padding.horizontal();
        let track_x_start_hit = actual_ox + self.padding.left;
        
        let is_active = self.ui.active_drag.as_ref().map(|d| d.id == self.id).unwrap_or(false);

        if is_hovered || is_active {
            self.ui.cursor_icon = crate::text::CursorIcon::Pointer;
        }

        if (self.ui.clicked && is_hovered && self.ui.can_interact(self.id)) || is_active {
            if track_w_hit > 0.0 {
                let relative_mouse_x = self.ui.mouse_x - track_x_start_hit;
                let new_percent = (relative_mouse_x / track_w_hit).clamp(0.0, 1.0);
                
                let mut new_val = self.min + new_percent * (self.max - self.min);
                if let Some(s) = self.step {
                    new_val = (new_val / s).round() * s;
                }
                
                if (*self.value - new_val).abs() > 0.001 {
                    *self.value = new_val;
                    changed = true;
                    self.ui.needs_redraw = true;
                    self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Change(crate::ui::EventValue::Float(new_val)));
                }
            }

            if self.ui.clicked && is_hovered {
                clicked = true;
                self.ui.claim_active(self.id);
                self.ui.active_drag = Some(crate::ui::ScrollDrag {
                    id: self.id,
                    start_mouse: self.ui.mouse_x,
                    start_scroll: 0.0, 
                });
                self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Click);
            }
        }
        
        if is_active && !self.ui.mouse_down {
            self.ui.active_drag = None;
            if self.ui.is_active(self.id) {
                self.ui.release_active();
            }
        }

        // --- Rendering ---
        let start_draw = self.ui.draws.len();
        let track_x_start = x + self.padding.left;
        let track_w = w - self.padding.horizontal();

        // 0. Background
        if self.bg.is_some() || self.border_color.is_some() || self.shadow_color.is_some() {
            let (bg_c1, bg_c2, _c3, _c4, bg_params, _stops, _vis) = crate::style_ext::resolve_background(&self.bg);

            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [x, y],
                    size: [w, h],
                    color: bg_c1,
                    color2: bg_c2,
                    gradient_params: bg_params,
                    radius: [
                        self.radius[3],
                        self.radius[2],
                        self.radius[1],
                        self.radius[0],
                    ],
                    border_width: self.border_width,
                    border_color: self.border_color.map(|c| c.to_array()).unwrap_or([0.0; 4]),
                    shadow_color: self.shadow_color.map(|c| c.to_array()).unwrap_or([0.0; 4]),
                    shadow_offset: self.shadow_offset,
                    shadow_blur: self.shadow_blur,
                    opacity: 1.0,
                    ..Default::default()
                }
            }));
        }

        // 1. Track
        let ty = y + (h - self.track_h) / 2.0;
        let (track_c1, track_c2, track_params) = if let Some((s, e, d)) = self.track_gradient {
            let (t, a) = d.to_type_and_angle();
            (s.to_array(), e.to_array(), [t, a, 0.0, 0.0])
        } else {
            (self.track_color.to_array(), [0.0; 4], [0.0; 4])
        };

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [track_x_start, ty],
                size: [track_w, self.track_h],
                color: track_c1,
                color2: track_c2,
                gradient_params: track_params,
                radius: [
                    self.track_radius[3],
                    self.track_radius[2],
                    self.track_radius[1],
                    self.track_radius[0],
                ],
                shadow_color: self.track_shadow_color.map(|c| c.to_array()).unwrap_or([0.0; 4]),
                shadow_offset: self.track_shadow_offset,
                shadow_blur: self.track_shadow_blur,
                opacity: 1.0,
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

        let (thumb_c1, thumb_c2, thumb_params) = if let Some((s, e, d)) = self.thumb_gradient {
            let (t, a) = d.to_type_and_angle();
            (s.to_array(), e.to_array(), [t, a, 0.0, 0.0])
        } else {
            (self.thumb_color.to_array(), [0.0; 4], [0.0; 4])
        };

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [thumb_x, thumb_y],
                size: [self.thumb_w, self.thumb_h],
                color: thumb_c1,
                color2: thumb_c2,
                gradient_params: thumb_params,
                radius: [
                    self.thumb_radius[3],
                    self.thumb_radius[2],
                    self.thumb_radius[1],
                    self.thumb_radius[0],
                ],
                brightness,
                shadow_color: self.thumb_shadow_color.map(|c| c.to_array()).unwrap_or([0.0; 4]),
                shadow_offset: self.thumb_shadow_offset,
                shadow_blur: self.thumb_shadow_blur,
                opacity: 1.0,
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
            pressed: is_active || (self.ui.clicked && is_hovered),
            submitted: false,
            changed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_slider_styling_and_decorations() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        let mut val = 0.5;

        // 1. Default Slider without container decoration: 2 draws (track, thumb)
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.slider(&mut val, "test_slider").width(200.0).show();
            assert_eq!(ui.draws.len(), 2, "Default slider should emit Track and Thumb");
        }

        // 2. Decorated Slider with border & shadow (bg=None): 3 draws (bg/border, track, thumb)
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.slider(&mut val, "test_slider")
                .width(200.0)
                .border((Color::WHITE, 1.5))
                .radius_all(6.0)
                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.4), 0.0, 2.0, 4.0)
                .show();

            assert_eq!(ui.draws.len(), 3, "Decorated slider must emit Background/Border quad");
            if let DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.color, Color::TRANSPARENT.to_array());
                assert_eq!(rd.instance.border_width, 1.5);
                assert_eq!(rd.instance.border_color, Color::WHITE.to_array());
                assert_eq!(rd.instance.radius, [6.0, 6.0, 6.0, 6.0]);
                assert_eq!(rd.instance.shadow_color, [0.0, 0.0, 0.0, 0.4]);
            } else {
                panic!("Expected RectDraw for slider background quad");
            }
        }

        // 3. Decorated Slider with solid background
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.slider(&mut val, "test_slider")
                .width(200.0)
                .bg(Color::rgb(0.1, 0.1, 0.15))
                .radius_all(8.0)
                .show();

            assert_eq!(ui.draws.len(), 3);
            if let DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.color, Color::rgb(0.1, 0.1, 0.15).to_array());
            } else {
                panic!("Expected RectDraw for solid slider background");
            }
        }
    }
}
