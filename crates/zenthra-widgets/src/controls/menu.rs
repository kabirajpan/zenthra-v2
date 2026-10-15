// crates/zenthra-widgets/src/controls/menu.rs

use crate::style_ext::BoxStyleExt;
use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{
    Background, Border, BoxStyle, Color, EdgeInsets, Id, IntoBorder, Rect, Response, Role,
    SemanticNode, Styleable,
};

pub const ACTIVE_MENU_KEY: Id = Id::from_u64(999999900);
pub const ACTIVE_SUBMENU_KEY: Id = Id::from_u64(999999901);
pub const HOVER_FLAG_KEY: Id = Id::from_u64(999999902);
pub const MENU_DEPTH_KEY: Id = Id::from_u64(999999903);
pub const CURRENT_MENU_ITEM_WIDTH_KEY: Id = Id::from_u64(999999904);
pub const CURRENT_MENU_X_KEY: Id = Id::from_u64(999999905);
pub const CURRENT_MENU_W_KEY: Id = Id::from_u64(999999906);
pub const DEEP_SUBMENU_BASE_KEY: u64 = 999999910;

#[inline]
pub fn get_submenu_key(sub_depth: usize) -> Id {
    if sub_depth <= 1 {
        ACTIVE_SUBMENU_KEY
    } else {
        Id::from_u64(DEEP_SUBMENU_BASE_KEY + (sub_depth as u64 - 2))
    }
}

pub fn close_all_menus(ui: &mut Ui) {
    ui.interaction_state.insert(ACTIVE_MENU_KEY, 0.0);
    ui.interaction_state.insert(ACTIVE_SUBMENU_KEY, 0.0);
    for d in 0..20 {
        ui.interaction_state.insert(Id::from_u64(DEEP_SUBMENU_BASE_KEY + d), 0.0);
    }
}

fn get_theme_color(ui: &Ui, base_id: u64, default: Color) -> Color {
    let r = ui.interaction_state.get(&Id::from_u64(base_id)).copied();
    let g = ui.interaction_state.get(&Id::from_u64(base_id + 1)).copied();
    let b = ui.interaction_state.get(&Id::from_u64(base_id + 2)).copied();
    let a = ui.interaction_state.get(&Id::from_u64(base_id + 3)).copied();

    if let (Some(r), Some(g), Some(b), Some(a)) = (r, g, b, a) {
        Color::rgba(r, g, b, a)
    } else {
        default
    }
}

// =====================================================================
// MENU BAR BUILDER
// =====================================================================

/// Builder for a horizontal menu bar container (`ui.menu_bar()`)
pub struct MenuBarBuilder<'u, 'a> {
    pub ui: &'u mut Ui<'a>,
    pub style: BoxStyle,
}

impl<'u, 'a> Styleable for MenuBarBuilder<'u, 'a> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

impl_box_style_methods!(MenuBarBuilder);

impl<'u, 'a> MenuBarBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        let mut style = BoxStyle::new();
        style.height = Some(30.0);
        style.fill_x = true;
        style.padding = EdgeInsets::symmetric(0.0, 8.0);
        Self { ui, style }
    }

    /// Renders the horizontal menu bar container and executes child menu builders
    pub fn show<F>(self, f: F)
    where
        F: FnOnce(&mut Ui),
    {
        // Reset menu depth for this frame
        self.ui.interaction_state.insert(MENU_DEPTH_KEY, 0.0);

        let default_bg = {
            let is_light_theme = self
                .ui
                .interaction_state
                .get(&zenthra_core::Id::from_u64(999999999))
                .copied()
                .unwrap_or(0.0)
                > 0.5;
            if is_light_theme {
                Background::Solid(Color::rgb(0.90, 0.90, 0.92))
            } else {
                Background::Solid(Color::rgb(2.0 / 255.0, 2.0 / 255.0, 2.0 / 255.0))
            }
        };

        let mut container = self
            .ui
            .container()
            .fill_x()
            .row()
            .style(self.style.clone());

        if container.style.bg.is_none() {
            container.style.bg = Some(default_bg);
        }

        container.show(|ui| {
            f(ui);
        });
    }
}

// =====================================================================
// MENU BUILDER (TOP-LEVEL & SUBMENU TRIGGER)
// =====================================================================

/// Builder for dropdown menus and nested submenus (`ui.menu("File")`)
pub struct MenuBuilder<'u, 'a> {
    pub ui: &'u mut Ui<'a>,
    pub label: String,
    pub id: Id,
    pub placement: zenthra_core::Placement,
    pub align: zenthra_core::PopoverAlign,
    pub gap: f32,
    pub is_submenu: Option<bool>,
    pub icon: Option<String>,

    // Unified Trigger Styling
    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,

    // Unified Popup Container Styling
    pub popup_style: BoxStyle,
}

impl<'u, 'a> Styleable for MenuBuilder<'u, 'a> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

impl_box_style_methods!(MenuBuilder);
impl_text_style_methods!(MenuBuilder);
impl_interaction_state_methods!(MenuBuilder);

