use crate::ui::{Ui, DrawCommand, OverlayRectDraw, ScrollDrag};
use crate::text::TextBuilder;
use zenthra_core::{Color, EdgeInsets};
use zenthra_platform::event::PlatformEvent;
use zenthra_text::prelude::{TextOptions, CosmicFontProvider, Padding, ShapedGlyph};
use zenthra_text::traits::FontProvider;

pub struct InputBuilder<'u, 'a, 'b> {
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
    width: Option<f32>,
    min_width: f32,
    scrollable: bool,
    full_width: bool,
    text_bg_full_width: bool,
}

impl<'u, 'a, 'b> InputBuilder<'u, 'a, 'b> {
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
            width: None,
            min_width: 200.0,
            scrollable: true,
            full_width: false,
            text_bg_full_width: false,
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

    pub fn show(self) {
        let is_focused = self.ui.focused_id == Some(self.id);
        
        // --- 1. Initial Measure ---
        let (w_text_raw, h_content, shaped_buffer) = if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
            let t_padding = Padding::from(self.text_padding);
            adapter.set_layout_size(1000000.0, 10000.0); 
            
            let options = TextOptions::new()
                .font_size(self.font_size)
                .line_height(self.line_height)
                .padding(t_padding);
            
            let buffer = adapter.shape(&self.buffer, &options);
            let (cw, _ch) = buffer.content_size();
            let m = adapter.metrics(&options);
            let h_total = m.line_height() + t_padding.vertical();
            
            (cw + t_padding.horizontal(), h_total, Some(buffer))
        } else {
            (self.min_width, 20.0, None)
        };

        let max_available_w = (self.ui.width - self.x).max(self.min_width);
        let w_box = if self.full_width {
            max_available_w
        } else {
            self.width.unwrap_or_else(|| {
                (w_text_raw + self.padding.horizontal()).min(max_available_w)
            }).max(self.min_width)
        };
        let h_box = h_content + self.padding.vertical();
        let w_view = w_box - self.padding.horizontal();

        // --- 2. Calculate Scrollbar Thumb ---
        let scroll_state_id = self.id + 100000;
        let mut scroll_x = *self.ui.scroll_state.get(&scroll_state_id).unwrap_or(&0.0);
        let max_scroll = (w_text_raw - w_view).max(0.0f32);

        let scroll_bar_h = 4.0;
        let scroll_bar_y = self.y + h_box - scroll_bar_h - 2.0;
        let thumb_w = if w_text_raw > w_view { (w_view / w_text_raw) * w_view } else { w_view };
        let thumb_w = thumb_w.max(20.0);
        let scroll_percent = if max_scroll > 0.0 { scroll_x / max_scroll } else { 0.0 };
        let track_width = w_view - thumb_w;
        let thumb_x = self.x + self.padding.left + scroll_percent * track_width;

        // --- 3. Handle Events ---
        let is_hovered = self.ui.mouse_in_rect(self.x, self.y, w_box, h_box);
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

        if is_focused || is_hovered || self.ui.active_drag.is_some() {
            let events = std::mem::take(&mut self.ui.input_events);
            let mut needs_auto_scroll = false;
            for event in &events {
                match event {
                    PlatformEvent::CharTyped(c) if is_focused => {
                         if *c != '\r' && *c != '\n' {
                              self.buffer.push(*c);
                              needs_auto_scroll = true;
                         }
                    }
                    PlatformEvent::KeyDown { key } if is_focused => {
                        if *key == winit::keyboard::KeyCode::Backspace {
                            self.buffer.pop();
                            needs_auto_scroll = true;
                        }
                    }
                    PlatformEvent::MouseWheel { delta_x, delta_y } if is_hovered => {
                        let effective_dx = if delta_x.abs() < 0.001 { *delta_y } else { *delta_x };
                        scroll_x -= effective_dx * 30.0; 
                    }
                    _ => {}
                }
            }
            
            if self.ui.clicked {
                if is_over_thumb {
                    self.ui.active_drag = Some(ScrollDrag {
                        id: scroll_state_id,
                        start_mouse: self.ui.mouse_x,
                        start_scroll: scroll_x,
                    });
                } else if is_hovered {
                    self.ui.focused_id = Some(self.id);
                    needs_auto_scroll = true;
                }
            }
            
            if let Some(drag) = self.ui.active_drag {
                if drag.id == scroll_state_id {
                    // println!("Dragging! mouse: {}, delta: {}", self.ui.mouse_x, self.ui.mouse_x - drag.start_mouse);
                }
            }
            
            // Auto-scroll to cursor after event processing
            let is_dragging_this = self.ui.active_drag.map(|d| d.id == scroll_state_id).unwrap_or(false);
            if is_focused && !is_dragging_this && needs_auto_scroll {
                if let Some(sb) = &shaped_buffer {
                    let lx = sb.glyphs().last().map(|g: &ShapedGlyph| g.x + g.width).unwrap_or(0.0);
                    let cursor_x_with_padding = lx + self.text_padding.left;
                    if cursor_x_with_padding > scroll_x + w_view - 40.0 {
                        scroll_x = cursor_x_with_padding - w_view + 40.0;
                    } else if cursor_x_with_padding < scroll_x + 10.0 {
                        scroll_x = (cursor_x_with_padding - 10.0).max(0.0);
                    }
                }
            }

            scroll_x = scroll_x.clamp(0.0, max_scroll);
            self.ui.scroll_state.insert(scroll_state_id, scroll_x);
            self.ui.input_events = events;
        } else if self.ui.active_drag.is_some() {
            // Also update if we are dragging even if not hovered (drag-out)
            scroll_x = scroll_x.clamp(0.0, max_scroll);
            self.ui.scroll_state.insert(scroll_state_id, scroll_x);
        }

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
                    radius: 4.0,
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
            .padding(self.text_padding)
            .max_width(1000000.0) 
            .pos(self.x + self.padding.left - scroll_x, self.y + self.padding.top)
            .clip_rect(self.x, self.y, w_box, h_box);

        if self.text_bg_full_width {
            text_builder = text_builder.min_width(w_view);
        }
        
        if let Some(tbg) = self.text_bg {
            text_builder = text_builder.bg(tbg).full_width_bg(false);
        }
        
        text_builder.draw_and_measure();
        
        // --- 6. Cursor Rendering ---
        if is_focused {
            let font_size = self.font_size;
            let lh = self.line_height;
            let cursor_height = font_size; 
            let total_line_height = font_size * lh;
            let vertical_offset = (total_line_height - cursor_height) / 2.0;

            if let Some(sb) = shaped_buffer {
                let lx = sb.glyphs().last().map(|g: &ShapedGlyph| g.x + g.width).unwrap_or(0.0);
                let cx = lx + self.x + self.padding.left + self.text_padding.left - scroll_x;
                let cy = self.y + self.padding.top + self.text_padding.top + vertical_offset;
                
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

        // --- 8. Advance UI ---
        self.ui.advance(w_box, h_box, start_draw);
    }
}
