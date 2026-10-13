# Zenthra Widget API Audit & Unification Tracker (`fixed.md`)

This living document tracks the canonical methods and remaining tasks for Zenthra's core widgets (`Container`, `Button`, `Text`, `Input`, `TextArea`, and `macros.rs`).

It lists:
1. **Methods to Keep** (Already good and canonical, strictly aligned with [`docs/api/container.md`](./container.md) and [`docs/api/macros.md`](./macros.md)).
2. **Methods & Issues to Remove / Fix** (Pending action items).

---

## 1. Pending Action Items

- [ ] **`macros.rs`**: Remove legacy `.bg_gradient(...)` helper once all external widgets (window/panel/card) migrate to `.bg(Gradient::linear(...))`.
- [ ] **`TextArea`**: Fix multiline scroll offset hit-testing calculation (resolving line 6 vs line 7 click discrepancy).

---

## 2. Macros (`crates/zenthra-widgets/src/macros.rs`)

> **Full Specification**: See [`docs/api/macros.md`](./macros.md) for the complete method catalog and user endpoint reference.

### Active Macro Catalog

| Macro | Method | Status | Lock | Notes |
| :--- | :--- | :---: | :---: | :--- |
| `impl_dimensions` | `.width(w: f32)` | **Keep** | 🔒 Locked | Canonical explicit width setter. |
| `impl_dimensions` | `.height(h: f32)` | **Keep** | 🔒 Locked | Canonical explicit height setter. |
| `impl_dimensions` | `.fill_x()` | **Keep** | 🔒 Locked | Canonical horizontal expansion trigger. |
| `impl_dimensions` | `.fill_y()` | **Keep** | 🔒 Locked | Canonical vertical expansion trigger. |
| `impl_dimensions` | `.fill()` | **Keep** | 🔒 Locked | Canonical dual-axis expansion shorthand. |
| `impl_dimensions` | `.min_width(w)` / `.max_width(w)` | **Keep** | 🔒 Locked | Canonical min/max width constraints. |
| `impl_dimensions` | `.min_height(h)` / `.max_height(h)` | **Keep** | 🔒 Locked | Canonical min/max height constraints. |
| `impl_padding` | `.padding(t, r, b, l)` | **Keep** | 🔒 Locked | Canonical 4-edge padding setter. |
| `impl_padding` | `.padding_all(p)` | **Keep** | 🔒 Locked | Canonical uniform padding setter. |
| `impl_padding` | `.padding_x(p)` / `.padding_y(p)` | **Keep** | 🔒 Locked | Canonical horizontal/vertical padding setters. |
| `impl_padding` | `.padding_top/bottom/left/right(p)` | **Keep** | 🔒 Locked | Canonical individual edge setters. |
| `impl_radius` | `.radius(tl, tr, br, bl)` | **Keep** | 🔒 Locked | Canonical 4-corner radius setter. |
| `impl_radius` | `.radius_all(r)` | **Keep** | 🔒 Locked | Canonical uniform radius setter. |
| `impl_radius` | `.radius_top/bottom/left/right(r)` | **Keep** | 🔒 Locked | Canonical paired corner setters. |
| `impl_radius` | Individual corner setters (`radius_top_left`, etc.) | **Keep** | 🔒 Locked | Canonical individual corner setters. |
| `impl_visuals` | `.style(BoxStyle)` | **Keep** | 🔒 Locked | Canonical raw `BoxStyle` replacement. |
| `impl_visuals` | `.bg(impl Into<Background>)` | **Keep** | 🔒 Locked | Unified background setter (`Solid`, `Gradient`, `None`). |
| `impl_visuals` | `.bg_gradient(start, end, dir)` | **Remove** | Pending | Legacy Duplicate: `.bg(Gradient::linear(...))` handles this cleanly. |
| `impl_visuals` | `.bg_opacity(opacity: f32)` | **Keep** | 🔒 Locked | Direct alpha adjustment for background. |
| `impl_visuals` | `.border(impl Into<Border>)` | **Keep** | 🔒 Locked | Unified border setter (`(Color, width)`, `Border::Solid`, `Border::None`). |
| `impl_visuals` | `.border_alignment(alignment)` | **Keep** | Review | Border inside/center/outside positioning. |
| `impl_visuals` | `.shadow(color, x, y, blur)` | **Keep** | 🔒 Locked | Canonical drop shadow setter. |
| `impl_visuals` | `.shadow_opacity(opacity)` | **Keep** | 🔒 Locked | Canonical shadow alpha setter. |
| `impl_visuals` | `.opacity(o: f32)` | **Keep** | 🔒 Locked | Composite opacity factor. |
| `impl_font_style_methods` | **`.size(size: f32)`** | **Keep** | 🔒 **Locked** | Canonical font size in points. Prefix-free, no collision. |
| `impl_font_style_methods` | **`.color(Color)`** | **Keep** | 🔒 **Locked** | Canonical text glyph color. Canonical single method. |
| `impl_font_style_methods` | `.font(&str)` / `.font_family(&str)` | **Keep** | Review | Font family specification. |
| `impl_font_style_methods` | `.weight(...)` / `.bold()` | **Keep** | Review | Weight specification. |
| `impl_font_style_methods` | `.line_height(lh: f32)` | **Keep** | Review | Multiplier for line box height. |
| `impl_text_wrap_methods` | `.wrap(TextWrap)` | **Keep** | 🔒 Locked | Canonical text wrap strategy. |
| `impl_text_decorations` | `.ellipsis(bool)` | **Keep** | 🔒 Locked | Truncation with trailing dots. |
| `impl_text_decorations` | `.decoration(TextDecoration)` | **Keep** | Review | Unified text decoration (Italic, Underline, Strikethrough, Highlight). |

