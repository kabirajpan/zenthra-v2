use crate::ui::{Ui, DrawCommand, OverlayRectDraw, ScrollDrag};
use crate::text::TextBuilder;
use crate::style_ext::BoxStyleExt;
use zenthra_core::{BoxStyle, Color, EdgeInsets, Id, Role, SemanticNode, Rect, Styleable};
use zenthra_platform::event::PlatformEvent;
use zenthra_text::prelude::{TextOptions, CosmicFontProvider, ShapedGlyph};
use zenthra_text::traits::FontProvider;

const CTRL_STATE_KEY: Id = Id::from_u64(0xFEED_C071);
const SHIFT_STATE_KEY: Id = Id::from_u64(0xFEED_581F);
const ALT_STATE_KEY: Id = Id::from_u64(0xFEED_A170);

fn get_selection_bounds(anchor: Option<usize>, cursor: usize, len: usize) -> (usize, usize, bool) {
    if let Some(a) = anchor {
        if a != cursor {
            let s = a.min(cursor).min(len);
            let e = a.max(cursor).min(len);
            (s, e, s < e)
        } else {
            (cursor, cursor, false)
        }
    } else {
        (cursor, cursor, false)
    }
}

fn find_prev_word_boundary(text: &str, cursor: usize) -> usize {
    let cursor = cursor.min(text.len());
    if cursor == 0 {
        return 0;
    }
    let sub = &text[..cursor];
    let mut chars = sub.char_indices().rev().peekable();
    let mut target = cursor;
    while let Some(&(_, c)) = chars.peek() {
        if c.is_whitespace() {
            let (idx, _) = chars.next().unwrap();
            target = idx;
        } else {
            break;
        }
    }
    while let Some(&(idx, c)) = chars.peek() {
        if c.is_whitespace() {
            break;
        }
        chars.next();
        target = idx;
    }
    target
}

fn find_next_word_boundary(text: &str, cursor: usize) -> usize {
    let len = text.len();
    if cursor >= len {
        return len;
    }
    let sub = &text[cursor..];
    let mut chars = sub.char_indices().peekable();
    let mut skipped_non_ws = false;
    let mut target = len;
    while let Some(&(_, c)) = chars.peek() {
        if !c.is_whitespace() {
            chars.next();
            skipped_non_ws = true;
        } else {
            break;
        }
    }
    while let Some(&(idx, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            target = cursor + idx;
            break;
        }
    }
    if !skipped_non_ws && target == len {
        target = cursor;
    }
    target
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

pub struct InputBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    buffer: &'b mut String,
    id: Id,
    x: f32,
    y: f32,

    // Box & typography styling
    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,

    // Input-specific fields
    cursor_color: Option<Color>,
    pub selection_color: Option<Color>,
    scrollable: bool,
    focus_border_color: Option<Color>,
    focus_border_width: Option<f32>,
    placeholder: Option<String>,
    placeholder_color: Option<Color>,
    render_mode: Option<zenthra_core::RenderMode>,
    select_all: bool,
}

