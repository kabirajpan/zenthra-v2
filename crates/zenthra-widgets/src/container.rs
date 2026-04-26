use crate::ui::{DrawCommand, RectDraw, Ui};
use zenthra_core::Color;
use zenthra_render::RectInstance;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HAlign {
    Left,
    Center,
    Right,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Wrap {
    NoWrap,
    Wrap,
    WrapReverse,
    RightToLeft,
    RightToLeftReverse,
}

pub struct ContainerBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    direction: Direction,
    halign: HAlign,
    valign: VAlign,
    wrap: Wrap,
    children_draws: Vec<DrawCommand>,
    child_sizes: Vec<(f32, f32)>,
    child_ranges: Vec<(usize, usize)>,
    start_x: f32,
    start_y: f32,
    pos_x: Option<f32>,
    pos_y: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    fill_x: bool,
    fill_y: bool,
    padding_top: f32,
    padding_bottom: f32,
    padding_left: f32,
    padding_right: f32,
    gap: f32,
    bg: Option<Color>,
    radius: f32,
    border_color: Option<Color>,
    border_width: f32,
    shadow_blur: f32,
    shadow_color: Option<Color>,
    opacity: f32,
    render_mode: Option<zenthra_core::RenderMode>,
}

impl<'u, 'a> ContainerBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        Self {
            ui,
            direction: Direction::Column,
            halign: HAlign::Left,
            valign: VAlign::Top,
            wrap: Wrap::NoWrap,
            children_draws: Vec::new(),
            child_sizes: Vec::new(),
            child_ranges: Vec::new(),
            start_x: 0.0,
            start_y: 0.0,
            pos_x: None,
            pos_y: None,
            width: None,
            height: None,
            fill_x: false,
            fill_y: false,
            padding_top: 0.0,
            padding_bottom: 0.0,
            padding_left: 0.0,
            padding_right: 0.0,
            gap: 0.0,
            bg: None,
            radius: 0.0,
            border_color: None,
            border_width: 0.0,
            shadow_blur: 0.0,
            shadow_color: None,
            opacity: 1.0,
            render_mode: None,
        }
    }

    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.pos_x = Some(x);
        self.pos_y = Some(y);
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
    pub fn fill_x(mut self) -> Self {
        self.fill_x = true;
        self
    }
    pub fn fill_y(mut self) -> Self {
        self.fill_y = true;
        self
    }
    pub fn fill(mut self) -> Self {
        self.fill_x = true;
        self.fill_y = true;
        self
    }
    pub fn padding(mut self, p: f32) -> Self {
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
    pub fn padding_top(mut self, p: f32) -> Self {
        self.padding_top = p;
        self
    }
    pub fn padding_bottom(mut self, p: f32) -> Self {
        self.padding_bottom = p;
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

    pub fn wrap(mut self) -> Self {
        self.wrap = Wrap::Wrap;
        self
    }

    pub fn no_wrap(mut self) -> Self {
        self.wrap = Wrap::NoWrap;
        self
    }

    pub fn center(mut self) -> Self {
        self.halign = HAlign::Center;
        self.valign = VAlign::Center;
        self
    }

    pub fn center_x(mut self) -> Self {
        self.halign = HAlign::Center;
        self
    }

    pub fn center_y(mut self) -> Self {
        self.valign = VAlign::Center;
        self
    }

    pub fn align_top(mut self) -> Self {
        self.valign = VAlign::Top;
        self
    }

    pub fn align_bottom(mut self) -> Self {
        self.valign = VAlign::Bottom;
        self
    }

    pub fn align_left(mut self) -> Self {
        self.halign = HAlign::Left;
        self
    }

    pub fn align_right(mut self) -> Self {
        self.halign = HAlign::Right;
        self
    }

    pub fn continuous(mut self) -> Self {
        self.render_mode = Some(zenthra_core::RenderMode::Continuous);
        self
    }

    pub fn static_mode(mut self) -> Self {
        self.render_mode = Some(zenthra_core::RenderMode::Static);
        self
    }

    pub fn render_mode(mut self, mode: zenthra_core::RenderMode) -> Self {
        self.render_mode = Some(mode);
        self
    }

    pub fn halign(mut self, align: HAlign) -> Self {
        self.halign = align;
        self
    }

    pub fn valign(mut self, align: VAlign) -> Self {
        self.valign = align;
        self
    }

    pub fn padding_left(mut self, p: f32) -> Self {
        self.padding_left = p;
        self
    }
    pub fn padding_right(mut self, p: f32) -> Self {
        self.padding_right = p;
        self
    }
    pub fn gap(mut self, g: f32) -> Self {
        self.gap = g;
        self
    }
    pub fn bg(mut self, c: Color) -> Self {
        self.bg = Some(c);
        self
    }
    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }
    pub fn border(mut self, c: Color, w: f32) -> Self {
        self.border_color = Some(c);
        self.border_width = w;
        self
    }
    pub fn shadow(mut self, blur: f32) -> Self {
        self.shadow_blur = blur;
        self
    }
    pub fn shadow_color(mut self, c: Color) -> Self {
        self.shadow_color = Some(c);
        self
    }
    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = o;
        self
    }

    pub fn show<F>(mut self, f: F) 
    where F: FnOnce(&mut Ui)
    {
        // 1. Capture the start position and own ID
        let id = self.ui.id();
        
        self.start_x = self.ui.cursor_x;
        self.start_y = self.ui.cursor_y;
        let ox = self.pos_x.unwrap_or(self.start_x);
        let oy = self.pos_y.unwrap_or(self.start_y);

        // -- Save current Ui environment --
        let prev_dir = self.ui.direction;
        let prev_line_h = self.ui.line_height;
        let prev_base_x = self.ui.base_x;
        let prev_base_y = self.ui.base_y;
        let prev_offset_x = self.ui.offset_x;
        let prev_offset_y = self.ui.offset_y;
        
        // Take and isolate state maps for children
        let prev_child_sizes = std::mem::take(&mut self.ui.child_sizes);
        let prev_child_ranges = std::mem::take(&mut self.ui.child_draw_ranges);
        let prev_id_ranges = std::mem::take(&mut self.ui.id_ranges);
        let prev_id_log = std::mem::take(&mut self.ui.id_log);

        // Set child environment
        self.ui.direction = self.direction;
        self.ui.line_height = 0.0;
        self.ui.base_x = ox + self.padding_left;
        self.ui.base_y = oy + self.padding_top;
        self.ui.cursor_x = self.ui.base_x;
        self.ui.cursor_y = self.ui.base_y;
        
        let mode = self.render_mode.unwrap_or(self.ui.current_render_mode());
        self.ui.render_mode_stack.push(mode);
        if mode == zenthra_core::RenderMode::Continuous {
            self.ui.needs_redraw = true;
        }

        self.ui.semantic_stack.push(id);
        self.ui.register_semantic(zenthra_core::SemanticNode::new(id, zenthra_core::Role::Container, zenthra_core::Rect::new(ox, oy, 0.0, 0.0)));

        let parent_draws = std::mem::take(&mut self.ui.draws);

        // -- Run Children --
        f(self.ui);

        // -- Restore Stacks and Environment --
        self.ui.render_mode_stack.pop();
        self.ui.semantic_stack.pop();
        
        // Capture everything the children created
        let child_ids_only = std::mem::replace(&mut self.ui.id_log, prev_id_log);
        let child_id_ranges = std::mem::replace(&mut self.ui.id_ranges, prev_id_ranges);
        self.children_draws = std::mem::replace(&mut self.ui.draws, parent_draws);
        self.child_sizes = std::mem::replace(&mut self.ui.child_sizes, prev_child_sizes);
        self.child_ranges = std::mem::replace(&mut self.ui.child_draw_ranges, prev_child_ranges);

        // Restore cursor/base
        self.ui.direction = prev_dir;
        self.ui.line_height = prev_line_h;
        self.ui.base_x = prev_base_x;
        self.ui.base_y = prev_base_y;
        self.ui.offset_x = prev_offset_x;
        self.ui.offset_y = prev_offset_y;
        self.ui.cursor_x = self.start_x;
        self.ui.cursor_y = self.start_y;

        // -- Layout Engine --
        let n = self.child_sizes.len();

        let avail_w = if self.fill_x {
            self.ui.width - ox
        } else if let Some(w) = self.width {
            w - self.padding_left - self.padding_right
        } else {
            f32::INFINITY
        };

        let avail_h = if self.fill_y {
            self.ui.height - oy
        } else if let Some(h) = self.height {
            h - self.padding_top - self.padding_bottom
        } else {
            f32::INFINITY
        };

        let mut target_positions: Vec<(f32, f32)> = vec![(0.0, 0.0); n];
        let (content_w, content_h) = match self.wrap {
            Wrap::NoWrap => self.layout_no_wrap(ox, oy, avail_w, avail_h, &mut target_positions),
            _ => self.layout_wrap(ox, oy, avail_w, avail_h, &mut target_positions),
        };

        let w = if self.fill_x {
            self.ui.width - ox
        } else {
            self.width.unwrap_or(content_w + self.padding_left + self.padding_right)
        };
        let h = if self.fill_y {
            self.ui.height - oy
        } else {
            self.height.unwrap_or(content_h + self.padding_top + self.padding_bottom)
        };

        let draw_start = self.ui.draws.len();
        self.ui.record_layout(id, zenthra_core::Rect::new(ox, oy, w, h));

        // Background
        if let Some(bg) = self.bg {
            let bc = self.border_color.unwrap_or(Color::TRANSPARENT);
            let sc = self.shadow_color.unwrap_or(Color::TRANSPARENT);
            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [ox, oy],
                    size: [w, h],
                    color: bg.to_array(),
                    radius: [self.radius; 4],
                    border_width: self.border_width,
                    border_color: bc.to_array(),
                    shadow_color: sc.to_array(),
                    shadow_offset: [0.0, 0.0],
                    shadow_blur: self.shadow_blur,
                    clip_rect: [0.0, 0.0, self.ui.width, self.ui.height],
                    grayscale: 0.0,
                    brightness: 1.0,
                    opacity: self.opacity,
                },
            }));
        }

        // Final Translation Pass
        for (i, (start, end)) in self.child_ranges.iter().enumerate() {
            if i >= target_positions.len() { break; }
            let (tx, ty) = target_positions[i];

            let (origin_x, origin_y) = self
                .children_draws
                .get(*start)
                .map(|d| draw_origin(d))
                .unwrap_or((ox + self.padding_left, oy + self.padding_top));

            let dx = tx - origin_x;
            let dy = ty - origin_y;

            if dx != 0.0 || dy != 0.0 {
                // visuals
                for draw in &mut self.children_draws[*start..*end] {
                    offset_draw(draw, dx, dy);
                }
                
                // hit-test regions
                if let Some(&(id_s, id_e)) = child_id_ranges.get(i) {
                    for j in id_s..id_e {
                        let cid = child_ids_only[j];
                        if let Some(rect) = self.ui.next_layout_cache.get_mut(&cid) {
                            rect.origin.x += dx;
                            rect.origin.y += dy;
                        }
                    }
                }
            }
        }

        // Flush children draws to parent
        for draw in self.children_draws.drain(..) {
            self.ui.draws.push(draw);
        }
        
        // Bubble IDs up to parent's scope
        self.ui.id_log.push(id);
        self.ui.id_log.extend(child_ids_only);
        
        self.ui.advance(w, h, draw_start);
    }

    fn layout_no_wrap(
        &self,
        ox: f32,
        oy: f32,
        inner_w: f32,
        inner_h: f32,
        targets: &mut [(f32, f32)],
    ) -> (f32, f32) {
        let n = self.child_sizes.len();
        if n == 0 { return (0.0, 0.0); }

        let (content_w, content_h) = match self.direction {
            Direction::Row => {
                let w = self.child_sizes.iter().map(|(w, _)| w).sum::<f32>()
                    + self.gap * (n.saturating_sub(1)) as f32;
                let h = self.child_sizes.iter().map(|(_, h)| *h).fold(0.0f32, f32::max);
                (w, h)
            }
            Direction::Column => {
                let w = self.child_sizes.iter().map(|(w, _)| *w).fold(0.0f32, f32::max);
                let h = self.child_sizes.iter().map(|(_, h)| h).sum::<f32>()
                    + self.gap * (n.saturating_sub(1)) as f32;
                (w, h)
            }
        };

        let real_w = if inner_w.is_infinite() { content_w } else { inner_w };
        let real_h = if inner_h.is_infinite() { content_h } else { inner_h };

        match self.target_halign_valign(ox, oy, real_w, real_h, content_w, content_h, targets) {
            (w, h) => (w, h),
        }
    }
    
    fn target_halign_valign(&self, ox: f32, oy: f32, real_w: f32, real_h: f32, content_w: f32, content_h: f32, targets: &mut [(f32, f32)]) -> (f32, f32) {
        let n = self.child_sizes.len();
        match self.direction {
            Direction::Row => {
                let extra = (real_w - content_w).max(0.0);
                let mut cx = ox + self.padding_left + match self.halign {
                    HAlign::Left => 0.0,
                    HAlign::Center => extra / 2.0,
                    HAlign::Right => extra,
                    HAlign::SpaceBetween => 0.0,
                    HAlign::SpaceAround => extra / (n as f32 * 2.0),
                };
                let gap = match self.halign {
                    HAlign::SpaceBetween if n > 1 => extra / (n - 1) as f32,
                    HAlign::SpaceAround => extra / n as f32,
                    _ => self.gap,
                };
                for (i, (cw, ch)) in self.child_sizes.iter().enumerate() {
                    let cy = oy + self.padding_top + match self.valign {
                        VAlign::Top => 0.0,
                        VAlign::Center => (real_h - ch) / 2.0,
                        VAlign::Bottom => real_h - ch,
                        _ => 0.0,
                    };
                    targets[i] = (cx, cy);
                    cx += cw + gap;
                }
            }
            Direction::Column => {
                let extra = (real_h - content_h).max(0.0);
                let mut cy = oy + self.padding_top + match self.valign {
                    VAlign::Top => 0.0,
                    VAlign::Center => extra / 2.0,
                    VAlign::Bottom => extra,
                    VAlign::SpaceBetween => 0.0,
                    VAlign::SpaceAround => extra / (n as f32 * 2.0),
                };
                let gap = match self.valign {
                    VAlign::SpaceBetween if n > 1 => extra / (n - 1) as f32,
                    VAlign::SpaceAround => extra / n as f32,
                    _ => self.gap,
                };
                for (i, (cw, ch)) in self.child_sizes.iter().enumerate() {
                    let cx = ox + self.padding_left + match self.halign {
                        HAlign::Left => 0.0,
                        HAlign::Center => (real_w - cw) / 2.0,
                        HAlign::Right => real_w - cw,
                        _ => 0.0,
                    };
                    targets[i] = (cx, cy);
                    cy += ch + gap;
                }
            }
        }
        (content_w, content_h)
    }

    fn layout_wrap(
        &self,
        ox: f32,
        oy: f32,
        inner_w: f32,
        inner_h: f32,
        targets: &mut [(f32, f32)],
    ) -> (f32, f32) {
        let n = self.child_sizes.len();
        if n == 0 { return (0.0, 0.0); }

        let (main_reversed, cross_reversed) = match self.wrap {
            Wrap::Wrap => (false, false),
            Wrap::WrapReverse => (false, true),
            Wrap::RightToLeft => (true, false),
            Wrap::RightToLeftReverse => (true, true),
            _ => (false, false),
        };

        match self.direction {
            Direction::Row => {
                let mut rows: Vec<Vec<usize>> = Vec::new();
                let mut current_row = Vec::new();
                let mut row_w = 0.0f32;

                for (i, (cw, _)) in self.child_sizes.iter().enumerate() {
                    let needed = if current_row.is_empty() { *cw } else { row_w + self.gap + cw };
                    if needed > inner_w && !current_row.is_empty() {
                        rows.push(std::mem::take(&mut current_row));
                        current_row.push(i);
                        row_w = *cw;
                    } else {
                        current_row.push(i);
                        row_w = needed;
                    }
                }
                if !current_row.is_empty() { rows.push(current_row); }

                let row_heights: Vec<f32> = rows.iter().map(|row| {
                    row.iter().map(|&i| self.child_sizes[i].1).fold(0.0f32, f32::max)
                }).collect();

                let total_h = row_heights.iter().sum::<f32>() + self.gap * (rows.len().saturating_sub(1)) as f32;
                let max_row_w = rows.iter().map(|row| {
                    row.iter().map(|&i| self.child_sizes[i].0).sum::<f32>() + self.gap * (row.len().saturating_sub(1)) as f32
                }).fold(0.0f32, f32::max);

                let real_h = if inner_h.is_infinite() { total_h } else { inner_h };
                let real_w = if inner_w.is_infinite() { max_row_w } else { inner_w };

                let mut cy = if cross_reversed {
                    oy + self.padding_top + match self.valign {
                        VAlign::Top => real_h,
                        VAlign::Center => (real_h + total_h) / 2.0,
                        VAlign::Bottom => total_h,
                        _ => real_h,
                    }
                } else {
                    oy + self.padding_top + match self.valign {
                        VAlign::Center => (real_h - total_h).max(0.0) / 2.0,
                        VAlign::Bottom => (real_h - total_h).max(0.0),
                        _ => 0.0,
                    }
                };

                for (ri, row) in rows.iter().enumerate() {
                    let row_h = row_heights[ri];
                    let row_content_w = row.iter().map(|&i| self.child_sizes[i].0).sum::<f32>() + self.gap * (row.len().saturating_sub(1)) as f32;
                    let extra = (real_w - row_content_w).max(0.0);
                    if cross_reversed { cy -= row_h; }

                    let mut cx = if main_reversed {
                        ox + self.padding_left + match self.halign {
                            HAlign::Left => real_w,
                            HAlign::Center => (real_w + row_content_w) / 2.0,
                            HAlign::Right => row_content_w,
                            _ => real_w,
                        }
                    } else {
                        ox + self.padding_left + match self.halign {
                            HAlign::Left => 0.0,
                            HAlign::Center => extra / 2.0,
                            HAlign::Right => extra,
                            HAlign::SpaceBetween => 0.0,
                            HAlign::SpaceAround => extra / (row.len() as f32 * 2.0),
                        }
                    };

                    let gap = match self.halign {
                        HAlign::SpaceBetween if row.len() > 1 && !main_reversed => extra / (row.len() - 1) as f32,
                        HAlign::SpaceAround if !main_reversed => extra / row.len() as f32,
                        _ => self.gap,
                    };

                    for &ci in row {
                        let (cw, ch) = self.child_sizes[ci];
                        if main_reversed { cx -= cw; }
                        let child_y = cy + match self.valign {
                            VAlign::Center => (row_h - ch) / 2.0,
                            VAlign::Bottom => row_h - ch,
                            _ => 0.0,
                        };
                        targets[ci] = (cx, child_y);
                        if main_reversed { cx -= gap; } else { cx += cw + gap; }
                    }
                    if cross_reversed { cy -= self.gap; } else { cy += row_h + self.gap; }
                }
                (max_row_w, total_h)
            }
            Direction::Column => {
                let mut cols: Vec<Vec<usize>> = Vec::new();
                let mut current_col = Vec::new();
                let mut col_h = 0.0f32;

                for (i, (_, ch)) in self.child_sizes.iter().enumerate() {
                    let needed = if current_col.is_empty() { *ch } else { col_h + self.gap + ch };
                    if needed > inner_h && !current_col.is_empty() {
                        cols.push(std::mem::take(&mut current_col));
                        current_col.push(i);
                        col_h = *ch;
                    } else {
                        current_col.push(i);
                        col_h = needed;
                    }
                }
                if !current_col.is_empty() { cols.push(current_col); }

                let col_widths: Vec<f32> = cols.iter().map(|col| {
                    col.iter().map(|&i| self.child_sizes[i].0).fold(0.0f32, f32::max)
                }).collect();

                let total_w = col_widths.iter().sum::<f32>() + self.gap * (cols.len().saturating_sub(1)) as f32;
                let max_col_h = cols.iter().map(|col| {
                    col.iter().map(|&i| self.child_sizes[i].1).sum::<f32>() + self.gap * (col.len().saturating_sub(1)) as f32
                }).fold(0.0f32, f32::max);

                let real_w = if inner_w.is_infinite() { total_w } else { inner_w };
                let real_h = if inner_h.is_infinite() { max_col_h } else { inner_h };

                let mut cx = if cross_reversed {
                    ox + self.padding_left + match self.halign {
                        HAlign::Left => real_w,
                        HAlign::Center => (real_w + total_w) / 2.0,
                        HAlign::Right => total_w,
                        _ => real_w,
                    }
                } else {
                    ox + self.padding_left + match self.halign {
                        HAlign::Center => (real_w - total_w).max(0.0) / 2.0,
                        HAlign::Right => (real_w - total_w).max(0.0),
                        _ => 0.0,
                    }
                };

                for (ci_idx, col) in cols.iter().enumerate() {
                    let col_w = col_widths[ci_idx];
                    let col_content_h = col.iter().map(|&i| self.child_sizes[i].1).sum::<f32>() + self.gap * (col.len().saturating_sub(1)) as f32;
                    let extra = (real_h - col_content_h).max(0.0);
                    if cross_reversed { cx -= col_w; }

                    let mut cy = if main_reversed {
                        oy + self.padding_top + match self.valign {
                            VAlign::Top => real_h,
                            VAlign::Center => (real_h + col_content_h) / 2.0,
                            VAlign::Bottom => col_content_h,
                            _ => real_h,
                        }
                    } else {
                        oy + self.padding_top + match self.valign {
                            VAlign::Top => 0.0,
                            VAlign::Center => extra / 2.0,
                            VAlign::Bottom => extra,
                            VAlign::SpaceBetween => 0.0,
                            VAlign::SpaceAround => extra / (col.len() as f32 * 2.0),
                        }
                    };

                    let gap = match self.valign {
                        VAlign::SpaceBetween if col.len() > 1 && !main_reversed => extra / (col.len() - 1) as f32,
                        VAlign::SpaceAround if !main_reversed => extra / col.len() as f32,
                        _ => self.gap,
                    };

                    for &ci in col {
                        let (cw, ch) = self.child_sizes[ci];
                        if main_reversed { cy -= ch; }
                        let child_x = cx + match self.halign {
                            HAlign::Center => (col_w - cw) / 2.0,
                            HAlign::Right => col_w - cw,
                            _ => 0.0,
                        };
                        targets[ci] = (child_x, cy);
                        if main_reversed { cy -= gap; } else { cy += ch + gap; }
                    }
                    if cross_reversed { cx -= self.gap; } else { cx += col_w + self.gap; }
                }
                (total_w, max_col_h)
            }
        }
    }
}

/// Offset a draw command's position by (dx, dy).
fn offset_draw(cmd: &mut DrawCommand, dx: f32, dy: f32) {
    match cmd {
        DrawCommand::Rect(r) => {
            r.instance.pos[0] += dx;
            r.instance.pos[1] += dy;
        }
        DrawCommand::Text(t) => {
            t.pos[0] += dx;
            t.pos[1] += dy;
            t.options.x += dx;
            t.options.y += dy;
            t.clip[0] += dx;
            t.clip[1] += dy;
        }
        DrawCommand::OverlayRect(c) => {
            c.x += dx;
            c.y += dy;
        }
    }
}

fn draw_origin(cmd: &DrawCommand) -> (f32, f32) {
    match cmd {
        DrawCommand::Rect(r) => (r.instance.pos[0], r.instance.pos[1]),
        DrawCommand::Text(t) => (t.pos[0], t.pos[1]),
        DrawCommand::OverlayRect(c) => (c.x, c.y),
    }
}