---

## 3. Container (`crates/zenthra-widgets/src/container.rs`)

`Container` is Zenthra's primary structural layout widget and the canonical design blueprint.

### Active Method Table

| Method Signature | Category | Status | Notes |
| :--- | :--- | :---: | :--- |
| `pub fn new(ui: &mut Ui) -> Self` | Constructor | **Keep** | Automatic deterministic ID assignment. |
| `pub fn id(self, id: impl Hash) -> Self` | Identity | **Keep** | Custom ID hash override. |
| `pub fn pos(self, x: f32, y: f32) -> Self` | Layout | **Keep** | Custom coordinate placement. |
| `pub fn absolute(self, x: f32, y: f32) -> Self` | Layout | **Keep** | Sets coordinates and marks `is_absolute = true`. |
| `pub fn row(self) -> Self` | Flow Direction | **Keep** | Sets `Direction::Row`. |
| `pub fn column(self) -> Self` | Flow Direction | **Keep** | Sets `Direction::Column` (default). |
| `pub fn stack(self) -> Self` | Flow Direction | **Keep** | Sets `Direction::Stack` (layered z-stacking). |
| `pub fn gap(self, g: f32) -> Self` | Layout | **Keep** | Spacing between sequential children. |
| `pub fn align(self, align: Align) -> Self` | Alignment | **Keep** | Sets both horizontal and vertical alignment. |
| `pub fn halign(self, align: Align) -> Self` | Alignment | **Keep** | Sets horizontal child alignment. |
| `pub fn valign(self, align: Align) -> Self` | Alignment | **Keep** | Sets vertical child alignment. |
| `pub fn wrap(self, strategy: Wrap) -> Self` | Wrapping | **Keep** | Sets flex wrapping strategy. |
| `pub fn auto_wrap(self) -> Self` | Wrapping | **Keep** | Shorthand for `Wrap::Auto`. |
| `pub fn no_wrap(self) -> Self` | Wrapping | **Keep** | Shorthand for `Wrap::NoWrap`. |
| `pub fn scroll_x(self, enabled: bool) -> Self` | Scrolling | **Keep** | Enables horizontal scroll clamping. |
| `pub fn scroll_y(self, enabled: bool) -> Self` | Scrolling | **Keep** | Enables vertical scroll clamping. |
| `pub fn scroll(self, enabled: bool) -> Self` | Scrolling | **Keep** | Enables dual-axis scrolling. |
| `pub fn show_scrollbar_x/y(self, bool) -> Self` | Scrollbar | **Keep** | Controls visual thumb rendering. |
| `pub fn hide_scrollbars(self) -> Self` | Scrollbar | **Keep** | Hides thumbs while keeping wheel scrolling. |
| `pub fn wheel_fallback_x(self, bool) -> Self` | Scrollbar | **Keep** | Maps vertical wheel to horizontal scroll when `scroll_x` only. |
| `pub fn clip(self, enabled: bool) -> Self` | Clipping | **Keep** | Force-enables bounding clip. |
| `pub fn no_clip(self) -> Self` | Clipping | **Keep** | Force-disables bounding clip. |
| `pub fn hover_bg(self, bg: impl Into<Background>)` | States | **Keep** | Overrides surface background on hover. |
| `pub fn active_bg(self, bg: impl Into<Background>)` | States | **Keep** | Overrides surface background on press. |
| `pub fn hover_border(self, border: impl Into<Border>)` | States | **Keep** | Overrides border on hover. |
| `pub fn active_border(self, border: impl Into<Border>)` | States | **Keep** | Overrides border on press. |
| `pub fn hover_scale(self, scale: f32) -> Self` | Micro-Interaction | **Keep** | Interactive hover transform scale. |
| `pub fn active_scale(self, scale: f32) -> Self` | Micro-Interaction | **Keep** | Interactive press transform scale. |
| `pub fn backdrop_blur(self, radius: f32) -> Self` | Glassmorphism | **Keep** | Frosted glass backdrop blur filter. |
| `pub fn backdrop_filter(self, filter: BackdropFilter)` | Filter | **Keep** | Advanced composited backdrop filter. |
| `pub fn cursor(self, icon: CursorIcon) -> Self` | Interaction | **Keep** | Sets hover cursor icon. |
| `pub fn interactive(self, bool) -> Self` | Interaction | **Keep** | Captures mouse pointer events. |
| `pub fn on_click / on_hover / on_scroll / on_event` | Events | **Keep** | Standard event listeners. |
| `pub fn render_mode(self, mode: RenderMode)` | Rendering | **Keep** | Pushes render mode to UI stack. |
| `pub fn show<F>(self, f: F) -> Response` | Execution | **Keep** | Evaluates child closure, handles layout and clipping. |
| `BoxStyle` methods via macro | Box Model | **Keep** | Generated by `impl_box_style_methods!`. |

