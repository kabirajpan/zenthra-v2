// crates/zenthra-widgets/src/button.rs

use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{BoxStyle, Color, EdgeInsets, Id, Rect, Role, SemanticNode, Styleable};
use crate::style_ext::BoxStyleExt;

pub struct ButtonBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,
    label: String,

    // Position & Layout
    pos: Option<(f32, f32)>,
    pub style: BoxStyle,

    // Typography
    pub text_style: zenthra_text::prelude::TextStyle,

    // States
    hover_bg: Option<zenthra_core::Background>,
    active_bg: Option<zenthra_core::Background>,

    // Other
    render_mode: Option<zenthra_core::RenderMode>,
    hover_scale: f32,
    hover_border_color: Option<Color>,
    hover_border_width: Option<f32>,
    active_border_color: Option<Color>,
    active_border_width: Option<f32>,
    align: zenthra_core::Align,
}

impl<'u, 'a> Styleable for ButtonBuilder<'u, 'a> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

// Single-source macro generates all BoxStyle builder methods (.width, .height, .padding, .radius, .bg, .border, .shadow, .style, etc.)
impl_box_style_methods!(ButtonBuilder);

// Single-source macro generates all TextStyle builder methods (.font_size, .color, .family, .weight, .wrap, etc.)
impl_text_style_methods!(ButtonBuilder);

impl<'u, 'a> ButtonBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, label: &str) -> Self {
        let id = ui.id();
        let mut style = BoxStyle::new();
        style.padding = EdgeInsets::symmetric(6.0, 12.0);
        style.bg = Some(zenthra_core::Background::Solid(Color::rgb(0.2, 0.2, 0.25)));
        style.radius = [4.0; 4];

        let mut text_style = zenthra_text::prelude::TextStyle::default();
        text_style.font_size = 14.0;
        text_style.color = Color::WHITE;
        text_style.wrap = zenthra_text::prelude::TextWrap::Word;

