// crates/zenthra-widgets/src/controls/dropdown.rs

use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{Color, Id, Rect, Response};
use zenthra_render::RectInstance;

pub struct DropdownBuilder<'u, 'a, 'b, T: PartialEq + Clone + ToString> {
    ui: &'u mut Ui<'a>,
    selected: &'b mut T,
    options: Vec<T>,
    id: Id,

    // Layout
    width: f32,
    height: f32,
    radius: [f32; 4],

    // Colors
    bg: Color,
    border_color: Color,
    border_width: f32,
    text_color: Color,
    hover_bg: Color,
    
    // Menu Style
    menu_bg: Color,
    menu_max_height: f32,

    // Styling
    glow: bool,
    shadow_color: Color,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,
    shadow_enabled: bool,
}

impl<'u, 'a, 'b, T: PartialEq + Clone + ToString> DropdownBuilder<'u, 'a, 'b, T> {
    pub fn new(ui: &'u mut Ui<'a>, selected: &'b mut T, options: Vec<T>) -> Self {
        let id = ui.id();
        Self {
            ui,
            selected,
            options,
            id,

            width: 180.0,
            height: 32.0,
            radius: [4.0; 4],

            bg: Color::rgb(0.12, 0.12, 0.14), // Solid default
            border_color: Color::rgb(0.25, 0.25, 0.3),
            border_width: 1.0,
            text_color: Color::WHITE,
            hover_bg: Color::rgb(0.18, 0.18, 0.22),

            menu_bg: Color::rgb(0.1, 0.1, 0.12), // Solid default
            menu_max_height: 250.0,

            glow: false,
            shadow_enabled: false, // OFF BY DEFAULT
            shadow_color: Color::rgb(0.0, 0.0, 0.0),
            shadow_offset: [0.0, 4.0],
            shadow_blur: 20.0,
            shadow_opacity: 1.0,
        }
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = color;
        self.border_width = width;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = w;
        self.height = h;
        self
    }

    pub fn colors(mut self, bg: Color, border: Color) -> Self {
        self.bg = bg;
        self.border_color = border;
        self
    }

    pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.radius = [tl, tr, br, bl];
        self
    }

    pub fn radius_all(mut self, r: f32) -> Self {
        self.radius = [r; 4];
        self
    }

    pub fn radius_top(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self.radius[1] = r;
        self
    }

    pub fn radius_bottom(mut self, r: f32) -> Self {
        self.radius[2] = r;
        self.radius[3] = r;
        self
    }

    pub fn radius_top_left(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self
    }

    pub fn radius_top_right(mut self, r: f32) -> Self {
        self.radius[1] = r;
        self
    }

    pub fn radius_bottom_right(mut self, r: f32) -> Self {
        self.radius[2] = r;
        self
    }

    pub fn radius_bottom_left(mut self, r: f32) -> Self {
        self.radius[3] = r;
        self
    }

    pub fn radius_left(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self.radius[3] = r;
        self
    }

    pub fn radius_right(mut self, r: f32) -> Self {
        self.radius[1] = r;
        self.radius[2] = r;
        self
    }

    pub fn glow(mut self, enabled: bool) -> Self {
        self.glow = enabled;
        self
    }

    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.shadow_color = color;
        self.shadow_offset = [x, y];
        self.shadow_blur = blur;
        self.shadow_enabled = true;
        self
    }

    pub fn shadow_opacity(mut self, opacity: f32) -> Self {
        self.shadow_opacity = opacity;
        self
    }

    pub fn show(self) -> Response {
        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        let open_id = Id::from_u64(self.id.raw().wrapping_add(4000000));
        let mut is_open = self.ui.interaction_state.get(&open_id).map(|&v| v > 0.5).unwrap_or(false);

        // 1. Hit-testing Header
        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                rect.size.width.max(self.width),
                rect.size.height.max(self.height)
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, self.width, self.height)
        };

        let is_hovered = self.ui.mouse_in_rect(actual_ox, actual_oy, actual_w, actual_h);
        
        if self.ui.clicked && is_hovered {
            is_open = !is_open;
            self.ui.interaction_state.insert(open_id, if is_open { 1.0 } else { 0.0 });
            self.ui.needs_redraw = true;
        }

        let start_draw = self.ui.draws.len();

        // Glow color for border
        let border_color = if self.glow && is_hovered {
            Color::rgb(0.4, 0.7, 1.0)
        } else {
            self.border_color
        };

        // 2. Draw Header
        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [x, y],
                size: [self.width, self.height],
                color: (if is_hovered { self.hover_bg } else { self.bg }).to_array(),
                radius: self.radius,
                border_width: self.border_width,
                border_color: border_color.to_array(),
                // Subtle glow if enabled
                shadow_color: if self.glow && is_hovered { [0.4, 0.7, 1.0, 0.3] } else { [0.0, 0.0, 0.0, 0.0] },
                shadow_blur: if self.glow && is_hovered { 15.0 } else { 0.0 },
                ..Default::default()
            }
        }));

        // Selected Text
        let selected_text = self.selected.to_string();
        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: selected_text,
            pos: [x + 10.0, y + (self.height - 14.0) / 2.0],
            options: zenthra_text::prelude::TextOptions::new().font_size(14.0).color(self.text_color),
            clip: [x, y, self.width, self.height],
        }));

        // Chevron
        let chevron = if is_open { "▴" } else { "▾" };
        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: chevron.to_string(),
            pos: [x + self.width - 20.0, y + (self.height - 14.0) / 2.0],
            options: zenthra_text::prelude::TextOptions::new().font_size(12.0).color(self.text_color),
            clip: [x, y, self.width, self.height],
        }));

        // 3. Draw Menu (Overlay)
        if is_open {
            let menu_ox = actual_ox;
            let menu_oy = actual_oy + self.height + 4.0;
            
            let item_h = 28.0;
            let menu_h = (self.options.len() as f32 * item_h).min(self.menu_max_height);
            
            // Background catcher (to close menu)
            self.ui.overlays.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [0.0, 0.0],
                    size: [self.ui.width, self.ui.height],
                    color: [0.0, 0.0, 0.0, 0.0], // Invisible
                    ..Default::default()
                }
            }));
            
            // If clicked outside the menu but on the catcher
            if self.ui.clicked && !is_hovered {
                let menu_hovered = self.ui.mouse_in_rect(menu_ox, menu_oy, self.width, menu_h);
                if !menu_hovered {
                    is_open = false;
                    self.ui.interaction_state.insert(open_id, 0.0);
                    self.ui.needs_redraw = true;
                }
            }

            // Menu Background (Opaque)
            self.ui.overlays.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [menu_ox, menu_oy],
                    size: [self.width, menu_h],
                    color: self.menu_bg.to_array(),
                    radius: self.radius,
                    border_width: 1.0,
                    border_color: self.border_color.to_array(),
                    shadow_color: if self.shadow_enabled {
                        let mut a = self.shadow_color.to_array();
                        a[3] *= self.shadow_opacity;
                        a
                    } else { [0.0, 0.0, 0.0, 0.0] },
                    shadow_offset: self.shadow_offset,
                    shadow_blur: if self.shadow_enabled { self.shadow_blur } else { 0.0 },
                    ..Default::default()
                }
            }));

            // Items
            for (i, opt) in self.options.iter().enumerate() {
                let item_oy = menu_oy + (i as f32 * item_h);
                if item_oy + item_h > menu_oy + menu_h { break; } 
                
                let item_hovered = self.ui.mouse_in_rect(menu_ox, item_oy, self.width, item_h);
                
                if item_hovered && self.ui.clicked {
                    *self.selected = opt.clone();
                    is_open = false;
                    self.ui.interaction_state.insert(open_id, 0.0);
                    self.ui.needs_redraw = true;
                }

                // Item background
                if item_hovered {
                    self.ui.overlays.push(DrawCommand::Rect(RectDraw {
                        instance: RectInstance {
                            pos: [menu_ox + 2.0, item_oy + 2.0],
                            size: [self.width - 4.0, item_h - 4.0],
                            color: self.hover_bg.to_array(),
                            radius: [
                                self.radius[0] - 2.0,
                                self.radius[1] - 2.0,
                                self.radius[2] - 2.0,
                                self.radius[3] - 2.0,
                            ],
                            ..Default::default()
                        }
                    }));
                }

                // Item text
                self.ui.overlays.push(DrawCommand::Text(TextDraw {
                    text: opt.to_string(),
                    pos: [menu_ox + 10.0, item_oy + (item_h - 14.0) / 2.0],
                    options: zenthra_text::prelude::TextOptions::new().font_size(13.0).color(self.text_color),
                    clip: [menu_ox, menu_oy, self.width, menu_h],
                }));
            }
        }

        self.ui.record_layout(self.id, Rect::new(x, y, self.width, self.height));
        self.ui.advance(self.width, self.height, start_draw);

        Response {
            clicked: false, // Dropdown handles its own clicks
            hovered: is_hovered,
            pressed: is_hovered && self.ui.mouse_down,
        }
    }
}