---

## 4. Button (`crates/zenthra-widgets/src/button.rs`)

`Button` is Zenthra's primary interactive action trigger. It measures text typography and manages hover/active press transitions.

### Active Method Table

| Method Signature | Status | Notes |
| :--- | :---: | :--- |
| `ui.button("Label")` | **Keep** | Standard constructor via `Ui`. |
| `.id(impl Hash)` | **Keep** | Explicit ID override. |
| `.pos(x, y)` | **Keep** | Explicit coordinate placement override. |
| `.align(Align)` | **Keep** | Content alignment within button box (`Left`, `Center`, `Right`). |
| `.hover_bg(impl Into<Background>)` | **Keep** | Hover surface override (supports `Color` and `Gradient`). |
| `.active_bg(impl Into<Background>)` | **Keep** | Active press surface override (supports `Color` and `Gradient`). |
| `.hover_border(impl Into<Border>)` | **Keep** | Hover border override. |
| `.active_border(impl Into<Border>)` | **Keep** | Active press border override. |
| `.hover_scale(f32)` | **Keep** | Interactive micro-interaction scale factor. |
| `.render_mode(RenderMode)` | **Keep** | Render mode override. |
| `.on_click(f)` / `.on_hover(f)` / `.on_event(...)` | **Keep** | Standard event listeners. |
| `.show() -> Response` | **Keep** | Measures text first, computes exact bounds, handles hit testing. |
| `BoxStyle` methods via macro | **Keep** | Unified `.width`, `.height`, `.padding`, `.radius_all`, `.bg`, `.border`, `.shadow`. |
| `TextStyle` methods via macro | **Keep** | Unified `.font_size`, `.text_color`, `.weight`, `.wrap`. |

