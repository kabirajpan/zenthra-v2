# Container API Specification & Unified Styling Architecture

`Container` is the foundational structural and aesthetic building block in Zenthra. It provides layout flow (row, column, stack), dimensional sizing, inner padding, clipping, scrolling, and visual decoration (backgrounds, borders, corner radii, and drop shadows).

Furthermore, the styling conventions established by `Container` serve as the **canonical design blueprint** for all other interactive widgets in Zenthra (`Input`, `DragValue`, `Button`, `TextArea`, `Dropdown`, `Text`).

---

## Table of Contents

1. [Design Philosophy & Unified Styling Rules](#1-design-philosophy--unified-styling-rules)
2. [Complete Container API Reference](#2-complete-container-api-reference)
   - [2.1 Backgrounds & Gradients](#21-backgrounds--gradients)
   - [2.2 Borders & Outlines](#22-borders--outlines)
   - [2.3 Corner Radii](#23-corner-radii)
   - [2.4 Shadows & Elevation](#24-shadows--elevation)
   - [2.5 Inner Spacing (Padding)](#25-inner-spacing-padding)
   - [2.6 Dimensions & Constraints](#26-dimensions--constraints)
   - [2.7 Layout Direction & Flow](#27-layout-direction--flow)
   - [2.8 Alignment & Justification](#28-alignment--justification)
   - [2.9 Scrolling, Viewports & Clipping](#29-scrolling-viewports--clipping)
   - [2.10 Pointer Interaction & Window Dragging](#210-pointer-interaction--window-dragging)
3. [Unified Styling Rules Across All Widgets](#3-unified-styling-rules-across-all-widgets)
   - [3.1 The "No-Magic" Styling Contract](#31-the-no-magic-styling-contract)
   - [3.2 Input Widget Styling](#32-input-widget-styling)
   - [3.3 DragValue / NumberBox Styling](#33-dragvalue--numberbox-styling)
   - [3.4 Button Widget Styling](#34-button-widget-styling)
   - [3.5 Text & TextArea Styling](#35-text--textarea-styling)
   - [3.6 Cross-Widget Style Reference Table](#36-cross-widget-style-reference-table)
4. [Production Recipes & Patterns](#4-production-recipes--patterns)
   - [Recipe 1: Sleek Inspector Panel](#recipe-1-sleek-inspector-panel)
   - [Recipe 2: Compact Property Row](#recipe-2-compact-property-row)
   - [Recipe 3: Floating Modal Card with Depth](#recipe-3-floating-modal-card-with-depth)
   - [Recipe 4: Scrollable Virtualized Asset List](#recipe-4-scrollable-virtualized-asset-list)
   - [Recipe 5: Draggable Title Bar & Window Header](#recipe-5-draggable-title-bar--window-header)
5. [Anti-Patterns & Troubleshooting](#5-anti-patterns--troubleshooting)
6. [Type Definitions & Enums](#6-type-definitions--enums)

---

## 1. Design Philosophy & Unified Styling Rules

### The Core Mental Model
In Zenthra, a `Container` is both a **layout orchestrator** and a **canvas decorator**:
- It translates and arranges child widgets along coordinate axes (`Direction::Row`, `Direction::Column`, `Direction::Stack`).
- It establishes clipping boundaries and handles virtual scroll offsets.
- It draws background quads, multi-stop gradients, borders, and drop shadows.

### Clean, Predictable State Over Negative Flags
Older UI toolkits often accumulate negative modifier flags such as `.no_border()`, `.no_bg()`, or `.borderless()`. This fragments the API:
- It creates two separate ways to set an attribute (calling a setter vs calling an un-setter).
- It breaks builder composability when conditional themes are applied.
- Different widgets inevitably name these methods differently (`borderless` vs `no_border` vs `without_frame`).

**Zenthra's Unified Rule:**
Every style property accepts either its concrete value or `None`.

```rust
// ✅ The Zenthra Standard:
ui.container()
    .bg(Color::rgb(0.12, 0.13, 0.18))             // Set background
    .bg(None)                                     // Remove background
    .border((Color::rgb(0.22, 0.25, 0.35), 1.0))  // Set border
    .border(None);                                // Remove border
```

This single rule applies identically to `Container`, `Input`, `DragValue`, `Button`, and all other widgets.

---

## 2. Complete Container API Reference

### 2.1 Backgrounds & Gradients

The container background can be a solid color, a linear/radial gradient, or completely transparent. Opacity/transparency is configured directly on the color using `Color::rgba(...)` or `Color::with_alpha(...)`.

#### Method Signatures

```rust
/// Sets the background using a Color, Gradient, or Background enum.
/// Pass `None` or `Color::TRANSPARENT` to render a completely transparent container.
pub fn bg(mut self, b: impl Into<Option<Background>>) -> Self
```

#### Code Examples

```rust
// 1. Solid Color Background (with optional alpha)
ui.container()
    .bg(Color::rgba(0.10, 0.11, 0.15, 0.95))
    .show(|ui| { ... });

// 2. Linear Gradient (Horizontal or Vertical)
ui.container()
    .bg(Gradient::linear(
        Color::rgb(0.15, 0.25, 0.45),
        Color::rgb(0.05, 0.08, 0.15),
        GradientDirection::Vertical
    ))
    .show(|ui| { ... });

// 3. Radial Gradient (Focused highlight)
ui.container()
    .bg(Gradient::radial(
        Color::rgb(0.30, 0.45, 0.85),
        Color::rgb(0.08, 0.09, 0.12),
    ))
    .show(|ui| { ... });

// 4. Transparent Layout Shell (Default)
ui.container()
    .bg(None) // Or omit .bg() completely
    .show(|ui| { ... });
```

---

### 2.2 Borders & Outlines

Borders outline the container boundary with pixel-perfect anti-aliased geometry.

#### Method Signatures

```rust
/// Sets the border styling.
/// Accepts:
/// - `(Color, f32)` or `(Color, f64)`
/// - `(Color, f32, BorderAlignment)` or `(Color, f64, BorderAlignment)`
/// - `Border` or `(Border, BorderAlignment)`
/// - `None` or `Option<Border>`
pub fn border(mut self, border: impl IntoBorder) -> Self
```

#### Border Alignments

| Alignment | Description | Common Use Case |
| :--- | :--- | :--- |
| `BorderAlignment::Inside` | Stroke renders inside the container bounds. Children do not overlap it. | **Default.** Forms, buttons, cards. |
| `BorderAlignment::Center` | Stroke is centered on the outer edge half-in, half-out. | Graphic editors, canvas viewports. |
| `BorderAlignment::Outside` | Stroke expands outward beyond the container rect. | Selection outlines, focus rings. |

#### Code Examples

```rust
// 1. Standard 1px Subtle Border (Default Inside Alignment)
ui.container()
    .border((Color::rgb(0.20, 0.23, 0.32), 1.0))
    .show(|ui| { ... });

// 2. Thick Accent Border with Explicit Alignment (Unified 3-Tuple)
ui.container()
    .border((Color::rgb(0.35, 0.58, 0.98), 2.0, BorderAlignment::Inside))
    .show(|ui| { ... });

// 3. Selection Focus Ring (Outside Alignment)
ui.container()
    .border((Color::BLUE, 2.0, BorderAlignment::Outside))
    .show(|ui| { ... });

// 4. Explicitly Borderless
ui.container()
    .border(None)
    .show(|ui| { ... });
```

---

### 2.3 Corner Radii

Corners can be uniformly rounded or individually sculpted for asymmetric tab or pill designs.

#### Method Signatures

```rust
/// Rounds all 4 corners with the identical radius.
pub fn radius_all(mut self, r: f32) -> Self

/// Configures all 4 corners individually: [top_left, top_right, bottom_right, bottom_left].
pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self

/// Rounds both top corners (top_left, top_right).
pub fn radius_top(mut self, r: f32) -> Self

/// Rounds both bottom corners (bottom_left, bottom_right).
pub fn radius_bottom(mut self, r: f32) -> Self

/// Rounds both left corners (top_left, bottom_left).
pub fn radius_left(mut self, r: f32) -> Self

/// Rounds both right corners (top_right, bottom_right).
pub fn radius_right(mut self, r: f32) -> Self

/// Granular individual corner setters:
pub fn radius_top_left(mut self, r: f32) -> Self
pub fn radius_top_right(mut self, r: f32) -> Self
pub fn radius_bottom_left(mut self, r: f32) -> Self
pub fn radius_bottom_right(mut self, r: f32) -> Self
```

#### Code Examples

```rust
// 1. Standard Soft Card Corners
ui.container()
    .radius_all(8.0)
    .show(|ui| { ... });

// 2. Browser Tab Style (Only Top Corners Rounded)
ui.container()
    .radius_top(6.0)
    .show(|ui| { ... });

// 3. Pill / Capsule Tag
ui.container()
    .height(24.0)
    .radius_all(12.0)
    .show(|ui| { ... });

// 4. Asymmetric Decorative Ribbon
ui.container()
    .radius(16.0, 0.0, 16.0, 0.0)
    .show(|ui| { ... });
```

---

### 2.4 Shadows & Elevation

Drop shadows provide visual depth, separating elevated panels, modals, and tooltips from the background.

#### Method Signatures

```rust
/// Applies an outer drop shadow.
/// Parameters: (color, offset_x, offset_y, blur_radius)
/// Shadow transparency is configured directly using `Color::rgba(...)`.
pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self
```

#### Code Examples

```rust
// 1. Subtle Dark Surface Elevation (Level 1)
ui.container()
    .bg(Color::rgb(0.14, 0.16, 0.22))
    .shadow(Color::rgba(0.0, 0.0, 0.0, 0.35), 0.0, 2.0, 6.0)
    .show(|ui| { ... });

// 2. High Floating Modal Shadow (Level 3)
ui.container()
    .bg(Color::rgb(0.16, 0.18, 0.26))
    .shadow(Color::rgba(0.0, 0.0, 0.0, 0.60), 0.0, 10.0, 28.0)
    .show(|ui| { ... });

// 3. Glowing Neon Accent Shadow
ui.container()
    .border((Color::rgb(0.35, 0.58, 0.98), 1.0))
    .shadow(Color::rgba(0.35, 0.58, 0.98, 0.40), 0.0, 0.0, 12.0)
    .show(|ui| { ... });
```

---

### 2.5 Inner Spacing (Padding)

Padding establishes breathing room between the container boundaries and its contained children.

#### Method Signatures

```rust
/// Sets inner padding for all four edges: (top, right, bottom, left).
pub fn padding(mut self, top: f32, right: f32, bottom: f32, left: f32) -> Self

/// Applies identical padding to all four edges.
pub fn padding_all(mut self, p: f32) -> Self

/// Applies symmetrical horizontal padding (left and right).
pub fn padding_x(mut self, p: f32) -> Self

/// Applies symmetrical vertical padding (top and bottom).
pub fn padding_y(mut self, p: f32) -> Self

/// Granular edge setters:
pub fn padding_top(mut self, p: f32) -> Self
pub fn padding_bottom(mut self, p: f32) -> Self
pub fn padding_left(mut self, p: f32) -> Self
pub fn padding_right(mut self, p: f32) -> Self
```

#### Code Examples

```rust
// 1. Balanced UI Card Padding
ui.container()
    .padding(14.0, 16.0, 14.0, 16.0)
    .show(|ui| { ... });

// 2. Symmetrical Form Row
ui.container()
    .padding_x(12.0)
    .padding_y(8.0)
    .show(|ui| { ... });

// 3. Header Title Bar
ui.container()
    .padding_left(20.0)
    .padding_right(12.0)
    .padding_y(6.0)
    .show(|ui| { ... });
```

---

### 2.6 Dimensions & Constraints

Containers can have fixed dimensions, fill available layout space, or adhere to minimum and maximum constraints.

#### Method Signatures

```rust
/// Sets an explicit fixed width in logical pixels.
pub fn width(mut self, w: f32) -> Self

/// Sets an explicit fixed height in logical pixels.
pub fn height(mut self, h: f32) -> Self

/// Expands the container to consume 100% of the parent layout's available width.
pub fn fill_x(mut self) -> Self

/// Expands the container to consume 100% of the parent layout's available height.
pub fn fill_y(mut self) -> Self

/// Expands the container across both X and Y axes.
pub fn fill(mut self) -> Self

/// Minimum width boundary.
pub fn min_width(mut self, w: f32) -> Self

/// Maximum width boundary (children layout width will not exceed this).
pub fn max_width(mut self, w: f32) -> Self

/// Minimum height boundary.
pub fn min_height(mut self, h: f32) -> Self

/// Maximum height boundary.
pub fn max_height(mut self, h: f32) -> Self
```

#### Code Examples

```rust
// 1. Full Screen / Viewport Background Wrapper
ui.container()
    .fill_x()
    .fill_y()
    .bg(Color::rgb(0.08, 0.09, 0.12))
    .show(|ui| { ... });

// 2. Fixed Width Inspector Sidebar
ui.container()
    .width(280.0)
    .fill_y()
    .bg(Color::rgb(0.12, 0.13, 0.18))
    .show(|ui| { ... });

// 3. Constrained Modal Window
ui.container()
    .min_width(320.0)
    .max_width(540.0)
    .padding_all(20.0)
    .show(|ui| { ... });
```

---

### 2.7 Layout Direction & Flow

The container determines how its child widgets are positioned in sequence.

#### Method Signatures

```rust
/// Arranges child widgets horizontally from left to right.
pub fn row(mut self) -> Self

/// Arranges child widgets vertically from top to bottom (Default).
pub fn column(mut self) -> Self

/// Places child widgets on top of one another at the same base origin (Z-Stack).
pub fn stack(mut self) -> Self

/// Sets the spacing gap in logical pixels between adjacent child widgets.
pub fn gap(mut self, g: f32) -> Self

/// Sets the child flex wrapping strategy (`Wrap::NoWrap`, `Wrap::Wrap`, `Wrap::WrapReverse`).
pub fn wrap(mut self, strategy: Wrap) -> Self
```

#### Code Examples

```rust
// 1. Horizontal Toolbar Row
ui.container()
    .row()
    .gap(8.0)
    .valign(Align::Center)
    .show(|ui| {
        ui.button("New").show();
        ui.button("Open").show();
        ui.button("Save").show();
    });

// 2. Vertical Settings Column
ui.container()
    .column()
    .gap(12.0)
    .show(|ui| {
        ui.text("General Settings").font_weight(FontWeight::Bold).show();
        ui.checkbox(&mut auto_save, "Auto-save changes").show();
    });

// 3. Layered Badge on an Avatar (Stack)
ui.container()
    .stack()
    .width(48.0)
    .height(48.0)
    .show(|ui| {
        // Base layer: Profile Avatar
        ui.image("avatar.png").size(48.0, 48.0).radius_all(24.0).show();
        // Overlay layer: Online Status Dot (shifted to corner)
        ui.container()
            .width(12.0)
            .height(12.0)
            .radius_all(6.0)
            .bg(Color::rgb(0.2, 0.85, 0.4))
            .show(|_| {});
    });
```

---

### 2.8 Alignment & Justification

Alignment controls where children sit along the main and cross axes.

#### Method Signatures

```rust
/// Sets horizontal alignment: Left, Center, Right, SpaceBetween.
pub fn halign(mut self, align: Align) -> Self

/// Sets vertical alignment: Top, Center, Bottom.
pub fn valign(mut self, align: Align) -> Self

/// Convenience method to assign the identical alignment to both axes.
pub fn align(mut self, align: Align) -> Self
```

#### Alignment Behaviors in Rows vs Columns

| Alignment | In `.row()` | In `.column()` |
| :--- | :--- | :--- |
| `Align::Left` | Children start on left edge | Children align to left edge |
| `Align::Center` | Children center horizontally | Children center vertically or horizontally |
| `Align::Right` | Children push to right edge | Children align to right edge |
| `Align::SpaceBetween` | Pushes first child left, last child right | Spreads children vertically between top & bottom |

#### Code Examples

```rust
// 1. Header with Title on Left and Actions on Right
ui.container()
    .fill_x()
    .row()
    .halign(Align::SpaceBetween)
    .valign(Align::Center)
    .show(|ui| {
        ui.text("Project Settings").size(16.0).font_weight(FontWeight::Bold).show();
        ui.button("Close").show();
    });

// 2. Centered Empty State Screen
ui.container()
    .fill()
    .column()
    .align(Align::Center)
    .gap(8.0)
    .show(|ui| {
        ui.text("No Items Found").size(18.0).font_weight(FontWeight::Bold).show();
        ui.text("Create a new document to get started.").color(Color::GRAY).show();
    });
```

---

### 2.9 Scrolling, Viewports & Clipping

Containers can act as scrollable viewports that clip overflowing content and handle mouse wheel inertia.

#### Method Signatures

```rust
/// Enables vertical scrolling with mouse wheel and drag gestures.
pub fn scroll_y(mut self, enable: bool) -> Self

/// Enables horizontal scrolling with mouse wheel (Shift+Wheel or touch).
pub fn scroll_x(mut self, enable: bool) -> Self

/// Whether to visually render the vertical scrollbar track & thumb (Default: true).
pub fn show_scrollbar_y(mut self, show: bool) -> Self

/// Whether to visually render the horizontal scrollbar track & thumb (Default: true).
pub fn show_scrollbar_x(mut self, show: bool) -> Self

/// Clips child draw commands to the container's physical screen rect.
pub fn clip(mut self, clip: bool) -> Self

/// Routes standard vertical mouse wheel to horizontal scrolling if only scroll_x is active.
pub fn wheel_fallback_x(mut self, enabled: bool) -> Self
```

#### Code Examples

```rust
// 1. Scrollable Asset List
ui.container()
    .fill_x()
    .height(300.0)
    .scroll_y(true)
    .padding_right(8.0)
    .show(|ui| {
        for i in 0..100 {
            ui.text(&format!("Asset Item #{}", i)).show();
        }
    });

// 2. Horizontal Timeline with Hidden Scrollbar (Clean Blender/DAW Style)
ui.container()
    .fill_x()
    .height(48.0)
    .scroll_x(true)
    .show_scrollbar_x(false)
    .wheel_fallback_x(true)
    .row()
    .gap(16.0)
    .show(|ui| {
        for frame in 0..500 {
            ui.text(&format!("F:{}", frame)).show();
        }
    });
```

---

### 2.10 Pointer Interaction & Window Dragging

Containers can capture mouse clicks, change the operating system cursor icon, or allow window dragging.

#### Method Signatures

```rust
/// Sets the mouse cursor icon displayed when hovering over this container.
pub fn cursor(mut self, cursor: CursorIcon) -> Self

/// Enables hit-testing on the container's background geometry.
pub fn interactive(mut self, interactive: bool) -> Self

/// Makes this container act as an OS window dragging grip (e.g. custom title bars).
pub fn draggable_window(mut self, drag: bool) -> Self

/// Explicitly assigns a unique Id to this container for state persistence and animations.
pub fn id(mut self, id: impl std::hash::Hash) -> Self
```

#### Code Examples

```rust
// 1. Custom Draggable Title Bar
ui.container()
    .fill_x()
    .height(32.0)
    .draggable_window(true)
    .bg(Color::rgb(0.08, 0.09, 0.12))
    .row()
    .valign(Align::Center)
    .show(|ui| {
        ui.text("My Application").size(12.0).show();
    });

// 2. Clickable Card Tile
let response = ui.container()
    .width(180.0)
    .height(120.0)
    .interactive(true)
    .cursor(CursorIcon::Pointer)
    .bg(Color::rgb(0.14, 0.16, 0.22))
    .radius_all(8.0)
    .show(|ui| {
        ui.text("Click to Select").show();
    });

if response.clicked {
    println!("Card clicked!");
}
```

---

## 3. Unified Styling Rules Across All Widgets

### 3.1 The "No-Magic" Styling Contract & Universal `BoxStyle` Single Source of Truth

Every widget in Zenthra follows the **exact same naming convention** for borders, backgrounds, corners, padding, dimensions, and shadows.

Styling across Zenthra is backed by **`BoxStyle`** (`zenthra_core::BoxStyle`), providing a single source of truth with zero duplication and zero magic parameters. Any widget implementing the **`Styleable`** trait (`zenthra_core::Styleable`) immediately inherits the standardized builder methods.

```rust
// The Universal BoxStyle Contract:
.bg(...)            // Color, Gradient, or None (alpha via Color::rgba)
.border(...)        // (Color, width), (Color, width, alignment), or None
.radius_all(...)    // Unified corner radius in pixels
.radius(...)        // Explicit [top_left, top_right, bottom_right, bottom_left]
.radius_top(...)    // [r, r, 0, 0]
.radius_bottom(...) // [0, 0, r, r]
.shadow(...)        // Drop shadow (Color, ox, oy, blur) with rgba transparency
.padding(...)       // Inner spacing (top, right, bottom, left)
.padding_all(...)   // Uniform padding
.padding_x(...)     // Left & right padding
.padding_y(...)     // Top & bottom padding
.width(...)         // Fixed width
.height(...)        // Fixed height
.fill_x()           // Fill parent width
.fill_y()           // Fill parent height
.style(box_style)   // Directly assign or share a reusable BoxStyle
```

#### Reusable `BoxStyle` Example:
```rust
let card_theme = BoxStyle::new()
    .bg(Color::rgb(0.12, 0.14, 0.18))
    .border((Color::rgb(0.25, 0.30, 0.40), 1.0))
    .radius_all(8.0)
    .shadow(Color::rgba(0.0, 0.0, 0.0, 0.3), 0.0, 4.0, 10.0)
    .padding_all(16.0);

// Applied directly to any container:
ui.container()
    .style(card_theme)
    .show(|ui| {
        ui.text("Shared theme container").show();
    });
```

---

### 3.2 Input Widget Styling

The `Input` widget adheres to this exact contract, adding a `.focus_border(...)` for typing states.

```rust
// Standard Boxed Input
ui.input(&mut search_query)
    .width(220.0)
    .bg(Color::rgb(0.12, 0.13, 0.18))
    .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
    .focus_border((Color::rgb(0.35, 0.58, 0.98), 1.5))
    .radius_all(6.0)
    .placeholder("Search assets...")
    .show();

// Seamless Inline Input (Pure Text, No Box)
ui.input(&mut file_name)
    .bg(None)       // No background box
    .border(None)   // No idle border
    .focus_border((Color::rgb(0.35, 0.58, 0.98), 1.0)) // Shows line only when typing
    .show();
```

---

### 3.3 DragValue / NumberBox Styling

The `DragValue` numeric scrub widget follows the exact same contract.

```rust
// Standard Boxed Number Field (Blender / Unity Inspector Style)
ui.drag_value(&mut scale, "box_scale")
    .width(110.0)
    .bg(Color::rgb(0.12, 0.13, 0.18))
    .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
    .focus_border((Color::rgb(0.35, 0.58, 0.98), 1.5))
    .radius_all(4.0)
    .suffix(" %")
    .precision(1)
    .show();

// Text-Only Scrubbable Value (After Effects / Premiere Pro Timeline Style)
ui.drag_value(&mut scale, "txt_scale")
    .bg(None)       // No background box
    .border(None)   // No idle border
    .color(Color::rgb(0.40, 0.75, 1.0)) // Colored accent text
    .suffix(" %")
    .show();
```

---

### 3.4 Button Widget Styling

Buttons add state-driven hover and active properties.

```rust
// Standard Raised Button
ui.button("Export")
    .bg(Color::rgb(0.20, 0.40, 0.85))
    .border((Color::rgb(0.30, 0.50, 0.95), 1.0))
    .radius_all(6.0)
    .show();

// Ghost / Text-Only Button (Link Style)
ui.button("Cancel")
    .bg(None)       // No background box
    .border(None)   // No border
    .color(Color::rgb(0.70, 0.75, 0.85))
    .show();
```

---

### 3.5 Text & TextArea Styling

Static text and multiline text areas use the identical parameters when decorated with borders or backgrounds.

```rust
// Tag / Badge Text
ui.text("PRO")
    .size(10.0)
    .bg(Color::rgb(0.95, 0.65, 0.20))
    .border((Color::rgb(1.0, 0.8, 0.4), 1.0))
    .radius_all(3.0)
    .padding(2.0, 6.0, 2.0, 6.0)
    .show();

// Code Editor TextArea
ui.text_area(&mut code_buffer, "editor")
    .bg(Color::rgb(0.08, 0.09, 0.12))
    .border((Color::rgb(0.20, 0.23, 0.30), 1.0))
    .focus_border((Color::rgb(0.35, 0.58, 0.98), 1.5))
    .radius_all(8.0)
    .show();
```

---

### 3.6 Cross-Widget Style Reference Table

The entire Zenthra widget ecosystem follows this unified styling API contract:

| Method Call | `Container` | `Button` | `Input` | `TextArea` | `DragValue` | `Card` | `Panel` | `Image` | `Slider` | `Checkbox` | `Dropdown` | `Window` |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `.bg(Color)` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| `.bg(None)` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| `.hover_bg(...)` | ✅ | ✅ | — | — | — | — | — | — | — | ✅ | ✅ | — |
| `.active_bg(...)` | ✅ | ✅ | — | — | — | — | — | — | — | ✅ | — | — |
| `.border((Color, w))` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| `.border(None)` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| `.focus_border((Color, w))` | — | — | ✅ | ✅ | ✅ | — | — | — | — | — | — | — |
| `.hover_border((Color, w))` | ✅ | ✅ | — | — | — | ✅ | — | ✅ | — | — | — | — |
| `.active_border((Color, w))` | ✅ | ✅ | — | — | — | ✅ | — | — | — | — | — | — |
| `.radius_all(f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| `.shadow(...)` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | — |

---

## 4. Production Recipes & Patterns

### Recipe 1: Sleek Inspector Panel

A panel with a distinct dark surface, subtle border, and rounded corners:

```rust
ui.container()
    .width(320.0)
    .bg(Color::rgb(0.12, 0.13, 0.18))
    .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
    .radius_all(8.0)
    .padding(14.0, 16.0, 14.0, 16.0)
    .gap(10.0)
    .show(|ui| {
        ui.text("TRANSFORM")
            .size(11.0)
            .font_weight(FontWeight::Bold)
            .color(Color::rgb(0.45, 0.65, 1.0))
            .show();

        // Position Row
        ui.container().row().gap(8.0).valign(Align::Center).show(|ui| {
            ui.text("Position:").width(70.0).show();
            ui.drag_value(&mut pos_x, "px").prefix("X: ").width(100.0).show();
            ui.drag_value(&mut pos_y, "py").prefix("Y: ").width(100.0).show();
        });
    });
```

---

### Recipe 2: Compact Property Row

A clean, non-boxing row where the label is aligned with a text-only scrubbable control:

```rust
ui.container()
    .row()
    .gap(10.0)
    .valign(Align::Center)
    .show(|ui| {
        ui.text("Scale:").size(13.0).color(Color::rgb(0.85, 0.88, 0.95)).show();
        ui.drag_value(&mut scale, "scale")
            .bg(None)
            .border(None)
            .suffix(" %")
            .precision(1)
            .width(80.0)
            .color(Color::rgb(0.95, 0.80, 0.40))
            .show();
    });
```

---

### Recipe 3: Floating Modal Card with Depth

A dialog elevated above the scene using a multi-stop shadow and prominent border:

```rust
ui.container()
    .width(440.0)
    .bg(Color::rgb(0.14, 0.16, 0.23))
    .border((Color::rgb(0.28, 0.32, 0.46), 1.5))
    .radius_all(12.0)
    .shadow(Color::rgba(0.0, 0.0, 0.0, 0.65), 0.0, 12.0, 32.0)
    .padding_all(24.0)
    .gap(16.0)
    .show(|ui| {
        ui.text("Save Changes?").size(18.0).font_weight(FontWeight::Bold).show();
        ui.text("You have unsaved changes in this document.").color(Color::GRAY).show();

        ui.container().row().halign(Align::Right).gap(10.0).show(|ui| {
            ui.button("Cancel").bg(None).border(None).show();
            ui.button("Save").bg(Color::rgb(0.35, 0.58, 0.98)).show();
        });
    });
```

---

### Recipe 4: Scrollable Virtualized Asset List

An asset viewport with a clean scroll track:

```rust
ui.container()
    .fill_x()
    .height(380.0)
    .scroll_y(true)
    .clip(true)
    .bg(Color::rgb(0.09, 0.10, 0.14))
    .border((Color::rgb(0.18, 0.20, 0.28), 1.0))
    .radius_all(6.0)
    .padding(8.0, 10.0, 8.0, 10.0)
    .gap(4.0)
    .show(|ui| {
        for index in 0..150 {
            ui.container()
                .fill_x()
                .padding_y(6.0)
                .padding_x(8.0)
                .radius_all(4.0)
                .interactive(true)
                .cursor(CursorIcon::Pointer)
                .show(|ui| {
                    ui.text(&format!("Texture_Layer_{:03}.png", index)).show();
                });
        }
    });
```

---

### Recipe 5: Draggable Title Bar & Window Header

A window chrome header bar supporting OS window dragging:

```rust
ui.container()
    .fill_x()
    .height(36.0)
    .draggable_window(true)
    .bg(Color::rgb(0.08, 0.09, 0.12))
    .border((Color::rgb(0.18, 0.20, 0.28), 1.0))
    .padding_x(14.0)
    .row()
    .halign(Align::SpaceBetween)
    .valign(Align::Center)
    .show(|ui| {
        ui.text("Zenthra Engine - Composition View").size(12.0).font_weight(FontWeight::Bold).show();

        ui.container().row().gap(8.0).show(|ui| {
            ui.button("—").width(28.0).bg(None).show();
            ui.button("✕").width(28.0).bg(None).show();
        });
    });
```

---

## 5. Anti-Patterns & Troubleshooting

### ❌ Anti-Pattern 1: Unnecessary Box Nesting
**Wrong:** Wrapping every single label or field in an extra `ui.container()` just to give it a layout width.
```rust
// ❌ Redundant container wrapper:
ui.container().width(70.0).show(|ui| {
    ui.text("Scale:").show();
});
```
**Correct:** Set the width directly on the widget itself:
```rust
// ✅ Clean, zero-cost layout sizing:
ui.text("Scale:").width(70.0).show();
```

---

### ❌ Anti-Pattern 2: Mixing `row()` Without Vertical Alignment
**Wrong:** Using `.row()` without specifying `.valign(Align::Center)`. By default, text labels and inputs will sit against the top edge, looking vertically misaligned.
```rust
// ❌ Labels will sit 4px higher than inputs:
ui.container().row().gap(8.0).show(|ui| { ... });
```
**Correct:** Always specify `.valign(Align::Center)` for form rows:
```rust
// ✅ Perfectly centered baseline:
ui.container().row().gap(8.0).valign(Align::Center).show(|ui| { ... });
```

---

### ❌ Anti-Pattern 3: Using Negative Style Flags
**Wrong:** Mixing `.borderless()`, `.no_bg()`, and `.no_border()`.
```rust
// ❌ Cluttered and inconsistent:
ui.input(&mut val).no_bg().no_border().show();
```
**Correct:** Use standard `None` parameters:
```rust
// ✅ Consistent across all widgets:
ui.input(&mut val).bg(None).border(None).show();
```

---

## 6. Type Definitions & Enums

### Background

```rust
pub enum Background {
    None,
    Solid(Color),
    Gradient(Gradient),
}

impl From<Color> for Background {
    fn from(c: Color) -> Self {
        Background::Solid(c)
    }
}

impl<T: Into<Background>> From<Option<T>> for Background {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => val.into(),
            None => Background::None,
        }
    }
}
```

### BorderAlignment

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderAlignment {
    #[default]
    Inside,
    Center,
    Outside,
}
```

### Direction & Align

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Column,
    Row,
    Stack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
    Top,
    Bottom,
    SpaceBetween,
}
```
