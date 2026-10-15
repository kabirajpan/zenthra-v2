// crates/zenthra-widgets/src/controls/menu.rs

use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{Color, Id, Rect, Response, Role, SemanticNode};
use zenthra_render::RectInstance;

pub const ACTIVE_MENU_KEY: Id = Id::from_u64(999999900);
pub const ACTIVE_SUBMENU_KEY: Id = Id::from_u64(999999901);
pub const HOVER_FLAG_KEY: Id = Id::from_u64(999999902);
pub const MENU_DEPTH_KEY: Id = Id::from_u64(999999903);
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

pub struct MenuBarBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    bg: Option<Color>,
    height: f32,
    padding_x: f32,
}

impl<'u, 'a> MenuBarBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        Self {
            ui,
            bg: None,
            height: 30.0,
            padding_x: 8.0,
        }
    }

    pub fn bg(mut self, bg: impl Into<Option<Color>>) -> Self {
        self.bg = bg.into();
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    pub fn padding_x(mut self, p: f32) -> Self {
        self.padding_x = p;
        self
    }

    pub fn show<F>(self, f: F)
    where F: FnOnce(&mut Ui) {
        // Reset menu depth for this frame
        self.ui.interaction_state.insert(MENU_DEPTH_KEY, 0.0);

        // Render the horizontal menu bar container
        let menu_bar_bg = self.bg.unwrap_or_else(|| {
            let is_light_theme = self.ui.interaction_state.get(&zenthra_core::Id::from_u64(999999999)).copied().unwrap_or(0.0) > 0.5;
            if is_light_theme {
                Color::rgb(0.90, 0.90, 0.92)
            } else {
                Color::rgb(2.0 / 255.0, 2.0 / 255.0, 2.0 / 255.0) // 2 – menu bar
            }
        });

        self.ui.container()
            .fill_x()
            .height(self.height)
            .bg(menu_bar_bg)
            .row()
            .padding_left(self.padding_x)
            .padding_right(self.padding_x)
            .show(|ui| {
                f(ui);
            });
    }
}

pub struct MenuBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    label: String,
    id: Id,
    placement: zenthra_core::Placement,
    align: zenthra_core::PopoverAlign,
    gap: f32,
    menu_width: f32,
    item_height: f32,
    is_submenu: Option<bool>,

    // Style properties
    text_color: Option<Color>,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    bg_color: Option<Color>,
    popup_bg: Option<Color>,
    popup_border: Option<(Color, f32)>,
    font_size: f32,
    radius: f32,
    padding_x: f32,
    icon: Option<String>,
}

