use crate::ui::{Ui, DrawCommand, OverlayRectDraw};
use crate::text::TextBuilder;
use zenthra_core::{Color, EdgeInsets};
use zenthra_platform::event::PlatformEvent;
use zenthra_text::prelude::{TextOptions, CosmicFontProvider, Padding};
use zenthra_text::traits::FontProvider;

pub struct TextAreaBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    buffer: &'b mut String,
    id: u64,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
    bg: Option<Color>,
    text_bg: Option<Color>,
    padding: EdgeInsets,
    text_padding: EdgeInsets,
    line_height: f32,
    width: f32,
    height: Option<f32>,
    scrollable: bool,
    overflow_hidden: bool,
    text_bg_full_width: bool,
    full_width: bool,
    wrap: zenthra_text::prelude::TextWrap,
}

impl<'u, 'a, 'b> TextAreaBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, buffer: &'b mut String, id: u64) -> Self {
        let x = ui.cursor_x;
        let y = ui.cursor_y;
        Self {
            ui,
            buffer,
            id,
            x,
            y,
            font_size: 18.0,
            color: Color::WHITE,
            bg: Some(Color::rgb(0.2, 0.2, 0.2)),
            text_bg: None,
            padding: EdgeInsets::ZERO,
            text_padding: EdgeInsets::ZERO,
            line_height: 1.2,
            width: 300.0,
            height: None,
            scrollable: false,
            overflow_hidden: false,
            text_bg_full_width: false,
            full_width: false,
            wrap: zenthra_text::prelude::TextWrap::Word,
        }
    }

    pub fn scrollable(mut self, enabled: bool) -> Self {
        self.scrollable = enabled;
        if enabled {
            self.overflow_hidden = true;
        }
        self
    }

    pub fn overflow_hidden(mut self, enabled: bool) -> Self {
        self.overflow_hidden = enabled;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn text_bg(mut self, bg: Color) -> Self {
        self.text_bg = Some(bg);
        self
    }

    pub fn padding(mut self, padding: impl Into<EdgeInsets>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn text_padding(mut self, padding: impl Into<EdgeInsets>) -> Self {
        self.text_padding = padding.into();
        self
    }

    pub fn line_height(mut self, lh: f32) -> Self {
        self.line_height = lh;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    pub fn text_bg_full_width(mut self, enabled: bool) -> Self {
        self.text_bg_full_width = enabled;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    pub fn wrap(mut self, strategy: zenthra_text::prelude::TextWrap) -> Self {
        self.wrap = strategy;
        self
    }

    pub fn show(self) {
        let is_focused = self.ui.focused_id == Some(self.id);
        
        // --- 1. Handle Scroll State ---
        let mut scroll_y = if self.scrollable {
            *self.ui.scroll_state.get(&self.id).unwrap_or(&0.0)
        } else {
            0.0
        };

        // --- 2. Initial Measure (to get content height) ---
        let actual_width = if self.full_width {
            (self.ui.width - self.x).max(self.width)
        } else {
            self.width
        };

        let (_content_w, mut h_content, mut shaped_buffer) = if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
            let t_padding = Padding::from(self.text_padding);
            let layout_width = actual_width - self.padding.horizontal() - t_padding.horizontal();
            adapter.set_layout_size(layout_width, 10000.0);
            
            let options = TextOptions::new()
                .font_size(self.font_size)
                .line_height(self.line_height)
                .wrap(self.wrap)
                .padding(t_padding);
            
            let buffer = adapter.shape(&self.buffer, &options);
            let (_, ch) = buffer.content_size();
            (actual_width, ch + t_padding.vertical(), Some(buffer))
        } else {
            (actual_width, 20.0, None)
        };

        let mut h_box = if self.scrollable {
            self.height.unwrap_or(200.0)
        } else {
            h_content + self.padding.vertical()
        };
        
        if self.scrollable && self.height.unwrap_or(0.0) == 0.0 {
            h_box = 200.0;
        }

        // --- 3. Handle Events ---
        let mut cursor_index = *self.ui.cursor_state.get(&self.id).unwrap_or(&self.buffer.len());
        cursor_index = cursor_index.min(self.buffer.len());
        if !self.buffer.is_char_boundary(cursor_index) {
            cursor_index = self.buffer.len(); // Safety
        }

        let is_hovered = self.ui.mouse_in_rect(self.x, self.y, actual_width, h_box);

        if is_focused || is_hovered {
            let events = std::mem::take(&mut self.ui.input_events);
            let mut changed = false;
            for event in &events {
                match event {
                    PlatformEvent::CharTyped(c) if is_focused => {
                        if *c != '\r' && *c != '\n' {
                             self.buffer.insert(cursor_index, *c);
                             cursor_index += c.len_utf8();
                             changed = true;
                        }
                    }
                    PlatformEvent::KeyDown { key } if is_focused => {
                        match key {
                            winit::keyboard::KeyCode::Backspace => {
                                if cursor_index > 0 {
                                    let mut chars = self.buffer[..cursor_index].chars();
                                    if let Some(c) = chars.next_back() {
                                        let len = c.len_utf8();
                                        self.buffer.remove(cursor_index - len);
                                        cursor_index -= len;
                                        changed = true;
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::ArrowLeft => {
                                if cursor_index > 0 {
                                    let mut chars = self.buffer[..cursor_index].chars();
                                    if let Some(c) = chars.next_back() {
                                        cursor_index -= c.len_utf8();
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::ArrowRight => {
                                if cursor_index < self.buffer.len() {
                                    let mut chars = self.buffer[cursor_index..].chars();
                                    if let Some(c) = chars.next() {
                                        cursor_index += c.len_utf8();
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::ArrowUp => {
                                if let Some(sb) = &shaped_buffer {
                                    let row_threshold = (self.font_size * self.line_height) * 0.5;

                                    // 1. Find EXACTLY which line index the cursor is on
                                    let mut current_line_idx = 0;
                                    for (i, line) in sb.lines().iter().enumerate() {
                                        if line.start_cluster <= cursor_index {
                                            current_line_idx = i;
                                        } else {
                                            break;
                                        }
                                    }

                                    if current_line_idx > 0 {
                                        // 2. Identify target line and target X
                                        let target_line_idx = current_line_idx - 1;
                                        let target_line = &sb.lines()[target_line_idx];
                                        
                                        // Find where we are horizontally on the current line to try and match it
                                        let mut target_x = 0.0;
                                        if let Some(current_line) = sb.lines().get(current_line_idx) {
                                            for g in sb.glyphs() {
                                                if (g.y - current_line.y).abs() < row_threshold {
                                                    if g.cluster < cursor_index {
                                                        target_x = g.x + g.width;
                                                    } else if g.cluster == cursor_index {
                                                        target_x = g.x;
                                                        break;
                                                    }
                                                }
                                            }
                                        }

                                        // 3. Find closest character on target line
                                        let mut best_idx = None;
                                        let mut best_dist = f32::INFINITY;
                                        for g in sb.glyphs() {
                                            if (g.y - target_line.y).abs() < row_threshold {
                                                let dist = (g.x - target_x).abs();
                                                if dist < best_dist {
                                                    best_dist = dist;
                                                    best_idx = Some(g.cluster);
                                                }
                                            }
                                        }

                                        if let Some(idx) = best_idx {
                                            cursor_index = idx;
                                        } else {
                                            // Fallback for empty target line
                                            cursor_index = target_line.start_cluster;
                                        }
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::ArrowDown => {
                                if let Some(sb) = &shaped_buffer {
                                    let row_threshold = (self.font_size * self.line_height) * 0.5;

                                    // 1. Find EXACTLY which line index the cursor is on
                                    let mut current_line_idx = 0;
                                    for (i, line) in sb.lines().iter().enumerate() {
                                        if line.start_cluster <= cursor_index {
                                            current_line_idx = i;
                                        } else {
                                            break;
                                        }
                                    }

                                    if current_line_idx < sb.lines().len() - 1 {
                                        // 2. Identify target line and target X
                                        let target_line_idx = current_line_idx + 1;
                                        let target_line = &sb.lines()[target_line_idx];
                                        
                                        // Find where we are horizontally on current line
                                        let mut target_x = 0.0;
                                        if let Some(current_line) = sb.lines().get(current_line_idx) {
                                            for g in sb.glyphs() {
                                                if (g.y - current_line.y).abs() < row_threshold {
                                                    if g.cluster < cursor_index {
                                                        target_x = g.x + g.width;
                                                    } else if g.cluster == cursor_index {
                                                        target_x = g.x;
                                                        break;
                                                    }
                                                }
                                            }
                                        }

                                        // 3. Find closest character on target line
                                        let mut best_idx = None;
                                        let mut best_dist = f32::INFINITY;
                                        for g in sb.glyphs() {
                                            if (g.y - target_line.y).abs() < row_threshold {
                                                let dist = (g.x - target_x).abs();
                                                if dist < best_dist {
                                                    best_dist = dist;
                                                    best_idx = Some(g.cluster);
                                                }
                                            }
                                        }

                                        if let Some(idx) = best_idx {
                                            cursor_index = idx;
                                        } else {
                                            // Fallback for empty target line
                                            cursor_index = target_line.start_cluster;
                                        }
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::Enter | winit::keyboard::KeyCode::NumpadEnter => {
                                self.buffer.insert(cursor_index, '\n');
                                cursor_index += 1;
                                changed = true;
                            }
                            _ => {}
                        }
                    }
                    PlatformEvent::MouseWheel { delta_x, delta_y } if self.scrollable && is_hovered => {
                        let total_delta = *delta_x + *delta_y;
                        scroll_y -= total_delta * 30.0; 
                    }
                    _ => {}
                }
            }

                    if changed {
                        // RE-SHAPE after buffer modification
                        if let Some(fs) = self.ui.font_system.as_ref() {
                            let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
                            let t_padding = Padding::from(self.text_padding);
                            let layout_width = actual_width - self.padding.horizontal() - t_padding.horizontal();
                            adapter.set_layout_size(layout_width, 10000.0);
                            
                            let options = TextOptions::new().font_size(self.font_size).line_height(self.line_height).wrap(self.wrap).padding(t_padding);
                            let buffer = adapter.shape(&self.buffer, &options);
                            let (_cw, ch) = buffer.content_size();
                            h_content = ch + t_padding.vertical();
                            shaped_buffer = Some(buffer);
                            
                            // Re-calculate h_box immediately if not fixed
                            if !self.scrollable {
                                h_box = h_content + self.padding.vertical();
                            }
                        }
                        
                        self.ui.cursor_state.insert(self.id, cursor_index);
                    }
                    
                    if self.scrollable {
                        let usable_h = h_box - self.padding.vertical();
                        let max_scroll = (h_content - usable_h).max(0.0);
                        scroll_y = scroll_y.clamp(0.0, max_scroll);
                    }
                    self.ui.input_events = events;
                    self.ui.cursor_state.insert(self.id, cursor_index);
                }

        // --- 4. Render Background (FIXED) ---
        let start_draw = self.ui.draws.len();
        if let Some(bg) = self.bg {
            use crate::ui::RectDraw;
            use zenthra_render::RectInstance;
            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [self.x, self.y],
                    size: [actual_width, h_box],
                    color: bg.to_array(),
                    radius: 4.0, // Match typical UI style
                    border_width: if is_focused { 1.0 } else { 0.0 },
                    border_color: [1.0, 1.0, 1.0, 0.4],
                    shadow_color: [0.0, 0.0, 0.0, 0.0],
                    shadow_offset: [0.0, 0.0],
                    shadow_blur: 0.0,
                    clip_rect: [0.0, 0.0, 9999.0, 9999.0],
                    grayscale: 0.0,
                    brightness: 1.0,
                    opacity: 1.0,
                }
            }));
        }

        // --- 5. Render Text (CLIPPED if overflow_hidden) ---
        let pos_y = self.y + self.padding.top - scroll_y;
        let mut text_builder = TextBuilder::new(self.ui, &self.buffer)
            .size(self.font_size)
            .line_height(self.line_height)
            .color(self.color)
            .full_width_bg(false)
            .padding(self.text_padding)
            .wrap(self.wrap)
            .max_width(actual_width - self.padding.horizontal())
            .pos(self.x + self.padding.left, pos_y);

        if self.text_bg_full_width {
            text_builder = text_builder.min_width(actual_width - self.padding.horizontal());
        }

        if let Some(tbg) = self.text_bg {
            text_builder = text_builder.bg(tbg).full_width_bg(false);
        }

        if self.overflow_hidden {
            // Reverted to full-box clipping so content can merge with edges during scroll
            text_builder = text_builder.clip_rect(self.x, self.y, actual_width, h_box);
        }
        
        // Final draw
        text_builder.draw_and_measure();

        // Update persistent scroll state
        if self.scrollable {
            self.ui.scroll_state.insert(self.id, scroll_y);

            // --- 5.5 Render Scroll Bar ---
            if h_content > h_box - self.padding.vertical() {
                let usable_h = h_box - self.padding.vertical();
                let scroll_bar_w = 4.0;
                let scroll_bar_x = self.x + actual_width - scroll_bar_w - 4.0;
                
                let thumb_h = (usable_h / h_content) * usable_h;
                let thumb_h = thumb_h.clamp(20.0, usable_h);
                
                let max_scroll = (h_content - usable_h).max(1.0);
                let scroll_percent = scroll_y / max_scroll;
                let thumb_y = self.y + self.padding.top + scroll_percent * (usable_h - thumb_h);

                self.ui.draws.push(DrawCommand::OverlayRect(OverlayRectDraw {
                    x: scroll_bar_x,
                    y: thumb_y,
                    width: scroll_bar_w,
                    height: thumb_h,
                    color: Color::rgba(1.0, 1.0, 1.0, 0.4),
                    clip: [self.x, self.y, actual_width, h_box], 
                }));
            }
        }

        // --- 6. Handle Focus ---
        if self.ui.mouse_down {
            if is_hovered {
                self.ui.focused_id = Some(self.id);
            }
        }

        // --- 7. Cursor Rendering ---
        if is_focused {
            let font_size = self.font_size;
            let lh = self.line_height;
            let cursor_height = font_size * lh;
            let visual_ascent = font_size * (0.8 + (lh - 1.0) / 2.0);

            if let Some(sb) = shaped_buffer {
                let first_line_y = sb.lines().first().map(|l| l.y).unwrap_or(visual_ascent);
                let v_shift = visual_ascent - first_line_y;

                let mut lx = 0.0;
                let mut ly = first_line_y;
                let mut found = false;

                for g in sb.glyphs() {
                    if g.cluster == cursor_index {
                        lx = g.x;
                        ly = g.y;
                        found = true;
                        break;
                    }
                }

                if !found {
                    let mut best_line = None;
                    for line in sb.lines() {
                        if line.start_cluster <= cursor_index {
                            best_line = Some(line);
                        } else {
                            break;
                        }
                    }

                    if let Some(line) = best_line {
                        ly = line.y;
                        // If we are past the start of the line and not found in glyphs,
                        // we're likely at a newline or trailing space.
                        if cursor_index > line.start_cluster {
                            lx = line.width;
                        } else {
                            lx = 0.0;
                        }
                    }
                    
                    if cursor_index == self.buffer.len() {
                        if let Some(lg) = sb.glyphs().last() {
                            if (lg.y - ly).abs() < 2.0 {
                                lx = lg.x + lg.width;
                            } else {
                                // Last line might be empty
                                if let Some(last_line) = sb.lines().last() {
                                    ly = last_line.y;
                                    lx = 0.0;
                                }
                            }
                        }
                    }
                }
                
                let cursor_height = font_size * lh;
                let v_shift = visual_ascent - first_line_y;
                let cx = lx + self.x + self.padding.left + self.text_padding.left;
                let cy = ly + self.y + self.padding.top + self.text_padding.top + v_shift - visual_ascent - scroll_y;
                
                // Blink logic: temporarily disabled for focus work
                let is_blink_visible = true; // self.ui.elapsed_time.fract() < 0.5;
                
                // We'll use a simple trick: if the cursor state was JUST updated in this frame,
                // we'll force visibility. Since we can't easily track "last activity" across frames 
                // without adding more state, we'll use the fract() logic for now, but I'll add 
                // a small "active" state check if possible.
                
                // Only draw cursor if it's within the viewport and in the visible blink phase
                if cy >= self.y - 2.0 && cy + cursor_height <= self.y + h_box + 2.0 && is_blink_visible {
                    self.ui.draws.push(DrawCommand::OverlayRect(OverlayRectDraw {
                        x: cx,
                        y: cy,
                        width: 2.0,
                        height: cursor_height,
                        color: Color::WHITE,
                        clip: [self.x, self.y, actual_width, h_box],
                    }));
                }
            }
        }

        // --- 8. Advance UI ---
        self.ui.advance(actual_width, h_box, start_draw);
    }
}
