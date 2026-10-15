// crates/zenthra-widgets/src/controls/radio.rs

use crate::style_ext::BoxStyleExt;
use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{Background, Border, BoxStyle, Color, Id, Rect, Response, Role, SemanticNode, Styleable};
use zenthra_render::RectInstance;

pub struct RadioBuilder<'u, 'a, 'b, T: PartialEq + Clone> {
    ui: &'u mut Ui<'a>,
    state: &'b mut T,
    value: T,
    label: String,
    id: Id,

    // Box Style for the outer ring & Typography for label
    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,

    // Layout
    gap: f32,

    // Radio dot & ring overrides
    dot_color: Color,

    // Premium Effects
    glow: bool,
    hover_scale: f32,
    pressed_scale: f32,

    // Interaction
    disabled: bool,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,
}

impl<'u, 'a, 'b, T: PartialEq + Clone> Styleable for RadioBuilder<'u, 'a, 'b, T> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

impl<'u, 'a, 'b, T: PartialEq + Clone> RadioBuilder<'u, 'a, 'b, T> {
    pub fn new(ui: &'u mut Ui<'a>, state: &'b mut T, value: T, label: &str) -> Self {
        let id = ui.id();

        let mut style = BoxStyle::new();
        style.width = Some(18.0);
        style.height = Some(18.0);
        style.radius = [9.0; 4];
        style.bg = Some(Background::Solid(Color::TRANSPARENT));
        style.border = Border::Solid(Color::rgb(0.4, 0.4, 0.4), 1.5);
        style.shadow_offset = [0.0, 2.0];
        style.shadow_blur = 8.0;

        let mut text_style = zenthra_text::prelude::TextStyle::default();
        text_style.font_size = 14.0;
        text_style.color = Color::WHITE;

        Self {
            ui,
            state,
            value,
            label: label.to_string(),
            id,

            style,
            text_style,

            gap: 10.0,
            dot_color: Color::rgb(0.2, 0.5, 1.0),

            glow: false,
            hover_scale: 1.0,
            pressed_scale: 1.0,
            disabled: false,
            hover_state: None,
            active_state: None,
        }
    }

    #[inline]
    pub fn size(mut self, size: f32) -> Self {
        self.style.width = Some(size);
        self.style.height = Some(size);
        self.style.radius = [size / 2.0; 4];
        self
    }

    #[inline]
    pub fn width(mut self, w: f32) -> Self {
        self.style.width = Some(w);
        self
    }

    #[inline]
    pub fn height(mut self, h: f32) -> Self {
        self.style.height = Some(h);
        self
    }

    #[inline]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    #[inline]
    pub fn colors(mut self, ring: Color, dot: Color) -> Self {
        self.style.border = Border::Solid(ring, self.style.border_width().max(1.5));
        self.dot_color = dot;
        self
    }

    #[inline]
    pub fn ring_color(mut self, color: Color) -> Self {
        self.style.border = Border::Solid(color, self.style.border_width().max(1.5));
        self
    }

    #[inline]
    pub fn dot_color(mut self, color: Color) -> Self {
        self.dot_color = color;
        self
    }

    #[inline]
    pub fn bg(mut self, bg: impl Into<Background>) -> Self {
        match bg.into() {
            Background::None => self.style.bg = None,
            other => self.style.bg = Some(other),
        }
        self
    }

    #[inline]
    pub fn border(mut self, border: impl Into<Border>) -> Self {
        self.style.border = border.into();
        self
    }

