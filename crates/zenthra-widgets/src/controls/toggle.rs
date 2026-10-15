// crates/zenthra-widgets/src/controls/toggle.rs

use crate::style_ext::BoxStyleExt;
use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{Background, BoxStyle, Color, EdgeInsets, Id, Rect, Response, Role, SemanticNode, Styleable};
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

    // Track Box & Typography Styles
    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,

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

    // External Label
    label_gap: f32,

    // Premium Effects
    glow: bool,
    hover_scale: f32,
    pressed_scale: f32,

    // State
    disabled: bool,
    animation_speed: f32,
    is_pill: bool,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,
}

impl<'u, 'a, 'b> Styleable for ToggleBuilder<'u, 'a, 'b> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

// Implement box dimensions, corner radius, and visuals
impl_dimensions!(ToggleBuilder, 'u, 'a, 'b);
impl_radius!(ToggleBuilder, 'u, 'a, 'b);
impl_visuals!(ToggleBuilder, 'u, 'a, 'b);

// Implement typography methods for the external label
impl_text_style_methods!(ToggleBuilder, 'u, 'a, 'b);

impl<'u, 'a, 'b> ToggleBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, state: &'b mut bool, label: Option<&str>) -> Self {
        let id = ui.id();

        let mut style = BoxStyle::new();
        style.width = Some(48.0);
        style.height = Some(24.0);
        style.radius = [12.0; 4];
        style.padding = EdgeInsets::all(3.0);
        style.shadow_offset = [0.0, 2.0];
        style.shadow_blur = 8.0;

        let mut text_style = zenthra_text::prelude::TextStyle::default();
        text_style.font_size = 14.0;
        text_style.color = Color::WHITE;

        Self {
            ui,
            state,
            id,
            label: label.map(|s| s.to_string()),
            label_side: LabelSide::Right,

            style,
            text_style,

            on_color: Color::rgb(0.0, 0.5, 1.0),
            off_color: Color::rgb(0.2, 0.2, 0.2),
            thumb_color: Color::WHITE,
            thumb_radius: None,
            thumb_width: None,

            inner_on: None,
            inner_off: None,
            inner_text_color: Color::WHITE,
            inner_font_size: 10.0,

            label_gap: 10.0,

            glow: false,
            hover_scale: 1.0,
            pressed_scale: 1.0,

            disabled: false,
            animation_speed: 15.0,
            is_pill: true,
            hover_state: None,
            active_state: None,
        }
    }

    #[inline]
    pub fn track_size(mut self, w: f32, h: f32) -> Self {
        self.style.width = Some(w);
        self.style.height = Some(h);
        self
    }

    #[inline]
    pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.style.padding = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }

    #[inline]
    pub fn padding_all(mut self, p: f32) -> Self {
        self.style.padding = EdgeInsets::all(p);
        self
    }

    #[inline]
    pub fn track_padding(self, p: f32) -> Self {
        self.padding_all(p)
    }

    #[inline]
    pub fn square(mut self) -> Self {
        self.style.radius = [0.0; 4];
        self.is_pill = false;
        self
    }

    #[inline]
    pub fn pill(mut self) -> Self {
        self.style.radius = [12.0; 4];
        self.is_pill = true;
        self
    }

    #[inline]
    pub fn inner_labels(mut self, on: &str, off: &str) -> Self {
        self.inner_on = Some(on.to_string());
        self.inner_off = Some(off.to_string());
        self
    }

    #[inline]
    pub fn label_left(mut self) -> Self {
        self.label_side = LabelSide::Left;
        self
    }

    #[inline]
    pub fn label_right(mut self) -> Self {
        self.label_side = LabelSide::Right;
        self
    }

    #[inline]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[inline]
    pub fn colors(mut self, on: Color, off: Color, thumb: Color) -> Self {
        self.on_color = on;
        self.off_color = off;
        self.thumb_color = thumb;
        self
    }

    #[inline]
    pub fn on_color(mut self, color: Color) -> Self {
        self.on_color = color;
        self
    }

    #[inline]
    pub fn off_color(mut self, color: Color) -> Self {
        self.off_color = color;
        self
    }

    #[inline]
    pub fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = color;
        self
    }

    #[inline]
    pub fn thumb_radius(mut self, r: f32) -> Self {
        self.thumb_radius = Some([r; 4]);
        self
    }

    #[inline]
    pub fn thumb_width(mut self, w: f32) -> Self {
        self.thumb_width = Some(w);
        self
    }

    #[inline]
    pub fn inner_color(mut self, color: Color) -> Self {
        self.inner_text_color = color;
        self
    }

    #[inline]
    pub fn inner_size(mut self, size: f32) -> Self {
        self.inner_font_size = size;
        self
    }

    #[inline]
    pub fn hover(mut self, props: impl crate::macros::IntoStateProps) -> Self {
        if self.hover_state.is_none() {
            self.hover_state = Some(crate::macros::WidgetStateOverrides::new());
        }
        if let Some(ref mut h) = self.hover_state {
            props.apply_to_overrides(h);
        }
        self
    }

    #[inline]
    pub fn active(mut self, props: impl crate::macros::IntoStateProps) -> Self {
        if self.active_state.is_none() {
            self.active_state = Some(crate::macros::WidgetStateOverrides::new());
        }
        if let Some(ref mut a) = self.active_state {
            props.apply_to_overrides(a);
        }
        self
    }

    #[inline]
    pub fn glow(mut self, enabled: bool) -> Self {
        self.glow = enabled;
        self
    }

    #[inline]
    pub fn scaling(mut self, hover: f32, pressed: f32) -> Self {
        self.hover_scale = hover;
        self.pressed_scale = pressed;
        self
    }

    #[inline]
    pub fn label_gap(mut self, gap: f32) -> Self {
        self.label_gap = gap;
        self
    }

    #[inline]
    pub fn animation_speed(mut self, speed: f32) -> Self {
        self.animation_speed = speed;
        self
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    pub fn on_click<F>(self, mut f: F) -> Self
    where
        F: FnMut() + 'a,
    {
        self.ui.add_listener(self.id, crate::ui::EventPhase::Bubble, move |_, event| {
            if let crate::ui::WidgetEvent::Click = event {
                f();
            }
        });
        self
    }

    pub fn on_change<F>(self, mut f: F) -> Self
    where
        F: FnMut(bool) + 'a,
    {
        self.ui.add_listener(self.id, crate::ui::EventPhase::Bubble, move |_, event| {
            if let crate::ui::WidgetEvent::Change(crate::ui::EventValue::Bool(val)) = event {
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
        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        let width = self.style.width.unwrap_or(48.0);
        let height = self.style.height.unwrap_or(24.0);

        // 1. Measure External Label
        let mut label_w = 0.0;
        let mut label_h = 0.0;
        if let Some(label_text) = &self.label {
            if let Some(fs) = self.ui.font_system.as_ref() {
                let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
                let options = self.text_style.to_options(0.0, 0.0, self.ui.scale_factor, self.ui.font_scale);
                let buffer = adapter.shape(label_text, &options);
                let (cw, ch) = buffer.content_size();
                label_w = cw;
                label_h = ch;
            }
        }

        let total_w = if label_w > 0.0 { width + self.label_gap + label_w } else { width };
        let total_h = height.max(label_h);

        // 2. Hit-testing
        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                rect.size.width.max(total_w),
                rect.size.height.max(total_h),
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, total_w, total_h)
        };

        let is_hovered = self.ui.is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h) && !self.disabled;
        let is_pressed = is_hovered && self.ui.mouse_down;
        let mut clicked = false;

        if is_hovered {
            self.ui.cursor_icon = crate::text::CursorIcon::Pointer;
        }

        self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Hover(is_hovered));

        if self.ui.clicked && is_hovered {
            *self.state = !*self.state;
            self.ui.needs_redraw = true;
            clicked = true;
            self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Click);
            self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Change(crate::ui::EventValue::Bool(*self.state)));
        }

        // 3. Animation (Generic & Selection)
        let sel_target = if *self.state { 1.0 } else { 0.0 };
        let current_sel = *self.ui.interaction_state.entry(self.id).or_insert(sel_target);

        let hover_id = Id::from_u64(self.id.raw().wrapping_add(3000000));
        let eff_hover_scale = self.hover_state.as_ref().map(|h| h.scale).filter(|&s| (s - 1.0).abs() > 0.001).unwrap_or(self.hover_scale);
        let eff_pressed_scale = self.active_state.as_ref().map(|a| a.scale).filter(|&s| (s - 1.0).abs() > 0.001).unwrap_or(self.pressed_scale);
        let hover_target = if is_pressed { eff_pressed_scale } else if is_hovered { eff_hover_scale } else { 1.0 };
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
        let base_w = width * final_scale;
        let base_h = height * final_scale;

        let current_radius = if self.is_pill && self.style.radius == [12.0; 4] {
            [base_h / 2.0; 4]
        } else {
            let mut r = self.style.radius;
            for i in 0..4 {
                r[i] *= final_scale;
            }
            r
        };

        let toggle_x = if let LabelSide::Left = self.label_side { x + label_w + self.label_gap } else { x };
        let toggle_y_orig = y + (total_h - height) / 2.0;

        // Midpoint of the original toggle track
        let mid_x = toggle_x + width / 2.0;
        let mid_y = toggle_y_orig + height / 2.0;

        // Final position for scaled track
        let tx = mid_x - base_w / 2.0;
        let ty = mid_y - base_h / 2.0;

        // 5. Draw Track
        let mut track_color = if final_pos > 0.5 { self.on_color } else { self.off_color };
        track_color.a *= self.style.opacity;
        if self.disabled {
            track_color.a *= 0.5;
        }

        let mut instance = self.style.build_rect_instance(
            tx,
            ty,
            base_w,
            base_h,
            Some(&Background::Solid(track_color)),
            None,
            None,
            1.0,
        );
        instance.radius = current_radius;
        if self.style.shadow_color.is_none() && self.glow {
            let mut sc = self.on_color.to_array();
            sc[3] *= 0.4 * final_pos;
            instance.shadow_color = sc;
            instance.shadow_blur = 12.0 * final_pos;
        }

        self.ui.draws.push(DrawCommand::Rect(RectDraw { instance }));

        // 6. Draw Inner Labels (clipped to track)
        let clip = [tx, ty, base_w, base_h];
        let padding = self.style.padding.top.max(self.style.padding.left);
        if let Some(on_text) = &self.inner_on {
            if let Some(off_text) = &self.inner_off {
                let (text, is_on) = if final_pos > 0.5 { (on_text, true) } else { (off_text, false) };

                let mut text_w = 0.0;
                let mut text_h = 0.0;
                if let Some(fs) = self.ui.font_system.as_ref() {
                    let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
                    let options = zenthra_text::prelude::TextOptions::new().size(self.inner_font_size * final_scale);
                    let buffer = adapter.shape(text, &options);
                    let (cw, ch) = buffer.content_size();
                    text_w = cw;
                    text_h = ch;
                }

                let thumb_h_adj = base_h - (padding * 2.0 * final_scale);
                let thumb_w_adj = self.thumb_width.map(|w| w * final_scale).unwrap_or(thumb_h_adj);
                let track_w_internal = base_w - thumb_w_adj - (padding * 2.0 * final_scale);

                let inner_tx = if is_on {
                    tx + (padding * final_scale) + (track_w_internal - text_w) / 2.0
                } else {
                    tx + (padding * final_scale) + thumb_w_adj + (track_w_internal - text_w) / 2.0
                };
                let inner_ty = ty + (base_h - text_h) / 2.0;

                let mut t_color = self.inner_text_color;
                let opacity = if is_on { (final_pos - 0.5) * 2.0 } else { (0.5 - final_pos) * 2.0 };
                t_color.a *= opacity.clamp(0.0, 1.0) * self.style.opacity;

                if t_color.a > 0.05 {
                    self.ui.draws.push(DrawCommand::Text(TextDraw {
                        text: text.clone(),
                        pos: [inner_tx, inner_ty],
                        options: zenthra_text::prelude::TextOptions::new()
                            .size(self.inner_font_size * final_scale)
                            .color(t_color),
                        clip,
                    }));
                }
            }
        }

        // 7. Draw Thumb
        let thumb_h_adj = base_h - (padding * 2.0 * final_scale);
        let thumb_w_adj = self.thumb_width.map(|w| w * final_scale).unwrap_or(thumb_h_adj);
        let move_range = base_w - thumb_w_adj - (padding * 2.0 * final_scale);
        let thumb_x = tx + (padding * final_scale) + (final_pos * move_range);
        let thumb_y = ty + (padding * final_scale);

        let t_radius = self.thumb_radius.map(|mut r| {
            for i in 0..4 {
                r[i] *= final_scale;
            }
            r
        }).unwrap_or_else(|| {
            let r = thumb_h_adj / 2.0;
            if self.is_pill {
                [r; 4]
            } else {
                let tr = self.style.radius[0] * final_scale;
                if tr < r { [tr; 4] } else { [r; 4] }
            }
        });

        let mut t_color = self.thumb_color;
        t_color.a *= self.style.opacity;
        if self.disabled {
            t_color.a *= 0.8;
        }

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [thumb_x, thumb_y],
                size: [thumb_w_adj, thumb_h_adj],
                color: t_color.to_array(),
                radius: t_radius,
                ..Default::default()
            },
        }));

        // 8. Draw External Label (Color Transition)
        if let Some(label_text) = &self.label {
            let lx = if let LabelSide::Left = self.label_side { x } else { toggle_x + width + self.label_gap };
            let ly = y + (total_h - label_h) / 2.0;

            let mut l_color = self.text_style.color;
            if final_pos > 0.5 {
                if let Some(c) = self.active_state.as_ref().and_then(|a| a.text_color) {
                    l_color = c;
                }
            }
            l_color.a *= self.style.opacity;

            let mut text_opts = self.text_style.to_options(lx, ly, self.ui.scale_factor, self.ui.font_scale);
            text_opts.color = l_color;

            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: label_text.clone(),
                pos: [lx, ly],
                options: text_opts,
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
            submitted: false,
            changed: clicked,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_toggle_box_styling_and_label() {
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

        let mut active = false;

        // 1. Unchecked Toggle with shadow and custom dimensions
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

            ui.toggle(&mut active, "Airplane Mode")
                .width(52.0)
                .height(26.0)
                .padding_all(4.0)
                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.4), 0.0, 2.0, 6.0)
                .show();

            // 1 RectDraw for Track, 1 RectDraw for Thumb, 1 TextDraw for Label
            assert_eq!(ui.draws.len(), 3);
            if let DrawCommand::Rect(track) = &ui.draws[0] {
                assert_eq!(track.instance.size, [52.0, 26.0]);
                assert_eq!(track.instance.shadow_color, [0.0, 0.0, 0.0, 0.4]);
            } else {
                panic!("Expected RectDraw for track");
            }

            if let DrawCommand::Rect(thumb) = &ui.draws[1] {
                assert_eq!(thumb.instance.size, [18.0, 18.0]); // 26 - 4*2 = 18
            } else {
                panic!("Expected RectDraw for thumb");
            }
        }

        // 2. Unified IntoBorder and typography styling on ToggleBuilder
        {
            let mut active = true;
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

            let t = ui.toggle(&mut active, "Dark Mode")
                .track_size(60.0, 30.0)
                .border((Color::GREEN, 2.0, zenthra_core::BorderAlignment::Inside))
                .size(16.0)
                .color(Color::YELLOW)
                .font_weight(600);

            assert_eq!(t.style.width, Some(60.0));
            assert_eq!(t.style.height, Some(30.0));
            assert_eq!(t.style.border, zenthra_core::Border::Solid(Color::GREEN, 2.0));
            assert_eq!(t.style.border_alignment, zenthra_core::BorderAlignment::Inside);
            assert_eq!(t.text_style.font_size, 16.0);
            assert_eq!(t.text_style.color, Color::YELLOW);
            assert_eq!(t.text_style.font_weight, zenthra_text::prelude::FontWeight::SemiBold);
        }
    }
}
