# Radio Button API Specification & Unified Styling Architecture

`Radio` is Zenthra's primary single-selection option control for enumerated or mutually exclusive values. It binds immediately to an immediate-mode generic state `&mut T` (`T: PartialEq + Clone`), comparing against its option target value `T`.

The `Radio` widget strictly adheres to the unified styling contract established in [`docs/theming.md`](../theming.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Radio API Reference](#2-complete-radio-api-reference)
   - [2.1 Construction & State Binding](#21-construction--state-binding)
   - [2.2 Dimensions & Ring Layout](#22-dimensions--ring-layout)
   - [2.3 Ring & Dot Color Styling](#23-ring--dot-color-styling)
   - [2.4 Box Surfaces & Shadows](#24-box-surfaces--shadows)
   - [2.5 Label Typography & Active Color](#25-label-typography--active-color)
   - [2.6 Animation & Glow Effects](#26-animation--glow-effects)
   - [2.7 Event Handling & Response](#27-event-handling--response)
3. [Recipes & Patterns](#3-recipes--patterns)

---

## 1. Design Philosophy & Mental Model

In Zenthra, a `Radio`:
- Compares `*self.state == self.value` to determine active state.
- Renders an outer circular ring quad via `BoxStyleExt::build_rect_instance` and an inner animated scale-interpolated dot quad.
- Renders an adjacent label via `TextStyle` separated by `.gap(...)`.
- Emits smooth scaling and brightness micro-interactions on hover and press.

```rust
let mut selected_theme = 0;

ui.radio(&mut selected_theme, 0, "Dark Theme")
    .size(20.0)
    .gap(10.0)
    .colors(Color::rgb(0.35, 0.35, 0.45), Color::rgb(0.2, 0.65, 1.0))
    .glow(true)
    .show();
```

---

## 2. Complete Radio API Reference

### 2.1 Construction & State Binding

```rust
// On Ui:
pub fn radio<'b, T: PartialEq + Clone>(
    &mut self,
    state: &'b mut T,
    value: T,
    label: &str,
) -> RadioBuilder<'_, 'a, 'b, T>

// On RadioBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
```

---

### 2.2 Dimensions & Ring Layout

```rust
pub fn size(mut self, size: f32) -> Self // Default: 18.0 (width & height, sets radius to size / 2.0)
pub fn gap(mut self, gap: f32) -> Self   // Default: 10.0 (spacing between ring and label)
pub fn width(mut self, width: f32) -> Self
pub fn height(mut self, height: f32) -> Self
```

---

### 2.3 Ring & Dot Color Styling

```rust
pub fn colors(mut self, ring: Color, dot: Color) -> Self
pub fn ring_color(mut self, color: Color) -> Self // Default: Color::rgb(0.4, 0.4, 0.4)
pub fn dot_color(mut self, color: Color) -> Self  // Default: Color::rgb(0.2, 0.5, 1.0)
```

---

### 2.4 Box Surfaces & Shadows

Generated via `impl_visuals!`:

```rust
pub fn bg(mut self, bg: impl Into<Background>) -> Self // Default: Transparent
pub fn border(mut self, border: impl Into<Border>) -> Self
pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self
pub fn shadow_opacity(mut self, opacity: f32) -> Self
pub fn opacity(mut self, opacity: f32) -> Self
```

---

### 2.5 Label Typography & Active Color

Generated via `impl_text_style_methods!`:

```rust
pub fn size(mut self, size: f32) -> Self      // Default: 14.0
pub fn color(mut self, color: Color) -> Self  // Default: Color::WHITE
pub fn active_label(mut self, color: Color) -> Self
pub fn font_family(mut self, family: impl Into<String>) -> Self
pub fn font_weight(mut self, weight: impl Into<zenthra_text::prelude::FontWeight>) -> Self
```

---

### 2.6 Animation & Glow Effects

```rust
pub fn glow(mut self, enabled: bool) -> Self
pub fn scaling(mut self, hover: f32, pressed: f32) -> Self
pub fn hover_brightness(mut self, b: f32) -> Self
pub fn disabled(mut self, disabled: bool) -> Self
```

---

### 2.7 Event Handling & Response

```rust
pub fn on_click<F>(self, f: F) -> Self where F: FnMut() + 'a
pub fn on_change<F>(self, f: F) -> Self where T: 'a, F: FnMut(&T) + 'a
pub fn on_hover<F>(self, f: F) -> Self where F: FnMut(bool) + 'a

pub fn show(self) -> Response
```
