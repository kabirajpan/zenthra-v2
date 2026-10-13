use zenthra_text::prelude::*;
pub use zenthra_text::prelude::{FontStyle, FontWeight, TextWrap};
// use zenthra_text::traits::FontProvider;
use crate::ui::{DrawCommand, OverlayRectDraw, TextDraw, Ui};
use zenthra_core::{Align, BoxStyle, Color, Id, Rect, Role, SemanticNode, Styleable};
use crate::style_ext::BoxStyleExt;
use zenthra_platform::event::PlatformEvent;

pub struct TextBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,
    content: String,
    selectable: bool,
    pub style: BoxStyle,
    pub text_style: TextStyle,
    cursor: CursorIcon,
    render_mode: Option<zenthra_core::RenderMode>,
    start_x: f32,
    start_y: f32,
    clip_rect: Option<[f32; 4]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorIcon {
    Default,
    Text,
    Pointer,
    Crosshair,
    ColResize,
    RowResize,
    Grab,
    Grabbing,
}

impl<'u, 'a> Styleable for TextBuilder<'u, 'a> {
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
impl_box_style_methods!(TextBuilder);

// Single-source macro generates all TextStyle builder methods (.font_size, .text_color, .font_family, .font_weight, .wrap, etc.)
impl_text_style_methods!(TextBuilder);

impl<'u, 'a> TextBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, content: &str) -> Self {
        let x = ui.cursor_x;
        let y = ui.cursor_y;
        let base_font_size = 16.0;

        // --- STABLE DETERMINISTIC ID ---
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        content.hash(&mut hasher);
        // Include parent ID in hash to differentiate same text in different containers
        if let Some(parent) = ui.semantic_stack.last() {
            parent.hash(&mut hasher);
        }
        let id_raw = hasher.finish();
        let id = Id::from_u64(id_raw);

        let mut text_style = TextStyle::default();
        text_style.font_size = base_font_size;
        text_style.color = Color::WHITE;

