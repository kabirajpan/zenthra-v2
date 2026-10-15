// crates/zenthra-widgets/src/containers/panel.rs

use crate::ui::Ui;
use zenthra_core::{Align, BoxStyle, Color, EdgeInsets, Id, Response, Styleable};

/// Generates a unique sub-ID for the panel's header container.
#[inline]
pub fn panel_header_key(id: Id) -> Id {
    Id::from_u64(((id.raw() & 0x00FF_FFFF_FFFF_FFFF) << 8) | 0x23)
}

/// A structured container widget featuring an optional title, subtitle, divider, and collapsible body.
/// Fully conforms to Zenthra's single-source BoxStyle architecture via `impl_box_style_methods!`.
pub struct PanelBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    id: Id,
    pub style: BoxStyle,

    // Header properties
    title: Option<String>,
    subtitle: Option<String>,
    header_bg: Option<zenthra_core::Background>,
    header_padding: EdgeInsets,

    // Collapsing state (direct two-way binding)
    collapsed: Option<&'b mut bool>,
}

impl<'u, 'a, 'b> Styleable for PanelBuilder<'u, 'a, 'b> {
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
impl_box_style_methods!(PanelBuilder, 'u, 'a, 'b);

impl<'u, 'a, 'b> PanelBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        let id = ui.id();
        let mut style = BoxStyle::new();
        style.padding = EdgeInsets::all(14.0);
        style.radius = [8.0; 4];
        style.bg = Some(zenthra_core::Background::Solid(Color::rgb(0.09, 0.11, 0.15)));
        style.border = zenthra_core::Border::Solid(Color::rgb(0.18, 0.22, 0.30), 1.0);

        Self {
            ui,
            id,
            style,
            title: None,
            subtitle: None,
            header_bg: None,
            header_padding: EdgeInsets::symmetric(10.0, 14.0),
            collapsed: None,
        }
    }

    #[inline]
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    #[inline]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[inline]
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Makes the panel collapsible and binds its open/close state directly to a boolean.
    #[inline]
    pub fn collapsed(mut self, state: &'b mut bool) -> Self {
        self.collapsed = Some(state);
        self
    }

    #[inline]
    pub fn header_bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self {
        match bg.into() {
            zenthra_core::Background::None => self.header_bg = None,
            other => self.header_bg = Some(other),
        }
        self
    }

