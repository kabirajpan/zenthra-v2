# ProgressBar API Specification & Unified Styling Architecture

`ProgressBar` is Zenthra's primary quantitative progress indicator. It displays deterministic progress values from `0.0` to `1.0`, supporting solid and gradient tracks/fills, animated shimmer light highlights, custom drop shadows, and full outer container styling (surfaces, borders, corner radii, and interaction states) via `BoxStyle`.

The `ProgressBar` widget strictly adheres to the unified styling contract established in [`docs/api/container.md`](./container.md) and [`docs/api/roadmap.md`](./roadmap.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete ProgressBar API Reference](#2-complete-progressbar-api-reference)
   - [2.1 Construction & Value](#21-construction--value)
   - [2.2 Dimensions, Width & Height](#22-dimensions-width--height)
   - [2.3 Container Styling (`BoxStyle` Parity)](#23-container-styling-boxstyle-parity)
   - [2.4 Track Styling & Customization](#24-track-styling--customization)
   - [2.5 Fill Styling & Gradients](#25-fill-styling--gradients)
   - [2.6 Animated Shimmer Effect](#26-animated-shimmer-effect)
   - [2.7 Interaction States & Hover/Active Overrides](#27-interaction-states--hoveractive-overrides)
3. [Production Recipes & Patterns](#3-production-recipes--patterns)
   - [Recipe 1: Minimalist 4px Accent Bar](#recipe-1-minimalist-4px-accent-bar)
   - [Recipe 2: Cyberpunk Neon Gradient Fill with Glow Shadow](#recipe-2-cyberpunk-neon-gradient-fill-with-glow-shadow)
   - [Recipe 3: Animated Shimmering File Downloader](#recipe-3-animated-shimmering-file-downloader)
   - [Recipe 4: Contained Card Progress Indicator](#recipe-4-contained-card-progress-indicator)
4. [Type Definitions](#4-type-definitions)

---

## 1. Design Philosophy & Mental Model

### The Three Layers
In Zenthra, a `ProgressBar` consists of three layered visual primitives:
1. **Container Enclosure (Optional `BoxStyle`)**: The bounding box framing the entire progress bar. By default transparent (`.bg(None)`). When styled with `.bg(...)`, `.border(...)`, or `.shadow(...)`, it acts as a card container.
2. **Track**: The full horizontal bar representing 100% capacity.
3. **Fill**: The filled portion representing the current normalized `value` (clamped between `0.0` and `1.0`).

### Unified `BoxStyle` Parity
`ProgressBarBuilder` implements all unified styling macros from `macros.rs`:
- `.width(w)` / `.height(h)` / `.track_size(w, h)`
- `.margin_all(m)` / `.padding_all(p)`
- `.radius_all(r)` / `.radius(tl, tr, br, bl)`
- `.bg(color_or_gradient)` / `.border(border)`
- `.shadow(color, ox, oy, blur)`
- `.hover([ ... ])` / `.active([ ... ])`

```rust
// Standard clean Zenthra usage:
ui.progress_bar(progress)
    .track_height(8.0)
    .radius_all(4.0)
    .fill_color(Color::rgb(0.3, 0.7, 1.0))
    .track_color(Color::rgb(0.14, 0.16, 0.22))
    .show();
```

---

## 2. Complete ProgressBar API Reference

### 2.1 Construction & Value
```rust
ui.progress_bar(value: f32) -> ProgressBarBuilder
```
- `value`: Normalized progress between `0.0` and `1.0`. Automatically clamped.
- Builder also provides `.id(hashable)` to explicitly override the widget ID.

### 2.2 Dimensions, Width & Height
| Method | Description |
| :--- | :--- |
| `.width(w: f32)` | Explicit width in logical pixels. Defaults to `available_width.max(100.0)`. |
| `.height(h: f32)` | Total outer widget height. Defaults to `track_h.max(12.0) + padding.vertical()`. |
| `.track_height(h: f32)` | Height of the inner track and fill bar. Default is `8.0`. |
| `.track_size(w: f32, h: f32)` | Convenience setter for both width and track height. |

### 2.3 Container Styling (`BoxStyle` Parity)
| Method | Description |
| :--- | :--- |
| `.bg(impl Into<Background>)` | Outer container background (solid color or gradient). |
| `.border(impl Into<Border>)` | Outer border (e.g. `(Color, width)`). |
| `.radius_all(r: f32)` | Outer container corner radius. Automatically inherited by track/fill if not customized. |
| `.padding_all(p: f32)` | Padding surrounding the track. Default is `4.0`. |
| `.shadow(color, x, y, blur)` | Outer container drop shadow. |

### 2.4 Track Styling & Customization
| Method | Description |
| :--- | :--- |
| `.track_color(color: Color)` | Solid color for the track background. |
| `.track_gradient(start, end, dir)` | Gradient for the track background. |
| `.track_bg(impl Into<Background>)` | Full `Background` enum for track. |
| `.track_radius_all(r: f32)` | Explicit corner radius for track. |
| `.track_shadow(color, x, y, blur)` | Inner shadow for track depth. |

### 2.5 Fill Styling & Gradients
| Method | Description |
| :--- | :--- |
| `.fill_color(color: Color)` | Solid color for the progress fill bar. |
| `.fill_gradient(start, end, dir)` | Gradient for the progress fill bar. |
| `.fill_bg(impl Into<Background>)` | Full `Background` enum for fill. |
| `.fill_radius_all(r: f32)` | Explicit corner radius for fill. |
| `.fill_shadow(color, x, y, blur)` | Outer glow / shadow around the filled bar. |

### 2.6 Animated Shimmer Effect
| Method | Description |
| :--- | :--- |
| `.shimmer(enabled: bool)` | Enables a dynamic light highlight beam that sweeps across the filled portion of the bar at 60fps. |

### 2.7 Interaction States & Hover/Active Overrides
```rust
ui.progress_bar(progress)
    .hover([
        Style::bg(Color::rgb(0.12, 0.14, 0.18)),
        Style::border((Color::rgb(0.3, 0.6, 1.0), 1.0)),
    ])
    .active([
        Style::border((Color::rgb(0.5, 0.8, 1.0), 1.5)),
        Transform::scale(1.01),
    ])
    .show();
```

---

## 3. Production Recipes & Patterns

### Recipe 1: Minimalist 4px Accent Bar
```rust
ui.progress_bar(progress)
    .track_height(4.0)
    .radius_all(2.0)
    .fill_color(Color::rgb(0.35, 0.65, 1.0))
    .track_color(Color::rgb(0.14, 0.16, 0.22))
    .show();
```

### Recipe 2: Cyberpunk Neon Gradient Fill with Glow Shadow
```rust
ui.progress_bar(progress)
    .track_height(10.0)
    .radius_all(5.0)
    .fill_gradient(
        Color::rgb(0.55, 0.20, 0.95),
        Color::rgb(0.85, 0.35, 1.00),
        GradientDirection::Horizontal,
    )
    .track_color(Color::rgb(0.12, 0.14, 0.19))
    .fill_shadow(Color::rgba(0.70, 0.30, 1.0, 0.50), 0.0, 2.0, 10.0)
    .show();
```

### Recipe 3: Animated Shimmering File Downloader
```rust
ui.progress_bar(download_progress)
    .track_height(12.0)
    .radius_all(6.0)
    .fill_gradient(
        Color::rgb(0.10, 0.85, 0.80),
        Color::rgb(0.25, 0.50, 1.00),
        GradientDirection::Horizontal,
    )
    .track_color(Color::rgb(0.12, 0.15, 0.20))
    .fill_shadow(Color::rgba(0.20, 0.60, 1.0, 0.40), 0.0, 2.0, 8.0)
    .shimmer(true)
    .show();
```

### Recipe 4: Contained Card Progress Indicator
```rust
ui.progress_bar(task_progress)
    .width(360.0)
    .bg(Color::rgb(0.10, 0.12, 0.16))
    .border((Color::rgb(0.20, 0.24, 0.32), 1.0))
    .radius_all(10.0)
    .padding_all(14.0)
    .track_height(8.0)
    .fill_color(Color::rgb(0.20, 0.80, 0.50))
    .show();
```

---

## 4. Type Definitions

```rust
pub struct ProgressBarBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,
    pub value: f32,

    pub style: BoxStyle,
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,

    pub track_h: f32,
    pub track_radius: Option<[f32; 4]>,
    pub track_bg: Option<zenthra_core::Background>,
    pub track_color: Color,
    pub track_shadow_color: Option<Color>,
    pub track_shadow_offset: [f32; 2],
    pub track_shadow_blur: f32,
    pub track_gradient: Option<(Color, Color, zenthra_core::GradientDirection)>,

    pub fill_bg: Option<zenthra_core::Background>,
    pub fill_color: Color,
    pub fill_radius: Option<[f32; 4]>,
    pub fill_shadow_color: Option<Color>,
    pub fill_shadow_offset: [f32; 2],
    pub fill_shadow_blur: f32,
    pub fill_gradient: Option<(Color, Color, zenthra_core::GradientDirection)>,

    pub shimmer: bool,
}
```
