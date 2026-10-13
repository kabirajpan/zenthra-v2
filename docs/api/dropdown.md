# Dropdown API Specification & Unified Styling Architecture

`Dropdown` is Zenthra's primary single-selection menu control. It binds directly to an immediate-mode generic `&mut T` state (where `T: PartialEq + Clone + ToString`), rendering a compact clickable trigger box with chevron icon and an elevated overlay popover list when open. It features outside-click dismissal, custom placement, scrollable menu bounds, typography control, and full conformance to the unified styling contract.

The `Dropdown` widget strictly adheres to the unified styling contract established in [`docs/api/container.md`](./container.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Dropdown API Reference](#2-complete-dropdown-api-reference)
   - [2.1 Construction & Generic State Binding](#21-construction--generic-state-binding)
   - [2.2 Dimensions & Layout (.fill_x, .padding_x, .placement)](#22-dimensions--layout-fill_x-padding_x-placement)
   - [2.3 Trigger Background & Borders (Unified Styling)](#23-trigger-background--borders-unified-styling)
   - [2.4 Corner Radii & Shadows](#24-corner-radii--shadows)
   - [2.5 Floating Menu Customization](#25-floating-menu-customization)
   - [2.6 Typography & Text Styling](#26-typography--text-styling)
   - [2.7 Event Handling & Response](#27-event-handling--response)
3. [Production Recipes & Patterns](#3-production-recipes--patterns)
   - [Recipe 1: Standard Settings Dropdown](#recipe-1-standard-settings-dropdown)
   - [Recipe 2: Borderless / Transparent Surface Dropdown](#recipe-2-borderless--transparent-surface-dropdown)
   - [Recipe 3: Premium Glowing Selection Control](#recipe-3-premium-glowing-selection-control)
   - [Recipe 4: Customized Floating Overlay Menu](#recipe-4-customized-floating-overlay-menu)
   - [Recipe 5: Full-Width Form Field (.fill_x)](#recipe-5-full-width-form-field-fill_x)
4. [Anti-Patterns & Troubleshooting](#4-anti-patterns--troubleshooting)
5. [Type Definitions](#5-type-definitions)

---

## 1. Design Philosophy & Mental Model

### The Core Mental Model
In Zenthra, a `Dropdown`:
- **Generic State Binding**: Binds directly to an immediate-mode `&mut T` reference where `T: PartialEq + Clone + ToString`. Selection updates mutate `*selected` in-place and return `response.changed == true`.
- **Portal Overlay Rendering**: The popup item list is rendered into `ui.overlays`, ensuring menu items hover above sibling and parent widgets without clipping.
- **Outside-Click Dismissal**: Automatically registers a full-screen transparent hit-catcher to close the menu when clicking anywhere outside.
- **Table 3.6 Styling Conformance**: Supports `.bg(...)`, `.border(...)`, `.radius_all(...)`, and `.shadow(...)` with both concrete values and `None`.

```rust
// ✅ Clean, consistent Zenthra styling:
let mut fruit = "Apple".to_string();
let fruits = vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string()];

ui.dropdown(&mut fruit, fruits)
    .width(220.0)
    .bg(Color::rgb(0.12, 0.13, 0.18))
    .border((Color::rgb(0.25, 0.28, 0.38), 1.0))
    .radius_all(6.0)
    .show();
```

---

## 2. Complete Dropdown API Reference

### 2.1 Construction & Generic State Binding

#### Method Signatures

```rust
// On Ui:
pub fn dropdown<'b, T: PartialEq + Clone + ToString>(
    &mut self,
    selected: &'b mut T,
    options: Vec<T>
) -> DropdownBuilder<'_, 'a, 'b, T>

// On DropdownBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
```

#### Code Examples

```rust
// With Strings
let mut theme = "Dark".to_string();
ui.dropdown(&mut theme, vec!["Dark".to_string(), "Light".to_string()]).show();

// With Custom Enums implementing PartialEq, Clone, ToString
#[derive(PartialEq, Clone)]
enum Quality { Low, Medium, High }

impl std::fmt::Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Low => write!(f, "Low (720p)"),
            Quality::Medium => write!(f, "Medium (1080p)"),
            Quality::High => write!(f, "High (4K)"),
        }
    }
}

let mut current_q = Quality::High;
ui.dropdown(&mut current_q, vec![Quality::Low, Quality::Medium, Quality::High]).show();
```

---

### 2.2 Dimensions & Layout (.fill_x, .padding_x, .placement)

Controls trigger dimensions, inner horizontal padding, and overlay placement.

#### Method Signatures

```rust
/// Sets the fixed width of the dropdown trigger and menu.
pub fn width(mut self, width: f32) -> Self

/// Expands the trigger width to fill all available horizontal space.
pub fn fill_x(mut self) -> Self

/// Expands the trigger height to fill all available vertical space in the parent container.
pub fn fill_y(mut self) -> Self

/// Sets the fixed height of the dropdown trigger (default 32.0).
pub fn height(mut self, height: f32) -> Self

/// Sets width and height simultaneously.
pub fn size(mut self, w: f32, h: f32) -> Self

/// Sets horizontal padding for trigger label and chevron (default 10.0).
pub fn padding_x(mut self, px: f32) -> Self

/// Sets placement direction relative to the trigger (e.g. Bottom, Top).
pub fn placement(mut self, placement: zenthra_core::Placement) -> Self

/// Sets the vertical/horizontal gap between the trigger and the menu overlay (default 4.0).
pub fn gap(mut self, gap: f32) -> Self
```

---

### 2.3 Trigger Background & Borders (Unified Styling)

Controls the surface color and borders of the dropdown trigger button according to Table 3.6.

#### Method Signatures

```rust
/// Sets trigger background color. Accepts Color, Option<Color>, or None (transparent).
pub fn bg(mut self, bg: impl Into<Option<Color>>) -> Self

/// Sets hover background color. Accepts Color, Option<Color>, or None.
pub fn hover_bg(mut self, color: impl Into<Option<Color>>) -> Self

/// Sets both background and border color simultaneously.
pub fn colors(mut self, bg: Color, border: Color) -> Self

/// Sets trigger border stroke. Accepts (Color, width), Option<(Color, width)>, or None.
pub fn border(mut self, border: impl Into<Border>) -> Self
```

#### Code Examples

```rust
// 1. Solid surface with border
ui.dropdown(&mut selected, options)
    .bg(Color::rgb(0.12, 0.13, 0.16))
    .border((Color::rgb(0.28, 0.32, 0.42), 1.0))
    .show();

// 2. Transparent background (outline dropdown)
ui.dropdown(&mut selected, options)
    .bg(None)
    .border((Color::rgb(0.40, 0.45, 0.60), 1.0))
    .show();

// 3. Completely borderless surface
ui.dropdown(&mut selected, options)
    .border(None)
    .show();
```

---

### 2.4 Corner Radii & Shadows

Corner radii are accurately mapped to the GPU quad renderer. Drop shadows can be applied with blur, offset, and opacity.

#### Method Signatures

```rust
/// Sets uniform radius across all four corners.
pub fn radius_all(mut self, r: f32) -> Self

/// Sets per-corner radii [tl, tr, br, bl].
pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self

pub fn radius_top(mut self, r: f32) -> Self
pub fn radius_bottom(mut self, r: f32) -> Self
pub fn radius_left(mut self, r: f32) -> Self
pub fn radius_right(mut self, r: f32) -> Self
pub fn radius_top_left(mut self, r: f32) -> Self
pub fn radius_top_right(mut self, r: f32) -> Self
pub fn radius_bottom_right(mut self, r: f32) -> Self
pub fn radius_bottom_left(mut self, r: f32) -> Self

/// Adds drop shadow to dropdown trigger and menu.
pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self

/// Multiplies shadow alpha channel.
pub fn shadow_opacity(mut self, opacity: f32) -> Self

/// Enables subtle primary glow on hover.
pub fn glow(mut self, enabled: bool) -> Self
```

---

### 2.5 Floating Menu Customization

Customizes the elevated overlay popup list when the dropdown is opened.

#### Method Signatures

```rust
/// Sets background color of the floating menu overlay (default Color::rgb(0.1, 0.1, 0.12)).
pub fn menu_bg(mut self, color: Color) -> Self

/// Sets maximum scrollable/clipped height of the menu overlay (default 250.0).
pub fn menu_max_height(mut self, max_h: f32) -> Self

/// Sets custom border stroke for the menu overlay.
pub fn menu_border(mut self, border: impl Into<Border>) -> Self

/// Sets uniform corner radius for the floating menu overlay.
pub fn menu_radius_all(mut self, r: f32) -> Self

/// Sets per-corner radii [tl, tr, br, bl] for the floating menu overlay.
pub fn menu_radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self
```

---

### 2.6 Typography & Text Styling

Controls text size and label colors across both the trigger button and menu items.

#### Method Signatures

```rust
/// Sets font size for trigger label and menu items (default 14.0).
pub fn size(mut self, size: f32) -> Self

/// Sets text color for label and chevron.
pub fn color(mut self, color: Color) -> Self
```

---

### 2.7 Event Handling & Response

The `.show()` method records layout, emits draw commands, and returns a standard `Response`:

```rust
let response = ui.dropdown(&mut current_mode, modes).show();

if response.changed {
    println!("Selected mode changed to: {}", current_mode);
}

if response.hovered {
    // Trigger is hovered
}
```

---

## 3. Production Recipes & Patterns

### Recipe 1: Standard Settings Dropdown

```rust
ui.dropdown(&mut selected_format, formats)
    .width(200.0)
    .height(34.0)
    .bg(Color::rgb(0.14, 0.15, 0.18))
    .border((Color::rgb(0.24, 0.26, 0.32), 1.0))
    .radius_all(6.0)
    .show();
```

---

### Recipe 2: Borderless / Transparent Surface Dropdown

```rust
ui.dropdown(&mut current_tab, tabs)
    .bg(None)
    .border(None)
    .hover_bg(Color::rgba(1.0, 1.0, 1.0, 0.08))
    .radius_all(4.0)
    .color(Color::rgb(0.85, 0.88, 0.95))
    .show();
```

---

### Recipe 3: Premium Glowing Selection Control

```rust
ui.dropdown(&mut target_device, devices)
    .width(240.0)
    .bg(Color::rgb(0.10, 0.11, 0.14))
    .border((Color::rgb(0.20, 0.45, 0.85), 1.0))
    .glow(true)
    .shadow(Color::rgb(0.10, 0.35, 0.80), 0.0, 4.0, 16.0)
    .shadow_opacity(0.35)
    .radius_all(8.0)
    .show();
```

---

### Recipe 4: Customized Floating Overlay Menu

```rust
ui.dropdown(&mut selected_branch, branches)
    .width(260.0)
    .menu_bg(Color::rgb(0.08, 0.09, 0.11))
    .menu_border((Color::rgb(0.28, 0.32, 0.40), 1.0))
    .menu_radius_all(10.0)
    .menu_max_height(320.0)
    .gap(6.0)
    .show();
```

---

### Recipe 5: Full-Width Form Field (.fill_x)

```rust
ui.column().gap(8.0).show(|ui| {
    ui.text("Output Resolution")
        .size(13.0)
        .color(Color::rgb(0.6, 0.6, 0.7))
        .show();

    ui.dropdown(&mut selected_res, resolutions)
        .fill_x()
        .height(36.0)
        .radius_all(6.0)
        .show();
});
```

---

## 4. Anti-Patterns & Troubleshooting

### Anti-Pattern 1: Rebuilding the Options Vector Every Frame
❌ **Wrong:** Constructing new `Vec<String>` on every immediate-mode frame causes unnecessary heap allocations.
```rust
// Bad: Allocates a new vector every single frame
ui.dropdown(&mut fruit, vec!["Apple".to_string(), "Banana".to_string()]).show();
```
✅ **Correct:** Store the options in persistent state or pass pre-allocated references/slices.
```rust
// Good: Reuses allocated options vector
ui.dropdown(&mut app_state.fruit, app_state.fruits.clone()).show();
```

### Anti-Pattern 2: Missing `.changed` Check
❌ **Wrong:** Comparing old state manually with temporary variables.
```rust
let prev = selected.clone();
ui.dropdown(&mut selected, options).show();
if prev != selected { ... }
```
✅ **Correct:** Use the idiomatic `response.changed` field.
```rust
if ui.dropdown(&mut selected, options).show().changed {
    trigger_sync();
}
```

---

## 5. Type Definitions

```rust
pub struct DropdownBuilder<'u, 'a, 'b, T: PartialEq + Clone + ToString> {
    ui: &'u mut Ui<'a>,
    selected: &'b mut T,
    options: Vec<T>,
    id: Id,

    // Layout
    width: f32,
    height: f32,
    radius: [f32; 4],
    padding_x: f32,
    font_size: f32,

    // Colors
    bg: Color,
    border_color: Color,
    border_width: f32,
    text_color: Color,
    hover_bg: Option<Color>,

    // Menu Style
    menu_bg: Color,
    menu_max_height: f32,
    menu_border_color: Option<Color>,
    menu_border_width: Option<f32>,
    menu_radius: Option<[f32; 4]>,

    // Styling
    glow: bool,
    shadow_color: Color,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,
    shadow_enabled: bool,

    placement: zenthra_core::Placement,
    gap: f32,
}
```