    #[inline]
    pub fn header_padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.header_padding = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }

    #[inline]
    pub fn header_padding_all(mut self, p: f32) -> Self {
        self.header_padding = EdgeInsets::all(p);
        self
    }

    #[inline]
    pub fn header_padding_x(mut self, p: f32) -> Self {
        self.header_padding.left = p;
        self.header_padding.right = p;
        self
    }

    #[inline]
    pub fn header_padding_y(mut self, p: f32) -> Self {
        self.header_padding.top = p;
        self.header_padding.bottom = p;
        self
    }

    pub fn show<F>(mut self, f: F) -> Response
    where
        F: FnOnce(&mut Ui),
    {
        let is_collapsible = self.collapsed.is_some();
        let mut is_collapsed = self.collapsed.as_deref().copied().unwrap_or(false);

        let has_header = self.title.is_some() || self.subtitle.is_some() || is_collapsible;

        // Separate outer box styling from body padding so the header spans flush edge-to-edge
        let body_padding = self.style.padding;
        let mut outer_style = self.style;
        outer_style.padding = EdgeInsets::ZERO; // Header touches borders cleanly

        let radius = outer_style.radius;
        let border_color = match outer_style.border {
            zenthra_core::Border::Solid(c, _) => c,
            _ => Color::rgba(1.0, 1.0, 1.0, 0.08),
        };

        let mut panel_resp = Response {
            clicked: false,
            hovered: false,
            pressed: false,
            submitted: false,
            changed: false,
        };

        let outer_id = self.id;
        let outer_resp = self.ui.container()
            .id(outer_id)
            .column()
            .style(outer_style)
            .show(|ui: &mut Ui| {
                if has_header {
                    let header_id = panel_header_key(outer_id);
                    let mut header_container = ui.container()
                        .id(header_id)
                        .fill_x()
                        .row()
                        .halign(Align::SpaceBetween)
                        .valign(Align::Center)
                        .padding(
                            self.header_padding.top,
                            self.header_padding.right,
                            self.header_padding.bottom,
                            self.header_padding.left,
                        );

                    if let Some(hbg) = self.header_bg {
                        header_container = header_container.bg(hbg);
                    }

                    // If expanded: round top corners only. If collapsed: round all corners to match outer container.
                    let bottom_r = if is_collapsed { radius[2] } else { 0.0 };
                    let bottom_l = if is_collapsed { radius[3] } else { 0.0 };
                    header_container = header_container.radius(radius[0], radius[1], bottom_r, bottom_l);

                    if is_collapsible {
                        header_container = header_container.cursor(crate::text::CursorIcon::Pointer);
                    }

                    let header_resp = header_container.show(|ui: &mut Ui| {
                        // Left: Title and subtitle
                        ui.container()
                            .column()
                            .gap(2.0)
                            .show(|ui: &mut Ui| {
                                if let Some(ref t) = self.title {
                                    ui.text(t)
                                        .size(14.0)
                                        .font_weight(zenthra_text::prelude::FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .show();
                                }
                                if let Some(ref s) = self.subtitle {
                                    ui.text(s)
                                        .size(11.0)
                                        .color(Color::rgb(0.60, 0.65, 0.75))
                                        .show();
                                }
                            });

                        // Right: Collapse/Expand Chevron
                        if is_collapsible {
                            let chevron = if is_collapsed {
                                crate::icons::NF_FA_CHEVRON_RIGHT
                            } else {
                                crate::icons::NF_FA_CHEVRON_DOWN
                            };
                            ui.text(chevron).size(12.0).color(Color::rgb(0.65, 0.70, 0.80)).show();
                        }
                    });

                    if is_collapsible && header_resp.clicked {
                        is_collapsed = !is_collapsed;
                        if let Some(ref mut state) = self.collapsed {
                            **state = is_collapsed;
                        }
                        ui.needs_redraw = true;
                        ui.consume_click();
                        panel_resp.changed = true;
                    }

                    panel_resp.clicked = header_resp.clicked;
                    panel_resp.hovered = header_resp.hovered;
                    panel_resp.pressed = header_resp.pressed;
                }

                // Body Section (only if not collapsed)
                if !is_collapsed {
                    // Draw subtle divider line if header is present
                    if has_header {
                        ui.container()
                            .fill_x()
                            .height(1.0)
                            .bg(border_color)
                            .show(|_| {});
                    }

                    // Body content container with user's padding
                    ui.container()
                        .fill_x()
                        .column()
                        .padding(
                            body_padding.top,
                            body_padding.right,
                            body_padding.bottom,
                            body_padding.left,
                        )
                        .show(f);
                }
            });

        if !has_header {
            panel_resp.clicked = outer_resp.clicked;
            panel_resp.pressed = outer_resp.pressed;
        }
        panel_resp.hovered = panel_resp.hovered || outer_resp.hovered;
        panel_resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use zenthra_core::{Border, Rect};

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
        ui.available_width = 800.0;
        ui
    }

    #[test]
    fn test_panel_box_style_methods_parity() {
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

        let panel = ui.panel()
            .width(300.0)
            .height(200.0)
            .fill_x()
            .padding(10.0, 12.0, 14.0, 16.0)
            .margin(5.0, 6.0, 7.0, 8.0)
            .radius(4.0, 6.0, 8.0, 10.0)
            .bg(Color::rgb(0.1, 0.2, 0.3))
            .border((Color::rgb(0.4, 0.5, 0.6), 2.0))
            .shadow(Color::rgba(0.0, 0.0, 0.0, 0.5), 2.0, 4.0, 8.0);

        assert_eq!(panel.style.width, Some(300.0));
        assert_eq!(panel.style.height, Some(200.0));
        assert!(panel.style.fill_x);
        assert_eq!(panel.style.padding.top, 10.0);
        assert_eq!(panel.style.padding.right, 12.0);
        assert_eq!(panel.style.padding.bottom, 14.0);
        assert_eq!(panel.style.padding.left, 16.0);
        assert_eq!(panel.style.margin.top, 5.0);
        assert_eq!(panel.style.radius, [4.0, 6.0, 8.0, 10.0]);
        assert_eq!(panel.style.border, Border::Solid(Color::rgb(0.4, 0.5, 0.6), 2.0));
    }

    #[test]
    fn test_panel_controlled_collapsed() {
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

        let mut is_collapsed = true;
        let mut body_rendered = false;

        ui.panel()
            .id("controlled_panel")
            .title("Controlled")
            .collapsed(&mut is_collapsed)
            .show(|_| {
                body_rendered = true;
            });

        // When collapsed is true, body closure must NOT be invoked
        assert!(!body_rendered);

        // When collapsed is false, body closure IS invoked
        is_collapsed = false;
        body_rendered = false;
        ui.panel()
            .id("controlled_panel")
            .title("Controlled")
            .collapsed(&mut is_collapsed)
            .show(|_| {
                body_rendered = true;
            });

        assert!(body_rendered);
    }
}
