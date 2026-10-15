// crates/zenthra-widgets/src/controls/checkbox.rs

use crate::style_ext::BoxStyleExt;
use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{Background, Border, BoxStyle, Color, Id, Rect, Response, Role, SemanticNode, Styleable};

pub struct CheckboxBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    value: &'b mut bool,
    label: String,
    id: Id,

    // Box & Typography Styles
    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,

    // Layout
    gap: f32,

    // Visuals (Checked)
    check_bg: Color,
    check_color: Color,

    // Premium Effects
    glow: bool,
    hover_scale: f32,
    pressed_scale: f32,

    // Interaction
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,
}

impl<'u, 'a, 'b> Styleable for CheckboxBuilder<'u, 'a, 'b> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

// Single-source macro generates all BoxStyle builder methods (.width, .height, .padding, .radius, .bg, .border, .shadow, etc.)
impl_box_style_methods!(CheckboxBuilder, 'u, 'a, 'b);

// Single-source macro generates all TextStyle builder methods (.font_size, .text_color, .font_family, .font_weight, .wrap, etc.)
impl_text_style_methods!(CheckboxBuilder, 'u, 'a, 'b);

// Interaction states (.hover([ ... ]), .active([ ... ]))
impl_interaction_state_methods!(CheckboxBuilder, 'u, 'a, 'b);

impl<'u, 'a, 'b> CheckboxBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, value: &'b mut bool, label: &str) -> Self {
        let id = ui.id();

        let mut style = BoxStyle::new();
        style.width = Some(18.0);
        style.height = Some(18.0);
        style.radius = [4.0; 4];
        style.bg = Some(Background::Solid(Color::rgb(0.15, 0.15, 0.15)));
        style.border = Border::Solid(Color::rgb(0.3, 0.3, 0.3), 1.0);
        style.shadow_offset = [0.0, 2.0];
        style.shadow_blur = 8.0;

        let mut text_style = zenthra_text::prelude::TextStyle::default();
        text_style.font_size = 14.0;
        text_style.color = Color::WHITE;

        Self {
            ui,
            value,
            label: label.to_string(),
            id,

            style,
            text_style,
            gap: 8.0,

            check_bg: Color::rgb(0.2, 0.5, 1.0),
            check_color: Color::WHITE,

            glow: false,
            hover_scale: 1.0,
            pressed_scale: 1.0,

            hover_state: None,
            active_state: None,
        }
    }

    #[inline]
    pub fn box_size(mut self, size: f32) -> Self {
        self.style.width = Some(size);
        self.style.height = Some(size);
        self
    }

    #[inline]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    #[inline]
    pub fn check_bg(mut self, bg: impl Into<Option<Color>>) -> Self {
        self.check_bg = bg.into().unwrap_or(Color::TRANSPARENT);
        self
    }

    #[inline]
    pub fn check_color(mut self, color: Color) -> Self {
        self.check_color = color;
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
        let box_w = self.style.width.unwrap_or(18.0);
        let box_h = self.style.height.unwrap_or(18.0);

        // 1. Measure Text
        let mut text_w = 0.0;
        let mut text_h = 0.0;
        if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let options = self.text_style.to_options(0.0, 0.0, self.ui.scale_factor, self.ui.font_scale);
            let buffer = adapter.shape(&self.label, &options);
            let (cw, ch) = buffer.content_size();
            text_w = cw;
            text_h = ch;
        }

        let total_w = box_w + self.gap + text_w;
        let total_h = box_h.max(text_h);

        // 2. Hit-testing (Use recorded layout for accurate positioning in containers)
        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                if rect.size.width > 0.0 { rect.size.width } else { total_w },
                if rect.size.height > 0.0 { rect.size.height } else { total_h }
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, total_w, total_h)
        };

        let is_hovered = self.ui.is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);
        let is_pressed = is_hovered && self.ui.mouse_down;
        let mut clicked = false;

        if is_hovered {
            self.ui.cursor_icon = crate::text::CursorIcon::Pointer;
        }

        self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Hover(is_hovered));

        if self.ui.clicked && is_hovered {
            *self.value = !*self.value;
            self.ui.needs_redraw = true;
            clicked = true;
            self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Click);
            self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Change(crate::ui::EventValue::Bool(*self.value)));
        }

        // 3. Animation (Generic & Selection)
        let sel_target = if *self.value { 1.0 } else { 0.0 };
        let current_sel = *self.ui.interaction_state.entry(self.id).or_insert(sel_target);

        let hover_id = Id::from_u64(self.id.raw().wrapping_add(2000000));
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
            final_scale += h_delta * 0.3;
            self.ui.interaction_state.insert(hover_id, final_scale);
            self.ui.needs_redraw = true;
        } else {
            final_scale = hover_target;
        }

        // Color Logic
        let unchecked_bg = self.style.bg.clone().unwrap_or(Background::Solid(Color::TRANSPARENT));
        let mut current_bg = if *self.value {
            Background::Solid(self.check_bg)
        } else {
            unchecked_bg
        };
        if is_pressed {
            if let Some(ref a) = self.active_state {
                if let Some(ref bg) = a.style.bg {
                    current_bg = bg.clone();
                }
            }
        } else if is_hovered {
            if let Some(ref h) = self.hover_state {
                if let Some(ref bg) = h.style.bg {
                    current_bg = bg.clone();
                }
            }
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

        // 4. Draw Box via build_rect_instance
        let mut instance = self.style.build_rect_instance(
            bx,
            by,
            base_w,
            base_h,
            Some(&current_bg),
            None,
            None,
            1.0,
        );
        if self.style.shadow_color.is_none() && self.glow && *self.value {
            let mut sc = self.check_bg.to_array();
            sc[3] *= 0.4 * final_sel;
            instance.shadow_color = sc;
            instance.shadow_blur = 10.0 * final_sel;
        }
        instance.radius = [
            self.style.radius[3] * final_scale,
            self.style.radius[2] * final_scale,
            self.style.radius[1] * final_scale,
            self.style.radius[0] * final_scale,
        ];

        self.ui.draws.push(DrawCommand::Rect(RectDraw { instance }));

        // 5. Draw Check Indicator (Unicode Checkmark)
        if *self.value {
            let check_str = "✓";
            let mut check_w = 0.0;
            let mut check_h = 0.0;
            let check_font_size = box_w * 0.8;

            if let Some(fs) = self.ui.font_system.as_ref() {
                let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
                let options = zenthra_text::prelude::TextOptions::new().size(check_font_size);
                let buffer = adapter.shape(check_str, &options);
                let (cw, ch) = buffer.content_size();
                check_w = cw;
                check_h = ch;
            }

            let cx = x + (box_w - check_w) / 2.0;
            let cy = y + (total_h - check_h) / 2.0;

            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: check_str.to_string(),
                pos: [cx, cy],
                options: zenthra_text::prelude::TextOptions::new()
                    .size(check_font_size)
                    .color(self.check_color),
                clip: [0.0, 0.0, 9999.0, 9999.0],
            }));
        }

        // 6. Draw Label (Color Transition)
        let lx = x + box_w + self.gap;
        let ly = y + (total_h - text_h) / 2.0;

        let mut l_color = self.text_style.color;
        if final_sel > 0.5 {
            if let Some(c) = self.active_state.as_ref().and_then(|a| a.text_color) {
                l_color = c;
            }
        }

        let mut text_opts = self.text_style.to_options(lx, ly, self.ui.scale_factor, self.ui.font_scale);
        text_opts.color = l_color;

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [lx, ly],
            options: text_opts,
            clip: [0.0, 0.0, 9999.0, 9999.0],
        }));

        // 7. Register Semantic & Advance
        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::CheckBox, Rect::new(x, y, total_w, total_h))
                .with_label(self.label.clone())
                .with_value(if *self.value { 1.0 } else { 0.0 }),
        );

        self.ui.record_layout(self.id, Rect::new(x, y, total_w, total_h));
        self.ui.advance(total_w, total_h, start_draw);

        Response {
            clicked,
            hovered: is_hovered,
            pressed: is_pressed,
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
    fn test_checkbox_styling_and_shadow() {
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

        let mut checked = false;

        // 1. Unchecked Checkbox with shadow and border
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

            ui.checkbox(&mut checked, "Enable Shadows")
                .bg(Color::rgb(0.12, 0.12, 0.16))
                .border((Color::rgb(0.3, 0.35, 0.45), 1.0))
                .radius_all(4.0)
                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.5), 0.0, 2.0, 6.0)
                .show();

            // Draws: 1 RectDraw (box), 1 TextDraw (label)
            assert_eq!(ui.draws.len(), 2);
            if let DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.color, Color::rgb(0.12, 0.12, 0.16).to_array());
                assert_eq!(rd.instance.border_width, 1.0);
                assert_eq!(rd.instance.border_color, Color::rgb(0.3, 0.35, 0.45).to_array());
                assert_eq!(rd.instance.radius, [4.0, 4.0, 4.0, 4.0]);
                assert_eq!(rd.instance.shadow_color, [0.0, 0.0, 0.0, 0.5]);
            } else {
                panic!("Expected RectDraw for checkbox box");
            }
        }

        // 2. Checked Checkbox with bg(None) and custom check colors
        {
            let mut checked_true = true;
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

            ui.checkbox(&mut checked_true, "Active Feature")
                .bg(None)
                .check_bg(Color::rgb(0.2, 0.6, 1.0))
                .check_color(Color::WHITE)
                .show();

            // Draws: 1 RectDraw (box), 1 TextDraw (checkmark), 1 TextDraw (label)
            assert_eq!(ui.draws.len(), 3);
            if let DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.color, Color::rgb(0.2, 0.6, 1.0).to_array());
            } else {
                panic!("Expected RectDraw for checkmark box");
            }
        }

        // 3. Unified IntoBorder and typography styling on CheckboxBuilder
        {
            let mut checked = false;
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

            let cb = ui.checkbox(&mut checked, "Accept Terms")
                .box_size(20.0)
                .gap(12.0)
                .border((Color::GREEN, 2.0, zenthra_core::BorderAlignment::Inside))
                .size(16.0)
                .color(Color::YELLOW)
                .font_weight(600);

            assert_eq!(cb.style.width, Some(20.0));
            assert_eq!(cb.style.height, Some(20.0));
            assert_eq!(cb.gap, 12.0);
            assert_eq!(cb.style.border, zenthra_core::Border::Solid(Color::GREEN, 2.0));
            assert_eq!(cb.style.border_alignment, zenthra_core::BorderAlignment::Inside);
            assert_eq!(cb.text_style.font_size, 16.0);
            assert_eq!(cb.text_style.color, Color::YELLOW);
            assert_eq!(cb.text_style.font_weight, zenthra_text::prelude::FontWeight::SemiBold);
        }
    }
}
