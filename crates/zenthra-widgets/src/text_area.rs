use crate::ui::{Ui, DrawCommand, OverlayRectDraw};
use crate::text::TextBuilder;
use crate::style_ext::BoxStyleExt;
use zenthra_core::{BoxStyle, Color, EdgeInsets, Id, IntoBorder, Rect, Styleable};
use zenthra_platform::event::PlatformEvent;
use zenthra_text::prelude::{TextOptions, CosmicFontProvider};
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

fn find_word_bounds(text: &str, cursor: usize) -> (usize, usize) {
    if text.is_empty() {
        return (0, 0);
    }
    let cursor = cursor.min(text.len());
    
    let char_at = text[cursor..].chars().next();
    let char_before = if cursor > 0 {
        text[..cursor].chars().next_back()
    } else {
        None
    };

    let target_char = match (char_at, char_before) {
        (Some(c), _) if c != '\n' => c,
        (_, Some(c)) if c != '\n' => c,
        (Some(c), _) => c,
        _ => return (cursor, cursor),
    };

    if target_char == '\n' {
        return (cursor, cursor);
    }

    let is_word = target_char.is_alphanumeric() || target_char == '_';
    let is_ws = target_char.is_whitespace();

    let mut start = cursor;
    if let Some(c) = char_at {
        if (is_word && (c.is_alphanumeric() || c == '_')) || (is_ws && c.is_whitespace() && c != '\n') {
            // on matching char
        } else if cursor > 0 {
            start = cursor - char_before.map(|c| c.len_utf8()).unwrap_or(0);
        }
    } else if cursor > 0 {
        start = cursor - char_before.map(|c| c.len_utf8()).unwrap_or(0);
    }

    for (idx, c) in text[..start].char_indices().rev() {
        if c == '\n' {
            break;
        }
        if is_word && (c.is_alphanumeric() || c == '_') {
            start = idx;
        } else if is_ws && c.is_whitespace() {
            start = idx;
        } else {
            break;
        }
    }

    let mut end = start;
    for (idx, c) in text[start..].char_indices() {
        if c == '\n' {
            break;
        }
        if is_word && (c.is_alphanumeric() || c == '_') {
            end = start + idx + c.len_utf8();
        } else if is_ws && c.is_whitespace() {
            end = start + idx + c.len_utf8();
        } else {
            break;
        }
    }

    (start, end)
}

fn hit_test_cursor(
    sb: &zenthra_text::primitives::shaped_buffer::ShapedBuffer,
    buffer: &str,
    rel_x: f32,
    rel_y: f32,
    font_size: f32,
    line_height: f32,
) -> usize {
    if buffer.is_empty() || sb.lines().is_empty() {
        return 0;
    }

    let line_height_px = if line_height > 3.0 {
        line_height
    } else {
        font_size * line_height
    };

    let first_line_y = sb.lines().first().map(|l| l.y).unwrap_or(0.0);

    // Mouse above all lines -> clamp to line 0
    let target_line_idx = if rel_y <= 0.0 {
        0
    } else {
        let last_line = sb.lines().last().unwrap();
        let last_line_bottom = (last_line.y - first_line_y) + line_height_px;
        // Mouse below all lines -> clamp to end of text
        if rel_y >= last_line_bottom {
            return buffer.len();
        }

        let mut found = sb.lines().len() - 1;
        for (i, line) in sb.lines().iter().enumerate() {
            let line_top = line.y - first_line_y;
            let line_bottom = line_top + line_height_px;
            if rel_y < line_bottom {
                found = i;
                break;
            }
        }
        found
    };

    let target_line = &sb.lines()[target_line_idx];
    let line_start = target_line.start_cluster.min(buffer.len());

    let line_end = if target_line_idx + 1 < sb.lines().len() {
        let mut next_start = sb.lines()[target_line_idx + 1].start_cluster.min(buffer.len());
        if next_start > line_start && buffer.as_bytes().get(next_start - 1) == Some(&b'\n') {
            next_start -= 1;
            if next_start > line_start && buffer.as_bytes().get(next_start - 1) == Some(&b'\r') {
                next_start -= 1;
            }
        }
        next_start
    } else {
        buffer.len()
    };

    if line_start >= line_end {
        return line_start;
    }

    if rel_x <= 0.0 {
        return line_start;
    }
    if rel_x >= target_line.width {
        return line_end;
    }

    let mut best_idx = None;
    let mut best_dist = f32::INFINITY;

    for g in sb.glyphs() {
        if g.cluster >= line_start && g.cluster < line_end {
            let center_x = g.x + g.width * 0.5;
            let dist = (center_x - rel_x).abs();
            if dist < best_dist {
                best_dist = dist;
                let char_len = buffer
                    .get(g.cluster..)
                    .and_then(|s| s.chars().next())
                    .map(|c| c.len_utf8())
                    .unwrap_or(0);
                best_idx = Some(if rel_x < center_x {
                    g.cluster
                } else {
                    (g.cluster + char_len).min(line_end)
                });
            }
        }
    }

    best_idx.unwrap_or_else(|| {
        if rel_x < target_line.width * 0.5 {
            line_start
        } else {
            line_end
        }
    })
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

    // Box & typography styling
    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,

    // TextArea-specific fields
    cursor_color: Option<Color>,
    pub selection_color: Option<Color>,
    scrollable: bool,
    focus_border_color: Option<Color>,
    focus_border_width: Option<f32>,
    placeholder: Option<String>,
    placeholder_color: Option<Color>,
    submit_on_enter: bool,
    render_mode: Option<zenthra_core::RenderMode>,
    select_all: bool,
}

