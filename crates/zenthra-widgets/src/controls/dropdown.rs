// crates/zenthra-widgets/src/controls/dropdown.rs

use crate::style_ext::BoxStyleExt;
use crate::ui::{DrawCommand, RectDraw, TextDraw, Ui};
use zenthra_core::{Background, Border, BoxStyle, Color, EdgeInsets, Id, Rect, Response, Styleable};
use zenthra_render::RectInstance;

#[inline]
pub fn dropdown_open_key(id: Id) -> Id {
    Id::from_u64(((id.raw() & 0x00FF_FFFF_FFFF_FFFF) << 8) | 0x20)
}

pub struct DropdownBuilder<'u, 'a, 'b, T: PartialEq + Clone + ToString> {
    pub ui: &'u mut Ui<'a>,
    pub selected: &'b mut T,
    pub options: Vec<T>,
    pub id: Id,

    // Header styling & Typography
    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,

    // Menu Style (Dropdown popup)
    pub menu_style: BoxStyle,
    pub menu_max_height: f32,

    // Styling & Effects
    pub glow: bool,
    pub placement: zenthra_core::Placement,
    pub gap: f32,
}

impl<'u, 'a, 'b, T: PartialEq + Clone + ToString> Styleable for DropdownBuilder<'u, 'a, 'b, T> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

impl<'u, 'a, 'b, T: PartialEq + Clone + ToString> DropdownBuilder<'u, 'a, 'b, T> {
    pub fn new(ui: &'u mut Ui<'a>, selected: &'b mut T, options: Vec<T>) -> Self {
        let id = ui.id();

        let mut style = BoxStyle::new();
        style.width = Some(180.0);
        style.height = Some(32.0);
        style.radius = [4.0; 4];
        style.padding.left = 10.0;
        style.padding.right = 10.0;
        style.bg = Some(Background::Solid(Color::rgb(0.12, 0.12, 0.14)));
        style.border = Border::Solid(Color::rgb(0.25, 0.25, 0.3), 1.0);
        style.shadow_offset = [0.0, 4.0];
        style.shadow_blur = 20.0;

        let mut text_style = zenthra_text::prelude::TextStyle::default();
        text_style.font_size = 14.0;
        text_style.color = Color::WHITE;

        let mut menu_style = BoxStyle::new();
        menu_style.bg = Some(Background::Solid(Color::rgb(0.1, 0.1, 0.12)));
        menu_style.radius = [4.0; 4];
        menu_style.border = Border::Solid(Color::rgb(0.25, 0.25, 0.3), 1.0);
        menu_style.shadow_color = Some(Color::rgba(0.0, 0.0, 0.0, 0.35));
        menu_style.shadow_offset = [0.0, 4.0];
        menu_style.shadow_blur = 16.0;

        Self {
            ui,
            selected,
            options,
            id,

            style,
            text_style,
            hover_state: None,
            active_state: None,

            menu_style,
            menu_max_height: 250.0,

            glow: false,
            placement: zenthra_core::Placement::Bottom,
            gap: 4.0,
        }
    }

    pub fn placement(mut self, placement: zenthra_core::Placement) -> Self {
        self.placement = placement;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    // --- Box Dimensions (impl_dimensions parity) ---
    pub fn width(mut self, width: f32) -> Self {
        self.style.width = Some(width);
        self.style.fill_x = false;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.style.height = Some(height);
        self.style.fill_y = false;
        self
    }

    pub fn fill_x(mut self) -> Self {
        self.style.fill_x = true;
        self
    }

    pub fn fill_y(mut self) -> Self {
        self.style.fill_y = true;
        self
    }

    pub fn fill(mut self) -> Self {
        self.style.fill_x = true;
        self.style.fill_y = true;
        self
    }

    pub fn min_width(mut self, w: f32) -> Self {
        self.style.min_width = Some(w);
        self
    }

    pub fn max_width(mut self, w: f32) -> Self {
        self.style.max_width = Some(w);
        self
    }

    pub fn min_height(mut self, h: f32) -> Self {
        self.style.min_height = Some(h);
        self
    }

    pub fn max_height(mut self, h: f32) -> Self {
        self.style.max_height = Some(h);
        self
    }

    // --- Padding (impl_padding parity) ---
    pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.style.padding = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }

    pub fn padding_all(mut self, p: f32) -> Self {
        self.style.padding = EdgeInsets::all(p);
        self
    }

    pub fn padding_x(mut self, px: f32) -> Self {
        self.style.padding.left = px;
        self.style.padding.right = px;
        self
    }

    pub fn padding_y(mut self, py: f32) -> Self {
        self.style.padding.top = py;
        self.style.padding.bottom = py;
        self
    }

