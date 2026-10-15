use crate::ui::{Ui, DrawCommand, RectDraw};
use zenthra_core::{Color, EdgeInsets, Id, Response, BoxStyle, Styleable};
use zenthra_render::RectInstance;
use crate::style_ext::BoxStyleExt;

pub struct SliderBuilder<'u, 'a, 'b> {
    pub ui: &'u mut Ui<'a>,
    pub value: &'b mut f32,
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    pub id: Id,
    
    // Canonical BoxStyle for container
    pub style: BoxStyle,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,

    // 2. Track Style
    pub track_w: Option<f32>,
    pub track_h: f32,
    pub track_radius: [f32; 4],
    pub track_color: Color,
    pub track_shadow_color: Option<Color>,
    pub track_shadow_offset: [f32; 2],
    pub track_shadow_blur: f32,
    pub track_gradient: Option<(Color, Color, zenthra_core::GradientDirection)>,

    // 3. Thumb Style
    pub thumb_w: f32,
    pub thumb_h: f32,
    pub thumb_radius: [f32; 4],
    pub thumb_color: Color,
    pub thumb_shadow_color: Option<Color>,
    pub thumb_shadow_offset: [f32; 2],
    pub thumb_shadow_blur: f32,
    pub thumb_gradient: Option<(Color, Color, zenthra_core::GradientDirection)>,
}

impl<'u, 'a, 'b> Styleable for SliderBuilder<'u, 'a, 'b> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

crate::impl_box_style_methods!(SliderBuilder, 'u, 'a, 'b);
crate::impl_interaction_state_methods!(SliderBuilder, 'u, 'a, 'b);

impl<'u, 'a, 'b> SliderBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, value: &'b mut f32) -> Self {
        let id = ui.id();
        let mut style = BoxStyle::new();
        style.padding = EdgeInsets::symmetric(0.0, 10.0);
        Self {
            ui,
            value,
            min: 0.0,
            max: 1.0,
            step: None,
            id,
            style,
            hover_state: None,
            active_state: None,

            // Default Track Style
            track_w: None,
            track_h: 4.0,
            track_radius: [2.0; 4],
            track_color: Color::rgb(0.2, 0.2, 0.2),
            track_shadow_color: None,
            track_shadow_offset: [0.0; 2],
            track_shadow_blur: 0.0,
            track_gradient: None,

            // Default Thumb Style
            thumb_w: 16.0,
            thumb_h: 16.0,
            thumb_radius: [8.0; 4],
            thumb_color: Color::rgb(0.4, 0.6, 1.0),
            thumb_shadow_color: None,
            thumb_shadow_offset: [0.0; 2],
            thumb_shadow_blur: 0.0,
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
        self.id = self.ui.id_from(id);
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

    pub fn track_radius_all(mut self, r: f32) -> Self {
        self.track_radius = [r; 4];
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

    pub fn thumb_width(mut self, w: f32) -> Self {
        self.thumb_w = w;
        self
    }

    pub fn thumb_height(mut self, h: f32) -> Self {
        self.thumb_h = h;
        self
    }

    pub fn thumb_radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.thumb_radius = [tl, tr, br, bl];
        self
    }

    pub fn thumb_radius_all(mut self, r: f32) -> Self {
        self.thumb_radius = [r; 4];
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
        let w = self.style.width.or(self.track_w).unwrap_or(self.ui.available_width.max(100.0));
        let h = self.style.height.unwrap_or_else(|| {
            self.thumb_h.max(self.track_h).max(20.0) + self.style.padding.vertical()
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
        let track_w_hit = actual_w - self.style.padding.horizontal();
        let track_x_start_hit = actual_ox + self.style.padding.left;
        
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
        let track_x_start = x + self.style.padding.left;
        let track_w = w - self.style.padding.horizontal();

        // 0. Background
        let active_overrides = if is_active { self.active_state.as_ref() } else { None };
        let hover_overrides = if is_hovered { self.hover_state.as_ref() } else { None };

        let effective_bg = active_overrides
            .and_then(|o| o.style.bg.as_ref())
            .or_else(|| hover_overrides.and_then(|o| o.style.bg.as_ref()))
            .or(self.style.bg.as_ref());

        let effective_border_color = active_overrides
            .and_then(|o| o.style.border_color())
            .or_else(|| hover_overrides.and_then(|o| o.style.border_color()))
            .or_else(|| self.style.border_color());

        let effective_border_width = active_overrides
            .map(|o| o.style.border_width())
            .or_else(|| hover_overrides.map(|o| o.style.border_width()))
            .unwrap_or_else(|| self.style.border_width());

        if let Some(instance) = self.style.build_rect_styled(
            x,
            y,
            w,
            h,
            effective_bg,
            effective_border_color,
            Some(effective_border_width),
            1.0,
        ) {
            self.ui.draws.push(DrawCommand::Rect(RectDraw { instance }));
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

        // 4. Test Sibling Interaction states and Macro BoxStyle parity
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

            let s = ui.slider(&mut val, "test_slider")
                .width(220.0)
                .height(30.0)
                .margin_all(5.0)
                .padding_all(6.0)
                .radius_all(10.0)
                .track_radius_all(4.0)
                .thumb_radius_all(8.0)
                .thumb_width(20.0)
                .thumb_height(20.0)
                .hover(crate::macros::Style::bg(Color::rgb(0.2, 0.2, 0.3)))
                .active(crate::macros::Style::bg(Color::rgb(0.3, 0.3, 0.4)));

            assert_eq!(s.style.width, Some(220.0));
            assert_eq!(s.style.height, Some(30.0));
            assert_eq!(s.style.margin, EdgeInsets::all(5.0));
            assert_eq!(s.style.padding, EdgeInsets::all(6.0));
            assert_eq!(s.style.radius, [10.0; 4]);
            assert_eq!(s.track_radius, [4.0; 4]);
            assert_eq!(s.thumb_radius, [8.0; 4]);
            assert_eq!(s.thumb_w, 20.0);
            assert_eq!(s.thumb_h, 20.0);
            assert!(s.hover_state.is_some());
            assert!(s.active_state.is_some());
        }
    }
}