impl<'u, 'a, 'b> Styleable for TextAreaBuilder<'u, 'a, 'b> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

// Single-source macro generates all BoxStyle builder methods (.width, .height, .padding, .radius, .bg, .border, .shadow, etc.)
impl_box_style_methods!(TextAreaBuilder, 'u, 'a, 'b);

// Single-source macro generates typography styling methods (.font_size, .text_color, .font_family, .font_weight, .bold, .italic, .line_height, etc.)
impl_font_style_methods!(TextAreaBuilder, 'u, 'a, 'b);

// Single-source macro generates text wrap methods (.wrap, .auto_wrap, .no_wrap)
impl_text_wrap_methods!(TextAreaBuilder, 'u, 'a, 'b);

impl<'u, 'a, 'b> TextAreaBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, buffer: &'b mut String) -> Self {
        let id = ui.id();
        let x = ui.cursor_x;
        let y = ui.cursor_y;
        let mut style = BoxStyle::new();
        style.width = Some(300.0);
        style.bg = Some(zenthra_core::Background::Solid(Color::rgb(0.2, 0.2, 0.2)));
        style.radius = [4.0; 4];
        style.padding = EdgeInsets::symmetric(6.0, 8.0);

        let mut text_style = zenthra_text::prelude::TextStyle::default();
        text_style.font_size = 18.0;
        text_style.color = Color::WHITE;
        text_style.line_height = 1.2;
        text_style.wrap = zenthra_text::prelude::TextWrap::Word;

        Self {
            ui,
            buffer,
            id,
            x,
            y,
            style,
            text_style,
            cursor_color: None,
            selection_color: None,
            scrollable: false,
            focus_border_color: None,
            focus_border_width: None,
            placeholder: None,
            placeholder_color: None,
            submit_on_enter: false,
            render_mode: None,
            select_all: false,
        }
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn scrollable(mut self, enabled: bool) -> Self {
        self.scrollable = enabled;
        self
    }

    pub fn cursor_color(mut self, color: Color) -> Self {
        self.cursor_color = Some(color);
        self
    }

    pub fn selection_color(mut self, color: Color) -> Self {
        self.selection_color = Some(color);
        self
    }

    pub fn focus_border(mut self, border: impl IntoBorder) -> Self {
        let (b, _align) = border.into_border();
        match b {
            zenthra_core::Border::None => {
                self.focus_border_color = Some(Color::TRANSPARENT);
                self.focus_border_width = Some(0.0);
            }
            zenthra_core::Border::Solid(c, w) => {
                self.focus_border_color = Some(c);
                self.focus_border_width = Some(w);
            }
        }
        self
    }

    pub fn submit_on_enter(mut self, enabled: bool) -> Self {
        self.submit_on_enter = enabled;
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

    pub fn on_change<F>(self, mut f: F) -> Self
    where
        F: FnMut(String) + 'a,
    {
        self.ui.add_listener(self.id, crate::ui::EventPhase::Bubble, move |_, event| {
            if let crate::ui::WidgetEvent::Change(crate::ui::EventValue::String(val)) = event {
                f(val.clone());
            }
        });
        self
    }

    pub fn on_hover<F>(self, mut f: F) -> Self
    where
        F: FnMut(bool) + 'a,
    {
        self.ui.add_listener(self.id, crate::ui::EventPhase::Bubble, move |_, event| {
            if let crate::ui::WidgetEvent::Hover(hovered) = event {
                f(*hovered);
            }
        });
        self
    }

    pub fn on_event<F>(self, phase: crate::ui::EventPhase, f: F) -> Self
    where
        F: FnMut(&mut crate::ui::EventContext, &crate::ui::WidgetEvent) + 'a,
    {
        self.ui.add_listener(self.id, phase, f);
        self
    }

    pub fn autofocus(self, enabled: bool) -> Self {
        let autofocus_key = Id::from_u64(self.id.raw() ^ 0xAF0C_0500);
        if enabled && !self.ui.interaction_state.contains_key(&autofocus_key) {
            self.ui.interaction_state.insert(autofocus_key, 1.0);
            self.ui.focused_id = Some(self.id);
        }
        self
    }

    pub fn select_all(mut self, enabled: bool) -> Self {
        self.select_all = enabled;
        self
    }

    pub fn show(mut self) -> zenthra_core::Response {
        if let Some(mode) = self.render_mode {
            self.ui.render_mode_stack.push(mode);
        }
        self.x += self.style.margin.left;
        self.y += self.style.margin.top;
        let mut is_focused = self.ui.focused_id == Some(self.id);

        let select_all_key = Id::from_u64(self.id.raw() ^ 0x5E1E_A110);
        if self.select_all && !self.ui.interaction_state.contains_key(&select_all_key) {
            self.ui.interaction_state.insert(select_all_key, 1.0);
            self.ui.focused_id = Some(self.id);
            is_focused = true;
            let sel_anchor_id = Id::from_u64(self.id.raw() ^ 0x5E1EC710);
            self.ui.cursor_state.insert(sel_anchor_id, 0);
            self.ui.cursor_state.insert(self.id, self.buffer.len());
        }
        
        // --- 1. Handle Scroll State ---
        let (scroll_x, mut scroll_y) = if self.scrollable || self.style.height.is_some() || self.style.max_height.is_some() {
            *self.ui.scroll_state.get(&self.id).unwrap_or(&(0.0, 0.0))
        } else {
            (0.0, 0.0)
        };

        // --- 2. Initial Measure (to get content height) ---
        let actual_width = if self.style.fill_x {
            (self.ui.max_x - self.x - self.style.margin.right).max(self.style.width.unwrap_or(300.0))
        } else {
            self.style.width.unwrap_or(300.0)
        };
        let effective_font_size = self.text_style.font_size * self.ui.font_scale;

        let (_content_w, mut h_content, mut shaped_buffer) = if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
            let layout_width = actual_width - self.style.padding.horizontal();
            adapter.set_layout_size(layout_width, 10000.0);
            
            let mut options = TextOptions::new()
                .size(effective_font_size)
                .font_weight(self.text_style.font_weight)
                .wrap(self.text_style.wrap)
                .max_width(layout_width);
            options.font_style = self.text_style.font_style;
            options.line_height = self.text_style.line_height;
            if let Some(ref family) = self.text_style.font_family {
                options = options.font_family(family.clone());
            }
            
            let buffer_to_shape = if self.buffer.is_empty() {
                self.placeholder.as_deref().unwrap_or("")
            } else {
                &self.buffer
            };
            let buffer = adapter.shape(buffer_to_shape, &options);
            let (_, ch) = buffer.content_size();
            (actual_width, ch, Some(buffer))
        } else {
            (actual_width, 20.0, None)
        };

        // --- Determine h_box and effective scroll mode ---
        // max_height: auto-grow up to this height, then scroll. Takes priority over fixed height.
        let (mut h_box, effective_scrollable) = if let Some(max_h) = self.style.max_height {
            let min_h = self.style.height.unwrap_or(36.0);
            let natural_h = (h_content + self.style.padding.vertical()).max(min_h);
            if natural_h >= max_h {
                (max_h, true) // content exceeds max: cap and scroll
            } else {
                (natural_h, false) // content fits: natural height, no scroll
            }
        } else if let Some(explicit_h) = self.style.height {
            (explicit_h, true)
        } else if self.scrollable {
            (200.0, true)
        } else {
            (h_content + self.style.padding.vertical(), false)
        };

        if self.style.fill_y {
            h_box = (self.ui.max_y - self.y - self.style.margin.bottom).max(h_box);
        }

        // --- 3. Handle Events ---
        let mut cursor_index = *self.ui.cursor_state.get(&self.id).unwrap_or(&self.buffer.len());
        cursor_index = cursor_index.min(self.buffer.len());
        if !self.buffer.is_char_boundary(cursor_index) {
            cursor_index = self.buffer.len(); // Safety
        }
        let initial_cursor_index = cursor_index;
        let is_multiline = self.buffer.contains('\n') || h_content > self.text_style.font_size * self.text_style.line_height * 1.5;

        let selection_id = Id::from_u64(self.id.raw() ^ 0x5E1EC710);
        let mut selection_anchor: Option<usize> = self.ui.cursor_state.get(&selection_id).copied();
        if let Some(anchor) = selection_anchor {
            if anchor > self.buffer.len() || !self.buffer.is_char_boundary(anchor) {
                selection_anchor = None;
            }
        }

        let (actual_x, actual_y) = (self.x + self.ui.offset_x, self.y + self.ui.offset_y);
        let is_hovered = self.ui.is_hovered(self.id, actual_x, actual_y, actual_width, h_box);

        // Click to focus and position cursor / double-click to select word / mouse drag selection
        let click_id = Id::from_u64(self.id.raw() ^ 0x00C7_1C40);
        let drag_id = Id::from_u64(self.id.raw() ^ 0x00D7_A600);

        if self.buffer.is_empty() {
            if self.ui.clicked && is_hovered {
                self.ui.focused_id = Some(self.id);
                is_focused = true;
                cursor_index = 0;
                selection_anchor = None;
                self.ui.interaction_state.insert(click_id, self.ui.elapsed_time);
                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                self.ui.needs_redraw = true;
            }
        } else if let Some(sb) = &shaped_buffer {
            let rel_x = self.ui.mouse_x - (actual_x + self.style.padding.left);
            let rel_y = if !is_multiline && !effective_scrollable && h_box > h_content + self.style.padding.vertical() {
                let y_offset = ((h_box - h_content) / 2.0).max(self.style.padding.top);
                self.ui.mouse_y - (actual_y + y_offset)
            } else {
                self.ui.mouse_y - (actual_y + self.style.padding.top - scroll_y)
            };

            if self.ui.clicked && is_hovered {
                self.ui.focused_id = Some(self.id);
                is_focused = true;
                self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Click);
                let hit_pos = hit_test_cursor(
                    sb,
                    &self.buffer,
                    rel_x,
                    rel_y,
                    effective_font_size,
                    self.text_style.line_height,
                );

                let last_click_time = self.ui.interaction_state.get(&click_id).copied().unwrap_or(0.0);
                if self.ui.elapsed_time - last_click_time < 0.35 {
                    // Double Click: select word
                    let (word_start, word_end) = find_word_bounds(&self.buffer, hit_pos);
                    if word_start < word_end {
                        selection_anchor = Some(word_start);
                        cursor_index = word_end;
                    } else {
                        selection_anchor = None;
                        cursor_index = hit_pos;
                    }
                    self.ui.interaction_state.insert(drag_id, 2.0); // 2.0 = word drag mode
                } else {
                    // Single Click
                    cursor_index = hit_pos;
                    selection_anchor = Some(hit_pos);
                    self.ui.interaction_state.insert(drag_id, 1.0); // 1.0 = char drag mode
                }

                self.ui.interaction_state.insert(click_id, self.ui.elapsed_time);
                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                self.ui.needs_redraw = true;
            } else if self.ui.mouse_down && is_focused {
                // Active mouse drag selection
                if let Some(&drag_mode) = self.ui.interaction_state.get(&drag_id) {
                    let hit_pos = hit_test_cursor(
                        sb,
                        &self.buffer,
                        rel_x,
                        rel_y,
                        effective_font_size,
                        self.text_style.line_height,
                    );

                    if drag_mode > 1.5 {
                        let (w_start, w_end) = find_word_bounds(&self.buffer, hit_pos);
                        let anchor = selection_anchor.unwrap_or(hit_pos);
                        if hit_pos < anchor {
                            cursor_index = w_start;
                        } else {
                            cursor_index = w_end;
                        }
                    } else {
                        cursor_index = hit_pos;
                    }

                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                    self.ui.needs_redraw = true;
                }
            } else {
                // Mouse released: clean up drag state
                if self.ui.interaction_state.remove(&drag_id).is_some() {
                    if let Some(anchor) = selection_anchor {
                        if anchor == cursor_index {
                            selection_anchor = None;
                        }
                    }
                }
            }
        }

        let mut is_ctrl_down = self.ui.interaction_state.get(&CTRL_STATE_KEY).copied().unwrap_or(0.0) > 0.5;
        let mut is_shift_down = self.ui.interaction_state.get(&SHIFT_STATE_KEY).copied().unwrap_or(0.0) > 0.5;
        let mut is_alt_down = self.ui.interaction_state.get(&ALT_STATE_KEY).copied().unwrap_or(0.0) > 0.5;

        let mut submitted = false;
        let mut changed = false;
        if is_focused || is_hovered {
            let mut events = std::mem::take(&mut self.ui.input_events);
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
                                        let target_line_start = target_line.start_cluster.min(self.buffer.len());
                                        let target_line_end = sb.lines()[current_line_idx].start_cluster.min(self.buffer.len());
                                        
                                        let current_line = &sb.lines()[current_line_idx];
                                        let current_line_start = current_line.start_cluster.min(self.buffer.len());
                                        let current_line_end = if current_line_idx + 1 < sb.lines().len() {
                                            sb.lines()[current_line_idx + 1].start_cluster.min(self.buffer.len())
                                        } else {
                                            self.buffer.len()
                                        };

                                        let mut target_x = 0.0;
                                        for g in sb.glyphs() {
                                            if g.cluster >= current_line_start && g.cluster < current_line_end {
                                                if g.cluster < cursor_index {
                                                    target_x = g.x + g.width;
                                                } else if g.cluster == cursor_index {
                                                    target_x = g.x;
                                                    break;
                                                }
                                            }
                                        }

                                        let mut best_idx = None;
                                        let mut best_dist = f32::INFINITY;
                                        for g in sb.glyphs() {
                                            if g.cluster >= target_line_start && g.cluster < target_line_end {
                                                let center_x = g.x + g.width * 0.5;
                                                let dist = (center_x - target_x).abs();
                                                if dist < best_dist {
                                                    best_dist = dist;
                                                    let char_len = self.buffer.get(g.cluster..).and_then(|s| s.chars().next()).map(|c| c.len_utf8()).unwrap_or(0);
                                                    best_idx = Some(if target_x < center_x { g.cluster } else { (g.cluster + char_len).min(target_line_end) });
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
                                        let target_line_start = target_line.start_cluster.min(self.buffer.len());
                                        let target_line_end = if target_line_idx + 1 < sb.lines().len() {
                                            sb.lines()[target_line_idx + 1].start_cluster.min(self.buffer.len())
                                        } else {
                                            self.buffer.len()
                                        };

                                        let current_line = &sb.lines()[current_line_idx];
                                        let current_line_start = current_line.start_cluster.min(self.buffer.len());
                                        let current_line_end = target_line_start;

                                        let mut target_x = 0.0;
                                        for g in sb.glyphs() {
                                            if g.cluster >= current_line_start && g.cluster < current_line_end {
                                                if g.cluster < cursor_index {
                                                    target_x = g.x + g.width;
                                                } else if g.cluster == cursor_index {
                                                    target_x = g.x;
                                                    break;
                                                }
                                            }
                                        }

                                        let mut best_idx = None;
                                        let mut best_dist = f32::INFINITY;
                                        for g in sb.glyphs() {
                                            if g.cluster >= target_line_start && g.cluster < target_line_end {
                                                let center_x = g.x + g.width * 0.5;
                                                let dist = (center_x - target_x).abs();
                                                if dist < best_dist {
                                                    best_dist = dist;
                                                    let char_len = self.buffer.get(g.cluster..).and_then(|s| s.chars().next()).map(|c| c.len_utf8()).unwrap_or(0);
                                                    best_idx = Some(if target_x < center_x { g.cluster } else { (g.cluster + char_len).min(target_line_end) });
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
                        let usable_h = h_box - self.style.padding.vertical();
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
                            let layout_width = actual_width - self.style.padding.horizontal();
                            adapter.set_layout_size(layout_width, 10000.0);
                            
                            let mut options = TextOptions::new()
                                .size(effective_font_size)
                                .font_weight(self.text_style.font_weight)
                                .wrap(self.text_style.wrap)
                                .max_width(layout_width);
                            options.font_style = self.text_style.font_style;
                            options.line_height = self.text_style.line_height;
                            if let Some(ref family) = self.text_style.font_family {
                                options = options.font_family(family.clone());
                            }
                            let buffer_to_shape = if self.buffer.is_empty() {
                                self.placeholder.as_deref().unwrap_or("")
                            } else {
                                &self.buffer
                            };
                            let buffer = adapter.shape(buffer_to_shape, &options);
                            let (_cw, ch) = buffer.content_size();
                            h_content = ch;
                            shaped_buffer = Some(buffer);
                            
                            // Re-calculate h_box immediately if not fixed
                            if !effective_scrollable {
                                let min_h = self.style.height.unwrap_or(36.0);
                                let natural_h = (h_content + self.style.padding.vertical()).max(min_h);
                                if let Some(max_h) = self.style.max_height {
                                    h_box = natural_h.min(max_h);
                                } else {
                                    h_box = natural_h;
                                }
                            }
                        }
                        
                        self.ui.cursor_state.insert(self.id, cursor_index);
                    }
                    
                    let cursor_changed = changed || cursor_index != initial_cursor_index;
                    if effective_scrollable {
                        let usable_h = h_box - self.style.padding.vertical();
                        let max_scroll = (h_content - usable_h).max(0.0);
                        
                        // Vertical Auto-scroll: only when typing or moving cursor!
                        if cursor_changed {
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
                                    let line_h = self.text_style.font_size * self.text_style.line_height;
                                    let first_l_y = sb.lines().first().map(|l| l.y).unwrap_or(0.0);
                                    let cursor_y_v = ly - first_l_y;
                                    
                                    if cursor_y_v > scroll_y + usable_h - line_h {
                                        scroll_y = cursor_y_v - usable_h + line_h;
                                    } else if cursor_y_v < scroll_y {
                                        scroll_y = cursor_y_v;
                                    }
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
                        self.ui.cursor_state.insert(selection_id, anchor);
                    } else {
                        self.ui.cursor_state.remove(&selection_id);
                    }
                    self.ui.scroll_state.insert(self.id, (scroll_x, scroll_y));
                }

        // --- 4. Render Background & Border ---
        let start_draw = self.ui.draws.len();
        use crate::ui::RectDraw;
        let effective_border_w = if is_focused {
            self.focus_border_width.unwrap_or_else(|| self.style.border_width().max(1.0))
        } else {
            self.style.border_width()
        };
        let effective_border_c = if is_focused {
            self.focus_border_color
                .or_else(|| self.style.border_color())
                .unwrap_or(Color::rgb(0.4, 0.7, 1.0))
        } else {
            self.style.border_color().unwrap_or(Color::TRANSPARENT)
        };
        let instance = self.style.build_rect_instance(
            self.x,
            self.y,
            actual_width,
            h_box,
            self.style.bg.as_ref(),
            Some(effective_border_c),
            Some(effective_border_w),
            1.0,
        );
        self.ui.draws.push(DrawCommand::Rect(RectDraw { instance }));

        // --- 5. Render Text (ALWAYS CLIPPED TO BOX) ---
        let pos_y = self.y + self.style.padding.top - scroll_y;
        let display_text = if self.buffer.is_empty() {
            self.placeholder.as_deref().unwrap_or("")
        } else {
            &self.buffer
        };
        let display_color = if self.buffer.is_empty() {
            self.placeholder_color.unwrap_or(Color::rgba(1.0, 1.0, 1.0, 0.4))
        } else {
            self.text_style.color
        };

        let mut clean_text_style = self.text_style.clone();
        clean_text_style.highlight_color = None;

        let mut text_builder = TextBuilder::new(self.ui, display_text)
            .color(display_color)
            .wrap(self.text_style.wrap)
            .width(actual_width - self.style.padding.horizontal())
            .pos(self.x + self.style.padding.left, pos_y);
        text_builder.text_style = clean_text_style;
        text_builder.text_style.color = display_color;

        // Always clip text to the textarea box boundary so text never overflows outside
        text_builder = text_builder.clip_rect(self.x + 1.0, self.y + 1.0, (actual_width - 2.0).max(0.0), (h_box - 2.0).max(0.0));
        
        // Final draw
        text_builder.draw_and_measure();

        // Update persistent scroll state
        if self.scrollable || effective_scrollable {
            self.ui.scroll_state.insert(self.id, (scroll_x, scroll_y));

            // --- 5.5 Render Scroll Bar ---
            if h_content > h_box - self.style.padding.vertical() {
                let usable_h = h_box - self.style.padding.vertical();
                let scroll_bar_w = 3.0;
                let scroll_bar_x = self.x + actual_width - scroll_bar_w - 2.0;
                
                let thumb_h = ((usable_h / h_content) * usable_h).clamp(16.0, usable_h);
                let max_scroll = (h_content - usable_h).max(1.0);
                let scroll_percent = (scroll_y / max_scroll).clamp(0.0, 1.0);
                let thumb_y = self.y + self.style.padding.top + scroll_percent * (usable_h - thumb_h);

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
            let lh = self.text_style.line_height;
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
                    let sel_color = self.selection_color.unwrap_or(Color::rgba(0.26, 0.52, 0.96, 0.35));
                    for (line_idx, line) in sb.lines().iter().enumerate() {
                        let line_start = line.start_cluster.min(self.buffer.len());
                        let line_end = if line_idx + 1 < sb.lines().len() {
                            sb.lines()[line_idx + 1].start_cluster.min(self.buffer.len())
                        } else {
                            self.buffer.len()
                        };

                        if line_end <= sel_start || line_start >= sel_end {
                            continue;
                        }

                        let line_cy = line.y + self.y + self.style.padding.top + v_shift - visual_ascent - scroll_y;

                        if line_cy + cursor_height < self.y - 4.0 || line_cy > self.y + h_box + 4.0 {
                            continue;
                        }

                        let mut line_min_x: Option<f32> = None;
                        let mut line_max_x: Option<f32> = None;

                        for g in sb.glyphs() {
                            if g.cluster >= line_start && g.cluster < line_end {
                                if g.cluster >= sel_start && g.cluster < sel_end {
                                    let left = g.x;
                                    let right = g.x + g.width;
                                    line_min_x = Some(line_min_x.map_or(left, |m: f32| m.min(left)));
                                    line_max_x = Some(line_max_x.map_or(right, |m: f32| m.max(right)));
                                }
                            }
                        }

                        if sel_start <= line_start {
                            line_min_x = Some(0.0);
                        }
                        if sel_end >= line_end {
                            let extra = if line_end < self.buffer.len() { font_size * 0.35 } else { 0.0 };
                            line_max_x = Some((line.width + extra).max(font_size * 0.4));
                        }

                        if let (Some(min_x), Some(max_x)) = (line_min_x, line_max_x) {
                            let sel_x = self.x + self.style.padding.left + min_x;
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

                let mut lx = 0.0;
                let mut ly = first_line_y;

                if self.buffer.is_empty() {
                    lx = 0.0;
                    ly = first_line_y;
                } else {
                    let mut found = false;
                    for g in sb.glyphs() {
                        if g.cluster == cursor_index {
                            lx = g.x;
                            found = true;
                            break;
                        }
                    }

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
                        if !found {
                            if cursor_index > line.start_cluster {
                                lx = line.width;
                            } else {
                                lx = 0.0;
                            }
                        }
                    }
                    
                    if cursor_index == self.buffer.len() {
                        if let Some(last_line) = sb.lines().last() {
                            ly = last_line.y;
                            lx = last_line.width;
                        }
                    }
                }
                
                let cx = lx + self.x + self.style.padding.left;
                let cy = ly + self.y + self.style.padding.top + v_shift - visual_ascent - scroll_y;
                
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
                        color: self.cursor_color.unwrap_or(self.text_style.color),
                        clip: [self.x + 1.0, self.y + 1.0, (actual_width - 2.0).max(0.0), (h_box - 2.0).max(0.0)],
                    }));
                }
            }
        }

        // --- 8. Advance UI ---
        self.ui.record_layout(self.id, Rect::new(self.x, self.y, actual_width, h_box));
        self.ui.advance(actual_width + self.style.margin.horizontal(), h_box + self.style.margin.vertical(), start_draw);

        if self.render_mode.is_some() {
            self.ui.render_mode_stack.pop();
        }

        if changed {
            self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Change(crate::ui::EventValue::String(self.buffer.clone())));
        }
        if submitted {
            self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Submit);
        }
        self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Hover(is_hovered));

        zenthra_core::Response {
            clicked: self.ui.clicked && is_hovered,
            hovered: is_hovered,
            pressed: is_hovered && self.ui.mouse_down,
            submitted,
            changed,
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

    #[test]
    fn test_text_area_styling_and_focus_border() {
        use std::collections::HashMap;
        use std::hash::{Hash, Hasher};

        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        let area_id_key = "my_text_area";
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        area_id_key.hash(&mut hasher);
        let expected_id = Id::from_u64(hasher.finish());

        let mut text = String::from("code block");

        // Case 1: bg(None), border(None), focus_border(...) when NOT focused
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.text_area_with_id(&mut text, area_id_key)
                .bg(None)
                .border(None)
                .focus_border((Color::rgb(0.35, 0.58, 0.98), 1.5))
                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.8), 0.0, 2.0, 4.0)
                .show();

            assert!(ui.draws.len() >= 2);
            if let crate::ui::DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.color, Color::TRANSPARENT.to_array());
                assert_eq!(rd.instance.border_width, 0.0);
                assert_eq!(rd.instance.shadow_color, [0.0, 0.0, 0.0, 0.8]);
            } else {
                panic!("Expected RectDraw for text_area at draws[0]");
            }
        }

        // Case 2: Same text_area when FOCUSED -> focus border must render!
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), Some(expected_id),
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.text_area_with_id(&mut text, area_id_key)
                .bg(None)
                .border(None)
                .focus_border((Color::rgb(0.35, 0.58, 0.98), 1.5))
                .show();

            assert!(ui.draws.len() >= 2);
            if let crate::ui::DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.color, Color::TRANSPARENT.to_array());
                assert_eq!(rd.instance.border_width, 1.5);
                assert_eq!(rd.instance.border_color, Color::rgb(0.35, 0.58, 0.98).to_array());
            } else {
                panic!("Expected RectDraw for focused text_area at draws[0]");
            }
        }

        // Case 3: Explicit focus_border(None) when FOCUSED -> no focus border!
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), Some(expected_id),
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.text_area_with_id(&mut text, area_id_key)
                .border((Color::WHITE, 1.0))
                .focus_border(None)
                .show();

            assert!(ui.draws.len() >= 2);
            if let crate::ui::DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.border_width, 0.0);
                assert_eq!(rd.instance.border_color, Color::TRANSPARENT.to_array());
            } else {
                panic!("Expected RectDraw at draws[0]");
            }
        }

        // Case 4: Unified IntoBorder 3-tuple and typography on TextArea
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            let area = ui.text_area_with_id(&mut text, "unified_area")
                .border((Color::GREEN, 2.0, zenthra_core::BorderAlignment::Outside))
                .focus_border((Color::CYAN, 3.0))
                .size(16.0)
                .color(Color::YELLOW)
                .font_weight(500);

            assert_eq!(area.style.border, zenthra_core::Border::Solid(Color::GREEN, 2.0));
            assert_eq!(area.style.border_alignment, zenthra_core::BorderAlignment::Outside);
            assert_eq!(area.focus_border_color, Some(Color::CYAN));
            assert_eq!(area.focus_border_width, Some(3.0));
            assert_eq!(area.text_style.font_size, 16.0);
            assert_eq!(area.text_style.color, Color::YELLOW);
            assert_eq!(area.text_style.font_weight, zenthra_text::prelude::FontWeight::Medium);
        }
    }

    #[test]
    fn test_hit_test_cursor_multiline() {
        use zenthra_text::defaults::cosmic_font::CosmicFontProvider;
        use zenthra_text::types::options::TextOptions;

        let text = "abc\ndef";
        let font_size = 14.0;
        let line_height = 1.0;

        let mut adapter = CosmicFontProvider::new();
        let mut options = TextOptions::new()
            .size(font_size);
        options.line_height = line_height;
        let sb = adapter.shape(text, &options);

        // Click above text -> line 0, start
        assert_eq!(super::hit_test_cursor(&sb, text, 0.0, -5.0, font_size, line_height), 0);

        // Click line 0 on first character
        assert_eq!(super::hit_test_cursor(&sb, text, 1.0, 5.0, font_size, line_height), 0);

        // Click line 0 past width -> end of line 0 before \n (index 3)
        assert_eq!(super::hit_test_cursor(&sb, text, 500.0, 5.0, font_size, line_height), 3);

        // Click line 1 on first character
        let line1_y = sb.lines().get(1).map(|l| l.y - sb.lines()[0].y).unwrap_or(14.0);
        assert_eq!(super::hit_test_cursor(&sb, text, 1.0, line1_y + 2.0, font_size, line_height), 4);

        // Click line 1 past width -> end of text (index 7)
        assert_eq!(super::hit_test_cursor(&sb, text, 500.0, line1_y + 2.0, font_size, line_height), 7);

        // Click below all lines -> end of text (index 7)
        assert_eq!(super::hit_test_cursor(&sb, text, 10.0, 100.0, font_size, line_height), 7);
    }

    #[test]
    fn test_hit_test_code_content() {
        use zenthra_text::defaults::cosmic_font::CosmicFontProvider;
        use zenthra_text::types::options::TextOptions;

        let code_content = String::from(
            "// Welcome to Zenthra TextArea!\n\
             fn calculate_metrics(count: usize) -> f64 {\n\
                 let factor = 1.6180339887;\n\
                 (count as f64) * factor\n\
             }\n\n\
             // Immediate mode UI with zero lag cursor\n\
             let result = calculate_metrics(42);\n\
             println!(\"Result: {}\", result);\n"
        );

        let font_size = 13.0;
        let line_height = 1.4;

        let mut adapter = CosmicFontProvider::new();
        let mut options = TextOptions::new()
            .size(font_size);
        options.line_height = line_height;
        let sb = adapter.shape(&code_content, &options);

        println!("TOTAL LINES: {}", sb.lines().len());
        for (i, line) in sb.lines().iter().enumerate() {
            let line_end = if i + 1 < sb.lines().len() {
                sb.lines()[i + 1].start_cluster
            } else {
                code_content.len()
            };
            let text_snippet = code_content.get(line.start_cluster..line_end).unwrap_or("<invalid>");
            println!("Line {}: y={}, width={}, start_cluster={}, snippet={:?}", i, line.y, line.width, line.start_cluster, text_snippet);
        }

        let first_line_y = sb.lines().first().map(|l| l.y).unwrap_or(0.0);
        let line_h = font_size * line_height;
        // Let's find which line has "// Immediate mode UI with zero lag cursor"
        // and hit test on that line!
        for (i, line) in sb.lines().iter().enumerate() {
            let rel_y = (line.y - first_line_y) + line_h * 0.5;
            let hit = super::hit_test_cursor(&sb, &code_content, 50.0, rel_y, font_size, line_height);
            let char_at_hit = code_content.get(hit..).and_then(|s| s.lines().next()).unwrap_or("");
            println!("Testing Line {} (rel_y={}): hit_idx={}, line starts with {:?}", i, rel_y, hit, char_at_hit);
        }
    }
}
