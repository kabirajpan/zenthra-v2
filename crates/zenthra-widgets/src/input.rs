use crate::ui::{Ui, DrawCommand, CursorDraw};
use crate::text::TextBuilder;
use zenthra_core::{Color, EdgeInsets};
use zenthra_platform::event::PlatformEvent;
use zenthra_text::prelude::TextOptions;

pub struct InputBuilder<'a> {
    ui: &'a mut Ui,
    buffer: &'a mut String,
    id: u64,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
    bg: Option<Color>,
    padding: EdgeInsets,
    line_height: f32,
    min_width: f32,
}

impl<'a> InputBuilder<'a> {
    pub fn new(ui: &'a mut Ui, buffer: &'a mut String, id: u64) -> Self {
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
            padding: EdgeInsets::ZERO,
            line_height: 1.2,
            min_width: 200.0,
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

    pub fn padding(mut self, padding: impl Into<EdgeInsets>) -> Self {
        self.padding = padding.into();
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

    pub fn line_height(mut self, lh: f32) -> Self {
        self.line_height = lh;
        self
    }

    pub fn min_width(mut self, w: f32) -> Self {
        self.min_width = w;
        self
    }

    pub fn show(self) {
        let is_focused = self.ui.focused_id == Some(self.id);
        
        // --- 1. Handle Events (Focused Only) ---
        if is_focused {
            let events = std::mem::take(&mut self.ui.input_events);
            for event in &events {
                match event {
                    PlatformEvent::CharTyped(c) => {
                        self.buffer.push(*c);
                    }
                    PlatformEvent::KeyDown { key } => {
                        if *key == winit::keyboard::KeyCode::Backspace {
                            self.buffer.pop();
                        }
                    }
                    _ => {}
                }
            }
            self.ui.input_events = events;
        }

        // --- 2. Render Text & Get Metrics ---
        let mut text_builder = TextBuilder::new(self.ui, &self.buffer)
            .size(self.font_size)
            .line_height(self.line_height) // Set internal line height
            .color(self.color)
            .full_width_bg(false)
            .padding(self.padding)
            .min_width(self.min_width);
        
        if let Some(bg) = self.bg {
            text_builder = text_builder.bg(bg);
        }
        
        // Build the text without advancing yet
        let (w, h, shaped_buffer, start_draw) = text_builder.draw_and_measure();
        
        // --- 3. Handle Focus (Now we have the real height H) ---
        if self.ui.mouse_down {
            if self.ui.mouse_in_rect(self.x, self.y, w, h) {
                self.ui.focused_id = Some(self.id);
            }
        }

        // --- 4. Cursor Rendering ---
        if is_focused {
            let font_size = self.font_size;
            let lh = self.line_height;
            let box_height = font_size * lh;
            let cursor_height = box_height; // Cursor height matches the text's line height
            
            if self.buffer.is_empty() {
                // For empty buffer, cursor starts after padding.top
                let cx = self.x + self.padding.left;
                let cy = self.y + self.padding.top; 
                self.ui.draws.push(DrawCommand::Cursor(CursorDraw {
                    x: cx,
                    y: cy,
                    height: cursor_height,
                    color: Color::WHITE,
                }));
            } else if let Some(sb) = shaped_buffer {
                // Find horizontal position
                let lx = sb.glyphs()
                    .last()
                    .map(|g| g.x + g.width)
                    .unwrap_or(0.0);
                
                let cx = lx + self.x + self.padding.left;
                let cy = self.y + self.padding.top; // Start after padding.top
                
                self.ui.draws.push(DrawCommand::Cursor(CursorDraw {
                    x: cx,
                    y: cy,
                    height: cursor_height,
                    color: Color::WHITE,
                }));
            }
        }

        // --- 5. Advance UI ---
        self.ui.advance(w, h, start_draw);
    }
}
