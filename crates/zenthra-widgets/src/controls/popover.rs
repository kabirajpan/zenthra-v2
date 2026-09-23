// crates/zenthra-widgets/src/controls/popover.rs

use crate::ui::Ui;
use zenthra_core::{BackdropFilter, Color, Id, Placement, Point, PopoverAlign, Rect, Response, Size};

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
    ui: &'u mut Ui<'a>,
    anchor_id: Id,
    id: Id,

    // Placement
    placement: Placement,
    align: PopoverAlign,
    gap: f32,
    auto_flip: bool,

    // Dimensions
    width: f32,
    height: Option<f32>,

    // Styling
    bg: Option<Color>,
    border_color: Option<Color>,
    border_width: f32,
    radius: [f32; 4],
    shadow: bool,
    shadow_color: Color,
    shadow_blur: f32,
    shadow_offset: [f32; 2],
    padding: [f32; 4],
    backdrop_filter: Option<BackdropFilter>,
    backdrop_blur: Option<f32>,
}

impl<'u, 'a> PopoverBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, anchor_id: Id) -> Self {
        let id = ui.id_from(anchor_id.raw().wrapping_add(888000));
        Self {
            ui,
            anchor_id,
            id,
            placement: Placement::Bottom,
            align: PopoverAlign::Start,
            gap: 4.0,
            auto_flip: true,
            width: 200.0,
            height: None,
            bg: None,
            border_color: None,
            border_width: 1.0,
            radius: [6.0; 4],
            shadow: true,
            shadow_color: Color::rgba(0.0, 0.0, 0.0, 0.5),
            shadow_blur: 16.0,
            shadow_offset: [0.0, 4.0],
            padding: [4.0, 4.0, 4.0, 4.0],
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

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self
    }

    pub fn radius_all(mut self, r: f32) -> Self {
        self.radius = [r; 4];
        self
    }

    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.shadow = true;
        self.shadow_color = color;
        self.shadow_offset = [x, y];
        self.shadow_blur = blur;
        self
    }

    pub fn padding(mut self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        self.padding = [top, right, bottom, left];
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
        let estimated_h = self.height.unwrap_or_else(|| {
            self.ui
                .screen_layout_cache
                .get(&self.id)
                .map(|r| r.size.height)
                .unwrap_or(120.0)
        });

        // 3. Compute position based on placement and alignment
        let (pos_x, pos_y) = compute_placement_pos(
            anchor_rect,
            self.width,
            estimated_h,
            self.placement,
            self.align,
            self.gap,
            self.auto_flip,
            self.ui.width,
            self.ui.height,
        );

        let content_w = self.width;
        let content_h = estimated_h;
        let mut clicked_inside = false;
        let is_hovered = self.ui.mouse_in_rect(pos_x, pos_y, content_w, content_h);

        // 4. Render as an overlay container
        let bg_color = self.bg.unwrap_or(Color::rgb(0.10, 0.10, 0.12));
        let border_color = self.border_color.unwrap_or(Color::rgb(0.20, 0.20, 0.24));
        let border_width = self.border_width;
        let radius = self.radius;
        let shadow_color = self.shadow_color;
        let shadow_offset = self.shadow_offset;
        let shadow_blur = self.shadow_blur;
        let padding = self.padding;
        let backdrop_filter = self.backdrop_filter;
        let backdrop_blur = self.backdrop_blur;
        let popover_id = self.id;
        let has_explicit_h = self.height;

        self.ui.overlay(|ui| {
            let mut container = ui
                .container()
                .raw_id(popover_id)
                .absolute(pos_x, pos_y)
                .overlay()
                .width(content_w)
                .bg(bg_color)
                .border(border_color, border_width)
                .radius(radius[0], radius[1], radius[2], radius[3])
                .padding(padding[0], padding[1], padding[2], padding[3])
                .column();

            if let Some(h) = has_explicit_h {
                container = container.height(h);
            }

            if self.shadow {
                container = container.shadow(shadow_color, shadow_offset[0], shadow_offset[1], shadow_blur);
            }

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

        Response {
            hovered: is_hovered,
            clicked: clicked_inside,
            pressed: false,
            submitted: false,
        }
    }
}