impl<'u, 'a> MenuBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, label: &str) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        label.hash(&mut hasher);
        if let Some(parent) = ui.semantic_stack.last() {
            parent.hash(&mut hasher);
        }
        let id = Id::from_u64((hasher.finish() & 0x7FFFFF) + 1);

        let mut style = BoxStyle::new();
        style.height = Some(26.0);
        style.radius = [4.0; 4];
        style.padding = EdgeInsets::symmetric(0.0, 10.0);

        let mut text_style = zenthra_text::prelude::TextStyle::default();
        text_style.font_size = 13.0;

        let mut popup_style = BoxStyle::new();
        popup_style.width = Some(270.0);
        popup_style.radius = [4.0; 4];
        popup_style.padding = EdgeInsets::all(4.0);

        Self {
            ui,
            label: label.to_string(),
            id,
            placement: zenthra_core::Placement::Bottom,
            align: zenthra_core::PopoverAlign::Start,
            gap: 2.0,
            is_submenu: None,
            icon: None,

            style,
            text_style,
            popup_style,
            hover_state: None,
            active_state: None,
        }
    }

    pub fn placement(mut self, placement: zenthra_core::Placement) -> Self {
        self.placement = placement;
        self
    }

    pub fn align(mut self, align: zenthra_core::PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn submenu(mut self, is_sub: bool) -> Self {
        self.is_submenu = Some(is_sub);
        self
    }

    // --- Popup Container Styling ---

    pub fn popup_style(mut self, style: BoxStyle) -> Self {
        self.popup_style = style;
        self
    }

    pub fn popup_bg(mut self, bg: impl Into<Background>) -> Self {
        match bg.into() {
            Background::None => self.popup_style.bg = None,
            other => self.popup_style.bg = Some(other),
        }
        self
    }

    pub fn popup_border(mut self, border: impl IntoBorder) -> Self {
        let (b, align) = border.into_border();
        self.popup_style.border = b;
        if let Some(align) = align {
            self.popup_style.border_alignment = align;
        }
        self
    }

    pub fn popup_radius(mut self, r: f32) -> Self {
        self.popup_style.radius = [r; 4];
        self
    }

    pub fn popup_padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.popup_style.padding = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }

    pub fn popup_padding_all(mut self, p: f32) -> Self {
        self.popup_style.padding = EdgeInsets::all(p);
        self
    }

    pub fn popup_shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.popup_style.shadow_color = Some(color);
        self.popup_style.shadow_offset = [x, y];
        self.popup_style.shadow_blur = blur;
        self
    }

    pub fn menu_width(mut self, width: f32) -> Self {
        self.popup_style.width = Some(width);
        self
    }

    pub fn item_height(mut self, height: f32) -> Self {
        self.style.height = Some(height);
        self
    }

    pub fn show<F>(self, f: F)
    where
        F: FnOnce(&mut Ui),
    {
        let depth = self
            .ui
            .interaction_state
            .get(&MENU_DEPTH_KEY)
            .copied()
            .unwrap_or(0.0) as usize;
        let is_sub = self.is_submenu.unwrap_or(depth > 0);

        if is_sub {
            self.show_submenu(depth, f);
        } else {
            self.show_toplevel(f);
        }
    }

    fn show_toplevel<F>(self, f: F)
    where
        F: FnOnce(&mut Ui),
    {
        let active_menu_id = self
            .ui
            .interaction_state
            .get(&ACTIVE_MENU_KEY)
            .copied()
            .map(|v| v as u64)
            .unwrap_or(0);
        let is_currently_active = active_menu_id == self.id.raw();

        let margin_left = self.style.margin.left;
        let margin_top = self.style.margin.top;
        let margin_bottom = self.style.margin.bottom;

        let x = self.ui.cursor_x + margin_left;
        let y = self.ui.cursor_y + margin_top;

        let icon_extra_w = if self.icon.is_some() { 20.0 } else { 0.0 };
        let mut text_w = self.label.len() as f32 * 7.0;
        if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter =
                zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let options = zenthra_text::prelude::TextOptions::new().size(self.text_style.font_size);
            adapter.set_layout_size(1000.0, 100.0);
            let buffer = adapter.shape(&self.label, &options);
            text_w = buffer.size().0;
        }

        let mut w = self
            .style
            .width
            .unwrap_or(text_w + icon_extra_w + self.style.padding.horizontal());
        let mut h = self.style.height.unwrap_or(26.0);

        if let Some(min_w) = self.style.min_width { w = w.max(min_w); }
        if let Some(max_w) = self.style.max_width { w = w.min(max_w); }
        if let Some(min_h) = self.style.min_height { h = h.max(min_h); }
        if let Some(max_h) = self.style.max_height { h = h.min(max_h); }

        let (actual_ox, actual_oy, actual_w, actual_h) =
            if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
                (
                    rect.origin.x + self.ui.offset_x,
                    rect.origin.y + self.ui.offset_y,
                    rect.size.width.max(w),
                    rect.size.height.max(h),
                )
            } else {
                (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
            };

        let is_hovered = self
            .ui
            .is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);

        if is_hovered {
            self.ui.interaction_state.insert(HOVER_FLAG_KEY, 1.0);
            let eff_cursor = self
                .hover_state
                .as_ref()
                .and_then(|h| h.cursor)
                .unwrap_or(crate::text::CursorIcon::Pointer);
            self.ui.cursor_icon = eff_cursor;

            if let Some(ref tip) = self.hover_state.as_ref().and_then(|h| h.tooltip.as_ref()) {
                self.ui.draw_tooltip(tip);
            }
        }

        // Auto-expand on hover if a top-level menu is already active
        if is_hovered && active_menu_id != 0 && !is_currently_active {
            self.ui
                .interaction_state
                .insert(ACTIVE_MENU_KEY, self.id.raw() as f32);
            self.ui.interaction_state.remove(&ACTIVE_SUBMENU_KEY);
            for d in 0..20 {
                self.ui
                    .interaction_state
                    .remove(&Id::from_u64(DEEP_SUBMENU_BASE_KEY + d));
            }
            self.ui.needs_redraw = true;
        }

        // Toggle on click
        if self.ui.clicked && is_hovered {
            if is_currently_active {
                self.ui.interaction_state.insert(ACTIVE_MENU_KEY, 0.0);
            } else {
                self.ui
                    .interaction_state
                    .insert(ACTIVE_MENU_KEY, self.id.raw() as f32);
            }
            self.ui.interaction_state.remove(&ACTIVE_SUBMENU_KEY);
            for d in 0..20 {
                self.ui
                    .interaction_state
                    .remove(&Id::from_u64(DEEP_SUBMENU_BASE_KEY + d));
            }
            self.ui.consume_click();
            self.ui.needs_redraw = true;
        }

        let is_light_theme = self
            .ui
            .interaction_state
            .get(&Id::from_u64(999999999))
            .copied()
            .unwrap_or(0.0)
            > 0.5;
        let is_glassmorphism = self
            .ui
            .interaction_state
            .get(&Id::from_u64(999999998))
            .copied()
            .unwrap_or(0.0)
            > 0.5;

        let theme_accent = get_theme_color(
            self.ui,
            999999980,
            Color::rgb(255.0 / 255.0, 214.0 / 255.0, 0.0 / 255.0),
        );
        let theme_highlight = get_theme_color(
            self.ui,
            999999970,
            Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.04),
        );
        let theme_text_primary = get_theme_color(
            self.ui,
            999999960,
            Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0),
        );
        let theme_bg_panel = get_theme_color(
            self.ui,
            999999940,
            Color::rgb(0.10, 0.12, 0.17),
        );
        let theme_border = get_theme_color(
            self.ui,
            999999930,
            Color::rgb(0.22, 0.26, 0.36),
        );

        let is_active = is_currently_active || (is_hovered && self.ui.mouse_down);
        let active_overrides = if is_active { self.active_state.as_ref() } else { None };
        let hover_overrides = if is_hovered { self.hover_state.as_ref() } else { None };

        let fallback_bg = if self.style.bg.is_none() {
            if is_currently_active {
                Some(Background::Solid(if is_light_theme {
                    Color::rgb(0.80, 0.80, 0.85)
                } else {
                    theme_highlight
                }))
            } else if is_hovered {
                Some(Background::Solid(if is_light_theme {
                    Color::rgb(0.88, 0.88, 0.90)
                } else {
                    theme_highlight
                }))
            } else {
                None
            }
        } else {
            None
        };

        let effective_bg = active_overrides
            .and_then(|o| o.style.bg.as_ref())
            .or_else(|| hover_overrides.and_then(|o| o.style.bg.as_ref()))
            .or_else(|| self.style.bg.as_ref())
            .or(fallback_bg.as_ref());

        let effective_border_color = active_overrides
            .and_then(|o| o.style.border_color())
            .or_else(|| hover_overrides.and_then(|o| o.style.border_color()))
            .or_else(|| self.style.border_color());

        let effective_border_width = active_overrides
            .map(|o| o.style.border_width())
            .filter(|&bw| bw > 0.0)
            .or_else(|| hover_overrides.map(|o| o.style.border_width()).filter(|&bw| bw > 0.0))
            .unwrap_or_else(|| self.style.border_width());

        let eff_scale = if is_active {
            active_overrides.map(|o| o.scale).filter(|&s| (s - 1.0).abs() > f32::EPSILON).unwrap_or(1.0)
        } else if is_hovered {
            hover_overrides.map(|o| o.scale).filter(|&s| (s - 1.0).abs() > f32::EPSILON).unwrap_or(1.0)
        } else {
            1.0
        };

        let effective_text_color = active_overrides
            .and_then(|o| o.text_color)
            .or_else(|| hover_overrides.and_then(|o| o.text_color))
            .unwrap_or_else(|| {
                if self.text_style.color != Color::WHITE {
                    self.text_style.color
                } else if is_currently_active || is_hovered {
                    if is_light_theme { Color::rgb(0.1, 0.1, 0.15) } else { theme_accent }
                } else if is_light_theme {
                    Color::rgb(0.1, 0.1, 0.15)
                } else {
                    theme_text_primary
                }
            });

        let start_draw = self.ui.draws.len();

        if let Some(instance) = self.style.build_rect_styled(
            x,
            y + 2.0,
            w,
            h,
            effective_bg,
            effective_border_color,
            Some(effective_border_width),
            eff_scale,
        ) {
            self.ui.draws.push(DrawCommand::Rect(RectDraw { instance }));
        }

        let text_start_x = if let Some(icon) = &self.icon {
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: icon.clone(),
                pos: [x + self.style.padding.left, y + (h - self.text_style.font_size) / 2.0],
                options: zenthra_text::prelude::TextOptions::new()
                    .size(self.text_style.font_size)
                    .color(effective_text_color),
                clip: [x, y, w, h + 4.0],
            }));
            x + self.style.padding.left + 18.0
        } else {
            x + (w - text_w) / 2.0
        };

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [text_start_x, y + (h - self.text_style.font_size) / 2.0],
            options: zenthra_text::prelude::TextOptions::new()
                .size(self.text_style.font_size)
                .color(effective_text_color),
            clip: [x, y, w, h + 4.0],
        }));

        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Menu, Rect::new(x, y, w, h + 4.0)),
        );
        self.ui.record_layout(self.id, Rect::new(x, y, w, h + 4.0));
        self.ui.advance(
            w + self.style.margin.horizontal(),
            h + 4.0 + margin_bottom,
            start_draw,
        );

        if is_currently_active {
            let popup_id = Id::from_u64((self.id.raw() << 8) | 10);

            // Check if popup is hovered
            if let Some(rect) = self.ui.screen_layout_cache.get(&popup_id) {
                if self.ui.mouse_in_rect(
                    rect.origin.x,
                    rect.origin.y,
                    rect.size.width,
                    rect.size.height,
                ) {
                    self.ui.interaction_state.insert(HOVER_FLAG_KEY, 1.0);
                }
            }

            // Isolate layout state variables
            let prev_child_ranges = std::mem::take(&mut self.ui.child_draw_ranges);
            let prev_id_ranges = std::mem::take(&mut self.ui.id_ranges);
            let prev_id_log = std::mem::take(&mut self.ui.id_log);
            let prev_child_sizes = std::mem::take(&mut self.ui.child_sizes);
            let prev_child_origins = std::mem::take(&mut self.ui.child_origins);

            let popup_w = self.popup_style.width.unwrap_or(270.0);
            let anchor_rect = Rect::new(actual_ox, actual_oy, actual_w, actual_h);
            let estimated_h = self
                .ui
                .screen_layout_cache
                .get(&popup_id)
                .map(|r| r.size.height)
                .unwrap_or(200.0);
            let (popup_x, popup_y) = crate::controls::popover::compute_placement_pos(
                anchor_rect,
                popup_w,
                estimated_h,
                self.placement,
                self.align,
                self.gap,
                true,
                self.ui.width,
                self.ui.height,
            );

            let pad_h = self.popup_style.padding.horizontal();
            let item_w = (popup_w - pad_h).max(60.0);

            self.ui.interaction_state.insert(CURRENT_MENU_ITEM_WIDTH_KEY, item_w);
            self.ui.interaction_state.insert(CURRENT_MENU_X_KEY, popup_x);
            self.ui.interaction_state.insert(CURRENT_MENU_W_KEY, popup_w);

            self.ui.overlay(|ui| {
                let mut container = ui
                    .container()
                    .raw_id(popup_id)
                    .pos(popup_x, popup_y)
                    .overlay()
                    .clip(false)
                    .style(self.popup_style.clone())
                    .column();

                if container.style.bg.is_none() {
                    container.style.bg = Some(Background::Solid(if is_light_theme {
                        Color::WHITE
                    } else {
                        theme_bg_panel
                    }));
                }
                if container.style.border == Border::None {
                    container.style.border = Border::Solid(
                        if is_light_theme {
                            Color::rgb(0.8, 0.8, 0.85)
                        } else {
                            theme_border
                        },
                        1.0,
                    );
                }
                if is_glassmorphism {
                    container = container.backdrop_filter(
                        zenthra_core::BackdropFilter::new()
                            .blur(15.0, zenthra_core::style::blur::Type::Glassmorphism),
                    );
                }

                container.show(|ui| {
                    ui.interaction_state.insert(MENU_DEPTH_KEY, 1.0);
                    f(ui);
                    ui.interaction_state.insert(MENU_DEPTH_KEY, 0.0);
                    ui.interaction_state.remove(&CURRENT_MENU_ITEM_WIDTH_KEY);
                    ui.interaction_state.remove(&CURRENT_MENU_X_KEY);
                    ui.interaction_state.remove(&CURRENT_MENU_W_KEY);
                });
            });

            // Discard child variables and restore parent's
            let _ = std::mem::take(&mut self.ui.child_draw_ranges);
            let _ = std::mem::take(&mut self.ui.id_ranges);
            let _ = std::mem::take(&mut self.ui.id_log);
            let _ = std::mem::take(&mut self.ui.child_sizes);
            let _ = std::mem::take(&mut self.ui.child_origins);

            self.ui.child_draw_ranges = prev_child_ranges;
            self.ui.id_ranges = prev_id_ranges;
            self.ui.id_log = prev_id_log;
            self.ui.child_sizes = prev_child_sizes;
            self.ui.child_origins = prev_child_origins;

            // Outside-click dismissal for menus
            if self.ui.clicked && !is_hovered {
                let popup_hovered = self
                    .ui
                    .screen_layout_cache
                    .get(&popup_id)
                    .map(|r| {
                        self.ui.mouse_in_rect(
                            r.origin.x,
                            r.origin.y,
                            r.size.width,
                            r.size.height,
                        )
                    })
                    .unwrap_or(false);
                let hover_flag = self
                    .ui
                    .interaction_state
                    .get(&HOVER_FLAG_KEY)
                    .copied()
                    .unwrap_or(0.0)
                    > 0.5;

                if !popup_hovered && !hover_flag {
                    close_all_menus(self.ui);
                    self.ui.needs_redraw = true;
                }
            }
        }
    }

    fn show_submenu<F>(self, depth: usize, f: F)
    where
        F: FnOnce(&mut Ui),
    {
        let sub_depth = depth.max(1);
        let active_key = get_submenu_key(sub_depth);

        let active_submenu_id = self
            .ui
            .interaction_state
            .get(&active_key)
            .copied()
            .map(|v| v as u64)
            .unwrap_or(0);
        let is_currently_active = active_submenu_id == self.id.raw();

        let margin_left = self.style.margin.left;
        let margin_top = self.style.margin.top;

        let (x, y) = (self.ui.cursor_x + margin_left, self.ui.cursor_y + margin_top);
        let default_w = self
            .ui
            .interaction_state
            .get(&CURRENT_MENU_ITEM_WIDTH_KEY)
            .copied()
            .unwrap_or(262.0);
        let mut w = self.style.width.unwrap_or(default_w);
        let mut h = self.style.height.unwrap_or(26.0);

        if let Some(min_w) = self.style.min_width { w = w.max(min_w); }
        if let Some(max_w) = self.style.max_width { w = w.min(max_w); }
        if let Some(min_h) = self.style.min_height { h = h.max(min_h); }
        if let Some(max_h) = self.style.max_height { h = h.min(max_h); }

        let (actual_ox, actual_oy, actual_w, actual_h) =
            if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
                (
                    rect.origin.x + self.ui.offset_x,
                    rect.origin.y + self.ui.offset_y,
                    rect.size.width.max(w),
                    rect.size.height.max(h),
                )
            } else {
                (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
            };

        let is_hovered = self
            .ui
            .is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);

        if is_hovered {
            self.ui.interaction_state.insert(HOVER_FLAG_KEY, 1.0);
            let eff_cursor = self
                .hover_state
                .as_ref()
                .and_then(|h| h.cursor)
                .unwrap_or(crate::text::CursorIcon::Pointer);
            self.ui.cursor_icon = eff_cursor;

            if let Some(ref tip) = self.hover_state.as_ref().and_then(|h| h.tooltip.as_ref()) {
                self.ui.draw_tooltip(tip);
            }
        }

        // Switch on hover if a sibling submenu at this depth is already open
        if is_hovered && active_submenu_id != 0 && !is_currently_active {
            self.ui
                .interaction_state
                .insert(active_key, self.id.raw() as f32);
            for d in (sub_depth + 1)..=20 {
                self.ui.interaction_state.remove(&get_submenu_key(d));
            }
            self.ui.needs_redraw = true;
        }

        // Open and toggle sub menu on click
        if self.ui.clicked && is_hovered {
            if is_currently_active {
                self.ui.interaction_state.insert(active_key, 0.0);
            } else {
                self.ui
                    .interaction_state
                    .insert(active_key, self.id.raw() as f32);
            }
            for d in (sub_depth + 1)..=20 {
                self.ui.interaction_state.remove(&get_submenu_key(d));
            }
            self.ui.consume_click();
            self.ui.needs_redraw = true;
        }

        let is_light_theme = self
            .ui
            .interaction_state
            .get(&Id::from_u64(999999999))
            .copied()
            .unwrap_or(0.0)
            > 0.5;
        let is_glassmorphism = self
            .ui
            .interaction_state
            .get(&Id::from_u64(999999998))
            .copied()
            .unwrap_or(0.0)
            > 0.5;

        let theme_accent = get_theme_color(
            self.ui,
            999999980,
            Color::rgb(255.0 / 255.0, 214.0 / 255.0, 0.0 / 255.0),
        );
        let theme_highlight = get_theme_color(
            self.ui,
            999999970,
            Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.04),
        );
        let theme_text_primary = get_theme_color(
            self.ui,
            999999960,
            Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0),
        );
        let theme_text_muted = get_theme_color(
            self.ui,
            999999950,
            Color::rgb(136.0 / 255.0, 136.0 / 255.0, 136.0 / 255.0),
        );
        let theme_bg_panel = get_theme_color(
            self.ui,
            999999940,
            Color::rgb(0.10, 0.12, 0.17),
        );
        let theme_border = get_theme_color(
            self.ui,
            999999930,
            Color::rgb(0.22, 0.26, 0.36),
        );

        let is_active = is_currently_active || (is_hovered && self.ui.mouse_down);
        let active_overrides = if is_active { self.active_state.as_ref() } else { None };
        let hover_overrides = if is_hovered { self.hover_state.as_ref() } else { None };

        let fallback_bg = if self.style.bg.is_none() {
            if is_currently_active || is_hovered {
                Some(Background::Solid(if is_light_theme {
                    Color::rgb(0.90, 0.90, 0.95)
                } else {
                    theme_highlight
                }))
            } else {
                None
            }
        } else {
            None
        };

        let effective_bg = active_overrides
            .and_then(|o| o.style.bg.as_ref())
            .or_else(|| hover_overrides.and_then(|o| o.style.bg.as_ref()))
            .or_else(|| self.style.bg.as_ref())
            .or(fallback_bg.as_ref());

        let effective_border_color = active_overrides
            .and_then(|o| o.style.border_color())
            .or_else(|| hover_overrides.and_then(|o| o.style.border_color()))
            .or_else(|| self.style.border_color());

        let effective_border_width = active_overrides
            .map(|o| o.style.border_width())
            .filter(|&bw| bw > 0.0)
            .or_else(|| hover_overrides.map(|o| o.style.border_width()).filter(|&bw| bw > 0.0))
            .unwrap_or_else(|| self.style.border_width());

        let eff_scale = if is_active {
            active_overrides.map(|o| o.scale).filter(|&s| (s - 1.0).abs() > f32::EPSILON).unwrap_or(1.0)
        } else if is_hovered {
            hover_overrides.map(|o| o.scale).filter(|&s| (s - 1.0).abs() > f32::EPSILON).unwrap_or(1.0)
        } else {
            1.0
        };

        let effective_text_color = active_overrides
            .and_then(|o| o.text_color)
            .or_else(|| hover_overrides.and_then(|o| o.text_color))
            .unwrap_or_else(|| {
                if self.text_style.color != Color::WHITE {
                    self.text_style.color
                } else if is_currently_active || is_hovered {
                    if is_light_theme { Color::rgb(0.1, 0.1, 0.15) } else { theme_accent }
                } else if is_light_theme {
                    Color::rgb(0.1, 0.1, 0.15)
                } else {
                    theme_text_primary
                }
            });

        let chevron_color = if is_light_theme {
            Color::rgb(0.5, 0.5, 0.5)
        } else if is_currently_active || is_hovered {
            theme_accent
        } else {
            theme_text_muted
        };

        let start_draw = self.ui.draws.len();

        if let Some(instance) = self.style.build_rect_styled(
            x,
            y,
            w,
            h,
            effective_bg,
            effective_border_color,
            Some(effective_border_width),
            eff_scale,
        ) {
            self.ui.draws.push(DrawCommand::Rect(RectDraw { instance }));
        }

        let text_start_x = if let Some(icon) = &self.icon {
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: icon.clone(),
                pos: [x + 8.0, y + (h - self.text_style.font_size) / 2.0],
                options: zenthra_text::prelude::TextOptions::new()
                    .size(self.text_style.font_size)
                    .color(effective_text_color),
                clip: [x, y, w, h],
            }));
            x + 28.0
        } else {
            x + 8.0
        };

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [text_start_x, y + (h - self.text_style.font_size) / 2.0],
            options: zenthra_text::prelude::TextOptions::new()
                .size(self.text_style.font_size)
                .color(effective_text_color),
            clip: [x, y, w - 24.0, h],
        }));

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: crate::icons::NF_FA_CHEVRON_RIGHT.to_string(),
            pos: [x + w - 20.0, y + (h - 10.0) / 2.0],
            options: zenthra_text::prelude::TextOptions::new()
                .size(10.0)
                .color(chevron_color),
            clip: [x, y, w, h],
        }));

        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Button, Rect::new(x, y, w, h)),
        );
        self.ui.record_layout(self.id, Rect::new(x, y, w, h));
        self.ui.advance(
            w + self.style.margin.horizontal(),
            h + self.style.margin.vertical(),
            start_draw,
        );

        if is_currently_active {
            let sub_popup_id =
                Id::from_u64((self.id.raw() << 8) | (11 + sub_depth as u64));

            // Check if sub-popup is hovered
            if let Some(rect) = self.ui.screen_layout_cache.get(&sub_popup_id) {
                if self.ui.mouse_in_rect(
                    rect.origin.x,
                    rect.origin.y,
                    rect.size.width,
                    rect.size.height,
                ) {
                    self.ui.interaction_state.insert(HOVER_FLAG_KEY, 1.0);
                }
            }

            // Isolate layout state variables
            let prev_child_ranges = std::mem::take(&mut self.ui.child_draw_ranges);
            let prev_id_ranges = std::mem::take(&mut self.ui.id_ranges);
            let prev_id_log = std::mem::take(&mut self.ui.id_log);
            let prev_child_sizes = std::mem::take(&mut self.ui.child_sizes);
            let prev_child_origins = std::mem::take(&mut self.ui.child_origins);

            let sub_w = self.popup_style.width.unwrap_or(220.0);
            let parent_x = self.ui.interaction_state.get(&CURRENT_MENU_X_KEY).copied().unwrap_or(actual_ox);
            let parent_w = self.ui.interaction_state.get(&CURRENT_MENU_W_KEY).copied().unwrap_or(actual_w + 8.0);
            let gap = 2.0;

            let sub_x = if parent_x + parent_w + sub_w + 4.0 > self.ui.width as f32 {
                (parent_x - sub_w - gap).max(4.0)
            } else {
                parent_x + parent_w + gap
            };

            let estimated_h = self
                .ui
                .screen_layout_cache
                .get(&sub_popup_id)
                .map(|r| r.size.height)
                .unwrap_or(150.0);
            let max_y = (self.ui.height as f32 - estimated_h - 8.0).max(4.0);
            let sub_y = (actual_oy - 4.0).min(max_y).max(4.0);

            let prev_menu_item_w = self.ui.interaction_state.get(&CURRENT_MENU_ITEM_WIDTH_KEY).copied();
            let prev_menu_x = self.ui.interaction_state.get(&CURRENT_MENU_X_KEY).copied();
            let prev_menu_w = self.ui.interaction_state.get(&CURRENT_MENU_W_KEY).copied();

            let sub_pad_h = self.popup_style.padding.horizontal();
            let sub_item_w = (sub_w - sub_pad_h).max(60.0);
            self.ui.interaction_state.insert(CURRENT_MENU_ITEM_WIDTH_KEY, sub_item_w);
            self.ui.interaction_state.insert(CURRENT_MENU_X_KEY, sub_x);
            self.ui.interaction_state.insert(CURRENT_MENU_W_KEY, sub_w);

            let start_draws = self.ui.draws.len();

            self.ui.overlay(|ui| {
                let mut container = ui
                    .container()
                    .raw_id(sub_popup_id)
                    .pos(sub_x, sub_y)
                    .overlay()
                    .clip(false)
                    .style(self.popup_style.clone())
                    .width(sub_w)
                    .column();

                if container.style.bg.is_none() {
                    container.style.bg = Some(Background::Solid(if is_light_theme {
                        Color::WHITE
                    } else {
                        theme_bg_panel
                    }));
                }
                if container.style.border == Border::None {
                    container.style.border = Border::Solid(
                        if is_light_theme {
                            Color::rgb(0.8, 0.8, 0.85)
                        } else {
                            theme_border
                        },
                        1.0,
                    );
                }
                if is_glassmorphism {
                    container = container.backdrop_filter(
                        zenthra_core::BackdropFilter::new()
                            .blur(15.0, zenthra_core::style::blur::Type::Glassmorphism),
                    );
                }

                container.show(|ui| {
                    ui.interaction_state.insert(MENU_DEPTH_KEY, (sub_depth + 1) as f32);
                    f(ui);
                    ui.interaction_state.insert(MENU_DEPTH_KEY, sub_depth as f32);
                });
            });

            // Drain submenu draw commands and register in window_overlays with elevated z-index
            // so they are rendered strictly ON TOP of all parent menu items
            let sub_cmds = self.ui.draws.drain(start_draws..).collect::<Vec<_>>();
            let z_key = Id::from_u64((sub_popup_id.raw() << 8) | 4);
            self.ui.interaction_state.insert(z_key, 1000.0 + (sub_depth as f32 * 100.0));
            self.ui.window_overlays.push((sub_popup_id, sub_cmds));

            // Restore parent menu dimensions in interaction state
            if let Some(v) = prev_menu_item_w {
                self.ui.interaction_state.insert(CURRENT_MENU_ITEM_WIDTH_KEY, v);
            } else {
                self.ui.interaction_state.remove(&CURRENT_MENU_ITEM_WIDTH_KEY);
            }
            if let Some(v) = prev_menu_x {
                self.ui.interaction_state.insert(CURRENT_MENU_X_KEY, v);
            } else {
                self.ui.interaction_state.remove(&CURRENT_MENU_X_KEY);
            }
            if let Some(v) = prev_menu_w {
                self.ui.interaction_state.insert(CURRENT_MENU_W_KEY, v);
            } else {
                self.ui.interaction_state.remove(&CURRENT_MENU_W_KEY);
            }

            // Discard child variables and restore parent's
            let _ = std::mem::take(&mut self.ui.child_draw_ranges);
            let _ = std::mem::take(&mut self.ui.id_ranges);
            let _ = std::mem::take(&mut self.ui.id_log);
            let _ = std::mem::take(&mut self.ui.child_sizes);
            let _ = std::mem::take(&mut self.ui.child_origins);

            self.ui.child_draw_ranges = prev_child_ranges;
            self.ui.id_ranges = prev_id_ranges;
            self.ui.id_log = prev_id_log;
            self.ui.child_sizes = prev_child_sizes;
            self.ui.child_origins = prev_child_origins;
        }
    }
}