---

## 5. Text (`crates/zenthra-widgets/src/text.rs`)

`Text` is the inline typography and badge widget with text selection and clipboard support.

### Active Method Table

| Method Signature | Status | Notes |
| :--- | :---: | :--- |
| `ui.text("Content")` / `ui.h1..h4` | **Keep** | Standard constructors. |
| `pub fn id(self, id: impl Hash) -> Self` | **Keep** | Deterministic ID override. |
| `pub fn pos(self, x: f32, y: f32) -> Self` | **Keep** | Coordinate placement override (`start_x`, `start_y`). |
| `pub fn selectable(self, enabled: bool) -> Self` | **Keep** | Enables cursor selection and Ctrl+C clipboard copy. |
| `pub fn align(self, alignment: Align) -> Self` | **Keep** | Canonical text alignment (`Left`, `Center`, `Right`). |
| `pub fn valign(self, alignment: Align) -> Self` | **Keep** | Vertical alignment (`Top`, `Center`, `Bottom`). |
| `pub fn cursor(self, c: CursorIcon) -> Self` | **Keep** | Mouse cursor icon. |
| `pub fn clip_rect(self, x, y, w, h) -> Self` | **Keep** | Explicit scissor clip rectangle. |
| `pub fn render_mode(self, mode: RenderMode)` | **Keep** | Render mode stack control. |
| `pub fn show(self) -> (Response, Option<ShapedBuffer>)` | **Keep** | Text layout, badge drawing, selection handling. |
| `BoxStyle` methods via macro | **Keep** | Unified box styling for text badges / chips. |
| `TextStyle` methods via macro | **Keep** | Unified `.font_size`, `.text_color`, `.weight`, `.wrap`, decorations. |

---

## 6. Input (`crates/zenthra-widgets/src/input.rs`)

`Input` is the single-line text input widget with cursor navigation, selection, and focus styling.

### Active Method Table

| Method Signature | Status | Notes |
| :--- | :---: | :--- |
| `ui.input(&mut String)` | **Keep** | Standard constructor binding to mutable string buffer. |
| `pub fn id(self, id: impl Hash) -> Self` | **Keep** | Deterministic ID override. |
| `pub fn pos(self, x: f32, y: f32) -> Self` | **Keep** | Explicit coordinate placement override (parity with Container/Button/Text). |
| `pub fn placeholder(self, impl Into<String>)` | **Keep** | Placeholder text when buffer is empty. |
| `pub fn placeholder_color(self, Color)` | **Keep** | Placeholder text color. |
| `pub fn focus_border(self, impl Into<Border>)` | **Keep** | Border override when input has keyboard focus. |
| `pub fn cursor_color(self, Color)` | **Keep** | Text cursor caret color. |
| `pub fn selection_color(self, Color)` | **Keep** | Selected text highlight background color. |
| `pub fn scrollable(self, bool)` | **Keep** | Horizontal scrolling when text exceeds width. |
| `pub fn autofocus(self, bool)` | **Keep** | Automatically acquires keyboard focus on mount. |
| `pub fn select_all(self, enabled: bool) -> Self` | **Keep** | Selects entire string upon initial focus. |
| `pub fn render_mode(self, RenderMode)` | **Keep** | Render mode stack control. |
| `pub fn on_change(self, FnMut(String))` | **Keep** | Text mutation event listener. |
| `pub fn on_hover(self, FnMut(bool))` | **Keep** | Hover state event listener. |
| `pub fn on_event(self, phase, f)` | **Keep** | Generic event listener. |
| `BoxStyle` methods via macro | **Keep** | Unified `.width`, `.height`, `.padding`, `.radius_all`, `.bg`, `.border`, `.shadow`. |
| `Typography` methods via macro | **Keep** | Unified `.font_size`, `.text_color`, `.font_family`, `.line_height`. |

