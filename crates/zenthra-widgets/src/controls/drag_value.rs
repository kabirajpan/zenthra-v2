use crate::ui::{DrawCommand, RectDraw, ScrollDrag, TextDraw, Ui};
use std::collections::HashMap;
use std::sync::Mutex;
use zenthra_core::{Align, Color, EdgeInsets, Id, Rect, Response, Role, SemanticNode};
use zenthra_platform::event::{KeyCode, PlatformEvent};
use zenthra_render::RectInstance;

/// Global storage for transient text buffers while typing into a numeric drag box
static DRAG_EDIT_BUFFERS: Mutex<Option<HashMap<Id, String>>> = Mutex::new(None);

fn with_edit_buffers<R>(f: impl FnOnce(&mut HashMap<Id, String>) -> R) -> R {
    let mut guard = DRAG_EDIT_BUFFERS.lock().unwrap();
    let map = guard.get_or_insert_with(HashMap::new);
    f(map)
}

/// Trait implemented by any numeric type that can be scrubbed and edited via `DragValue`
pub trait NumericValue: Copy + PartialEq + PartialOrd + std::fmt::Display + Send + 'static {
    fn to_f64(self) -> f64;
    fn from_f64(val: f64) -> Self;
    fn is_integer() -> bool;

    fn default_precision() -> usize {
        if Self::is_integer() {
            0
        } else {
            2
        }
    }

    fn default_speed() -> f64 {
        if Self::is_integer() {
            1.0
        } else {
            0.1
        }
    }
}

macro_rules! impl_numeric_float {
    ($t:ident) => {
        impl NumericValue for $t {
            #[inline]
            fn to_f64(self) -> f64 {
                self as f64
            }
            #[inline]
            fn from_f64(val: f64) -> Self {
                val as $t
            }
            #[inline]
            fn is_integer() -> bool {
                false
            }
        }
    };
}

macro_rules! impl_numeric_int {
    ($t:ident) => {
        impl NumericValue for $t {
            #[inline]
            fn to_f64(self) -> f64 {
                self as f64
            }
            #[inline]
            fn from_f64(val: f64) -> Self {
                val.round().clamp($t::MIN as f64, $t::MAX as f64) as $t
            }
            #[inline]
            fn is_integer() -> bool {
                true
            }
        }
    };
}

impl_numeric_float!(f32);
impl_numeric_float!(f64);

impl_numeric_int!(i8);
impl_numeric_int!(i16);
impl_numeric_int!(i32);
impl_numeric_int!(i64);
impl_numeric_int!(isize);
impl_numeric_int!(u8);
impl_numeric_int!(u16);
impl_numeric_int!(u32);
impl_numeric_int!(u64);
impl_numeric_int!(usize);

/// Builder for interactive scrubbable numeric drag input box
pub struct DragValueBuilder<'u, 'a, 'b, N: NumericValue> {
    ui: &'u mut Ui<'a>,
    value: &'b mut N,
    id: Id,

    min: Option<f64>,
    max: Option<f64>,
    speed: f64,
    step: Option<f64>,
    precision: usize,
    prefix: String,
    suffix: String,

    // Style
    width: Option<f32>,
    height: f32,
    bg: Color,
    hover_bg: Color,
    active_bg: Color,
    custom_hover_bg: bool,
    custom_active_bg: bool,
    border_color: Color,
    border_width: f32,
    focus_border_color: Color,
    focus_border_width: f32,
    shadow_color: Option<Color>,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    radius: [f32; 4],
    font_size: f32,
    font_family: Option<String>,
    text_color: Color,
    prefix_color: Option<Color>,
    suffix_color: Option<Color>,
    padding: EdgeInsets,
    align: Align,
}

impl<'u, 'a, 'b, N: NumericValue> DragValueBuilder<'u, 'a, 'b, N> {
    pub fn new(ui: &'u mut Ui<'a>, value: &'b mut N, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        let id = Id::from_u64(hasher.finish() & 0x00FF_FFFF_FFFF_FFFF);

        Self {
            ui,
            value,
            id,
            min: None,
            max: None,
            speed: N::default_speed(),
            step: None,
            precision: N::default_precision(),
            prefix: String::new(),
            suffix: String::new(),

            // Modern creative dark theme defaults (e.g. Blender/After Effects style)
            width: None,
            height: 24.0,
            bg: Color::rgb(0.12, 0.13, 0.17),
            hover_bg: Color::rgb(0.17, 0.19, 0.25),
            active_bg: Color::rgb(0.20, 0.24, 0.34),
            custom_hover_bg: false,
            custom_active_bg: false,
            border_color: Color::rgb(0.22, 0.25, 0.33),
            border_width: 1.0,
            focus_border_color: Color::rgb(0.35, 0.58, 0.98),
            focus_border_width: 1.5,
            shadow_color: None,
            shadow_offset: [0.0, 0.0],
            shadow_blur: 0.0,
            radius: [4.0; 4],
            font_size: 12.0,
            font_family: None,
            text_color: Color::rgb(0.92, 0.94, 0.98),
            prefix_color: Some(Color::rgb(0.55, 0.60, 0.72)),
            suffix_color: Some(Color::rgb(0.50, 0.55, 0.68)),
            padding: EdgeInsets::symmetric(0.0, 8.0),
            align: Align::Center,
        }
    }

