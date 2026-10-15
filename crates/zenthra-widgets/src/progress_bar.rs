use crate::style_ext::BoxStyleExt;
use crate::ui::{DrawCommand, RectDraw, Ui};
use zenthra_core::{BoxStyle, Color, EdgeInsets, Id, Response, Styleable};
use zenthra_render::RectInstance;

pub struct ProgressBarBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,
    pub value: f32, // 0.0 to 1.0

    // 1. Overall Widget Style
    pub style: BoxStyle,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,

    // 2. Track Style (The background of the bar)
    pub track_h: f32,
    pub track_radius: Option<[f32; 4]>,
    pub track_bg: Option<zenthra_core::Background>,
    pub track_color: Color,
    pub track_shadow_color: Option<Color>,
    pub track_shadow_offset: [f32; 2],
    pub track_shadow_blur: f32,
    pub track_gradient: Option<(Color, Color, zenthra_core::GradientDirection)>,

    // 3. Fill Style (The actual progress)
    pub fill_bg: Option<zenthra_core::Background>,
    pub fill_color: Color,
    pub fill_radius: Option<[f32; 4]>,
    pub fill_shadow_color: Option<Color>,
    pub fill_shadow_offset: [f32; 2],
    pub fill_shadow_blur: f32,
    pub fill_gradient: Option<(Color, Color, zenthra_core::GradientDirection)>,

    // 4. Effects
    pub shimmer: bool,
}

impl<'u, 'a> Styleable for ProgressBarBuilder<'u, 'a> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

crate::impl_box_style_methods!(ProgressBarBuilder, 'u, 'a);
crate::impl_interaction_state_methods!(ProgressBarBuilder, 'u, 'a);

impl<'u, 'a> ProgressBarBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, value: f32) -> Self {
        let id = ui.id();
        let mut style = BoxStyle::new();
        style.padding = EdgeInsets::symmetric(4.0, 4.0);

