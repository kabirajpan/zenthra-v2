# TextArea API Specification & Unified Styling Architecture

`TextArea` is Zenthra's primary multi-line text editing widget. It provides full multi-line UTF-8 text editing, 2D viewport scrolling (vertical and horizontal), word wrapping, cursor navigation across rows and columns, mouse selection, system clipboard integration (Cut, Copy, Paste), customizable line heights, placeholders, and unified styling (background surfaces, borders, focus rings, corner radii, and drop shadows).

The `TextArea` widget strictly adheres to the unified styling contract established in [`docs/api/container.md`](./container.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete TextArea API Reference](#2-complete-textarea-api-reference)
   - [2.1 Construction & Identification](#21-construction--identification)
   - [2.2 Value & Typography](#22-value--typography)
   - [2.3 Placeholders & Cursor](#23-placeholders--cursor)
   - [2.4 Dimensions, Sizing & Auto-Expand](#24-dimensions-sizing--auto-expand)
   - [2.5 Scrolling, Overflow & Text Wrapping](#25-scrolling-overflow--text-wrapping)
   - [2.6 Backgrounds & Colors](#26-backgrounds--colors)
   - [2.7 Borders & Focus Rings](#27-borders--focus-rings)
   - [2.8 Corner Radii](#28-corner-radii)
   - [2.9 Shadows & Opacity](#29-shadows--opacity)
   - [2.10 Inner Padding & Text Padding](#210-inner-padding--text-padding)
   - [2.11 Enter Key Behavior & Submission](#211-enter-key-behavior--submission)
   - [2.12 Event Handling & Callbacks](#212-event-handling--callbacks)
3. [Keyboard Navigation & Shortcuts](#3-keyboard-navigation--shortcuts)
4. [Production Recipes & Patterns](#4-production-recipes--patterns)
   - [Recipe 1: Code Editor & Script Console](#recipe-1-code-editor--script-console)
   - [Recipe 2: Multi-line Comment & Feedback Box](#recipe-2-multi-line-comment--feedback-box)
   - [Recipe 3: Transparent Inline Note Editor](#recipe-3-transparent-inline-note-editor)
   - [Recipe 4: Chat Message Input (Submit on Enter)](#recipe-4-chat-message-input-submit-on-enter)
   - [Recipe 5: Fixed-Height Scrollable Buffer](#recipe-5-fixed-height-scrollable-buffer)
5. [Anti-Patterns & Troubleshooting](#5-anti-patterns--troubleshooting)
6. [Type Definitions](#6-type-definitions)

---

## 1. Design Philosophy & Mental Model

### The Core Mental Model
In Zenthra, a `TextArea`:
- Binds directly to an immediate-mode `&mut String` buffer.
- Manages multi-line cursor indexing, selection anchors, line heights, and 2D scroll state across frames using stable widget IDs.
- Unlike single-line `Input` (which moves cursor horizontally and submits on `Enter`), `TextArea` supports multi-row cursor traversal (`Up`/`Down`), line wrapping, and inserts newline characters (`\n`).
- Follows the **Unified Styling Contract**: Every aesthetic property accepts concrete values or `None`.
- Renders an underlying background quad unconditionally (`Color::TRANSPARENT` when `.bg(None)` is passed), allowing seamless border-only, focus-ring-only, or drop-shadowed multi-line editors without extra container wrapping.

```rust
// ✅ Code Editor TextArea (Directly from docs/api/container.md Section 3.5):
ui.text_area(&mut code_buffer)
    .width(500.0)
    .height(240.0)
    .bg(Color::rgb(0.08, 0.09, 0.12))
    .border((Color::rgb(0.20, 0.23, 0.30), 1.0))
    .focus_border((Color::rgb(0.35, 0.58, 0.98), 1.5))
    .radius_all(8.0)
    .show();
```

---

## 2. Complete TextArea API Reference

### 2.1 Construction & Identification

#### Method Signatures

```rust
// On Ui:
pub fn text_area<'b>(
    &mut self, 
    buffer: &'b mut String
) -> TextAreaBuilder<'_, 'a, 'b>

pub fn text_area_with_id<'b>(
    &mut self, 
    buffer: &'b mut String, 
    id: impl std::hash::Hash
) -> TextAreaBuilder<'_, 'a, 'b>

// On TextAreaBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
pub fn pos(mut self, x: f32, y: f32) -> Self
```

#### Description
- `ui.text_area(buffer)` initializes a `TextAreaBuilder` with an automatically assigned stable layout ID.
- `ui.text_area_with_id(buffer, id)` allows explicit ID assignment for deterministic state persistence.
- `id` uniquely tracks cursor index, selection range, scroll offsets (`scroll_x`, `scroll_y`), and active focus.

---

### 2.2 Value & Typography

#### Method Signatures

```rust
// Individual Typography Setters
pub fn font_size(mut self, size: f32) -> Self
pub fn color(mut self, color: Color) -> Self
pub fn text_color(self, color: Color) -> Self // Alias for .color()
pub fn line_height(mut self, lh: f32) -> Self
pub fn font_family(mut self, family: impl Into<String>) -> Self
pub fn family(self, family: impl Into<String>) -> Self
pub fn monospace(mut self) -> Self
pub fn font_weight(mut self, weight: impl Into<FontWeight>) -> Self
pub fn weight(self, weight: impl Into<FontWeight>) -> Self
pub fn bold(mut self) -> Self
pub fn font_style(mut self, style: FontStyle) -> Self
pub fn italic(mut self) -> Self

// Bulk Typography Styling
pub fn text_style(mut self, style: TextStyle) -> Self
```

#### Defaults
- `font_size`: `18.0`
- `color`: `Color::WHITE`
- `line_height`: `1.2`
- `wrap`: `TextWrap::Word`

```rust
// Fluent builder typography:
ui.text_area(&mut code, "code_view")
    .monospace()
    .font_size(13.0)
    .line_height(1.4)
    .color(Color::rgb(0.85, 0.90, 0.95))
    .show();

// Bulk TextStyle theme reuse:
ui.text_area(&mut editor_buffer, "editor")
    .text_style(my_editor_font)
    .show();
```

---

### 2.3 Placeholders & Cursor

#### Method Signatures

```rust
pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self
pub fn placeholder_color(mut self, color: Color) -> Self

pub fn cursor_color(mut self, color: Color) -> Self
```

#### Description
- `placeholder`: Rendered when `buffer.is_empty()`. Defaults to dimmed color (`Color::rgba(1.0, 1.0, 1.0, 0.4)`).
- `cursor_color`: Sets the blinking vertical caret color.

---

### 2.4 Dimensions, Sizing & Auto-Expand

#### Method Signatures

```rust
pub fn width(mut self, w: f32) -> Self       // Default: 300.0
pub fn fill_x(mut self) -> Self               // Fills remaining container width

pub fn height(mut self, h: f32) -> Self       // Fixed height
pub fn max_height(mut self, h: f32) -> Self   // Clamps natural height
```

#### Sizing Model
1. **Natural Height (Default)**:
   - When `.height(...)` is omitted, the `TextArea` dynamically calculates its height based on the number of text lines (`natural_h = (h_content + padding).max(36.0)`).
   - If `.max_height(h)` is provided, natural growth stops at `h`, and scrolling activates if `.scrollable(true)` is set.
2. **Fixed Height (`.height(h)`)**:
   - Locks the box height to `h`. Content overflowing `h` scrolls when `.scrollable(true)` is enabled.

---

### 2.5 Scrolling, Overflow & Text Wrapping

#### Method Signatures

```rust
pub fn scrollable(mut self, enabled: bool) -> Self       // Default: false
pub fn overflow_hidden(mut self, enabled: bool) -> Self  // Default: false

pub fn wrap(mut self, wrap: zenthra_text::prelude::TextWrap) -> Self // Default: TextWrap::Word
```

#### Behavior
- `scrollable(true)`: Enables 2D mouse wheel scrolling and automatically clips overflow (`overflow_hidden = true`).
- `wrap`: Supports `TextWrap::Word`, `TextWrap::Glyph`, and `TextWrap::None`.
  - In code editors, using `TextWrap::None` alongside `scrollable(true)` enables horizontal scrolling for long code lines.

---

### 2.6 Backgrounds & Colors

#### Method Signatures

```rust
pub fn bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self
pub fn bg_gradient(mut self, start: Color, end: Color, direction: GradientDirection) -> Self
pub fn bg_opacity(mut self, opacity: f32) -> Self

// Bulk Box Styling
pub fn style(mut self, style: BoxStyle) -> Self
```

#### Unified Contract
- `bg`: Sets the background surface quad. Accepts solid colors or `Background::Solid(Color)`, `Background::Gradient(...)`, or `None` for transparency. Default: `Some(Background::Solid(Color::rgb(0.2, 0.2, 0.2)))`.
- `bg_gradient`: Sets a linear or radial gradient surface.
- `bg_opacity`: Controls alpha transparency of the background.
- When `.bg(None)` is set, borders, focus rings, and drop shadows continue to render properly.
- `style`: Applies a complete, reusable `BoxStyle` theme.

---

### 2.7 Borders & Focus Rings

#### Method Signatures

```rust
pub fn border(mut self, border: impl Into<zenthra_core::Border>) -> Self
pub fn border_alignment(mut self, alignment: BorderAlignment) -> Self
pub fn focus_border(mut self, border: impl Into<zenthra_core::Border>) -> Self
```

#### Behavior & Unified Contract
- `border(...)`:
  - Accepts `(Color, f32)` (e.g. `(Color::WHITE, 1.0)`) or `Border::Solid(color, width)`.
  - Accepts `None` or `Border::None` to remove the border.
  - Default: `None` (0.0 width).
- `border_alignment(...)`:
  - Position of border relative to box boundary (`Inside`, `Center`, or `Outside`).
- `focus_border(...)`:
  - Applied automatically whenever the text area is focused.
  - Accepts `(Color, f32)` or `Border::Solid(color, width)`.
  - Accepts `None` or `Border::None` to **explicitly disable** the focus border.
  - If omitted and focused, defaults to `1.0` width accent blue (`Color::rgb(0.4, 0.7, 1.0)`).

```rust
// Clean subtle border with vibrant focus ring:
ui.text_area(&mut notes, "notes")
    .border((Color::rgb(0.20, 0.22, 0.28), 1.0))
    .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
    .show();
```

---

### 2.8 Corner Radii

#### Method Signatures

```rust
pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self
pub fn radius_all(mut self, r: f32) -> Self
pub fn radius_top(mut self, r: f32) -> Self
pub fn radius_bottom(mut self, r: f32) -> Self
pub fn radius_left(mut self, r: f32) -> Self
pub fn radius_right(mut self, r: f32) -> Self
pub fn radius_top_left(mut self, r: f32) -> Self
pub fn radius_top_right(mut self, r: f32) -> Self
pub fn radius_bottom_right(mut self, r: f32) -> Self
pub fn radius_bottom_left(mut self, r: f32) -> Self
```

#### Defaults
- Default: `[4.0; 4]` (uniform 4px rounded corners).

---

### 2.9 Shadows & Opacity

#### Method Signatures

```rust
pub fn shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self
pub fn shadow_opacity(mut self, opacity: f32) -> Self
pub fn opacity(mut self, o: f32) -> Self
```

---

### 2.10 Inner Padding (Unified BoxStyle)

#### Method Signatures

```rust
pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self
pub fn padding_all(mut self, p: f32) -> Self
pub fn padding_x(mut self, x: f32) -> Self
pub fn padding_y(mut self, y: f32) -> Self
pub fn padding_top(mut self, t: f32) -> Self
pub fn padding_bottom(mut self, b: f32) -> Self
pub fn padding_left(mut self, l: f32) -> Self
pub fn padding_right(mut self, r: f32) -> Self
```

Padding is fully unified into `BoxStyle::padding` (defaults to `6.0` vertical, `8.0` horizontal). Text selection boxes, cursors, and shaped glyphs automatically align with box padding without manual offsets.

---

### 2.11 Enter Key Behavior & Submission

#### Method Signatures

```rust
pub fn submit_on_enter(mut self, enabled: bool) -> Self // Default: false
```

#### Behavior
- `submit_on_enter(false)` (Default): Pressing `Enter` inserts a newline character (`\n`) into the buffer.
- `submit_on_enter(true)` (Chat Style): Pressing `Enter` dispatches `WidgetEvent::Submit` (`res.submitted == true`). Pressing `Shift+Enter` inserts a newline character.

---

### 2.12 Event Handling & Callbacks

#### Method Signatures

```rust
pub fn on_change<F>(self, f: F) -> Self
where
    F: FnMut(String) + 'a

pub fn on_hover<F>(self, f: F) -> Self
where
    F: FnMut(bool) + 'a

pub fn on_event<F>(self, f: F) -> Self
where
    F: FnMut(crate::ui::WidgetEvent) + 'a

pub fn selection_color(mut self, color: impl Into<Color>) -> Self
pub fn autofocus(mut self, enabled: bool) -> Self
pub fn select_all(mut self, enabled: bool) -> Self

pub fn show(self) -> zenthra_core::Response
```

#### Response Flags
- `res.clicked`: Mouse clicked inside editor.
- `res.hovered`: Mouse pointer is hovering over editor.
- `res.pressed`: Mouse button is actively pressed.
- `res.changed`: Content buffer changed this frame.
- `res.submitted`: Dispatched when `Enter` is pressed with `submit_on_enter(true)`.

---

## 3. Keyboard Navigation & Shortcuts

| Shortcut | Action |
| :--- | :--- |
| `Left` / `Right` | Move cursor backward / forward one character |
| `Up` / `Down` | Move cursor to previous / next line at matching column |
| `Ctrl` / `Alt` + `Left` / `Right` | Jump to previous / next word boundary |
| `Shift` + `Arrow Keys` | Expand / shrink selection range across characters and lines |
| `Home` / `End` | Jump to start / end of current line |
| `Ctrl` + `Home` / `End` | Jump to very beginning / end of entire document |
| `Shift` + `Home` / `End` | Select to start / end of current line |
| `PageUp` / `PageDown` | Scroll viewport up / down by one page |
| `Backspace` | Delete previous character (or delete selected range) |
| `Delete` | Delete next character (or delete selected range) |
| `Ctrl` + `Backspace` | Delete previous word |
| `Ctrl` + `Delete` | Delete next word |
| `Ctrl` + `A` | Select entire buffer |
| `Ctrl` + `C` | Copy selection to clipboard |
| `Ctrl` + `X` | Cut selection to clipboard |
| `Ctrl` + `V` | Paste clipboard at cursor (replaces selection) |
| `Enter` | Insert newline `\n` (or submit if `submit_on_enter(true)`) |
| `Shift` + `Enter` | Always insert newline `\n` |

---

## 4. Production Recipes & Patterns

### Recipe 1: Code Editor & Script Console

A fixed-dimension code editor with dark background, monospaced line spacing, and scrolling:

```rust
ui.text_area(&mut script_code, "script_editor")
    .width(560.0)
    .height(280.0)
    .font_size(13.0)
    .line_height(1.4)
    .scrollable(true)
    .wrap(TextWrap::None) // Horizontal scrolling for long code lines
    .bg(Color::rgb(0.08, 0.09, 0.12))
    .border((Color::rgb(0.18, 0.20, 0.26), 1.0))
    .focus_border((Color::rgb(0.35, 0.60, 0.98), 1.5))
    .radius_all(6.0)
    .padding(10.0, 12.0, 10.0, 12.0)
    .show();
```

---

### Recipe 2: Multi-line Comment & Feedback Box

An auto-expanding comment box that grows naturally with text:

```rust
ui.column().spacing(6.0).show(|ui| {
    ui.text("Feedback / Notes")
        .font_size(13.0)
        .color(Color::rgb(0.70, 0.75, 0.85))
        .show();

    ui.text_area(&mut feedback_text, "feedback_box")
        .fill_x()
        .max_height(200.0) // Natural height up to 200px, then scrolls
        .scrollable(true)
        .bg(Color::rgb(0.12, 0.13, 0.17))
        .border((Color::rgb(0.22, 0.25, 0.32), 1.0))
        .focus_border((Color::rgb(0.40, 0.75, 1.00), 1.5))
        .radius_all(6.0)
        .padding(8.0, 10.0, 8.0, 10.0)
        .placeholder("Write your thoughts here...")
        .show();
});
```

---

### Recipe 3: Transparent Inline Note Editor

A distraction-free, seamless note editor with zero idle borders:

```rust
ui.text_area(&mut scratchpad, "scratchpad")
    .fill_x()
    .height(180.0)
    .font_size(14.0)
    .bg(None)                                           // Transparent surface
    .border(None)                                       // Borderless when idle
    .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.0))   // Subtle focus box
    .radius_all(4.0)
    .padding(6.0, 8.0, 6.0, 8.0)
    .placeholder("Quick scratchpad notes...")
    .show();
```

---

### Recipe 4: Chat Message Input (Submit on Enter)

A chat input field where `Enter` sends the message, while `Shift+Enter` inserts a newline:

```rust
let res = ui.text_area(&mut chat_message, "chat_input")
    .fill_x()
    .height(70.0)
    .submit_on_enter(true) // 👈 Enter submits, Shift+Enter newlines
    .bg(Color::rgb(0.14, 0.15, 0.20))
    .border((Color::rgb(0.24, 0.26, 0.34), 1.0))
    .focus_border((Color::rgb(0.30, 0.70, 0.95), 1.5))
    .radius_all(8.0)
    .placeholder("Type a message... (Press Enter to send)")
    .show();

if res.submitted && !chat_message.trim().is_empty() {
    send_chat_message(chat_message.clone());
    chat_message.clear();
}
```

---

### Recipe 5: Fixed-Height Scrollable Buffer

A constrained log or description viewer:

```rust
ui.text_area(&mut release_notes, "release_notes")
    .width(420.0)
    .height(150.0)
    .scrollable(true)
    .bg(Color::rgb(0.10, 0.11, 0.14))
    .border((Color::rgb(0.20, 0.22, 0.28), 1.0))
    .radius_all(6.0)
    .padding_all(8.0)
    .show();
```

---

## 5. Anti-Patterns & Troubleshooting

### Anti-Pattern 1: Expecting `Enter` to Submit by Default
By default, `TextArea` is a multi-line editor; pressing `Enter` inserts `\n`. If you are building a chat field or single-action box, explicitly call `.submit_on_enter(true)`.

### Anti-Pattern 2: Wrapping in a Container for Scrolling
`TextArea` has built-in viewport management and scroll state. Do NOT wrap it in a scrollable `ui.container().scrollable_y(true)` unless you specifically want the entire widget box to scroll along with other form fields. For internal text scrolling, call `.scrollable(true)` directly on the `TextArea`.

---

## 6. Type Definitions

```rust
// Defined in zenthra-core:
pub enum Border {
    None,
    Solid(Color, f32),
}

pub struct Response {
    pub clicked: bool,
    pub hovered: bool,
    pub pressed: bool,
    pub submitted: bool,
    pub changed: bool,
}
```
