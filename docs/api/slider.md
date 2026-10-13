# Slider API Specification & Unified Styling Architecture

`Slider` is Zenthra's primary numerical continuous control widget. It provides interactive drag-to-seek, range normalization, discrete stepping, custom track and thumb styling, gradient fills, and unified outer container styling (surfaces, borders, corner radii, and drop shadows).

The `Slider` widget strictly adheres to the unified styling contract established in [`docs/api/container.md`](./container.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Slider API Reference](#2-complete-slider-api-reference)
   - [2.1 Construction & Range](#21-construction--range)
   - [2.2 Dimensions, Width & Fill](#22-dimensions-width--fill)
   - [2.3 Container Background & Border (Unified Styling)](#23-container-background--border-unified-styling)
   - [2.4 Container Corner Radii & Shadows](#24-container-corner-radii--shadows)
   - [2.5 Inner Padding](#25-inner-padding)
   - [2.6 Track Styling & Customization](#26-track-styling--customization)
   - [2.7 Thumb Handle Styling & Customization](#27-thumb-handle-styling--customization)
   - [2.8 Event Handling & Response](#28-event-handling--response)
3. [Production Recipes & Patterns](#3-production-recipes--patterns)
   - [Recipe 1: Minimalist Track-Only Slider](#recipe-1-minimalist-track-only-slider)
   - [Recipe 2: Contained Property Slider with Border & Shadow](#recipe-2-contained-property-slider-with-border--shadow)
   - [Recipe 3: Stepped Integer Slider (Quality / Steps)](#recipe-3-stepped-integer-slider-quality--steps)
   - [Recipe 4: Vibrant Audio Volume Slider (Gradient Track)](#recipe-4-vibrant-audio-volume-slider-gradient-track)
   - [Recipe 5: Compact Inspector Row with Label and Slider](#recipe-5-compact-inspector-row-with-label-and-slider)
4. [Anti-Patterns & Troubleshooting](#4-anti-patterns--troubleshooting)
5. [Type Definitions](#5-type-definitions)

---

## 1. Design Philosophy & Mental Model

### The Core Mental Model
In Zenthra, a `Slider` consists of three layered visual elements:
1. **Container Surface (Optional)**: An outer box framing the slider control. Transparent by default (`.bg(None)`). When styled with `.border(...)` or `.shadow(...)`, it synthesizes an elegant enclosure for the slider.
2. **Track Bar**: The horizontal trough indicating the full range (`min` to `max`).
3. **Thumb Handle**: The draggable indicator located at the current value percentage.

### Unified Styling Contract
Just like `Container`:
- `.bg(Color)` sets an enclosing surface; `.bg(None)` (default) keeps the container transparent.
- `.border((Color, width))` sets an enclosing border; `.border(None)` removes it.
- `.radius_all(f32)` rounds the outer container corners.
- `.shadow(color, ox, oy, blur)` and `.shadow_opacity(opacity)` elevate the slider container.

```rust
// ✅ Clean, consistent Zenthra styling:
ui.slider(&mut opacity, "opacity_slider")
    .range(0.0, 1.0)
    .width(240.0)
    .border((Color::rgb(0.24, 0.26, 0.35), 1.0))
    .radius_all(6.0)
    .show();
```

---

## 2. Complete Slider API Reference

### 2.1 Construction & Range

#### Method Signatures

```rust
// On Ui:
pub fn slider<'b>(
    &mut self, 
    value: &'b mut f32, 
    id: impl std::hash::Hash
) -> SliderBuilder<'_, 'a, 'b>

// On SliderBuilder:
pub fn range(mut self, min: f32, max: f32) -> Self // Default: 0.0 .. 1.0
pub fn step(mut self, step: f32) -> Self           // Default: None (Continuous)
pub fn id(mut self, id: impl std::hash::Hash) -> Self
```

#### Description
- `ui.slider(value, id)` initializes a `SliderBuilder` binding to `&mut f32`.
- `range(min, max)` maps normalized drag offsets (`0.0 .. 1.0`) to the specified numeric interval.
- `step(step)` snaps dragged values to discrete multiples of `step`.

---

### 2.2 Dimensions, Width & Fill

#### Method Signatures

```rust
pub fn width(mut self, w: f32) -> Self
pub fn height(mut self, h: f32) -> Self
pub fn size(mut self, w: f32, h: f32) -> Self
pub fn fill_x(mut self) -> Self
```

#### Defaults
- Width: If `.width()` is omitted, the slider automatically spans `self.ui.available_width.max(100.0)`.
- Height: Automatically calculated based on the maximum of thumb and track heights plus vertical padding (`(thumb_h.max(track_h).max(20.0) + padding.vertical())`).

---

### 2.3 Container Background & Border (Unified Styling)

#### Method Signatures

```rust
pub fn bg(mut self, bg: impl Into<Option<Color>>) -> Self
pub fn bg_gradient(mut self, start: Color, end: Color, direction: GradientDirection) -> Self

pub fn border(mut self, border: impl Into<zenthra_core::Border>) -> Self
pub fn opacity(mut self, o: f32) -> Self
```

#### Behavior & Unified Rules
- `bg`: Sets the outer container surface. Default: `None` (transparent).
- `border`: Accepts `(Color, f32)` or `Border::Solid(c, w)`. Accepts `None` or `Border::None` to remove the border.
- When `bg`, `bg_gradient`, `border`, or `shadow` is configured, `Slider` automatically renders the outer container quad.

```rust
// Enclosed slider box with subtle border:
ui.slider(&mut brightness, "brightness")
    .bg(Color::rgb(0.12, 0.13, 0.18))
    .border((Color::rgb(0.25, 0.28, 0.36), 1.0))
    .show();
```

---

### 2.4 Container Corner Radii & Shadows

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

pub fn shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self
pub fn shadow_opacity(mut self, o: f32) -> Self
```

---

### 2.5 Inner Padding

#### Method Signatures

```rust
pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self
pub fn padding_all(mut self, p: f32) -> Self
pub fn padding_x(mut self, x: f32) -> Self
pub fn padding_y(mut self, y: f32) -> Self
```

#### Defaults
- Default padding: `EdgeInsets::symmetric(0.0, 10.0)` (0px vertical, 10px horizontal), providing clearance so the thumb handle does not clip past the track edge.

---

### 2.6 Track Styling & Customization

#### Method Signatures

```rust
pub fn track_size(mut self, w: f32, h: f32) -> Self
pub fn track_width(mut self, w: f32) -> Self
pub fn track_height(mut self, h: f32) -> Self

pub fn track_color(mut self, c: Color) -> Self
pub fn track_gradient(mut self, start: Color, end: Color, direction: GradientDirection) -> Self

pub fn track_radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self
pub fn track_radius_all(mut self, r: f32) -> Self

pub fn track_shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self
pub fn track_shadow_opacity(mut self, o: f32) -> Self
```

#### Defaults
- Track height: `4.0`
- Track color: `Color::rgb(0.2, 0.2, 0.2)`
- Track radius: `[2.0; 4]` (pill rounded bar)

```rust
// Slim, sleek track:
ui.slider(&mut zoom, "zoom")
    .track_height(2.0)
    .track_color(Color::rgb(0.18, 0.20, 0.26))
    .show();
```

---

### 2.7 Thumb Handle Styling & Customization

#### Method Signatures

```rust
pub fn thumb_size(mut self, w: f32, h: f32) -> Self
pub fn thumb_width(mut self, w: f32) -> Self
pub fn thumb_height(mut self, h: f32) -> Self

pub fn thumb_color(mut self, c: Color) -> Self
pub fn thumb_gradient(mut self, start: Color, end: Color, direction: GradientDirection) -> Self

pub fn thumb_radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self
pub fn thumb_radius_all(mut self, r: f32) -> Self

pub fn thumb_shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self
pub fn thumb_shadow_opacity(mut self, o: f32) -> Self
```

#### Defaults
- Thumb size: `16.0 x 16.0`
- Thumb radius: `[8.0; 4]` (perfect circular thumb)
- Thumb color: `Color::rgb(0.4, 0.6, 1.0)`
- Hover / Active transitions: Automatic subtle brightness boost on hover (1.1x) and drag (1.3x).

```rust
// Large circular thumb with drop shadow:
ui.slider(&mut volume, "vol")
    .thumb_size(20.0, 20.0)
    .thumb_radius_all(10.0)
    .thumb_color(Color::WHITE)
    .thumb_shadow(Color::BLACK, 0.0, 2.0, 6.0)
    .thumb_shadow_opacity(0.4)
    .show();
```

---

### 2.8 Event Handling & Response

#### Method Signatures

```rust
pub fn on_change<F>(self, f: F) -> Self where F: FnMut(f32) + 'a
pub fn show(self) -> zenthra_core::Response
```

#### Response Flags
`show()` returns `Response`:
- `res.changed`: True on frames where the slider value changed.
- `res.hovered`: True while the mouse pointer is hovering over the slider.
- `res.pressed`: True while the user is actively dragging the thumb.
- `res.clicked`: True when the user clicks anywhere on the track or thumb.

```rust
let res = ui.slider(&mut zoom_level, "zoom")
    .range(0.25, 4.0)
    .show();

if res.changed {
    update_viewport_scale(zoom_level);
}
```

---

## 3. Production Recipes & Patterns

### Recipe 1: Minimalist Track-Only Slider

A clean, modern slider with a slim track and prominent accent thumb:

```rust
ui.slider(&mut progress, "progress")
    .range(0.0, 100.0)
    .width(260.0)
    .track_height(3.0)
    .track_color(Color::rgb(0.18, 0.20, 0.25))
    .thumb_size(14.0, 14.0)
    .thumb_radius_all(7.0)
    .thumb_color(Color::rgb(0.35, 0.65, 1.00))
    .show();
```

---

### Recipe 2: Contained Property Slider with Border & Shadow

An enclosed slider field ideal for dark-mode inspector property panels:

```rust
ui.slider(&mut exposure, "exposure")
    .range(-2.0, 2.0)
    .width(280.0)
    .padding(6.0, 12.0, 6.0, 12.0)
    .bg(Color::rgb(0.11, 0.12, 0.16))
    .border((Color::rgb(0.22, 0.24, 0.32), 1.0))
    .radius_all(6.0)
    .track_height(4.0)
    .track_color(Color::rgb(0.20, 0.22, 0.28))
    .thumb_size(16.0, 16.0)
    .thumb_radius_all(8.0)
    .thumb_color(Color::rgb(0.40, 0.70, 0.95))
    .show();
```

---

### Recipe 3: Stepped Integer Slider (Quality / Steps)

Stepping through discrete integer values (e.g. 1 to 10):

```rust
ui.slider(&mut quality_level, "quality")
    .range(1.0, 10.0)
    .step(1.0)
    .width(220.0)
    .show();
```

---

### Recipe 4: Vibrant Audio Volume Slider (Gradient Track)

A stylized audio slider with multi-color gradient track:

```rust
ui.slider(&mut master_vol, "master_vol")
    .range(0.0, 1.0)
    .width(300.0)
    .track_height(6.0)
    .track_gradient(
        Color::rgb(0.20, 0.70, 0.40), // Green (Safe)
        Color::rgb(0.95, 0.30, 0.20), // Red (Clipping)
        GradientDirection::LeftToRight,
    )
    .thumb_size(18.0, 18.0)
    .thumb_radius_all(9.0)
    .thumb_color(Color::WHITE)
    .thumb_shadow(Color::BLACK, 0.0, 3.0, 6.0)
    .show();
```

---

### Recipe 5: Compact Inspector Row with Label and Slider

Inline row pairing a fixed-width label with a slider:

```rust
ui.row().spacing(8.0).valign(Align::Center).show(|ui| {
    ui.text("Roughness:").width(80.0).show();
    ui.slider(&mut roughness, "roughness")
        .range(0.0, 1.0)
        .width(180.0)
        .show();
});
```

---

## 4. Anti-Patterns & Troubleshooting

### Anti-Pattern 1: Wrapping in a Container Just for Outer Borders
**Wrong:**
```rust
// ❌ Redundant container nesting:
ui.container()
    .border((Color::WHITE, 1.0))
    .radius_all(4.0)
    .show(|ui| {
        ui.slider(&mut val, "id").show();
    });
```
**Right:**
```rust
// ✅ Slider directly supports container styling:
ui.slider(&mut val, "id")
    .border((Color::WHITE, 1.0))
    .radius_all(4.0)
    .show();
```

### Anti-Pattern 2: Forgetting Unique IDs in Loops
**Wrong:**
```rust
for (i, val) in values.iter_mut().enumerate() {
    ui.slider(val, "slider").show(); // ❌ All share the same drag state!
}
```
**Right:**
```rust
for (i, val) in values.iter_mut().enumerate() {
    ui.slider(val, ("slider", i)).show(); // ✅ Stable compound ID
}
```

---

## 5. Type Definitions

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