---

## 7. TextArea (`crates/zenthra-widgets/src/text_area.rs`)

`TextArea` is the multiline text editing widget with word wrap, scrolling, and hit testing.

### Active Method Table

| Method Signature | Status | Notes |
| :--- | :---: | :--- |
| `ui.text_area(&mut String)` | **Keep** | Standard constructor binding to mutable string buffer. |
| `pub fn id(self, id: impl Hash) -> Self` | **Keep** | Deterministic ID override. |
| `pub fn pos(self, x: f32, y: f32) -> Self` | **Keep** | Explicit coordinate placement override (parity with Container/Button/Text). |
| `pub fn placeholder(self, impl Into<String>)` | **Keep** | Multiline placeholder text. |
| `pub fn placeholder_color(self, Color)` | **Keep** | Placeholder text color. |
| `pub fn focus_border(self, impl Into<Border>)` | **Keep** | Border override when text area is focused. |
| `pub fn cursor_color(self, Color)` | **Keep** | Caret cursor color. |
| `pub fn selection_color(self, Color)` | **Keep** | Highlight background for selected text ranges. |
| `pub fn scrollable(self, enabled: bool)` | **Keep** | Enables vertical scrolling within box bounds. |
| `pub fn submit_on_enter(self, bool)` | **Keep** | Submits on Enter instead of inserting newline. |
| `pub fn autofocus(self, bool)` | **Keep** | Automatically focuses on mount. |
| `pub fn select_all(self, enabled: bool) -> Self` | **Keep** | Selects entire string upon initial focus. |
| `pub fn render_mode(self, RenderMode)` | **Keep** | Render mode stack control. |
| `pub fn on_change / on_hover / on_event` | **Keep** | Standard event listeners. |
| Line 6 vs Line 7 hit-test offset calculation | **To Fix** | **Bug**: Scrolling offset discrepancy causes clicks near line boundaries to select the wrong line. |
| `BoxStyle` methods via macro | **Keep** | Unified `.width`, `.height`, `.padding`, `.radius_all`, `.bg`, `.border`, `.shadow`. |
| `Typography` methods via macro | **Keep** | Unified `.font_size`, `.text_color`, `.font_family`, `.line_height`, `.wrap`. |

---

## 8. Cross-Widget Parity Matrix

| Builder Method | `Container` | `Button` | `Text` | `Input` | `TextArea` | Parity Status |
| :--- | :---: | :---: | :---: | :---: | :---: | :--- |
| `.id(hash)` | ✅ | ✅ | ✅ | ✅ | ✅ | **100% Unified** |
| `.pos(x, y)` | ✅ | ✅ | ✅ | ✅ | ✅ | **100% Unified** |
| `BoxStyle` (`width`, `height`, `padding`, `radius`, `bg`, `border`, `shadow`) | ✅ | ✅ | ✅ | ✅ | ✅ | **100% Unified** (via macro) |
| `Typography` (`font_size`, `text_color`, `weight`, `wrap`, etc.) | — | ✅ | ✅ | ✅ | ✅ | **100% Unified** (via macro) |
| Action states (`.hover_bg`, `.active_bg`, `.hover_border`, `.active_border`) | ✅ | ✅ | — | — | — | **Consistent** (Action surfaces only) |
| Text entry states (`.focus_border`, `.selection_color`) | — | — | — | ✅ | ✅ | **100% Unified** (Text entry only) |
| `.scrollable(bool)` | — | — | — | ✅ | ✅ | **100% Unified** |
| `.select_all(bool)` | — | — | — | ✅ | ✅ | **100% Unified** |
