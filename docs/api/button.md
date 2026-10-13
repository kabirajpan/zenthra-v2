# Button API Specification & Unified Styling Architecture

`Button` is Zenthra's primary interactive action trigger widget. It provides mouse hover and active press state management, automatic text measurement and layout centering, solid and multi-stop gradient surfaces, state-driven borders and backgrounds, micro-interaction transitions (hover scale, brightness), drop shadows, and event dispatching.

The `Button` widget strictly adheres to the unified styling contract established in [`docs/api/container.md`](./container.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Button API Reference](#2-complete-button-api-reference)
   - [2.1 Construction & Identification](#21-construction--identification)
   - [2.2 Typography & Text Sizing](#22-typography--text-sizing)
   - [2.3 Dimensions & Layout Sizing](#23-dimensions--layout-sizing)
   - [2.4 Backgrounds, Gradients & State Surfaces](#24-backgrounds-gradients--state-surfaces)
   - [2.5 Borders & Interactive Border States](#25-borders--interactive-border-states)
   - [2.6 Corner Radii](#26-corner-radii)
   - [2.7 Shadows & Elevation](#27-shadows--elevation)
   - [2.8 Inner Padding](#28-inner-padding)
   - [2.9 Hover Micro-Interactions](#29-hover-micro-interactions)
   - [2.10 Event Handling & Response](#210-event-handling--response)
3. [Production Recipes & Patterns](#3-production-recipes--patterns)
   - [Recipe 1: Primary Accent Call-to-Action](#recipe-1-primary-accent-call-to-action)
   - [Recipe 2: Ghost / Text-Only Button (Link Style)](#recipe-2-ghost--text-only-button-link-style)
   - [Recipe 3: Outline / Secondary Button](#recipe-3-outline--secondary-button)
   - [Recipe 4: Vibrant Linear Gradient Button](#recipe-4-vibrant-linear-gradient-button)
   - [Recipe 5: Compact Square Tool / Window Button](#recipe-5-compact-square-tool--window-button)
4. [Anti-Patterns & Troubleshooting](#4-anti-patterns--troubleshooting)
5. [Type Definitions](#5-type-definitions)

---

## 1. Design Philosophy & Mental Model

### The Core Mental Model
In Zenthra, a `Button` is an immediate-mode interactive surface that:
- Measures label typography with cosmic-text and centers or aligns it within its allocated box.
- Automatically transitions visual properties between **Idle**, **Hover**, and **Pressed** states.
- Follows the **Unified Styling Contract**: Every property accepts concrete values or `None`.
- Supports transparent ghost/link buttons via `.bg(None)` and `.border(None)`.

```rust
// ✅ Clean, unified button styling (Directly from docs/api/container.md Section 3.4):
ui.button("Export")
    .bg(Color::rgb(0.20, 0.40, 0.85))
    .border((Color::rgb(0.30, 0.50, 0.95), 1.0))
    .radius_all(6.0)
    .show();
```

---

## 2. Complete Button API Reference

### 2.1 Construction & Identification

#### Method Signatures

```rust
// On Ui:
pub fn button(&mut self, label: &str) -> ButtonBuilder<'_, 'a>

// On ButtonBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
pub fn pos(mut self, x: f32, y: f32) -> Self
```

#### Description
- `ui.button("Label")`: Standard constructor. Generates a unique `Id` from `Ui`'s deterministic counter or stack hierarchy.
- `.id(custom_id)`: Explicit ID override when generating buttons inside loops or dynamic lists.
- `.pos(x, y)`: Explicit coordinate placement override instead of automatic layout cursor placement.

---

### 2.2 Typography & Text Sizing

#### Method Signatures

```rust
pub fn size(mut self, size: f32) -> Self
pub fn color(mut self, color: Color) -> Self
pub fn font_family(mut self, family: impl Into<String>) -> Self
pub fn font_weight(mut self, weight: impl Into<zenthra_text::prelude::FontWeight>) -> Self
pub fn wrap(mut self, strategy: impl Into<zenthra_text::prelude::TextWrap>) -> Self
```

#### Defaults
- `size`: `14.0`
- `color`: `Color::WHITE`
- `wrap`: `TextWrap::Word`

```rust
ui.button("Deploy")
    .size(15.0)
    .color(Color::rgb(0.9, 0.95, 1.0))
    .show();
```

---

### 2.3 Dimensions & Layout Sizing

#### Method Signatures

```rust
pub fn width(mut self, w: f32) -> Self
pub fn height(mut self, h: f32) -> Self
pub fn fill_x(mut self) -> Self
pub fn align(mut self, strategy: zenthra_core::Align) -> Self // Default: Align::Center
```

#### Sizing Model
1. **Auto Fit (Default)**: If `.width()` and `.height()` are omitted, the button sizes automatically to fit its label text plus inner padding (`text_w + padding.horizontal()`, `text_h + padding.vertical()`).
2. **Fixed Dimensions**: Setting `.width(w)` and `.height(h)` forces exact box dimensions.
3. **Fill Width (`.fill_x()`)**: Expands width to occupy the remaining available horizontal space.
4. **Text Alignment**: Supports `Align::Left`, `Align::Center`, and `Align::Right` within the allocated button bounds.

```rust
// Full-width form submit button with centered text:
ui.button("Submit Order")
    .fill_x()
    .height(42.0)
    .show();
```

---

### 2.4 Backgrounds, Gradients & State Surfaces

#### Method Signatures

```rust
// Idle Background:
pub fn bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self
pub fn bg_gradient(mut self, start: Color, end: Color, direction: zenthra_core::GradientDirection) -> Self

// Hover State Background:
pub fn hover_bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self

// Active / Pressed State Background:
pub fn active_bg(mut self, bg: impl Into<zenthra_core::Background>) -> Self
```

#### Unified Rules & Defaults
- Default idle background: `Color::rgb(0.2, 0.2, 0.25)`.
- Pass `.bg(None)` to make the button surface transparent (Ghost button).
- If `hover_bg` or `active_bg` is omitted, the button automatically tints its idle background using brightness transitions.

```rust
// Gradient button with custom active color:
ui.button("Get Started")
    .bg_gradient(Color::rgb(0.25, 0.50, 0.95), Color::rgb(0.40, 0.70, 1.00), GradientDirection::LeftToRight)
    .active_bg(Color::rgb(0.18, 0.40, 0.80))
    .show();
```

---

### 2.5 Borders & Interactive Border States

#### Method Signatures

```rust
// Idle Border:
pub fn border(mut self, border: impl Into<zenthra_core::Border>) -> Self

// Hover State Border:
pub fn hover_border(mut self, border: impl Into<zenthra_core::Border>) -> Self

// Active / Pressed State Border:
pub fn active_border(mut self, border: impl Into<zenthra_core::Border>) -> Self
```

#### Behavior & Unified Rules
- `border(...)`:
  - Accepts `(Color, f32)` (e.g. `(Color::WHITE, 1.0)`) or `Border::Solid(c, w)`.
  - Accepts `None` or `Border::None` to remove the border. Default: 0.0 width.
- `hover_border(...)` / `active_border(...)`:
  - Changes the border stroke when hovered or pressed.
  - Accepts `(Color, f32)` to set an accent outline.
  - Accepts `None` or `Border::None` to **explicitly turn off** the border during hover/press.

```rust
// Idle border that illuminates brighter on hover:
ui.button("Inspect")
    .border((Color::rgb(0.30, 0.33, 0.42), 1.0))
    .hover_border((Color::rgb(0.50, 0.75, 1.00), 1.5))
    .active_border((Color::rgb(0.20, 0.50, 0.90), 1.5))
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

---

### 2.7 Shadows & Elevation

#### Method Signatures

```rust
pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self
pub fn shadow_opacity(mut self, opacity: f32) -> Self
pub fn opacity(mut self, o: f32) -> Self
```

```rust
ui.button("Elevated Action")
    .shadow(Color::BLACK, 0.0, 4.0, 10.0)
    .shadow_opacity(0.35)
    .show();
```

---

### 2.8 Inner Padding

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

#### Defaults
- Default: `EdgeInsets::symmetric(6.0, 12.0)` (6px vertical, 12px horizontal).

---

### 2.9 Hover Micro-Interactions

#### Method Signatures

```rust
pub fn hover_scale(mut self, s: f32) -> Self      // Default: 1.0
```

#### Description
- `hover_scale`: Multiplies widget dimensions when hovered (e.g. `1.02` for subtle pop).

---

### 2.10 Event Handling & Response

#### Method Signatures

```rust
pub fn on_click<F>(self, f: F) -> Self where F: FnMut() + 'a
pub fn on_hover<F>(self, f: F) -> Self where F: FnMut(bool) + 'a
pub fn on_event<F>(self, phase: crate::ui::EventPhase, f: F) -> Self where F: FnMut(&mut crate::ui::EventContext, &crate::ui::WidgetEvent) + 'a

pub fn show(self) -> zenthra_core::Response
```

#### Response Flags
`show()` returns `Response`:
- `res.clicked`: True on the single frame where mouse click completed on the button.
- `res.hovered`: True while mouse pointer is within button bounds.
- `res.pressed`: True while mouse button is held down on the button.

```rust
// Idiomatic immediate-mode usage:
if ui.button("Save Changes").show().clicked {
    save_document();
}

// Or callback-based usage:
ui.button("Discard")
    .on_click(|| println!("Discarded!"))
    .show();
```

---

## 3. Production Recipes & Patterns

### Recipe 1: Primary Accent Call-to-Action

A solid raised primary button with comfortable padding and rounded corners:

```rust
if ui.button("Save Project")
    .bg(Color::rgb(0.20, 0.45, 0.90))
    .hover_bg(Color::rgb(0.25, 0.52, 0.98))
    .active_bg(Color::rgb(0.16, 0.38, 0.80))
    .border((Color::rgb(0.30, 0.55, 1.00), 1.0))
    .radius_all(6.0)
    .padding(8.0, 16.0, 8.0, 16.0)
    .shadow(Color::BLACK, 0.0, 3.0, 8.0)
    .shadow_opacity(0.30)
    .show()
    .clicked 
{
    save_project();
}
```

---

### Recipe 2: Ghost / Text-Only Button (Link Style)

A transparent text-only button with zero borders (directly from [`docs/api/container.md`](./container.md) Section 3.4):

```rust
if ui.button("Cancel")
    .bg(None)                                   // Transparent surface
    .border(None)                               // No border
    .color(Color::rgb(0.70, 0.75, 0.85))
    .show()
    .clicked 
{
    close_modal();
}
```

---

### Recipe 3: Outline / Secondary Button

A neutral outline button that lights up on hover:

```rust
ui.button("Export As...")
    .bg(Color::rgb(0.12, 0.13, 0.18))
    .border((Color::rgb(0.26, 0.28, 0.38), 1.0))
    .hover_border((Color::rgb(0.45, 0.70, 1.00), 1.0))
    .radius_all(6.0)
    .padding(8.0, 14.0, 8.0, 14.0)
    .show();
```

---

### Recipe 4: Vibrant Linear Gradient Button

A modern gradient button with subtle hover scale lift:

```rust
ui.button("Upgrade to Pro")
    .bg_gradient(Color::rgb(0.95, 0.40, 0.20), Color::rgb(0.98, 0.65, 0.25), GradientDirection::LeftToRight)
    .radius_all(18.0) // Pill shape
    .padding(8.0, 20.0, 8.0, 20.0)
    .hover_scale(1.03)
    .show();
```

---

### Recipe 5: Compact Square Tool / Window Button

Fixed 28x28 square button with transparent surface for headers and window controls:

```rust
ui.row().gap(4.0).valign(Align::Center).show(|ui| {
    if ui.button("—").width(28.0).height(28.0).bg(None).radius_all(4.0).show().clicked {
        minimize_window();
    }
    if ui.button("✕").width(28.0).height(28.0).bg(None).radius_all(4.0).show().clicked {
        close_window();
    }
});
```

---

## 4. Anti-Patterns & Troubleshooting

### Anti-Pattern 1: Wrapping a Button in a Container for Hover Borders
**Wrong:**
```rust
// ❌ Redundant container wrapper just to draw hover borders:
ui.container()
    .border(if is_hovered { (Color::BLUE, 1.0) } else { (Color::GRAY, 1.0) })
    .show(|ui| {
        ui.button("Click").show();
    });
```
**Right:**
```rust
// ✅ Button has first-class state borders:
ui.button("Click")
    .border((Color::GRAY, 1.0))
    .hover_border((Color::BLUE, 1.0))
    .show();
```

### Anti-Pattern 2: Forgetting to Return the Response in Loops
**Wrong:**
```rust
for action in actions {
    ui.button(action); // ❌ Button never evaluated!
}
```
**Right:**
```rust
for action in actions {
    if ui.button(action).show().clicked {
        execute(action);
    }
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

pub enum Background {
    None,
    Solid(Color),
    Gradient(Gradient),
}

pub struct Response {
    pub clicked: bool,
    pub hovered: bool,
    pub pressed: bool,
    pub submitted: bool,
    pub changed: bool,
}
```
