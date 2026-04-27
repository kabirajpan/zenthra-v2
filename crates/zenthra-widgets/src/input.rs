use crate::ui::{Ui, DrawCommand, OverlayRectDraw, ScrollDrag};
use crate::text::TextBuilder;
use zenthra_core::{Color, EdgeInsets, Id, Role, SemanticNode, Rect};
use zenthra_platform::event::PlatformEvent;
use zenthra_text::prelude::{TextOptions, CosmicFontProvider, Padding, ShapedGlyph};
use zenthra_text::traits::FontProvider;

pub struct InputBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    buffer: &'b mut String,
    id: Id,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
    bg: Option<Color>,
    text_bg: Option<Color>,
    highlight: Option<Color>,
    padding: EdgeInsets,
    text_padding: EdgeInsets,
    line_height: f32,
    width: Option<f32>,
    min_width: f32,
    scrollable: bool,
    full_width: bool,
    text_bg_full_width: bool,
    render_mode: Option<zenthra_core::RenderMode>,
}

impl<'u, 'a, 'b> InputBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, buffer: &'b mut String, id: Id) -> Self {
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
            width: None,
            min_width: 200.0,
            scrollable: true,
            full_width: false,
            text_bg_full_width: false,
            highlight: None,
            render_mode: None,
        }
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

    pub fn highlight(mut self, color: Color) -> Self {
        self.highlight = Some(color);
        self
    }

    pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.padding = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }
    pub fn padding_x(mut self, x: f32) -> Self {
        self.padding.left = x;
        self.padding.right = x;
        self
    }
    pub fn padding_y(mut self, y: f32) -> Self {
        self.padding.top = y;
        self.padding.bottom = y;
        self
    }
    pub fn padding_top(mut self, t: f32) -> Self {
        self.padding.top = t;
        self
    }
    pub fn padding_bottom(mut self, b: f32) -> Self {
        self.padding.bottom = b;
        self
    }
    pub fn padding_left(mut self, l: f32) -> Self {
        self.padding.left = l;
        self
    }
    pub fn padding_right(mut self, r: f32) -> Self {
        self.padding.right = r;
        self
    }

    pub fn text_padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.text_padding = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }
    pub fn text_padding_x(mut self, x: f32) -> Self {
        self.text_padding.left = x;
        self.text_padding.right = x;
        self
    }
    pub fn text_padding_y(mut self, y: f32) -> Self {
        self.text_padding.top = y;
        self.text_padding.bottom = y;
        self
    }
    pub fn text_padding_top(mut self, t: f32) -> Self {
        self.text_padding.top = t;
        self
    }
    pub fn text_padding_bottom(mut self, b: f32) -> Self {
        self.text_padding.bottom = b;
        self
    }
    pub fn text_padding_left(mut self, l: f32) -> Self {
        self.text_padding.left = l;
        self
    }
    pub fn text_padding_right(mut self, r: f32) -> Self {
        self.text_padding.right = r;
        self
    }

    pub fn line_height(mut self, lh: f32) -> Self {
        self.line_height = lh;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn min_width(mut self, w: f32) -> Self {
        self.min_width = w;
        self
    }

    pub fn scrollable(mut self, s: bool) -> Self {
        self.scrollable = s;
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

    pub fn render_mode(mut self, mode: zenthra_core::RenderMode) -> Self {
        self.render_mode = Some(mode);
        self
    }

    pub fn show(self) {
        if let Some(mode) = self.render_mode {
            self.ui.render_mode_stack.push(mode);
        }
        let is_focused = self.ui.focused_id == Some(self.id);
        
        // --- 1. Initial Measure (for hit-testing and initial sizing) ---
        let (mut w_text_raw, h_content, mut shaped_buffer) = if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
            let t_padding = Padding::from(self.text_padding);
            adapter.set_layout_size(1000000.0, 10000.0);
            let options = TextOptions::new()
                .font_size(self.font_size)
                .line_height(self.line_height)
                .wrap(zenthra_text::prelude::TextWrap::None);
            let buffer = adapter.shape(&self.buffer, &options);
            let (cw, _ch) = buffer.content_size();
            let m = adapter.metrics(&options);
            (cw + t_padding.horizontal(), m.line_height() + t_padding.vertical(), Some(buffer))
        } else {
            (self.min_width, 20.0, None)
        };

        let max_available_w = (self.ui.width - self.x).max(self.min_width);
        let mut w_box = if self.full_width { max_available_w } else { self.width.unwrap_or_else(|| (w_text_raw + self.padding.horizontal()).min(max_available_w)).max(self.min_width) };
        let h_box = h_content + self.padding.vertical();
        let mut w_view = w_box - self.padding.horizontal();

        // --- 2. Hit Testing & Event Handling ---
        let mut cursor_index = *self.ui.cursor_state.get(&self.id).unwrap_or(&self.buffer.len());
        cursor_index = cursor_index.min(self.buffer.len());
        if !self.buffer.is_char_boundary(cursor_index) {
            cursor_index = self.buffer.len();
        }

        let is_hovered = self.ui.mouse_in_rect(self.x, self.y, w_box, h_box);
        let mut needs_auto_scroll = false;
        let mut changed = false;

        if is_focused || is_hovered || self.ui.active_drag.is_some() {
            let events = std::mem::take(&mut self.ui.input_events);
            for event in &events {
                match event {
                    PlatformEvent::CharTyped(c) if is_focused => {
                        if *c != '\r' && *c != '\n' {
                             self.buffer.insert(cursor_index, *c);
                             cursor_index += c.len_utf8();
                             needs_auto_scroll = true;
                             changed = true;
                             self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
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
                                        needs_auto_scroll = true;
                                        changed = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::ArrowLeft => {
                                if cursor_index > 0 {
                                    let mut chars = self.buffer[..cursor_index].chars();
                                    if let Some(c) = chars.next_back() {
                                        cursor_index -= c.len_utf8();
                                        needs_auto_scroll = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::ArrowRight => {
                                if cursor_index < self.buffer.len() {
                                    let mut chars = self.buffer[cursor_index..].chars();
                                    if let Some(c) = chars.next() {
                                        cursor_index += c.len_utf8();
                                        needs_auto_scroll = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    PlatformEvent::MouseWheel { delta_x, delta_y } if is_hovered => {
                        let _id = self.id;
                        let (mut sx, sy) = *self.ui.scroll_state.get(&self.id).unwrap_or(&(0.0, 0.0));
                        let effect = if delta_x.abs() < 0.001 { *delta_y } else { *delta_x };
                        sx -= effect * 30.0;
                        self.ui.scroll_state.insert(self.id, (sx, sy));
                    }
                    _ => {}
                }
            }
            
            if self.ui.clicked && is_hovered {
                self.ui.focused_id = Some(self.id);
                needs_auto_scroll = true;
            }
            
            self.ui.input_events = events;
            self.ui.cursor_state.insert(self.id, cursor_index);
        }

        // --- 3. Re-Measure if content changed ---
        if changed {
             if let Some(fs) = self.ui.font_system.as_ref() {
                let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
                let t_padding = Padding::from(self.text_padding);
                adapter.set_layout_size(1000000.0, 10000.0);
                let options = TextOptions::new()
                    .font_size(self.font_size)
                    .line_height(self.line_height)
                    .wrap(zenthra_text::prelude::TextWrap::None);
                let buffer = adapter.shape(&self.buffer, &options);
                let (cw, _ch) = buffer.content_size();
                w_text_raw = cw + t_padding.horizontal();
                shaped_buffer = Some(buffer);
                
                // Re-calculate boxes
                w_box = if self.full_width { max_available_w } else { self.width.unwrap_or_else(|| (w_text_raw + self.padding.horizontal()).min(max_available_w)).max(self.min_width) };
                w_view = w_box - self.padding.horizontal();
            }
        }

        // --- 4. Scroll & Auto-Scroll Calculation ---
        let scroll_state_id = self.id;
        let (mut scroll_x, scroll_y) = *self.ui.scroll_state.get(&scroll_state_id).unwrap_or(&(0.0, 0.0));
        let max_scroll = (w_text_raw - w_view).max(0.0f32);

        let scroll_bar_h = 4.0;
        let scroll_bar_y = self.y + h_box - scroll_bar_h - 2.0;
        let thumb_w = if w_text_raw > w_view { (w_view / w_text_raw) * w_view } else { w_view };
        let thumb_w = thumb_w.max(20.0);
        let scroll_percent = if max_scroll > 0.0 { scroll_x / max_scroll } else { 0.0 };
        let track_width = (w_view - thumb_w).max(0.0);
        let thumb_x = self.x + self.padding.left + scroll_percent * track_width;

        let is_over_thumb = self.ui.mouse_in_rect(thumb_x, scroll_bar_y - 2.0, thumb_w, scroll_bar_h + 4.0);

        // Handle dragging
        if let Some(drag) = self.ui.active_drag {
            if drag.id == scroll_state_id {
                let delta_mouse = self.ui.mouse_x - drag.start_mouse;
                if track_width > 0.0 {
                    let scroll_delta = (delta_mouse / track_width) * max_scroll;
                    scroll_x = (drag.start_scroll + scroll_delta).clamp(0.0, max_scroll);
                }
            }
        }
        
        if self.ui.clicked && is_over_thumb {
            self.ui.active_drag = Some(ScrollDrag {
                id: scroll_state_id,
                start_mouse: self.ui.mouse_x,
                start_scroll: scroll_x,
            });
        }

        // Auto-scroll logic (Now uses FRESH buffer)
        let is_dragging_this = self.ui.active_drag.map(|d| d.id == scroll_state_id).unwrap_or(false);
        if is_focused && !is_dragging_this && needs_auto_scroll {
            if let Some(sb) = &shaped_buffer {
                let mut clx = 0.0;
                let mut found = false;
                for g in sb.glyphs() {
                    if g.cluster == cursor_index {
                        clx = g.x;
                        found = true;
                        break;
                    }
                }
                if !found && cursor_index == self.buffer.len() {
                    clx = sb.glyphs().last().map(|g: &ShapedGlyph| g.x + g.width).unwrap_or(0.0);
                }

                let cursor_x_v = clx + self.text_padding.left;
                if cursor_x_v > scroll_x + w_view - 40.0 {
                    scroll_x = cursor_x_v - w_view + 40.0;
                } else if cursor_x_v < scroll_x + 10.0 {
                    scroll_x = (cursor_x_v - 10.0).max(0.0);
                }
            }
        }

        scroll_x = scroll_x.clamp(0.0, max_scroll);
        self.ui.scroll_state.insert(scroll_state_id, (scroll_x, scroll_y));

        // --- 4. Render Background ---
        let start_draw = self.ui.draws.len();
        if let Some(bg) = self.bg {
            use crate::ui::RectDraw;
            use zenthra_render::RectInstance;
            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [self.x, self.y],
                    size: [w_box, h_box],
                    color: bg.to_array(),
                    radius: [4.0; 4],
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

        // --- 5. Render Text ---
        let mut text_builder = TextBuilder::new(self.ui, &self.buffer)
            .size(self.font_size)
            .line_height(self.line_height)
            .color(self.color)
            .full_width_bg(false) 
            .padding(self.text_padding.top, self.text_padding.right, self.text_padding.bottom, self.text_padding.left)
            .wrap(zenthra_text::prelude::TextWrap::None)
            .max_width(1000000.0) 
            .pos(self.x + self.padding.left - scroll_x, self.y + self.padding.top)
            .clip_rect(self.x, self.y, w_box, h_box);

        if self.text_bg_full_width {
            text_builder = text_builder.min_width(w_view);
        }
        
        if let Some(tbg) = self.text_bg {
            text_builder = text_builder.bg(tbg).full_width_bg(false);
        }

        if let Some(h) = self.highlight {
            text_builder = text_builder.highlight(h);
        }
        
        let (_, _, final_sb, _) = text_builder.draw_and_measure();
        
        // --- 6. Cursor Rendering ---
        if is_focused {
            let font_size = self.font_size;
            let lh = self.line_height;
            let cursor_height = font_size * lh; 

            if let Some(sb) = final_sb {
                let mut lx = 0.0;
                let mut found = false;
                for g in sb.glyphs() {
                    if g.cluster == cursor_index {
                        lx = g.x;
                        found = true;
                        break;
                    }
                }

                if !found && cursor_index == self.buffer.len() {
                    lx = sb.glyphs().last().map(|g: &ShapedGlyph| g.x + g.width).unwrap_or(0.0);
                }

                let cx = lx + self.x + self.padding.left + self.text_padding.left - scroll_x;
                let cy = self.y + self.padding.top + self.text_padding.top;
                
                // Smart Blink: Solid while typing, blink when idle
                let last_activity = *self.ui.interaction_state.get(&self.id).unwrap_or(&0.0);
                let time_since_activity = self.ui.elapsed_time - last_activity;
                
                let is_blink_visible = if time_since_activity < 0.5 {
                    true // Solid during and just after activity
                } else {
                    // Start blinking after 500ms of idle. 
                    // Offset by last_activity so the cycle always starts "ON" when you stop.
                    (self.ui.elapsed_time - last_activity).fract() < 0.5
                };

                if is_blink_visible {
                    self.ui.draws.push(DrawCommand::OverlayRect(OverlayRectDraw {
                        x: cx,
                        y: cy,
                        width: 2.0,
                        height: cursor_height,
                        color: Color::WHITE,
                        clip: [self.x, self.y, w_box, h_box], 
                    }));
                }
            }
        }

        // --- 7. Render Horizontal Scroll Bar ---
        if self.scrollable && w_text_raw > w_view {
            let is_dragging = self.ui.active_drag.map(|d| d.id == scroll_state_id).unwrap_or(false);
            
            self.ui.draws.push(DrawCommand::OverlayRect(OverlayRectDraw {
                x: thumb_x,
                y: scroll_bar_y,
                width: thumb_w,
                height: scroll_bar_h,
                color: if is_dragging { Color::rgba(1.0, 1.0, 1.0, 0.8) } else { Color::rgba(1.0, 1.0, 1.0, 0.4) },
                clip: [self.x, self.y, w_box, h_box],
            }));
        }

        // --- 8. Semantic Registration ---
        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::TextInput, Rect::new(self.x, self.y, w_box, h_box))
                .with_label(self.buffer.clone())
                .with_focus(is_focused)
        );

        // --- 9. Advance UI ---
        self.ui.advance(w_box, h_box, start_draw);

        if self.render_mode.is_some() {
            self.ui.render_mode_stack.pop();
        }
    }
}
