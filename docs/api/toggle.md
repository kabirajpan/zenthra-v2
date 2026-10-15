# Toggle API Specification & Unified Styling Architecture

`Toggle` is Zenthra's interactive sliding switch control for immediate-mode boolean state binding. It supports smooth spring animations, track box styling with full `BoxStyle` unification, customizable thumb knobs, optional inner track status labels ("ON" / "OFF"), external labels on either side, glow effects, drop shadows, and disabling.

The `Toggle` widget strictly adheres to the unified styling contract established in [`docs/api/macros.md`](macros.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Toggle API Reference](#2-complete-toggle-api-reference)
   - [2.1 Construction & State Binding](#21-construction--state-binding)
   - [2.2 Track Box Dimensions & Shape](#22-track-box-dimensions--shape)
   - [2.3 Box Background, Borders & Shadows](#23-box-background-borders--shadows)
   - [2.4 Colors & Thumb Customization](#24-colors--thumb-customization)
   - [2.5 Inner Track Labels](#25-inner-track-labels)
   - [2.6 External Label & Typography](#26-external-label--typography)
   - [2.7 Animation, Glow & Micro-Interactions](#27-animation-glow--micro-interactions)
   - [2.8 Event Handling & Response](#28-event-handling--response)
3. [Recipes & Patterns](#3-recipes--patterns)

---

## 1. Design Philosophy & Mental Model

In Zenthra, a `Toggle`:
- Directly mutates an immediate-mode `&mut bool` reference on user click.
- Renders a styled track quad (`BoxStyle`), an animated thumb knob, optional inner status labels, and an optional adjacent external label (`TextStyle`).
- Interpolates sliding position and hover scale across frames via `ui.interaction_state`.
- Adheres to the **Unified Styling System**: Track dimensions, corner radii, borders, and shadows are configured directly or via `.style(BoxStyle)`.

```rust
ui.toggle(&mut is_active, "Power State")
    .width(52.0)
    .height(26.0)
    .pill()
    .colors(
        Color::rgb(0.15, 0.75, 0.45), // Emerald on active
        Color::rgb(0.18, 0.18, 0.22), // Charcoal on inactive
        Color::WHITE,                 // Thumb
    )
    .show();
```

---

## 2. Complete Toggle API Reference

### 2.1 Construction & State Binding

```rust
// On Ui:
pub fn toggle<'b>(
    &mut self,
    value: &'b mut bool,
    label: impl Into<Option<&'b str>>,
) -> ToggleBuilder<'_, 'a, 'b>

// On ToggleBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
```

- `ui.toggle(&mut val, "Label")`: Creates a toggle with an external label.
- `ui.toggle(&mut val, None)`: Creates a standalone toggle switch without external label.

---

### 2.2 Track Box Dimensions & Shape

```rust
pub fn track_size(mut self, width: f32, height: f32) -> Self
pub fn width(mut self, width: f32) -> Self      // Default: 48.0
pub fn height(mut self, height: f32) -> Self    // Default: 24.0
pub fn padding(mut self, padding: f32) -> Self  // Default: 3.0 (spacing to thumb)
pub fn pill(mut self) -> Self                   // Default (radius = height / 2.0)
pub fn square(mut self) -> Self                 // Rectangular toggle (radius = 0.0)
pub fn radius_all(mut self, radius: f32) -> Self
pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self
```

---

### 2.3 Box Background, Borders & Shadows

Generated via `impl_visuals!(ToggleBuilder, ...)`:

```rust
pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self
pub fn border(mut self, border: impl IntoBorder) -> Self
```

- Shadow opacity is configured directly via `Color::rgba(...)`.
- `.border(...)` accepts `(Color, width)`, `(Color, width, BorderAlignment)`, `Border`, or `None`.

---

### 2.4 Colors & Thumb Customization

```rust
pub fn colors(mut self, on: Color, off: Color, thumb: Color) -> Self
pub fn on_color(mut self, color: Color) -> Self     // Default: Blue (0.0, 0.5, 1.0)
pub fn off_color(mut self, color: Color) -> Self    // Default: Gray (0.2, 0.2, 0.2)
pub fn thumb_color(mut self, color: Color) -> Self  // Default: Color::WHITE
pub fn thumb_radius(mut self, radius: f32) -> Self  // Overrides circular thumb
pub fn thumb_width(mut self, width: f32) -> Self    // Overrides square/circular thumb width
```

---

### 2.5 Inner Track Labels

Displays compact status labels directly inside the track channel, visible when the knob slides away:

```rust
pub fn inner_labels(mut self, on: &str, off: &str) -> Self
pub fn inner_color(mut self, color: Color) -> Self  // Default: Color::WHITE
pub fn inner_size(mut self, size: f32) -> Self      // Default: 10.0
```

```rust
ui.toggle(&mut wifi, None)
    .width(56.0)
    .inner_labels("ON", "OFF")
    .show();
```

---

### 2.6 External Label & Typography

Generated via `impl_text_style_methods!(ToggleBuilder, ...)`:

```rust
pub fn label_left(mut self) -> Self
pub fn label_right(mut self) -> Self                // Default
pub fn label_gap(mut self, gap: f32) -> Self        // Default: 10.0
pub fn size(mut self, size: f32) -> Self            // Default: 14.0 (Label text size)
pub fn color(mut self, color: Color) -> Self        // Default: Color::WHITE (Label text color)
pub fn font_weight(mut self, weight: impl Into<zenthra_text::prelude::FontWeight>) -> Self
pub fn font_family(mut self, family: impl Into<String>) -> Self

// State Siblings (.active(Text::color(...)) for label transition when ON):
pub fn hover(mut self, props: impl IntoStateProps) -> Self
pub fn active(mut self, props: impl IntoStateProps) -> Self
```

---

### 2.7 Animation, Glow & Micro-Interactions

```rust
pub fn animation_speed(mut self, speed: f32) -> Self // Default: 15.0
pub fn glow(mut self, enabled: bool) -> Self         // Accent neon aura when active
pub fn scaling(mut self, hover: f32, pressed: f32) -> Self
pub fn disabled(mut self, disabled: bool) -> Self
```

---

### 2.8 Event Handling & Response

```rust
pub fn on_click<F>(self, f: F) -> Self where F: FnMut() + 'a
pub fn on_change<F>(self, f: F) -> Self where F: FnMut(bool) + 'a
pub fn on_hover<F>(self, f: F) -> Self where F: FnMut(bool) + 'a

pub fn show(self) -> Response
```

Returns a `Response` containing `.clicked`, `.hovered`, `.pressed`, and `.changed`.
