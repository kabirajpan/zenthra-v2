# Dropdown API Specification & Unified Styling Architecture

`Dropdown` is Zenthra's primary single-selection menu control. It binds directly to an immediate-mode generic `&mut T` state (where `T: PartialEq + Clone + ToString`), rendering a compact clickable trigger box with chevron icon and an elevated overlay popover list when open. It features outside-click dismissal, custom placement, scrollable menu bounds, typography control, and full conformance to the unified styling contract.

The `Dropdown` widget strictly adheres to the unified styling contract established in [`docs/api/container.md`](./container.md) and [`docs/api/macros.md`](./macros.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Dropdown API Reference](#2-complete-dropdown-api-reference)
   - [2.1 Construction & Generic State Binding](#21-construction--generic-state-binding)
   - [2.2 Dimensions & Layout (.width, .height, .fill_x, .padding, .margin)](#22-dimensions--layout-width-height-fill_x-padding-margin)
   - [2.3 Trigger Styling (`BoxStyle` Parity)](#23-trigger-styling-boxstyle-parity)
   - [2.4 Corner Radii & Shadows](#24-corner-radii--shadows)
   - [2.5 Typography & Text Styling](#25-typography--text-styling)
   - [2.6 Interaction States & Hover/Active Overrides](#26-interaction-states--hoveractive-overrides)
   - [2.7 Floating Menu Overlay Customization](#27-floating-menu-overlay-customization)
   - [2.8 Event Handling & Response](#28-event-handling--response)
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
- **Unified `BoxStyle` Architecture**: Implements `zenthra_core::Styleable`, storing trigger styling in `self.style: BoxStyle` and typography in `self.text_style: TextStyle`.
- **State Blending**: State overrides (`.hover([ ... ])` and `.active([ ... ])`) blend seamlessly over the base trigger `BoxStyle`.
- **Portal Overlay Rendering**: The popup item list is rendered into `ui.overlays`, ensuring menu items hover above sibling and parent widgets without clipping.
- **Outside-Click Dismissal**: Automatically registers an overlay hit-catcher to close the menu when clicking anywhere outside.

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

### 2.2 Dimensions & Layout (.width, .height, .fill_x, .padding, .margin)

`DropdownBuilder` provides full layout and dimension setters conforming to `macros.md`:

| Method | Description |
| :--- | :--- |
| `.width(w: f32)` | Fixed trigger width (default: 180.0). Sets `fill_x = false`. |
| `.height(h: f32)` | Fixed trigger height (default: 32.0). Sets `fill_y = false`. |
| `.fill_x()` | Expands trigger horizontally to fill available width. |
| `.fill_y()` | Expands trigger vertically to fill available height. |
| `.fill()` | Expands both horizontally and vertically. |
| `.min_width(w)` / `.max_width(w)` | Clamp width bounds. |
| `.min_height(h)` / `.max_height(h)` | Clamp height bounds. |
| `.padding(t, r, b, l)` | Per-edge internal padding for trigger content. |
| `.padding_all(p: f32)` | Uniform padding. Default is `(0.0, 10.0, 0.0, 10.0)`. |
| `.padding_x(px)` / `.padding_y(py)` | Axis-specific padding. |
| `.margin(t, r, b, l)` / `.margin_all(m)` | Outer container spacing. |
| `.placement(Placement)` | Overlay alignment relative to trigger (`Bottom`, `Top`, etc.). |
| `.gap(f32)` | Gap in logical pixels between trigger and menu overlay (default: 4.0). |

---

### 2.3 Trigger Styling (`BoxStyle` Parity)

Controls the surface color and borders of the dropdown trigger button according to `macros.md`:

| Method | Description |
| :--- | :--- |
| `.style(BoxStyle)` | Replaces entire underlying `BoxStyle` at once. |
| `.bg(impl Into<Background>)` | Sets background (solid `Color`, `Gradient`, or `None`). |
| `.border(impl IntoBorder)` | Sets border stroke `(Color, width)`, `(Color, width, BorderAlignment)`, or `Border::None`. |
| `.shadow(color, x, y, blur)` | Adds drop shadow with offset and blur radius. |
| `.glow(enabled: bool)` | Enables subtle ambient accent glow. |

---

### 2.4 Corner Radii & Shadows

Corner radii are accurately mapped to the GPU quad renderer:

| Method | Description |
| :--- | :--- |
| `.radius_all(r: f32)` | Uniform radius across all four corners. Default is `4.0`. |
| `.radius(tl, tr, br, bl)` | Per-corner radii `[tl, tr, br, bl]`. |
| `.radius_top(r)` / `.radius_bottom(r)` | Top or bottom pair corner radii. |
| `.radius_left(r)` / `.radius_right(r)` | Left or right pair corner radii. |
| `.radius_top_left(r)`, `.radius_top_right(r)`, etc. | Specific single-corner radius. |

---

### 2.5 Typography & Text Styling

Controls text size, color, family, and weight across the trigger button and menu items:

| Method | Description |
| :--- | :--- |
| `.size(size: f32)` | Sets typography font size (default: 14.0). |
| `.color(color: Color)` | Sets text glyph color (default: `Color::WHITE`). |
| `.font_family(impl Into<String>)` | Custom font family (e.g. `"Inter"`, `"Fira Code"`). |
| `.font_weight(impl Into<FontWeight>)` | Font weight (`FontWeight::Medium`, `600`, etc.). |

---

### 2.6 Interaction States & Hover/Active Overrides

Interactive states use Zenthra's zero-cost Sibling architecture:

```rust
ui.dropdown(&mut current_mode, modes)
    .bg(Color::rgb(0.12, 0.13, 0.16))
    .border((Color::rgb(0.25, 0.28, 0.35), 1.0))
    .hover([
        Style::bg(Color::rgb(0.16, 0.18, 0.24)),
        Style::border((Color::rgb(0.35, 0.55, 0.95), 1.5)),
        Cursor::Pointer,
    ])
    .active([
        Style::border((Color::rgb(0.45, 0.70, 1.0), 1.5)),
        Transform::scale(0.98),
    ])
    .show();
```

---

### 2.7 Floating Menu Overlay Customization

Customizes the elevated overlay popup list when the dropdown is opened:

| Method | Description |
| :--- | :--- |
| `.menu_style(BoxStyle)` | Replaces the entire menu overlay `BoxStyle`. |
| `.menu_bg(impl Into<Background>)` | Background for the menu overlay (default `Color::rgb(0.1, 0.1, 0.12)`). |
| `.menu_border(impl IntoBorder)` | Border for the menu overlay (default `(Color::rgb(0.25, 0.25, 0.3), 1.0)`). |
| `.menu_radius_all(r: f32)` | Corner radius for the menu overlay (default `4.0`). |
| `.menu_radius(tl, tr, br, bl)` | Per-corner radii for the menu overlay. |
| `.menu_max_height(max_h: f32)` | Maximum scrollable/clipped height of the popup list (default: 250.0). |

---

### 2.8 Event Handling & Response

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
    .bg(Background::None)
    .border(Border::None)
    .hover([
        Style::bg(Color::rgba(1.0, 1.0, 1.0, 0.08)),
        Cursor::Pointer,
    ])
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
    .shadow(Color::rgba(0.10, 0.35, 0.80, 0.35), 0.0, 4.0, 16.0)
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
    pub ui: &'u mut Ui<'a>,
    pub selected: &'b mut T,
    pub options: Vec<T>,
    pub id: Id,

    // Header styling & Typography
    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,

    // Menu Style (Dropdown popup)
    pub menu_style: BoxStyle,
    pub menu_max_height: f32,

    // Styling & Effects
    pub glow: bool,
    pub placement: zenthra_core::Placement,
    pub gap: f32,
}
```
