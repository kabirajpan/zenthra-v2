// crates/zenthra-widgets/src/controls/popover.rs

use crate::ui::Ui;
use zenthra_core::{
    BackdropFilter, Background, Border, BoxStyle, Color, EdgeInsets, Id, Placement, Point,
    PopoverAlign, Rect, Response, Size, Styleable,
};

/// Computes the (x, y) coordinates for a floating element relative to an anchor rect.
pub fn compute_placement_pos(
    anchor: Rect,
    content_w: f32,
    content_h: f32,
    placement: Placement,
    align: PopoverAlign,
    gap: f32,
    auto_flip: bool,
    viewport_w: f32,
    viewport_h: f32,
) -> (f32, f32) {
    let mut chosen_placement = placement;

    if auto_flip {
        match placement {
            Placement::Bottom => {
                let y = anchor.origin.y + anchor.size.height + gap;
                if y + content_h > viewport_h - 8.0 && anchor.origin.y - content_h - gap >= 8.0 {
                    chosen_placement = Placement::Top;
                }
            }
            Placement::Top => {
                let y = anchor.origin.y - content_h - gap;
                if y < 8.0 && anchor.origin.y + anchor.size.height + gap + content_h <= viewport_h - 8.0 {
                    chosen_placement = Placement::Bottom;
                }
            }
            Placement::Right => {
                let x = anchor.origin.x + anchor.size.width + gap;
                if x + content_w > viewport_w - 8.0 && anchor.origin.x - content_w - gap >= 8.0 {
                    chosen_placement = Placement::Left;
                }
            }
            Placement::Left => {
                let x = anchor.origin.x - content_w - gap;
                if x < 8.0 && anchor.origin.x + anchor.size.width + gap + content_w <= viewport_w - 8.0 {
                    chosen_placement = Placement::Right;
                }
            }
            Placement::Custom(_, _) => {}
        }
    }

    let (mut x, mut y) = match chosen_placement {
        Placement::Bottom => {
            let y = anchor.origin.y + anchor.size.height + gap;
            let x = match align {
                PopoverAlign::Start => anchor.origin.x,
                PopoverAlign::Center => anchor.origin.x + (anchor.size.width - content_w) / 2.0,
                PopoverAlign::End => anchor.origin.x + anchor.size.width - content_w,
            };
            (x, y)
        }
        Placement::Top => {
            let y = anchor.origin.y - content_h - gap;
            let x = match align {
                PopoverAlign::Start => anchor.origin.x,
                PopoverAlign::Center => anchor.origin.x + (anchor.size.width - content_w) / 2.0,
                PopoverAlign::End => anchor.origin.x + anchor.size.width - content_w,
            };
            (x, y)
        }
        Placement::Right => {
            let x = anchor.origin.x + anchor.size.width + gap;
            let y = match align {
                PopoverAlign::Start => anchor.origin.y,
                PopoverAlign::Center => anchor.origin.y + (anchor.size.height - content_h) / 2.0,
                PopoverAlign::End => anchor.origin.y + anchor.size.height - content_h,
            };
            (x, y)
        }
        Placement::Left => {
            let x = anchor.origin.x - content_w - gap;
            let y = match align {
                PopoverAlign::Start => anchor.origin.y,
                PopoverAlign::Center => anchor.origin.y + (anchor.size.height - content_h) / 2.0,
                PopoverAlign::End => anchor.origin.y + anchor.size.height - content_h,
            };
            (x, y)
        }
        Placement::Custom(ox, oy) => (anchor.origin.x + ox, anchor.origin.y + oy),
    };

    // Viewport clamping so the popover is never rendered outside screen bounds
    x = x.clamp(8.0, (viewport_w - content_w - 8.0).max(8.0));
    y = y.clamp(8.0, (viewport_h - content_h - 8.0).max(8.0));

    (x, y)
}

/// A floating container anchored to any widget on screen with Placement options.
pub struct PopoverBuilder<'u, 'a> {
    pub ui: &'u mut Ui<'a>,
    pub anchor_id: Id,
    pub id: Id,

    // Placement
    pub placement: Placement,
    pub align: PopoverAlign,
    pub gap: f32,
    pub auto_flip: bool,

    // Unified Box Surface Style
    pub style: BoxStyle,

    // Backdrop effects
    pub backdrop_filter: Option<BackdropFilter>,
    pub backdrop_blur: Option<f32>,
}

impl<'u, 'a> Styleable for PopoverBuilder<'u, 'a> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

impl_box_style_methods!(PopoverBuilder);

impl<'u, 'a> PopoverBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, anchor_id: Id) -> Self {
        let id = ui.id_from(anchor_id.raw().wrapping_add(888000));
        let mut style = BoxStyle::new();
        style.width = Some(200.0);
        style.radius = [6.0; 4];
        style.padding = EdgeInsets::all(4.0);
        style.bg = Some(Background::Solid(Color::rgb(0.10, 0.10, 0.12)));
        style.border = Border::Solid(Color::rgb(0.20, 0.20, 0.24), 1.0);
        style.shadow_color = Some(Color::rgba(0.0, 0.0, 0.0, 0.5));
        style.shadow_offset = [0.0, 4.0];
        style.shadow_blur = 16.0;

