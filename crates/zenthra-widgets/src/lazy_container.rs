use crate::ui::{Ui, DrawCommand, RectDraw, OverlayRectDraw};
use crate::container::{Direction, Wrap};
use zenthra_core::{Id, Color};
use zenthra_render::RectInstance;
use zenthra_platform::event::PlatformEvent;

pub struct LazyContainerBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    item_width: f32,
    item_height: f32,
    count: usize,
    gap: f32,
    padding: f32,
    bg: Option<Color>,
    radius: f32,
    direction: Direction,
    wrap_strategy: Wrap,
    id: Option<Id>,
}

impl<'u, 'a> LazyContainerBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        Self {
            ui,
            item_width: 100.0,
            item_height: 100.0,
            count: 0,
            gap: 15.0,
            padding: 0.0,
            bg: None,
            radius: 0.0,
            direction: Direction::Column,
            wrap_strategy: Wrap::NoWrap,
            id: None,
        }
    }

    pub fn item_size(mut self, w: f32, h: f32) -> Self {
        self.item_width = w;
        self.item_height = h;
        self
    }

    pub fn count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn padding(mut self, t: f32, _r: f32, _b: f32, _l: f32) -> Self {
        self.padding = t; // Note: Internal implementation only supports uniform padding for now
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn radius(mut self, tl: f32, _tr: f32, _br: f32, _bl: f32) -> Self {
        self.radius = tl; // Note: Internal implementation only supports uniform radius for now
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

    pub fn wrap(mut self, strategy: Wrap) -> Self {
        self.wrap_strategy = strategy;
        self
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        use std::hash::Hasher;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        id.hash(&mut hasher);
        self.id = Some(Id::from_u64(hasher.finish()));
        self
    }

    pub fn show<F>(self, mut f: F)
    where
        F: FnMut(&mut Ui, usize),
    {
        use std::hash::{Hash, Hasher};

        // ── 1. Stable identity ────────────────────────────────────────────────
        let id = self.id.unwrap_or_else(|| {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            "lazy_container".hash(&mut hasher);
            self.ui.id_counter.hash(&mut hasher);
            Id::from_u64(hasher.finish())
        });

        let ui = self.ui;

        // Container bounds (absolute screen position)
        let ox = ui.cursor_x;
        let oy = ui.cursor_y;
        let viewport_w = ui.available_width;
        let viewport_h = ui.height - oy;

        // ── 2. Grid math ──────────────────────────────────────────────────────
        let usable_w = viewport_w - self.padding * 2.0;

        // Simulate the container's wrap algorithm exactly so items_per_row matches reality
        let items_per_row = match (self.direction, self.wrap_strategy) {
            (Direction::Row, Wrap::Wrap) | (Direction::Row, Wrap::WrapReverse) => {
                let mut count = 0usize;
                let mut current_w = 0.0f32;
                for _ in 0..self.count.min(1000) {
                    let needed = if count == 0 {
                        self.item_width
                    } else {
                        current_w + self.gap + self.item_width
                    };
                    if needed > usable_w && count > 0 {
                        break;
                    }
                    current_w = needed;
                    count += 1;
                }
                count.max(1)
            }
            (Direction::Row, _) => self.count.max(1), // single horizontal row
            (Direction::Column, _) => 1,               // single vertical column
        };

        let row_count = (self.count as f32 / items_per_row as f32).ceil() as usize;
        // Total virtual content height (without padding)
        let total_content_h = row_count as f32 * self.item_height
            + row_count.saturating_sub(1) as f32 * self.gap;
        let total_scroll_h = total_content_h + self.padding * 2.0;

        // ── 3. Scroll state ───────────────────────────────────────────────────
        let (mut scroll_x, mut scroll_y) =
            *ui.scroll_state.get(&id).unwrap_or(&(0.0, 0.0));

        let is_hover = ui.mouse_x >= ox
            && ui.mouse_x <= ox + viewport_w
            && ui.mouse_y >= oy
            && ui.mouse_y <= oy + viewport_h;

        if is_hover {
            // Clone events to avoid borrow conflict
            let events: Vec<_> = ui.input_events.iter().cloned().collect();
            for event in &events {
                if let PlatformEvent::MouseWheel { delta_y, delta_x, .. } = event {
                    scroll_y -= delta_y * 15.0;
                    scroll_x -= delta_x * 15.0;
                    ui.needs_redraw = true;
                }
            }
        }

        let max_sy = (total_scroll_h - viewport_h).max(0.0);
        scroll_y = scroll_y.clamp(0.0, max_sy);
        scroll_x = scroll_x.clamp(0.0, 0.0); // horizontal scroll not needed for wrap layout
        ui.scroll_state.insert(id, (scroll_x, scroll_y));

        // ── 4. Visible index window ───────────────────────────────────────────
        let row_h = self.item_height + self.gap;
        let bleed = 2usize; // extra rows above & below viewport to prevent flicker
        let start_row = (scroll_y / row_h).floor() as usize;
        let start_row = start_row.saturating_sub(bleed);
        let end_row = ((scroll_y + viewport_h) / row_h).ceil() as usize + bleed;
        let start_idx = (start_row * items_per_row).min(self.count);
        let end_idx   = (end_row   * items_per_row).min(self.count);

        let clip = [ox, oy, viewport_w, viewport_h];

        // ── 5. Isolate child rendering from parent layout engine ──────────────
        // Save everything the parent cares about
        let prev_dir        = ui.direction;
        let prev_line_h     = ui.line_height;
        let prev_base_x     = ui.base_x;
        let prev_base_y     = ui.base_y;
        let prev_max_x      = ui.max_x;
        let prev_max_y      = ui.max_y;
        let prev_avail_w    = ui.available_width;
        let prev_offset_x   = ui.offset_x;
        let prev_offset_y   = ui.offset_y;
        let prev_child_sizes   = std::mem::take(&mut ui.child_sizes);
        let prev_child_ranges  = std::mem::take(&mut ui.child_draw_ranges);
        let prev_id_ranges     = std::mem::take(&mut ui.id_ranges);
        let prev_id_log        = std::mem::take(&mut ui.id_log);
        // Swap out the parent draw list; we'll collect children separately
        let parent_draws = std::mem::take(&mut ui.draws);

        // ── 6. Render visible items at absolute (scroll-adjusted) positions ───
        // Apply scroll offset to ui's logical offset so visibility checks work
        ui.offset_y -= scroll_y;
        ui.offset_x -= scroll_x;

        for i in start_idx..end_idx {
            let row = i / items_per_row;
            let col = i % items_per_row;

            let item_x = ox + self.padding + col as f32 * (self.item_width + self.gap);
            let item_y = oy + self.padding + row as f32 * row_h - scroll_y;

            // Set up the rendering context for this item
            ui.cursor_x      = item_x;
            ui.cursor_y      = item_y;
            ui.base_x        = item_x;
            ui.base_y        = item_y;
            ui.direction     = Direction::Column;
            ui.line_height   = 0.0;
            ui.max_x         = item_x + self.item_width;
            ui.max_y         = item_y + self.item_height;
            ui.available_width = self.item_width;

            let draw_start = ui.draws.len();
            f(ui, i);

            // Apply clip rect to every draw command this item produced
            for draw in &mut ui.draws[draw_start..] {
                apply_clip(draw, clip);
            }

            // Discard this item's layout tracking — we don't need it
            ui.child_sizes.clear();
            ui.child_draw_ranges.clear();
            ui.id_ranges.clear();
        }

        // Restore scroll offset
        ui.offset_y += scroll_y;
        ui.offset_x += scroll_x;

        // Collect item draws, restore parent draw list
        let item_draws = std::mem::replace(&mut ui.draws, parent_draws);

        // Restore all parent layout state
        ui.direction       = prev_dir;
        ui.line_height     = prev_line_h;
        ui.base_x          = prev_base_x;
        ui.base_y          = prev_base_y;
        ui.max_x           = prev_max_x;
        ui.max_y           = prev_max_y;
        ui.available_width = prev_avail_w;
        ui.offset_x        = prev_offset_x;
        ui.offset_y        = prev_offset_y;
        ui.cursor_x        = ox;
        ui.cursor_y        = oy;
        ui.child_sizes     = prev_child_sizes;
        ui.child_draw_ranges = prev_child_ranges;
        ui.id_ranges       = prev_id_ranges;
        ui.id_log          = prev_id_log;

        // ── 7. Flush draws: background → items → scrollbar ───────────────────
        let draw_start = ui.draws.len(); // for advance()

        // Background
        if let Some(bg) = self.bg {
            ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos:          [ox, oy],
                    size:         [viewport_w, viewport_h],
                    color:        bg.to_array(),
                    radius:       [self.radius; 4],
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT.to_array(),
                    shadow_color: Color::TRANSPARENT.to_array(),
                    shadow_offset:[0.0, 0.0],
                    shadow_blur:  0.0,
                    clip_rect:    [0.0, 0.0, ui.width, ui.height],
                    grayscale:    0.0,
                    brightness:   1.0,
                    opacity:      1.0,
                },
            }));
        }

        // Items
        for draw in item_draws {
            ui.draws.push(draw);
        }

        // Vertical scrollbar (if content overflows)
        if max_sy > 0.0 {
            let bar_thickness = 6.0;
            let bar_margin    = 2.0;
            let thumb_h = (viewport_h / total_scroll_h * viewport_h).max(20.0);
            let scroll_ratio = scroll_y / max_sy;
            let thumb_y = oy + (viewport_h - thumb_h) * scroll_ratio;
            let thumb_x = ox + viewport_w - bar_thickness - bar_margin;

            let is_thumb_hover = ui.mouse_in_rect(thumb_x - 2.0, thumb_y, bar_thickness + 4.0, thumb_h);
            let color = if is_thumb_hover {
                Color::rgba(1.0, 1.0, 1.0, 0.6)
            } else {
                Color::rgba(1.0, 1.0, 1.0, 0.25)
            };
            ui.draws.push(DrawCommand::OverlayRect(OverlayRectDraw {
                x: thumb_x, y: thumb_y,
                width: bar_thickness, height: thumb_h,
                color,
                clip,
            }));
        }

        // ── 8. Advance parent cursor ──────────────────────────────────────────
        ui.advance(viewport_w, viewport_h, draw_start);
    }
}

/// Apply a clip rect to any draw command.
fn apply_clip(cmd: &mut DrawCommand, clip: [f32; 4]) {
    match cmd {
        DrawCommand::Rect(r)        => r.instance.clip_rect = clip,
        DrawCommand::Text(t)        => t.clip = clip,
        DrawCommand::OverlayRect(o) => o.clip = clip,
    }
}
