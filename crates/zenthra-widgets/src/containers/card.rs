// crates/zenthra-widgets/src/containers/card.rs

use crate::ui::Ui;
use zenthra_core::{Align, Background, Border, BoxStyle, Color, EdgeInsets, Id, Response, Styleable};

/// An interactive surface container widget for grouping content with elevation, hover animations, and unified styling.
/// Fully conforms to Zenthra's single-source BoxStyle architecture via `impl_box_style_methods!`.
pub struct CardBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,
    pub style: BoxStyle,

    // Layout
    is_row: bool,
    halign: Option<Align>,
    valign: Option<Align>,
    gap: f32,

    // Sibling Interaction & State Overrides (.hover([ ... ]), .active([ ... ]))
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,
}

impl<'u, 'a> Styleable for CardBuilder<'u, 'a> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

// Single-source macro generates all canonical BoxStyle methods:
// .width(), .height(), .fill_x(), .fill_y(), .fill(), .min/max_width(), .min/max_height(),
// .padding(), .padding_all(), .padding_x(), .padding_y(), .padding_top/bottom/left/right(),
// .margin(), .margin_all(), .margin_x(), .margin_y(), .margin_top/bottom/left/right(),
// .radius(), .radius_all(), .radius_top/bottom/left/right(), .bg(), .border(), .shadow(), .style()
impl_box_style_methods!(CardBuilder, 'u, 'a);
impl_interaction_state_methods!(CardBuilder, 'u, 'a);

impl<'u, 'a> CardBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        let id = ui.id();
        let mut style = BoxStyle::new();
        style.padding = EdgeInsets::all(16.0);
        style.radius = [10.0; 4];
        style.bg = Some(Background::Solid(Color::rgb(0.09, 0.11, 0.15)));
        style.border = Border::Solid(Color::rgb(0.18, 0.22, 0.30), 1.0);

        Self {
            ui,
            id,
            style,
            is_row: false,
            halign: None,
            valign: None,
            gap: 0.0,
            hover_state: None,
            active_state: None,
        }
    }

    #[inline]
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = self.ui.id_from(id);
        self
    }

    #[inline]
    pub fn row(mut self) -> Self {
        self.is_row = true;
        self
    }

    #[inline]
    pub fn column(mut self) -> Self {
        self.is_row = false;
        self
    }

    #[inline]
    pub fn halign(mut self, align: Align) -> Self {
        self.halign = Some(align);
        self
    }

    #[inline]
    pub fn valign(mut self, align: Align) -> Self {
        self.valign = Some(align);
        self
    }

    #[inline]
    pub fn align(mut self, align: Align) -> Self {
        self.halign = Some(align);
        self.valign = Some(align);
        self
    }

    #[inline]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn show<F>(self, f: F) -> Response
    where
        F: FnOnce(&mut Ui),
    {
        let mut container = self.ui.container()
            .id(self.id)
            .style(self.style);

        if self.is_row {
            container = container.row();
        } else {
            container = container.column();
        }

        if let Some(ha) = self.halign {
            container = container.halign(ha);
        }
        if let Some(va) = self.valign {
            container = container.valign(va);
        }
        if self.gap > 0.0 {
            container = container.gap(self.gap);
        }

        container.hover_state = self.hover_state;
        container.active_state = self.active_state;

        container.show(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use zenthra_core::{BorderAlignment, Rect};

    fn create_test_ui<'a>(
        scroll_state: &'a mut HashMap<Id, (f32, f32)>,
        cursor_state: &'a mut HashMap<Id, usize>,
        interaction_state: &'a mut HashMap<Id, f32>,
        layout_cache: &'a HashMap<Id, (Rect, u64)>,
        next_layout_cache: &'a mut HashMap<Id, (Rect, u64)>,
        screen_layout_cache: &'a HashMap<Id, Rect>,
        next_screen_layout_cache: &'a mut HashMap<Id, Rect>,
        widget_window_map: &'a HashMap<Id, Id>,
        next_widget_window_map: &'a mut HashMap<Id, Id>,
        image_sizes: &'a HashMap<zenthra_core::ImageSource, (u32, u32)>,
    ) -> Ui<'a> {
        let mut ui = Ui::new(
            800, 600, 1.0, None, Vec::new(), None,
            (0.0, 0.0), false,
            scroll_state, cursor_state, interaction_state,
            None, None, false, false, 0.0,
            layout_cache, next_layout_cache,
            screen_layout_cache, next_screen_layout_cache,
            widget_window_map, next_widget_window_map,
            image_sizes,
        );
        ui.mouse_x = 50.0;
        ui.mouse_y = 50.0;
        ui
    }

    #[test]
    fn test_card_box_style_methods_parity() {
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

        let mut ui = create_test_ui(
            &mut scroll_state, &mut cursor_state, &mut interaction_state,
            &layout_cache, &mut next_layout_cache,
            &screen_layout_cache, &mut next_screen_layout_cache,
            &widget_window_map, &mut next_widget_window_map,
            &image_sizes,
        );

        let card = ui.card()
            .width(320.0)
            .height(200.0)
            .fill_x()
            .padding(12.0, 16.0, 12.0, 16.0)
            .margin(8.0, 10.0, 8.0, 10.0)
            .radius(12.0, 12.0, 8.0, 8.0)
            .bg(Color::rgb(0.1, 0.15, 0.2))
            .border((Color::rgb(0.3, 0.4, 0.6), 1.5, BorderAlignment::Inside))
            .shadow(Color::rgba(0.0, 0.0, 0.0, 0.4), 0.0, 4.0, 12.0)
            .gap(10.0)
            .row();

        assert_eq!(card.style.width, Some(320.0));
        assert_eq!(card.style.height, Some(200.0));
        assert!(card.style.fill_x);
        assert_eq!(card.style.padding.top, 12.0);
        assert_eq!(card.style.padding.right, 16.0);
        assert_eq!(card.style.margin.top, 8.0);
        assert_eq!(card.style.radius, [12.0, 12.0, 8.0, 8.0]);
        assert_eq!(card.is_row, true);
        assert_eq!(card.gap, 10.0);
    }

    #[test]
    fn test_card_sibling_interaction_states() {
        use crate::macros::{Style, Transform, Cursor, Tooltip, Animation};

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

        let mut ui = create_test_ui(
            &mut scroll_state, &mut cursor_state, &mut interaction_state,
            &layout_cache, &mut next_layout_cache,
            &screen_layout_cache, &mut next_screen_layout_cache,
            &widget_window_map, &mut next_widget_window_map,
            &image_sizes,
        );

        let card = ui.card()
            .bg(Color::rgb(0.1, 0.1, 0.1))
            .hover([
                Style::bg(Color::rgb(0.2, 0.2, 0.2)),
                Style::border((Color::WHITE, 1.0)),
                Transform::scale(1.05),
                Cursor::Pointer,
                Tooltip::new("Hovered Card"),
                Animation::duration(200).ease_out(),
            ])
            .active([
                Style::bg(Color::rgb(0.05, 0.05, 0.05)),
                Transform::scale(0.95),
            ]);

        assert!(card.hover_state.is_some());
        let h = card.hover_state.as_ref().unwrap();
        assert_eq!(h.scale, 1.05);
        assert_eq!(h.cursor, Some(crate::text::CursorIcon::Pointer));
        assert_eq!(h.tooltip.as_deref(), Some("Hovered Card"));
        assert_eq!(h.duration_ms, Some(200));

        assert!(card.active_state.is_some());
        let a = card.active_state.as_ref().unwrap();
        assert_eq!(a.scale, 0.95);
    }
}