// =====================================================================
// MENU ITEM BUILDER
// =====================================================================

/// Builder for interactive items within menus (`ui.menu_item("New File")`)
pub struct MenuItemBuilder<'u, 'a> {
    pub ui: &'u mut Ui<'a>,
    pub label: String,
    pub shortcut: Option<String>,
    pub id: Id,
    pub icon: Option<String>,

    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,
}

impl<'u, 'a> Styleable for MenuItemBuilder<'u, 'a> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

impl_box_style_methods!(MenuItemBuilder);
impl_text_style_methods!(MenuItemBuilder);
impl_interaction_state_methods!(MenuItemBuilder);

impl<'u, 'a> MenuItemBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, label: &str) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        label.hash(&mut hasher);
        if let Some(parent) = ui.semantic_stack.last() {
            parent.hash(&mut hasher);
        }
        let id = Id::from_u64((hasher.finish() & 0x7FFFFF) + 1);

        let mut style = BoxStyle::new();
        style.height = Some(26.0);
        style.width = None;
        style.radius = [3.0; 4];
        style.padding = EdgeInsets::symmetric(0.0, 8.0);

        let mut text_style = zenthra_text::prelude::TextStyle::default();
        text_style.font_size = 13.0;

        Self {
            ui,
            label: label.to_string(),
            shortcut: None,
            id,
            icon: None,

            style,
            text_style,
            hover_state: None,
            active_state: None,
        }
    }

    pub fn shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn show(self) -> Response {
        let margin_left = self.style.margin.left;
        let margin_top = self.style.margin.top;

        let (x, y) = (self.ui.cursor_x + margin_left, self.ui.cursor_y + margin_top);
        let default_w = self
            .ui
            .interaction_state
            .get(&CURRENT_MENU_ITEM_WIDTH_KEY)
            .copied()
            .unwrap_or(262.0);
        let mut w = self.style.width.unwrap_or(default_w);
        let mut h = self.style.height.unwrap_or(26.0);

        if let Some(min_w) = self.style.min_width { w = w.max(min_w); }
        if let Some(max_w) = self.style.max_width { w = w.min(max_w); }
        if let Some(min_h) = self.style.min_height { h = h.max(min_h); }
        if let Some(max_h) = self.style.max_height { h = h.min(max_h); }

        let (actual_ox, actual_oy, actual_w, actual_h) =
            if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
                (
                    rect.origin.x + self.ui.offset_x,
                    rect.origin.y + self.ui.offset_y,
                    rect.size.width.max(w),
                    rect.size.height.max(h),
                )
            } else {
                (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
            };

        let is_hovered = self
            .ui
            .is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);

        if is_hovered {
            self.ui.interaction_state.insert(HOVER_FLAG_KEY, 1.0);

            let eff_cursor = self
                .hover_state
                .as_ref()
                .and_then(|h| h.cursor)
                .unwrap_or(crate::text::CursorIcon::Pointer);
            self.ui.cursor_icon = eff_cursor;

            if let Some(ref tip) = self.hover_state.as_ref().and_then(|h| h.tooltip.as_ref()) {
                self.ui.draw_tooltip(tip);
            }

            // If a sibling regular menu item is hovered, close any open submenus at or deeper than this depth
            let depth = self
                .ui
                .interaction_state
                .get(&MENU_DEPTH_KEY)
                .copied()
                .unwrap_or(0.0) as usize;
            if depth > 0 {
                for d in depth..=20 {
                    let key = get_submenu_key(d);
                    if self.ui.interaction_state.get(&key).copied().unwrap_or(0.0) != 0.0 {
                        self.ui.interaction_state.insert(key, 0.0);
                        self.ui.needs_redraw = true;
                    }
                }
            }
        }

        let is_light_theme = self
            .ui
            .interaction_state
            .get(&Id::from_u64(999999999))
            .copied()
            .unwrap_or(0.0)
            > 0.5;

        let theme_accent = get_theme_color(
            self.ui,
            999999980,
            Color::rgb(255.0 / 255.0, 214.0 / 255.0, 0.0 / 255.0),
        );
        let theme_highlight = get_theme_color(
            self.ui,
            999999970,
            Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.04),
        );
        let theme_text_primary = get_theme_color(
            self.ui,
            999999960,
            Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0),
        );
        let theme_text_muted = get_theme_color(
            self.ui,
            999999950,
            Color::rgb(136.0 / 255.0, 136.0 / 255.0, 136.0 / 255.0),
        );

        let is_pressed = is_hovered && self.ui.mouse_down;
        let active_overrides = if is_pressed { self.active_state.as_ref() } else { None };
        let hover_overrides = if is_hovered { self.hover_state.as_ref() } else { None };

        let fallback_bg = if self.style.bg.is_none() {
            if is_hovered {
                Some(Background::Solid(if is_light_theme {
                    Color::rgb(0.9, 0.9, 0.95)
                } else {
                    theme_highlight
                }))
            } else {
                None
            }
        } else {
            None
        };

        let effective_bg = active_overrides
            .and_then(|o| o.style.bg.as_ref())
            .or_else(|| hover_overrides.and_then(|o| o.style.bg.as_ref()))
            .or_else(|| self.style.bg.as_ref())
            .or(fallback_bg.as_ref());

        let effective_border_color = active_overrides
            .and_then(|o| o.style.border_color())
            .or_else(|| hover_overrides.and_then(|o| o.style.border_color()))
            .or_else(|| self.style.border_color());

        let effective_border_width = active_overrides
            .map(|o| o.style.border_width())
            .filter(|&bw| bw > 0.0)
            .or_else(|| hover_overrides.map(|o| o.style.border_width()).filter(|&bw| bw > 0.0))
            .unwrap_or_else(|| self.style.border_width());

        let eff_scale = if is_pressed {
            active_overrides.map(|o| o.scale).filter(|&s| (s - 1.0).abs() > f32::EPSILON).unwrap_or(1.0)
        } else if is_hovered {
            hover_overrides.map(|o| o.scale).filter(|&s| (s - 1.0).abs() > f32::EPSILON).unwrap_or(1.0)
        } else {
            1.0
        };

        let effective_text_color = active_overrides
            .and_then(|o| o.text_color)
            .or_else(|| hover_overrides.and_then(|o| o.text_color))
            .unwrap_or_else(|| {
                if self.text_style.color != Color::WHITE {
                    self.text_style.color
                } else if is_hovered {
                    if is_light_theme { Color::rgb(0.1, 0.1, 0.15) } else { theme_accent }
                } else if is_light_theme {
                    Color::rgb(0.1, 0.1, 0.15)
                } else {
                    theme_text_primary
                }
            });

        let shortcut_color = if is_light_theme {
            Color::rgb(0.5, 0.5, 0.5)
        } else if is_hovered {
            theme_accent
        } else {
            theme_text_muted
        };

        let start_draw = self.ui.draws.len();

        if let Some(instance) = self.style.build_rect_styled(
            x,
            y,
            w,
            h,
            effective_bg,
            effective_border_color,
            Some(effective_border_width),
            eff_scale,
        ) {
            self.ui.draws.push(DrawCommand::Rect(RectDraw { instance }));
        }

        let text_start_x = if let Some(icon) = &self.icon {
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: icon.clone(),
                pos: [x + 8.0, y + (h - self.text_style.font_size) / 2.0],
                options: zenthra_text::prelude::TextOptions::new()
                    .size(self.text_style.font_size)
                    .color(effective_text_color),
                clip: [x, y, w, h],
            }));
            x + 28.0
        } else {
            x + 8.0
        };

        let (label_clip_w, shortcut_info) = if let Some(ref sh) = self.shortcut {
            let sc_w = (sh.len() as f32 * 7.2).max(40.0);
            let sc_x = x + w - sc_w - 8.0;
            let l_clip = (sc_x - text_start_x - 8.0).max(10.0);
            (l_clip, Some((sc_x, sh.clone())))
        } else {
            (w - (text_start_x - x) - 8.0, None)
        };

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [text_start_x, y + (h - self.text_style.font_size) / 2.0],
            options: zenthra_text::prelude::TextOptions::new()
                .size(self.text_style.font_size)
                .color(effective_text_color),
            clip: [text_start_x, y, label_clip_w, h],
        }));

        if let Some((sc_x, sh)) = shortcut_info {
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: sh,
                pos: [sc_x, y + (h - 12.0) / 2.0],
                options: zenthra_text::prelude::TextOptions::new()
                    .size(12.0)
                    .color(shortcut_color),
                clip: [sc_x, y, w - (sc_x - x), h],
            }));
        }

        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Button, Rect::new(x, y, w, h)),
        );
        self.ui.record_layout(self.id, Rect::new(x, y, w, h));
        self.ui.advance(
            w + self.style.margin.horizontal(),
            h + self.style.margin.vertical(),
            start_draw,
        );

        let clicked = self.ui.clicked && is_hovered;
        if clicked {
            // Close all menus upon item selection
            close_all_menus(self.ui);
            self.ui.consume_click();
            self.ui.needs_redraw = true;
        }

        Response {
            clicked,
            hovered: is_hovered,
            pressed: is_pressed,
            submitted: false,
            changed: false,
        }
    }
}

