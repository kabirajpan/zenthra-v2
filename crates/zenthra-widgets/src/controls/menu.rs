// crates/zenthra-widgets/src/controls/menu.rs

use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{Color, Id, Rect, Response, Align};
use zenthra_render::RectInstance;

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
}

impl<'u, 'a> MenuBarBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        Self { ui, bg: None }
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn show<F>(self, f: F)
    where F: FnOnce(&mut Ui) {
        let active_menu_key = Id::from_u64(999999900);
        let active_submenu_key = Id::from_u64(999999901);
        let hover_flag_key = Id::from_u64(999999902);

        // Reset hover flag for this frame
        self.ui.interaction_state.insert(hover_flag_key, 0.0);

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
            .full_width()
            .height(30.0)
            .bg(menu_bar_bg)
            .row()
            .padding_left(8.0)
            .padding_right(8.0)
            .show(|ui| {
                f(ui);
            });

        // Click outside (light dismiss) handling
        let clicked = self.ui.clicked;
        let active_menu_id = self.ui.interaction_state.get(&active_menu_key).copied().map(|v| v as u64).unwrap_or(0);
        let hover_flag = self.ui.interaction_state.get(&hover_flag_key).copied().unwrap_or(0.0) > 0.5;

        if active_menu_id != 0 && clicked && !hover_flag {
            self.ui.interaction_state.insert(active_menu_key, 0.0);
            self.ui.interaction_state.insert(active_submenu_key, 0.0);
            self.ui.needs_redraw = true;
        }
    }
}

