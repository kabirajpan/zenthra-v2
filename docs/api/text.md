# Text API Specification & Unified Styling Architecture

`Text` is Zenthra's core typographic display widget. Powered by the `cosmic-text` layout and shaping engine, it provides rich text styling, font weight and family control, automated wrapping and truncation (ellipsis), horizontal/vertical alignment, interactive mouse selection with clipboard copy, fixed dimensional constraints, and unified container-style decoration (backgrounds, borders, corner radii, and drop shadows).

The `Text` widget strictly adheres to the unified styling contract established in [`docs/api/container.md`](./container.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Text API Reference](#2-complete-text-api-reference)
   - [2.1 Construction & Quick Headings](#21-construction--quick-headings)
   - [2.2 Typography & Fonts](#22-typography--fonts)
   - [2.3 Text Alignment & Justification](#23-text-alignment--justification)
   - [2.4 Wrapping & Truncation (Ellipsis)](#24-wrapping--truncation-ellipsis)
   - [2.5 Dimensions & Width Constraints](#25-dimensions--width-constraints)
   - [2.6 Inner Padding & Outer Margin](#26-inner-padding--outer-margin)
   - [2.7 Unified Styling: Backgrounds, Borders & Radii](#27-unified-styling-backgrounds-borders--radii)
   - [2.8 Shadows & Elevation](#28-shadows--elevation)
   - [2.9 Selection & Clipboard Integration](#29-selection--clipboard-integration)
   - [2.10 Pointer Cursors & Render Modes](#210-pointer-cursors--render-modes)
   - [2.11 Show & Response](#211-show--response)
3. [Production Recipes & Patterns](#3-production-recipes--patterns)
   - [Recipe 1: Status / Category Badge (Tag Style)](#recipe-1-status--category-badge-tag-style)
   - [Recipe 2: Outline Pill Badge (Transparent Background)](#recipe-2-outline-pill-badge-transparent-background)
   - [Recipe 3: Aligned Property Label in Rows](#recipe-3-aligned-property-label-in-rows)
   - [Recipe 4: Truncated Card Header with Ellipsis](#recipe-4-truncated-card-header-with-ellipsis)
   - [Recipe 5: Multi-line Selectable Body Copy](#recipe-5-multi-line-selectable-body-copy)
4. [Anti-Patterns & Troubleshooting](#4-anti-patterns--troubleshooting)
5. [Type Definitions](#5-type-definitions)

---

## 1. Design Philosophy & Mental Model

### The Core Mental Model
In Zenthra, `Text` is versatile:
- **Plain Text**: When invoked simply (e.g. `ui.text("Hello").show()`), it emits a single `TextDraw` command with zero extra quad overhead.
- **Decorated Text (Badges, Tags & Chips)**: When styled with `.bg(...)`, `.border(...)`, or `.shadow(...)`, it automatically synthesizes a background quad, eliminating the need to wrap simple labels in separate containers.
- **Unified Styling**: Conforms to the standard Zenthra styling contract where properties accept concrete values or `None`.

```rust
// ✅ Clean, unified tag styling without extra container nesting:
ui.text("PRO")
    .font_size(10.0)
    .bold()
    .bg(Color::rgb(0.95, 0.65, 0.20))
    .border((Color::rgb(1.0, 0.8, 0.4), 1.0))
    .radius_all(3.0)
    .padding(2.0, 6.0, 2.0, 6.0)
    .show();
```

---

## 2. Complete Text API Reference

### 2.1 Construction & Quick Headings

#### Method Signatures

```rust
// On Ui:
pub fn text(&mut self, content: &str) -> TextBuilder<'_, 'a>
pub fn h1(&mut self, content: &str) -> TextBuilder<'_, 'a>   // 40px bold
pub fn h2(&mut self, content: &str) -> TextBuilder<'_, 'a>   // 32px bold
pub fn h3(&mut self, content: &str) -> TextBuilder<'_, 'a>   // 24px bold
pub fn h4(&mut self, content: &str) -> TextBuilder<'_, 'a>   // 20px bold

// On TextBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
pub fn pos(mut self, x: f32, y: f32) -> Self
```

#### Description
- `ui.text("Label")`: Standard text constructor. Automatically hashes content and container hierarchy into a deterministic `Id`.
- `ui.h1("Title")`: Convenience shorthand for `ui.text(content).font_size(40.0).bold()`.
- `ui.h2("Subtitle")`: Convenience shorthand for `ui.text(content).font_size(32.0).bold()`.
- `.pos(x, y)`: Explicit coordinate override if placing outside automatic flow.

---

### 2.2 Typography & Fonts

#### Method Signatures

```rust
pub fn font_size(mut self, s: f32) -> Self

pub fn color(mut self, c: Color) -> Self
pub fn text_color(self, c: Color) -> Self // Alias for .color()

pub fn weight(mut self, w: impl Into<FontWeight>) -> Self
pub fn bold(mut self) -> Self
pub fn bold_when(mut self, bold: bool) -> Self
pub fn italic(mut self) -> Self
pub fn italic_when(mut self, italic: bool) -> Self

pub fn underline(mut self, underline: bool) -> Self
pub fn underline_color(mut self, color: Color) -> Self

pub fn strikethrough(mut self, strikethrough: bool) -> Self
pub fn strikethrough_color(mut self, color: Color) -> Self

pub fn family(mut self, f: impl Into<String>) -> Self
pub fn monospace(mut self) -> Self

pub fn line_height(mut self, lh: f32) -> Self
pub fn highlight(mut self, c: Color) -> Self
```

#### Defaults
- Base font size: `16.0 * ui.font_scale`
- Font weight: `FontWeight::Normal`
- Font family: System UI default
- Color: `Color::WHITE`
- Line height: `1.0`

#### Examples

```rust
ui.text("Mono Code")
    .monospace()
    .font_size(13.0)
    .color(Color::rgb(0.40, 0.80, 0.60))
    .show();
```

---

### 2.3 Text Alignment & Justification

#### Method Signatures

```rust
pub fn align(mut self, alignment: Align) -> Self
pub fn valign(mut self, alignment: Align) -> Self
```

#### Description
- Controls alignment within the measured or explicitly constrained layout box (`.width(...)`).
- Supported horizontal alignments: `Align::Left`, `Align::Center`, `Align::Right`.
- Supported vertical alignments: `Align::Top`, `Align::Center`, `Align::Bottom`.

```rust
// Center-aligned title inside a 300px block:
ui.text("Centered Title")
    .width(300.0)
    .align_center()
    .show();
```

---

### 2.4 Wrapping & Truncation (Ellipsis)

#### Method Signatures

```rust
pub fn wrap(mut self, strategy: impl Into<TextWrap>) -> Self
pub fn auto_wrap(mut self) -> Self  // TextWrap::Word
pub fn no_wrap(mut self) -> Self    // TextWrap::None

pub fn ellipsis(mut self, enabled: bool) -> Self
```

#### Description
- `auto_wrap()`: Wraps words automatically when they reach the available container width or `.max_width(...)`.
- `no_wrap()`: Prevents line breaks, letting text overflow or clip horizontally.
- `ellipsis(bool)`: Truncates overflowing text with an ellipsis (`…`).

```rust
ui.text("A very long item path that might exceed the available sidebar width")
    .max_width(220.0)
    .ellipsis(true)
    .show();
```

---

### 2.5 Dimensions & Width Constraints

#### Method Signatures

```rust
pub fn width(mut self, w: f32) -> Self
pub fn height(mut self, h: f32) -> Self

pub fn min_width(mut self, w: f32) -> Self
pub fn max_width(mut self, w: f32) -> Self
pub fn fill_x(mut self, enabled: bool) -> Self
```

#### Dimensional Model
1. **Explicit Width (`.width(w)`)**:
   - Forces the text widget's layout box and cursor advance to exactly `w`.
   - Essential for aligning labels in property rows (e.g. `ui.text("Position:").width(70.0).show()`).
   - Inner text layout width is automatically constrained to `w - padding`.
2. **Fill Width (`.fill_x(true)`)**: Expands width to span the remaining width of the current container.
3. **Min / Max Constraints**: Sets soft bounds for dynamic text lengths.

---

### 2.6 Inner Padding & Outer Margin

#### Method Signatures

```rust
// Inner Padding (between text content and background quad):
pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self
pub fn padding_all(mut self, p: f32) -> Self
pub fn padding_x(mut self, p: f32) -> Self
pub fn padding_y(mut self, p: f32) -> Self
pub fn padding_top(mut self, p: f32) -> Self
pub fn padding_bottom(mut self, p: f32) -> Self
pub fn padding_left(mut self, p: f32) -> Self
pub fn padding_right(mut self, p: f32) -> Self

// Outer Margin (spacing around the widget layout box):
pub fn margin(mut self, t: f32, r: f32, b: f32, l: f32) -> Self
pub fn margin_all(mut self, m: f32) -> Self
pub fn margin_x(mut self, m: f32) -> Self
pub fn margin_y(mut self, m: f32) -> Self
pub fn margin_top(mut self, m: f32) -> Self
pub fn margin_bottom(mut self, m: f32) -> Self
pub fn margin_left(mut self, m: f32) -> Self
pub fn margin_right(mut self, m: f32) -> Self
```

---

### 2.7 Unified Styling: Backgrounds, Borders & Radii

#### Method Signatures

```rust
pub fn bg(mut self, c: impl Into<Option<Color>>) -> Self
pub fn border(mut self, border: impl Into<zenthra_core::Border>) -> Self

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

#### Behavior & Unified Rules
- `.bg(...)`: Accepts `Color` or `None`. Default: `None`.
- `.border(...)`: Accepts `(Color, f32)` or `None`. Default: `None`.
- When either `bg`, `border`, or `shadow` is configured, `Text` automatically renders a background quad with `color: self.bg_color.unwrap_or(Color::TRANSPARENT)`.
- If no background, border, or shadow is set, no quad is emitted (plain text performance).

```rust
// Outline Tag (Transparent Background, White Border):
ui.text("FEATURED")
    .font_size(11.0)
    .border((Color::WHITE, 1.0))
    .radius_all(4.0)
    .padding(2.0, 6.0, 2.0, 6.0)
    .show();
```

---

### 2.8 Shadows & Elevation

#### Method Signatures

```rust
pub fn shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self
pub fn shadow_opacity(mut self, opacity: f32) -> Self
pub fn opacity(mut self, o: f32) -> Self
```

#### Description
- `shadow`: Emits a soft box shadow beneath the decorated text badge.
- `shadow_opacity`: Alpha multiplier for the shadow color (default: `1.0`).
- `opacity`: Alpha multiplier for the text and its decorations.

---

### 2.9 Selection & Clipboard Integration

#### Method Signatures

```rust
pub fn selectable(mut self, enabled: bool) -> Self
```

#### Description
When `.selectable(true)` is enabled:
- The cursor switches to an I-beam text pointer (`CursorIcon::Text`) on hover.
- Users can click and drag to select text ranges.
- Double-clicking any word selects the full word.
- Selected text displays a vertically centered highlight overlay.
- Pressing `Ctrl+C` copies the highlighted substring directly to the system clipboard.

```rust
ui.text("Click and drag to copy this license key: 4X9B-89A1-ZZ42")
    .monospace()
    .selectable(true)
    .show();
```

---

### 2.10 Pointer Cursors & Render Modes

#### Method Signatures

```rust
pub fn cursor(mut self, c: CursorIcon) -> Self
pub fn render_mode(mut self, mode: zenthra_core::RenderMode) -> Self
pub fn clip_rect(mut self, x: f32, y: f32, w: f32, h: f32) -> Self
```

---

### 2.11 Show & Response

#### Method Signature

```rust
pub fn show(self) -> (zenthra_core::Response, Option<ShapedBuffer>)
```

#### Return Values
- `Response`: Standard interaction flags (`res.clicked`, `res.hovered`, `res.pressed`). Allows using `ui.text(...)` as clickable links or breadcrumbs.
- `Option<ShapedBuffer>`: Internal cosmic-text shaped glyph buffer containing shaped layout metrics, glyph cluster coordinates, and bounding sizes.

```rust
let (res, _) = ui.text("View documentation ->")
    .color(Color::rgb(0.40, 0.75, 1.0))
    .cursor(CursorIcon::Pointer)
    .show();

if res.clicked {
    open_docs_link();
}
```

---

## 3. Production Recipes & Patterns

### Recipe 1: Status / Category Badge (Tag Style)

A high-contrast badge chip directly from [`docs/api/container.md`](./container.md) Section 3.5:

```rust
ui.text("PRO")
    .font_size(10.0)
    .bold()
    .bg(Color::rgb(0.95, 0.65, 0.20))
    .border((Color::rgb(1.0, 0.8, 0.4), 1.0))
    .radius_all(3.0)
    .padding(2.0, 6.0, 2.0, 6.0)
    .show();
```

---

### Recipe 2: Outline Pill Badge (Transparent Background)

A sleek, modern outline badge with a transparent surface and rounded pill corners:

```rust
ui.text("v2.4.0")
    .font_size(11.0)
    .color(Color::rgb(0.75, 0.80, 0.90))
    .bg(None)
    .border((Color::rgb(0.35, 0.40, 0.50), 1.0))
    .radius_all(12.0)
    .padding(3.0, 8.0, 3.0, 8.0)
    .show();
```

---

### Recipe 3: Aligned Property Label in Rows

Aligning fixed-width labels in an inspector panel row:

```rust
ui.row().spacing(8.0).valign(Align::Center).show(|ui| {
    ui.text("Position:").width(70.0).color(Color::rgb(0.7, 0.7, 0.8)).show();
    ui.drag_value(&mut pos_x).speed(0.1).show();
    ui.drag_value(&mut pos_y).speed(0.1).show();
});

ui.row().spacing(8.0).valign(Align::Center).show(|ui| {
    ui.text("Scale:").width(70.0).color(Color::rgb(0.7, 0.7, 0.8)).show();
    ui.drag_value(&mut scale).speed(0.01).show();
});
```

---

### Recipe 4: Truncated Card Header with Ellipsis

Preventing long text titles from breaking parent container layouts:

```rust
ui.text(&document_title)
    .font_size(15.0)
    .bold()
    .width(240.0)
    .no_wrap()
    .ellipsis(true)
    .show();
```

---

### Recipe 5: Multi-line Selectable Body Copy

A readable, selectable paragraph block with comfortable line height and word wrapping:

```rust
ui.text("Zenthra provides zero-overhead GPU UI rendering with unified declarative ergonomics. All widgets adhere to consistent design tokens across immediate-mode trees.")
    .font_size(14.0)
    .line_height(1.4)
    .color(Color::rgb(0.80, 0.85, 0.92))
    .auto_wrap()
    .selectable(true)
    .show();
```

---

## 4. Anti-Patterns & Troubleshooting

### Anti-Pattern 1: Nesting Text Inside a Container Just for Badges
**Wrong:**
```rust
// ❌ Redundant container wrapper for a simple badge:
ui.container()
    .bg(Color::RED)
    .radius_all(4.0)
    .padding(2.0, 4.0, 2.0, 4.0)
    .show(|ui| {
        ui.text("NEW").show();
    });
```
**Right:**
```rust
// ✅ Text directly supports background, border, radius, and padding:
ui.text("NEW")
    .bg(Color::RED)
    .radius_all(4.0)
    .padding(2.0, 4.0, 2.0, 4.0)
    .show();
```

### Anti-Pattern 2: Manually Padding with Space Characters for Alignment
**Wrong:**
```rust
// ❌ Fragile spacing using string whitespace:
ui.text("Scale:     ").show();
ui.text("Position:  ").show();
```
**Right:**
```rust
// ✅ Deterministic layout box width:
ui.text("Scale:").width(70.0).show();
ui.text("Position:").width(70.0).show();
```

---

## 5. Type Definitions

```rust
// Defined in zenthra-text:
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

pub enum TextWrap {
    None,
    Glyph,
    Word,
}

pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

// Defined in zenthra-core:
pub enum Align {
    Start,
    Center,
    End,
    Left,
    Right,
    Top,
    Bottom,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub struct Response {
    pub clicked: bool,
    pub hovered: bool,
    pub pressed: bool,
    pub submitted: bool,
    pub changed: bool,
}
```