        Self {
            ui,
            id,
            value: value.clamp(0.0, 1.0),

            style,
            hover_state: None,
            active_state: None,

            track_h: 8.0,
            track_radius: None,
            track_bg: None,
            track_color: Color::rgb(0.15, 0.15, 0.15),
            track_shadow_color: None,
            track_shadow_offset: [0.0, 2.0],
            track_shadow_blur: 5.0,
            track_gradient: None,

            fill_bg: None,
            fill_color: Color::rgb(0.4, 0.6, 1.0),
            fill_radius: None,
            fill_shadow_color: None,
            fill_shadow_offset: [0.0, 1.0],
            fill_shadow_blur: 3.0,
            fill_gradient: None,

            shimmer: false,
        }
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = self.ui.id_from(id);
        self
    }

    // --- 2. Track Methods ---
    pub fn track_height(mut self, h: f32) -> Self {
        self.track_h = h;
        self
    }

    pub fn track_size(mut self, w: f32, h: f32) -> Self {
        self.style.width = Some(w);
        self.track_h = h;
        self
    }

    pub fn track_radius_all(mut self, r: f32) -> Self {
        self.track_radius = Some([r; 4]);
        self
    }

    pub fn track_radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.track_radius = Some([tl, tr, br, bl]);
        self
    }

    pub fn track_bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self {
        self.track_bg = Some(bg.into());
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

    pub fn track_shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.track_shadow_color = Some(color);
        self.track_shadow_offset = [x, y];
        self.track_shadow_blur = blur;
        self
    }

    // --- 3. Fill Methods ---
    pub fn fill_bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self {
        self.fill_bg = Some(bg.into());
        self
    }

    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    pub fn fill_gradient(mut self, start: Color, end: Color, direction: zenthra_core::GradientDirection) -> Self {
        self.fill_gradient = Some((start, end, direction));
        self
    }

    pub fn fill_radius_all(mut self, r: f32) -> Self {
        self.fill_radius = Some([r; 4]);
        self
    }

    pub fn fill_radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.fill_radius = Some([tl, tr, br, bl]);
        self
    }

    pub fn fill_shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.fill_shadow_color = Some(color);
        self.fill_shadow_offset = [x, y];
        self.fill_shadow_blur = blur;
        self
    }

    // --- 4. Effects ---
    pub fn shimmer(mut self, enabled: bool) -> Self {
        self.shimmer = enabled;
        self
    }

    // --- 5. Event Callbacks ---
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
        let w = self.style.width.unwrap_or(self.ui.available_width.max(100.0));

        let h = self.style.height.unwrap_or_else(|| {
            self.track_h.max(12.0) + self.style.padding.vertical()
        });

        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some(screen_rect) = self.ui.screen_layout_cache.get(&self.id) {
            (
                screen_rect.origin.x,
                screen_rect.origin.y,
                if screen_rect.size.width > 0.0 { screen_rect.size.width } else { w },
                if screen_rect.size.height > 0.0 { screen_rect.size.height } else { h },
            )
        } else if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                if rect.size.width > 0.0 { rect.size.width } else { w },
                if rect.size.height > 0.0 { rect.size.height } else { h },
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
        };

        let is_hovered = self.ui.is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);
        let is_active = is_hovered && self.ui.mouse_down;
        self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Hover(is_hovered));

        let start_draw = self.ui.draws.len();

        // 0. Background container (using BoxStyleExt)
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

        // 1. Draw Track
        let track_w = w - self.style.padding.horizontal();
        let track_x = x + self.style.padding.left;
        let ty = y + (h - self.track_h) / 2.0;

        let track_radius = self.track_radius.unwrap_or_else(|| self.style.gpu_radius());
        let track_shadow_color = self.track_shadow_color.map(|c| c.to_array()).unwrap_or([0.0; 4]);

        let (track_c1, track_c2, track_params) = if let Some(ref bg) = self.track_bg {
            let (c1, c2, _c3, _c4, params, _stops, _) = crate::style_ext::resolve_background(&Some(bg.clone()));
            (c1, c2, params)
        } else if let Some((s, e, d)) = self.track_gradient {
            let (t, a) = d.to_type_and_angle();
            (s.to_array(), e.to_array(), [t, a, 0.0, 0.0])
        } else {
            (self.track_color.to_array(), [0.0; 4], [0.0; 4])
        };

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [track_x, ty],
                size: [track_w, self.track_h],
                color: track_c1,
                color2: track_c2,
                gradient_params: track_params,
                radius: track_radius,
                opacity: 1.0,
                shadow_color: track_shadow_color,
                shadow_offset: self.track_shadow_offset,
                shadow_blur: self.track_shadow_blur,
                ..Default::default()
            }
        }));

        // 2. Draw Fill
        let fill_w = (track_w * self.value).clamp(0.0, track_w);
        if fill_w > 0.1 {
            let fill_radius = self.fill_radius.unwrap_or(track_radius);
            let fill_shadow_color = self.fill_shadow_color.map(|c| c.to_array()).unwrap_or([0.0; 4]);

            let (fill_c1, fill_c2, fill_params) = if let Some(ref bg) = self.fill_bg {
                let (c1, c2, _c3, _c4, params, _stops, _) = crate::style_ext::resolve_background(&Some(bg.clone()));
                (c1, c2, params)
            } else if let Some((s, e, d)) = self.fill_gradient {
                let (t, a) = d.to_type_and_angle();
                (s.to_array(), e.to_array(), [t, a, 0.0, 0.0])
            } else {
                (self.fill_color.to_array(), [0.0; 4], [0.0; 4])
            };

            let mut brightness = 1.0;
            if is_active {
                brightness = 1.25;
            } else if is_hovered {
                brightness = 1.1;
            }

            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [track_x, ty],
                    size: [fill_w, self.track_h],
                    color: fill_c1,
                    color2: fill_c2,
                    gradient_params: fill_params,
                    radius: fill_radius,
                    opacity: self.style.opacity,
                    shadow_color: fill_shadow_color,
                    shadow_offset: self.fill_shadow_offset,
                    shadow_blur: self.fill_shadow_blur,
                    brightness,
                    ..Default::default()
                }
            }));

            // 3. Shimmer Effect
            if self.shimmer {
                let shimmer_t = (self.ui.elapsed_time * 1.5) % 2.0 - 0.5; // Moves from -0.5 to 1.5
                let shimmer_x = track_x + shimmer_t * track_w;
                let shimmer_w = track_w * 0.2;

                if shimmer_x + shimmer_w > track_x && shimmer_x < track_x + fill_w {
                    // Clip shimmer to the fill bar
                    let actual_shimmer_x = shimmer_x.max(track_x);
                    let actual_shimmer_w = (shimmer_x + shimmer_w).min(track_x + fill_w) - actual_shimmer_x;

                    if actual_shimmer_w > 0.0 {
                        self.ui.draws.push(DrawCommand::Rect(RectDraw {
                            instance: RectInstance {
                                pos: [actual_shimmer_x, ty],
                                size: [actual_shimmer_w, self.track_h],
                                color: [1.0, 1.0, 1.0, 0.2],
                                radius: fill_radius,
                                brightness: 1.5,
                                ..Default::default()
                            }
                        }));
                        self.ui.request_redraw(); // Keep animating
                    }
                }
            }
        }

        self.ui.record_layout(self.id, zenthra_core::Rect::new(x, y, w, h));
        self.ui.advance(w, h, start_draw);

        Response {
            clicked: self.ui.clicked && is_hovered,
            hovered: is_hovered,
            pressed: is_active,
            submitted: false,
            changed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::Ui;
    use std::collections::HashMap;

    #[test]
    fn test_progress_bar_box_style_parity() {
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

        let pb = ui.progress_bar(0.65)
            .width(300.0)
            .height(24.0)
            .margin_all(5.0)
            .padding_all(6.0)
            .radius_all(12.0)
            .track_radius_all(6.0)
            .track_height(10.0)
            .fill_radius_all(4.0)
            .hover(crate::macros::Style::bg(Color::rgb(0.2, 0.2, 0.3)))
            .active(crate::macros::Style::bg(Color::rgb(0.3, 0.3, 0.4)));

        assert_eq!(pb.value, 0.65);
        assert_eq!(pb.style.width, Some(300.0));
        assert_eq!(pb.style.height, Some(24.0));
        assert_eq!(pb.style.margin, EdgeInsets::all(5.0));
        assert_eq!(pb.style.padding, EdgeInsets::all(6.0));
        assert_eq!(pb.style.radius, [12.0; 4]);
        assert_eq!(pb.track_radius, Some([6.0; 4]));
        assert_eq!(pb.track_h, 10.0);
        assert_eq!(pb.fill_radius, Some([4.0; 4]));
        assert!(pb.hover_state.is_some());
        assert!(pb.active_state.is_some());
    }
}
