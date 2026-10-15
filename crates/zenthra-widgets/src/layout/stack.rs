// crates/zenthra-widgets/src/layout/stack.rs

use crate::container::Direction;
use crate::style_ext::BoxStyleExt;
use crate::ui::{DrawCommand, RectDraw, Ui};
use zenthra_core::{Align, BoxStyle, Id, Rect, Styleable};

/// Builder for layered, overlapping stack layouts (`ui.stack()`)
pub struct StackBuilder<'u, 'a> {
    pub ui: &'u mut Ui<'a>,
    pub id: Id,
    pub child_origins: Vec<(f32, f32)>,

    // Unified Box Surface Style
    pub style: BoxStyle,

    // Stack Alignment & Behavior
    pub halign: Align,
    pub valign: Align,
    pub clip: bool,
    pub is_overlay: bool,
    pub is_absolute: bool,
}

impl<'u, 'a> Styleable for StackBuilder<'u, 'a> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

impl_box_style_methods!(StackBuilder);

impl<'u, 'a> StackBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        let id = ui.id();
        Self {
            ui,
            id,
            child_origins: Vec::new(),
            style: BoxStyle::new(),
            halign: Align::Left,
            valign: Align::Top,
            clip: false,
            is_overlay: false,
            is_absolute: false,
        }
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = self.ui.id_from(id);
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
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

    pub fn align(mut self, h: Align, v: Align) -> Self {
        self.halign = h;
        self.valign = v;
        self
    }

    pub fn overlay(mut self) -> Self {
        self.is_overlay = true;
        self
    }

    pub fn absolute(mut self) -> Self {
        self.is_absolute = true;
        self
    }

    pub fn show<F>(mut self, f: F)
    where
        F: FnOnce(&mut Ui),
    {
        let id = self.id;
        let start_x = self.ui.cursor_x;
        let start_y = self.ui.cursor_y;
        let ox = start_x;
        let oy = start_y;

        // -- Save environment --
        let prev_dir = self.ui.direction;
        let prev_line_h = self.ui.line_height;
        let prev_base_x = self.ui.base_x;
        let prev_base_y = self.ui.base_y;
        let prev_offset_x = self.ui.offset_x;
        let prev_offset_y = self.ui.offset_y;
        let prev_viewport = self.ui.current_viewport;

        let prev_child_sizes = std::mem::take(&mut self.ui.child_sizes);
        let prev_child_ranges = std::mem::take(&mut self.ui.child_draw_ranges);
        let prev_child_origins = std::mem::take(&mut self.ui.child_origins);
        let prev_id_ranges = std::mem::take(&mut self.ui.id_ranges);
        let prev_id_log = std::mem::take(&mut self.ui.id_log);

        let prev_max_x = self.ui.max_x;
        let prev_max_y = self.ui.max_y;

        let border_w = self.style.border_width();

        // Isolate stack bounds limits
        if let Some(w) = self.style.width {
            self.ui.max_x = (ox + w - self.style.padding.right).min(prev_max_x);
        }
        if let Some(h) = self.style.height {
            self.ui.max_y = (oy + h - self.style.padding.bottom).min(prev_max_y);
        }

        self.ui.direction = Direction::Stack;
        self.ui.line_height = 0.0;

        let avail_w = if let Some(w) = self.style.width {
            w - self.style.padding.horizontal() - 2.0 * border_w
        } else {
            (prev_max_x - ox - self.style.padding.horizontal() - 2.0 * border_w).max(0.0)
        };
        let avail_h = if let Some(h) = self.style.height {
            h - self.style.padding.vertical() - 2.0 * border_w
        } else {
            (prev_max_y - oy - self.style.padding.vertical() - 2.0 * border_w).max(0.0)
        };

        self.ui.max_x = ox + self.style.padding.left + border_w + avail_w;
        self.ui.max_y = oy + self.style.padding.top + border_w + avail_h;

        self.ui.semantic_stack.push(id);
        self.ui.register_semantic(zenthra_core::SemanticNode::new(
            id,
            zenthra_core::Role::Container,
            zenthra_core::Rect::new(ox, oy, 0.0, 0.0),
        ));

        let parent_draws = std::mem::take(&mut self.ui.draws);

        self.ui.base_x = ox + self.style.padding.left + border_w;
        self.ui.base_y = oy + self.style.padding.top + border_w;
        self.ui.cursor_x = self.ui.base_x;
        self.ui.cursor_y = self.ui.base_y;

        // Apply clip rect if requested
        if self.clip {
            if let Some(w) = self.style.width {
                if let Some(h) = self.style.height {
                    self.ui.current_viewport = Rect::new(ox + prev_offset_x, oy + prev_offset_y, w, h);
                }
            }
        }

        // -- Run Children --
        f(self.ui);

        // -- Restore Stacks and Environment --
        self.ui.semantic_stack.pop();
        self.ui.current_viewport = prev_viewport;

        // Capture child data
        let child_ids_only = std::mem::replace(&mut self.ui.id_log, prev_id_log);
        let child_id_ranges = std::mem::replace(&mut self.ui.id_ranges, prev_id_ranges);
        let children_draws = std::mem::replace(&mut self.ui.draws, parent_draws);
        let child_sizes = std::mem::replace(&mut self.ui.child_sizes, prev_child_sizes);
        let child_ranges = std::mem::replace(&mut self.ui.child_draw_ranges, prev_child_ranges);
        self.child_origins = std::mem::replace(&mut self.ui.child_origins, prev_child_origins);

        // Restore layout cursor
        self.ui.direction = prev_dir;
        self.ui.line_height = prev_line_h;
        self.ui.base_x = prev_base_x;
        self.ui.base_y = prev_base_y;
        self.ui.cursor_x = start_x;
        self.ui.cursor_y = start_y;
        self.ui.max_x = prev_max_x;
        self.ui.max_y = prev_max_y;

        // Calculate Stack sizes
        let content_w = child_sizes.iter().map(|(cw, _)| *cw).fold(0.0f32, f32::max);
        let content_h = child_sizes.iter().map(|(_, ch)| *ch).fold(0.0f32, f32::max);

        let mut w = if self.style.fill_x {
            self.ui.max_x - ox
        } else {
            self.style.width.unwrap_or(content_w + self.style.padding.horizontal() + 2.0 * border_w)
        };
        let mut h = if self.style.fill_y {
            self.ui.max_y - oy
        } else {
            self.style.height.unwrap_or(content_h + self.style.padding.vertical() + 2.0 * border_w)
        };

        if let Some(min_w) = self.style.min_width { w = w.max(min_w); }
        if let Some(max_w) = self.style.max_width { w = w.min(max_w); }
        if let Some(min_h) = self.style.min_height { h = h.max(min_h); }
        if let Some(max_h) = self.style.max_height { h = h.min(max_h); }

        self.ui.record_layout(id, Rect::new(ox, oy, w, h));

        // Advance parent layout cursor
        let (adv_w, adv_h) = match prev_dir {
            Direction::Column => (0.0, h),
            Direction::Row => (w, 0.0),
            Direction::Stack => (0.0, 0.0),
        };

        // Background / Border visual Rect
        let mut target_draws = Vec::new();
        if self.style.bg.is_some() || border_w > 0.0 {
            if let Some(instance) = self.style.build_rect_styled(
                ox,
                oy,
                w,
                h,
                self.style.bg.as_ref(),
                self.style.border_color(),
                Some(border_w),
                1.0,
            ) {
                target_draws.push(DrawCommand::Rect(RectDraw { instance }));
            }
        }

        // Process children layout and shifting
        let inner_w = w - self.style.padding.horizontal() - 2.0 * border_w;
        let inner_h = h - self.style.padding.vertical() - 2.0 * border_w;

        let clip = [ox + prev_offset_x, oy + prev_offset_y, w, h];

        let mut children_draws_mut = children_draws;

        for (i, (start, end)) in child_ranges.iter().enumerate() {
            if i >= child_sizes.len() { break; }
            let (cw, ch) = child_sizes[i];

            let tx = ox + self.style.padding.left + border_w + match self.halign {
                Align::Center => (inner_w - cw).max(0.0) / 2.0,
                Align::Right => inner_w - cw,
                _ => 0.0,
            };

            let ty = oy + self.style.padding.top + border_w + match self.valign {
                Align::Center => (inner_h - ch).max(0.0) / 2.0,
                Align::Bottom => inner_h - ch,
                _ => 0.0,
            };

            let (origin_x, origin_y) = self.child_origins.get(i).copied().unwrap_or_else(|| {
                children_draws_mut
                    .get(*start)
                    .map(|d| draw_origin(d))
                    .unwrap_or((ox + self.style.padding.left + border_w, oy + self.style.padding.top + border_w))
            });

            let dx = tx - origin_x;
            let dy = ty - origin_y;

            for draw in &mut children_draws_mut[*start..*end] {
                if self.clip {
                    set_clip(draw, clip);
                }
                offset_draw(draw, dx, dy);
            }

            // Shift IDs recursively in layout cache
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

        // Flush target draws and shifted children draws to the parent list
        for draw in target_draws {
            if self.is_overlay {
                self.ui.overlays.push(draw);
            } else {
                self.ui.draws.push(draw);
            }
        }

        let advance_draw_start = self.ui.draws.len();

        for draw in children_draws_mut {
            if self.is_overlay {
                self.ui.overlays.push(draw);
            } else {
                self.ui.draws.push(draw);
            }
        }

        // Bubble IDs up to parent's scope
        self.ui.id_log.extend(child_ids_only);

        if !self.is_absolute {
            self.ui.advance(adv_w, adv_h, advance_draw_start);
        }
    }
}

