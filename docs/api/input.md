# Input API Specification & Unified Styling Architecture

`Input` is Zenthra's primary single-line text entry widget. It provides full UTF-8 text editing, keyboard navigation, text selection, system clipboard integration (Cut, Copy, Paste), horizontal auto-scrolling for long inputs, placeholder text, and unified styling (backgrounds, borders, focus rings, corner radii, and drop shadows).

The `Input` widget strictly adheres to the unified styling contract established in [`docs/api/container.md`](./container.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Input API Reference](#2-complete-input-api-reference)
   - [2.1 Construction & Identification](#21-construction--identification)
   - [2.2 Value & Typography](#22-value--typography)
   - [2.3 Placeholders & Cursor](#23-placeholders--cursor)
   - [2.4 Backgrounds & Colors](#24-backgrounds--colors)
   - [2.5 Borders & Focus Rings](#25-borders--focus-rings)
   - [2.6 Corner Radii](#26-corner-radii)
   - [2.7 Shadows & Opacity](#27-shadows--opacity)
   - [2.8 Sizing, Width & Padding](#28-sizing-width--padding)
   - [2.9 Scrolling & Selection](#29-scrolling--selection)
   - [2.10 Event Handling & Callbacks](#210-event-handling--callbacks)
3. [Keyboard Navigation & Shortcuts](#3-keyboard-navigation--shortcuts)
4. [Production Recipes & Patterns](#4-production-recipes--patterns)
   - [Recipe 1: Standard Form Input](#recipe-1-standard-form-input)
   - [Recipe 2: Seamless Inline Rename Input](#recipe-2-seamless-inline-rename-input)
   - [Recipe 3: Filter / Search Bar with Drop Shadow](#recipe-3-filter--search-bar-with-drop-shadow)
   - [Recipe 4: Custom Branded Focus Ring](#recipe-4-custom-branded-focus-ring)
   - [Recipe 5: Compact Fixed-Width Property Field](#recipe-5-compact-fixed-width-property-field)
5. [Anti-Patterns & Troubleshooting](#5-anti-patterns--troubleshooting)
6. [Type Definitions](#6-type-definitions)

---

## 1. Design Philosophy & Mental Model

### The Core Mental Model
In Zenthra, an `Input` widget:
- Binds directly to an immediate-mode `&mut String` buffer.
- Maintains interactive cursor position, selection range, and scroll state across frames using stable widget IDs.
- Follows the **Unified Styling Contract**: Every aesthetic property accepts concrete types or `None`.
- Renders an underlying background quad unconditionally (`Color::TRANSPARENT` when `.bg(None)` is passed), allowing seamless border-only or focus-ring-only inline inputs without extra container wrapping.

### The Unified "No-Magic" Styling Contract
Just like `Container`:
- `.bg(Color)` sets a solid surface background; `.bg(None)` makes the surface transparent.
- `.border((Color, width))` sets the default unfocused border; `.border(None)` removes it.
- `.focus_border((Color, width))` customizes the active focus ring; `.focus_border(None)` explicitly disables the focus border.
- `.shadow(color, ox, oy, blur)` and `.shadow_opacity(opacity)` elevate the input off the canvas.

```rust
// ✅ Clean, consistent Zenthra styling:
ui.input(&mut name)
    .bg(Color::rgb(0.12, 0.13, 0.18))
    .border((Color::rgb(0.25, 0.28, 0.38), 1.0))
    .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
    .radius_all(6.0)
    .padding_x(10.0)
    .show();
```

---

## 2. Complete Input API Reference

### 2.1 Construction & Identification

#### Method Signatures

```rust
// In Ui:
pub fn input<'b>(&mut self, buffer: &'b mut String) -> InputBuilder<'_, 'a, 'b>
pub fn input_with_id<'b>(&mut self, buffer: &'b mut String, id: impl std::hash::Hash) -> InputBuilder<'_, 'a, 'b>

// In InputBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
pub fn pos(mut self, x: f32, y: f32) -> Self
```

#### Description
- `ui.input(buffer)` initializes an `InputBuilder` targeting `&mut String` with an automatically assigned ID.
- `ui.input_with_id(buffer, id)` or `.id(...)` allows specifying a custom ID when needed.
- `id` uniquely identifies the input in `Ui`'s state cache (cursor index, selection anchor, horizontal scroll offset, and focus tracking).
- If manual coordinates are needed instead of automatic layout cursor placement, call `.pos(x, y)`.

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

#### Examples

```rust
// Fluent builder typography:
ui.input(&mut search_term)
    .font_family("Inter")
    .bold()
    .font_size(14.0)
    .color(Color::rgb(0.9, 0.9, 0.95))
    .show();

// Bulk TextStyle theme reuse:
ui.input(&mut code_snippet)
    .text_style(my_code_font)
    .show();
```

---

### 2.3 Placeholders & Cursor

#### Method Signatures

```rust
pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self
pub fn placeholder_color(mut self, color: Color) -> Self

pub fn cursor_color(mut self, color: Color) -> Self
pub fn selection_color(mut self, color: Color) -> Self
```

#### Description
- `placeholder`: Shown when `buffer.is_empty()`. Defaults to dimmed text (`color.with_alpha(0.4)`).
- `cursor_color`: Sets the blinking vertical insertion caret color.
- `selection_color`: Sets the background fill color for highlighted/selected text ranges.

```rust
ui.input(&mut email)
    .placeholder("user@example.com")
    .placeholder_color(Color::rgb(0.45, 0.48, 0.55))
    .selection_color(Color::rgba(0.25, 0.50, 0.90, 0.45))
    .show();
```

---

### 2.4 Backgrounds & Colors

#### Method Signatures

```rust
pub fn bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self
pub fn bg_gradient(mut self, start: Color, end: Color, direction: GradientDirection) -> Self
pub fn bg_opacity(mut self, opacity: f32) -> Self

// Bulk Box Styling
pub fn style(mut self, style: BoxStyle) -> Self
```

#### Description
- `bg`: Sets the background quad. Accepts solid colors or `Background::Solid(Color)`, `Background::Gradient(...)`, or `None` for transparency. Default: `Some(Background::Solid(Color::rgb(0.2, 0.2, 0.2)))`.
- `bg_gradient`: Sets a linear or radial gradient surface.
- `bg_opacity`: Controls alpha transparency of the background.
- `style`: Applies a complete, reusable `BoxStyle` theme.

```rust
// Transparent background with subtle border
ui.input(&mut tag)
    .bg(None)
    .border((Color::rgb(0.3, 0.3, 0.35), 1.0))
    .show();

// Reusable BoxStyle
ui.input(&mut code)
    .style(my_dark_input_theme)
    .show();
```

---

### 2.5 Borders & Focus Rings

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
  - Applied automatically whenever this input has keyboard focus (`ui.focused_id == Some(id)`).
  - Accepts `(Color, f32)` or `Border::Solid(color, width)` to customize the focus ring.
  - Accepts `None` or `Border::None` to **completely disable** the focus ring.
  - If omitted and focused, defaults to a `1.0` width accent blue border (`Color::rgb(0.4, 0.7, 1.0)`).

```rust
// Custom 2px emerald focus ring:
ui.input(&mut title)
    .border((Color::rgb(0.2, 0.2, 0.2), 1.0))
    .focus_border((Color::rgb(0.1, 0.8, 0.5), 2.0))
    .show();

// Disabling focus ring entirely (e.g., in headless or embedded styling):
ui.input(&mut title)
    .border(None)
    .focus_border(None)
    .show();
```

---

### 2.6 Corner Radii

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

```rust
// Pill-shaped input:
ui.input(&mut search)
    .radius_all(18.0)
    .show();
```

---

### 2.7 Shadows & Opacity

#### Method Signatures

```rust
pub fn shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self
pub fn shadow_opacity(mut self, opacity: f32) -> Self
pub fn opacity(mut self, o: f32) -> Self
```

#### Description
- `shadow`: Draws a soft drop shadow beneath the input box.
- `shadow_opacity`: Multiplier for the shadow color's alpha channel (default: `1.0`).
- `opacity`: Overall opacity for the entire input quad and glyphs (default: `1.0`).

```rust
ui.input(&mut search)
    .shadow(Color::BLACK, 0.0, 4.0, 10.0)
    .shadow_opacity(0.35)
    .show();
```

---

### 2.8 Sizing, Width & Padding

#### Method Signatures

```rust
pub fn width(mut self, w: f32) -> Self
pub fn min_width(mut self, w: f32) -> Self
pub fn fill_x(mut self) -> Self

// Outer box padding:
pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self
pub fn padding_all(mut self, p: f32) -> Self
pub fn padding_x(mut self, x: f32) -> Self
pub fn padding_y(mut self, y: f32) -> Self
pub fn padding_top(mut self, t: f32) -> Self
pub fn padding_bottom(mut self, b: f32) -> Self
pub fn padding_left(mut self, l: f32) -> Self
pub fn padding_right(mut self, r: f32) -> Self
```

#### Sizing Model
1. **Explicit Width (`.width(w)`)**: Sets fixed box width.
2. **Auto Width**: If `.width()` is omitted, the input automatically sizes to accommodate the text buffer or placeholder, constrained by `.min_width(w)`.
3. **Fill Width (`.fill_x()`)**: Expands the input to occupy all available horizontal container space.

---

### 2.9 Scrolling & Selection

#### Method Signatures

```rust
pub fn scrollable(mut self, s: bool) -> Self // Default: true
```

#### Automatic Horizontal Viewport Tracking
When the typed text exceeds the input's visible width:
- The input automatically scrolls horizontally (`scroll_x`) so the cursor remains visible with a comfortable margin (10px–40px).
- Mouse drag selection across edges automatically scrolls the text viewport.
- Clicking or dragging across characters selects substrings.

---

### 2.10 Event Handling & Callbacks

#### Method Signatures

```rust
pub fn on_change<F>(self, f: F) -> Self
where
    F: FnMut(String) + 'a

pub fn show(self) -> zenthra_core::Response
```

#### Response Flags
`show()` returns `Response`:
- `res.clicked`: User clicked the input this frame.
- `res.hovered`: Mouse pointer is hovering over the input.
- `res.pressed`: Mouse button is actively down on the input.
- `res.changed`: Buffer content was modified this frame.
- `res.submitted`: User pressed `Enter` while focused.

```rust
let res = ui.input(&mut query)
    .placeholder("Type and press Enter...")
    .show();

if res.changed {
    println!("Buffer updated: {}", query);
}

if res.submitted {
    execute_search(&query);
}
```

---

## 3. Keyboard Navigation & Shortcuts

The `Input` widget handles full text navigation and editing shortcuts:

| Shortcut | Action |
| :--- | :--- |
| `Left` / `Right` | Move cursor one character backward / forward |
| `Ctrl` / `Alt` + `Left` | Jump to previous word boundary |
| `Ctrl` / `Alt` + `Right` | Jump to next word boundary |
| `Shift` + `Arrow` | Expand / shrink selection range |
| `Home` / `End` | Jump to beginning / end of buffer |
| `Shift` + `Home` / `End` | Select from cursor to beginning / end |
| `Backspace` | Delete character before cursor (or delete selection) |
| `Delete` | Delete character after cursor (or delete selection) |
| `Ctrl` + `Backspace` | Delete entire previous word |
| `Ctrl` + `Delete` | Delete entire next word |
| `Ctrl` + `A` | Select entire buffer |
| `Ctrl` + `C` | Copy selection to system clipboard |
| `Ctrl` + `X` | Cut selection to system clipboard |
| `Ctrl` + `V` | Paste clipboard text at cursor (replaces selection) |
| `Enter` / `NumpadEnter` | Dispatch `WidgetEvent::Submit` (`res.submitted == true`) |
| `Escape` | Remove focus / blur |

---

## 4. Production Recipes & Patterns

### Recipe 1: Standard Form Input

A standard dark-mode form input with label, placeholder, subtle border, and rounded corners:

```rust
ui.column().spacing(6.0).show(|ui| {
    ui.text("Display Name")
        .font_size(13.0)
        .color(Color::rgb(0.70, 0.75, 0.85))
        .show();

    ui.input(&mut user.name)
        .width(280.0)
        .font_size(14.0)
        .bg(Color::rgb(0.12, 0.13, 0.17))
        .border((Color::rgb(0.22, 0.25, 0.32), 1.0))
        .focus_border((Color::rgb(0.35, 0.60, 0.98), 1.5))
        .radius_all(6.0)
        .padding(8.0, 10.0, 8.0, 10.0)
        .placeholder("e.g. Alex Mercer")
        .show();
});
```

---

### Recipe 2: Seamless Inline Rename Input

Pure text with zero background or border when unfocused, activating an elegant accent underline or focus box only while editing:

```rust
// Seamless Inline Input (Directly from docs/api/container.md Section 3.2)
ui.input(&mut item_name)
    .font_size(14.0)
    .bg(None)                                           // Transparent surface
    .border(None)                                       // Borderless when idle
    .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.0))   // Clean highlight when editing
    .radius_all(3.0)
    .padding(2.0, 4.0, 2.0, 4.0)
    .show();
```

---

### Recipe 3: Filter / Search Bar with Drop Shadow

Pill-shaped search field with subtle depth and clear placeholder:

```rust
ui.input(&mut search_query)
    .fill_x()
    .font_size(14.0)
    .bg(Color::rgb(0.14, 0.15, 0.20))
    .border((Color::rgb(0.24, 0.27, 0.35), 1.0))
    .focus_border((Color::rgb(0.40, 0.75, 1.00), 1.5))
    .radius_all(18.0)
    .padding(8.0, 14.0, 8.0, 14.0)
    .shadow(Color::BLACK, 0.0, 3.0, 8.0)
    .shadow_opacity(0.30)
    .placeholder("Search assets, components...")
    .show();
```

---

### Recipe 4: Custom Branded Focus Ring

Inputs with project-specific theme colors (e.g. Warning / Error validation state):

```rust
let is_invalid = email.contains('@') == false && !email.is_empty();

let border_color = if is_invalid {
    Color::rgb(0.95, 0.30, 0.30) // Red error border
} else {
    Color::rgb(0.22, 0.25, 0.32)
};

let focus_color = if is_invalid {
    Color::rgb(1.00, 0.40, 0.40)
} else {
    Color::rgb(0.35, 0.60, 0.98)
};

ui.input(&mut email)
    .width(260.0)
    .border((border_color, 1.0))
    .focus_border((focus_color, 1.5))
    .radius_all(6.0)
    .show();
```

---

### Recipe 5: Compact Fixed-Width Property Field

In property inspectors or numerical editors where labels and inputs sit inline:

```rust
ui.row().spacing(8.0).valign(Align::Center).show(|ui| {
    ui.text("Label:").width(60.0).show();
    
    ui.input(&mut label_text)
        .width(140.0)
        .font_size(12.0)
        .bg(Color::rgb(0.10, 0.11, 0.14))
        .border((Color::rgb(0.20, 0.22, 0.28), 1.0))
        .radius_all(4.0)
        .padding(4.0, 6.0, 4.0, 6.0)
        .show();
});
```

---

## 5. Anti-Patterns & Troubleshooting

### Anti-Pattern 1: Hardcoding IDs with Identical Strings
**Wrong:**
```rust
for (i, item) in items.iter_mut().enumerate() {
    ui.input_with_id(&mut item.name, "name_field").show(); // ❌ All inputs share the same ID!
}
```
**Right:**
```rust
for (i, item) in items.iter_mut().enumerate() {
    ui.input_with_id(&mut item.name, ("item_name", i)).show(); // ✅ Stable, unique compound ID (or omit for auto-ID)
}
```

### Anti-Pattern 2: Wrapping an Input in an Extra Container for Focus Borders
**Wrong:**
```rust
// ❌ Redundant container wrapper just to draw a border:
ui.container()
    .border(if is_focused { (Color::BLUE, 1.0) } else { (Color::GRAY, 1.0) })
    .show(|ui| {
        ui.input(&mut text).bg(None).show();
    });
```
**Right:**
```rust
// ✅ Input has native first-class border & focus_border:
ui.input(&mut text)
    .border((Color::GRAY, 1.0))
    .focus_border((Color::BLUE, 1.0))
    .show();
```

### Anti-Pattern 3: Trying to Use `.borderless()`
**Wrong:**
```rust
ui.input(&mut text).borderless().show(); // ❌ Method does not exist
```
**Right:**
```rust
ui.input(&mut text).border(None).show(); // ✅ Canonical Zenthra convention
```

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