        Self {
            ui,
            id,
            content: content.to_string(),
            selectable: false,
            style: BoxStyle::new(),
            text_style,
            cursor: CursorIcon::Default,
            render_mode: None,
            start_x: x,
            start_y: y,
            clip_rect: None,
        }
    }

    #[inline]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.selectable = enabled;
        self
    }

    #[inline]
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        id.hash(&mut hasher);
        if let Some(parent) = self.ui.semantic_stack.last() {
            parent.hash(&mut hasher);
        }
        self.id = zenthra_core::Id::from_u64(hasher.finish());
        self
    }

    #[inline]
    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.start_x = x;
        self.start_y = y;
        self
    }

    #[inline]
    pub fn align(mut self, alignment: Align) -> Self {
        let halign = match alignment {
            Align::Left => HorizontalAlignment::Left,
            Align::Center => HorizontalAlignment::Center,
            Align::Right => HorizontalAlignment::Right,
            _ => HorizontalAlignment::Left,
        };
        self.text_style.align = Some(halign);
        self
    }

    #[inline]
    pub fn valign(mut self, alignment: Align) -> Self {
        let valign = match alignment {
            Align::Top => VerticalAlignment::Top,
            Align::Center => VerticalAlignment::Center,
            Align::Bottom => VerticalAlignment::Bottom,
            _ => VerticalAlignment::Top,
        };
        self.text_style.valign = Some(valign);
        self
    }

    #[inline]
    pub fn cursor(mut self, c: CursorIcon) -> Self {
        self.cursor = c;
        self
    }

    #[inline]
    pub fn clip_rect(mut self, x: f32, y: f32, w: f32, h: f32) -> Self {
        self.clip_rect = Some([x, y, w, h]);
        self
    }

    #[inline]
    pub fn render_mode(mut self, mode: zenthra_core::RenderMode) -> Self {
        self.render_mode = Some(mode);
        self
    }

    pub fn show(mut self) -> (zenthra_core::Response, Option<ShapedBuffer>) {
        if let Some(mode) = self.render_mode {
            self.ui.render_mode_stack.push(mode);
        }
        self.start_x += self.style.margin.left;
        self.start_y += self.style.margin.top;

        let (w, h, buffer, start) = self.draw_and_measure();
        
        let (screen_x, screen_y, screen_w, screen_h) = if let Some(sr) = self.ui.screen_layout_cache.get(&self.id) {
            (sr.origin.x, sr.origin.y, sr.size.width, sr.size.height)
        } else {
            (self.start_x + self.ui.offset_x, self.start_y + self.ui.offset_y, w, h)
        };

        let is_hovered = self.ui.is_hovered(self.id, screen_x, screen_y, screen_w, screen_h);
        if self.selectable && is_hovered {
            self.ui.cursor_icon = CursorIcon::Text;
        }

        if self.selectable {
            let anchor_id = Id::from_u64(self.id.raw() ^ 0x5E1E_C710);
            let cursor_id = Id::from_u64(self.id.raw() ^ 0xCAFE_BABE);
            let active_id = Id::from_u64(self.id.raw() ^ 0xA011_E001);
            let click_id = Id::from_u64(self.id.raw() ^ 0x00C7_1C40);
            let word_mode_id = Id::from_u64(self.id.raw() ^ 0xB0B0_B0B0);

            if let Some(ref sb) = buffer {
                let text_screen_x = screen_x + self.style.padding.left;
                let text_screen_y = screen_y + self.style.padding.top;

                if is_hovered && self.ui.clicked {
                    let rel_x = self.ui.mouse_x - text_screen_x;
                    let rel_y = self.ui.mouse_y - text_screen_y;
                    let idx = sb.index_at(rel_x, rel_y);

                    let last_click_time = self.ui.interaction_state.get(&click_id).copied().unwrap_or(0.0);
                    if self.ui.elapsed_time - last_click_time < 0.35 && self.ui.elapsed_time - last_click_time > 0.0 {
                        // Double click: select the full word under the cursor
                        let word_start = find_word_start(&self.content, idx);
                        let word_end = find_word_end(&self.content, idx);
                        if word_start < word_end {
                            self.ui.cursor_state.insert(anchor_id, word_start);
                            self.ui.cursor_state.insert(cursor_id, word_end);
                            self.ui.interaction_state.insert(word_mode_id, 1.0);
                        } else {
                            self.ui.cursor_state.insert(anchor_id, idx);
                            self.ui.cursor_state.insert(cursor_id, idx);
                            self.ui.interaction_state.remove(&word_mode_id);
                        }
                    } else {
                        self.ui.cursor_state.insert(anchor_id, idx);
                        self.ui.cursor_state.insert(cursor_id, idx);
                        self.ui.interaction_state.remove(&word_mode_id);
                    }
                    self.ui.interaction_state.insert(click_id, self.ui.elapsed_time);
                    self.ui.interaction_state.insert(active_id, 1.0);
                    self.ui.needs_redraw = true;
                } else if self.ui.mouse_down && self.ui.interaction_state.get(&active_id).copied().unwrap_or(0.0) > 0.5 {
                    let is_word_mode = self.ui.interaction_state.get(&word_mode_id).copied().unwrap_or(0.0) > 0.5;
                    let rel_x = self.ui.mouse_x - text_screen_x;
                    let rel_y = self.ui.mouse_y - text_screen_y;
                    let idx = sb.index_at(rel_x, rel_y);

                    if is_word_mode {
                        let base_start = self.ui.cursor_state.get(&anchor_id).copied().unwrap_or(0);
                        let base_end = find_word_end(&self.content, base_start);
                        let drag_word_start = find_word_start(&self.content, idx);
                        let drag_word_end = find_word_end(&self.content, idx);

                        if drag_word_end <= base_start {
                            self.ui.cursor_state.insert(cursor_id, drag_word_start);
                        } else {
                            self.ui.cursor_state.insert(cursor_id, drag_word_end.max(base_end));
                        }
                    } else {
                        self.ui.cursor_state.insert(cursor_id, idx);
                    }
                    self.ui.needs_redraw = true;
                } else if !self.ui.mouse_down {
                    self.ui.interaction_state.remove(&active_id);
                    self.ui.interaction_state.remove(&word_mode_id);
                }

                // If user clicked outside, clear selection
                if self.ui.clicked && !is_hovered {
                    self.ui.cursor_state.remove(&anchor_id);
                    self.ui.cursor_state.remove(&cursor_id);
                    self.ui.interaction_state.remove(&active_id);
                    self.ui.interaction_state.remove(&word_mode_id);
                }

                if let (Some(&anchor), Some(&cur_pos)) = (self.ui.cursor_state.get(&anchor_id), self.ui.cursor_state.get(&cursor_id)) {
                    if anchor != cur_pos {
                        let sel_start = anchor.min(cur_pos);
                        let sel_end = anchor.max(cur_pos);
                        let clip = self.clip_rect.unwrap_or([-100000.0, -100000.0, 2000000.0, 2000000.0]);
                        let font_size = self.text_style.font_size * self.ui.font_scale;
                        let line_height_factor = self.text_style.line_height;
                        let actual_line_height = if line_height_factor > 3.0 {
                            line_height_factor
                        } else {
                            (font_size * line_height_factor).max(font_size)
                        };
                        let half_leading = (actual_line_height - font_size).max(0.0) / 2.0;
                        let visual_ascent = font_size * 0.85 + half_leading;
                        let first_line_y = sb.lines().first().map(|l| l.y).unwrap_or(visual_ascent);
                        let row_threshold = actual_line_height * 0.6;

                        for line in sb.lines() {
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

                            if let (Some(x0), Some(x1)) = (line_min_x, line_max_x) {
                                let sel_x = self.start_x + self.style.padding.left + x0;
                                let sel_y = self.start_y + self.style.padding.top + (line.y - first_line_y);
                                let sel_w = (x1 - x0).max(3.0);
                                let sel_h = actual_line_height;

                                self.ui.draws.push(DrawCommand::OverlayRect(OverlayRectDraw {
                                    x: sel_x,
                                    y: sel_y,
                                    width: sel_w,
                                    height: sel_h,
                                    color: Color::rgba(0.25, 0.50, 0.95, 0.35),
                                    clip,
                                }));
                            }
                        }

                        // Check Ctrl + C
                        let is_ctrl_c = self.ui.input_events.iter().any(|e| matches!(e,
                            PlatformEvent::KeyDown { key: winit::keyboard::KeyCode::KeyC }
                        ));
                        if is_ctrl_c && (is_hovered || self.ui.interaction_state.contains_key(&active_id)) {
                            let safe_start = sel_start.min(self.content.len());
                            let safe_end = sel_end.min(self.content.len());
                            if safe_start < safe_end {
                                if let Some(sub) = self.content.get(safe_start..safe_end) {
                                    crate::ui::copy_to_system_clipboard(sub);
                                }
                            }
                        }
                    }
                }
            }
        }

        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Label, Rect::new(self.start_x, self.start_y, w, h))
                .with_label(self.content.clone())
        );

        // Advance cursor and record draw range AFTER pushing OverlayRectDraw commands
        // so that the parent container's layout translation and scrolling apply to both!
        self.ui.advance(w + self.style.margin.horizontal(), h + self.style.margin.vertical(), start);
        
        if self.render_mode.is_some() {
            self.ui.render_mode_stack.pop();
        }

        let response = zenthra_core::Response {
            clicked: self.ui.clicked && is_hovered,
            hovered: is_hovered,
            pressed: is_hovered && self.ui.mouse_down,
            submitted: false,
            changed: false,
        };

        (response, buffer)
    }

    pub fn draw_and_measure(&mut self) -> (f32, f32, Option<ShapedBuffer>, usize) {
        let clip = self.clip_rect.unwrap_or([-100000.0, -100000.0, 2000000.0, 2000000.0]);

        // Use explicitly set max_width if available, otherwise fallback to explicit width or container width
        let layout_width = self.style.max_width.unwrap_or_else(|| {
            if let Some(w) = self.style.width {
                (w - self.style.padding.horizontal()).max(0.0)
            } else {
                (self.ui.available_width - self.style.padding.horizontal() - self.style.margin.horizontal()).max(0.0)
            }
        });

        let (w, h, buffer) = if let Some(fs) = self.ui.font_system.as_ref() {
             let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
             
             let layout_height = self.style.max_height.unwrap_or(20000.0);
             adapter.set_layout_size(layout_width, layout_height);
             
             let mut shape_opts = self.text_style.to_options(0.0, 0.0, self.ui.scale_factor, self.ui.font_scale);
             shape_opts.max_width = Some(layout_width);
             shape_opts.max_height = self.style.max_height;
             shape_opts.min_width = self.style.min_width;
             shape_opts.clip_rect = self.clip_rect;

             let buffer = adapter.shape(&self.content, &shape_opts);
             let (cw, ch) = buffer.size();

             // Important: record layout for next frame culling
             let mut w = cw + self.style.padding.horizontal();
             if let Some(explicit_w) = self.style.width {
                 w = explicit_w;
             } else if self.style.fill_x {
                 w = (self.ui.max_x - self.start_x - self.style.margin.right).max(0.0);
             } else if let Some(min_w) = self.style.min_width {
                 if w < min_w { w = min_w; }
             }
             let mut h = ch + self.style.padding.vertical();
             if let Some(explicit_h) = self.style.height {
                 h = explicit_h;
             } else if self.style.fill_y {
                 h = (self.ui.max_y - self.start_y - self.style.margin.bottom).max(0.0);
             }
             self.ui.record_layout(self.id, Rect::new(self.start_x, self.start_y, w, h));

             (w, h, Some(buffer))
        } else {
            let w = self.style.width.unwrap_or(100.0);
            let h = self.style.height.unwrap_or(20.0);
            (w, h, None)
        };

        let start_draw = self.ui.draws.len();

        // 1. Background & Border & Shadow
        if let Some(mut rect_inst) = self.style.build_rect(self.start_x, self.start_y, w, h) {
            rect_inst.clip_rect = clip;
            self.ui.draws.push(DrawCommand::Rect(crate::ui::RectDraw {
                instance: rect_inst,
            }));
        }

        // --- 2. Draw Text (Padded) ---
        let mut text_opts = self.text_style.to_options(
            self.start_x + self.style.padding.left,
            self.start_y + self.style.padding.top,
            self.ui.scale_factor,
            self.ui.font_scale,
        );
        text_opts.max_width = Some(layout_width);
        text_opts.max_height = self.style.max_height;
        text_opts.min_width = self.style.min_width;
        text_opts.clip_rect = self.clip_rect;

        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.content.clone().into(),
            pos: [self.start_x + self.style.padding.left, self.start_y + self.style.padding.top],
            options: text_opts,
            clip,
        }));

        (w, h, buffer, start_draw)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use zenthra_core::Color;

    #[test]
    fn test_text_styling_and_decorations() {
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

        // 1. Tag / Badge text from docs/api/container.md Section 3.5
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

            ui.text("PRO")
                .size(10.0)
                .bg(Color::rgb(0.95, 0.65, 0.20))
                .border((Color::rgb(1.0, 0.8, 0.4), 1.0))
                .radius_all(3.0)
                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.5), 0.0, 2.0, 4.0)
                .show();

            assert_eq!(ui.draws.len(), 2, "Expected 1 RectDraw background and 1 TextDraw");
            match &ui.draws[0] {
                DrawCommand::Rect(rect) => {
                    assert_eq!(rect.instance.color, Color::rgb(0.95, 0.65, 0.20).to_array());
                    assert_eq!(rect.instance.border_width, 1.0);
                    assert_eq!(rect.instance.border_color, Color::rgb(1.0, 0.8, 0.4).to_array());
                    assert_eq!(rect.instance.radius, [3.0, 3.0, 3.0, 3.0]);
                    assert_eq!(rect.instance.shadow_color, [0.0, 0.0, 0.0, 0.5]);
                }
                _ => panic!("Expected RectDraw for badge background"),
            }
        }

        // 2. Outline Badge (no bg or bg(None)), with explicit width
        {
            let mut ui2 = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui2.text("OUTLINE")
                .width(70.0)
                .border((Color::WHITE, 1.5))
                .show();

            assert_eq!(ui2.draws.len(), 2, "Outline badge should still emit RectDraw for the border");
            match &ui2.draws[0] {
                DrawCommand::Rect(rect) => {
                    assert_eq!(rect.instance.color, Color::TRANSPARENT.to_array());
                    assert_eq!(rect.instance.border_width, 1.5);
                    assert_eq!(rect.instance.border_color, Color::WHITE.to_array());
                    assert_eq!(rect.instance.size[0], 70.0);
                }
                _ => panic!("Expected RectDraw for outline border"),
            }
        }

        // 3. Plain text should not emit a redundant RectDraw
        {
            let mut ui3 = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui3.text("PLAIN").show();
            assert_eq!(ui3.draws.len(), 1, "Plain text should only emit TextDraw");
            assert!(matches!(&ui3.draws[0], DrawCommand::Text(_)));
        }

        // 4. Test multiline text wrapping and ellipsis with font system
        {
            let font_sys = std::sync::Arc::new(std::sync::Mutex::new(zenthra_text::prelude::FontSystem::new()));
            let mut ui4 = Ui::new(
                800, 600, 1.0, Some(font_sys), Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui4.container()
                .width(360.0)
                .padding_all(14.0)
                .column()
                .gap(8.0)
                .show(|ui: &mut Ui| {
                    ui.text("Multi-line Auto Wrap")
                        .size(12.0)
                        .show();

                    let (_, buf) = ui.text("Zenthra computes typography layout with sub-pixel precision. When text exceeds available container width, it seamlessly breaks across word boundaries while maintaining consistent line spacing.")
                        .size(14.0)
                        .decoration(crate::macros::TextDecoration::LineHeight(20.0))
                        .wrap(zenthra_text::prelude::TextWrap::Word)
                        .show();

                    let b = buf.expect("Expected shaped buffer");
                    assert!(b.lines().len() >= 4, "Expected multiline wrapped lines >= 4");
                    assert!(b.glyphs().len() > 50);
                });

            assert_eq!(ui4.draws.len(), 2);
        }

        // 5. Test word boundary detection for double-click text selection
        {
            let text = "Zenthra computes typography";
            let s1 = find_word_start(text, 3);
            let e1 = find_word_end(text, 3);
            assert_eq!(&text[s1..e1], "Zenthra");

            let s2 = find_word_start(text, 10);
            let e2 = find_word_end(text, 10);
            assert_eq!(&text[s2..e2], "computes");

            let s3 = find_word_start(text, 20);
            let e3 = find_word_end(text, 20);
            assert_eq!(&text[s3..e3], "typography");
        }

        // 6. Test unified font styling and decoration macros
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

            let t = ui.text("Hello Zenthra")
                .size(18.0)
                .color(Color::BLUE)
                .font_weight(700)
                .decoration([crate::macros::TextDecoration::Underline, crate::macros::TextDecoration::Italic]);

            assert_eq!(t.text_style.font_size, 18.0);
            assert_eq!(t.text_style.color, Color::BLUE);
            assert_eq!(t.text_style.font_weight, zenthra_text::prelude::FontWeight::Bold);
            assert!(t.text_style.underline);
            assert_eq!(t.text_style.font_style, zenthra_text::prelude::FontStyle::Italic);
        }
    }

    #[test]
    fn test_text_scroll_reproduction() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let mut layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let mut screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();
        let font_sys = std::sync::Arc::new(std::sync::Mutex::new(zenthra_text::prelude::FontSystem::new()));

        // Frame 1
        {
            let mut ui = Ui::new(
                1050, 800, 1.0, Some(font_sys.clone()), Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            // Replicate examples/text.rs structure
            ui.container()
                .fill()
                .scroll_y(true)
                .gap(24.0)
                .show(|ui: &mut Ui| {
                    for i in 1..=20 {
                        ui.container().fill_x().show(|ui: &mut Ui| {
                            ui.text(&format!("Section {}", i)).size(20.0).show();
                        });
                    }

                    // Section 4
                    ui.container().fill_x().column().show(|ui: &mut Ui| {
                        ui.text("4. Text Wrapping & Ellipsis").size(20.0).show();
                        ui.container().row().gap(20.0).show(|ui: &mut Ui| {
                            ui.container().width(360.0).show(|ui: &mut Ui| {
                                ui.text("Multi-line Auto Wrap").size(12.0).show();
                                ui.text("Zenthra computes typography layout with sub-pixel precision. When text exceeds available container width, it seamlessly breaks across word boundaries while maintaining consistent line spacing.")
                                    .size(14.0)
                                    .wrap(zenthra_text::prelude::TextWrap::Word)
                                    .show();
                            });
                        });
                    });
                });

            println!("FRAME 1 DRAWS: {}", ui.draws.len());
            for (idx, d) in ui.draws.iter().enumerate() {
                match d {
                    DrawCommand::Text(t) => {
                        println!("F1 Draw {}: Text '{}' pos={:?} clip={:?}", idx, t.text, t.pos, t.clip);
                    }
                    DrawCommand::Rect(r) => {
                        println!("F1 Draw {}: Rect pos={:?} size={:?} clip={:?}", idx, r.instance.pos, r.instance.size, r.instance.clip_rect);
                    }
                    _ => {}
                }
            }
        }

        // Frame 2 with scroll
        layout_cache = next_layout_cache;
        next_layout_cache = HashMap::new();
        screen_layout_cache = next_screen_layout_cache;
        next_screen_layout_cache = HashMap::new();
        // Simulate scroll_y = 400.0 on root container
        let root_id = Id::from_u64(1);
        scroll_state.insert(root_id, (0.0, 400.0));

        {
            let mut ui = Ui::new(
                1050, 800, 1.0, Some(font_sys.clone()), Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.container()
                .fill()
                .scroll_y(true)
                .gap(24.0)
                .show(|ui: &mut Ui| {
                    for i in 1..=20 {
                        ui.container().fill_x().show(|ui: &mut Ui| {
                            ui.text(&format!("Section {}", i)).size(20.0).show();
                        });
                    }

                    // Section 4
                    ui.container().fill_x().column().show(|ui: &mut Ui| {
                        ui.text("4. Text Wrapping & Ellipsis").size(20.0).show();
                        ui.container().row().gap(20.0).show(|ui: &mut Ui| {
                            ui.container().width(360.0).show(|ui: &mut Ui| {
                                ui.text("Multi-line Auto Wrap").size(12.0).show();
                                ui.text("Zenthra computes typography layout with sub-pixel precision. When text exceeds available container width, it seamlessly breaks across word boundaries while maintaining consistent line spacing.")
                                    .size(14.0)
                                    .wrap(zenthra_text::prelude::TextWrap::Word)
                                    .show();
                            });
                        });
                    });
                });

            println!("FRAME 2 DRAWS: {}", ui.draws.len());
            for (idx, d) in ui.draws.iter().enumerate() {
                match d {
                    DrawCommand::Text(t) => {
                        println!("F2 Draw {}: Text '{}' pos={:?} clip={:?}", idx, t.text, t.pos, t.clip);
                    }
                    DrawCommand::Rect(r) => {
                        println!("F2 Draw {}: Rect pos={:?} size={:?} clip={:?}", idx, r.instance.pos, r.instance.size, r.instance.clip_rect);
                    }
                    _ => {}
                }
            }
        }
    }
}