// Layout helper implementations
fn offset_draw(cmd: &mut DrawCommand, dx: f32, dy: f32) {
    match cmd {
        DrawCommand::Rect(r) => {
            r.instance.pos[0] += dx;
            r.instance.pos[1] += dy;
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
    use zenthra_core::{Background, Border, Color};

    #[test]
    fn test_stack_box_style_parity() {
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

        let stack = ui.stack()
            .width(300.0)
            .height(200.0)
            .padding_all(10.0)
            .margin_all(5.0)
            .radius_all(8.0)
            .bg(Color::rgb(0.1, 0.12, 0.15))
            .border((Color::rgb(0.3, 0.4, 0.5), 2.0))
            .shadow(Color::rgba(0.0, 0.0, 0.0, 0.5), 0.0, 4.0, 12.0)
            .halign(Align::Center)
            .valign(Align::Center);

        assert_eq!(stack.style.width, Some(300.0));
        assert_eq!(stack.style.height, Some(200.0));
        assert_eq!(stack.style.padding.top, 10.0);
        assert_eq!(stack.style.margin.top, 5.0);
        assert_eq!(stack.style.radius, [8.0; 4]);
        assert_eq!(stack.style.bg, Some(Background::Solid(Color::rgb(0.1, 0.12, 0.15))));
        assert_eq!(stack.style.border, Border::Solid(Color::rgb(0.3, 0.4, 0.5), 2.0));
        assert_eq!(stack.halign, Align::Center);
        assert_eq!(stack.valign, Align::Center);
    }
}
