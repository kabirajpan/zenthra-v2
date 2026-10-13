use crate::ui::{DrawCommand, RectDraw, Ui};
pub use zenthra_core::BorderAlignment;
use zenthra_core::{Align, BoxStyle, Color, Styleable};
use crate::style_ext::BoxStyleExt;
pub use crate::style_ext::resolve_background;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Row,
    Column,
    Stack,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Wrap {
    #[default]
    NoWrap,
    Auto,
    Wrap,
    WrapReverse,
    RightToLeft,
    RightToLeftReverse,
}

pub struct ContainerBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    direction: Direction,
    halign: Align,
    valign: Align,
    wrap: Wrap,
    children_draws: Vec<DrawCommand>,
    child_sizes: Vec<(f32, f32)>,
    child_ranges: Vec<(usize, usize)>,
    child_origins: Vec<(f32, f32)>,
    start_x: f32,
    start_y: f32,
    pos_x: Option<f32>,
    pos_y: Option<f32>,
    pub style: BoxStyle,
    gap: f32,
    render_mode: Option<zenthra_core::RenderMode>,
    scroll_x: bool,
    scroll_y: bool,
    show_scrollbars: bool,
    clip: Option<bool>,
    pub id: zenthra_core::Id,
    is_absolute: bool,
    is_overlay: bool,
    hover_bg: Option<zenthra_core::Background>,
    hover_border_color: Option<Color>,
    hover_border_width: Option<f32>,
    hover_scale: f32,
    active_bg: Option<zenthra_core::Background>,
    active_border_color: Option<Color>,
    active_border_width: Option<f32>,
    active_scale: f32,
    backdrop_blur: Option<f32>,
    post_process_shader: Option<&'static str>,
    post_process_params: [f32; 8],
    backdrop_filter: Option<zenthra_core::BackdropFilter>,
    draggable_window: bool,
    pub role: Option<zenthra_core::Role>,
    pub custom_id: bool,
    pub interactive: bool,
    pub cursor: Option<crate::text::CursorIcon>,
}