    #[inline]
    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.style.shadow_color = Some(color);
        self.style.shadow_offset = [x, y];
        self.style.shadow_blur = blur;
        self
    }

    #[inline]
    pub fn color(mut self, color: Color) -> Self {
        self.text_style.color = color;
        self
    }

    #[inline]
    pub fn label_style(mut self, size: f32, color: Color) -> Self {
        self.text_style.font_size = size;
        self.text_style.color = color;
        self
    }

    #[inline]
    pub fn label_size(mut self, size: f32) -> Self {
        self.text_style.font_size = size;
        self
    }

    #[inline]
    pub fn label_color(mut self, color: Color) -> Self {
        self.text_style.color = color;
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
    pub fn font_weight(mut self, weight: impl Into<zenthra_text::prelude::FontWeight>) -> Self {
        self.text_style.font_weight = weight.into();
        self
    }

    #[inline]
    pub fn decoration(mut self, d: impl crate::IntoDecoration) -> Self {
        d.apply_decoration(&mut self.text_style);
        self
    }

    #[inline]
    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.text_style.font_family = Some(family.into());
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
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = self.ui.id_from(id);
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
        T: 'a,
        F: FnMut(&T) + 'a,
    {
        let val = self.value.clone();
        self.ui.add_listener(self.id, crate::ui::EventPhase::Bubble, move |_, event| {
            if let crate::ui::WidgetEvent::Click = event {
                f(&val);
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
        let is_active = *self.state == self.value;
        let box_w = self.style.width.unwrap_or(18.0);
        let box_h = self.style.height.unwrap_or(18.0);

        // 1. Measure Label
        let mut label_w = 0.0;
        let mut label_h = 0.0;
        if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let options = zenthra_text::prelude::TextOptions::new().size(self.text_style.font_size);
            let buffer = adapter.shape(&self.label, &options);
            let (cw, ch) = buffer.content_size();
            label_w = cw;
            label_h = ch;
        }

        let total_w = box_w + self.gap + label_w;
        let total_h = box_h.max(label_h);

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
            if !is_active {
                *self.state = self.value.clone();
                self.ui.needs_redraw = true;
                clicked = true;
                self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Click);
            }
        }

        // 3. Animation (Generic & Selection)
        let sel_target = if is_active { 1.0 } else { 0.0 };
        let current_sel = *self.ui.interaction_state.entry(self.id).or_insert(sel_target);

        let hover_id = Id::from_u64(self.id.raw().wrapping_add(1000000));
        let eff_hover_scale = self.hover_state.as_ref().map(|h| h.scale).filter(|&s| (s - 1.0).abs() > 0.001).unwrap_or(self.hover_scale);
        let eff_pressed_scale = self.active_state.as_ref().map(|a| a.scale).filter(|&s| (s - 1.0).abs() > 0.001).unwrap_or(self.pressed_scale);
        let hover_target = if is_pressed { eff_pressed_scale } else if is_hovered { eff_hover_scale } else { 1.0 };
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
            final_scale += h_delta * 0.5;
            self.ui.interaction_state.insert(hover_id, final_scale);
            self.ui.needs_redraw = true;
        } else {
            final_scale = hover_target;
        }

        let start_draw = self.ui.draws.len();

        // Visual Midpoints
        let mid_x = x + box_w / 2.0;
        let mid_y = y + (total_h / 2.0);

        // Scaled dimensions
        let base_w = box_w * final_scale;
        let base_h = box_h * final_scale;
        let bx = mid_x - base_w / 2.0;
        let by = mid_y - base_h / 2.0;
        let radius = [base_w / 2.0; 4];

        // 4. Draw Ring
        let active_ring_color = if is_active {
            self.dot_color
        } else {
            self.style.border_color().unwrap_or(Color::rgb(0.4, 0.4, 0.4))
        };
        let mut r_color = active_ring_color;
        if self.disabled {
            r_color.a *= 0.5;
        }

        let mut instance = self.style.build_rect_instance(
            bx,
            by,
            base_w,
            base_h,
            None,
            Some(r_color),
            None,
            1.0,
        );
        instance.radius = radius;
        if is_hovered {
            instance.brightness = 1.2;
        }
        if self.style.shadow_color.is_none() && self.glow && is_active {
            let mut sc = self.dot_color.to_array();
            sc[3] *= 0.4 * final_sel;
            instance.shadow_color = sc;
            instance.shadow_blur = 10.0 * final_sel;
        }

        self.ui.draws.push(DrawCommand::Rect(RectDraw { instance }));

        // 5. Draw Dot (Animated Scale)
        if final_sel > 0.01 {
            let dot_size = base_w * 0.5 * final_sel;
            let dx = bx + (base_w - dot_size) / 2.0;
            let dy = by + (base_h - dot_size) / 2.0;

            let mut d_color = self.dot_color;
            d_color.a *= final_sel;
            if self.disabled {
                d_color.a *= 0.5;
            }

            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [dx, dy],
                    size: [dot_size, dot_size],
                    color: d_color.to_array(),
                    radius: [dot_size / 2.0; 4],
                    ..Default::default()
                },
            }));
        }

        // 6. Draw Label (Color Transition)
        let lx = x + box_w + self.gap;
        let ly = y + (total_h - label_h) / 2.0;

        let mut l_color = self.text_style.color;
        if final_sel > 0.5 {
            if let Some(c) = self.active_state.as_ref().and_then(|a| a.text_color) {
                l_color = c;
            }
        }

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [lx, ly],
            options: zenthra_text::prelude::TextOptions::new()
                .size(self.text_style.font_size)
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
    fn test_radio_styling_and_state() {
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

        let mut choice = 1;

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

        // Active option
        ui.radio(&mut choice, 1, "Option 1")
            .size(20.0)
            .colors(Color::rgb(0.3, 0.3, 0.3), Color::rgb(0.2, 0.7, 1.0))
            .shadow(Color::rgba(0.0, 0.0, 0.0, 0.3), 0.0, 2.0, 4.0)
            .show();

        // 1 RectDraw for Ring, 1 RectDraw for Dot (active), 1 TextDraw for Label
        assert_eq!(ui.draws.len(), 3);
        if let DrawCommand::Rect(ring) = &ui.draws[0] {
            assert_eq!(ring.instance.size, [20.0, 20.0]);
            assert_eq!(ring.instance.border_color, Color::rgb(0.2, 0.7, 1.0).to_array());
            assert_eq!(ring.instance.shadow_color, [0.0, 0.0, 0.0, 0.3]);
        } else {
            panic!("Expected RectDraw for radio ring");
        }
    }
}