    /// Sets an explicit ID for the widget
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish() & 0x00FF_FFFF_FFFF_FFFF);
        self
    }

    /// Minimum allowed value
    pub fn min(mut self, min: impl Into<f64>) -> Self {
        self.min = Some(min.into());
        self
    }

    /// Maximum allowed value
    pub fn max(mut self, max: impl Into<f64>) -> Self {
        self.max = Some(max.into());
        self
    }

    /// Clamp bounds [min, max]
    pub fn range(mut self, min: impl Into<f64>, max: impl Into<f64>) -> Self {
        self.min = Some(min.into());
        self.max = Some(max.into());
        self
    }

    /// Scrub sensitivity per horizontal dragged pixel (default: 0.1 for floats, 1.0 for ints)
    pub fn speed(mut self, speed: impl Into<f64>) -> Self {
        self.speed = speed.into();
        self
    }

    /// Optional snapping step (e.g. 1.0, 0.1, 5.0)
    pub fn step(mut self, step: impl Into<f64>) -> Self {
        self.step = Some(step.into());
        self
    }

    /// Number of decimal places to format
    pub fn precision(mut self, precision: usize) -> Self {
        self.precision = precision;
        self
    }

    /// Prefix displayed before the value (e.g. `"X: "`, `"W: "`)
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Suffix displayed after the value (e.g. `" px"`, `"%"`, `"°"`)
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }

    /// Widget width (defaults to 75px or available layout)
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Widget height (defaults to 24px)
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Background color in normal state (supports Color, None)
    pub fn bg(mut self, color: impl Into<Option<Color>>) -> Self {
        match color.into() {
            Some(c) => {
                self.bg = c;
                if c.a == 0.0 {
                    if !self.custom_hover_bg {
                        self.hover_bg = Color::rgba(1.0, 1.0, 1.0, 0.08);
                    }
                    if !self.custom_active_bg {
                        self.active_bg = Color::rgba(1.0, 1.0, 1.0, 0.14);
                    }
                }
            }
            None => {
                self.bg = Color::TRANSPARENT;
                if !self.custom_hover_bg {
                    self.hover_bg = Color::rgba(1.0, 1.0, 1.0, 0.08);
                }
                if !self.custom_active_bg {
                    self.active_bg = Color::rgba(1.0, 1.0, 1.0, 0.14);
                }
            }
        }
        self
    }

    /// Background color when hovered
    pub fn hover_bg(mut self, color: Color) -> Self {
        self.hover_bg = color;
        self.custom_hover_bg = true;
        self
    }

    /// Background color when actively dragging
    pub fn active_bg(mut self, color: Color) -> Self {
        self.active_bg = color;
        self.custom_active_bg = true;
        self
    }

    /// Border color and width (supports (Color, f32), Border, None)
    pub fn border(mut self, border: impl Into<zenthra_core::Border>) -> Self {
        match border.into() {
            zenthra_core::Border::None => {
                self.border_color = Color::TRANSPARENT;
                self.border_width = 0.0;
            }
            zenthra_core::Border::Solid(c, w) => {
                self.border_color = c;
                self.border_width = w;
            }
        }
        self
    }

    /// Focused border color and width (when typing text directly)
    pub fn focus_border(mut self, border: impl Into<zenthra_core::Border>) -> Self {
        match border.into() {
            zenthra_core::Border::None => {
                self.focus_border_color = Color::TRANSPARENT;
                self.focus_border_width = 0.0;
            }
            zenthra_core::Border::Solid(c, w) => {
                self.focus_border_color = c;
                self.focus_border_width = w;
            }
        }
        self
    }

    /// Box shadow
    pub fn shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self {
        self.shadow_color = Some(color);
        self.shadow_offset = [ox, oy];
        self.shadow_blur = blur;
        self
    }

    /// Corner radius
    pub fn radius(mut self, r: f32) -> Self {
        self.radius = [r; 4];
        self
    }

    /// Corner radius for all 4 corners
    pub fn radius_all(mut self, r: f32) -> Self {
        self.radius = [r; 4];
        self
    }

    /// Specific corner radiuses [top_left, top_right, bottom_right, bottom_left]
    pub fn radius_corners(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.radius = [tl, tr, br, bl];
        self
    }

    /// Font size
    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Font family
    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    /// Numeric text color
    pub fn color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Prefix text color
    pub fn prefix_color(mut self, color: Color) -> Self {
        self.prefix_color = Some(color);
        self
    }

    /// Suffix text color
    pub fn suffix_color(mut self, color: Color) -> Self {
        self.suffix_color = Some(color);
        self
    }

    /// Padding inside the drag box
    pub fn padding(mut self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        self.padding = EdgeInsets {
            top,
            right,
            bottom,
            left,
        };
        self
    }

    /// Horizontal padding
    pub fn padding_x(mut self, px: f32) -> Self {
        self.padding.left = px;
        self.padding.right = px;
        self
    }

    /// Text alignment (Align::Left, Align::Center, Align::Right)
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Formats the raw numeric value into a string for editing
    pub fn format_raw(val: f64, precision: usize) -> String {
        if N::is_integer() || precision == 0 {
            format!("{:.0}", val.round())
        } else {
            format!("{:.1$}", val, precision)
        }
    }

    /// Formats the display label including prefix and suffix
    fn format_display(&self, val: f64) -> String {
        let num_str = Self::format_raw(val, self.precision);
        format!("{}{}{}", self.prefix, num_str, self.suffix)
    }

    /// Renders the widget and handles mouse dragging & direct text typing
    pub fn show(self) -> Response {
        let base_raw = self.id.raw() & 0x00FF_FFFF_FFFF_FFFF;
        let edit_mode_key = Id::from_u64((base_raw << 8) | 0x11);
        let drag_start_x_key = Id::from_u64((base_raw << 8) | 0x12);
        let drag_start_val_key = Id::from_u64((base_raw << 8) | 0x13);
        let drag_moved_key = Id::from_u64((base_raw << 8) | 0x14);
        let sel_all_key = Id::from_u64((base_raw << 8) | 0x15);

        let w = self.width.unwrap_or(75.0);
        let h = self.height;
        let x = self.ui.cursor_x;
        let y = self.ui.cursor_y;

        let (actual_ox, actual_oy, actual_w, actual_h) =
            if let Some(screen_rect) = self.ui.screen_layout_cache.get(&self.id) {
                (
                    screen_rect.origin.x,
                    screen_rect.origin.y,
                    if screen_rect.size.width > 0.0 {
                        screen_rect.size.width
                    } else {
                        w
                    },
                    if screen_rect.size.height > 0.0 {
                        screen_rect.size.height
                    } else {
                        h
                    },
                )
            } else if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
                (
                    rect.origin.x + self.ui.offset_x,
                    rect.origin.y + self.ui.offset_y,
                    if rect.size.width > 0.0 {
                        rect.size.width
                    } else {
                        w
                    },
                    if rect.size.height > 0.0 {
                        rect.size.height
                    } else {
                        h
                    },
                )
            } else {
                (x + self.ui.offset_x, y + self.ui.offset_y, w, h)
            };

        let is_hovered = self
            .ui
            .is_hovered(self.id, actual_ox, actual_oy, actual_w, actual_h);
        let is_active_drag = self
            .ui
            .active_drag
            .as_ref()
            .map(|d| d.id == self.id)
            .unwrap_or(false);

        let was_editing = self
            .ui
            .interaction_state
            .get(&edit_mode_key)
            .copied()
            .unwrap_or(0.0)
            > 0.5;

        let is_focused = self.ui.focused_id == Some(self.id);
        let mut changed = false;
        let mut clicked = false;

        // If another widget was clicked / focus lost: commit and exit!
        if was_editing && !is_focused {
            if let Some(buf) = with_edit_buffers(|map| map.remove(&self.id)) {
                if let Ok(num) = buf.trim().parse::<f64>() {
                    let mut clamped = num;
                    if let Some(min) = self.min { clamped = clamped.max(min); }
                    if let Some(max) = self.max { clamped = clamped.min(max); }
                    let new_val = N::from_f64(clamped);
                    if new_val != *self.value {
                        *self.value = new_val;
                        changed = true;
                    }
                }
            }
            self.ui.interaction_state.insert(edit_mode_key, 0.0);
            self.ui.interaction_state.insert(sel_all_key, 0.0);
            self.ui.needs_redraw = true;
        }

        let is_editing = was_editing && is_focused;

        // -------------------------------------------------------------
        // MODE 1: DIRECT IN-PLACE TEXT EDITING
        // -------------------------------------------------------------
        if is_editing {
            let mut buffer = with_edit_buffers(|map| {
                map.entry(self.id)
                    .or_insert_with(|| Self::format_raw(self.value.to_f64(), self.precision))
                    .clone()
            });

            let mut commit = false;
            let mut cancel = false;
            let mut buffer_dirty = false;
            let mut is_sel_all = self
                .ui
                .interaction_state
                .get(&sel_all_key)
                .copied()
                .unwrap_or(0.0)
                > 0.5;

            let mut char_from_key = None;

            // Process keyboard input
            for event in &self.ui.input_events {
                match event {
                    PlatformEvent::KeyDown { key } => match key {
                        KeyCode::Enter | KeyCode::NumpadEnter => {
                            commit = true;
                        }
                        KeyCode::Escape => {
                            cancel = true;
                        }
                        KeyCode::Backspace => {
                            if is_sel_all {
                                buffer.clear();
                                is_sel_all = false;
                                self.ui.interaction_state.insert(sel_all_key, 0.0);
                                buffer_dirty = true;
                            } else {
                                buffer.pop();
                                buffer_dirty = true;
                            }
                            self.ui.needs_redraw = true;
                        }
                        KeyCode::Delete => {
                            buffer.clear();
                            is_sel_all = false;
                            self.ui.interaction_state.insert(sel_all_key, 0.0);
                            buffer_dirty = true;
                            self.ui.needs_redraw = true;
                        }
                        KeyCode::Digit0 | KeyCode::Numpad0 => char_from_key = Some('0'),
                        KeyCode::Digit1 | KeyCode::Numpad1 => char_from_key = Some('1'),
                        KeyCode::Digit2 | KeyCode::Numpad2 => char_from_key = Some('2'),
                        KeyCode::Digit3 | KeyCode::Numpad3 => char_from_key = Some('3'),
                        KeyCode::Digit4 | KeyCode::Numpad4 => char_from_key = Some('4'),
                        KeyCode::Digit5 | KeyCode::Numpad5 => char_from_key = Some('5'),
                        KeyCode::Digit6 | KeyCode::Numpad6 => char_from_key = Some('6'),
                        KeyCode::Digit7 | KeyCode::Numpad7 => char_from_key = Some('7'),
                        KeyCode::Digit8 | KeyCode::Numpad8 => char_from_key = Some('8'),
                        KeyCode::Digit9 | KeyCode::Numpad9 => char_from_key = Some('9'),
                        KeyCode::Period | KeyCode::NumpadDecimal => char_from_key = Some('.'),
                        KeyCode::Minus | KeyCode::NumpadSubtract => char_from_key = Some('-'),
                        _ => {}
                    },
                    PlatformEvent::CharTyped(c) => {
                        // Allow digits, signs, decimal points, exponents
                        if !c.is_control() && (*c == '.' || *c == '-' || *c == '+' || c.is_ascii_digit() || *c == 'e' || *c == 'E') {
                            char_from_key = Some(*c);
                        }
                    }
                    _ => {}
                }
            }

            if let Some(c) = char_from_key {
                if is_sel_all {
                    buffer.clear();
                    is_sel_all = false;
                    self.ui.interaction_state.insert(sel_all_key, 0.0);
                }
                buffer.push(c);
                buffer_dirty = true;
                self.ui.needs_redraw = true;
            }

            // Click inside deselects all so user sees cursor
            if self.ui.clicked && is_hovered {
                if is_sel_all {
                    is_sel_all = false;
                    self.ui.interaction_state.insert(sel_all_key, 0.0);
                }
                self.ui.consume_click();
                self.ui.needs_redraw = true;
            }

            // Outside click commits value and exits edit mode
            if self.ui.clicked && !is_hovered {
                commit = true;
            }

            if commit {
                if let Ok(num) = buffer.trim().parse::<f64>() {
                    let mut clamped = num;
                    if let Some(min) = self.min {
                        clamped = clamped.max(min);
                    }
                    if let Some(max) = self.max {
                        clamped = clamped.min(max);
                    }
                    let new_val = N::from_f64(clamped);
                    if new_val != *self.value {
                        *self.value = new_val;
                        changed = true;
                    }
                }
                self.ui.interaction_state.insert(edit_mode_key, 0.0);
                self.ui.interaction_state.insert(sel_all_key, 0.0);
                if self.ui.focused_id == Some(self.id) {
                    self.ui.focused_id = None;
                }
                with_edit_buffers(|map| {
                    map.remove(&self.id);
                });
                self.ui.needs_redraw = true;
            } else if cancel {
                self.ui.interaction_state.insert(edit_mode_key, 0.0);
                self.ui.interaction_state.insert(sel_all_key, 0.0);
                if self.ui.focused_id == Some(self.id) {
                    self.ui.focused_id = None;
                }
                with_edit_buffers(|map| {
                    map.remove(&self.id);
                });
                self.ui.needs_redraw = true;
            } else if buffer_dirty {
                with_edit_buffers(|map| {
                    map.insert(self.id, buffer.clone());
                });
            }

            // Render in-place edit box
            let start_draw = self.ui.draws.len();

            // Background with focused border
            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [x, y],
                    size: [w, h],
                    color: self.active_bg.to_array(),
                    radius: self.radius,
                    border_width: self.focus_border_width,
                    border_color: self.focus_border_color.to_array(),
                    shadow_color: self.shadow_color.map(|c| c.to_array()).unwrap_or([0.0; 4]),
                    shadow_offset: self.shadow_offset,
                    shadow_blur: self.shadow_blur,
                    clip_rect: [0.0, 0.0, 9999.0, 9999.0],
                    opacity: 1.0,
                    ..Default::default()
                },
            }));

            // Text measurement
            let mut text_w = 0.0;
            let text_h = 14.0;
            if let Some(fs) = self.ui.font_system.as_ref() {
                let mut adapter = zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
                let mut options = zenthra_text::prelude::TextOptions::new()
                    .size(self.font_size * self.ui.font_scale)
                    .scale_factor(self.ui.scale_factor)
                    .wrap(zenthra_text::prelude::TextWrap::None);
                if let Some(ref family) = self.font_family {
                    options = options.font_family(family.clone());
                }
                let shaped = adapter.shape(&buffer, &options);
                let (cw, _) = shaped.content_size();
                text_w = cw;
            }

            let tx = match self.align {
                Align::Left => (x + self.padding.left).round(),
                Align::Right => (x + w - text_w - self.padding.right).round(),
                _ => (x + (w - text_w) / 2.0).round(),
            };
            let ty = (y + (h - text_h) / 2.0).round();

            // Selection highlight
            if is_sel_all && text_w > 0.0 {
                self.ui.draws.push(DrawCommand::Rect(RectDraw {
                    instance: RectInstance {
                        pos: [tx - 2.0, ty - 1.0],
                        size: [text_w + 4.0, text_h + 2.0],
                        color: Color::rgba(0.35, 0.58, 0.98, 0.35).to_array(),
                        radius: [2.0; 4],
                        border_width: 0.0,
                        border_color: [0.0; 4],
                        clip_rect: [x, y, w, h],
                        opacity: 1.0,
                        ..Default::default()
                    },
                }));
            }

            // Text
            let mut text_opts = zenthra_text::prelude::TextOptions::new()
                .size(self.font_size * self.ui.font_scale)
                .scale_factor(self.ui.scale_factor)
                .color(self.text_color)
                .wrap(zenthra_text::prelude::TextWrap::None)
                .at(tx, ty);
            if let Some(ref family) = self.font_family {
                text_opts = text_opts.font_family(family.clone());
            }

            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: buffer.clone(),
                pos: [tx, ty],
                options: text_opts,
                clip: [x, y, w, h],
            }));

            // Blinking cursor (when not selecting all)
            if !is_sel_all && (self.ui.elapsed_time * 2.0).fract() < 0.5 {
                let cursor_x = tx + text_w + 1.0;
                self.ui.draws.push(DrawCommand::Rect(RectDraw {
                    instance: RectInstance {
                        pos: [cursor_x, ty],
                        size: [1.5, text_h],
                        color: self.text_color.to_array(),
                        radius: [0.0; 4],
                        border_width: 0.0,
                        border_color: [0.0; 4],
                        clip_rect: [x, y, w, h],
                        opacity: 1.0,
                        ..Default::default()
                    },
                }));
            }

            self.ui.record_layout(self.id, Rect::new(x, y, w, h));
            self.ui.advance(w, h, start_draw);

            return Response {
                clicked: false,
                hovered: is_hovered,
                pressed: false,
                submitted: commit,
                changed,
            };
        }

        // -------------------------------------------------------------
        // MODE 2: DISPLAY & SCRUBBING
        // -------------------------------------------------------------
        if is_hovered || is_active_drag {
            self.ui.cursor_icon = crate::text::CursorIcon::ColResize;
        }

        let is_capturing = self.ui.interaction_state.contains_key(&drag_start_x_key);

        // 1. Mouse down: start capturing drag (or immediate edit mode on fast click)
        if self.ui.clicked && is_hovered && self.ui.can_interact(self.id) {
            clicked = true;
            self.ui.consume_click();
            if !self.ui.mouse_down {
                // Immediate click with mouse already released
                self.ui.interaction_state.insert(edit_mode_key, 1.0);
                self.ui.interaction_state.insert(sel_all_key, 1.0);
                self.ui.focused_id = Some(self.id);

                let raw_str = Self::format_raw(self.value.to_f64(), self.precision);
                with_edit_buffers(|map| {
                    map.insert(self.id, raw_str);
                });
                self.ui.needs_redraw = true;
            } else {
                self.ui.claim_active(self.id);
                self.ui.active_drag = Some(ScrollDrag {
                    id: self.id,
                    start_mouse: self.ui.mouse_x,
                    start_scroll: self.value.to_f64() as f32,
                });
                self.ui.interaction_state.insert(drag_start_x_key, self.ui.mouse_x);
                self.ui
                    .interaction_state
                    .insert(drag_start_val_key, self.value.to_f64() as f32);
                self.ui.interaction_state.insert(drag_moved_key, 0.0);
            }
        }

        // 2. Active scrubbing (with deadzone)
        let deadzone = 6.0;
        if (is_active_drag || is_capturing) && self.ui.mouse_down {
            let start_mouse_x = self
                .ui
                .interaction_state
                .get(&drag_start_x_key)
                .copied()
                .unwrap_or(self.ui.mouse_x);
            let start_val = self
                .ui
                .interaction_state
                .get(&drag_start_val_key)
                .copied()
                .unwrap_or(self.value.to_f64() as f32) as f64;

            let dx = (self.ui.mouse_x - start_mouse_x) as f64;
            let mut has_moved = self
                .ui
                .interaction_state
                .get(&drag_moved_key)
                .copied()
                .unwrap_or(0.0)
                > 0.5;

            // Deadzone: require >= 6.0px movement to begin scrubbing
            // This prevents normal clicks and hand jitter from ever accidentally mutating numbers!
            if !has_moved && dx.abs() >= deadzone {
                has_moved = true;
                self.ui.interaction_state.insert(drag_moved_key, 1.0);
            }

            if has_moved {
                let scrub_dx = if dx > 0.0 { dx - deadzone } else { dx + deadzone };

                // Modifiers: Shift = fine (0.1x), Ctrl = coarse (10x)
                let is_shift = self
                    .ui
                    .interaction_state
                    .get(&Id::from_u64(0xFEED_581F))
                    .copied()
                    .unwrap_or(0.0)
                    > 0.5;
                let is_ctrl = self
                    .ui
                    .interaction_state
                    .get(&Id::from_u64(0xFEED_C001))
                    .or_else(|| self.ui.interaction_state.get(&Id::from_u64(0xFEED_C071)))
                    .copied()
                    .unwrap_or(0.0)
                    > 0.5;
                let modifier_mult = if is_shift {
                    0.1
                } else if is_ctrl {
                    10.0
                } else {
                    1.0
                };

                let mut new_val = start_val + scrub_dx * self.speed * modifier_mult;

                if let Some(step) = self.step {
                    new_val = (new_val / step).round() * step;
                }
                if let Some(min) = self.min {
                    new_val = new_val.max(min);
                }
                if let Some(max) = self.max {
                    new_val = new_val.min(max);
                }

                let new_typed = N::from_f64(new_val);
                if new_typed != *self.value {
                    *self.value = new_typed;
                    changed = true;
                    self.ui.needs_redraw = true;
                }
            }
        }

        // 3. Mouse release / Click detection
        if is_capturing && !self.ui.mouse_down {
            let start_mouse_x = self
                .ui
                .interaction_state
                .get(&drag_start_x_key)
                .copied()
                .unwrap_or(self.ui.mouse_x);
            let total_dx = (self.ui.mouse_x - start_mouse_x).abs() as f64;
            let has_moved = self
                .ui
                .interaction_state
                .get(&drag_moved_key)
                .copied()
                .unwrap_or(0.0)
                > 0.5;

            if !has_moved || total_dx < deadzone {
                // Clicked without scrubbing -> Enter in-place typing mode!
                self.ui.interaction_state.insert(edit_mode_key, 1.0);
                self.ui.interaction_state.insert(sel_all_key, 1.0);
                self.ui.focused_id = Some(self.id);

                let raw_str = Self::format_raw(self.value.to_f64(), self.precision);
                with_edit_buffers(|map| {
                    map.insert(self.id, raw_str);
                });
                self.ui.consume_click();
                self.ui.needs_redraw = true;
            }

            // Cleanup drag states
            self.ui.interaction_state.remove(&drag_start_x_key);
            self.ui.interaction_state.remove(&drag_start_val_key);
            self.ui.interaction_state.remove(&drag_moved_key);
            self.ui.active_drag = None;
            if self.ui.is_active(self.id) {
                self.ui.release_active();
            }
        }

        // -------------------------------------------------------------
        // RENDERING
        // -------------------------------------------------------------
        let start_draw = self.ui.draws.len();

        let current_bg = if is_active_drag {
            self.active_bg
        } else if is_hovered {
            self.hover_bg
        } else {
            self.bg
        };

        let current_border = if is_active_drag {
            self.focus_border_color
        } else {
            self.border_color
        };

        // 1. Box background & border
        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [x, y],
                size: [w, h],
                color: current_bg.to_array(),
                radius: self.radius,
                border_width: self.border_width,
                border_color: current_border.to_array(),
                shadow_color: self.shadow_color.map(|c| c.to_array()).unwrap_or([0.0; 4]),
                shadow_offset: self.shadow_offset,
                shadow_blur: self.shadow_blur,
                clip_rect: [0.0, 0.0, 9999.0, 9999.0],
                opacity: 1.0,
                ..Default::default()
            },
        }));

        // 2. Measure & Draw Label (prefix + value + suffix)
        let num_str = Self::format_raw(self.value.to_f64(), self.precision);
        let display_label = self.format_display(self.value.to_f64());
        let mut total_w = 0.0;
        let mut text_h = 14.0;
        let mut prefix_w = 0.0;
        let mut num_w = 0.0;
        let mut suffix_w = 0.0;

        let has_custom_segment_colors = self.prefix_color.is_some() || self.suffix_color.is_some();

        if let Some(fs) = self.ui.font_system.as_ref() {
            let mut adapter =
                zenthra_text::prelude::CosmicFontProvider::new_with_system(fs.clone());
            let mut options = zenthra_text::prelude::TextOptions::new()
                .size(self.font_size * self.ui.font_scale)
                .scale_factor(self.ui.scale_factor)
                .wrap(zenthra_text::prelude::TextWrap::None);
            if let Some(ref family) = self.font_family {
                options = options.font_family(family.clone());
            }

            if has_custom_segment_colors && (!self.prefix.is_empty() || !self.suffix.is_empty()) {
                if !self.prefix.is_empty() {
                    let p_buf = adapter.shape(&self.prefix, &options);
                    prefix_w = p_buf.content_size().0;
                }
                let n_buf = adapter.shape(&num_str, &options);
                let (cw, ch) = n_buf.content_size();
                num_w = cw;
                text_h = ch;
                if !self.suffix.is_empty() {
                    let s_buf = adapter.shape(&self.suffix, &options);
                    suffix_w = s_buf.content_size().0;
                }
                total_w = prefix_w + num_w + suffix_w;
            } else {
                let buffer = adapter.shape(&display_label, &options);
                let (cw, ch) = buffer.content_size();
                total_w = cw;
                text_h = ch;
            }
        }

        let tx = match self.align {
            Align::Left => (x + self.padding.left).round(),
            Align::Right => (x + w - total_w - self.padding.right).round(),
            _ => (x + (w - total_w) / 2.0).round(),
        };
        let ty = (y + (h - text_h) / 2.0).round();

        if has_custom_segment_colors && (!self.prefix.is_empty() || !self.suffix.is_empty()) {
            let mut cur_x = tx;
            if !self.prefix.is_empty() {
                let mut p_opts = zenthra_text::prelude::TextOptions::new()
                    .size(self.font_size * self.ui.font_scale)
                    .scale_factor(self.ui.scale_factor)
                    .color(self.prefix_color.unwrap_or(self.text_color))
                    .wrap(zenthra_text::prelude::TextWrap::None)
                    .at(cur_x, ty);
                if let Some(ref family) = self.font_family {
                    p_opts = p_opts.font_family(family.clone());
                }
                self.ui.draws.push(DrawCommand::Text(TextDraw {
                    text: self.prefix.clone(),
                    pos: [cur_x, ty],
                    options: p_opts,
                    clip: [x, y, w, h],
                }));
                cur_x += prefix_w;
            }

            let mut n_opts = zenthra_text::prelude::TextOptions::new()
                .size(self.font_size * self.ui.font_scale)
                .scale_factor(self.ui.scale_factor)
                .color(self.text_color)
                .wrap(zenthra_text::prelude::TextWrap::None)
                .at(cur_x, ty);
            if let Some(ref family) = self.font_family {
                n_opts = n_opts.font_family(family.clone());
            }
            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: num_str.clone(),
                pos: [cur_x, ty],
                options: n_opts,
                clip: [x, y, w, h],
            }));
            cur_x += num_w;

            if !self.suffix.is_empty() {
                let mut s_opts = zenthra_text::prelude::TextOptions::new()
                    .size(self.font_size * self.ui.font_scale)
                    .scale_factor(self.ui.scale_factor)
                    .color(self.suffix_color.unwrap_or(self.text_color))
                    .wrap(zenthra_text::prelude::TextWrap::None)
                    .at(cur_x, ty);
                if let Some(ref family) = self.font_family {
                    s_opts = s_opts.font_family(family.clone());
                }
                self.ui.draws.push(DrawCommand::Text(TextDraw {
                    text: self.suffix.clone(),
                    pos: [cur_x, ty],
                    options: s_opts,
                    clip: [x, y, w, h],
                }));
            }
        } else {
            let mut text_opts = zenthra_text::prelude::TextOptions::new()
                .size(self.font_size * self.ui.font_scale)
                .scale_factor(self.ui.scale_factor)
                .color(self.text_color)
                .wrap(zenthra_text::prelude::TextWrap::None)
                .at(tx, ty);
            if let Some(ref family) = self.font_family {
                text_opts = text_opts.font_family(family.clone());
            }

            self.ui.draws.push(DrawCommand::Text(TextDraw {
                text: display_label.clone(),
                pos: [tx, ty],
                options: text_opts,
                clip: [x, y, w, h],
            }));
        }

        // 3. Register Semantic Node
        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Slider, Rect::new(x, y, w, h))
                .with_label(display_label)
                .with_value(self.value.to_f64() as f32),
        );

        // 4. Advance Layout
        self.ui.record_layout(self.id, Rect::new(x, y, w, h));
        self.ui.advance(w, h, start_draw);

        Response {
            clicked,
            hovered: is_hovered,
            pressed: is_active_drag || (self.ui.mouse_down && is_hovered),
            submitted: false,
            changed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_value_conversions() {
        assert_eq!(12.5f32.to_f64(), 12.5);
        assert_eq!(f32::from_f64(42.5), 42.5f32);
        assert!(!f32::is_integer());
        assert_eq!(f32::default_precision(), 2);

        assert_eq!(100i32.to_f64(), 100.0);
        assert_eq!(i32::from_f64(99.6), 100i32);
        assert!(i32::is_integer());
        assert_eq!(i32::default_precision(), 0);

        assert_eq!(5usize.to_f64(), 5.0);
        assert_eq!(usize::from_f64(8.2), 8usize);
        assert!(usize::is_integer());
        assert_eq!(usize::default_precision(), 0);
    }

    #[test]
    fn test_click_box_scale() {
        use std::collections::HashMap;

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

        let mut pos_x: f32 = 960.0;
        let mut pos_y: f32 = 540.0;
        let mut scale: f32 = 100.0;
        let mut rotation: f32 = 0.0;
        let mut opacity: f32 = 100.0;
        let mut duration_sec: f64 = 5.25;
        let mut frame_count: i32 = 120;
        let mut layer_id: usize = 1;
        let mut roughness: f32 = 0.45;

        let mut focused_id: Option<Id> = None;

        let font_system = std::sync::Arc::new(std::sync::Mutex::new(zenthra_text::prelude::FontSystem::new()));

        let mut run_ui = |mouse: (f32, f32), mouse_down: bool, clicked: bool,
                          lc: &HashMap<Id, (Rect, u64)>, nlc: &mut HashMap<Id, (Rect, u64)>,
                          slc: &HashMap<Id, Rect>, nslc: &mut HashMap<Id, Rect>,
                          is: &mut HashMap<Id, f32>,
                          f_id: &mut Option<Id>| {
            let mut ui = Ui::new(
                1080, 720, 1.0, Some(font_system.clone()), Vec::new(), *f_id,
                mouse, mouse_down,
                &mut scroll_state, &mut cursor_state, is,
                None, None, clicked, false, 0.0,
                lc, nlc,
                slc, nslc,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.container()
                .fill_x()
                .fill_y()
                .bg(Color::rgb(0.08, 0.09, 0.12))
                .padding(24.0, 32.0, 24.0, 32.0)
                .show(|ui: &mut Ui| {
                    ui.text("Drag Value / Numeric Box")
                        .size(22.0)
                        .show();
                    ui.spacing(4.0);
                    ui.text("Scrub horizontally...")
                        .size(13.0)
                        .show();
                    ui.spacing(20.0);

                    ui.container().row().gap(24.0).show(|ui: &mut Ui| {
                        // Left Column
                        ui.container().width(490.0).show(|ui: &mut Ui| {
                            ui.container()
                                .fill_x()
                                .padding(14.0, 16.0, 14.0, 16.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("BOX STYLE: TRANSFORM").size(11.0).show();
                                    ui.spacing(10.0);

                                    ui.container().row().gap(10.0).valign(Align::Center).show(|ui| {
                                        ui.container().width(70.0).show(|ui| {
                                            ui.text("Position:").size(13.0).show();
                                        });
                                        ui.drag_value(&mut pos_x, "box_pos_x")
                                            .prefix("X: ")
                                            .suffix(" px")
                                            .speed(1.0)
                                            .precision(1)
                                            .width(130.0)
                                            .show();
                                        ui.drag_value(&mut pos_y, "box_pos_y")
                                            .prefix("Y: ")
                                            .suffix(" px")
                                            .speed(1.0)
                                            .precision(1)
                                            .width(130.0)
                                            .show();
                                    });

                                    ui.spacing(8.0);

                                    ui.container().row().gap(10.0).valign(Align::Center).show(|ui| {
                                        ui.container().width(70.0).show(|ui| {
                                            ui.text("Scale:").size(13.0).show();
                                        });
                                        ui.drag_value(&mut scale, "box_scale")
                                            .min(0.0)
                                            .max(1000.0)
                                            .speed(0.5)
                                            .suffix(" %")
                                            .precision(1)
                                            .width(110.0)
                                            .show();

                                        ui.text("Rotation:").size(13.0).show();
                                        ui.drag_value(&mut rotation, "box_rotation")
                                            .speed(0.5)
                                            .step(1.0)
                                            .suffix("°")
                                            .precision(0)
                                            .width(90.0)
                                            .show();
                                    });

                                    ui.spacing(8.0);

                                    ui.container().row().gap(10.0).valign(Align::Center).show(|ui| {
                                        ui.container().width(70.0).show(|ui| {
                                            ui.text("Opacity:").size(13.0).show();
                                        });
                                        ui.drag_value(&mut opacity, "box_opacity")
                                            .range(0.0, 100.0)
                                            .speed(0.2)
                                            .suffix(" %")
                                            .precision(1)
                                            .width(110.0)
                                            .show();
                                    });
                                });

                            ui.spacing(12.0);

                            ui.container()
                                .fill_x()
                                .padding(14.0, 16.0, 14.0, 16.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("BOX STYLE: TIMELINE / SHADER").size(11.0).show();
                                    ui.spacing(10.0);

                                    ui.container().row().gap(10.0).valign(Align::Center).show(|ui| {
                                        ui.container().width(95.0).show(|ui| { ui.text("Duration:").size(13.0).show(); });
                                        ui.drag_value(&mut duration_sec, "box_duration")
                                            .min(0.0)
                                            .speed(0.05)
                                            .suffix(" s")
                                            .precision(2)
                                            .width(120.0)
                                            .show();
                                    });

                                    ui.spacing(8.0);

                                    ui.container().row().gap(10.0).valign(Align::Center).show(|ui| {
                                        ui.container().width(95.0).show(|ui| { ui.text("Frames:").size(13.0).show(); });
                                        ui.drag_value(&mut frame_count, "box_frame_count")
                                            .min(0)
                                            .speed(1.0)
                                            .suffix(" f")
                                            .width(120.0)
                                            .show();
                                    });

                                    ui.spacing(8.0);

                                    ui.container().row().gap(10.0).valign(Align::Center).show(|ui| {
                                        ui.container().width(95.0).show(|ui| { ui.text("Layer:").size(13.0).show(); });
                                        ui.drag_value(&mut layer_id, "box_layer_id")
                                            .min(1)
                                            .speed(1.0)
                                            .prefix("#")
                                            .width(90.0)
                                            .show();
                                    });

                                    ui.spacing(8.0);

                                    ui.container().row().gap(10.0).valign(Align::Center).show(|ui| {
                                        ui.container().width(95.0).show(|ui| { ui.text("Roughness:").size(13.0).show(); });
                                        ui.drag_value(&mut roughness, "box_roughness")
                                            .range(0.0, 1.0)
                                            .speed(0.01)
                                            .precision(3)
                                            .width(120.0)
                                            .show();
                                    });
                                });
                        });

                        // Right Column
                        ui.container().width(490.0).show(|ui: &mut Ui| {
                            ui.container()
                                .fill_x()
                                .padding(14.0, 16.0, 14.0, 16.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("WITHOUT BOX (INTERACTIVE DRAGGABLE TEXT)").size(11.0).show();
                                    ui.spacing(10.0);

                                    ui.container().row().gap(10.0).valign(Align::Center).show(|ui| {
                                        ui.container().width(70.0).show(|ui| {
                                            ui.text("Position:").size(13.0).show();
                                        });

                                        ui.drag_value(&mut pos_x, "txt_pos_x")
                                            .bg(None)
                                            .border(None)
                                            .prefix("X: ")
                                            .suffix(" px")
                                            .speed(1.0)
                                            .precision(1)
                                            .width(100.0)
                                            .show();

                                        ui.drag_value(&mut pos_y, "txt_pos_y")
                                            .bg(None)
                                            .border(None)
                                            .prefix("Y: ")
                                            .suffix(" px")
                                            .speed(1.0)
                                            .precision(1)
                                            .width(100.0)
                                            .show();
                                    });

                                    ui.spacing(8.0);

                                    ui.container().row().gap(10.0).valign(Align::Center).show(|ui| {
                                        ui.container().width(70.0).show(|ui| {
                                            ui.text("Scale:").size(13.0).show();
                                        });
                                        ui.drag_value(&mut scale, "txt_scale")
                                            .bg(None)
                                            .border(None)
                                            .min(0.0)
                                            .max(1000.0)
                                            .speed(0.5)
                                            .suffix(" %")
                                            .precision(1)
                                            .width(90.0)
                                            .show();

                                        ui.text("Rotation:").size(13.0).show();
                                        ui.drag_value(&mut rotation, "txt_rotation")
                                            .bg(None)
                                            .border(None)
                                            .speed(0.5)
                                            .step(1.0)
                                            .suffix("°")
                                            .precision(0)
                                            .width(80.0)
                                            .show();
                                    });

                                    ui.spacing(8.0);

                                    ui.container().row().gap(10.0).valign(Align::Center).show(|ui| {
                                        ui.container().width(70.0).show(|ui| {
                                            ui.text("Opacity:").size(13.0).show();
                                        });
                                        ui.drag_value(&mut opacity, "txt_opacity")
                                            .bg(None)
                                            .border(None)
                                            .range(0.0, 100.0)
                                            .speed(0.2)
                                            .suffix(" %")
                                            .precision(1)
                                            .width(90.0)
                                            .show();

                                        ui.text("Duration:").size(13.0).show();
                                        ui.drag_value(&mut duration_sec, "txt_duration")
                                            .bg(None)
                                            .border(None)
                                            .min(0.0)
                                            .speed(0.05)
                                            .suffix(" s")
                                            .precision(2)
                                            .width(80.0)
                                            .show();
                                    });
                                });

                            ui.spacing(12.0);

                            ui.container()
                                .fill_x()
                                .padding(14.0, 16.0, 14.0, 16.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("LIVE VALUE READOUT (PURE TEXT)").size(11.0).show();
                                    ui.spacing(10.0);
                                    ui.text(&format!("Position:    X = {:.1} px,  Y = {:.1} px", pos_x, pos_y)).size(13.0).show();
                                    ui.spacing(6.0);
                                    ui.text(&format!("Scale:       {:.1} %", scale)).size(13.0).show();
                                });
                        });
                    });
                });

            *f_id = ui.focused_id;
        };

        // Frame 1: Layout
        run_ui((0.0, 0.0), false, false,
               &layout_cache, &mut next_layout_cache,
               &screen_layout_cache, &mut next_screen_layout_cache,
               &mut interaction_state, &mut focused_id);

        layout_cache = next_layout_cache.clone();
        screen_layout_cache = next_screen_layout_cache.clone();
        next_layout_cache.clear();
        next_screen_layout_cache.clear();

        let scale_id = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            "box_scale".hash(&mut hasher);
            Id::from_u64(hasher.finish() & 0x00FF_FFFF_FFFF_FFFF)
        };

        for (id, rect) in &screen_layout_cache {
            println!("  cache id: {:?} -> {:?}", id, rect);
        }
        let scale_rect = screen_layout_cache.get(&scale_id).copied().expect("box_scale in layout cache");
        println!("scale_rect: {:?}", scale_rect);
        let click_x = scale_rect.origin.x + scale_rect.size.width / 2.0;
        let click_y = scale_rect.origin.y + scale_rect.size.height / 2.0;
        println!("click position: ({}, {})", click_x, click_y);

        // Frame 2: MouseDown
        run_ui((click_x, click_y), true, true,
               &layout_cache, &mut next_layout_cache,
               &screen_layout_cache, &mut next_screen_layout_cache,
               &mut interaction_state, &mut focused_id);

        layout_cache = next_layout_cache.clone();
        screen_layout_cache = next_screen_layout_cache.clone();
        next_layout_cache.clear();
        next_screen_layout_cache.clear();

        // Frame 3: MouseUp
        run_ui((click_x, click_y), false, false,
               &layout_cache, &mut next_layout_cache,
               &screen_layout_cache, &mut next_screen_layout_cache,
               &mut interaction_state, &mut focused_id);

        let edit_mode_key = Id::from_u64(((scale_id.raw() & 0x00FF_FFFF_FFFF_FFFF) << 8) | 0x11);
        let in_edit_mode = interaction_state.get(&edit_mode_key).copied().unwrap_or(0.0) > 0.5;
        assert!(in_edit_mode, "box_scale must enter edit mode after click!");

        layout_cache = next_layout_cache.clone();
        screen_layout_cache = next_screen_layout_cache.clone();
        next_layout_cache.clear();
        next_screen_layout_cache.clear();

        // Frame 4: Redraw while editing
        run_ui((click_x, click_y), false, false,
               &layout_cache, &mut next_layout_cache,
               &screen_layout_cache, &mut next_screen_layout_cache,
               &mut interaction_state, &mut focused_id);

        let in_edit_mode_f4 = interaction_state.get(&edit_mode_key).copied().unwrap_or(0.0) > 0.5;
        assert!(in_edit_mode_f4, "box_scale must STAY in edit mode in Frame 4!");
    }
}