        Self {
            ui,
            id,
            label: label.to_string(),
            pos: None,
            style,
            text_style,
            hover_bg: None,
            active_bg: None,
            render_mode: None,
            hover_scale: 1.0,
            hover_border_color: None,
            hover_border_width: None,
            active_border_color: None,
            active_border_width: None,
            align: zenthra_core::Align::Center,
        }
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.pos = Some((x, y));
        self
    }

    pub fn hover_bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self {
        self.hover_bg = Some(bg.into());
        self
    }

    pub fn active_bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self {
        self.active_bg = Some(bg.into());
        self
    }

    pub fn hover_scale(mut self, s: f32) -> Self {
        self.hover_scale = s;
        self
    }

    pub fn hover_border(mut self, border: impl Into<zenthra_core::Border>) -> Self {
        match border.into() {
            zenthra_core::Border::None => {
                self.hover_border_color = Some(Color::TRANSPARENT);
                self.hover_border_width = Some(0.0);
            }
            zenthra_core::Border::Solid(c, w) => {
                self.hover_border_color = Some(c);
                self.hover_border_width = Some(w);
            }
        }
        self
    }

    pub fn active_border(mut self, border: impl Into<zenthra_core::Border>) -> Self {
        match border.into() {
            zenthra_core::Border::None => {
                self.active_border_color = Some(Color::TRANSPARENT);
                self.active_border_width = Some(0.0);
            }
            zenthra_core::Border::Solid(c, w) => {
                self.active_border_color = Some(c);
                self.active_border_width = Some(w);
            }
        }
        self
    }

    pub fn render_mode(mut self, mode: zenthra_core::RenderMode) -> Self {
        self.render_mode = Some(mode);
        self
    }

    pub fn align(mut self, strategy: zenthra_core::Align) -> Self {
        self.align = strategy;
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

    pub fn show(self) -> zenthra_core::Response {
        if let Some(mode) = self.render_mode {
            self.ui.render_mode_stack.push(mode);
        }

        let (x, y) = self.pos.unwrap_or((self.ui.cursor_x, self.ui.cursor_y));

        // 1. Measure text FIRST to get true content dimensions (no magic fallbacks)
        let mut text_w = 0.0;
        let mut text_h = 0.0;
        if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter =
                zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let options = self.text_style.to_options(0.0, 0.0, self.ui.scale_factor, self.ui.font_scale);
            let buffer = adapter.shape(&self.label, &options);
            let (cw, ch) = buffer.content_size();
            text_w = cw;
            text_h = ch;
        }

        let mut final_w = if self.style.fill_x {
            self.ui.available_width
        } else {
            self.style.width.unwrap_or(text_w + self.style.padding.horizontal())
        };
        let mut final_h = if self.style.fill_y {
            (self.ui.max_y - y).max(self.style.height.unwrap_or(text_h + self.style.padding.vertical()))
        } else {
            self.style.height.unwrap_or(text_h + self.style.padding.vertical())
        };

        if let Some(min_w) = self.style.min_width { final_w = final_w.max(min_w); }
        if let Some(max_w) = self.style.max_width { final_w = final_w.min(max_w); }
        if let Some(min_h) = self.style.min_height { final_h = final_h.max(min_h); }
        if let Some(max_h) = self.style.max_height { final_h = final_h.min(max_h); }

        // 2. Hit-testing with recorded layout support
        let (actual_ox, actual_oy) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (rect.origin.x + self.ui.offset_x, rect.origin.y + self.ui.offset_y)
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y)
        };

        let is_hovered = self.ui.is_hovered(
            self.id,
            actual_ox,
            actual_oy,
            final_w,
            final_h,
        );

        let is_pressed = if self.ui.mouse_down {
            if self.ui.active_widget_id == Some(self.id) {
                true
            } else if self.ui.active_widget_id.is_none() && is_hovered {
                self.ui.active_widget_id = Some(self.id);
                true
            } else {
                false
            }
        } else {
            false
        };

        if is_hovered {
            self.ui.cursor_icon = crate::text::CursorIcon::Pointer;
        }

        let mut clicked = false;
        if self.ui.clicked && is_hovered {
            clicked = true;
            self.ui.consume_click();
        }

        // Dispatch events
        self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Hover(is_hovered));

        if clicked {
            self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Click);
        }

        // 3. Determine effective background and border based on state
        let effective_bg = if is_pressed && self.active_bg.is_some() {
            self.active_bg.as_ref()
        } else if is_hovered && self.hover_bg.is_some() {
            self.hover_bg.as_ref()
        } else {
            self.style.bg.as_ref()
        };

        let effective_border_w = if is_pressed {
            self.active_border_width.unwrap_or_else(|| self.style.border_width())
        } else if is_hovered {
            self.hover_border_width.unwrap_or_else(|| self.style.border_width())
        } else {
            self.style.border_width()
        };

        let effective_border_c = if is_pressed {
            self.active_border_color.or_else(|| self.style.border_color())
        } else if is_hovered {
            self.hover_border_color.or_else(|| self.style.border_color())
        } else {
            self.style.border_color()
        };

        if is_hovered {
            final_w *= self.hover_scale;
            final_h *= self.hover_scale;
        }

        let start_draw = self.ui.draws.len();

        // 4. Draw Background
        let rect_inst = self.style.build_rect_instance(
            x,
            y,
            final_w,
            final_h,
            effective_bg,
            effective_border_c,
            Some(effective_border_w),
            1.0,
        );
        self.ui.draws.push(DrawCommand::Rect(RectDraw { instance: rect_inst }));

        // 5. Draw Text (aligned inside allocated button bounds)
        let tx = match self.align {
            zenthra_core::Align::Left => (x + self.style.padding.left).round(),
            zenthra_core::Align::Right => (x + final_w - text_w - self.style.padding.right).round(),
            _ => (x + (final_w - text_w) / 2.0).round(),
        };
        let ty = (y + (final_h - text_h) / 2.0).round();

        let mut text_opts = self.text_style.to_options(tx, ty, self.ui.scale_factor, self.ui.font_scale);
        text_opts.color = self.text_style.color;

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [tx, ty],
            options: text_opts,
            clip: [x, y, final_w, final_h],
        }));

        // 6. Register Semantics & Layout
        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Button, Rect::new(x, y, final_w, final_h))
                .with_label(self.label.clone()),
        );

        self.ui
            .record_layout(self.id, Rect::new(x, y, final_w, final_h));
        self.ui.advance(final_w, final_h, start_draw);

        if self.render_mode.is_some() {
            self.ui.render_mode_stack.pop();
        }

        zenthra_core::Response {
            clicked,
            hovered: is_hovered,
            pressed: is_pressed,
            submitted: false,
            changed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_button_styling_and_borders() {
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

        // 1. Standard Raised Button from docs/api/container.md Section 3.4
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

            ui.button("Export")
                .bg(Color::rgb(0.20, 0.40, 0.85))
                .border((Color::rgb(0.30, 0.50, 0.95), 1.0))
                .radius_all(6.0)
                .shadow(Color::BLACK, 0.0, 2.0, 4.0)
                .shadow_opacity(0.5)
                .show();

            assert_eq!(ui.draws.len(), 2, "Expected 1 RectDraw background and 1 TextDraw");
            if let DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.color, Color::rgb(0.20, 0.40, 0.85).to_array());
                assert_eq!(rd.instance.border_width, 1.0);
                assert_eq!(rd.instance.border_color, Color::rgb(0.30, 0.50, 0.95).to_array());
                assert_eq!(rd.instance.radius, [6.0, 6.0, 6.0, 6.0]);
                assert_eq!(rd.instance.shadow_color, [0.0, 0.0, 0.0, 0.5]);
            } else {
                panic!("Expected RectDraw for button background");
            }
        }

        // 2. Ghost / Text-Only Button (Link Style) from Section 3.4
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

            ui.button("Cancel")
                .bg(None)
                .border(None)
                .color(Color::rgb(0.70, 0.75, 0.85))
                .show();

            assert_eq!(ui.draws.len(), 2);
            if let DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.color, Color::TRANSPARENT.to_array());
                assert_eq!(rd.instance.border_width, 0.0);
            } else {
                panic!("Expected RectDraw for ghost button background");
            }
        }

        // 3. Button with idle border and hover_border(None)
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

            let btn = ui.button("Interactive")
                .border((Color::WHITE, 1.0))
                .hover_border(None)
                .active_border(None);

            assert_eq!(btn.hover_border_width, Some(0.0));
            assert_eq!(btn.hover_border_color, Some(Color::TRANSPARENT));
            assert_eq!(btn.active_border_width, Some(0.0));
            assert_eq!(btn.active_border_color, Some(Color::TRANSPARENT));
        }

        // 4. BoxStyle unification on ButtonBuilder
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

            let btn = ui.button("Styled")
                .width(180.0)
                .height(45.0)
                .padding_all(10.0)
                .radius(8.0, 8.0, 4.0, 4.0)
                .bg(Color::RED)
                .border((Color::BLUE, 2.0));

            assert_eq!(btn.style.width, Some(180.0));
            assert_eq!(btn.style.height, Some(45.0));
            assert_eq!(btn.style.padding, EdgeInsets::all(10.0));
            assert_eq!(btn.style.radius, [8.0, 8.0, 4.0, 4.0]);
            assert_eq!(btn.style.bg, Some(zenthra_core::Background::Solid(Color::RED)));
            assert_eq!(btn.style.border, zenthra_core::Border::Solid(Color::BLUE, 2.0));

            let mut custom_style = BoxStyle::new();
            custom_style.fill_x = true;
            let btn2 = ui.button("Custom").style(custom_style);
            assert!(btn2.style.fill_x);
        }
    }
}