    pub fn padding_top(mut self, p: f32) -> Self {
        self.style.padding.top = p;
        self
    }

    pub fn padding_bottom(mut self, p: f32) -> Self {
        self.style.padding.bottom = p;
        self
    }

    pub fn padding_left(mut self, p: f32) -> Self {
        self.style.padding.left = p;
        self
    }

    pub fn padding_right(mut self, p: f32) -> Self {
        self.style.padding.right = p;
        self
    }

    // --- Margin (impl_margin parity) ---
    pub fn margin(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.style.margin = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }

    pub fn margin_all(mut self, m: f32) -> Self {
        self.style.margin = EdgeInsets::all(m);
        self
    }

    pub fn margin_x(mut self, mx: f32) -> Self {
        self.style.margin.left = mx;
        self.style.margin.right = mx;
        self
    }

    pub fn margin_y(mut self, my: f32) -> Self {
        self.style.margin.top = my;
        self.style.margin.bottom = my;
        self
    }

    // --- Radius (impl_radius parity) ---
    pub fn radius_all(mut self, r: f32) -> Self {
        self.style.radius = [r; 4];
        self
    }

    pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.style.radius = [tl, tr, br, bl];
        self
    }

    pub fn radius_top(mut self, r: f32) -> Self {
        self.style.radius[0] = r;
        self.style.radius[1] = r;
        self
    }

    pub fn radius_bottom(mut self, r: f32) -> Self {
        self.style.radius[2] = r;
        self.style.radius[3] = r;
        self
    }

    pub fn radius_top_left(mut self, r: f32) -> Self {
        self.style.radius[0] = r;
        self
    }

    pub fn radius_top_right(mut self, r: f32) -> Self {
        self.style.radius[1] = r;
        self
    }

    pub fn radius_bottom_right(mut self, r: f32) -> Self {
        self.style.radius[2] = r;
        self
    }

    pub fn radius_bottom_left(mut self, r: f32) -> Self {
        self.style.radius[3] = r;
        self
    }

    pub fn radius_left(mut self, r: f32) -> Self {
        self.style.radius[0] = r;
        self.style.radius[3] = r;
        self
    }

    pub fn radius_right(mut self, r: f32) -> Self {
        self.style.radius[1] = r;
        self.style.radius[2] = r;
        self
    }

    // --- Visuals (impl_visuals parity) ---
    pub fn style(mut self, style: BoxStyle) -> Self {
        self.style = style;
        self
    }

    pub fn bg(mut self, bg: impl Into<Background>) -> Self {
        match bg.into() {
            Background::None => self.style.bg = None,
            other => self.style.bg = Some(other),
        }
        self
    }

    pub fn border(mut self, border: impl crate::macros::IntoBorder) -> Self {
        let (b, align) = border.into_border();
        self.style.border = b;
        if let Some(a) = align {
            self.style.border_alignment = a;
        }
        self
    }

    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.style.shadow_color = Some(color);
        self.style.shadow_offset = [x, y];
        self.style.shadow_blur = blur;
        self
    }

    pub fn glow(mut self, enabled: bool) -> Self {
        self.glow = enabled;
        self
    }

    // --- Typography (impl_font_style_methods parity) ---
    pub fn size(mut self, size: f32) -> Self {
        self.text_style.font_size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.text_style.color = color;
        self
    }

    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.text_style.font_family = Some(family.into());
        self
    }

    pub fn font_weight(mut self, weight: impl Into<zenthra_text::prelude::FontWeight>) -> Self {
        self.text_style.font_weight = weight.into();
        self
    }

    // --- Interaction States (impl_interaction_state_methods parity) ---
    #[inline]
    pub fn hover(mut self, props: impl crate::macros::IntoStateProps) -> Self {
        if self.hover_state.is_none() {
            let mut s = crate::macros::WidgetStateOverrides::new();
            s.style = self.style.clone();
            self.hover_state = Some(s);
        }
        if let Some(ref mut h) = self.hover_state {
            props.apply_to_overrides(h);
        }
        self
    }

    #[inline]
    pub fn active(mut self, props: impl crate::macros::IntoStateProps) -> Self {
        if self.active_state.is_none() {
            let mut s = crate::macros::WidgetStateOverrides::new();
            s.style = self.style.clone();
            self.active_state = Some(s);
        }
        if let Some(ref mut a) = self.active_state {
            props.apply_to_overrides(a);
        }
        self
    }

    // --- Menu Sub-Component Styling ---
    pub fn menu_style(mut self, style: BoxStyle) -> Self {
        self.menu_style = style;
        self
    }

    pub fn menu_bg(mut self, bg: impl Into<Background>) -> Self {
        match bg.into() {
            Background::None => self.menu_style.bg = None,
            other => self.menu_style.bg = Some(other),
        }
        self
    }

    pub fn menu_max_height(mut self, max_h: f32) -> Self {
        self.menu_max_height = max_h;
        self
    }

    pub fn menu_border(mut self, border: impl crate::macros::IntoBorder) -> Self {
        let (b, align) = border.into_border();
        self.menu_style.border = b;
        if let Some(a) = align {
            self.menu_style.border_alignment = a;
        }
        self
    }

    pub fn menu_radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.menu_style.radius = [tl, tr, br, bl];
        self
    }

    pub fn menu_radius_all(mut self, r: f32) -> Self {
        self.menu_style.radius = [r; 4];
        self
    }

    pub fn menu_shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.menu_style.shadow_color = Some(color);
        self.menu_style.shadow_offset = [x, y];
        self.menu_style.shadow_blur = blur;
        self
    }

    pub fn show(self) -> Response {
        let (x, y) = (self.ui.cursor_x, self.ui.cursor_y);
        let width = if self.style.fill_x {
            self.ui.available_width
        } else {
            self.style.width.unwrap_or(180.0)
        };
        let height = if self.style.fill_y {
            (self.ui.max_y - y).max(self.style.height.unwrap_or(32.0))
        } else {
            self.style.height.unwrap_or(32.0)
        };

        let open_id = dropdown_open_key(self.id);
        let mut is_open = self.ui.interaction_state.get(&open_id).map(|&v| v > 0.5).unwrap_or(false);

        // 1. Hit-testing Header
        let (actual_ox, actual_oy, actual_w, actual_h) = if let Some(screen_rect) = self.ui.screen_layout_cache.get(&self.id) {
            (
                screen_rect.origin.x,
                screen_rect.origin.y,
                screen_rect.size.width.max(width),
                screen_rect.size.height.max(height),
            )
        } else if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                rect.size.width.max(width),
                rect.size.height.max(height),
            )
        } else {
            (x + self.ui.offset_x, y + self.ui.offset_y, width, height)
        };

        let is_hovered = self.ui.is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);
        let is_active = is_hovered && self.ui.mouse_down;
        let mut changed = false;

        let just_opened = if self.ui.clicked && is_hovered {
            is_open = !is_open;
            self.ui.interaction_state.insert(open_id, if is_open { 1.0 } else { 0.0 });
            self.ui.needs_redraw = true;
            self.ui.consume_click();
            true
        } else {
            false
        };

        let start_draw = self.ui.draws.len();

        // 2. Draw Header via BoxStyleExt with state blending
        let active_overrides = if is_active { self.active_state.as_ref() } else { None };
        let hover_overrides = if is_hovered { self.hover_state.as_ref() } else { None };

        let effective_bg = active_overrides
            .and_then(|o| o.style.bg.as_ref())
            .or_else(|| hover_overrides.and_then(|o| o.style.bg.as_ref()))
            .or(self.style.bg.as_ref());

        let effective_border_color = if self.glow && is_hovered {
            Some(Color::rgb(0.4, 0.7, 1.0))
        } else {
            active_overrides
                .and_then(|o| o.style.border_color())
                .or_else(|| hover_overrides.and_then(|o| o.style.border_color()))
                .or_else(|| self.style.border_color())
        };

        let effective_border_width = active_overrides
            .map(|o| o.style.border_width())
            .or_else(|| hover_overrides.map(|o| o.style.border_width()))
            .unwrap_or_else(|| self.style.border_width());

        if let Some(instance) = self.style.build_rect_styled(
            x,
            y,
            width,
            height,
            effective_bg,
            effective_border_color,
            Some(effective_border_width),
            1.0,
        ) {
            self.ui.draws.push(DrawCommand::Rect(RectDraw { instance }));
        }

        // Selected Text
        let selected_text = self.selected.to_string();
        let padding_left = self.style.padding.left.max(4.0);
        let text_clip_w = (width - padding_left - 18.0).max(0.0);
        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: selected_text,
            pos: [x + padding_left, y + (height - self.text_style.font_size) / 2.0],
            options: zenthra_text::prelude::TextOptions::new()
                .size(self.text_style.font_size)
                .color(self.text_style.color),
            clip: [x, y, text_clip_w, height],
        }));

        // Chevron
        let chevron = if is_open {
            crate::icons::NF_FA_CHEVRON_UP
        } else {
            crate::icons::NF_FA_CHEVRON_DOWN
        };
        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: chevron.to_string(),
            pos: [x + width - 20.0, y + (height - 12.0) / 2.0],
            options: zenthra_text::prelude::TextOptions::new().size(12.0).color(self.text_style.color),
            clip: [x, y, width, height],
        }));

        // 3. Draw Menu (Overlay)
        if is_open {
            let item_h = (self.text_style.font_size + 14.0).max(28.0);
            let menu_h = (self.options.len() as f32 * item_h).min(self.menu_max_height);
            let anchor_rect = Rect::new(actual_ox, actual_oy, actual_w, actual_h);
            let (menu_ox, menu_oy) = crate::controls::popover::compute_placement_pos(
                anchor_rect,
                width,
                menu_h,
                self.placement,
                zenthra_core::PopoverAlign::Start,
                self.gap,
                true,
                self.ui.width,
                self.ui.height,
            );

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
            if !just_opened && self.ui.clicked && !is_hovered {
                let menu_hovered = self.ui.mouse_in_rect(menu_ox, menu_oy, width, menu_h);
                if !menu_hovered {
                    self.ui.interaction_state.insert(open_id, 0.0);
                    self.ui.needs_redraw = true;
                }
            }

            // Menu Background
            let (menu_c1, menu_c2, _c3, _c4, menu_params, _stops, _) =
                crate::style_ext::resolve_background(&self.menu_style.bg);
            let menu_bc = self.menu_style.border_color().unwrap_or(Color::TRANSPARENT);
            let menu_bw = self.menu_style.border_width();
            let menu_shadow_color = self.menu_style.gpu_shadow_color();

            self.ui.overlays.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [menu_ox, menu_oy],
                    size: [width, menu_h],
                    color: menu_c1,
                    color2: menu_c2,
                    gradient_params: menu_params,
                    radius: self.menu_style.gpu_radius(),
                    border_width: menu_bw,
                    border_color: menu_bc.to_array(),
                    shadow_color: menu_shadow_color,
                    shadow_offset: self.menu_style.shadow_offset,
                    shadow_blur: self.menu_style.shadow_blur,
                    ..Default::default()
                }
            }));

            // Items
            let hover_color = self.hover_state.as_ref()
                .and_then(|s| match s.style.bg {
                    Some(zenthra_core::Background::Solid(c)) => Some(c),
                    _ => None,
                })
                .unwrap_or(Color::rgb(0.18, 0.18, 0.22));

            let item_radius = [
                (self.menu_style.radius[3] - 2.0).max(0.0),
                (self.menu_style.radius[2] - 2.0).max(0.0),
                (self.menu_style.radius[1] - 2.0).max(0.0),
                (self.menu_style.radius[0] - 2.0).max(0.0),
            ];

            for (i, opt) in self.options.iter().enumerate() {
                let item_oy = menu_oy + (i as f32 * item_h);
                if item_oy + item_h > menu_oy + menu_h { break; }

                let item_hovered = self.ui.mouse_in_rect(menu_ox, item_oy, width, item_h);

                if !just_opened && item_hovered && self.ui.clicked {
                    *self.selected = opt.clone();
                    changed = true;
                    self.ui.interaction_state.insert(open_id, 0.0);
                    self.ui.needs_redraw = true;
                    self.ui.consume_click();
                }

                // Item background
                if item_hovered {
                    self.ui.overlays.push(DrawCommand::Rect(RectDraw {
                        instance: RectInstance {
                            pos: [menu_ox + 2.0, item_oy + 2.0],
                            size: [width - 4.0, item_h - 4.0],
                            color: hover_color.to_array(),
                            radius: item_radius,
                            ..Default::default()
                        }
                    }));
                }

                // Item text
                self.ui.overlays.push(DrawCommand::Text(TextDraw {
                    text: opt.to_string(),
                    pos: [menu_ox + padding_left, item_oy + (item_h - (self.text_style.font_size - 1.0)) / 2.0],
                    options: zenthra_text::prelude::TextOptions::new()
                        .size((self.text_style.font_size - 1.0).max(11.0))
                        .color(self.text_style.color),
                    clip: [menu_ox, menu_oy, width, menu_h],
                }));
            }
        }

        self.ui.record_layout(self.id, Rect::new(x, y, width, height));
        self.ui.advance(width, height, start_draw);

        Response {
            clicked: false, // Dropdown handles its own clicks
            hovered: is_hovered,
            pressed: is_hovered && self.ui.mouse_down,
            submitted: false,
            changed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
        Ui::new(
            800, 600, 1.0, None, Vec::new(), None,
            (0.0, 0.0), false,
            scroll_state, cursor_state, interaction_state,
            None, None, false, false, 0.0,
            layout_cache, next_layout_cache,
            screen_layout_cache, next_screen_layout_cache,
            widget_window_map, next_widget_window_map,
            image_sizes,
        )
    }

    #[test]
    fn test_dropdown_builder_defaults() {
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

        let mut selected = "Option 1".to_string();
        let options = vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()];

        let builder = DropdownBuilder::new(&mut ui, &mut selected, options);
        assert_eq!(builder.style.width, Some(180.0));
        assert_eq!(builder.style.height, Some(32.0));
        assert_eq!(builder.style.radius, [4.0, 4.0, 4.0, 4.0]);
        assert_eq!(builder.style.padding.left, 10.0);
        assert_eq!(builder.text_style.font_size, 14.0);
        assert_eq!(builder.style.border_width(), 1.0);
    }

    #[test]
    fn test_dropdown_styling_api() {
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

        let mut selected = "Rust".to_string();
        let options = vec!["Rust".to_string(), "Go".to_string(), "C++".to_string()];

        let builder = DropdownBuilder::new(&mut ui, &mut selected, options)
            .bg(Background::None)
            .border(Border::None)
            .radius_all(8.0)
            .size(16.0)
            .padding_x(14.0)
            .color(Color::WHITE)
            .shadow(Color::BLACK, 0.0, 4.0, 12.0)
            .menu_bg(Color::rgb(0.08, 0.08, 0.10))
            .menu_max_height(300.0)
            .menu_border((Color::rgb(0.3, 0.3, 0.35), 1.5))
            .menu_radius_all(6.0);

        assert_eq!(builder.style.bg, None);
        assert_eq!(builder.style.border_width(), 0.0);
        assert_eq!(builder.style.border_color(), None);
        assert_eq!(builder.style.radius, [8.0, 8.0, 8.0, 8.0]);
        assert_eq!(builder.text_style.font_size, 16.0);
        assert_eq!(builder.style.padding.left, 14.0);
        assert_eq!(builder.text_style.color, Color::WHITE);
        assert_eq!(builder.style.shadow_color, Some(Color::BLACK));
        assert_eq!(builder.menu_style.bg, Some(Background::Solid(Color::rgb(0.08, 0.08, 0.10))));
        assert_eq!(builder.menu_max_height, 300.0);
        assert_eq!(builder.menu_style.border_width(), 1.5);
        assert_eq!(builder.menu_style.radius, [6.0; 4]);
    }

    #[test]
    fn test_dropdown_individual_radii() {
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

        let mut selected = "One".to_string();
        let options = vec!["One".to_string(), "Two".to_string()];

        let builder = DropdownBuilder::new(&mut ui, &mut selected, options)
            .radius_top(10.0)
            .radius_bottom(2.0);

        // radius format: [tl, tr, br, bl]
        assert_eq!(builder.style.radius[0], 10.0);
        assert_eq!(builder.style.radius[1], 10.0);
        assert_eq!(builder.style.radius[2], 2.0);
        assert_eq!(builder.style.radius[3], 2.0);
    }

    #[test]
    fn test_dropdown_show_draw_commands() {
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

        let mut selected = "Alpha".to_string();
        let options = vec!["Alpha".to_string(), "Beta".to_string()];

        let resp = ui.dropdown(&mut selected, options)
            .width(200.0)
            .height(36.0)
            .radius_all(6.0)
            .show();

        assert!(!resp.clicked);
        assert!(!resp.changed);
        // Should have pushed at least 3 draw commands: background Rect, selected Text, chevron Text
        assert!(ui.draws.len() >= 3);

        if let DrawCommand::Rect(rect) = &ui.draws[0] {
            assert_eq!(rect.instance.size, [200.0, 36.0]);
            // Verify inverted corner radius mapping: [bl, br, tr, tl]
            assert_eq!(rect.instance.radius, [6.0, 6.0, 6.0, 6.0]);
        } else {
            panic!("Expected first draw command to be Rect");
        }
    }

    #[test]
    fn test_dropdown_fill_x_and_fill_y() {
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

        let mut selected = "Filled".to_string();
        let options = vec!["Filled".to_string()];

        ui.dropdown(&mut selected, options)
            .fill_x()
            .fill_y()
            .show();

        if let DrawCommand::Rect(rect) = &ui.draws[0] {
            assert_eq!(rect.instance.size, [800.0, 600.0]);
        } else {
            panic!("Expected Rect for dropdown");
        }
    }

    #[test]
    fn test_dropdown_click_open_lifecycle() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let mut layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let mut screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let mut widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        let mut selected = "Alpha".to_string();
        let options = vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()];

        // Frame 1: Initial layout (mouse at 0, 0, no click)
        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            ui.dropdown(&mut selected, options.clone())
                .id("test_dd")
                .width(200.0)
                .height(40.0)
                .show();
        }
        layout_cache = std::mem::take(&mut next_layout_cache);
        screen_layout_cache = std::mem::take(&mut next_screen_layout_cache);
        widget_window_map = std::mem::take(&mut next_widget_window_map);

        let open_id = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            "test_dd".hash(&mut hasher);
            dropdown_open_key(Id::from_u64(hasher.finish())).raw()
        };
        assert_eq!(interaction_state.get(&Id::from_u64(open_id)).copied().unwrap_or(0.0), 0.0);

        // Frame 2: Mouse click on header at (50.0, 20.0)
        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            ui.mouse_x = 50.0;
            ui.mouse_y = 20.0;
            ui.clicked = true;

            ui.dropdown(&mut selected, options.clone())
                .id("test_dd")
                .width(200.0)
                .height(40.0)
                .show();
        }
        layout_cache = std::mem::take(&mut next_layout_cache);
        screen_layout_cache = std::mem::take(&mut next_screen_layout_cache);
        widget_window_map = std::mem::take(&mut next_widget_window_map);

        println!("After Frame 2 click: open_state = {:?}", interaction_state.get(&Id::from_u64(open_id)));
        assert_eq!(interaction_state.get(&Id::from_u64(open_id)).copied().unwrap_or(0.0), 1.0);

        // Frame 3: Mouse still at (50.0, 20.0), but clicked = false (user released mouse or next frame)
        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            ui.mouse_x = 50.0;
            ui.mouse_y = 20.0;
            ui.clicked = false;

            ui.dropdown(&mut selected, options.clone())
                .id("test_dd")
                .width(200.0)
                .height(40.0)
                .show();
        }
        println!("After Frame 3 idle: open_state = {:?}", interaction_state.get(&Id::from_u64(open_id)));
        assert_eq!(interaction_state.get(&Id::from_u64(open_id)).copied().unwrap_or(0.0), 1.0);
    }

    #[test]
    fn test_two_dropdowns_interaction() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let mut layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let mut screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let mut widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        let mut sel_a = "Theme A".to_string();
        let mut sel_b = "Log Info".to_string();
        let opts_a = vec!["Theme A".to_string(), "Theme B".to_string()];
        let opts_b = vec!["Log Info".to_string(), "Log Warn".to_string()];

        // Frame 1: Layout both dropdowns
        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            ui.dropdown(&mut sel_a, opts_a.clone())
                .id("dd_a")
                .width(200.0)
                .height(40.0)
                .show();
            ui.dropdown(&mut sel_b, opts_b.clone())
                .id("dd_b")
                .width(200.0)
                .height(40.0)
                .show();
        }
        layout_cache = std::mem::take(&mut next_layout_cache);
        screen_layout_cache = std::mem::take(&mut next_screen_layout_cache);
        widget_window_map = std::mem::take(&mut next_widget_window_map);

        let open_id_a = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            "dd_a".hash(&mut hasher);
            dropdown_open_key(Id::from_u64(hasher.finish())).raw()
        };
        let open_id_b = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            "dd_b".hash(&mut hasher);
            dropdown_open_key(Id::from_u64(hasher.finish())).raw()
        };

        // Frame 2: Click on Dropdown B (which is at cursor_y = 40.0, so y = 50.0 is on dd_b!)
        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            ui.mouse_x = 50.0;
            ui.mouse_y = 50.0; // Over dd_b!
            ui.clicked = true;

            ui.dropdown(&mut sel_a, opts_a.clone())
                .id("dd_a")
                .width(200.0)
                .height(40.0)
                .show();
            ui.dropdown(&mut sel_b, opts_b.clone())
                .id("dd_b")
                .width(200.0)
                .height(40.0)
                .show();
        }
        let _ = std::mem::take(&mut next_layout_cache);
        let _ = std::mem::take(&mut next_screen_layout_cache);
        let _ = std::mem::take(&mut next_widget_window_map);

        println!("After click on B: open_a = {:?}, open_b = {:?}",
            interaction_state.get(&Id::from_u64(open_id_a)),
            interaction_state.get(&Id::from_u64(open_id_b))
        );
        assert_eq!(interaction_state.get(&Id::from_u64(open_id_b)).copied().unwrap_or(0.0), 1.0);
    }

    #[test]
    fn test_dropdown_in_nested_containers() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let mut layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let mut screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let mut widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        let mut sel_theme = "Dark Velvet".to_string();
        let mut sel_env = "Prod".to_string();
        let mut sel_log = "Info".to_string();
        let levels = vec!["Trace".to_string(), "Info".to_string(), "Warn".to_string()];

        // Frame 1: Render Section 1 nested hierarchy
        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.container().fill_x().column().gap(18.0).show(|ui: &mut Ui| {
                ui.row().fill_x().gap(24.0).show(|ui: &mut Ui| {
                    ui.column().gap(6.0).show(|ui: &mut Ui| {
                        ui.text("Theme").show();
                        ui.dropdown(&mut sel_theme, vec!["Dark Velvet".to_string()]).id("theme_dd").width(220.0).height(36.0).show();
                    });
                    ui.column().gap(6.0).show(|ui: &mut Ui| {
                        ui.text("Env").show();
                        ui.dropdown(&mut sel_env, vec!["Prod".to_string()]).id("env_dd").width(240.0).height(36.0).show();
                    });
                    ui.column().gap(6.0).show(|ui: &mut Ui| {
                        ui.text("Log Severity").show();
                        ui.dropdown(&mut sel_log, levels.clone()).id("log_dd").width(160.0).height(36.0).show();
                    });
                });
            });
        }
        layout_cache = std::mem::take(&mut next_layout_cache);
        screen_layout_cache = std::mem::take(&mut next_screen_layout_cache);
        widget_window_map = std::mem::take(&mut next_widget_window_map);

        let open_id_log = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            "log_dd".hash(&mut hasher);
            dropdown_open_key(Id::from_u64(hasher.finish())).raw()
        };

        let log_id = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            "log_dd".hash(&mut hasher);
            Id::from_u64(hasher.finish())
        };

        let log_rect = screen_layout_cache.get(&log_id).cloned();
        println!("Log Dropdown Screen Rect: {:?}", log_rect);
        let rect = log_rect.expect("Log rect should be recorded");

        // Frame 2: Click on Log Dropdown at its center
        let click_x = rect.origin.x + rect.size.width / 2.0;
        let click_y = rect.origin.y + rect.size.height / 2.0;
        println!("Clicking Log Dropdown at ({}, {})", click_x, click_y);

        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            ui.mouse_x = click_x;
            ui.mouse_y = click_y;
            ui.clicked = true;

            ui.container().fill_x().column().gap(18.0).show(|ui: &mut Ui| {
                ui.row().fill_x().gap(24.0).show(|ui: &mut Ui| {
                    ui.column().gap(6.0).show(|ui: &mut Ui| {
                        ui.text("Theme").show();
                        ui.dropdown(&mut sel_theme, vec!["Dark Velvet".to_string()]).id("theme_dd").width(220.0).height(36.0).show();
                    });
                    ui.column().gap(6.0).show(|ui: &mut Ui| {
                        ui.text("Env").show();
                        ui.dropdown(&mut sel_env, vec!["Prod".to_string()]).id("env_dd").width(240.0).height(36.0).show();
                    });
                    ui.column().gap(6.0).show(|ui: &mut Ui| {
                        ui.text("Log Severity").show();
                        ui.dropdown(&mut sel_log, levels.clone()).id("log_dd").width(160.0).height(36.0).show();
                    });
                });
            });
        }
        layout_cache = std::mem::take(&mut next_layout_cache);
        screen_layout_cache = std::mem::take(&mut next_screen_layout_cache);
        widget_window_map = std::mem::take(&mut next_widget_window_map);

        println!("After Frame 2 click: log open_state = {:?}", interaction_state.get(&Id::from_u64(open_id_log)));
        assert_eq!(interaction_state.get(&Id::from_u64(open_id_log)).copied().unwrap_or(0.0), 1.0);

        // Frame 3: Next frame (mouse still at click pos, clicked = false)
        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            ui.mouse_x = click_x;
            ui.mouse_y = click_y;
            ui.clicked = false;

            ui.container().fill_x().column().gap(18.0).show(|ui: &mut Ui| {
                ui.row().fill_x().gap(24.0).show(|ui: &mut Ui| {
                    ui.column().gap(6.0).show(|ui: &mut Ui| {
                        ui.text("Theme").show();
                        ui.dropdown(&mut sel_theme, vec!["Dark Velvet".to_string()]).id("theme_dd").width(220.0).height(36.0).show();
                    });
                    ui.column().gap(6.0).show(|ui: &mut Ui| {
                        ui.text("Env").show();
                        ui.dropdown(&mut sel_env, vec!["Prod".to_string()]).id("env_dd").width(240.0).height(36.0).show();
                    });
                    ui.column().gap(6.0).show(|ui: &mut Ui| {
                        ui.text("Log Severity").show();
                        ui.dropdown(&mut sel_log, levels.clone()).id("log_dd").width(160.0).height(36.0).show();
                    });
                });
            });
        }
        println!("After Frame 3 idle: log open_state = {:?}", interaction_state.get(&Id::from_u64(open_id_log)));
        assert_eq!(interaction_state.get(&Id::from_u64(open_id_log)).copied().unwrap_or(0.0), 1.0);
    }

    #[test]
    fn test_showcase_dropdowns_interaction() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let mut layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let mut screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let mut widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        let mut selected_theme = "Dark Velvet".to_string();
        let mut selected_env = "Production".to_string();
        let mut selected_log_level = "Info".to_string();
        let mut selected_region = "us-east-1".to_string();

        let render = |ui: &mut Ui,
                      sel_theme: &mut String,
                      sel_env: &mut String,
                      sel_log: &mut String,
                      sel_reg: &mut String| {
            ui.container()
                .fill()
                .padding_all(24.0)
                .scroll_y(true)
                .gap(24.0)
                .show(|ui: &mut Ui| {
                    // Header card
                    ui.container().fill_x().padding_all(20.0).show(|ui: &mut Ui| {
                        ui.text("Header Card").show();
                    });

                    // Section 1
                    ui.container().fill_x().padding_all(18.0).column().gap(18.0).show(|ui: &mut Ui| {
                        ui.text("Section 1").show();
                        ui.row().fill_x().gap(24.0).show(|ui: &mut Ui| {
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.text("Color Theme").show();
                                ui.dropdown(sel_theme, vec!["Dark Velvet".to_string()]).id("theme_dropdown").width(220.0).height(36.0).show();
                            });
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.text("Environment").show();
                                ui.dropdown(sel_env, vec!["Production".to_string()]).id("env_dropdown").width(240.0).height(36.0).show();
                            });
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.text("Log Severity").show();
                                ui.dropdown(sel_log, vec!["Trace".to_string(), "Info".to_string(), "Warn".to_string()]).id("log_dropdown").width(160.0).height(36.0).show();
                            });
                        });
                    });

                    // Section 2
                    ui.container().fill_x().padding_all(18.0).column().gap(18.0).show(|ui: &mut Ui| {
                        ui.text("Section 2").show();
                        ui.column().gap(6.0).show(|ui: &mut Ui| {
                            ui.text("Primary Compute Region").show();
                            ui.dropdown(sel_reg, vec!["us-east-1".to_string(), "us-west-2".to_string()]).id("region_dropdown").fill_x().height(40.0).show();
                        });
                    });
                });
        };

        // Frame 1: Initial layout pass
        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            render(&mut ui, &mut selected_theme, &mut selected_env, &mut selected_log_level, &mut selected_region);
        }
        layout_cache = std::mem::take(&mut next_layout_cache);
        screen_layout_cache = std::mem::take(&mut next_screen_layout_cache);
        widget_window_map = std::mem::take(&mut next_widget_window_map);

        let open_id_log = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            "log_dropdown".hash(&mut hasher);
            dropdown_open_key(Id::from_u64(hasher.finish())).raw()
        };

        let log_id = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            "log_dropdown".hash(&mut hasher);
            Id::from_u64(hasher.finish())
        };

        let log_rect = screen_layout_cache.get(&log_id).cloned().expect("log_dropdown rect");
        println!("log_rect on screen: {:?}", log_rect);

        // Frame 2: Click on log_dropdown
        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            ui.mouse_x = log_rect.origin.x + log_rect.size.width / 2.0;
            ui.mouse_y = log_rect.origin.y + log_rect.size.height / 2.0;
            ui.clicked = true;
            render(&mut ui, &mut selected_theme, &mut selected_env, &mut selected_log_level, &mut selected_region);
        }
        layout_cache = std::mem::take(&mut next_layout_cache);
        screen_layout_cache = std::mem::take(&mut next_screen_layout_cache);
        widget_window_map = std::mem::take(&mut next_widget_window_map);

        let reg_id = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            "region_dropdown".hash(&mut hasher);
            Id::from_u64(hasher.finish())
        };

        let open_id_reg = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            "region_dropdown".hash(&mut hasher);
            dropdown_open_key(Id::from_u64(hasher.finish())).raw()
        };

        let reg_rect = screen_layout_cache.get(&reg_id).cloned().expect("region_dropdown rect");
        println!("reg_rect on screen: {:?}", reg_rect);

        // Frame 3: Click on region_dropdown
        {
            let mut ui = create_test_ui(
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            ui.mouse_x = reg_rect.origin.x + reg_rect.size.width / 2.0;
            ui.mouse_y = reg_rect.origin.y + reg_rect.size.height / 2.0;
            ui.clicked = true;
            render(&mut ui, &mut selected_theme, &mut selected_env, &mut selected_log_level, &mut selected_region);
        }
        let _ = std::mem::take(&mut next_layout_cache);
        let _ = std::mem::take(&mut next_screen_layout_cache);
        let _ = std::mem::take(&mut next_widget_window_map);

        println!("Showcase after Frame 3 click on region: reg open_state = {:?}, log open_state = {:?}",
            interaction_state.get(&Id::from_u64(open_id_reg)),
            interaction_state.get(&Id::from_u64(open_id_log))
        );
    }
}

