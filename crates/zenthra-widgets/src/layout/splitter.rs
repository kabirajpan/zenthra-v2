// crates/zenthra-widgets/src/layout/splitter.rs

use crate::style_ext::resolve_background;
use crate::ui::{DrawCommand, RectDraw, Ui};
use zenthra_core::{BoxStyle, Color, Id, Rect, Response, Styleable};
use zenthra_render::RectInstance;

/// Builder for draggable splitter divider lines (`ui.splitter()`).
/// Automatically detects horizontal vs. vertical splitting based on dimensions.
pub struct SplitterBuilder<'u, 'a, 'b> {
    pub ui: &'u mut Ui<'a>,
    pub id: Id,
    pub value: &'b mut f32,
    pub min: f32,
    pub max: f32,
    pub invert: bool,

    // Unified Box Surface Style
    pub style: BoxStyle,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,
}

impl<'u, 'a, 'b> Styleable for SplitterBuilder<'u, 'a, 'b> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

impl_box_style_methods!(SplitterBuilder, 'u, 'a, 'b);
impl_interaction_state_methods!(SplitterBuilder, 'u, 'a, 'b);

impl<'u, 'a, 'b> SplitterBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, value: &'b mut f32, range: std::ops::RangeInclusive<f32>) -> Self {
        let id = ui.id();
        let (min, max) = range.into_inner();
        Self {
            ui,
            id,
            value,
            min,
            max,
            invert: false,
            style: BoxStyle::new(),
            hover_state: None,
            active_state: None,
        }
    }

    #[inline]
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = self.ui.id_from(id);
        self
    }

    /// Inverts the drag delta direction (useful when resizing a right-anchored or bottom-anchored panel).
    #[inline]
    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    /// Renders the splitter, handles mouse drag interactions, clamps the value, and returns the Response.
    pub fn show(self) -> Response {
        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);

        // 1. Resolve dimensions
        let is_row = matches!(self.ui.direction, crate::container::Direction::Row);

        let parent_avail_h = if self.ui.line_height > 0.0 {
            self.ui.line_height
        } else {
            (self.ui.height as f32 - y).max(0.0)
        };

        let w = if let Some(w) = self.style.width {
            w
        } else if self.style.fill_x {
            (self.ui.available_width - self.style.margin.horizontal()).max(0.0)
        } else if is_row {
            4.0
        } else {
            (self.ui.available_width - self.style.margin.horizontal()).max(0.0)
        };

        let h = if let Some(h) = self.style.height {
            h
        } else if self.style.fill_y {
            (parent_avail_h - self.style.margin.vertical()).max(0.0)
        } else if is_row {
            (parent_avail_h - self.style.margin.vertical()).max(0.0)
        } else {
            4.0
        };

        // Determine orientation: if height >= width, it is a vertical bar dividing along X axis
        let is_vertical_bar = h >= w;

        // 2. Hit testing with generous grab target (at least 8px)
        let hit_w = if is_vertical_bar { w.max(8.0) } else { w };
        let hit_h = if is_vertical_bar { h } else { h.max(8.0) };

        let (actual_ox, actual_oy, actual_w, actual_h) = if self.ui.current_window_id.is_some() {
            (x, y, w, h)
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
        };

        let hit_ox = if is_vertical_bar {
            actual_ox - (hit_w - actual_w) * 0.5
        } else {
            actual_ox
        };
        let hit_oy = if is_vertical_bar {
            actual_oy
        } else {
            actual_oy - (hit_h - actual_h) * 0.5
        };

        let is_hovered = self.ui.is_hovered(self.id, hit_ox, hit_oy, hit_w, hit_h);
        let is_active = self.ui.active_drag.as_ref().map(|d| d.id == self.id).unwrap_or(false);

        // Cursor feedback
        if is_hovered || is_active {
            self.ui.cursor_icon = if is_vertical_bar {
                crate::text::CursorIcon::ColResize
            } else {
                crate::text::CursorIcon::RowResize
            };
        }

        let mut changed = false;
        let mut clicked = false;

        // 3. Drag interaction logic
        if self.ui.clicked && is_hovered && self.ui.can_interact(self.id) {
            clicked = true;
            self.ui.claim_active(self.id);
            self.ui.active_drag = Some(crate::ui::ScrollDrag {
                id: self.id,
                start_mouse: if is_vertical_bar { self.ui.mouse_x } else { self.ui.mouse_y },
                start_scroll: *self.value,
            });
        }

        if is_active {
            if self.ui.mouse_down {
                if let Some(drag) = &self.ui.active_drag {
                    let current_mouse = if is_vertical_bar { self.ui.mouse_x } else { self.ui.mouse_y };
                    let delta = current_mouse - drag.start_mouse;
                    let signed_delta = if self.invert { -delta } else { delta };
                    let new_val = (drag.start_scroll + signed_delta).clamp(self.min, self.max);
                    if (*self.value - new_val).abs() > 0.001 {
                        *self.value = new_val;
                        changed = true;
                        self.ui.needs_redraw = true;
                    }
                }
            } else {
                self.ui.active_drag = None;
                if self.ui.is_active(self.id) {
                    self.ui.release_active();
                }
            }
        }

        // 4. Rendering
        let start_draw = self.ui.draws.len();

        let active_overrides = if is_active { self.active_state.as_ref() } else { None };
        let hover_overrides = if is_hovered { self.hover_state.as_ref() } else { None };

        // Resolve background: active -> hover -> idle -> subtle fallback
        let effective_bg = active_overrides
            .and_then(|o| o.style.bg.as_ref())
            .or_else(|| hover_overrides.and_then(|o| o.style.bg.as_ref()))
            .or(self.style.bg.as_ref());

        let default_color = if is_active {
            Color::rgba(0.35, 0.60, 1.0, 0.90)
        } else if is_hovered {
            Color::rgba(0.35, 0.60, 1.0, 0.65)
        } else {
            Color::rgba(1.0, 1.0, 1.0, 0.08)
        };

        let bg_opt = effective_bg.cloned().or_else(|| Some(zenthra_core::Background::Solid(default_color)));
        let (c1, c2, c3, c4, grad_params, stops, _is_visible) = resolve_background(&bg_opt);

        let effective_border = active_overrides
            .and_then(|o| match o.style.border {
                zenthra_core::Border::None => None,
                b => Some(b),
            })
            .or_else(|| hover_overrides.and_then(|o| match o.style.border {
                zenthra_core::Border::None => None,
                b => Some(b),
            }))
            .unwrap_or(self.style.border);

        let (border_col, border_w) = match effective_border {
            zenthra_core::Border::Solid(c, bw) => (c.to_array(), bw),
            _ => ([0.0; 4], 0.0),
        };

        let radius = self.style.radius;

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [x + self.style.margin.left, y + self.style.margin.top],
                size: [w, h],
                color: c1,
                color2: c2,
                color3: c3,
                color4: c4,
                gradient_params: grad_params,
                gradient_stops: stops,
                radius: [radius[3], radius[2], radius[1], radius[0]],
                border_width: border_w,
                border_color: border_col,
                shadow_color: self.style.shadow_color.map(|c| c.to_array()).unwrap_or([0.0; 4]),
                shadow_offset: self.style.shadow_offset,
                shadow_blur: self.style.shadow_blur,
                opacity: self.style.opacity,
                ..Default::default()
            },
        }));

        self.ui.record_layout(self.id, Rect::new(x, y, w, h));
        self.ui.advance(w + self.style.margin.horizontal(), h + self.style.margin.vertical(), start_draw);

        Response {
            clicked,
            hovered: is_hovered,
            pressed: is_active || (self.ui.clicked && is_hovered),
            submitted: false,
            changed,
        }
    }
}
