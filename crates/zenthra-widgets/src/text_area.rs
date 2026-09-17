use crate::ui::{Ui, DrawCommand, OverlayRectDraw};
use crate::text::TextBuilder;
use zenthra_core::{Color, EdgeInsets, Id, Rect};
use zenthra_platform::event::PlatformEvent;
use zenthra_text::prelude::{TextOptions, CosmicFontProvider, Padding};
// use zenthra_text::traits::FontProvider;

const CTRL_STATE_KEY: Id = Id::from_u64(0xFEED_C001);
const SHIFT_STATE_KEY: Id = Id::from_u64(0xFEED_581F);
const ALT_STATE_KEY: Id = Id::from_u64(0xFEED_A170);

fn get_clipboard_text() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("WAYLAND_DISPLAY").is_some() {
            if let Ok(output) = std::process::Command::new("wl-paste")
                .args(&["--type", "text/plain;charset=utf-8", "-n"])
                .output()
            {
                if output.status.success() {
                    if let Ok(s) = String::from_utf8(output.stdout) {
                        if !s.is_empty() {
                            return Some(s);
                        }
                    }
                }
            }
            if let Ok(output) = std::process::Command::new("wl-paste")
                .arg("-n")
                .output()
            {
                if output.status.success() {
                    if let Ok(s) = String::from_utf8(output.stdout) {
                        if !s.is_empty() {
                            return Some(s);
                        }
                    }
                }
            }
        }
    }

    if let Ok(mut cb) = arboard::Clipboard::new() {
        if let Ok(text) = cb.get_text() {
            if !text.is_empty() {
                return Some(text);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = std::process::Command::new("xclip")
            .args(&["-selection", "clipboard", "-o"])
            .output()
        {
            if output.status.success() {
                if let Ok(s) = String::from_utf8(output.stdout) {
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
        }
    }

    None
}

fn set_clipboard_text(text: &str) {
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("WAYLAND_DISPLAY").is_some() {
            use std::io::Write;
            if let Ok(mut child) = std::process::Command::new("wl-copy")
                .stdin(std::process::Stdio::piped())
                .spawn()
            {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(text.as_bytes());
                }
                let _ = child.wait();
            }
        }
    }

    if let Ok(mut cb) = arboard::Clipboard::new() {
        let _ = cb.set_text(text);
    }
}

fn find_prev_word_boundary(text: &str, cursor: usize) -> usize {
    let cursor = cursor.min(text.len());
    if cursor == 0 {
        return 0;
    }
    let sub = &text[..cursor];
    let mut chars = sub.char_indices().rev().peekable();
    
    let mut skipped_ws = false;
    let mut target = cursor;
    while let Some(&(_, c)) = chars.peek() {
        if c == '\n' {
            if !skipped_ws && target == cursor {
                return chars.next().unwrap().0;
            }
            break;
        }
        if c.is_whitespace() {
            let (idx, _) = chars.next().unwrap();
            target = idx;
            skipped_ws = true;
        } else {
            break;
        }
    }

    if let Some(&(_, first_c)) = chars.peek() {
        let is_word = first_c.is_alphanumeric() || first_c == '_';
        while let Some(&(idx, c)) = chars.peek() {
            if c == '\n' {
                break;
            }
            let c_is_word = c.is_alphanumeric() || c == '_';
            if c.is_whitespace() || c_is_word != is_word {
                break;
            }
            target = idx;
            chars.next();
        }
    }

    target
}

fn find_next_word_boundary(text: &str, cursor: usize) -> usize {
    let cursor = cursor.min(text.len());
    if cursor >= text.len() {
        return text.len();
    }
    let sub = &text[cursor..];
    let mut chars = sub.char_indices().peekable();

    let mut skipped_ws = false;
    let mut target_rel = 0;
    while let Some(&(_, c)) = chars.peek() {
        if c == '\n' {
            if !skipped_ws && target_rel == 0 {
                chars.next();
                return cursor + chars.peek().map(|(idx, _)| *idx).unwrap_or(sub.len());
            }
            break;
        }
        if c.is_whitespace() {
            chars.next();
            target_rel = chars.peek().map(|(idx, _)| *idx).unwrap_or(sub.len());
            skipped_ws = true;
        } else {
            break;
        }
    }

    if let Some(&(_, first_c)) = chars.peek() {
        let is_word = first_c.is_alphanumeric() || first_c == '_';
        while let Some(&(_idx, c)) = chars.peek() {
            if c == '\n' {
                break;
            }
            let c_is_word = c.is_alphanumeric() || c == '_';
            if c.is_whitespace() || c_is_word != is_word {
                break;
            }
            chars.next();
            target_rel = chars.peek().map(|(next_idx, _)| *next_idx).unwrap_or(sub.len());
        }
    }

    cursor + target_rel
}

fn find_word_start(text: &str, cursor: usize) -> usize {
    let cursor = cursor.min(text.len());
    if cursor == 0 {
        return 0;
    }
    let sub = &text[..cursor];
    let mut target = cursor;
    for (idx, c) in sub.char_indices().rev() {
        if c.is_alphanumeric() || c == '_' {
            target = idx;
        } else {
            break;
        }
    }
    target
}

fn find_word_end(text: &str, cursor: usize) -> usize {
    let cursor = cursor.min(text.len());
    if cursor >= text.len() {
        return text.len();
    }
    let sub = &text[cursor..];
    let mut target_rel = 0;
    for (idx, c) in sub.char_indices() {
        if c.is_alphanumeric() || c == '_' {
            target_rel = idx + c.len_utf8();
        } else {
            break;
        }
    }
    cursor + target_rel
}

fn find_line_start(text: &str, cursor: usize) -> usize {
    let cursor = cursor.min(text.len());
    if cursor == 0 {
        return 0;
    }
    let sub = &text[..cursor];
    if let Some(pos) = sub.rfind('\n') {
        pos + 1
    } else {
        0
    }
}

fn find_line_end(text: &str, cursor: usize) -> usize {
    let cursor = cursor.min(text.len());
    if cursor >= text.len() {
        return text.len();
    }
    let sub = &text[cursor..];
    if let Some(pos) = sub.find('\n') {
        cursor + pos
    } else {
        text.len()
    }
}

pub struct TextAreaBuilder<'u, 'a, 'b> {
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
    width: f32,
    height: Option<f32>,
    max_height: Option<f32>,
    scrollable: bool,
    overflow_hidden: bool,
    text_bg_fill_x: bool,
    fill_x: bool,
    wrap: zenthra_text::prelude::TextWrap,
    radius: [f32; 4],
    border_color: Option<Color>,
    border_width: f32,
    focus_border_color: Option<Color>,
    focus_border_width: Option<f32>,
    shadow_color: Option<Color>,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,
    opacity: f32,
    render_mode: Option<zenthra_core::RenderMode>,
    placeholder: Option<String>,
    placeholder_color: Option<Color>,
    submit_on_enter: bool,
}