impl<'u, 'a, 'b> Styleable for InputBuilder<'u, 'a, 'b> {
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
impl_box_style_methods!(InputBuilder, 'u, 'a, 'b);

// Single-source macro generates all typography styling builder methods (.font_size, .text_color, .font_family, .font_weight, etc.)
impl_font_style_methods!(InputBuilder, 'u, 'a, 'b);

impl<'u, 'a, 'b> InputBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, buffer: &'b mut String) -> Self {
        let id = ui.id();
        let x = ui.cursor_x;
        let y = ui.cursor_y;
        let mut style = BoxStyle::new();
        style.bg = Some(zenthra_core::Background::Solid(Color::rgb(0.2, 0.2, 0.2)));
        style.radius = [4.0; 4];
        style.padding = EdgeInsets::symmetric(4.0, 8.0);

        let mut text_style = zenthra_text::prelude::TextStyle::default();
        text_style.font_size = 18.0;
        text_style.color = Color::WHITE;
        text_style.line_height = 1.2;

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
            scrollable: true,
            focus_border_color: None,
            focus_border_width: None,
            placeholder: None,
            placeholder_color: None,
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

    pub fn cursor_color(mut self, color: Color) -> Self {
        self.cursor_color = Some(color);
        self
    }

    pub fn selection_color(mut self, color: Color) -> Self {
        self.selection_color = Some(color);
        self
    }

    pub fn scrollable(mut self, s: bool) -> Self {
        self.scrollable = s;
        self
    }

    pub fn focus_border(mut self, border: impl Into<zenthra_core::Border>) -> Self {
        match border.into() {
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

    pub fn render_mode(mut self, mode: zenthra_core::RenderMode) -> Self {
        self.render_mode = Some(mode);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
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

    pub fn show(self) -> zenthra_core::Response {
        if let Some(mode) = self.render_mode {
            self.ui.render_mode_stack.push(mode);
        }
        let mut is_focused = self.ui.focused_id == Some(self.id);

        let select_all_key = Id::from_u64(self.id.raw() ^ 0x5E1E_A110);
        if self.select_all && !self.ui.interaction_state.contains_key(&select_all_key) {
            self.ui.interaction_state.insert(select_all_key, 1.0);
            self.ui.focused_id = Some(self.id);
            is_focused = true;
            let sel_anchor_id = Id::from_u64(self.id.raw() ^ 0x5E1EC700);
            self.ui.cursor_state.insert(sel_anchor_id, 0);
            self.ui.cursor_state.insert(self.id, self.buffer.len());
        }
        
        // --- 1. Initial Measure (for hit-testing and initial sizing) ---
        let text_to_measure = if self.buffer.is_empty() {
            self.placeholder.as_deref().unwrap_or("")
        } else {
            &self.buffer
        };
        let (mut w_text_raw, h_content, mut shaped_buffer) = if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
            let mut options = TextOptions::new()
                .font_size(self.text_style.font_size * self.ui.font_scale)
                .font_weight(self.text_style.font_weight)
                .font_style(self.text_style.font_style)
                .line_height(self.text_style.line_height)
                .wrap(zenthra_text::prelude::TextWrap::None);
            if let Some(ref family) = self.text_style.font_family {
                options = options.font_family(family.clone());
            }
            let buffer = adapter.shape(text_to_measure, &options);
            let (cw, _ch) = buffer.content_size();
            let m = adapter.metrics(&options);
            (cw, m.line_height(), Some(buffer))
        } else {
            (self.style.min_width.unwrap_or(0.0), 20.0, None)
        };

        let min_w = self.style.min_width.unwrap_or(0.0);
        let max_available_w = (self.ui.max_x - self.x).max(min_w);
        let mut w_box = if self.style.fill_x { max_available_w } else { self.style.width.unwrap_or_else(|| (w_text_raw + self.style.padding.horizontal()).min(max_available_w)).max(min_w) };
        let h_box = if self.style.fill_y {
            (self.ui.max_y - self.y).max(h_content + self.style.padding.vertical())
        } else {
            self.style.height.unwrap_or(h_content + self.style.padding.vertical())
        };
        let mut w_view = (w_box - self.style.padding.horizontal()).max(0.0);

        // --- 2. Hit Testing & Event Handling ---
        let mut cursor_index = *self.ui.cursor_state.get(&self.id).unwrap_or(&self.buffer.len());
        cursor_index = cursor_index.min(self.buffer.len());
        if !self.buffer.is_char_boundary(cursor_index) {
            cursor_index = self.buffer.len();
        }

        let sel_anchor_id = Id::from_u64(self.id.raw() ^ 0x5E1EC700);
        let mut selection_anchor: Option<usize> = self.ui.cursor_state.get(&sel_anchor_id).copied();
        if let Some(a) = selection_anchor {
            if a > self.buffer.len() {
                selection_anchor = Some(self.buffer.len());
            }
        }

        let mut is_ctrl_down = self.ui.interaction_state.get(&CTRL_STATE_KEY).copied().unwrap_or(0.0) > 0.5;
        let mut is_shift_down = self.ui.interaction_state.get(&SHIFT_STATE_KEY).copied().unwrap_or(0.0) > 0.5;
        let mut is_alt_down = self.ui.interaction_state.get(&ALT_STATE_KEY).copied().unwrap_or(0.0) > 0.5;

        let (actual_x, actual_y) = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            (rect.origin.x + self.ui.offset_x, rect.origin.y + self.ui.offset_y)
        } else {
            (self.x + self.ui.offset_x, self.y + self.ui.offset_y)
        };
        let is_hovered = self.ui.is_hovered(self.id, actual_x, actual_y, w_box, h_box);
        if is_hovered {
            self.ui.cursor_icon = crate::text::CursorIcon::Text;
        }
        self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Hover(is_hovered));
        let mut needs_auto_scroll = false;
        let mut changed = false;
        let mut submitted = false;

        if is_focused || is_hovered || self.ui.active_drag.is_some() {
            let events = std::mem::take(&mut self.ui.input_events);
            for event in &events {
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
                    PlatformEvent::CharTyped(c) if is_focused => {
                        match *c {
                            '\u{1}' => {
                                // Ctrl+A
                                selection_anchor = Some(0);
                                cursor_index = self.buffer.len();
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                self.ui.needs_redraw = true;
                            }
                            '\u{3}' => {
                                // Ctrl+C
                                let (s, e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                if has_sel {
                                    set_clipboard_text(&self.buffer[s..e]);
                                } else if !self.buffer.is_empty() {
                                    set_clipboard_text(&self.buffer);
                                }
                            }
                            '\u{18}' => {
                                // Ctrl+X
                                let (s, e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                if has_sel {
                                    set_clipboard_text(&self.buffer[s..e]);
                                    self.buffer.drain(s..e);
                                    cursor_index = s;
                                    selection_anchor = None;
                                    changed = true;
                                    needs_auto_scroll = true;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                }
                            }
                            '\u{16}' => {
                                // Ctrl+V
                                if let Some(text) = get_clipboard_text() {
                                    let clean_text = text.replace(['\r', '\n'], "");
                                    if !clean_text.is_empty() {
                                        let (s, e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                        if has_sel {
                                            self.buffer.drain(s..e);
                                            cursor_index = s;
                                            selection_anchor = None;
                                        }
                                        self.buffer.insert_str(cursor_index, &clean_text);
                                        cursor_index += clean_text.len();
                                        changed = true;
                                        needs_auto_scroll = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                }
                            }
                            _ if !is_ctrl_down && !c.is_control() && *c != '\r' && *c != '\n' => {
                                let (s, e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                if has_sel {
                                    self.buffer.drain(s..e);
                                    cursor_index = s;
                                    selection_anchor = None;
                                }
                                self.buffer.insert(cursor_index, *c);
                                cursor_index += c.len_utf8();
                                needs_auto_scroll = true;
                                changed = true;
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
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
                                let (s, e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                if has_sel {
                                    set_clipboard_text(&self.buffer[s..e]);
                                } else if !self.buffer.is_empty() {
                                    set_clipboard_text(&self.buffer);
                                }
                            }
                            winit::keyboard::KeyCode::KeyX if is_ctrl_down => {
                                let (s, e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                if has_sel {
                                    set_clipboard_text(&self.buffer[s..e]);
                                    self.buffer.drain(s..e);
                                    cursor_index = s;
                                    selection_anchor = None;
                                    changed = true;
                                    needs_auto_scroll = true;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                } else if !self.buffer.is_empty() {
                                    set_clipboard_text(&self.buffer);
                                    self.buffer.clear();
                                    cursor_index = 0;
                                    selection_anchor = None;
                                    changed = true;
                                    needs_auto_scroll = true;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                }
                            }
                            winit::keyboard::KeyCode::KeyV if is_ctrl_down => {
                                if let Some(text) = get_clipboard_text() {
                                    let clean_text = text.replace(['\r', '\n'], "");
                                    if !clean_text.is_empty() {
                                        let (s, e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                        if has_sel {
                                            self.buffer.drain(s..e);
                                            cursor_index = s;
                                            selection_anchor = None;
                                        }
                                        self.buffer.insert_str(cursor_index, &clean_text);
                                        cursor_index += clean_text.len();
                                        changed = true;
                                        needs_auto_scroll = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::Backspace => {
                                let (s, e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                if has_sel {
                                    self.buffer.drain(s..e);
                                    cursor_index = s;
                                    selection_anchor = None;
                                    changed = true;
                                    needs_auto_scroll = true;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                } else if is_ctrl_down || is_alt_down {
                                    let prev_word = find_prev_word_boundary(&self.buffer, cursor_index);
                                    if prev_word < cursor_index {
                                        self.buffer.drain(prev_word..cursor_index);
                                        cursor_index = prev_word;
                                        changed = true;
                                        needs_auto_scroll = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                } else if cursor_index > 0 {
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
                            winit::keyboard::KeyCode::Delete => {
                                let (s, e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                if has_sel {
                                    self.buffer.drain(s..e);
                                    cursor_index = s;
                                    selection_anchor = None;
                                    changed = true;
                                    needs_auto_scroll = true;
                                    self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                } else if is_ctrl_down || is_alt_down {
                                    let next_word = find_next_word_boundary(&self.buffer, cursor_index);
                                    if next_word > cursor_index {
                                        self.buffer.drain(cursor_index..next_word);
                                        changed = true;
                                        needs_auto_scroll = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                } else if cursor_index < self.buffer.len() {
                                    let mut chars = self.buffer[cursor_index..].chars();
                                    if let Some(_c) = chars.next() {
                                        self.buffer.remove(cursor_index);
                                        needs_auto_scroll = true;
                                        changed = true;
                                        self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::ArrowLeft => {
                                let (s, _e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                if has_sel && !is_shift_down {
                                    cursor_index = s;
                                    selection_anchor = None;
                                } else {
                                    if is_shift_down && selection_anchor.is_none() {
                                        selection_anchor = Some(cursor_index);
                                    }
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
                                }
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                self.ui.needs_redraw = true;
                                needs_auto_scroll = true;
                            }
                            winit::keyboard::KeyCode::ArrowRight => {
                                let (_s, e, has_sel) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
                                if has_sel && !is_shift_down {
                                    cursor_index = e;
                                    selection_anchor = None;
                                } else {
                                    if is_shift_down && selection_anchor.is_none() {
                                        selection_anchor = Some(cursor_index);
                                    }
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
                                }
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                self.ui.needs_redraw = true;
                                needs_auto_scroll = true;
                            }
                            winit::keyboard::KeyCode::Home => {
                                if is_shift_down && selection_anchor.is_none() {
                                    selection_anchor = Some(cursor_index);
                                }
                                cursor_index = 0;
                                if !is_shift_down {
                                    selection_anchor = None;
                                }
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                self.ui.needs_redraw = true;
                                needs_auto_scroll = true;
                            }
                            winit::keyboard::KeyCode::End => {
                                if is_shift_down && selection_anchor.is_none() {
                                    selection_anchor = Some(cursor_index);
                                }
                                cursor_index = self.buffer.len();
                                if !is_shift_down {
                                    selection_anchor = None;
                                }
                                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                                self.ui.needs_redraw = true;
                                needs_auto_scroll = true;
                            }
                            winit::keyboard::KeyCode::Enter | winit::keyboard::KeyCode::NumpadEnter => {
                                submitted = true;
                                self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Submit);
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
            
            let click_id = Id::from_u64(self.id.raw() ^ 0x00C7_1C40);
            let drag_id = Id::from_u64(self.id.raw() ^ 0x00D7_A600);

            let (sx, _sy) = *self.ui.scroll_state.get(&self.id).unwrap_or(&(0.0, 0.0));
            let rel_x = self.ui.mouse_x - (self.x + self.style.padding.left - sx);

            if self.ui.clicked && is_hovered {
                self.ui.focused_id = Some(self.id);
                needs_auto_scroll = true;
                self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Click);

                let hit_pos = if let Some(sb) = &shaped_buffer {
                    if rel_x <= 0.0 {
                        0
                    } else if rel_x >= w_text_raw {
                        self.buffer.len()
                    } else {
                        let mut best_idx = None;
                        let mut best_dist = f32::INFINITY;
                        for g in sb.glyphs() {
                            let center_x = g.x + g.width * 0.5;
                            let dist = (center_x - rel_x).abs();
                            if dist < best_dist {
                                best_dist = dist;
                                let char_len = self.buffer.get(g.cluster..).and_then(|s| s.chars().next()).map(|c| c.len_utf8()).unwrap_or(0);
                                best_idx = Some(if rel_x < center_x { g.cluster } else { (g.cluster + char_len).min(self.buffer.len()) });
                            }
                        }
                        best_idx.unwrap_or(self.buffer.len())
                    }
                } else {
                    self.buffer.len()
                };

                let last_click_time = self.ui.interaction_state.get(&click_id).copied().unwrap_or(0.0);
                if self.ui.elapsed_time - last_click_time < 0.35 {
                    let (w_start, w_end) = find_word_bounds(&self.buffer, hit_pos);
                    if w_start < w_end {
                        selection_anchor = Some(w_start);
                        cursor_index = w_end;
                    } else {
                        selection_anchor = None;
                        cursor_index = hit_pos;
                    }
                    self.ui.interaction_state.insert(drag_id, 2.0);
                } else {
                    cursor_index = hit_pos;
                    selection_anchor = Some(hit_pos);
                    self.ui.interaction_state.insert(drag_id, 1.0);
                }

                self.ui.interaction_state.insert(click_id, self.ui.elapsed_time);
                self.ui.interaction_state.insert(self.id, self.ui.elapsed_time);
                self.ui.needs_redraw = true;
            } else if self.ui.mouse_down && is_focused {
                if let Some(&drag_mode) = self.ui.interaction_state.get(&drag_id) {
                    let hit_pos = if let Some(sb) = &shaped_buffer {
                        if rel_x <= 0.0 {
                            0
                        } else if rel_x >= w_text_raw {
                            self.buffer.len()
                        } else {
                            let mut best_idx = None;
                            let mut best_dist = f32::INFINITY;
                            for g in sb.glyphs() {
                                let center_x = g.x + g.width * 0.5;
                                let dist = (center_x - rel_x).abs();
                                if dist < best_dist {
                                    best_dist = dist;
                                    let char_len = self.buffer.get(g.cluster..).and_then(|s| s.chars().next()).map(|c| c.len_utf8()).unwrap_or(0);
                                    best_idx = Some(if rel_x < center_x { g.cluster } else { (g.cluster + char_len).min(self.buffer.len()) });
                                }
                            }
                            best_idx.unwrap_or(self.buffer.len())
                        }
                    } else {
                        self.buffer.len()
                    };

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
                if self.ui.interaction_state.remove(&drag_id).is_some() {
                    if let Some(anchor) = selection_anchor {
                        if anchor == cursor_index {
                            selection_anchor = None;
                        }
                    }
                }
            }
            
            if let Some(anchor) = selection_anchor {
                self.ui.cursor_state.insert(sel_anchor_id, anchor);
            } else {
                self.ui.cursor_state.remove(&sel_anchor_id);
            }
            self.ui.input_events = events;
            self.ui.cursor_state.insert(self.id, cursor_index);

            if changed {
                self.ui.dispatch_event(self.id, crate::ui::WidgetEvent::Change(crate::ui::EventValue::String(self.buffer.clone())));
            }
        }

        // --- 3. Re-Measure if content changed ---
        if changed {
             if let Some(fs) = self.ui.font_system.as_ref() {
                let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
                adapter.set_layout_size(1000000.0, 10000.0);
                let mut options = TextOptions::new()
                    .font_size(self.text_style.font_size * self.ui.font_scale)
                    .font_weight(self.text_style.font_weight)
                    .font_style(self.text_style.font_style)
                    .line_height(self.text_style.line_height)
                    .wrap(zenthra_text::prelude::TextWrap::None);
                if let Some(ref family) = self.text_style.font_family {
                    options = options.font_family(family.clone());
                }
                let text_to_measure = if self.buffer.is_empty() {
                    self.placeholder.as_deref().unwrap_or("")
                } else {
                    &self.buffer
                };
                let buffer = adapter.shape(text_to_measure, &options);
                let (cw, _ch) = buffer.content_size();
                w_text_raw = cw;
                shaped_buffer = Some(buffer);
                
                // Re-calculate boxes
                let min_w = self.style.min_width.unwrap_or(0.0);
                w_box = if self.style.fill_x { max_available_w } else { self.style.width.unwrap_or_else(|| (w_text_raw + self.style.padding.horizontal()).min(max_available_w)).max(min_w) };
                w_view = (w_box - self.style.padding.horizontal()).max(0.0);
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
        let thumb_x = self.x + self.style.padding.left + scroll_percent * track_width;

        let actual_thumb_x = actual_x + self.style.padding.left + scroll_percent * track_width;
        let actual_scroll_bar_y = actual_y + h_box - scroll_bar_h - 2.0;
        let is_over_thumb = self.ui.mouse_in_rect(actual_thumb_x, actual_scroll_bar_y - 2.0, thumb_w, scroll_bar_h + 4.0);

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

                let cursor_x_v = clx;
                if cursor_x_v > scroll_x + w_view - 40.0 {
                    scroll_x = cursor_x_v - w_view + 40.0;
                } else if cursor_x_v < scroll_x + 10.0 {
                    scroll_x = (cursor_x_v - 10.0).max(0.0);
                }
            }
        }

        scroll_x = scroll_x.clamp(0.0, max_scroll);
        self.ui.scroll_state.insert(scroll_state_id, (scroll_x, scroll_y));

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
            w_box,
            h_box,
            self.style.bg.as_ref(),
            Some(effective_border_c),
            Some(effective_border_w),
            1.0,
        );
        self.ui.draws.push(DrawCommand::Rect(RectDraw { instance }));

        // --- 5. Render Text ---
        let (text_to_draw, draw_color) = if self.buffer.is_empty() && self.placeholder.is_some() {
            (self.placeholder.as_deref().unwrap_or(""), self.placeholder_color.unwrap_or(self.text_style.color.with_alpha(0.4)))
        } else {
            (self.buffer.as_str(), self.text_style.color)
        };

        let mut clean_text_style = self.text_style.clone();
        clean_text_style.highlight_color = None;

        let mut text_builder = TextBuilder::new(self.ui, text_to_draw)
            .text_style(clean_text_style)
            .color(draw_color)
            .wrap(zenthra_text::prelude::TextWrap::None)
            .pos(self.x + self.style.padding.left - scroll_x, self.y + self.style.padding.top)
            .clip_rect(self.x, self.y, w_box, h_box);

        if self.style.fill_x {
            text_builder = text_builder.fill_x();
        }

        let (_, _, final_sb, _) = text_builder.draw_and_measure();

        // --- 5b. Render Selection Highlight ---
        let (sel_start, sel_end, has_selection) = get_selection_bounds(selection_anchor, cursor_index, self.buffer.len());
        if is_focused && has_selection && !self.buffer.is_empty() {
            if let Some(sb) = &final_sb {
                let mut min_x: Option<f32> = None;
                let mut max_x: Option<f32> = None;
                for g in sb.glyphs() {
                    if g.cluster >= sel_start && g.cluster < sel_end {
                        min_x = Some(min_x.map_or(g.x, |m| m.min(g.x)));
                        max_x = Some(max_x.map_or(g.x + g.width, |m| m.max(g.x + g.width)));
                    }
                }
                if let (Some(x0), Some(x1)) = (min_x, max_x) {
                    let sel_x = self.x + self.style.padding.left + x0 - scroll_x;
                    let sel_y = self.y + self.style.padding.top;
                    let sel_w = (x1 - x0).max(2.0);
                    let sel_color = self.selection_color.unwrap_or(Color::rgba(0.2, 0.5, 0.9, 0.35));
                    self.ui.draws.push(DrawCommand::OverlayRect(OverlayRectDraw {
                        x: sel_x,
                        y: sel_y,
                        width: sel_w,
                        height: self.text_style.font_size * self.text_style.line_height,
                        color: sel_color,
                        clip: [self.x, self.y, w_box, h_box],
                    }));
                }
            }
        }
        
        // --- 6. Cursor Rendering ---
        if is_focused {
            let font_size = self.text_style.font_size;
            let lh = self.text_style.line_height;
            let cursor_height = font_size * lh; 

            if let Some(sb) = final_sb {
                let mut lx = 0.0;
                let mut found = false;
                if !self.buffer.is_empty() {
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
                }

                let cx = lx + self.x + self.style.padding.left - scroll_x;
                let cy = self.y + self.style.padding.top;
                
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
                        color: self.cursor_color.unwrap_or(self.text_style.color),
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
        self.ui.record_layout(self.id, Rect::new(self.x, self.y, w_box, h_box));
        self.ui.advance(w_box, h_box, start_draw);

        if self.render_mode.is_some() {
            self.ui.render_mode_stack.pop();
        }

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
    use std::collections::HashMap;
    use zenthra_core::{Id, Color};
    use crate::ui::Ui;

    #[test]
    fn test_input_styling_and_focus_border() {
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

        let input_id_key = "test_input";
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        input_id_key.hash(&mut hasher);
        let expected_id = Id::from_u64(hasher.finish());

        let mut text = String::from("hello");

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

            ui.input_with_id(&mut text, input_id_key)
                .bg(None)
                .border(None)
                .focus_border((Color::rgb(0.35, 0.58, 0.98), 1.5))
                .shadow_opacity(0.8)
                .show();

            assert_eq!(ui.draws.len(), 2);
            if let crate::ui::DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.color, Color::TRANSPARENT.to_array());
                assert_eq!(rd.instance.border_width, 0.0);
            } else {
                panic!("Expected RectDraw for input at draws[0]");
            }
        }

        // Case 2: Same input when FOCUSED -> focus border must render!
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

            ui.input_with_id(&mut text, input_id_key)
                .bg(None)
                .border(None)
                .focus_border((Color::rgb(0.35, 0.58, 0.98), 1.5))
                .show();

            // When focused, it has RectDraw + TextDraw + Blinking Cursor Draw (or 2/3 draws)
            assert!(ui.draws.len() >= 2);
            if let crate::ui::DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.color, Color::TRANSPARENT.to_array());
                assert_eq!(rd.instance.border_width, 1.5);
                assert_eq!(rd.instance.border_color, Color::rgb(0.35, 0.58, 0.98).to_array());
            } else {
                panic!("Expected RectDraw for focused input at draws[0]");
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

            ui.input_with_id(&mut text, input_id_key)
                .bg(None)
                .border(None)
                .focus_border(None)
                .show();

            assert!(ui.draws.len() >= 2);
            if let crate::ui::DrawCommand::Rect(rd) = &ui.draws[0] {
                assert_eq!(rd.instance.border_width, 0.0);
            } else {
                panic!("Expected RectDraw for input at draws[0]");
            }
        }
    }
}
