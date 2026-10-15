# DragValue API Specification & Unified Styling Architecture

`DragValue` is Zenthra's primary scrubbable numeric input control. It binds directly to an immediate-mode generic `&mut N` state (where `N: NumericValue` including `f32`, `f64`, `i8`, `i16`, `i32`, `i64`, `isize`, `u8`, `u16`, `u32`, `u64`, `usize`). It allows horizontal dragging/scrubbing with deadzone detection and modifier-key acceleration (Shift for fine 0.1x control, Ctrl for coarse 10x control), as well as direct in-place text typing upon clicking. It conforms 100% to the unified styling contract and zero-cost Sibling architecture.

The `DragValue` widget strictly adheres to the unified styling contract established in [`docs/api/container.md`](./container.md) and [`docs/api/macros.md`](./macros.md).

---

## Table of Contents

1. [Design Philosophy & Dual Modes](#1-design-philosophy--dual-modes)
2. [Complete DragValue API Reference](#2-complete-dragvalue-api-reference)
   - [2.1 Construction & Generic Numeric Binding](#21-construction--generic-numeric-binding)
   - [2.2 Numeric Configuration (.min, .max, .range, .speed, .step, .precision)](#22-numeric-configuration-min-max-range-speed-step-precision)
   - [2.3 Affixes & Alignment (.prefix, .suffix, .align)](#23-affixes--alignment-prefix-suffix-align)
   - [2.4 Dimensions & Layout (.width, .height, .fill_x, .padding, .margin)](#24-dimensions--layout-width-height-fill_x-padding-margin)
   - [2.5 Container Surface & Focus Styling (`BoxStyle` Parity)](#25-container-surface--focus-styling-boxstyle-parity)
   - [2.6 Corner Radii & Shadows](#26-corner-radii--shadows)
   - [2.7 Typography & Text Styling](#27-typography--text-styling)
   - [2.8 Interaction States & Sibling Architecture](#28-interaction-states--sibling-architecture)
   - [2.9 Event Handling & Response](#29-event-handling--response)
3. [Production Recipes & Patterns](#3-production-recipes--patterns)
   - [Recipe 1: Blender-Style Compact Transform Field](#recipe-1-blender-style-compact-transform-field)
   - [Recipe 2: Percentage Scrubbing Slider Alternative](#recipe-2-percentage-scrubbing-slider-alternative)
   - [Recipe 3: Degree Rotation Input with Snapping Step](#recipe-3-degree-rotation-input-with-snapping-step)
   - [Recipe 4: Clean Borderless Form Readout](#recipe-4-clean-borderless-form-readout)
4. [Anti-Patterns & Troubleshooting](#4-anti-patterns--troubleshooting)
5. [Type Definitions](#5-type-definitions)

---

## 1. Design Philosophy & Dual Modes

### The Dual Interaction Modes
In Zenthra, a `DragValue` seamlessly toggles between two interaction modes:
1. **Scrubbing Mode (Default)**:
   - Hovering displays a `ColResize` cursor.
   - Dragging horizontally scrubs the value based on `.speed(s)`.
   - Built-in **deadzone detection (6.0px)** prevents accidental mutations on normal clicks.
   - **Modifier Acceleration**: Hold `Shift` for fine 0.1x adjustments; hold `Ctrl` for coarse 10x adjustments.
2. **Direct Text Entry Mode (In-Place)**:
   - Clicking without dragging immediately activates direct text entry mode.
   - Automatically selects all text (`Ctrl+A` behavior on click), displays a blinking cursor, and handles keyboard inputs (`Enter` to commit, `Escape` to cancel, `Backspace`, `Delete`, arrow keys).
   - Draws with `.focus_border(...)` during active typing.

### Unified `BoxStyle` Parity
`DragValueBuilder` implements `zenthra_core::Styleable`, storing container geometry in `self.style: BoxStyle` and text styling in `self.text_style: TextStyle`. State transitions (`.hover([ ... ])` and `.active([ ... ])`) blend via GPU pipeline using `BoxStyleExt::build_rect_styled`.

```rust
// ✅ Clean, canonical Zenthra usage:
let mut scale = 100.0f32;

ui.drag_value(&mut scale, "scale_input")
    .prefix("Scale: ")
    .suffix("%")
    .speed(0.5)
    .range(0.0, 500.0)
    .width(120.0)
    .bg(Color::rgb(0.12, 0.14, 0.18))
    .border((Color::rgb(0.24, 0.28, 0.36), 1.0))
    .radius_all(6.0)
    .show();
```

---

## 2. Complete DragValue API Reference

### 2.1 Construction & Generic Numeric Binding

```rust
// On Ui:
pub fn drag_value<'b, N: NumericValue>(
    &mut self,
    value: &'b mut N,
    id: impl std::hash::Hash
) -> DragValueBuilder<'_, 'a, 'b, N>

// On DragValueBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
```

#### Supported Numeric Types
Any type implementing `NumericValue`:
- Floating-point: `f32`, `f64` (default speed: `0.1`, default precision: `2`)
- Signed integers: `i8`, `i16`, `i32`, `i64`, `isize` (default speed: `1.0`, default precision: `0`)
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `usize` (default speed: `1.0`, default precision: `0`)

---

### 2.2 Numeric Configuration (.min, .max, .range, .speed, .step, .precision)

| Method | Description |
| :--- | :--- |
| `.min(impl Into<f64>)` | Minimum allowed value. Values below are clamped. |
| `.max(impl Into<f64>)` | Maximum allowed value. Values above are clamped. |
| `.range(min, max)` | Convenience setter clamping value within `[min, max]`. |
| `.speed(impl Into<f64>)` | Scrub sensitivity in numeric units per horizontal pixel moved. |
| `.step(impl Into<f64>)` | Snapping step increment (e.g. `1.0`, `5.0`, `0.5`). |
| `.precision(usize)` | Number of decimal places to format for display and editing. |

---

### 2.3 Affixes & Alignment (.prefix, .suffix, .align)

| Method | Description |
| :--- | :--- |
| `.prefix(impl Into<String>)` | Static text shown before the numeric value (e.g. `"X: "`, `"Radius: "`). |
| `.suffix(impl Into<String>)` | Static text shown after the numeric value (e.g. `" px"`, `"%"`, `"°"`). |
| `.prefix_color(Color)` | Custom text color for the prefix label. |
| `.suffix_color(Color)` | Custom text color for the suffix label. |
| `.align(Align)` | Horizontal alignment of the label (`Align::Left`, `Align::Center`, `Align::Right`). |

---

### 2.4 Dimensions & Layout (.width, .height, .fill_x, .padding, .margin)

| Method | Description |
| :--- | :--- |
| `.width(w: f32)` | Explicit width in logical pixels (default: `75.0`). |
| `.height(h: f32)` | Explicit height in logical pixels (default: `24.0`). |
| `.fill_x()` | Expands horizontally to fill available container width. |
| `.fill_y()` | Expands vertically to fill available container height. |
| `.fill()` | Expands both horizontally and vertically. |
| `.min_width(w)` / `.max_width(w)` | Clamps width constraints. |
| `.min_height(h)` / `.max_height(h)` | Clamps height constraints. |
| `.padding(t, r, b, l)` | Per-edge internal padding. |
| `.padding_all(p: f32)` | Uniform internal padding (default: `symmetric(0.0, 8.0)`). |
| `.padding_x(px)` / `.padding_y(py)` | Axis-specific internal padding. |
| `.margin(t, r, b, l)` / `.margin_all(m)` | Outer container spacing. |

---

### 2.5 Container Surface & Focus Styling (`BoxStyle` Parity)

| Method | Description |
| :--- | :--- |
| `.style(BoxStyle)` | Replaces entire underlying container `BoxStyle`. |
| `.bg(impl Into<Background>)` | Container background (`Color`, `Gradient`, or `None`). |
| `.border(impl IntoBorder)` | Standard border stroke `(Color, width)`, `(Color, width, BorderAlignment)`, or `Border::None`. |
| `.focus_border(impl IntoBorder)` | Active border stroke used while typing in direct edit mode. Default is `(Color::rgb(0.35, 0.58, 0.98), 1.5)`. |
| `.shadow(color, ox, oy, blur)` | Drop shadow offset and blur. |

---

### 2.6 Corner Radii & Shadows

| Method | Description |
| :--- | :--- |
| `.radius_all(r: f32)` | Uniform radius across all four corners (default: `4.0`). |
| `.radius(tl, tr, br, bl)` | Per-corner radii `[tl, tr, br, bl]`. |
| `.radius_top(r)` / `.radius_bottom(r)` | Top or bottom pair corner radii. |
| `.radius_left(r)` / `.radius_right(r)` | Left or right pair corner radii. |
| `.radius_top_left(r)`, etc. | Specific single-corner radius. |

---

### 2.7 Typography & Text Styling

| Method | Description |
| :--- | :--- |
| `.size(size: f32)` / `.font_size(size: f32)` | Font size in logical points (default: `12.0`). |
| `.color(color: Color)` | Primary numeric text color (default: `Color::rgb(0.92, 0.94, 0.98)`). |
| `.font_family(impl Into<String>)` | Custom font family name (e.g. `"Inter"`, `"Fira Code"`). |
| `.font_weight(impl Into<FontWeight>)` | Font weight (`FontWeight::Medium`, `600`, etc.). |

---

### 2.8 Interaction States & Sibling Architecture

Interactions strictly use Zenthra's zero-cost **Sibling architecture** (`Style::*`, `Transform::*`, `Cursor::*`, `Tooltip::*`, `Animation::*`):

```rust
ui.drag_value(&mut exposure, "exp_input")
    .bg(Color::rgb(0.12, 0.14, 0.18))
    .border((Color::rgb(0.24, 0.28, 0.36), 1.0))
    .hover([
        Style::bg(Color::rgb(0.16, 0.19, 0.26)),
        Style::border((Color::rgb(0.35, 0.55, 0.95), 1.5)),
        Cursor::ColResize,
        Tooltip::new("Drag horizontally to scrub exposure value"),
    ])
    .active([
        Style::border((Color::rgb(0.45, 0.70, 1.0), 1.5)),
        Transform::scale(0.99),
    ])
    .show();
```

---

### 2.9 Event Handling & Response

The `.show()` method records layout, emits draw commands, and returns a standard `Response`:

```rust
let response = ui.drag_value(&mut brightness, "brightness").show();

if response.changed {
    println!("Brightness updated to: {}", brightness);
}

if response.clicked {
    // Clicked (entered typing mode)
}
```

---

## 3. Production Recipes & Patterns

### Recipe 1: Blender-Style Compact Transform Field

```rust
ui.container().row().gap(8.0).valign(Align::Center).show(|ui| {
    ui.drag_value(&mut pos_x, "transform_x")
        .prefix("X: ")
        .suffix(" m")
        .speed(0.1)
        .precision(2)
        .width(90.0)
        .bg(Color::rgb(0.14, 0.15, 0.19))
        .border((Color::rgb(0.26, 0.28, 0.36), 1.0))
        .prefix_color(Color::rgb(0.85, 0.30, 0.30)) // Red accent for X
        .radius_all(4.0)
        .show();

    ui.drag_value(&mut pos_y, "transform_y")
        .prefix("Y: ")
        .suffix(" m")
        .speed(0.1)
        .precision(2)
        .width(90.0)
        .bg(Color::rgb(0.14, 0.15, 0.19))
        .border((Color::rgb(0.26, 0.28, 0.36), 1.0))
        .prefix_color(Color::rgb(0.30, 0.80, 0.40)) // Green accent for Y
        .radius_all(4.0)
        .show();
});
```

---

### Recipe 2: Percentage Scrubbing Slider Alternative

```rust
ui.drag_value(&mut opacity, "layer_opacity")
    .prefix("Opacity: ")
    .suffix("%")
    .range(0.0, 100.0)
    .speed(0.5)
    .precision(0)
    .width(130.0)
    .bg(Color::rgb(0.11, 0.12, 0.16))
    .border((Color::rgb(0.22, 0.25, 0.34), 1.0))
    .radius_all(6.0)
    .show();
```

---

### Recipe 3: Degree Rotation Input with Snapping Step

```rust
ui.drag_value(&mut rotation, "obj_rotation")
    .prefix("Rot: ")
    .suffix("°")
    .step(15.0) // Snaps to 15-degree increments while scrubbing
    .speed(1.0)
    .precision(0)
    .width(100.0)
    .radius_all(4.0)
    .show();
```

---

### Recipe 4: Clean Borderless Form Readout

```rust
ui.drag_value(&mut fps, "target_fps")
    .suffix(" FPS")
    .range(1, 240)
    .speed(1.0)
    .bg(Background::None)
    .border(Border::None)
    .hover([
        Style::bg(Color::rgba(1.0, 1.0, 1.0, 0.08)),
        Cursor::ColResize,
    ])
    .radius_all(4.0)
    .show();
```

---

## 4. Anti-Patterns & Troubleshooting

### Anti-Pattern 1: Manually Comparing Old State in Place of `.changed`
❌ **Wrong:**
```rust
let old_val = val;
ui.drag_value(&mut val, "id").show();
if val != old_val { ... }
```
✅ **Correct:** Use the idiomatic response:
```rust
if ui.drag_value(&mut val, "id").show().changed {
    send_update_network_packet(val);
}
```

### Anti-Pattern 2: Forgetting Unique String/Hash IDs in Dynamic Lists
❌ **Wrong:** Using a static ID in a loop causes interaction state collisions between elements:
```rust
for (idx, item) in items.iter_mut().enumerate() {
    ui.drag_value(&mut item.val, "val").show(); // ❌ Collision!
}
```
✅ **Correct:** Include the index or unique key in the hash:
```rust
for (idx, item) in items.iter_mut().enumerate() {
    ui.drag_value(&mut item.val, ("val", idx)).show();
}
```

---

## 5. Type Definitions

```rust
pub struct DragValueBuilder<'u, 'a, 'b, N: NumericValue> {
    pub ui: &'u mut Ui<'a>,
    pub value: &'b mut N,
    pub id: Id,

    pub min: Option<f64>,
    pub max: Option<f64>,
    pub speed: f64,
    pub step: Option<f64>,
    pub precision: usize,
    pub prefix: String,
    pub suffix: String,

    // Box Style & Typography
    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,
    pub focus_border: Option<Border>,
    pub prefix_color: Option<Color>,
    pub suffix_color: Option<Color>,
    pub align: Align,

    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,
}
```