impl<'u, 'a, 'b> TextAreaBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, buffer: &'b mut String) -> Self {
        let id = ui.id();
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
            max_height: None,
            scrollable: false,
            overflow_hidden: false,
            text_bg_fill_x: false,
            fill_x: false,
            highlight: None,
            wrap: zenthra_text::prelude::TextWrap::Word,
            radius: [4.0; 4],
            border_color: None,
            border_width: 0.0,
            focus_border_color: None,
            focus_border_width: None,
            shadow_color: None,
            shadow_offset: [0.0; 2],
            shadow_blur: 0.0,
            shadow_opacity: 1.0,
            opacity: 1.0,
            render_mode: None,
            placeholder: None,
            placeholder_color: None,
            submit_on_enter: false,
        }
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
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
        self.width = w;
        self
    }

    pub fn fill_x(mut self) -> Self {
        self.fill_x = true;
        self
    }

    pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.radius = [tl, tr, br, bl];
        self
    }

    pub fn radius_all(mut self, r: f32) -> Self {
        self.radius = [r; 4];
        self
    }

    pub fn radius_top(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self.radius[1] = r;
        self
    }

    pub fn radius_bottom(mut self, r: f32) -> Self {
        self.radius[2] = r;
        self.radius[3] = r;
        self
    }

    pub fn radius_top_left(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self
    }

    pub fn radius_top_right(mut self, r: f32) -> Self {
        self.radius[1] = r;
        self
    }

    pub fn radius_bottom_right(mut self, r: f32) -> Self {
        self.radius[2] = r;
        self
    }

    pub fn radius_bottom_left(mut self, r: f32) -> Self {
        self.radius[3] = r;
        self
    }

    pub fn radius_left(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self.radius[3] = r;
        self
    }

    pub fn radius_right(mut self, r: f32) -> Self {
        self.radius[1] = r;
        self.radius[2] = r;
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self
    }

    pub fn focus_border(mut self, color: Color, width: f32) -> Self {
        self.focus_border_color = Some(color);
        self.focus_border_width = Some(width);
        self
    }

    pub fn shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self {
        self.shadow_color = Some(color);
        self.shadow_offset = [ox, oy];
        self.shadow_blur = blur;
        self
    }

    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = o;
        self
    }

    pub fn text_bg_fill_x(mut self, enabled: bool) -> Self {
        self.text_bg_fill_x = enabled;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    pub fn max_height(mut self, h: f32) -> Self {
        self.max_height = Some(h);
        self
    }

    pub fn submit_on_enter(mut self, enabled: bool) -> Self {
        self.submit_on_enter = enabled;
        self
    }

    pub fn wrap(mut self, strategy: zenthra_text::prelude::TextWrap) -> Self {
        self.wrap = strategy;
        self
    }

    pub fn render_mode(mut self, mode: zenthra_core::RenderMode) -> Self {
        self.render_mode = Some(mode);
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    pub fn placeholder_color(mut self, color: Color) -> Self {
        self.placeholder_color = Some(color);
        self
    }

    pub fn show(self) -> zenthra_core::Response {
        if let Some(mode) = self.render_mode {
            self.ui.render_mode_stack.push(mode);
        }
        let mut is_focused = self.ui.focused_id == Some(self.id);
        
        // --- 1. Handle Scroll State ---
        let (scroll_x, mut scroll_y) = if self.scrollable || self.max_height.is_some() {
            *self.ui.scroll_state.get(&self.id).unwrap_or(&(0.0, 0.0))
        } else {
            (0.0, 0.0)
        };

        // --- 2. Initial Measure (to get content height) ---
        let actual_width = if self.fill_x {
            (self.ui.max_x - self.x).max(self.width)
        } else {
            self.width
        };
        let effective_font_size = self.font_size * self.ui.font_scale;

        let (_content_w, mut h_content, mut shaped_buffer) = if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
            let t_padding = Padding::from(self.text_padding);
            let layout_width = actual_width - self.padding.horizontal() - t_padding.horizontal();
            adapter.set_layout_size(layout_width, 10000.0);
            
            let options = TextOptions::new()
                .font_size(effective_font_size)
                .line_height(self.line_height)
                .wrap(self.wrap)
                .max_width(layout_width);
            
            let buffer_to_shape = if self.buffer.is_empty() {
                self.placeholder.as_deref().unwrap_or("")
            } else {
                &self.buffer
            };
            let buffer = adapter.shape(buffer_to_shape, &options);
            let (_, ch) = buffer.content_size();
            (actual_width, ch + t_padding.vertical(), Some(buffer))
        } else {
            (actual_width, 20.0, None)
        };

        // --- Determine h_box and effective scroll mode ---
        // max_height: auto-grow up to this height, then scroll. Takes priority over fixed height.
        let (mut h_box, effective_scrollable) = if let Some(max_h) = self.max_height {
            let min_h = self.height.unwrap_or(36.0);
            let natural_h = (h_content + self.padding.vertical()).max(min_h);
            if natural_h >= max_h {
                (max_h, true) // content exceeds max: cap and scroll
            } else {
                (natural_h, false) // content fits: natural height, no scroll
            }
        } else if self.scrollable {
            (self.height.unwrap_or(200.0), true)
        } else {
            (h_content + self.padding.vertical(), false)
        };

        if self.scrollable && self.height.unwrap_or(0.0) == 0.0 && self.max_height.is_none() {
            h_box = 200.0;
        }

        // --- 3. Handle Events ---
        let mut cursor_index = *self.ui.cursor_state.get(&self.id).unwrap_or(&self.buffer.len());
        cursor_index = cursor_index.min(self.buffer.len());
        if !self.buffer.is_char_boundary(cursor_index) {
            cursor_index = self.buffer.len(); // Safety
        }

        let selection_id = Id::from_u64(self.id.raw() ^ 0x5E1EC710);
        let mut selection_anchor: Option<usize> = self.ui.cursor_state.get(&selection_id).copied();
        if let Some(anchor) = selection_anchor {
            if anchor > self.buffer.len() || !self.buffer.is_char_boundary(anchor) {
                selection_anchor = None;
            }
        }

        let (actual_x, actual_y) = (self.x + self.ui.offset_x, self.y + self.ui.offset_y);
        let is_hovered = self.ui.is_hovered(self.id, actual_x, actual_y, actual_width, h_box);

        // Click to focus and position cursor / double-click to select word
        let click_id = Id::from_u64(self.id.raw() ^ 0x00C7_1C40);
        if self.ui.clicked && is_hovered {
            self.ui.focused_id = Some(self.id);
            is_focused = true;
            if self.buffer.is_empty() {
                cursor_index = 0;
                selection_anchor = None;
                self.ui.interaction_state.insert(click_id, self.ui.elapsed_time);
                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                self.ui.needs_redraw = true;
            } else if let Some(sb) = &shaped_buffer {
                let rel_x = self.ui.mouse_x - (self.x + self.padding.left + self.text_padding.left);
                let rel_y = if !effective_scrollable && h_box > h_content + self.padding.vertical() {
                    let y_offset = ((h_box - h_content) / 2.0).max(self.padding.top);
                    self.ui.mouse_y - (self.y + y_offset + self.text_padding.top)
                } else {
                    self.ui.mouse_y - (self.y + self.padding.top + self.text_padding.top - scroll_y)
                };

                let row_threshold = (effective_font_size * self.line_height).max(12.0);
                let mut best_line = None;
                let mut min_line_dist = f32::INFINITY;
                for line in sb.lines() {
                    let dist = (line.y - rel_y).abs();
                    if dist < min_line_dist {
                        min_line_dist = dist;
                        best_line = Some(line);
                    }
                }

                if let Some(target_line) = best_line {
                    let mut best_idx = None;
                    let mut best_dist = f32::INFINITY;
                    for g in sb.glyphs() {
                        if (g.y - target_line.y).abs() < row_threshold * 0.6 {
                            let center_x = g.x + g.width * 0.5;
                            let dist = (center_x - rel_x).abs();
                            if dist < best_dist {
                                best_dist = dist;
                                let char_len = self.buffer.get(g.cluster..).and_then(|s| s.chars().next()).map(|c| c.len_utf8()).unwrap_or(0);
                                best_idx = Some(if rel_x < center_x { g.cluster } else { (g.cluster + char_len).min(self.buffer.len()) });
                            }
                        }
                    }

                    if let Some(idx) = best_idx {
                        cursor_index = idx.min(self.buffer.len());
                    } else if rel_x > target_line.width {
                        cursor_index = find_line_end(&self.buffer, target_line.start_cluster);
                    } else {
                        cursor_index = target_line.start_cluster.min(self.buffer.len());
                    }
                }

                let last_click_time = self.ui.interaction_state.get(&click_id).copied().unwrap_or(0.0);
                if self.ui.elapsed_time - last_click_time < 0.35 {
                    let word_start = find_word_start(&self.buffer, cursor_index);
                    let word_end = find_word_end(&self.buffer, cursor_index);
                    if word_start < word_end {
                        selection_anchor = Some(word_start);
                        cursor_index = word_end;
                    } else {
                        selection_anchor = None;
                    }
                } else {
                    selection_anchor = None;
                }
                self.ui.interaction_state.insert(click_id, self.ui.elapsed_time);
                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                self.ui.needs_redraw = true;
            }
        }

        let mut is_ctrl_down = self.ui.interaction_state.get(&CTRL_STATE_KEY).copied().unwrap_or(0.0) > 0.5;
        let mut is_shift_down = self.ui.interaction_state.get(&SHIFT_STATE_KEY).copied().unwrap_or(0.0) > 0.5;
        let mut is_alt_down = self.ui.interaction_state.get(&ALT_STATE_KEY).copied().unwrap_or(0.0) > 0.5;

        let mut submitted = false;
        if is_focused || is_hovered {
            let mut events = std::mem::take(&mut self.ui.input_events);
            let mut changed = false;
            for event in &events {
                let (sel_start, sel_end, has_selection) = if let Some(a) = selection_anchor {
                    if a != cursor_index {
                        let s = a.min(cursor_index).min(self.buffer.len());
                        let e = a.max(cursor_index).min(self.buffer.len());
                        (s, e, s < e)
                    } else {
                        (cursor_index, cursor_index, false)
                    }
                } else {
                    (cursor_index, cursor_index, false)
                };

                match event {
                    PlatformEvent::KeyUp { key } => {
                        match key {
                            winit::keyboard::KeyCode::ControlLeft | winit::keyboard::KeyCode::ControlRight |
                            winit::keyboard::KeyCode::SuperLeft | winit::keyboard::KeyCode::SuperRight => {
                                is_ctrl_down = false;
                                self.ui.interaction_state.insert(CTRL_STATE_KEY, 0.0);
                            }
                            winit::keyboard::KeyCode::ShiftLeft | winit::keyboard::KeyCode::ShiftRight => {
                                is_shift_down = false;
                                self.ui.interaction_state.insert(SHIFT_STATE_KEY, 0.0);
                            }
                            winit::keyboard::KeyCode::AltLeft | winit::keyboard::KeyCode::AltRight => {
                                is_alt_down = false;
                                self.ui.interaction_state.insert(ALT_STATE_KEY, 0.0);
                            }
                            _ => {}
                        }
                    }
                    PlatformEvent::KeyDown { key } if is_focused => {
                        match key {
                            winit::keyboard::KeyCode::ControlLeft | winit::keyboard::KeyCode::ControlRight |
                            winit::keyboard::KeyCode::SuperLeft | winit::keyboard::KeyCode::SuperRight => {
                                is_ctrl_down = true;
                                self.ui.interaction_state.insert(CTRL_STATE_KEY, 1.0);
                            }
                            winit::keyboard::KeyCode::ShiftLeft | winit::keyboard::KeyCode::ShiftRight => {
                                is_shift_down = true;
                                self.ui.interaction_state.insert(SHIFT_STATE_KEY, 1.0);
                            }
                            winit::keyboard::KeyCode::AltLeft | winit::keyboard::KeyCode::AltRight => {
                                is_alt_down = true;
                                self.ui.interaction_state.insert(ALT_STATE_KEY, 1.0);
                            }
                            winit::keyboard::KeyCode::KeyA if is_ctrl_down => {
                                selection_anchor = Some(0);
                                cursor_index = self.buffer.len();
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                self.ui.needs_redraw = true;
                            }
                            winit::keyboard::KeyCode::KeyC if is_ctrl_down => {
                                if has_selection {
                                    set_clipboard_text(&self.buffer[sel_start..sel_end]);
                                } else if !self.buffer.is_empty() {
                                    set_clipboard_text(&self.buffer);
                                }
                            }
                            winit::keyboard::KeyCode::KeyX if is_ctrl_down => {
                                if has_selection {
                                    set_clipboard_text(&self.buffer[sel_start..sel_end]);
                                    self.buffer.drain(sel_start..sel_end);
                                    cursor_index = sel_start;
                                    selection_anchor = None;
                                    changed = true;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                } else if !self.buffer.is_empty() {
                                    set_clipboard_text(&self.buffer);
                                    self.buffer.clear();
                                    cursor_index = 0;
                                    selection_anchor = None;
                                    changed = true;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                }
                            }
                            winit::keyboard::KeyCode::KeyV if is_ctrl_down => {
                                if let Some(text) = get_clipboard_text() {
                                    if !text.is_empty() {
                                        if has_selection {
                                            self.buffer.drain(sel_start..sel_end);
                                            cursor_index = sel_start;
                                            selection_anchor = None;
                                        }
                                        self.buffer.insert_str(cursor_index, &text);
                                        cursor_index += text.len();
                                        changed = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::Backspace => {
                                if has_selection {
                                    self.buffer.drain(sel_start..sel_end);
                                    cursor_index = sel_start;
                                    selection_anchor = None;
                                    changed = true;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                } else if is_ctrl_down || is_alt_down {
                                    let prev_word = find_prev_word_boundary(&self.buffer, cursor_index);
                                    if prev_word < cursor_index {
                                        self.buffer.drain(prev_word..cursor_index);
                                        cursor_index = prev_word;
                                        changed = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                } else if cursor_index > 0 {
                                    let mut chars = self.buffer[..cursor_index].chars();
                                    if let Some(c) = chars.next_back() {
                                        let len = c.len_utf8();
                                        self.buffer.remove(cursor_index - len);
                                        cursor_index -= len;
                                        changed = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::Delete => {
                                if has_selection {
                                    self.buffer.drain(sel_start..sel_end);
                                    cursor_index = sel_start;
                                    selection_anchor = None;
                                    changed = true;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                } else if is_ctrl_down || is_alt_down {
                                    let next_word = find_next_word_boundary(&self.buffer, cursor_index);
                                    if next_word > cursor_index {
                                        self.buffer.drain(cursor_index..next_word);
                                        changed = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                } else if cursor_index < self.buffer.len() {
                                    let next_char_len = self.buffer[cursor_index..].chars().next().map(|c| c.len_utf8()).unwrap_or(0);
                                    if next_char_len > 0 {
                                        self.buffer.drain(cursor_index..cursor_index + next_char_len);
                                        changed = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::ArrowLeft => {
                                if is_shift_down {
                                    if selection_anchor.is_none() {
                                        selection_anchor = Some(cursor_index);
                                    }
                                } else if has_selection {
                                    cursor_index = sel_start;
                                    selection_anchor = None;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    self.ui.needs_redraw = true;
                                }
                                if !has_selection || is_shift_down {
                                    if is_ctrl_down || is_alt_down {
                                        cursor_index = find_prev_word_boundary(&self.buffer, cursor_index);
                                    } else if cursor_index > 0 {
                                        let mut chars = self.buffer[..cursor_index].chars();
                                        if let Some(c) = chars.next_back() {
                                            cursor_index -= c.len_utf8();
                                        }
                                    }
                                    if !is_shift_down {
                                        selection_anchor = None;
                                    }
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    self.ui.needs_redraw = true;
                                }
                            }
                            winit::keyboard::KeyCode::ArrowRight => {
                                if is_shift_down {
                                    if selection_anchor.is_none() {
                                        selection_anchor = Some(cursor_index);
                                    }
                                } else if has_selection {
                                    cursor_index = sel_end;
                                    selection_anchor = None;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    self.ui.needs_redraw = true;
                                }
                                if !has_selection || is_shift_down {
                                    if is_ctrl_down || is_alt_down {
                                        cursor_index = find_next_word_boundary(&self.buffer, cursor_index);
                                    } else if cursor_index < self.buffer.len() {
                                        let mut chars = self.buffer[cursor_index..].chars();
                                        if let Some(c) = chars.next() {
                                            cursor_index += c.len_utf8();
                                        }
                                    }
                                    if !is_shift_down {
                                        selection_anchor = None;
                                    }
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    self.ui.needs_redraw = true;
                                }
                            }
                            winit::keyboard::KeyCode::Home => {
                                if is_shift_down && selection_anchor.is_none() {
                                    selection_anchor = Some(cursor_index);
                                }
                                if is_ctrl_down {
                                    cursor_index = 0;
                                } else {
                                    cursor_index = find_line_start(&self.buffer, cursor_index);
                                }
                                if !is_shift_down {
                                    selection_anchor = None;
                                }
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                self.ui.needs_redraw = true;
                            }
                            winit::keyboard::KeyCode::End => {
                                if is_shift_down && selection_anchor.is_none() {
                                    selection_anchor = Some(cursor_index);
                                }
                                if is_ctrl_down {
                                    cursor_index = self.buffer.len();
                                } else {
                                    cursor_index = find_line_end(&self.buffer, cursor_index);
                                }
                                if !is_shift_down {
                                    selection_anchor = None;
                                }
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                self.ui.needs_redraw = true;
                            }
                            winit::keyboard::KeyCode::PageUp => {
                                if is_shift_down && selection_anchor.is_none() {
                                    selection_anchor = Some(cursor_index);
                                }
                                cursor_index = 0;
                                if !is_shift_down {
                                    selection_anchor = None;
                                }
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                self.ui.needs_redraw = true;
                            }
                            winit::keyboard::KeyCode::PageDown => {
                                if is_shift_down && selection_anchor.is_none() {
                                    selection_anchor = Some(cursor_index);
                                }
                                cursor_index = self.buffer.len();
                                if !is_shift_down {
                                    selection_anchor = None;
                                }
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                self.ui.needs_redraw = true;
                            }
                            winit::keyboard::KeyCode::Escape => {
                                if has_selection {
                                    selection_anchor = None;
                                    self.ui.needs_redraw = true;
                                }
                            }
                            winit::keyboard::KeyCode::ArrowUp => {
                                if is_shift_down {
                                    if selection_anchor.is_none() {
                                        selection_anchor = Some(cursor_index);
                                    }
                                } else {
                                    selection_anchor = None;
                                }
                                if self.buffer.is_empty() {
                                    cursor_index = 0;
                                } else if let Some(sb) = &shaped_buffer {
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    self.ui.needs_redraw = true;
                                    let row_threshold = (self.font_size * self.line_height) * 0.5;

                                    let mut current_line_idx = 0;
                                    for (i, line) in sb.lines().iter().enumerate() {
                                        if line.start_cluster <= cursor_index {
                                            current_line_idx = i;
                                        } else {
                                            break;
                                        }
                                    }

                                    if current_line_idx > 0 {
                                        let target_line_idx = current_line_idx - 1;
                                        let target_line = &sb.lines()[target_line_idx];
                                        
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
                                            cursor_index = target_line.start_cluster;
                                        }
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::ArrowDown => {
                                if is_shift_down {
                                    if selection_anchor.is_none() {
                                        selection_anchor = Some(cursor_index);
                                    }
                                } else {
                                    selection_anchor = None;
                                }
                                if self.buffer.is_empty() {
                                    cursor_index = 0;
                                } else if let Some(sb) = &shaped_buffer {
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    self.ui.needs_redraw = true;
                                    let row_threshold = (self.font_size * self.line_height) * 0.5;

                                    let mut current_line_idx = 0;
                                    for (i, line) in sb.lines().iter().enumerate() {
                                        if line.start_cluster <= cursor_index {
                                            current_line_idx = i;
                                        } else {
                                            break;
                                        }
                                    }

                                    if current_line_idx < sb.lines().len() - 1 {
                                        let target_line_idx = current_line_idx + 1;
                                        let target_line = &sb.lines()[target_line_idx];
                                        
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
                                            cursor_index = target_line.start_cluster;
                                        }
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::Enter | winit::keyboard::KeyCode::NumpadEnter => {
                                if self.submit_on_enter && !is_shift_down {
                                    submitted = true;
                                } else {
                                    if has_selection {
                                        self.buffer.drain(sel_start..sel_end);
                                        cursor_index = sel_start;
                                        selection_anchor = None;
                                    }
                                    self.buffer.insert(cursor_index, '\n');
                                    cursor_index += 1;
                                    changed = true;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                }
                            }
                            _ => {}
                        }
                    }
                    PlatformEvent::KeyDown { key } => {
                        match key {
                            winit::keyboard::KeyCode::ControlLeft | winit::keyboard::KeyCode::ControlRight |
                            winit::keyboard::KeyCode::SuperLeft | winit::keyboard::KeyCode::SuperRight => {
                                is_ctrl_down = true;
                                self.ui.interaction_state.insert(CTRL_STATE_KEY, 1.0);
                            }
                            winit::keyboard::KeyCode::ShiftLeft | winit::keyboard::KeyCode::ShiftRight => {
                                is_shift_down = true;
                                self.ui.interaction_state.insert(SHIFT_STATE_KEY, 1.0);
                            }
                            winit::keyboard::KeyCode::AltLeft | winit::keyboard::KeyCode::AltRight => {
                                is_alt_down = true;
                                self.ui.interaction_state.insert(ALT_STATE_KEY, 1.0);
                            }
                            _ => {}
                        }
                    }
                    PlatformEvent::CharTyped(c) if is_focused => {
                        if !is_ctrl_down && *c != '\r' && *c != '\n' {
                            if has_selection {
                                self.buffer.drain(sel_start..sel_end);
                                cursor_index = sel_start;
                                selection_anchor = None;
                            }
                            self.buffer.insert(cursor_index, *c);
                            cursor_index += c.len_utf8();
                            changed = true;
                            self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                        }
                    }
                    PlatformEvent::MouseWheel { delta_x, delta_y } if effective_scrollable && is_hovered => {
                        let total_delta = if delta_y.abs() > 0.001 { *delta_y } else { *delta_x };
                        let step = if total_delta.abs() <= 5.0 { total_delta * 38.0 } else { total_delta };
                        let usable_h = h_box - self.padding.vertical();
                        let max_scroll = (h_content - usable_h).max(0.0);
                        scroll_y = (scroll_y - step).clamp(0.0, max_scroll);
                        self.ui.needs_redraw = true;
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
                            
                            let options = TextOptions::new().font_size(effective_font_size).line_height(self.line_height).wrap(self.wrap).max_width(layout_width);
                            let buffer_to_shape = if self.buffer.is_empty() {
                                self.placeholder.as_deref().unwrap_or("")
                            } else {
                                &self.buffer
                            };
                            let buffer = adapter.shape(buffer_to_shape, &options);
                            let (_cw, ch) = buffer.content_size();
                            h_content = ch + t_padding.vertical();
                            shaped_buffer = Some(buffer);
                            
                            // Re-calculate h_box immediately if not fixed
                            if !effective_scrollable {
                                let min_h = self.height.unwrap_or(36.0);
                                let natural_h = (h_content + self.padding.vertical()).max(min_h);
                                if let Some(max_h) = self.max_height {
                                    h_box = natural_h.min(max_h);
                                } else {
                                    h_box = natural_h;
                                }
                            }
                        }
                        
                        self.ui.cursor_state.insert(self.id, cursor_index);
                    }
                    
                    if effective_scrollable {
                        let usable_h = h_box - self.padding.vertical();
                        let max_scroll = (h_content - usable_h).max(0.0);
                        
                        // Vertical Auto-scroll
                        if let Some(sb) = &shaped_buffer {
                            let mut ly = 0.0;
                            let mut found = false;
                            for line in sb.lines() {
                                if line.start_cluster <= cursor_index {
                                    ly = line.y;
                                    found = true;
                                } else { break; }
                            }
                            if found {
                                let line_h = self.font_size * self.line_height;
                                let cursor_y_v = ly + self.text_padding.top;
                                
                                if cursor_y_v > scroll_y + usable_h - line_h {
                                    scroll_y = cursor_y_v - usable_h + line_h;
                                } else if cursor_y_v < scroll_y {
                                    scroll_y = cursor_y_v;
                                }
                            }
                        }

                        scroll_y = scroll_y.clamp(0.0, max_scroll);
                    } else {
                        scroll_y = 0.0;
                    }
                    if effective_scrollable && is_hovered {
                        events.retain(|e| !matches!(e, PlatformEvent::MouseWheel { .. }));
                    }
                    self.ui.input_events = events;
                    self.ui.cursor_state.insert(self.id, cursor_index);
                    let selection_id = Id::from_u64(self.id.raw() ^ 0x5E1EC710);
                    if let Some(anchor) = selection_anchor {
                        if anchor != cursor_index {
                            self.ui.cursor_state.insert(selection_id, anchor);
                        } else {
                            self.ui.cursor_state.remove(&selection_id);
                        }
                    } else {
                        self.ui.cursor_state.remove(&selection_id);
                    }
                    self.ui.scroll_state.insert(self.id, (scroll_x, scroll_y));
                }

        // --- 4. Render Background (FIXED) ---
        let start_draw = self.ui.draws.len();
        if let Some(bg) = self.bg {
            use crate::ui::RectDraw;
            use zenthra_render::RectInstance;
            let border_col = if is_focused {
                self.focus_border_color
                    .or(self.border_color)
                    .unwrap_or(if self.border_width > 0.0 { Color::rgba(1.0, 1.0, 1.0, 0.4) } else { Color::TRANSPARENT })
                    .to_array()
            } else {
                self.border_color
                    .unwrap_or(if self.border_width > 0.0 { Color::rgba(1.0, 1.0, 1.0, 0.2) } else { Color::TRANSPARENT })
                    .to_array()
            };
            let border_w = if is_focused {
                self.focus_border_width.unwrap_or(if self.border_width > 0.0 { self.border_width.max(1.0) } else { 0.0 })
            } else {
                self.border_width
            };
            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [self.x, self.y],
                    size: [actual_width, h_box],
                    color: bg.to_array(),
                    radius: self.radius,
                    border_width: border_w,
                    border_color: border_col,
                    shadow_color: self.shadow_color.map(|c| {
                        let mut a = c.to_array();
                        a[3] *= self.shadow_opacity;
                        a
                    }).unwrap_or([0.0, 0.0, 0.0, 0.0]),
                    shadow_offset: self.shadow_offset,
                    shadow_blur: self.shadow_blur,
                    clip_rect: [0.0, 0.0, 9999.0, 9999.0],
                    grayscale: 0.0,
                    brightness: 1.0,
                    opacity: self.opacity,
                    ..Default::default()
                }
            }));
        }

        // --- 5. Render Text (ALWAYS CLIPPED TO BOX) ---
        let pos_y = if !effective_scrollable && h_box > h_content + self.padding.vertical() {
            // When box is taller than content (e.g. min_height > content), center vertically!
            self.y + ((h_box - h_content) / 2.0).max(self.padding.top)
        } else {
            self.y + self.padding.top - scroll_y
        };
        let display_text = if self.buffer.is_empty() {
            self.placeholder.as_deref().unwrap_or("")
        } else {
            &self.buffer
        };
        let display_color = if self.buffer.is_empty() {
            self.placeholder_color.unwrap_or(Color::rgba(1.0, 1.0, 1.0, 0.4))
        } else {
            self.color
        };

        let mut text_builder = TextBuilder::new(self.ui, display_text)
            .size(self.font_size)
            .line_height(self.line_height)
            .color(display_color)
            .fill_x(false)
            .padding(self.text_padding.top, self.text_padding.right, self.text_padding.bottom, self.text_padding.left)
            .wrap(self.wrap)
            .max_width(actual_width - self.padding.horizontal() - self.text_padding.horizontal())
            .pos(self.x + self.padding.left, pos_y);

        if self.text_bg_fill_x {
            text_builder = text_builder.min_width(actual_width - self.padding.horizontal());
        }

        if let Some(tbg) = self.text_bg {
            text_builder = text_builder.bg(tbg).fill_x(false);
        }

        if let Some(h) = self.highlight {
            text_builder = text_builder.highlight(h);
        }

        // Always clip text to the textarea box boundary so text never overflows outside
        text_builder = text_builder.clip_rect(self.x + 1.0, self.y + 1.0, (actual_width - 2.0).max(0.0), (h_box - 2.0).max(0.0));
        
        // Final draw
        text_builder.draw_and_measure();

        // Update persistent scroll state
        if self.scrollable {
            self.ui.scroll_state.insert(self.id, (scroll_x, scroll_y));

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
                is_focused = true;
            }
        }

        // --- 7. Cursor & Selection Rendering ---
        if is_focused {
            self.ui.request_redraw_after(std::time::Duration::from_millis(250));
            let font_size = effective_font_size;
            let lh = self.line_height;
            let visual_ascent = font_size * (0.8 + (lh - 1.0) / 2.0);
            let cursor_height = font_size * lh;

            let (sel_start, sel_end, has_selection) = if let Some(a) = selection_anchor {
                if a != cursor_index {
                    let s = a.min(cursor_index).min(self.buffer.len());
                    let e = a.max(cursor_index).min(self.buffer.len());
                    (s, e, s < e)
                } else {
                    (cursor_index, cursor_index, false)
                }
            } else {
                (cursor_index, cursor_index, false)
            };

            if let Some(sb) = shaped_buffer {
                let first_line_y = sb.lines().first().map(|l| l.y).unwrap_or(visual_ascent);
                let v_shift = visual_ascent - first_line_y;

                // Selection highlight
                if has_selection && !self.buffer.is_empty() {
                    let sel_color = Color::rgba(0.26, 0.52, 0.96, 0.35);
                    let row_threshold = (font_size * lh) * 0.6;
                    for line in sb.lines() {
                        let line_cy = if !effective_scrollable && h_box > h_content + self.padding.vertical() {
                            let y_offset = ((h_box - h_content) / 2.0).max(self.padding.top);
                            line.y + self.y + y_offset + self.text_padding.top + v_shift - visual_ascent
                        } else {
                            line.y + self.y + self.padding.top + self.text_padding.top + v_shift - visual_ascent - scroll_y
                        };

                        if line_cy + cursor_height < self.y - 4.0 || line_cy > self.y + h_box + 4.0 {
                            continue;
                        }

                        let mut line_min_x: Option<f32> = None;
                        let mut line_max_x: Option<f32> = None;

                        for g in sb.glyphs() {
                            if (g.y - line.y).abs() < row_threshold {
                                if g.cluster >= sel_start && g.cluster < sel_end {
                                    let left = g.x;
                                    let right = g.x + g.width;
                                    line_min_x = Some(line_min_x.map_or(left, |m: f32| m.min(left)));
                                    line_max_x = Some(line_max_x.map_or(right, |m: f32| m.max(right)));
                                }
                            }
                        }

                        if line_min_x.is_none() && line.start_cluster >= sel_start && line.start_cluster < sel_end {
                            line_min_x = Some(0.0);
                            line_max_x = Some(font_size * 0.4);
                        }

                        if let (Some(min_x), Some(max_x)) = (line_min_x, line_max_x) {
                            let sel_x = self.x + self.padding.left + self.text_padding.left + min_x;
                            let sel_w = (max_x - min_x).max(3.0);
                            self.ui.draws.push(DrawCommand::OverlayRect(OverlayRectDraw {
                                x: sel_x,
                                y: line_cy,
                                width: sel_w,
                                height: cursor_height,
                                color: sel_color,
                                clip: [self.x + 1.0, self.y + 1.0, (actual_width - 2.0).max(0.0), (h_box - 2.0).max(0.0)],
                            }));
                        }
                    }
                }

                let _final_border_w = self.border_width;
                let _final_border_c = self.border_color.unwrap_or(Color::rgba(1.0, 1.0, 1.0, 0.4));
                let mut lx = 0.0;
                let mut ly = first_line_y;
                let mut found = false;

                if self.buffer.is_empty() {
                    lx = 0.0;
                    ly = first_line_y;
                } else {
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
                }
                
                let cx = lx + self.x + self.padding.left + self.text_padding.left;
                let cy = if !effective_scrollable && h_box > h_content + self.padding.vertical() {
                    let y_offset = ((h_box - h_content) / 2.0).max(self.padding.top);
                    ly + self.y + y_offset + self.text_padding.top + v_shift - visual_ascent
                } else {
                    ly + self.y + self.padding.top + self.text_padding.top + v_shift - visual_ascent - scroll_y
                };
                
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
                 
                 // Only draw cursor if it's within the viewport and in the visible blink phase
                 if cy >= self.y - 2.0 && cy + cursor_height <= self.y + h_box + 2.0 && is_blink_visible {
                    self.ui.draws.push(DrawCommand::OverlayRect(OverlayRectDraw {
                        x: cx,
                        y: cy,
                        width: 2.0,
                        height: cursor_height,
                        color: Color::WHITE,
                        clip: [self.x + 1.0, self.y + 1.0, (actual_width - 2.0).max(0.0), (h_box - 2.0).max(0.0)],
                    }));
                }
            }
        }

        // --- 8. Advance UI ---
        self.ui.record_layout(self.id, Rect::new(self.x, self.y, actual_width, h_box));
        self.ui.advance(actual_width, h_box, start_draw);

        if self.render_mode.is_some() {
            self.ui.render_mode_stack.pop();
        }

        zenthra_core::Response {
            clicked: self.ui.clicked && is_hovered,
            hovered: is_hovered,
            pressed: is_hovered && self.ui.mouse_down,
            submitted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_prev_word_boundary() {
        let s = "hello world, test";
        assert_eq!(find_prev_word_boundary(s, 17), 13); // from end of "test" to start of "test"
        assert_eq!(find_prev_word_boundary(s, 13), 11); // from start of "test" to start of ", "
        assert_eq!(find_prev_word_boundary(s, 11), 6);  // from "," to start of "world"
        assert_eq!(find_prev_word_boundary(s, 6), 0);   // from "world" to start of "hello"
        assert_eq!(find_prev_word_boundary(s, 0), 0);   // start of string stays 0
    }

    #[test]
    fn test_find_next_word_boundary() {
        let s = "hello world, test";
        assert_eq!(find_next_word_boundary(s, 0), 5);   // from start of "hello" to end of "hello"
        assert_eq!(find_next_word_boundary(s, 5), 11);  // from end of "hello" to end of "world"
        assert_eq!(find_next_word_boundary(s, 11), 12); // from end of "world" to end of ","
        assert_eq!(find_next_word_boundary(s, 12), 17); // from "," to end of "test"
        assert_eq!(find_next_word_boundary(s, 17), 17); // end of string stays 17
    }

    #[test]
    fn test_multibyte_utf8_word_boundary() {
        let s = "🦀 crab 🚀 rocket";
        assert_eq!(find_prev_word_boundary(s, s.len()), 15); // "rocket" starts at byte 15
        assert_eq!(find_next_word_boundary(s, 0), 4);        // first emoji (4 bytes)
    }

    #[test]
    fn test_find_line_start_and_end() {
        let s = "line 1\nline 2 longer\nline 3";
        // line 2 starts at index 7 ("line 2...") and ends at index 20
        assert_eq!(find_line_start(s, 10), 7);
        assert_eq!(find_line_end(s, 10), 20);
        assert_eq!(find_line_start(s, 0), 0);
        assert_eq!(find_line_end(s, 22), s.len());
    }

    #[test]
    fn test_word_by_word_delete_backward() {
        let mut buf = String::from("hello world test");
        let mut cur = buf.len();

        // 1st word delete: "test" -> "hello world "
        let prev = find_prev_word_boundary(&buf, cur);
        buf.drain(prev..cur);
        cur = prev;
        assert_eq!(buf, "hello world ");

        // 2nd word delete: "world " -> "hello "
        let prev = find_prev_word_boundary(&buf, cur);
        buf.drain(prev..cur);
        cur = prev;
        assert_eq!(buf, "hello ");

        // 3rd word delete: "hello " -> ""
        let prev = find_prev_word_boundary(&buf, cur);
        buf.drain(prev..cur);
        cur = prev;
        assert_eq!(buf, "");
        assert_eq!(cur, 0);
    }

    #[test]
    fn test_word_by_word_delete_forward() {
        let mut buf = String::from("hello world test");
        let cur = 0;

        // 1st forward word delete: "hello" -> " world test"
        let next = find_next_word_boundary(&buf, cur);
        buf.drain(cur..next);
        assert_eq!(buf, " world test");

        // 2nd forward word delete: " world" -> " test"
        let next = find_next_word_boundary(&buf, cur);
        buf.drain(cur..next);
        assert_eq!(buf, " test");

        // 3rd forward word delete: " test" -> ""
        let next = find_next_word_boundary(&buf, cur);
        buf.drain(cur..next);
        assert_eq!(buf, "");
    }

    #[test]
    fn test_reverse_delete_letter() {
        let mut buf = String::from("hello");
        let cur = 0;
        let char_len = buf[cur..].chars().next().map(|c| c.len_utf8()).unwrap();
        buf.drain(cur..cur + char_len);
        assert_eq!(buf, "ello");
    }
}