pub struct MenuBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    label: String,
    id: Id,
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
        }
    }

    pub fn show<F>(self, f: F)
    where F: FnOnce(&mut Ui) {
        let active_menu_key = Id::from_u64(999999900);
        let active_submenu_key = Id::from_u64(999999901);
        let hover_flag_key = Id::from_u64(999999902);

        let active_menu_id = self.ui.interaction_state.get(&active_menu_key).copied().map(|v| v as u64).unwrap_or(0);
        let is_currently_active = active_menu_id == self.id.raw();

        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        let mut text_w = self.label.len() as f32 * 7.0;
        if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let options = zenthra_text::prelude::TextOptions::new().font_size(13.0);
            adapter.set_layout_size(1000.0, 100.0);
            let buffer = adapter.shape(&self.label, &options);
            text_w = buffer.size().0;
        }
        let padding_x = 10.0;
        let w = text_w + padding_x * 2.0;
        let h = 26.0;

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
            self.ui.interaction_state.insert(hover_flag_key, 1.0);
        }

        // Auto-expand on hover if menu bar is active
        if is_hovered && active_menu_id != 0 && !is_currently_active {
            self.ui.interaction_state.insert(active_menu_key, self.id.raw() as f32);
            self.ui.interaction_state.remove(&active_submenu_key);
            self.ui.needs_redraw = true;
        }

        // Toggle on click
        if self.ui.clicked && is_hovered {
            if is_currently_active {
                self.ui.interaction_state.insert(active_menu_key, 0.0);
            } else {
                self.ui.interaction_state.insert(active_menu_key, self.id.raw() as f32);
            }
            self.ui.interaction_state.remove(&active_submenu_key);
            self.ui.needs_redraw = true;
        }

        let is_light_theme = self.ui.interaction_state.get(&Id::from_u64(999999999)).copied().unwrap_or(0.0) > 0.5;
        let is_glassmorphism = self.ui.interaction_state.get(&Id::from_u64(999999998)).copied().unwrap_or(0.0) > 0.5;

        let theme_accent = get_theme_color(self.ui, 999999980, Color::rgb(255.0 / 255.0, 214.0 / 255.0, 0.0 / 255.0));
        let theme_highlight = get_theme_color(self.ui, 999999970, Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.08));
        let theme_text_primary = get_theme_color(self.ui, 999999960, Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0));
        let theme_bg_panel = get_theme_color(self.ui, 999999940, Color::rgb(1.0 / 255.0, 1.0 / 255.0, 1.0 / 255.0));
        let theme_border = get_theme_color(self.ui, 999999930, Color::rgb(3.0 / 255.0, 3.0 / 255.0, 3.0 / 255.0));

        let bg_color = if is_light_theme {
            if is_currently_active {
                Color::rgb(0.80, 0.80, 0.85)
            } else if is_hovered {
                Color::rgb(0.88, 0.88, 0.90)
            } else {
                Color::TRANSPARENT
            }
        } else {
            if is_currently_active || is_hovered {
                theme_highlight
            } else {
                Color::TRANSPARENT
            }
        };

        let text_color = if is_light_theme {
            Color::rgb(0.1, 0.1, 0.15)
        } else {
            if is_currently_active || is_hovered {
                theme_accent
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
                radius: [3.0; 4],
                ..Default::default()
            }
        }));

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [x + (w - text_w) / 2.0, y + 6.0],
            options: zenthra_text::prelude::TextOptions::new().font_size(13.0).color(text_color),
            clip: [x, y, w, h + 4.0],
        }));

        self.ui.record_layout(self.id, Rect::new(x, y, w, h + 4.0));
        self.ui.advance(w, h + 4.0, start_draw);

        if is_currently_active {
            let popup_id = Id::from_u64((self.id.raw() << 8) | 10);

            // Check if popup is hovered
            if let Some(rect) = self.ui.screen_layout_cache.get(&popup_id) {
                if self.ui.mouse_in_rect(rect.origin.x, rect.origin.y, rect.size.width, rect.size.height) {
                    self.ui.interaction_state.insert(hover_flag_key, 1.0);
                }
            }

            // Isolate layout state variables
            let prev_child_ranges = std::mem::take(&mut self.ui.child_draw_ranges);
            let prev_id_ranges = std::mem::take(&mut self.ui.id_ranges);
            let prev_id_log = std::mem::take(&mut self.ui.id_log);
            let prev_child_sizes = std::mem::take(&mut self.ui.child_sizes);
            let prev_child_origins = std::mem::take(&mut self.ui.child_origins);

            let popup_bg = if is_light_theme {
                Color::WHITE
            } else {
                theme_bg_panel
            };

            let popup_border = if is_light_theme {
                Color::rgb(0.8, 0.8, 0.85)
            } else {
                theme_border
            };

            let popup_x = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
                rect.origin.x
            } else {
                x
            };
            let popup_y = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
                rect.origin.y
            } else {
                y
            };

            self.ui.overlay(|ui| {
                let mut container = ui.container()
                    .id(popup_id)
                    .absolute(popup_x, popup_y + h + 2.0)
                    .overlay()
                    .width(270.0)
                    .radius_all(4.0)
                    .padding(4.0, 4.0, 4.0, 4.0)
                    .column();

                if is_glassmorphism {
                    container = container
                        .bg(popup_bg.with_alpha(0.65))
                        .border(popup_border.with_alpha(0.08), 1.0)
                        .backdrop_filter(zenthra_core::BackdropFilter::new().blur(15.0, zenthra_core::style::blur::Type::Glassmorphism));
                } else {
                    container = container
                        .bg(popup_bg)
                        .border(popup_border, 1.0);
                }

                container.show(|ui| {
                    f(ui);
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

pub struct SubMenuBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    label: String,
    id: Id,
    width: f32,
}

impl<'u, 'a> SubMenuBuilder<'u, 'a> {
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
            width: 262.0,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn show<F>(self, f: F)
    where F: FnOnce(&mut Ui) {
        let active_submenu_key = Id::from_u64(999999901);
        let hover_flag_key = Id::from_u64(999999902);

        let active_submenu_id = self.ui.interaction_state.get(&active_submenu_key).copied().map(|v| v as u64).unwrap_or(0);
        let is_currently_active = active_submenu_id == self.id.raw();

        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        let w = self.width;
        let h = 26.0;

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
            self.ui.interaction_state.insert(hover_flag_key, 1.0);
        }

        // Toggle on click
        if self.ui.clicked && is_hovered {
            if is_currently_active {
                self.ui.interaction_state.insert(active_submenu_key, 0.0);
            } else {
                self.ui.interaction_state.insert(active_submenu_key, self.id.raw() as f32);
            }
            self.ui.needs_redraw = true;
        }

        let is_light_theme = self.ui.interaction_state.get(&Id::from_u64(999999999)).copied().unwrap_or(0.0) > 0.5;
        let is_glassmorphism = self.ui.interaction_state.get(&Id::from_u64(999999998)).copied().unwrap_or(0.0) > 0.5;

        let theme_accent = get_theme_color(self.ui, 999999980, Color::rgb(255.0 / 255.0, 214.0 / 255.0, 0.0 / 255.0));
        let theme_highlight = get_theme_color(self.ui, 999999970, Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.08));
        let theme_text_primary = get_theme_color(self.ui, 999999960, Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0));
        let theme_text_muted = get_theme_color(self.ui, 999999950, Color::rgb(136.0 / 255.0, 136.0 / 255.0, 136.0 / 255.0));
        let theme_bg_panel = get_theme_color(self.ui, 999999940, Color::rgb(1.0 / 255.0, 1.0 / 255.0, 1.0 / 255.0));
        let theme_border = get_theme_color(self.ui, 999999930, Color::rgb(3.0 / 255.0, 3.0 / 255.0, 3.0 / 255.0));

        let bg_color = if is_light_theme {
            if is_currently_active || is_hovered {
                Color::rgb(0.90, 0.90, 0.95)
            } else {
                Color::TRANSPARENT
            }
        } else {
            if is_currently_active || is_hovered {
                theme_highlight
            } else {
                Color::TRANSPARENT
            }
        };

        let text_color = if is_light_theme {
            Color::rgb(0.1, 0.1, 0.15)
        } else {
            if is_currently_active || is_hovered {
                theme_accent
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

        self.ui.container()
            .id(self.id)
            .width(w)
            .height(26.0)
            .row()
            .valign(Align::Center)
            .padding(2.0, 14.0, 2.0, 8.0)
            .radius_all(4.0)
            .bg(bg_color)
            .show(|ui| {
                ui.text(&self.label)
                    .size(11.0)
                    .color(text_color)
                    .show();

                ui.container().fill_x().halign(Align::Right).show(|ui| {
                    ui.text(crate::icons::NF_FA_CHEVRON_RIGHT)
                        .size(10.0)
                        .color(chevron_color)
                        .show();
                });
            });

        if is_currently_active {
            let sub_popup_id = Id::from_u64((self.id.raw() << 8) | 11);

            // Check if sub-popup is hovered
            if let Some(rect) = self.ui.screen_layout_cache.get(&sub_popup_id) {
                if self.ui.mouse_in_rect(rect.origin.x, rect.origin.y, rect.size.width, rect.size.height) {
                    self.ui.interaction_state.insert(hover_flag_key, 1.0);
                }
            }

            // Isolate layout state variables
            let prev_child_ranges = std::mem::take(&mut self.ui.child_draw_ranges);
            let prev_id_ranges = std::mem::take(&mut self.ui.id_ranges);
            let prev_id_log = std::mem::take(&mut self.ui.id_log);
            let prev_child_sizes = std::mem::take(&mut self.ui.child_sizes);
            let prev_child_origins = std::mem::take(&mut self.ui.child_origins);

            let popup_bg = if is_light_theme {
                Color::WHITE
            } else {
                theme_bg_panel
            };

            let popup_border = if is_light_theme {
                Color::rgb(0.8, 0.8, 0.85)
            } else {
                theme_border
            };

            self.ui.overlay(|ui| {
                let mut container = ui.container()
                    .id(sub_popup_id)
                    .absolute(x + w - 2.0, y)
                    .overlay()
                    .width(w + 12.0)
                    .radius_all(4.0)
                    .padding(4.0, 4.0, 4.0, 4.0)
                    .column();

                if is_glassmorphism {
                    container = container
                        .bg(popup_bg.with_alpha(0.65))
                        .border(popup_border.with_alpha(0.08), 1.0)
                        .backdrop_filter(zenthra_core::BackdropFilter::new().blur(15.0, zenthra_core::style::blur::Type::Glassmorphism));
                } else {
                    container = container
                        .bg(popup_bg)
                        .border(popup_border, 1.0);
                }

                container.show(|ui| {
                    f(ui);
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
        }
    }

    pub fn shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }

    pub fn show(self) -> Response {
        let hover_flag_key = Id::from_u64(999999902);

        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        let w = 262.0;
        let h = 26.0;

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
            self.ui.interaction_state.insert(hover_flag_key, 1.0);
        }

        let is_light_theme = self.ui.interaction_state.get(&Id::from_u64(999999999)).copied().unwrap_or(0.0) > 0.5;

        let theme_accent = get_theme_color(self.ui, 999999980, Color::rgb(255.0 / 255.0, 214.0 / 255.0, 0.0 / 255.0));
        let theme_highlight = get_theme_color(self.ui, 999999970, Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.08));
        let theme_text_primary = get_theme_color(self.ui, 999999960, Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0));
        let theme_text_muted = get_theme_color(self.ui, 999999950, Color::rgb(136.0 / 255.0, 136.0 / 255.0, 136.0 / 255.0));

        let bg_color = if is_light_theme {
            if is_hovered {
                Color::rgb(0.9, 0.9, 0.95)
            } else {
                Color::TRANSPARENT
            }
        } else {
            if is_hovered {
                theme_highlight
            } else {
                Color::TRANSPARENT
            }
        };

        let text_color = if is_light_theme {
            Color::rgb(0.1, 0.1, 0.15)
        } else {
            if is_hovered {
                theme_accent
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
                radius: [3.0; 4],
                ..Default::default()
            }
        }));

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.label.clone(),
            pos: [x + 8.0, y + 6.0],
            options: zenthra_text::prelude::TextOptions::new().font_size(13.0).color(text_color),
            clip: [x, y, w, h],
        }));

        if let Some(sh) = self.shortcut {
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: sh,
                pos: [x + w - 80.0, y + 6.0],
                options: zenthra_text::prelude::TextOptions::new().font_size(12.0).color(shortcut_color),
                clip: [x, y, w, h],
            }));
        }

        self.ui.record_layout(self.id, Rect::new(x, y, w, h));
        self.ui.advance(w, h, start_draw);

        let clicked = self.ui.clicked && is_hovered;
        if clicked {
            // Close all menus upon item selection
            let active_menu_key = Id::from_u64(999999900);
            let active_submenu_key = Id::from_u64(999999901);
            self.ui.interaction_state.insert(active_menu_key, 0.0);
            self.ui.interaction_state.insert(active_submenu_key, 0.0);
            self.ui.needs_redraw = true;
        }

        Response {
            clicked,
            hovered: is_hovered,
            pressed: is_hovered && self.ui.mouse_down,
        }
    }
}