impl<'u, 'a> Styleable for ContainerBuilder<'u, 'a> {
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
impl_box_style_methods!(ContainerBuilder);

/// Parses an IntoBorder value into (color, width) overrides.
#[inline]
fn parse_border(border: impl crate::macros::IntoBorder) -> (Option<Color>, Option<f32>) {
    let (b, _) = border.into_border();
    match b {
        zenthra_core::Border::None => (Some(Color::TRANSPARENT), Some(0.0)),
        zenthra_core::Border::Solid(c, w) => (Some(c), Some(w)),
    }
}

impl<'u, 'a> ContainerBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        let id = ui.id();
        Self {
            ui,
            direction: Direction::Column,
            halign: Align::Left,
            valign: Align::Top,
            wrap: Wrap::NoWrap,
            children_draws: Vec::new(),
            child_sizes: Vec::new(),
            child_ranges: Vec::new(),
            child_origins: Vec::new(),
            custom_id: false,
            interactive: false,
            cursor: None,
            start_x: 0.0,
            start_y: 0.0,
            pos_x: None,
            pos_y: None,
            style: BoxStyle::new(),
            gap: 0.0,
            render_mode: None,
            scroll_x: false,
            scroll_y: false,
            show_scrollbars: true,
            clip: None,
            id,
            is_absolute: false,
            is_overlay: false,
            hover_bg: None,
            hover_border_color: None,
            hover_border_width: None,
            hover_scale: 1.0,
            active_bg: None,
            active_border_color: None,
            active_border_width: None,
            active_scale: 1.0,
            backdrop_blur: None,
            post_process_shader: None,
            post_process_params: [0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0],
            backdrop_filter: None,
            draggable_window: false,
            role: None,
        }
    }

    /// Sets explicit position for the container, taking it out of parent layout flow.
    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.pos_x = Some(x);
        self.pos_y = Some(y);
        self.is_absolute = true;
        self
    }

    pub fn scroll(mut self, enabled: bool) -> Self {
        self.scroll_x = enabled;
        self.scroll_y = enabled;
        if enabled { self.clip = Some(true); }
        self
    }

    pub fn draggable_window(mut self, drag: bool) -> Self {
        self.draggable_window = drag;
        self
    }

    pub fn scroll_x(mut self, enabled: bool) -> Self {
        self.scroll_x = enabled;
        if enabled { self.clip = Some(true); }
        self
    }

    pub fn scroll_y(mut self, enabled: bool) -> Self {
        self.scroll_y = enabled;
        if enabled { self.clip = Some(true); }
        self
    }

    /// Controls whether scrollbar thumbs are visually rendered (default: true).
    pub fn scrollbars(mut self, show: bool) -> Self {
        self.show_scrollbars = show;
        self
    }

    /// Force clip the overflowing content bounds of this container.
    pub fn clip(mut self, enabled: bool) -> Self {
        self.clip = Some(enabled);
        self
    }

    pub fn row(mut self) -> Self {
        self.direction = Direction::Row;
        self
    }

    pub fn column(mut self) -> Self {
        self.direction = Direction::Column;
        self
    }

    pub fn stack(mut self) -> Self {
        self.direction = Direction::Stack;
        self
    }

    pub fn wrap(mut self, strategy: Wrap) -> Self {
        self.wrap = strategy;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.halign = align;
        self.valign = align;
        self
    }

    pub fn halign(mut self, align: Align) -> Self {
        self.halign = align;
        self
    }

    pub fn valign(mut self, align: Align) -> Self {
        self.valign = align;
        self
    }

    pub fn active_bg(mut self, color: impl Into<zenthra_core::Background>) -> Self {
        self.active_bg = Some(color.into());
        self
    }

    pub fn active_border(mut self, border: impl crate::macros::IntoBorder) -> Self {
        let (color, width) = parse_border(border);
        self.active_border_color = color;
        self.active_border_width = width;
        self
    }

    pub fn active_scale(mut self, scale: f32) -> Self {
        self.active_scale = scale;
        self
    }

    pub fn gap(mut self, g: f32) -> Self {
        self.gap = g;
        self
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        id.hash(&mut hasher);
        if let Some(parent) = self.ui.semantic_stack.last() {
            parent.hash(&mut hasher);
        }
        self.id = zenthra_core::Id::from_u64(hasher.finish());
        self.custom_id = true;
        self
    }

    pub fn raw_id(mut self, id: zenthra_core::Id) -> Self {
        self.id = id;
        self.custom_id = true;
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn cursor(mut self, icon: crate::text::CursorIcon) -> Self {
        self.cursor = Some(icon);
        self
    }

    pub fn hover_bg(mut self, color: impl Into<zenthra_core::Background>) -> Self {
        self.hover_bg = Some(color.into());
        self
    }

    pub fn hover_border(mut self, border: impl crate::macros::IntoBorder) -> Self {
        let (color, width) = parse_border(border);
        self.hover_border_color = color;
        self.hover_border_width = width;
        self
    }

    pub fn hover_scale(mut self, scale: f32) -> Self {
        self.hover_scale = scale;
        self
    }

    /// Enable glassmorphism: blurs the scene content behind this container.
    /// `radius` is in logical pixels — higher values produce a softer, deeper blur.
    /// Typically 10.0–40.0. Combine with a semi-transparent `.bg()` for the frosted glass look.
    pub fn backdrop_blur(mut self, radius: f32) -> Self {
        self.backdrop_blur = Some(radius);
        self
    }

    pub fn shader(mut self, shader_id: &'static str) -> Self {
        self.post_process_shader = Some(shader_id);
        self
    }

    pub fn backdrop_filter(mut self, filter: zenthra_core::BackdropFilter) -> Self {
        self.backdrop_filter = Some(filter);
        self
    }

    pub fn render_mode(mut self, mode: zenthra_core::RenderMode) -> Self {
        self.render_mode = Some(mode);
        self
    }

    pub fn overlay(mut self) -> Self {
        self.is_overlay = true;
        self
    }

    pub fn role(mut self, role: zenthra_core::Role) -> Self {
        self.role = Some(role);
        self
    }


    pub fn show<F>(mut self, f: F) -> zenthra_core::Response
    where
        F: FnOnce(&mut Ui),
    {
        // 1. Capture style values and own ID
        let id = self.id;
        let width = self.style.width;
        let height = self.style.height;
        let padding_left = self.style.padding.left;
        let padding_right = self.style.padding.right;
        let padding_top = self.style.padding.top;
        let padding_bottom = self.style.padding.bottom;
        let margin_left = self.style.margin.left;
        let margin_right = self.style.margin.right;
        let margin_top = self.style.margin.top;
        let margin_bottom = self.style.margin.bottom;
        let border_width = self.style.border_width();
        let border_color = self.style.border_color();
        let fill_x = self.style.fill_x;
        let fill_y = self.style.fill_y;
        let min_width = self.style.min_width;
        let max_width = self.style.max_width;
        let min_height = self.style.min_height;
        let max_height = self.style.max_height;

        let actual_clip = self.clip.unwrap_or(width.is_some() || height.is_some() || self.scroll_x || self.scroll_y);

        self.start_x = self.ui.cursor_x;
        self.start_y = self.ui.cursor_y;
        let ox = self.pos_x.unwrap_or(self.start_x) + margin_left;
        let oy = self.pos_y.unwrap_or(self.start_y) + margin_top;

        // -- Save current Ui environment --
        let prev_dir = self.ui.direction;
        let prev_line_h = self.ui.line_height;
        let prev_base_x = self.ui.base_x;
        let prev_base_y = self.ui.base_y;
        let prev_offset_x = self.ui.offset_x;
        let prev_offset_y = self.ui.offset_y;
        let prev_viewport = self.ui.current_viewport;
        
        // Take and isolate state maps for children
        let prev_child_sizes = std::mem::take(&mut self.ui.child_sizes);
        let prev_child_ranges = std::mem::take(&mut self.ui.child_draw_ranges);
        let prev_child_origins = std::mem::take(&mut self.ui.child_origins);
        let prev_id_ranges = std::mem::take(&mut self.ui.id_ranges);
        let prev_id_log = std::mem::take(&mut self.ui.id_log);
        
        let prev_max_x = self.ui.max_x;
        let prev_max_y = self.ui.max_y;
        
        let _id_log_start_idx = self.ui.id_log.len();

        // Set child environment
        if let Some(w) = width {
            self.ui.max_x = (ox + w - padding_right).min(prev_max_x);
        }
        if let Some(h) = height {
            self.ui.max_y = (oy + h - padding_bottom).min(prev_max_y);
        }

        self.ui.direction = self.direction;
        self.ui.line_height = 0.0;

        let mode = self.render_mode.unwrap_or(self.ui.current_render_mode());
        self.ui.render_mode_stack.push(mode);
        if mode == zenthra_core::RenderMode::Continuous {
            self.ui.needs_redraw = true;
        }

        let avail_w = if let Some(w) = width {
            w - padding_left - padding_right - 2.0 * border_width
        } else {
            (prev_max_x - ox - padding_left - padding_right - 2.0 * border_width - margin_right).max(0.0)
        };
        let avail_h = if let Some(h) = height {
            h - padding_top - padding_bottom - 2.0 * border_width
        } else {
            (prev_max_y - oy - padding_top - padding_bottom - 2.0 * border_width - margin_bottom).max(0.0)
        };

        // Important: If we are absolute, our internal boundary is NOT capped by prev_max_x.
        // This allows us to have a full width even at the edge of the screen.
        self.ui.max_x = ox + padding_left + border_width + avail_w;
        self.ui.max_y = oy + padding_top + border_width + avail_h;

        self.ui.semantic_stack.push(id);
        let role = self.role.unwrap_or(zenthra_core::Role::Container);
        self.ui.register_semantic(zenthra_core::SemanticNode::new(id, role, zenthra_core::Rect::new(ox, oy, 0.0, 0.0)));

        let parent_draws = std::mem::take(&mut self.ui.draws);

        // Update global offsets for children (for visibility/culling logic)
        let prev_global_ox = self.ui.offset_x;
        let prev_global_oy = self.ui.offset_y;
        let prev_avail_w = self.ui.available_width;
        
        let (sx, sy) = if self.scroll_x || self.scroll_y {
            *self.ui.scroll_state.get(&id).unwrap_or(&(0.0, 0.0))
        } else {
            (0.0, 0.0)
        };

        self.ui.offset_x -= sx;
        self.ui.offset_y -= sy;
        self.ui.available_width = avail_w;

        self.ui.base_x = ox + padding_left + border_width;
        self.ui.base_y = oy + padding_top + border_width;
        self.ui.cursor_x = self.ui.base_x;
        self.ui.cursor_y = self.ui.base_y;

        // Update viewport for children if clipping is enabled
        if actual_clip {
            if let Some((rect, _)) = self.ui.get_recorded_layout(id) {
                let my_screen_rect = [rect.origin.x + prev_global_ox, rect.origin.y + prev_global_oy, rect.size.width, rect.size.height];
                let parent_rect = [prev_viewport.origin.x, prev_viewport.origin.y, prev_viewport.size.width, prev_viewport.size.height];
                let intersected = intersect_rects(my_screen_rect, parent_rect);
                self.ui.current_viewport = zenthra_core::Rect::new(intersected[0], intersected[1], intersected[2], intersected[3]);
            }
        }

        // -- Run Children --
        if self.is_overlay {
            let key = crate::ui::overlay_key(id);
            self.ui.interaction_state.insert(key, 1.0);
            self.ui.active_overlays.push(id);
            self.ui.active_overlay_stack.push(id);
        }

        f(self.ui);

        if self.is_overlay {
            self.ui.active_overlay_stack.pop();
        }

        // Restore global offsets
        self.ui.offset_x = prev_global_ox;
        self.ui.offset_y = prev_global_oy;
        self.ui.available_width = prev_avail_w;

        // -- Restore Stacks and Environment --
        self.ui.render_mode_stack.pop();
        self.ui.semantic_stack.pop();
        self.ui.current_viewport = prev_viewport;
        
        // Capture everything the children created
        let child_ids_only = std::mem::replace(&mut self.ui.id_log, prev_id_log);
        let child_id_ranges = std::mem::replace(&mut self.ui.id_ranges, prev_id_ranges);
        self.children_draws = std::mem::replace(&mut self.ui.draws, parent_draws);
        self.child_sizes = std::mem::replace(&mut self.ui.child_sizes, prev_child_sizes);
        self.child_ranges = std::mem::replace(&mut self.ui.child_draw_ranges, prev_child_ranges);
        self.child_origins = std::mem::replace(&mut self.ui.child_origins, prev_child_origins);
        
        let _id_log_end_idx = child_ids_only.len();

        // Restore cursor/base
        self.ui.direction = prev_dir;
        self.ui.line_height = prev_line_h;
        self.ui.base_x = prev_base_x;
        self.ui.base_y = prev_base_y;
        self.ui.offset_x = prev_offset_x;
        self.ui.offset_y = prev_offset_y;
        self.ui.cursor_x = self.start_x;
        self.ui.cursor_y = self.start_y;
        self.ui.max_x = prev_max_x;
        self.ui.max_y = prev_max_y;

        // -- Layout Engine --
        let n = self.child_sizes.len();
        let mut target_positions: Vec<(f32, f32)> = vec![(0.0, 0.0); n];
        let (content_w, content_h) = match self.wrap {
            Wrap::NoWrap => self.layout_no_wrap(ox, oy, avail_w, avail_h, &mut target_positions),
            Wrap::Auto => {
                // Auto wrapper: automatically wrap if direction is Row, not horizontally scrolling,
                // not single-row distributed (SpaceBetween/SpaceAround), not fixed-height, and children exceed available width.
                let should_wrap = self.direction == Direction::Row
                    && !self.scroll_x
                    && height.is_none()
                    && self.halign != Align::SpaceBetween
                    && self.halign != Align::SpaceAround
                    && (avail_w > 0.0)
                    && {
                        let total_w: f32 = self.child_sizes.iter().map(|(w, _)| *w).sum::<f32>()
                            + self.gap * (n.saturating_sub(1) as f32);
                        total_w > avail_w
                    };
                if should_wrap {
                    self.layout_wrap(ox, oy, avail_w, avail_h, &mut target_positions)
                } else {
                    self.layout_no_wrap(ox, oy, avail_w, avail_h, &mut target_positions)
                }
            }
            _ => self.layout_wrap(ox, oy, avail_w, avail_h, &mut target_positions),
        };

        let mut w = if fill_x {
            (self.ui.max_x - ox - margin_right).max(0.0)  // use parent's right boundary, not global window width
        } else {
            width.unwrap_or(content_w + padding_left + padding_right + 2.0 * border_width)
        };
        
        if let Some(mw) = max_width { w = w.min(mw); }
        if let Some(mw) = min_width { w = w.max(mw); }

        let mut h = if fill_y {
            (self.ui.max_y - oy - margin_bottom).max(0.0)  // use parent's bottom boundary, not global window height
        } else {
            height.unwrap_or(content_h + padding_top + padding_bottom + 2.0 * border_width)
        };

        if let Some(mh) = max_height { h = h.min(mh); }
        if let Some(mh) = min_height { h = h.max(mh); }

        let draw_start = self.ui.draws.len();
        self.ui.record_layout(id, zenthra_core::Rect::new(ox, oy, w, h));

        let max_sx = (content_w + padding_left + padding_right - w).max(0.0);
        let max_sy = (content_h + padding_top + padding_bottom - h).max(0.0);

        let (actual_ox, actual_oy, actual_w, actual_h) = if self.is_absolute || self.pos_x.is_some() || self.pos_y.is_some() {
            (ox + prev_global_ox, oy + prev_global_oy, w, h)
        } else if let Some(screen_rect) = self.ui.screen_layout_cache.get(&id) {
            (screen_rect.origin.x, screen_rect.origin.y, screen_rect.size.width, screen_rect.size.height)
        } else if let Some((rect, _)) = self.ui.get_recorded_layout(id) {
            (rect.origin.x + prev_global_ox, rect.origin.y + prev_global_oy, rect.size.width, rect.size.height)
        } else {
            (ox + prev_global_ox, oy + prev_global_oy, w, h)
        };

        let mouse_in_parent = if self.ui.skip_clip_stack.last().cloned().unwrap_or(false) {
            true
        } else {
            prev_viewport.contains(zenthra_core::Point::new(self.ui.mouse_x, self.ui.mouse_y))
        };

        let can_interact = self.ui.can_interact(id);
        let raw_hover = mouse_in_parent && self.ui.mouse_in_rect(actual_ox, actual_oy, actual_w, actual_h) && !self.ui.is_occluded(id, self.ui.mouse_x, self.ui.mouse_y);
        let container_hover = raw_hover && can_interact;

        if container_hover {
            if let Some(cursor) = self.cursor {
                self.ui.cursor_icon = cursor;
            }
        }

        let is_interactive = self.role == Some(zenthra_core::Role::Button)
            || self.interactive
            || self.draggable_window
            || self.scroll_x
            || self.scroll_y
            || self.cursor.is_some()
            || self.hover_bg.is_some()
            || self.hover_border_color.is_some()
            || self.active_bg.is_some()
            || self.active_border_color.is_some();

        let container_active = if self.ui.mouse_down {
            if self.ui.active_widget_id == Some(id) {
                true
            } else if self.ui.active_widget_id.is_none() && raw_hover && is_interactive {
                self.ui.active_widget_id = Some(id);
                true
            } else {
                false
            }
        } else {
            false
        };

        let (scroll_x, scroll_y) = if self.scroll_x || self.scroll_y {
            let (mut sx, mut sy) = *self.ui.scroll_state.get(&id).unwrap_or(&(0.0, 0.0));
            
            if container_hover && !self.ui.resize_active {
                let mut events = std::mem::take(&mut self.ui.input_events);
                events.retain(|event| {
                    let mut keep = true;
                    if let zenthra_platform::event::PlatformEvent::MouseWheel { delta_y, delta_x, .. } = event {
                        let mut consumed = false;
                        let step_y = if delta_y.abs() <= 5.0 { *delta_y * 38.0 } else { *delta_y };
                        let step_x = if delta_x.abs() <= 5.0 { *delta_x * 38.0 } else { *delta_x };
                        if self.scroll_y && *delta_y != 0.0 {
                            let can_up = sy > 0.0 && *delta_y > 0.0;
                            let can_down = sy < max_sy && *delta_y < 0.0;
                            if can_up || can_down {
                                sy -= step_y;
                            }
                            consumed = true;
                        }
                        if self.scroll_x && *delta_x != 0.0 {
                            let can_left = sx > 0.0 && *delta_x > 0.0;
                            let can_right = sx < max_sx && *delta_x < 0.0;
                            if can_left || can_right {
                                sx -= step_x;
                            }
                            consumed = true;
                        }
                        // Fallback: standard vertical wheel scrolls horizontal-only viewports (if allowed)
                        if self.scroll_x && !self.scroll_y && *delta_y != 0.0 && *delta_x == 0.0 {
                            let can_left = sx > 0.0 && *delta_y > 0.0;
                            let can_right = sx < max_sx && *delta_y < 0.0;
                            if can_left || can_right {
                                sx -= step_y;
                            }
                            consumed = true;
                        }
                        if consumed {
                            keep = false;
                            self.ui.needs_redraw = true;
                        }
                    }
                    keep
                });
                self.ui.input_events = events;
            }
            
            sx = sx.clamp(0.0, max_sx);
            sy = sy.clamp(0.0, max_sy);

            self.ui.scroll_state.insert(id, (sx, sy));
            (sx, sy)
        } else {
            (0.0, 0.0)
        };

        let clip = [ox, oy, w, h];

        // Glassmorphism/Custom Shaders: emit draw command before background
        let filter_draw = if let Some(ref filter) = self.backdrop_filter {
            let mut blur_radius = 0.0;
            let mut blur_type = zenthra_core::style::blur::Type::Normal;
            let mut brightness = 1.0;
            let mut saturation = 1.0;
            let mut contrast = 1.0;
            let mut opacity = 1.0;

            for f in &filter.filters {
                match f {
                    zenthra_core::Filter::Blur(r, t) => {
                        blur_radius = *r;
                        blur_type = *t;
                    }
                    zenthra_core::Filter::Opacity(o) => {
                        opacity = *o;
                    }
                    zenthra_core::Filter::Saturate(s) => {
                        saturation = *s;
                    }
                    zenthra_core::Filter::Brightness(b) => {
                        brightness = *b;
                    }
                    zenthra_core::Filter::Contrast(c) => {
                        contrast = *c;
                    }
                }
            }

            if blur_radius > 0.0 {
                Some(crate::ui::BackdropBlurDraw {
                    x:           ox,
                    y:           oy,
                    width:       w,
                    height:      h,
                    radius:      self.style.gpu_radius(),
                    blur_radius,
                    clip_rect: clip,
                    brightness,
                    saturation,
                    contrast,
                    opacity,
                    blur_type,
                })
            } else {
                None
            }
        } else if let Some(blur_radius) = self.backdrop_blur {
            Some(crate::ui::BackdropBlurDraw {
                x:           ox,
                y:           oy,
                width:       w,
                height:      h,
                radius:      self.style.gpu_radius(),
                blur_radius,
                clip_rect: clip,
                brightness: 1.12,
                saturation: 1.20,
                contrast: 1.0,
                opacity: 1.0,
                blur_type: zenthra_core::style::blur::Type::Glassmorphism, // fallback
            })
        } else {
            None
        };

        let push_overlay = self.is_overlay && !self.ui.skip_clip_stack.last().cloned().unwrap_or(false);

        if let Some(shader_id) = self.post_process_shader {
            let blur_radius = if let Some(ref filter) = self.backdrop_filter {
                let mut r = 0.0;
                for f in &filter.filters {
                    if let zenthra_core::Filter::Blur(radius, _) = f {
                        r = *radius;
                    }
                }
                Some(r)
            } else {
                None
            }.or(self.backdrop_blur).unwrap_or(0.0);
            let cp = crate::ui::CustomPostProcessDraw {
                x:           ox,
                y:           oy,
                width:       w,
                height:      h,
                radius:      self.style.gpu_radius(),
                blur_radius,
                shader_id,
                clip_rect: clip,
                params: self.post_process_params,
            };
            if push_overlay {
                self.ui.overlays.push(DrawCommand::CustomPostProcess(cp));
            } else {
                self.ui.draws.push(DrawCommand::CustomPostProcess(cp));
            }
        } else if let Some(bd) = filter_draw {
            if push_overlay {
                self.ui.overlays.push(DrawCommand::BackdropBlur(bd));
            } else {
                self.ui.draws.push(DrawCommand::BackdropBlur(bd));
            }
        }

        // Background, Border & Shadow (Single source via BoxStyleExt)
        {
            let (effective_bg, effective_bw, effective_bc, final_scale) = if container_active {
                (
                    self.active_bg.as_ref().or(self.hover_bg.as_ref()).or(self.style.bg.as_ref()),
                    self.active_border_width.or(self.hover_border_width).unwrap_or(border_width),
                    self.active_border_color.or(self.hover_border_color).or(border_color),
                    self.active_scale,
                )
            } else if container_hover {
                (
                    self.hover_bg.as_ref().or(self.style.bg.as_ref()),
                    self.hover_border_width.unwrap_or(border_width),
                    self.hover_border_color.or(border_color),
                    self.hover_scale,
                )
            } else {
                (
                    self.style.bg.as_ref(),
                    border_width,
                    border_color,
                    1.0,
                )
            };

            if let Some(rect_inst) = self.style.build_rect_styled(
                ox,
                oy,
                w,
                h,
                effective_bg,
                effective_bc,
                Some(effective_bw),
                final_scale,
            ) {
                if push_overlay {
                    self.ui.overlays.push(DrawCommand::Rect(RectDraw { instance: rect_inst }));
                } else {
                    self.ui.draws.push(DrawCommand::Rect(RectDraw { instance: rect_inst }));
                }
            }
        }

        // Final Translation pass (Always run to keep cache in sync)
        for (i, (start, end)) in self.child_ranges.iter().enumerate() {
            if i >= target_positions.len() { break; }
            let (tx, ty) = target_positions[i];
            
            let (origin_x, origin_y) = self.child_origins.get(i).copied().unwrap_or_else(|| {
                self.children_draws
                    .get(*start)
                    .map(|d| draw_origin(d))
                    .unwrap_or((ox + padding_left + border_width, oy + padding_top + border_width))
            });

            let dx = tx - origin_x;
            let dy = ty - origin_y;
            
            let final_dx = dx - scroll_x;
            let final_dy = dy - scroll_y;

            // Shift visual commands
            for draw in &mut self.children_draws[*start..*end] {
                offset_draw(draw, final_dx, final_dy);
                if self.scroll_x || self.scroll_y || actual_clip {
                    set_clip(draw, clip);
                }
            }
            
            // Shift IDs RECURSIVELY (Layout shift only, scrolling is handled live)
            if let Some(&(ids_start, ids_end)) = child_id_ranges.get(i) {
                for j in ids_start..ids_end {
                    let cid = child_ids_only[j];
                    if let Some((rect, _)) = self.ui.next_layout_cache.get_mut(&cid) {
                        rect.origin.x += dx;
                        rect.origin.y += dy;
                    }
                    if let Some(rect) = self.ui.next_screen_layout_cache.get_mut(&cid) {
                        rect.origin.x += dx;
                        rect.origin.y += dy;
                    }
                }
            }
        }

        if self.draggable_window && container_active {
            let is_interactive_hovered = self.ui.semantic_nodes.iter().any(|node| {
                match node.role {
                    zenthra_core::Role::Button |
                    zenthra_core::Role::Link |
                    zenthra_core::Role::CheckBox |
                    zenthra_core::Role::RadioButton |
                    zenthra_core::Role::TextInput |
                    zenthra_core::Role::TextArea |
                    zenthra_core::Role::Slider |
                    zenthra_core::Role::Switch |
                    zenthra_core::Role::Menu => {
                        if let Some(rect) = self.ui.screen_layout_cache.get(&node.id)
                            .or_else(|| self.ui.next_screen_layout_cache.get(&node.id))
                        {
                            rect.contains(zenthra_core::Point::new(self.ui.mouse_x, self.ui.mouse_y))
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            });

            if !is_interactive_hovered {
                self.ui.drag();
            }
        }


        // Flush children draws to parent
        let push_overlay = self.is_overlay && !self.ui.skip_clip_stack.last().cloned().unwrap_or(false);
        for draw in self.children_draws.drain(..) {
            if push_overlay {
                self.ui.overlays.push(draw);
            } else {
                self.ui.draws.push(draw);
            }
        }
        
        // --- Visual Scrollbars & Dragging ---
        if self.scroll_x || self.scroll_y {
            let bar_thickness = 4.0;
            let bar_margin = 2.0;

            // Vertical Scrollbar
            if self.show_scrollbars && self.scroll_y && max_sy > 0.0 {
                let thumb_h = (h / (content_h + padding_top + padding_bottom)) * h;
                let thumb_h = thumb_h.max(20.0);
                let scroll_ratio = scroll_y / max_sy;
                let is_hover = !self.ui.resize_active
                    && self.ui.mouse_in_rect(actual_ox + w - bar_thickness - bar_margin - 2.0, actual_oy + (h - thumb_h) * scroll_ratio, bar_thickness + 4.0, thumb_h)
                    && !self.ui.is_occluded(id, self.ui.mouse_x, self.ui.mouse_y);
                let is_dragging = !self.ui.resize_active
                    && self.ui.active_drag.as_ref().map(|d| d.id == id && d.start_mouse <= -1000.0).unwrap_or(false); 

                // Handle Drag Y
                if !self.ui.resize_active && self.ui.clicked && is_hover {
                    self.ui.active_drag = Some(crate::ui::ScrollDrag {
                        id,
                        start_mouse: -1000.0 - self.ui.mouse_y, 
                        start_scroll: scroll_y,
                    });
                }
                
                if let Some(drag) = &self.ui.active_drag {
                    if !self.ui.resize_active && drag.id == id && drag.start_mouse <= -1000.0 {
                        let current_marker = -1000.0 - self.ui.mouse_y;
                        let delta = drag.start_mouse - current_marker;
                        let scroll_range = h - thumb_h;
                        if scroll_range > 0.0 {
                            let new_sy = (drag.start_scroll + (delta / scroll_range) * max_sy).clamp(0.0, max_sy);
                            self.ui.scroll_state.insert(id, (scroll_x, new_sy));
                            self.ui.needs_redraw = true;
                        }
                    }
                }

                {
                    let color = if is_hover || is_dragging { 
                        Color::rgba(1.0, 1.0, 1.0, 0.5) 
                    } else if container_hover && !self.ui.resize_active {
                        Color::rgba(1.0, 1.0, 1.0, 0.2)
                    } else {
                        Color::rgba(1.0, 1.0, 1.0, 0.1)
                    };
                    
                    // Use layout coordinates (ox, oy) for drawing so they are shifted by parents correctly.
                    // But interaction uses actual_ox/actual_oy (screen space).
                    let visual_thumb_x = ox + w - bar_thickness - bar_margin - border_width;
                    let visual_thumb_y = oy + (h - thumb_h) * scroll_ratio;

                    let draw = crate::ui::DrawCommand::OverlayRect(crate::ui::OverlayRectDraw {
                        x: visual_thumb_x,
                        y: visual_thumb_y,
                        width: bar_thickness,
                        height: thumb_h,
                        color,
                        clip: [ox, oy, w, h], 
                    });
                    if push_overlay { self.ui.overlays.push(draw); } else { self.ui.draws.push(draw); }
                }
            }

            // Horizontal Scrollbar
            if self.show_scrollbars && self.scroll_x && max_sx > 0.0 {
                let thumb_w = (w / (content_w + padding_left + padding_right)) * w;
                let thumb_w = thumb_w.max(20.0);
                let scroll_ratio = scroll_x / max_sx;
                let is_hover = !self.ui.resize_active
                    && self.ui.mouse_in_rect(actual_ox + (w - thumb_w) * scroll_ratio, actual_oy + h - bar_thickness - bar_margin - 2.0, thumb_w, bar_thickness + 4.0)
                    && !self.ui.is_occluded(id, self.ui.mouse_x, self.ui.mouse_y);
                let is_dragging = !self.ui.resize_active
                    && self.ui.active_drag.as_ref().map(|d| d.id == id && d.start_mouse > -1000.0).unwrap_or(false);

                if !self.ui.resize_active && self.ui.clicked && is_hover {
                    self.ui.active_drag = Some(crate::ui::ScrollDrag {
                        id,
                        start_mouse: self.ui.mouse_x,
                        start_scroll: scroll_x,
                    });
                }

                if let Some(drag) = &self.ui.active_drag {
                    if !self.ui.resize_active && drag.id == id && drag.start_mouse > -1000.0 {
                        let delta = self.ui.mouse_x - drag.start_mouse;
                        let scroll_range = w - thumb_w;
                        if scroll_range > 0.0 {
                            let new_sx = (drag.start_scroll + (delta / scroll_range) * max_sx).clamp(0.0, max_sx);
                            let (_, current_sy) = *self.ui.scroll_state.get(&id).unwrap_or(&(0.0, 0.0));
                            self.ui.scroll_state.insert(id, (new_sx, current_sy));
                            self.ui.needs_redraw = true;
                        }
                    }
                }

                {
                    let color = if is_hover || is_dragging { 
                        Color::rgba(1.0, 1.0, 1.0, 0.5) 
                    } else if container_hover && !self.ui.resize_active {
                        Color::rgba(1.0, 1.0, 1.0, 0.2)
                    } else {
                        Color::rgba(1.0, 1.0, 1.0, 0.1)
                    };
                    
                    let visual_thumb_x = ox + (w - thumb_w) * scroll_ratio;
                    let visual_thumb_y = oy + h - bar_thickness - bar_margin - border_width;

                    let draw = crate::ui::DrawCommand::OverlayRect(crate::ui::OverlayRectDraw {
                        x: visual_thumb_x,
                        y: visual_thumb_y,
                        width: thumb_w,
                        height: bar_thickness,
                        color,
                        clip: [ox, oy, w, h],
                    });
                    if push_overlay { self.ui.overlays.push(draw); } else { self.ui.draws.push(draw); }
                }
            }
        }

        // Bubble IDs up to parent's scope
        self.ui.id_log.extend(child_ids_only);
        
        if !self.is_absolute {
            self.ui.advance(w + margin_left + margin_right, h + margin_top + margin_bottom, draw_start);
        }

        // Dispatch events
        self.ui.dispatch_event(id, crate::ui::WidgetEvent::Hover(container_hover));

        if self.ui.clicked && container_hover {
            self.ui.dispatch_event(id, crate::ui::WidgetEvent::Click);
        }

        let events = std::mem::take(&mut self.ui.input_events);
        for event in &events {
            if let zenthra_platform::event::PlatformEvent::MouseWheel { delta_x, delta_y } = event {
                if container_hover {
                    self.ui.dispatch_event(id, crate::ui::WidgetEvent::Scroll(*delta_x, *delta_y));
                }
            }
        }
        self.ui.input_events = events;

        zenthra_core::Response {
            clicked: self.ui.clicked && container_hover,
            hovered: container_hover,
            pressed: container_active,
            submitted: false,
            changed: false,
        }
    }

    fn layout_no_wrap(
        &self,
        ox: f32,
        oy: f32,
        inner_w: f32,
        inner_h: f32,
        targets: &mut [(f32, f32)],
    ) -> (f32, f32) {
        let n = self.child_sizes.len();
        if n == 0 { return (0.0, 0.0); }

        let (content_w, content_h) = match self.direction {
            Direction::Row => {
                let w = self.child_sizes.iter().map(|(w, _)| w).sum::<f32>()
                    + self.gap * (n.saturating_sub(1)) as f32;
                let h = self.child_sizes.iter().map(|(_, h)| *h).fold(0.0f32, f32::max);
                (w, h)
            }
            Direction::Column => {
                let w = self.child_sizes.iter().map(|(w, _)| *w).fold(0.0f32, f32::max);
                let h = self.child_sizes.iter().map(|(_, h)| h).sum::<f32>()
                    + self.gap * (n.saturating_sub(1)) as f32;
                (w, h)
            }
            Direction::Stack => {
                let w = self.child_sizes.iter().map(|(w, _)| *w).fold(0.0f32, f32::max);
                let h = self.child_sizes.iter().map(|(_, h)| *h).fold(0.0f32, f32::max);
                (w, h)
            }
        };

        let real_w = if self.style.width.is_some() || self.style.fill_x { inner_w } else { content_w };
        let real_h = if self.style.height.is_some() || self.style.fill_y { inner_h } else { content_h };

        match self.target_halign_valign(ox, oy, real_w, real_h, content_w, content_h, targets) {
            (w, h) => (w, h),
        }
    }
    
    fn target_halign_valign(&self, ox: f32, oy: f32, real_w: f32, real_h: f32, content_w: f32, content_h: f32, targets: &mut [(f32, f32)]) -> (f32, f32) {
        let padding_left = self.style.padding.left;
        let padding_top = self.style.padding.top;
        let border_width = self.style.border_width();
        let n = self.child_sizes.len();
        match self.direction {
            Direction::Row => {
                let extra = (real_w - content_w).max(0.0);
                let mut cx = ox + padding_left + border_width + match self.halign {
                    Align::Left => 0.0,
                    Align::Center => extra / 2.0,
                    Align::Right => extra,
                    Align::SpaceBetween => 0.0,
                    Align::SpaceAround => extra / (n as f32 * 2.0),
                    _ => 0.0,
                };
                let gap = match self.halign {
                    Align::SpaceBetween if n > 1 => extra / (n - 1) as f32,
                    Align::SpaceAround => extra / n as f32,
                    _ => self.gap,
                };
                for (i, (cw, ch)) in self.child_sizes.iter().enumerate() {
                    let cy = oy + padding_top + border_width + match self.valign {
                        Align::Top => 0.0,
                        Align::Center => (real_h - ch) / 2.0,
                        Align::Bottom => real_h - ch,
                        _ => 0.0,
                    };
                    targets[i] = (cx, cy);
                    cx += cw + gap;
                }
            }
            Direction::Column => {
                let extra = (real_h - content_h).max(0.0);
                let mut cy = oy + padding_top + border_width + match self.valign {
                    Align::Top => 0.0,
                    Align::Center => extra / 2.0,
                    Align::Bottom => extra,
                    Align::SpaceBetween => 0.0,
                    Align::SpaceAround => extra / (n as f32 * 2.0),
                    _ => 0.0,
                };
                let gap = match self.valign {
                    Align::SpaceBetween if n > 1 => extra / (n - 1) as f32,
                    Align::SpaceAround => extra / n as f32,
                    _ => self.gap,
                };
                for (i, (cw, ch)) in self.child_sizes.iter().enumerate() {
                    let cx = ox + padding_left + border_width + match self.halign {
                        Align::Left => 0.0,
                        Align::Center => (real_w - cw) / 2.0,
                        Align::Right => real_w - cw,
                        _ => 0.0,
                    };
                    targets[i] = (cx, cy);
                    cy += ch + gap;
                }
            }
            Direction::Stack => {
                for (i, (cw, ch)) in self.child_sizes.iter().enumerate() {
                    let cx = ox + padding_left + border_width + match self.halign {
                        Align::Center => (real_w - cw).max(0.0) / 2.0,
                        Align::Right => real_w - cw,
                        _ => 0.0,
                    };
                    let cy = oy + padding_top + border_width + match self.valign {
                        Align::Center => (real_h - ch).max(0.0) / 2.0,
                        Align::Bottom => real_h - ch,
                        _ => 0.0,
                    };
                    targets[i] = (cx, cy);
                }
            }
        }
        (content_w, content_h)
    }

    fn layout_wrap(
        &self,
        ox: f32,
        oy: f32,
        inner_w: f32,
        inner_h: f32,
        targets: &mut [(f32, f32)],
    ) -> (f32, f32) {
        let padding_left = self.style.padding.left;
        let padding_top = self.style.padding.top;
        let border_width = self.style.border_width();
        let has_width = self.style.width.is_some() || self.style.fill_x;
        let has_height = self.style.height.is_some() || self.style.fill_y;

        let n = self.child_sizes.len();
        if n == 0 { return (0.0, 0.0); }

        let (main_reversed, cross_reversed) = match self.wrap {
            Wrap::Wrap | Wrap::Auto => (false, false),
            Wrap::WrapReverse => (false, true),
            Wrap::RightToLeft => (true, false),
            Wrap::RightToLeftReverse => (true, true),
            _ => (false, false),
        };

        match self.direction {
            Direction::Row => {
                let mut rows: Vec<Vec<usize>> = Vec::new();
                let mut current_row = Vec::new();
                let mut row_w = 0.0f32;

                for (i, (cw, _)) in self.child_sizes.iter().enumerate() {
                    let needed = if current_row.is_empty() { *cw } else { row_w + self.gap + cw };
                    if needed > inner_w && !current_row.is_empty() {
                        rows.push(std::mem::take(&mut current_row));
                        current_row.push(i);
                        row_w = *cw;
                    } else {
                        current_row.push(i);
                        row_w = needed;
                    }
                }
                if !current_row.is_empty() { rows.push(current_row); }

                let row_heights: Vec<f32> = rows.iter().map(|row| {
                    row.iter().map(|&i| self.child_sizes[i].1).fold(0.0f32, f32::max)
                }).collect();

                let total_h = row_heights.iter().sum::<f32>() + self.gap * (rows.len().saturating_sub(1)) as f32;
                let max_row_w = rows.iter().map(|row| {
                    row.iter().map(|&i| self.child_sizes[i].0).sum::<f32>() + self.gap * (row.len().saturating_sub(1)) as f32
                }).fold(0.0f32, f32::max);

                let real_h = if has_height { inner_h } else { total_h };
                let real_w = if has_width { inner_w } else { max_row_w };

                let mut cy = if cross_reversed {
                    oy + padding_top + border_width + match self.valign {
                        Align::Top => total_h,
                        Align::Center => (real_h + total_h) / 2.0,
                        Align::Bottom => real_h,
                        _ => total_h,
                    }
                } else {
                    oy + padding_top + border_width + match self.valign {
                        Align::Center => (real_h - total_h).max(0.0) / 2.0,
                        Align::Bottom => (real_h - total_h).max(0.0),
                        _ => 0.0,
                    }
                };

                for (ri, row) in rows.iter().enumerate() {
                    let row_h = row_heights[ri];
                    let row_content_w = row.iter().map(|&i| self.child_sizes[i].0).sum::<f32>() + self.gap * (row.len().saturating_sub(1)) as f32;
                    let extra = (real_w - row_content_w).max(0.0);
                    if cross_reversed { cy -= row_h; }

                    let mut cx = if main_reversed {
                        ox + padding_left + border_width + match self.halign {
                            Align::Left => row_content_w,
                            Align::Center => (real_w + row_content_w) / 2.0,
                            Align::Right => real_w,
                            _ => row_content_w,
                        }
                    } else {
                        ox + padding_left + border_width + match self.halign {
                            Align::Left => 0.0,
                            Align::Center => extra / 2.0,
                            Align::Right => extra,
                            Align::SpaceBetween => 0.0,
                            Align::SpaceAround => extra / (row.len() as f32 * 2.0),
                            _ => 0.0,
                        }
                    };

                    let gap = match self.halign {
                        Align::SpaceBetween if row.len() > 1 && !main_reversed => extra / (row.len() - 1) as f32,
                        Align::SpaceAround if !main_reversed => extra / row.len() as f32,
                        _ => self.gap,
                    };

                    for &ci in row {
                        let (cw, ch) = self.child_sizes[ci];
                        if main_reversed { cx -= cw; }
                        let child_y = cy + match self.valign {
                            Align::Center => (row_h - ch) / 2.0,
                            Align::Bottom => row_h - ch,
                            _ => 0.0,
                        };
                        targets[ci] = (cx, child_y);
                        if main_reversed { cx -= gap; } else { cx += cw + gap; }
                    }
                    if cross_reversed { cy -= self.gap; } else { cy += row_h + self.gap; }
                }
                (max_row_w, total_h)
            }
            Direction::Column => {
                let mut cols: Vec<Vec<usize>> = Vec::new();
                let mut current_col = Vec::new();
                let mut col_h = 0.0f32;

                for (i, (_, ch)) in self.child_sizes.iter().enumerate() {
                    let needed = if current_col.is_empty() { *ch } else { col_h + self.gap + ch };
                    if needed > inner_h && !current_col.is_empty() {
                        cols.push(std::mem::take(&mut current_col));
                        current_col.push(i);
                        col_h = *ch;
                    } else {
                        current_col.push(i);
                        col_h = needed;
                    }
                }
                if !current_col.is_empty() { cols.push(current_col); }

                let col_widths: Vec<f32> = cols.iter().map(|col| {
                    col.iter().map(|&i| self.child_sizes[i].0).fold(0.0f32, f32::max)
                }).collect();

                let total_w = col_widths.iter().sum::<f32>() + self.gap * (cols.len().saturating_sub(1)) as f32;
                let max_col_h = cols.iter().map(|col| {
                    col.iter().map(|&i| self.child_sizes[i].1).sum::<f32>() + self.gap * (col.len().saturating_sub(1)) as f32
                }).fold(0.0f32, f32::max);

                let real_w = if has_width { inner_w } else { total_w };
                let real_h = if has_height { inner_h } else { max_col_h };

                let mut cx = if cross_reversed {
                    ox + padding_left + border_width + match self.halign {
                        Align::Left => total_w,
                        Align::Center => (real_w + total_w) / 2.0,
                        Align::Right => real_w,
                        _ => total_w,
                    }
                } else {
                    ox + padding_left + border_width + match self.halign {
                        Align::Center => (real_w - total_w).max(0.0) / 2.0,
                        Align::Right => (real_w - total_w).max(0.0),
                        _ => 0.0,
                    }
                };

                for (ci_idx, col) in cols.iter().enumerate() {
                    let col_w = col_widths[ci_idx];
                    let col_content_h = col.iter().map(|&i| self.child_sizes[i].1).sum::<f32>() + self.gap * (col.len().saturating_sub(1)) as f32;
                    let extra = (real_h - col_content_h).max(0.0);
                    if cross_reversed { cx -= col_w; }

                    let mut cy = if main_reversed {
                        oy + padding_top + border_width + match self.valign {
                            Align::Top => col_content_h,
                            Align::Center => (real_h + col_content_h) / 2.0,
                            Align::Bottom => real_h,
                            _ => col_content_h,
                        }
                    } else {
                        oy + padding_top + border_width + match self.valign {
                            Align::Top => 0.0,
                            Align::Center => extra / 2.0,
                            Align::Bottom => extra,
                            Align::SpaceBetween => 0.0,
                            Align::SpaceAround => extra / (col.len() as f32 * 2.0),
                            _ => 0.0,
                        }
                    };

                    let gap = match self.valign {
                        Align::SpaceBetween if col.len() > 1 && !main_reversed => extra / (col.len() - 1) as f32,
                        Align::SpaceAround if !main_reversed => extra / col.len() as f32,
                        _ => self.gap,
                    };

                    for &ci in col {
                        let (cw, ch) = self.child_sizes[ci];
                        if main_reversed { cy -= ch; }
                    let child_x = cx + match self.halign {
                        Align::Center => (col_w - cw) / 2.0,
                        Align::Right => col_w - cw,
                        _ => 0.0,
                    };
                        targets[ci] = (child_x, cy);
                        if main_reversed { cy -= gap; } else { cy += ch + gap; }
                    }
                    if cross_reversed { cx -= self.gap; } else { cx += col_w + self.gap; }
                }
                (total_w, max_col_h)
            }
            Direction::Stack => {
                let w = self.child_sizes.iter().map(|(w, _)| *w).fold(0.0f32, f32::max);
                let h = self.child_sizes.iter().map(|(_, h)| *h).fold(0.0f32, f32::max);
                for (i, (cw, ch)) in self.child_sizes.iter().enumerate() {
                    let cx = ox + padding_left + border_width + match self.halign {
                        Align::Center => (inner_w - cw).max(0.0) / 2.0,
                        Align::Right => inner_w - cw,
                        _ => 0.0,
                    };
                    let cy = oy + padding_top + border_width + match self.valign {
                        Align::Center => (inner_h - ch).max(0.0) / 2.0,
                        Align::Bottom => inner_h - ch,
                        _ => 0.0,
                    };
                    targets[i] = (cx, cy);
                }
                (w, h)
            }
        }
    }
}

/// Offset a draw command's position and its clipping rect by (dx, dy).
fn offset_draw(cmd: &mut DrawCommand, dx: f32, dy: f32) {
    match cmd {
        DrawCommand::Rect(r) => {
            r.instance.pos[0] += dx;
            r.instance.pos[1] += dy;
            // Translate the widget's internal clip rect so it stays with the widget
            r.instance.clip_rect[0] += dx;
            r.instance.clip_rect[1] += dy;
        }
        DrawCommand::Text(t) => {
            t.pos[0] += dx;
            t.pos[1] += dy;
            t.clip[0] += dx;
            t.clip[1] += dy;
        }
        DrawCommand::OverlayRect(c) => {
            c.x += dx;
            c.y += dy;
            c.clip[0] += dx;
            c.clip[1] += dy;
        }
        DrawCommand::Image(i) => {
            i.instance.pos[0] += dx;
            i.instance.pos[1] += dy;
            i.instance.clip_rect[0] += dx;
            i.instance.clip_rect[1] += dy;
        }
        DrawCommand::BackdropBlur(b) => {
            b.x += dx;
            b.y += dy;
            b.clip_rect[0] += dx;
            b.clip_rect[1] += dy;
        }
        DrawCommand::CustomPostProcess(b) => {
            b.x += dx;
            b.y += dy;
            b.clip_rect[0] += dx;
            b.clip_rect[1] += dy;
        }
    }
}

fn draw_origin(cmd: &DrawCommand) -> (f32, f32) {
    match cmd {
        DrawCommand::Rect(r) => (r.instance.pos[0], r.instance.pos[1]),
        DrawCommand::Text(t) => (t.pos[0], t.pos[1]),
        DrawCommand::OverlayRect(c) => (c.x, c.y),
        DrawCommand::Image(i) => (i.instance.pos[0], i.instance.pos[1]),
        DrawCommand::BackdropBlur(b) => (b.x, b.y),
        DrawCommand::CustomPostProcess(b) => (b.x, b.y),
    }
}

fn set_clip(cmd: &mut DrawCommand, clip: [f32; 4]) {
    match cmd {
        DrawCommand::Rect(r) => r.instance.clip_rect = intersect_rects(r.instance.clip_rect, clip),
        DrawCommand::Text(t) => t.clip = intersect_rects(t.clip, clip),
        DrawCommand::OverlayRect(c) => c.clip = intersect_rects(c.clip, clip),
        DrawCommand::Image(i) => i.instance.clip_rect = intersect_rects(i.instance.clip_rect, clip),
        DrawCommand::BackdropBlur(b) => b.clip_rect = intersect_rects(b.clip_rect, clip),
        DrawCommand::CustomPostProcess(b) => b.clip_rect = intersect_rects(b.clip_rect, clip),
    }
}

fn intersect_rects(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    let x1 = a[0].max(b[0]);
    let y1 = a[1].max(b[1]);
    let x2 = (a[0] + a[2]).min(b[0] + b[2]);
    let y2 = (a[1] + a[3]).min(b[1] + b[3]);
    
    let w = (x2 - x1).max(0.0);
    let h = (y2 - y1).max(0.0);
    
    [x1, y1, w, h]
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_container_box_style_unification() {
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

        // 1. Inherent/Styleable builder methods write directly to self.style
        let c = ui.container()
            .bg(Color::RED)
            .border((Color::BLUE, 2.0, BorderAlignment::Outside))
            .radius(10.0, 20.0, 30.0, 40.0)
            .shadow(Color::rgba(0.0, 0.0, 0.0, 0.8), 5.0, 5.0, 15.0)
            .padding(8.0, 12.0, 16.0, 20.0)
            .width(250.0)
            .height(150.0);

        assert_eq!(c.style.bg, Some(zenthra_core::Background::Solid(Color::RED)));
        assert_eq!(c.style.border, zenthra_core::Border::Solid(Color::BLUE, 2.0));
        assert_eq!(c.style.border_width(), 2.0);
        assert_eq!(c.style.border_color(), Some(Color::BLUE));
        assert_eq!(c.style.border_alignment, BorderAlignment::Outside);
        assert_eq!(c.style.border_alignment_f32(), 1.0);
        assert_eq!(c.style.radius, [10.0, 20.0, 30.0, 40.0]);
        // GPU RectInstance expects [bl, br, tr, tl] = [40.0, 30.0, 20.0, 10.0]
        assert_eq!(c.style.gpu_radius(), [40.0, 30.0, 20.0, 10.0]);
        assert_eq!(c.style.shadow_color, Some(Color::rgba(0.0, 0.0, 0.0, 0.8)));
        assert_eq!(c.style.shadow_offset, [5.0, 5.0]);
        assert_eq!(c.style.shadow_blur, 15.0);
        assert_eq!(c.style.padding.top, 8.0);
        assert_eq!(c.style.padding.right, 12.0);
        assert_eq!(c.style.padding.bottom, 16.0);
        assert_eq!(c.style.padding.left, 20.0);
        assert_eq!(c.style.width, Some(250.0));
        assert_eq!(c.style.height, Some(150.0));

        let (cx, cy, cw, ch) = c.style.inner_bounds(100.0, 100.0, 250.0, 150.0);
        // padding: (t: 8, r: 12, b: 16, l: 20), border_width: 2.0
        assert_eq!(cx, 100.0 + 20.0 + 2.0); // 122.0
        assert_eq!(cy, 100.0 + 8.0 + 2.0);  // 110.0
        assert_eq!(cw, 250.0 - 20.0 - 12.0 - 4.0); // 214.0
        assert_eq!(ch, 150.0 - 8.0 - 16.0 - 4.0);  // 122.0

        let rect_opt = c.style.build_rect(100.0, 100.0, 250.0, 150.0);
        assert!(rect_opt.is_some());
        let rect = rect_opt.unwrap();
        assert_eq!(rect.pos, [100.0, 100.0]);
        assert_eq!(rect.size, [250.0, 150.0]);
        assert_eq!(rect.border_width, 2.0);

        // 2. Direct style injection via .style(BoxStyle)
        let mut custom_style = BoxStyle::new();
        custom_style.fill_x = true;
        custom_style.fill_y = true;
        custom_style.radius = [5.0, 5.0, 5.0, 5.0];

        let c2 = ui.container().style(custom_style);
        assert!(c2.style.fill_x);
        assert!(c2.style.fill_y);
        assert_eq!(c2.style.gpu_radius(), [5.0, 5.0, 5.0, 5.0]);

        // 3. Container show renders quad with BoxStyle
        ui.container()
            .bg(Color::rgb(0.1, 0.2, 0.3))
            .border((Color::WHITE, 1.5))
            .radius_all(12.0)
            .shadow(Color::rgba(0.0, 0.0, 0.0, 0.25), 0.0, 4.0, 8.0)
            .width(200.0)
            .height(100.0)
            .show(|_| {});

        assert!(!ui.draws.is_empty());
        let rect_draw = ui.draws.iter().find_map(|d| match d {
            DrawCommand::Rect(r) => Some(&r.instance),
            _ => None,
        }).expect("Container background RectInstance should be drawn");

        assert_eq!(rect_draw.size, [200.0, 100.0]);
        assert_eq!(rect_draw.radius, [12.0, 12.0, 12.0, 12.0]);
        assert_eq!(rect_draw.border_width, 1.5);
        assert_eq!(rect_draw.border_color, Color::WHITE.to_array());
        assert_eq!(rect_draw.shadow_offset, [0.0, 4.0]);
        assert_eq!(rect_draw.shadow_blur, 8.0);
        assert_eq!(rect_draw.shadow_color, [0.0, 0.0, 0.0, 0.25]); // Direct alpha from Color::rgba

        // 4. Container with ONLY border (no bg) still renders border cleanly
        ui.draws.clear();
        ui.container()
            .border((Color::RED, 2.0))
            .width(100.0)
            .height(50.0)
            .show(|_| {});

        let border_only_draw = ui.draws.iter().find_map(|d| match d {
            DrawCommand::Rect(r) => Some(&r.instance),
            _ => None,
        }).expect("Container with only border should render a RectInstance");

        assert_eq!(border_only_draw.color, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(border_only_draw.border_width, 2.0);
        assert_eq!(border_only_draw.border_color, Color::RED.to_array());

        // 5. Container hover_border(Border::None) correctly sets zero border
        let c_hover = ui.container()
            .border((Color::WHITE, 2.0))
            .hover_border(zenthra_core::Border::None)
            .active_border(zenthra_core::Border::None);

        assert_eq!(c_hover.hover_border_width, Some(0.0));
        assert_eq!(c_hover.hover_border_color, Some(Color::TRANSPARENT));
        assert_eq!(c_hover.active_border_width, Some(0.0));
        assert_eq!(c_hover.active_border_color, Some(Color::TRANSPARENT));
    }
}


