// crates/zenthra-widgets/src/layout/stack.rs

use crate::ui::{DrawCommand, RectDraw, Ui};
use zenthra_core::{Color, Id, Rect, Align, BorderAlignment};
use zenthra_render::RectInstance;
use crate::container::Direction;

pub struct StackBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,
    child_origins: Vec<(f32, f32)>,

    // Layout
    width: Option<f32>,
    height: Option<f32>,
    fill_x: bool,
    fill_y: bool,
    padding_top: f32,
    padding_bottom: f32,
    padding_left: f32,
    padding_right: f32,
    halign: Align,
    valign: Align,

    // Styling
    bg: Option<Color>,
    border_color: Option<Color>,
    border_width: f32,
    radius: [f32; 4],
    shadow_color: Option<Color>,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,
    opacity: f32,
    clip: bool,
    border_alignment: BorderAlignment,
    is_overlay: bool,
    is_absolute: bool,
}

impl<'u, 'a> StackBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        let id = ui.id();
        Self {
            ui,
            id,
            child_origins: Vec::new(),
            width: None,
            height: None,
            fill_x: false,
            fill_y: false,
            padding_top: 0.0,
            padding_bottom: 0.0,
            padding_left: 0.0,
            padding_right: 0.0,
            halign: Align::Left,
            valign: Align::Top,
            bg: None,
            border_color: None,
            border_width: 0.0,
            radius: [0.0; 4],
            shadow_color: None,
            shadow_offset: [0.0, 0.0],
            shadow_blur: 0.0,
            shadow_opacity: 1.0,
            opacity: 1.0,
            clip: false,
            border_alignment: BorderAlignment::Inside,
            is_overlay: false,
            is_absolute: false,
        }
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = Some(w);
        self.height = Some(h);
        self
    }

    pub fn full_width(mut self) -> Self {
        self.fill_x = true;
        self
    }

    pub fn full_height(mut self) -> Self {
        self.fill_y = true;
        self
    }

    pub fn fill(mut self) -> Self {
        self.fill_x = true;
        self.fill_y = true;
        self
    }

    pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.padding_top = t;
        self.padding_bottom = b;
        self.padding_left = l;
        self.padding_right = r;
        self
    }

    pub fn padding_all(mut self, p: f32) -> Self {
        self.padding_top = p;
        self.padding_bottom = p;
        self.padding_left = p;
        self.padding_right = p;
        self
    }

    pub fn padding_x(mut self, p: f32) -> Self {
        self.padding_left = p;
        self.padding_right = p;
        self
    }

    pub fn padding_y(mut self, p: f32) -> Self {
        self.padding_top = p;
        self.padding_bottom = p;
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self
    }

    pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.radius = [tl, tr, br, bl];
        self
    }

    pub fn radius_all(mut self, r: f32) -> Self {
        self.radius = [r, r, r, r];
        self
    }

    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.shadow_color = Some(color);
        self.shadow_offset = [x, y];
        self.shadow_blur = blur;
        self
    }

    pub fn shadow_opacity(mut self, opacity: f32) -> Self {
        self.shadow_opacity = opacity;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    pub fn border_alignment(mut self, alignment: BorderAlignment) -> Self {
        self.border_alignment = alignment;
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

        // Isolate stack bounds limits
        if let Some(w) = self.width {
            self.ui.max_x = (ox + w - self.padding_right).min(prev_max_x);
        }
        if let Some(h) = self.height {
            self.ui.max_y = (oy + h - self.padding_bottom).min(prev_max_y);
        }

        self.ui.direction = Direction::Stack;
        self.ui.line_height = 0.0;

        let avail_w = if let Some(w) = self.width {
            w - self.padding_left - self.padding_right - 2.0 * self.border_width
        } else {
            (prev_max_x - ox - self.padding_left - self.padding_right - 2.0 * self.border_width).max(0.0)
        };
        let avail_h = if let Some(h) = self.height {
            h - self.padding_top - self.padding_bottom - 2.0 * self.border_width
        } else {
            (prev_max_y - oy - self.padding_top - self.padding_bottom - 2.0 * self.border_width).max(0.0)
        };

        self.ui.max_x = ox + self.padding_left + self.border_width + avail_w;
        self.ui.max_y = oy + self.padding_top + self.border_width + avail_h;

        self.ui.semantic_stack.push(id);
        self.ui.register_semantic(zenthra_core::SemanticNode::new(
            id,
            zenthra_core::Role::Container,
            zenthra_core::Rect::new(ox, oy, 0.0, 0.0),
        ));

        let parent_draws = std::mem::take(&mut self.ui.draws);

        self.ui.base_x = ox + self.padding_left + self.border_width;
        self.ui.base_y = oy + self.padding_top + self.border_width;
        self.ui.cursor_x = self.ui.base_x;
        self.ui.cursor_y = self.ui.base_y;

        // Update viewport for children if clipping is enabled
        if self.clip {
            if let Some((rect, _)) = self.ui.get_recorded_layout(id) {
                let my_screen_rect = [
                    rect.origin.x + prev_offset_x,
                    rect.origin.y + prev_offset_y,
                    rect.size.width,
                    rect.size.height,
                ];
                let parent_rect = [
                    prev_viewport.origin.x,
                    prev_viewport.origin.y,
                    prev_viewport.size.width,
                    prev_viewport.size.height,
                ];
                let intersected = intersect_rects(my_screen_rect, parent_rect);
                self.ui.current_viewport = Rect::new(intersected[0], intersected[1], intersected[2], intersected[3]);
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

        let w = if self.fill_x {
            self.ui.max_x - ox
        } else {
            self.width.unwrap_or(content_w + self.padding_left + self.padding_right + 2.0 * self.border_width)
        };
        let h = if self.fill_y {
            self.ui.max_y - oy
        } else {
            self.height.unwrap_or(content_h + self.padding_top + self.padding_bottom + 2.0 * self.border_width)
        };

        self.ui.record_layout(id, Rect::new(ox, oy, w, h));

        // Advance parent layout cursor
        let (adv_w, adv_h) = match prev_dir {
            Direction::Column => (0.0, h),
            Direction::Row => (w, 0.0),
            Direction::Stack => (0.0, 0.0),
        };

        // Background / Border visual Rect
        let mut target_draws = Vec::new();
        if let Some(bg) = self.bg {
            let bw = self.border_width;
            let bc = self.border_color.unwrap_or(Color::TRANSPARENT);

            target_draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [ox, oy],
                    size: [w, h],
                    color: bg.to_array(),
                    radius: [
                        self.radius[3], // Bottom-Left -> Top-Left
                        self.radius[2], // Bottom-Right -> Top-Right
                        self.radius[1], // Top-Right -> Bottom-Right
                        self.radius[0], // Top-Left -> Bottom-Left
                    ],
                    border_width: bw,
                    border_color: bc.to_array(),
                    shadow_color: self.shadow_color.map(|c| {
                        let mut a = c.to_array();
                        a[3] *= self.shadow_opacity;
                        a
                    }).unwrap_or([0.0, 0.0, 0.0, 0.0]),
                    shadow_offset: self.shadow_offset,
                    shadow_blur: self.shadow_blur,
                    clip_rect: [-100000.0, -100000.0, 2000000.0, 2000000.0],
                    grayscale: 0.0,
                    brightness: 1.0,
                    opacity: self.opacity,
                    border_alignment: match self.border_alignment {
                        BorderAlignment::Inside => 0.0,
                        BorderAlignment::Center => 0.5,
                        BorderAlignment::Outside => 1.0,
                    },
                }
            }));
        }

        // Process children layout and shifting
        let inner_w = w - self.padding_left - self.padding_right - 2.0 * self.border_width;
        let inner_h = h - self.padding_top - self.padding_bottom - 2.0 * self.border_width;

        let clip = [ox + prev_offset_x, oy + prev_offset_y, w, h];

        let mut children_draws_mut = children_draws;

        for (i, (start, end)) in child_ranges.iter().enumerate() {
            if i >= child_sizes.len() { break; }
            let (cw, ch) = child_sizes[i];

            let tx = ox + self.padding_left + self.border_width + match self.halign {
                Align::Center => (inner_w - cw).max(0.0) / 2.0,
                Align::Right => inner_w - cw,
                _ => 0.0,
            };

            let ty = oy + self.padding_top + self.border_width + match self.valign {
                Align::Center => (inner_h - ch).max(0.0) / 2.0,
                Align::Bottom => inner_h - ch,
                _ => 0.0,
            };

            let (origin_x, origin_y) = self.child_origins.get(i).copied().unwrap_or_else(|| {
                children_draws_mut
                    .get(*start)
                    .map(|d| draw_origin(d))
                    .unwrap_or((ox + self.padding_left + self.border_width, oy + self.padding_top + self.border_width))
            });

            let dx = tx - origin_x;
            let dy = ty - origin_y;

            for draw in &mut children_draws_mut[*start..*end] {
                set_clip(draw, clip);
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
