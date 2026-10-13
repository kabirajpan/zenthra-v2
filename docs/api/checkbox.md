# Checkbox API Specification & Unified Styling Architecture

`Checkbox` is Zenthra's primary boolean selection control. It binds directly to an immediate-mode `&mut bool` state, providing animated transitions, custom box and checkmark colors, typography control, interactive hover/pressed micro-animations, glowing effects, and unified styling (background surfaces, borders, corner radii, and drop shadows).

The `Checkbox` widget strictly adheres to the unified styling contract established in [`docs/api/container.md`](./container.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Checkbox API Reference](#2-complete-checkbox-api-reference)
   - [2.1 Construction & State Binding](#21-construction--state-binding)
   - [2.2 Box Dimensions & Label Spacing](#22-box-dimensions--label-spacing)
   - [2.3 Box Background & Border (Unified Styling)](#23-box-background--border-unified-styling)
   - [2.4 Checked State Customization](#24-checked-state-customization)
   - [2.5 Corner Radii & Shadows](#25-corner-radii--shadows)
   - [2.6 Label Typography & Active Color](#26-label-typography--active-color)
   - [2.7 Micro-Interactions, Scaling & Glow](#27-micro-interactions-scaling--glow)
   - [2.8 Event Handling & Response](#28-event-handling--response)
3. [Production Recipes & Patterns](#3-production-recipes--patterns)
   - [Recipe 1: Standard Settings Toggle Checkbox](#recipe-1-standard-settings-toggle-checkbox)
   - [Recipe 2: Outline Checkbox with Transparent Background](#recipe-2-outline-checkbox-with-transparent-background)
   - [Recipe 3: Premium Glowing Checkbox (Accent Glow)](#recipe-3-premium-glowing-checkbox-accent-glow)
   - [Recipe 4: Active Label Color Transition](#recipe-4-active-label-color-transition)
   - [Recipe 5: Compact Dense Options List in Column](#recipe-5-compact-dense-options-list-in-column)
4. [Anti-Patterns & Troubleshooting](#4-anti-patterns--troubleshooting)
5. [Type Definitions](#5-type-definitions)

---

## 1. Design Philosophy & Mental Model

### The Core Mental Model
In Zenthra, a `Checkbox`:
- Directly mutates an immediate-mode `&mut bool` reference on user click.
- Renders a styled checkbox quad, an animated checkmark glyph (`✓`), and an adjacent text label separated by `.gap(...)`.
- Emits smooth scale and color interpolation across frames via `ui.interaction_state`.
- Follows the **Unified Styling Contract**: Every property accepts concrete values or `None`.

```rust
// ✅ Clean, consistent Zenthra styling:
ui.checkbox(&mut auto_save, "Auto-save changes")
    .bg(Color::rgb(0.12, 0.13, 0.18))
    .border((Color::rgb(0.25, 0.28, 0.38), 1.0))
    .check_bg(Color::rgb(0.20, 0.55, 1.00))
    .radius_all(4.0)
    .show();
```

---

## 2. Complete Checkbox API Reference

### 2.1 Construction & State Binding

#### Method Signatures

```rust
// On Ui:
pub fn checkbox<'b>(
    &mut self, 
    value: &'b mut bool, 
    label: &str
) -> CheckboxBuilder<'_, 'a, 'b>

// On CheckboxBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
```

#### Description
- `ui.checkbox(value, label)` initializes a `CheckboxBuilder` bound to `&mut bool`.
- Generates a deterministic `Id` from label text and hierarchy or can be explicitly overridden with `.id(...)`.

---

### 2.2 Box Dimensions & Label Spacing

#### Method Signatures

```rust
pub fn box_size(mut self, size: f32) -> Self // Default: 18.0
pub fn gap(mut self, gap: f32) -> Self      // Default: 8.0
```

#### Description
- `box_size`: Sets width and height of the checkbox box (in logical pixels).
- `gap`: Horizontal spacing between the right edge of the checkbox box and the start of the label text.

```rust
// Compact 14px checkbox with tight 6px gap:
ui.checkbox(&mut enabled, "Snap to Grid")
    .box_size(14.0)
    .gap(6.0)
    .show();
```

---

### 2.3 Box Background & Border (Unified Styling)

#### Method Signatures

```rust
pub fn bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self
pub fn border(mut self, border: impl IntoBorder) -> Self
```

#### Unified Rules & Defaults
- Default unchecked background: `Color::rgb(0.15, 0.15, 0.15)`.
- Default unchecked border: `(Color::rgb(0.3, 0.3, 0.3), 1.0)`.
- `.bg(...)` accepts `Color`, `Background::Solid(...)`, `Background::Gradient(...)`, or `None` for transparency.
- `.border(...)` accepts `(Color, width)`, `(Color, width, BorderAlignment)`, `Border`, or `None` to remove the border stroke.

```rust
// Transparent box with crisp white outline:
ui.checkbox(&mut show_guides, "Show Alignment Guides")
    .bg(None)
    .border((Color::WHITE, 1.0))
    .show();
```

---

### 2.4 Checked State Customization

#### Method Signatures

```rust
pub fn check_bg(mut self, bg: impl Into<Option<Color>>) -> Self
pub fn check_color(mut self, color: Color) -> Self
```

#### Defaults
- Default `check_bg`: Accent Blue `Color::rgb(0.2, 0.5, 1.0)`.
- Default `check_color`: `Color::WHITE` (the `✓` checkmark glyph).

```rust
// Emerald green checkmark when active:
ui.checkbox(&mut verified, "Verified Account")
    .check_bg(Color::rgb(0.15, 0.75, 0.45))
    .check_color(Color::WHITE)
    .show();
```

---

### 2.5 Corner Radii & Shadows

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
```

#### Defaults
- Default radius: `[4.0; 4]` (uniform 4px rounded corners).
- Shadow opacity is configured directly via `Color::rgba(...)`.

```rust
ui.checkbox(&mut opt, "Option")
    .radius_all(9.0) // Circular / radio style
    .shadow(Color::rgba(0.0, 0.0, 0.0, 0.3), 0.0, 2.0, 4.0)
    .show();
```

---

### 2.6 Label Typography & Active Color

#### Method Signatures

```rust
// Individual Typography Setters (via impl_text_style_methods!)
pub fn size(mut self, size: f32) -> Self
pub fn color(mut self, color: impl Into<Color>) -> Self
pub fn font_weight(mut self, weight: impl Into<FontWeight>) -> Self
pub fn font_family(mut self, family: impl Into<String>) -> Self
pub fn decoration(mut self, dec: impl IntoDecoration) -> Self

// Active Checked Label
pub fn active_label(mut self, color: Color) -> Self
```

#### Defaults
- `size`: `14.0`
- `color`: `Color::WHITE`
- `active_label`: If set, the text smoothly transitions to this color when checked.

```rust
ui.checkbox(&mut active, "Enable High Performance")
    .color(Color::rgb(0.6, 0.6, 0.7))
    .active_label(Color::rgb(0.3, 0.7, 1.0)) // Illuminates text when active
    .show();
```

---

### 2.7 Micro-Interactions, Scaling & Glow

#### Method Signatures

```rust
pub fn glow(mut self, enabled: bool) -> Self
pub fn scaling(mut self, hover: f32, pressed: f32) -> Self
pub fn hover_bg(mut self, color: Color) -> Self
pub fn active_bg(mut self, color: Color) -> Self
```

#### Description
- `glow(true)`: Emits a soft outer neon glow matching `check_bg` when the checkbox is active.
- `scaling(hover, pressed)`: Smoothly scales the box on hover (e.g. `1.05`) and press (e.g. `0.95`).
- `hover_bg` / `active_bg`: Overrides box color on hover or mouse down.

---

### 2.8 Event Handling & Response

#### Method Signatures

```rust
pub fn on_click<F>(self, f: F) -> Self where F: FnMut() + 'a
pub fn on_change<F>(self, f: F) -> Self where F: FnMut(bool) + 'a
pub fn on_hover<F>(self, f: F) -> Self where F: FnMut(bool) + 'a

pub fn show(self) -> zenthra_core::Response
```

#### Response Flags
`show()` returns `Response`:
- `res.changed`: True on the single frame where value toggled.
- `res.clicked`: True when the user clicked the checkbox.
- `res.hovered`: True while the mouse pointer is over the checkbox bounds.
- `res.pressed`: True while the mouse button is down.

```rust
if ui.checkbox(&mut vsync, "Vertical Sync").show().changed {
    reconfigure_graphics_pipeline(vsync);
}
```

---

## 3. Production Recipes & Patterns

### Recipe 1: Standard Settings Toggle Checkbox

Clean, dark-mode settings checkbox with rounded corners:

```rust
ui.checkbox(&mut notifications, "Enable desktop notifications")
    .box_size(16.0)
    .gap(10.0)
    .size(13.0)
    .bg(Color::rgb(0.12, 0.13, 0.17))
    .border((Color::rgb(0.24, 0.26, 0.34), 1.0))
    .check_bg(Color::rgb(0.25, 0.55, 0.98))
    .radius_all(4.0)
    .show();
```

---

### Recipe 2: Outline Checkbox with Transparent Background

Transparent checkbox surface with subtle border and crisp white checkmark:

```rust
ui.checkbox(&mut dark_mode, "Force Dark Theme")
    .bg(None)
    .border((Color::rgb(0.40, 0.45, 0.55), 1.0))
    .check_bg(Color::rgb(0.20, 0.50, 0.90))
    .radius_all(4.0)
    .show();
```

---

### Recipe 3: Premium Glowing Checkbox (Accent Glow)

Vibrant cyan checkbox with animated scaling and neon glow:

```rust
ui.checkbox(&mut turbo_mode, "Turbo Engine Boost")
    .box_size(20.0)
    .check_bg(Color::rgb(0.10, 0.85, 0.95))
    .check_color(Color::rgb(0.02, 0.05, 0.10))
    .glow(true)
    .scaling(1.06, 0.94)
    .radius_all(6.0)
    .show();
```

---

### Recipe 4: Active Label Color Transition

Dimmed inactive label that illuminates into white/accent color upon activation:

```rust
ui.checkbox(&mut sync_enabled, "Cloud Synchronization")
    .color(Color::rgb(0.50, 0.55, 0.65))
    .active_label(Color::rgb(0.95, 0.98, 1.00))
    .check_bg(Color::rgb(0.30, 0.60, 1.00))
    .show();
```

---

### Recipe 5: Compact Dense Options List in Column

Building a clean preferences checklist:

```rust
ui.column().spacing(6.0).show(|ui| {
    ui.checkbox(&mut show_grid, "Grid Overlay").box_size(14.0).show();
    ui.checkbox(&mut snap_to_pixel, "Pixel Snapping").box_size(14.0).show();
    ui.checkbox(&mut show_rulers, "Rulers & Guides").box_size(14.0).show();
});
```

---

## 4. Anti-Patterns & Troubleshooting

### Anti-Pattern 1: Manually Inverting `*value` in `on_click`
`Checkbox` automatically inverts `*value` when clicked. Do NOT manually flip `*value` inside an `on_click` or `if res.clicked` block, or the value will flip twice (canceling itself out):

**Wrong:**
```rust
if ui.checkbox(&mut is_active, "Active").show().clicked {
    is_active = !is_active; // ❌ Flips twice!
}
```

**Right:**
```rust
// ✅ Automatically mutated:
if ui.checkbox(&mut is_active, "Active").show().changed {
    println!("New state: {}", is_active);
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