        Self {
            ui,
            anchor_id,
            id,
            placement: Placement::Bottom,
            align: PopoverAlign::Start,
            gap: 4.0,
            auto_flip: true,
            style,
            backdrop_filter: None,
            backdrop_blur: None,
        }
    }

    pub fn id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn auto_flip(mut self, auto_flip: bool) -> Self {
        self.auto_flip = auto_flip;
        self
    }

    pub fn backdrop_filter(mut self, filter: BackdropFilter) -> Self {
        self.backdrop_filter = Some(filter);
        self
    }

    pub fn backdrop_blur(mut self, radius: f32) -> Self {
        self.backdrop_blur = Some(radius);
        self
    }

    /// Renders the popover overlay anchored to the trigger widget.
    pub fn show<F>(self, f: F) -> Response
    where
        F: FnOnce(&mut Ui),
    {
        // 1. Locate anchor widget rect from layout cache
        let anchor_rect = self
            .ui
            .next_screen_layout_cache
            .get(&self.anchor_id)
            .or_else(|| self.ui.screen_layout_cache.get(&self.anchor_id))
            .copied()
            .unwrap_or(Rect {
                origin: Point {
                    x: self.ui.cursor_x,
                    y: self.ui.cursor_y,
                },
                size: Size {
                    width: 32.0,
                    height: 32.0,
                },
            });

        // 2. Measure content dimensions (use measured height from last frame if not explicitly given)
        let content_w = self.style.width.unwrap_or(200.0);
        let estimated_h = self.style.height.unwrap_or_else(|| {
            self.ui
                .screen_layout_cache
                .get(&self.id)
                .map(|r| r.size.height)
                .unwrap_or(120.0)
        });

        // 3. Compute position based on placement and alignment
        let (pos_x, pos_y) = compute_placement_pos(
            anchor_rect,
            content_w,
            estimated_h,
            self.placement,
            self.align,
            self.gap,
            self.auto_flip,
            self.ui.width,
            self.ui.height,
        );

        let content_h = estimated_h;
        let mut clicked_inside = false;
        let is_content_hovered = self.ui.mouse_in_rect(pos_x, pos_y, content_w, content_h);
        let is_anchor_hovered = self.ui.mouse_in_rect(
            anchor_rect.origin.x,
            anchor_rect.origin.y,
            anchor_rect.size.width,
            anchor_rect.size.height,
        );
        let is_hovered = is_content_hovered || is_anchor_hovered;

        // 4. Render as an overlay container
        let popover_id = self.id;

        let anchor_win = self
            .ui
            .widget_window_map
            .get(&self.anchor_id)
            .or_else(|| self.ui.next_widget_window_map.get(&self.anchor_id))
            .copied()
            .or(self.ui.modal_window_id);
        let prev_win_id = self.ui.current_window_id;
        if let Some(win) = anchor_win {
            self.ui.current_window_id = Some(win);
        }

        let backdrop_filter = self.backdrop_filter;
        let backdrop_blur = self.backdrop_blur;

        self.ui.overlay(|ui| {
            let mut container = ui
                .container()
                .raw_id(popover_id)
                .pos(pos_x, pos_y)
                .overlay()
                .style(self.style.clone())
                .column();

            if let Some(bf) = backdrop_filter {
                container = container.backdrop_filter(bf);
            }

            if let Some(radius) = backdrop_blur {
                container = container.backdrop_blur(radius);
            }

            let resp = container.show(|ui| {
                f(ui);
            });
            clicked_inside = resp.clicked;
        });

        self.ui.current_window_id = prev_win_id;

        Response {
            hovered: is_hovered,
            clicked: clicked_inside,
            pressed: false,
            submitted: false,
            changed: false,
        }
    }

    /// Renders the popover and automatically dismisses `is_open` when clicking outside both the popover and trigger anchor.
    pub fn show_toggle<F>(self, is_open: &mut bool, f: F) -> Response
    where
        F: FnOnce(&mut Ui),
    {
        let clicked = self.ui.clicked;
        let resp = self.show(f);
        if clicked && !resp.hovered {
            *is_open = false;
        }
        resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_popover_box_style_parity() {
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

        let anchor = Id::from_u64(12345);
        let popover = ui.popover(anchor)
            .width(260.0)
            .height(180.0)
            .bg(Color::rgb(0.12, 0.13, 0.18))
            .border((Color::rgb(0.3, 0.4, 0.6), 1.5))
            .radius_all(10.0)
            .padding_all(12.0)
            .shadow(Color::rgba(0.0, 0.0, 0.0, 0.6), 0.0, 8.0, 24.0);

        assert_eq!(popover.style.width, Some(260.0));
        assert_eq!(popover.style.height, Some(180.0));
        assert_eq!(popover.style.bg, Some(Background::Solid(Color::rgb(0.12, 0.13, 0.18))));
        assert_eq!(popover.style.border, Border::Solid(Color::rgb(0.3, 0.4, 0.6), 1.5));
        assert_eq!(popover.style.radius, [10.0; 4]);
        assert_eq!(popover.style.padding.top, 12.0);
        assert_eq!(popover.style.shadow_blur, 24.0);
    }
}