// =====================================================================
// UNIT TESTS
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::{Cursor, Style, Text, Transform};
    use std::collections::HashMap;

    #[test]
    fn test_menu_builders_box_style_and_sibling_parity() {
        let mut scroll = HashMap::new();
        let mut cursor = HashMap::new();
        let mut inter = HashMap::new();
        let l_cache = HashMap::new();
        let mut nl_cache = HashMap::new();
        let sl_cache = HashMap::new();
        let mut nsl_cache = HashMap::new();
        let ww_map = HashMap::new();
        let mut nww_map = HashMap::new();
        let img = HashMap::new();

        let mut ui = Ui::new(
            800, 600, 1.0, None, Vec::new(), None,
            (0.0, 0.0), false,
            &mut scroll, &mut cursor, &mut inter,
            None, None, false, false, 0.0,
            &l_cache, &mut nl_cache,
            &sl_cache, &mut nsl_cache,
            &ww_map, &mut nww_map,
            &img,
        );

        // 1. MenuBarBuilder BoxStyle parity
        let bar = ui.menu_bar()
            .height(32.0)
            .bg(Color::rgb(0.1, 0.1, 0.12))
            .padding_all(6.0);
        assert_eq!(bar.style.height, Some(32.0));
        assert_eq!(bar.style.padding.top, 6.0);
        assert_eq!(bar.style.bg, Some(Background::Solid(Color::rgb(0.1, 0.1, 0.12))));

        // 2. MenuBuilder BoxStyle + Sibling parity
        let menu = ui.menu("File")
            .height(28.0)
            .width(120.0)
            .menu_width(220.0)
            .bg(Color::rgb(0.15, 0.15, 0.2))
            .border((Color::rgb(0.3, 0.3, 0.4), 1.0))
            .radius_all(6.0)
            .size(14.0)
            .color(Color::WHITE)
            .hover([
                Style::bg(Color::rgb(0.2, 0.25, 0.35)),
                Cursor::Pointer,
            ])
            .active([
                Transform::scale(0.98),
            ])
            .popup_bg(Color::rgb(0.12, 0.12, 0.16))
            .popup_radius(8.0)
            .popup_border((Color::rgb(0.25, 0.25, 0.35), 1.0));

        assert_eq!(menu.style.height, Some(28.0));
        assert_eq!(menu.style.width, Some(120.0));
        assert_eq!(menu.popup_style.width, Some(220.0));
        assert_eq!(menu.style.radius, [6.0; 4]);
        assert_eq!(menu.text_style.font_size, 14.0);
        assert_eq!(menu.text_style.color, Color::WHITE);
        assert!(menu.hover_state.is_some());
        assert!(menu.active_state.is_some());
        assert_eq!(menu.popup_style.radius, [8.0; 4]);

        // 3. MenuItemBuilder BoxStyle + Sibling parity
        let item = ui.menu_item("Save")
            .shortcut("Ctrl+S")
            .height(24.0)
            .width(200.0)
            .radius_all(4.0)
            .size(13.0)
            .color(Color::rgb(0.9, 0.9, 0.9))
            .hover([
                Style::bg(Color::rgb(0.25, 0.35, 0.55)),
                Text::color(Color::WHITE),
                Cursor::Pointer,
            ]);

        assert_eq!(item.style.height, Some(24.0));
        assert_eq!(item.style.width, Some(200.0));
        assert_eq!(item.style.radius, [4.0; 4]);
        assert_eq!(item.text_style.font_size, 13.0);
        assert!(item.hover_state.is_some());
    }
}