impl<'u, 'a> MenuBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, label: &str) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        label.hash(&mut hasher);
        if let Some(parent) = ui.semantic_stack.last() {
            parent.hash(&mut hasher);
        }
        let id = Id::from_u64((hasher.finish() & 0x7FFFFF) + 1);
        Self {
            ui,
            label: label.to_string(),
            id,
            placement: zenthra_core::Placement::Bottom,
            align: zenthra_core::PopoverAlign::Start,
            gap: 2.0,
            menu_width: 270.0,
            item_height: 26.0,
            is_submenu: None,
            text_color: None,
            hover_state: None,
            bg_color: None,
            popup_bg: None,
            popup_border: None,
            font_size: 13.0,
            radius: 4.0,
            padding_x: 10.0,
            icon: None,
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

    pub fn width(mut self, width: f32) -> Self {
        self.menu_width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.item_height = height;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
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

    pub fn popup_bg(mut self, bg: impl Into<Option<Color>>) -> Self {
        self.popup_bg = bg.into();
        self
    }

    pub fn popup_border(mut self, border: impl Into<zenthra_core::Border>) -> Self {
        match border.into() {
            zenthra_core::Border::None => {
                self.popup_border = Some((Color::TRANSPARENT, 0.0));
            }
            zenthra_core::Border::Solid(c, w) => {
                self.popup_border = Some((c, w));
            }
        }
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn radius_all(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }

    pub fn padding_x(mut self, p: f32) -> Self {
        self.padding_x = p;
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

    pub fn show<F>(self, f: F)
    where F: FnOnce(&mut Ui) {
        let depth = self.ui.interaction_state.get(&MENU_DEPTH_KEY).copied().unwrap_or(0.0) as usize;
        let is_sub = self.is_submenu.unwrap_or(depth > 0);

        if is_sub {
            self.show_submenu(depth, f);
        } else {
            self.show_toplevel(f);
        }
    }

    fn show_toplevel<F>(self, f: F)
    where F: FnOnce(&mut Ui) {
        let active_menu_id = self.ui.interaction_state.get(&ACTIVE_MENU_KEY).copied().map(|v| v as u64).unwrap_or(0);
        let is_currently_active = active_menu_id == self.id.raw();

        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        let icon_extra_w = if self.icon.is_some() { 20.0 } else { 0.0 };
        let mut text_w = self.label.len() as f32 * 7.0;
        if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let options = zenthra_text::prelude::TextOptions::new().size(self.font_size);
            adapter.set_layout_size(1000.0, 100.0);
            let buffer = adapter.shape(&self.label, &options);
            text_w = buffer.size().0;
        }
        let w = text_w + icon_extra_w + self.padding_x * 2.0;
        let h = self.item_height;

        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                rect.size.width.max(w),
                rect.size.height.max(h)
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
        };

        let is_hovered = self.ui.is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);

        if is_hovered {
            self.ui.interaction_state.insert(HOVER_FLAG_KEY, 1.0);
        }

        // Auto-expand on hover if a top-level menu is already active
        if is_hovered && active_menu_id != 0 && !is_currently_active {
            self.ui.interaction_state.insert(ACTIVE_MENU_KEY, self.id.raw() as f32);
            self.ui.interaction_state.remove(&ACTIVE_SUBMENU_KEY);
            for d in 0..20 {
                self.ui.interaction_state.remove(&Id::from_u64(DEEP_SUBMENU_BASE_KEY + d));
            }
            self.ui.needs_redraw = true;
        }

        // Toggle on click
        if self.ui.clicked && is_hovered {
            if is_currently_active {
                self.ui.interaction_state.insert(ACTIVE_MENU_KEY, 0.0);
            } else {
                self.ui.interaction_state.insert(ACTIVE_MENU_KEY, self.id.raw() as f32);
            }
            self.ui.interaction_state.remove(&ACTIVE_SUBMENU_KEY);
            for d in 0..20 {
                self.ui.interaction_state.remove(&Id::from_u64(DEEP_SUBMENU_BASE_KEY + d));
            }
            self.ui.consume_click();
            self.ui.needs_redraw = true;
        }

        let is_light_theme = self.ui.interaction_state.get(&Id::from_u64(999999999)).copied().unwrap_or(0.0) > 0.5;
        let is_glassmorphism = self.ui.interaction_state.get(&Id::from_u64(999999998)).copied().unwrap_or(0.0) > 0.5;

        let theme_accent = get_theme_color(self.ui, 999999980, Color::rgb(255.0 / 255.0, 214.0 / 255.0, 0.0 / 255.0));
        let theme_highlight = get_theme_color(self.ui, 999999970, Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.04));
        let theme_text_primary = get_theme_color(self.ui, 999999960, Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0));
        let theme_bg_panel = get_theme_color(self.ui, 999999940, Color::rgb(1.0 / 255.0, 1.0 / 255.0, 1.0 / 255.0));
        let theme_border = get_theme_color(self.ui, 999999930, Color::rgb(3.0 / 255.0, 3.0 / 255.0, 3.0 / 255.0));

        let effective_hover_bg = self.hover_state.as_ref()
            .and_then(|s| match s.style.bg {
                Some(zenthra_core::Background::Solid(c)) => Some(c),
                _ => None,
            });
        let effective_hover_text = self.hover_state.as_ref().and_then(|s| s.text_color);

        let bg_color = if let Some(custom) = self.bg_color {
            if is_currently_active || is_hovered {
                effective_hover_bg.unwrap_or(custom)
            } else {
                custom
            }
        } else if is_light_theme {
            if is_currently_active {
                Color::rgb(0.80, 0.80, 0.85)
            } else if is_hovered {
                effective_hover_bg.unwrap_or(Color::rgb(0.88, 0.88, 0.90))
            } else {
                Color::TRANSPARENT
            }
        } else {
            if is_currently_active || is_hovered {
                effective_hover_bg.unwrap_or(theme_highlight)
            } else {
                Color::TRANSPARENT
            }
        };

        let text_color = if let Some(custom) = self.text_color {
            if is_currently_active || is_hovered {
                effective_hover_text.unwrap_or(custom)
            } else {
                custom
            }
        } else if is_light_theme {
            Color::rgb(0.1, 0.1, 0.15)
        } else {
            if is_currently_active || is_hovered {
                effective_hover_text.unwrap_or(theme_accent)
            } else {
                theme_text_primary
            }
        };

        let start_draw = self.ui.draws.len();
        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [x, y + 2.0],
                size: [w, h],
                color: bg_color.to_array(),
                radius: [self.radius; 4],
                ..Default::default()
            }
        }));

        let text_start_x = if let Some(icon) = &self.icon {
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: icon.clone(),
                pos: [x + self.padding_x, y + (h - self.font_size) / 2.0],
                options: zenthra_text::prelude::TextOptions::new().size(self.font_size).color(text_color),
                clip: [x, y, w, h + 4.0],
            }));
            x + self.padding_x + 18.0
        } else {
            x + (w - text_w) / 2.0
        };

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [text_start_x, y + (h - self.font_size) / 2.0],
            options: zenthra_text::prelude::TextOptions::new().size(self.font_size).color(text_color),
            clip: [x, y, w, h + 4.0],
        }));

        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Menu, Rect::new(x, y, w, h + 4.0))
        );
        self.ui.record_layout(self.id, Rect::new(x, y, w, h + 4.0));
        self.ui.advance(w, h + 4.0, start_draw);

        if is_currently_active {
            let popup_id = Id::from_u64((self.id.raw() << 8) | 10);

            // Check if popup is hovered
            if let Some(rect) = self.ui.screen_layout_cache.get(&popup_id) {
                if self.ui.mouse_in_rect(rect.origin.x, rect.origin.y, rect.size.width, rect.size.height) {
                    self.ui.interaction_state.insert(HOVER_FLAG_KEY, 1.0);
                }
            }

            // Isolate layout state variables
            let prev_child_ranges = std::mem::take(&mut self.ui.child_draw_ranges);
            let prev_id_ranges = std::mem::take(&mut self.ui.id_ranges);
            let prev_id_log = std::mem::take(&mut self.ui.id_log);
            let prev_child_sizes = std::mem::take(&mut self.ui.child_sizes);
            let prev_child_origins = std::mem::take(&mut self.ui.child_origins);

            let popup_bg = self.popup_bg.unwrap_or_else(|| {
                if is_light_theme {
                    Color::WHITE
                } else {
                    theme_bg_panel
                }
            });

            let (popup_border, popup_border_w) = self.popup_border.unwrap_or_else(|| {
                if is_light_theme {
                    (Color::rgb(0.8, 0.8, 0.85), 1.0)
                } else {
                    (theme_border, 1.0)
                }
            });

            let anchor_rect = Rect::new(actual_ox, actual_oy, actual_w, actual_h);
            let estimated_h = self.ui.screen_layout_cache.get(&popup_id).map(|r| r.size.height).unwrap_or(200.0);
            let (popup_x, popup_y) = crate::controls::popover::compute_placement_pos(
                anchor_rect,
                self.menu_width,
                estimated_h,
                self.placement,
                self.align,
                self.gap,
                true,
                self.ui.width,
                self.ui.height,
            );

            self.ui.overlay(|ui| {
                let mut container = ui.container()
                    .raw_id(popup_id)
                    .pos(popup_x, popup_y)
                    .overlay()
                    .clip(false)
                    .width(self.menu_width)
                    .radius_all(self.radius)
                    .padding(4.0, 4.0, 4.0, 4.0)
                    .column();

                if is_glassmorphism {
                    container = container
                        .bg(popup_bg.with_alpha(0.65))
                        .border((popup_border.with_alpha(0.08), popup_border_w))
                        .backdrop_filter(zenthra_core::BackdropFilter::new().blur(15.0, zenthra_core::style::blur::Type::Glassmorphism));
                } else {
                    container = container
                        .bg(popup_bg)
                        .border((popup_border, popup_border_w));
                }

                container.show(|ui| {
                    ui.interaction_state.insert(MENU_DEPTH_KEY, 1.0);
                    f(ui);
                    ui.interaction_state.insert(MENU_DEPTH_KEY, 0.0);
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
                let popup_hovered = self.ui.screen_layout_cache.get(&popup_id)
                    .map(|r| self.ui.mouse_in_rect(r.origin.x, r.origin.y, r.size.width, r.size.height))
                    .unwrap_or(false);
                let hover_flag = self.ui.interaction_state.get(&HOVER_FLAG_KEY).copied().unwrap_or(0.0) > 0.5;

                if !popup_hovered && !hover_flag {
                    close_all_menus(self.ui);
                    self.ui.needs_redraw = true;
                }
            }
        }
    }

    fn show_submenu<F>(self, depth: usize, f: F)
    where F: FnOnce(&mut Ui) {
        let sub_depth = depth.max(1);
        let active_key = get_submenu_key(sub_depth);

        let active_submenu_id = self.ui.interaction_state.get(&active_key).copied().map(|v| v as u64).unwrap_or(0);
        let is_currently_active = active_submenu_id == self.id.raw();

        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        let w = self.menu_width.min(262.0);
        let h = self.item_height;

        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                rect.size.width.max(w),
                rect.size.height.max(h)
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
        };

        let is_hovered = self.ui.is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);

        if is_hovered {
            self.ui.interaction_state.insert(HOVER_FLAG_KEY, 1.0);
        }

        // Switch on hover if a sibling submenu at this depth is already open
        if is_hovered && active_submenu_id != 0 && !is_currently_active {
            self.ui.interaction_state.insert(active_key, self.id.raw() as f32);
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
                self.ui.interaction_state.insert(active_key, self.id.raw() as f32);
            }
            for d in (sub_depth + 1)..=20 {
                self.ui.interaction_state.remove(&get_submenu_key(d));
            }
            self.ui.consume_click();
            self.ui.needs_redraw = true;
        }

        let is_light_theme = self.ui.interaction_state.get(&Id::from_u64(999999999)).copied().unwrap_or(0.0) > 0.5;
        let is_glassmorphism = self.ui.interaction_state.get(&Id::from_u64(999999998)).copied().unwrap_or(0.0) > 0.5;

        let theme_accent = get_theme_color(self.ui, 999999980, Color::rgb(255.0 / 255.0, 214.0 / 255.0, 0.0 / 255.0));
        let theme_highlight = get_theme_color(self.ui, 999999970, Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.04));
        let theme_text_primary = get_theme_color(self.ui, 999999960, Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0));
        let theme_text_muted = get_theme_color(self.ui, 999999950, Color::rgb(136.0 / 255.0, 136.0 / 255.0, 136.0 / 255.0));
        let theme_bg_panel = get_theme_color(self.ui, 999999940, Color::rgb(1.0 / 255.0, 1.0 / 255.0, 1.0 / 255.0));
        let theme_border = get_theme_color(self.ui, 999999930, Color::rgb(3.0 / 255.0, 3.0 / 255.0, 3.0 / 255.0));
        let effective_hover_bg = self.hover_state.as_ref()
            .and_then(|s| match s.style.bg {
                Some(zenthra_core::Background::Solid(c)) => Some(c),
                _ => None,
            });
        let effective_hover_text = self.hover_state.as_ref().and_then(|s| s.text_color);

        let bg_color = if let Some(custom) = self.bg_color {
            if is_currently_active || is_hovered {
                effective_hover_bg.unwrap_or(custom)
            } else {
                custom
            }
        } else if is_light_theme {
            if is_currently_active || is_hovered {
                Color::rgb(0.90, 0.90, 0.95)
            } else {
                Color::TRANSPARENT
            }
        } else {
            if is_currently_active || is_hovered {
                effective_hover_bg.unwrap_or(theme_highlight)
            } else {
                Color::TRANSPARENT
            }
        };

        let text_color = if let Some(custom) = self.text_color {
            if is_currently_active || is_hovered {
                effective_hover_text.unwrap_or(custom)
            } else {
                custom
            }
        } else if is_light_theme {
            Color::rgb(0.1, 0.1, 0.15)
        } else {
            if is_currently_active || is_hovered {
                effective_hover_text.unwrap_or(theme_accent)
            } else {
                theme_text_primary
            }
        };

        let chevron_color = if is_light_theme {
            Color::rgb(0.5, 0.5, 0.5)
        } else {
            if is_currently_active || is_hovered {
                theme_accent
            } else {
                theme_text_muted
            }
        };

        let start_draw = self.ui.draws.len();

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [x, y],
                size: [w, h],
                color: bg_color.to_array(),
                radius: [self.radius; 4],
                ..Default::default()
            }
        }));

        let text_start_x = if let Some(icon) = &self.icon {
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: icon.clone(),
                pos: [x + 8.0, y + (h - self.font_size) / 2.0],
                options: zenthra_text::prelude::TextOptions::new().size(self.font_size).color(text_color),
                clip: [x, y, w, h],
            }));
            x + 28.0
        } else {
            x + 8.0
        };

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [text_start_x, y + (h - self.font_size) / 2.0],
            options: zenthra_text::prelude::TextOptions::new().size(self.font_size).color(text_color),
            clip: [x, y, w - 24.0, h],
        }));

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: crate::icons::NF_FA_CHEVRON_RIGHT.to_string(),
            pos: [x + w - 20.0, y + (h - 10.0) / 2.0],
            options: zenthra_text::prelude::TextOptions::new().size(10.0).color(chevron_color),
            clip: [x, y, w, h],
        }));

        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Button, Rect::new(x, y, w, h))
        );
        self.ui.record_layout(self.id, Rect::new(x, y, w, h));
        self.ui.advance(w, h, start_draw);

        if is_currently_active {
            let sub_popup_id = Id::from_u64((self.id.raw() << 8) | (11 + sub_depth as u64));

            // Check if sub-popup is hovered
            if let Some(rect) = self.ui.screen_layout_cache.get(&sub_popup_id) {
                if self.ui.mouse_in_rect(rect.origin.x, rect.origin.y, rect.size.width, rect.size.height) {
                    self.ui.interaction_state.insert(HOVER_FLAG_KEY, 1.0);
                }
            }

            // Isolate layout state variables
            let prev_child_ranges = std::mem::take(&mut self.ui.child_draw_ranges);
            let prev_id_ranges = std::mem::take(&mut self.ui.id_ranges);
            let prev_id_log = std::mem::take(&mut self.ui.id_log);
            let prev_child_sizes = std::mem::take(&mut self.ui.child_sizes);
            let prev_child_origins = std::mem::take(&mut self.ui.child_origins);

            let popup_bg = self.popup_bg.unwrap_or_else(|| {
                if is_light_theme {
                    Color::WHITE
                } else {
                    theme_bg_panel
                }
            });

            let (popup_border, popup_border_w) = self.popup_border.unwrap_or_else(|| {
                if is_light_theme {
                    (Color::rgb(0.8, 0.8, 0.85), 1.0)
                } else {
                    (theme_border, 1.0)
                }
            });

            let sub_w = w + 12.0;
            let sub_x = if actual_ox + actual_w + sub_w > self.ui.width as f32 {
                actual_ox - sub_w + 2.0
            } else {
                actual_ox + actual_w - 2.0
            };

            let estimated_h = self.ui.screen_layout_cache.get(&sub_popup_id).map(|r| r.size.height).unwrap_or(150.0);
            let max_y = (self.ui.height as f32 - estimated_h - 8.0).max(4.0);
            let sub_y = (actual_oy - 4.0).min(max_y).max(4.0);

            self.ui.overlay(|ui| {
                let mut container = ui.container()
                    .raw_id(sub_popup_id)
                    .pos(sub_x, sub_y)
                    .overlay()
                    .clip(false)
                    .width(sub_w)
                    .radius_all(self.radius)
                    .padding(4.0, 4.0, 4.0, 4.0)
                    .column();

                if is_glassmorphism {
                    container = container
                        .bg(popup_bg.with_alpha(0.65))
                        .border((popup_border.with_alpha(0.08), popup_border_w))
                        .backdrop_filter(zenthra_core::BackdropFilter::new().blur(15.0, zenthra_core::style::blur::Type::Glassmorphism));
                } else {
                    container = container
                        .bg(popup_bg)
                        .border((popup_border, popup_border_w));
                }

                container.show(|ui| {
                    ui.interaction_state.insert(MENU_DEPTH_KEY, (sub_depth + 1) as f32);
                    f(ui);
                    ui.interaction_state.insert(MENU_DEPTH_KEY, sub_depth as f32);
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
        }
    }
}


pub struct MenuItemBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    label: String,
    shortcut: Option<String>,
    id: Id,
    height: f32,
    text_color: Option<Color>,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    bg_color: Option<Color>,
    font_size: f32,
    radius: f32,
    icon: Option<String>,
}

impl<'u, 'a> MenuItemBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, label: &str) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        label.hash(&mut hasher);
        if let Some(parent) = ui.semantic_stack.last() {
            parent.hash(&mut hasher);
        }
        let id = Id::from_u64((hasher.finish() & 0x7FFFFF) + 1);
        Self {
            ui,
            label: label.to_string(),
            shortcut: None,
            id,
            height: 26.0,
            text_color: None,
            hover_state: None,
            bg_color: None,
            font_size: 13.0,
            radius: 3.0,
            icon: None,
        }
    }

    pub fn shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
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

    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn radius_all(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn show(self) -> Response {
        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        let w = 262.0;
        let h = self.height;

        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                rect.size.width.max(w),
                rect.size.height.max(h)
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
        };

        let is_hovered = self.ui.is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);

        if is_hovered {
            self.ui.interaction_state.insert(HOVER_FLAG_KEY, 1.0);

            // If a sibling regular menu item is hovered, close any open submenus at or deeper than this depth
            let depth = self.ui.interaction_state.get(&MENU_DEPTH_KEY).copied().unwrap_or(0.0) as usize;
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

        let is_light_theme = self.ui.interaction_state.get(&Id::from_u64(999999999)).copied().unwrap_or(0.0) > 0.5;

        let theme_accent = get_theme_color(self.ui, 999999980, Color::rgb(255.0 / 255.0, 214.0 / 255.0, 0.0 / 255.0));
        let theme_highlight = get_theme_color(self.ui, 999999970, Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.04));
        let theme_text_primary = get_theme_color(self.ui, 999999960, Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0));
        let theme_text_muted = get_theme_color(self.ui, 999999950, Color::rgb(136.0 / 255.0, 136.0 / 255.0, 136.0 / 255.0));

        let effective_hover_bg = self.hover_state.as_ref()
            .and_then(|s| match s.style.bg {
                Some(zenthra_core::Background::Solid(c)) => Some(c),
                _ => None,
            });
        let effective_hover_text = self.hover_state.as_ref().and_then(|s| s.text_color);

        let bg_color = if let Some(custom) = self.bg_color {
            if is_hovered {
                effective_hover_bg.unwrap_or(custom)
            } else {
                custom
            }
        } else if is_light_theme {
            if is_hovered {
                Color::rgb(0.9, 0.9, 0.95)
            } else {
                Color::TRANSPARENT
            }
        } else {
            if is_hovered {
                effective_hover_bg.unwrap_or(theme_highlight)
            } else {
                Color::TRANSPARENT
            }
        };

        let text_color = if let Some(custom) = self.text_color {
            if is_hovered {
                effective_hover_text.unwrap_or(custom)
            } else {
                custom
            }
        } else if is_light_theme {
            Color::rgb(0.1, 0.1, 0.15)
        } else {
            if is_hovered {
                effective_hover_text.unwrap_or(theme_accent)
            } else {
                theme_text_primary
            }
        };

        let shortcut_color = if is_light_theme {
            Color::rgb(0.5, 0.5, 0.5)
        } else {
            if is_hovered {
                theme_accent
            } else {
                theme_text_muted
            }
        };

        let start_draw = self.ui.draws.len();

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [x, y],
                size: [w, h],
                color: bg_color.to_array(),
                radius: [self.radius; 4],
                ..Default::default()
            }
        }));

        let text_start_x = if let Some(icon) = &self.icon {
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: icon.clone(),
                pos: [x + 8.0, y + (h - self.font_size) / 2.0],
                options: zenthra_text::prelude::TextOptions::new().size(self.font_size).color(text_color),
                clip: [x, y, w, h],
            }));
            x + 28.0
        } else {
            x + 8.0
        };

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [text_start_x, y + (h - self.font_size) / 2.0],
            options: zenthra_text::prelude::TextOptions::new().size(self.font_size).color(text_color),
            clip: [x, y, w - 80.0, h],
        }));

        if let Some(sh) = self.shortcut {
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: sh,
                pos: [x + w - 80.0, y + (h - 12.0) / 2.0],
                options: zenthra_text::prelude::TextOptions::new().size(12.0).color(shortcut_color),
                clip: [x, y, w, h],
            }));
        }

        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Button, Rect::new(x, y, w, h))
        );
        self.ui.record_layout(self.id, Rect::new(x, y, w, h));
        self.ui.advance(w, h, start_draw);

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
            pressed: is_hovered && self.ui.mouse_down,
            submitted: false,
            changed: false,
        }
    }
}
