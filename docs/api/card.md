# Card API Specification & Unified Styling Architecture

`Card` is Zenthra's primary elevated surface container widget for grouping related content, metrics, form sections, and interactive selections. It features unified box styling via `impl_box_style_methods!`, layout direction and alignment controls, smooth hover/active animations, backdrop filter support, and full immediate-mode interaction response handling.

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Card API Reference](#2-complete-card-api-reference)
   - [2.1 Construction & Deterministic Identification](#21-construction--deterministic-identification)
   - [2.2 Layout Direction & Alignment (.row, .column, .halign, .valign, .gap)](#22-layout-direction--alignment-row-column-halign-valign-gap)
   - [2.3 Interactive States & Sibling Architecture (.hover, .active)](#23-interactive-states--sibling-architecture-hover-active)
   - [2.4 Box Dimensions & Sizing (Unified BoxStyle)](#24-box-dimensions--sizing-unified-boxstyle)
   - [2.5 Padding & Margin (Unified BoxStyle)](#25-padding--margin-unified-boxstyle)
   - [2.6 Backgrounds, Borders, Radii, Shadows & Backdrop Filter](#26-backgrounds-borders-radii-shadows--backdrop-filter)
   - [2.7 Execution & Event Response (.show -> Response)](#27-execution--event-response-show---response)
   - [2.8 Associated Helpers (ui.card_header)](#28-associated-helpers-uicard_header)
3. [Production Recipes & Patterns](#3-production-recipes--patterns)
   - [Recipe 1: Basic Metric / Telemetry Card](#recipe-1-basic-metric--telemetry-card)
   - [Recipe 2: Clickable Interactive Selection Card](#recipe-2-clickable-interactive-selection-card)
   - [Recipe 3: Card with Custom Header & Action Controls](#recipe-3-card-with-custom-header--action-controls)
   - [Recipe 4: Premium Glassmorphic Card with Backdrop Filter](#recipe-4-premium-glassmorphic-card-with-backdrop-filter)
   - [Recipe 5: Responsive Flex Card Row](#recipe-5-responsive-flex-card-row)
4. [Anti-Patterns & Troubleshooting](#4-anti-patterns--troubleshooting)
5. [Architecture & BoxStyle Conformance](#5-architecture--boxstyle-conformance)

---

## 1. Design Philosophy & Mental Model

### The Core Mental Model

In Zenthra, a `Card`:
- **Elevated Interactive Surface**: Designed to hold heterogeneous widgets (text, buttons, inputs, images) with consistent padding, rounded corners, and subtle borders.
- **Single-Source Styling Conformance**: Implements `Styleable` and generates canonical box-styling methods via `impl_box_style_methods!(CardBuilder, 'u, 'a)`. It eliminates legacy ad-hoc aliases (`padding(f32)`, `radius(f32)`, `size(w, h)`).
- **Interactive State Transitions**: Supports unified Sibling states `.hover([Style::*, Transform::*, Cursor::*, Tooltip::*, Animation::*])` and `.active([ ... ])` with smooth micro-motion and zero heap allocations.
- **Full Response Propagation**: Unlike older card implementations that returned `()`, modern `CardBuilder::show()` returns a `Response` enabling immediate detection of `.clicked`, `.hovered`, and `.pressed`.

```rust
// ✅ Clean, canonical Zenthra Card:
ui.card()
    .fill_x()
    .padding_all(18.0)
    .radius_all(12.0)
    .column()
    .gap(12.0)
    .show(|ui: &mut Ui| {
        ui.text("Cluster Health: 100%").color(Color::rgb(0.3, 0.85, 0.5)).show();
    });
```

---

## 2. Complete Card API Reference

### 2.1 Construction & Deterministic Identification

```rust
// On Ui:
pub fn card(&mut self) -> CardBuilder<'_, 'a>

// On CardBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
```

- `ui.card()`: Instantiates a `CardBuilder` with dark slate defaults (`bg`: `rgb(0.09, 0.11, 0.15)`, `border`: `1.0px rgb(0.18, 0.22, 0.30)`, `radius`: `10.0px`, `padding`: `16.0px`).
- `.id(impl Hash)`: Sets a deterministic ID (required when tracking interactive state across dynamic lists or loops).

---

### 2.2 Layout Direction & Alignment (.row, .column, .halign, .valign, .gap)

Configures how children inside the card flow and align.

```rust
pub fn row(mut self) -> Self                    // Layout children horizontally
pub fn column(mut self) -> Self                 // Layout children vertically (default)
pub fn halign(mut self, align: Align) -> Self   // Horizontal alignment (Left, Center, Right, SpaceBetween)
pub fn valign(mut self, align: Align) -> Self   // Vertical alignment (Top, Center, Bottom)
pub fn align(mut self, align: Align) -> Self    // Sets both halign & valign simultaneously
pub fn gap(mut self, gap: f32) -> Self          // Spacing between child elements (in points)
```

---

### 2.3 Interactive States & Sibling Architecture (.hover, .active)

Built-in micro-interactions and transitions for elevated, clickable cards using Zenthra's Sibling architecture:

```rust
pub fn hover(mut self, props: impl IntoStateProps) -> Self
pub fn active(mut self, props: impl IntoStateProps) -> Self
pub fn backdrop_blur(mut self, radius: f32) -> Self                    // Shorthand for real-time background blur (e.g. 20.0)
pub fn backdrop_filter(mut self, filter: BackdropFilter) -> Self       // Advanced GPU backdrop filter (blur, saturate, etc.)
```

#### Sibling Interaction Example:
```rust
ui.card()
    .bg(Color::rgb(0.10, 0.12, 0.16))
    .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
    .hover([
        Style::bg(Color::rgb(0.14, 0.18, 0.26)),
        Style::border((Color::rgb(0.35, 0.65, 1.0), 1.5)),
        Transform::scale(1.03),
        Cursor::Pointer,
        Tooltip::new("Click to inspect cluster metrics"),
        Animation::duration(150).ease_out(),
    ])
    .active([
        Style::bg(Color::rgb(0.08, 0.10, 0.14)),
        Transform::scale(0.98),
    ])
    .show(|ui| { ... });
```

---

### 2.4 Box Dimensions & Sizing (Unified BoxStyle)

All standard dimension methods generated via `impl_dimensions!`:

```rust
pub fn width(mut self, width: f32) -> Self
pub fn height(mut self, height: f32) -> Self
pub fn fill_x(mut self) -> Self
pub fn fill_y(mut self) -> Self
pub fn fill(mut self) -> Self
pub fn min_width(mut self, min: f32) -> Self
pub fn max_width(mut self, max: f32) -> Self
pub fn min_height(mut self, min: f32) -> Self
pub fn max_height(mut self, max: f32) -> Self
```

---

### 2.5 Padding & Margin (Unified BoxStyle)

Methods generated via `impl_padding!` and `impl_margin!`:

```rust
// Internal Padding
pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self
pub fn padding_all(mut self, p: f32) -> Self
pub fn padding_x(mut self, p: f32) -> Self
pub fn padding_y(mut self, p: f32) -> Self
pub fn padding_top(mut self, p: f32) -> Self
pub fn padding_bottom(mut self, p: f32) -> Self
pub fn padding_left(mut self, p: f32) -> Self
pub fn padding_right(mut self, p: f32) -> Self

// Outer Margin
pub fn margin(mut self, t: f32, r: f32, b: f32, l: f32) -> Self
pub fn margin_all(mut self, m: f32) -> Self
pub fn margin_x(mut self, m: f32) -> Self
pub fn margin_y(mut self, m: f32) -> Self
pub fn margin_top(mut self, m: f32) -> Self
pub fn margin_bottom(mut self, m: f32) -> Self
pub fn margin_left(mut self, m: f32) -> Self
pub fn margin_right(mut self, m: f32) -> Self
```

---

### 2.6 Backgrounds, Borders, Radii, Shadows & Backdrop Filter

Methods generated via `impl_radius!` and `impl_visuals!`:

```rust
// Corner Radii
pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self
pub fn radius_all(mut self, r: f32) -> Self
pub fn radius_top(mut self, r: f32) -> Self
pub fn radius_bottom(mut self, r: f32) -> Self
pub fn radius_left(mut self, r: f32) -> Self
pub fn radius_right(mut self, r: f32) -> Self

// Visuals
pub fn bg(mut self, bg: impl Into<Background>) -> Self
pub fn border(mut self, border: impl IntoBorder) -> Self
pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self
pub fn style(mut self, style: BoxStyle) -> Self
```

---

### 2.7 Execution & Event Response (.show -> Response)

```rust
pub fn show<F>(self, f: F) -> Response
where
    F: FnOnce(&mut Ui)
```

#### Returns:
`Response` indicating whether the card was:
- `response.clicked`: `true` on the frame the user clicked within card boundaries.
- `response.hovered`: `true` while the cursor is over the card.
- `response.pressed`: `true` while mouse button is held down over the card.

---

### 2.8 Associated Helpers (ui.card_header)

Zenthra provides a convenience helper `ui.card_header` for standardized card titles with an action slot:

```rust
ui.card_header("Card Title", "Card Subtitle", |ui: &mut Ui| {
    ui.button("Options").show();
});
```

---

## 3. Production Recipes & Patterns

### Recipe 1: Basic Metric / Telemetry Card

```rust
ui.card()
    .fill_x()
    .padding_all(20.0)
    .column()
    .gap(12.0)
    .show(|ui: &mut Ui| {
        ui.text("Database Throughput")
            .size(13.0)
            .color(Color::rgb(0.6, 0.65, 0.75))
            .show();

        ui.text("48,250 ops/sec")
            .size(24.0)
            .font_weight(FontWeight::Bold)
            .color(Color::WHITE)
            .show();

        ui.text("▲ 12.4% vs last hour")
            .size(12.0)
            .color(Color::rgb(0.3, 0.85, 0.5))
            .show();
    });
```

---

### Recipe 2: Clickable Interactive Selection Card

```rust
let mut selected_plan = 1;

let resp = ui.card()
    .id("plan_pro")
    .width(260.0)
    .height(160.0)
    .padding_all(18.0)
    .radius_all(12.0)
    .bg(if selected_plan == 1 { Color::rgb(0.12, 0.18, 0.30) } else { Color::rgb(0.09, 0.11, 0.15) })
    .border(if selected_plan == 1 {
        (Color::rgb(0.35, 0.65, 1.0), 1.5)
    } else {
        (Color::rgb(0.18, 0.22, 0.30), 1.0)
    })
    .hover([
        Style::bg(Color::rgb(0.14, 0.20, 0.34)),
        Transform::scale(1.03),
        Cursor::Pointer,
    ])
    .column()
    .gap(8.0)
    .show(|ui: &mut Ui| {
        ui.text("Pro Cluster").size(16.0).font_weight(FontWeight::Bold).color(Color::WHITE).show();
        ui.text("$99/mo").size(20.0).color(Color::rgb(0.35, 0.65, 1.0)).show();
        ui.text("Dedicated 16-core compute nodes").size(11.0).color(Color::rgb(0.65, 0.7, 0.8)).show();
    });

if resp.clicked {
    selected_plan = 1;
}
```

---

### Recipe 3: Card with Custom Header & Action Controls

```rust
ui.card()
    .fill_x()
    .padding_all(16.0)
    .column()
    .gap(12.0)
    .show(|ui: &mut Ui| {
        ui.card_header("Active Deployments", "Cluster us-east-2", |ui: &mut Ui| {
            if ui.button("Deploy New").show().clicked {
                // Trigger deploy flow
            }
        });

        ui.text("3 microservices healthy, 0 alerts active.").show();
    });
```

---

### Recipe 4: Premium Glassmorphic Card with Backdrop Filter

```rust
ui.card()
    .fill_x()
    .bg(Color::rgba(0.08, 0.10, 0.16, 0.70))
    .backdrop_filter(BackdropFilter::blur(20.0))
    .border((Color::rgba(1.0, 1.0, 1.0, 0.12), 1.0))
    .radius_all(16.0)
    .shadow(Color::rgba(0.0, 0.0, 0.0, 0.40), 0.0, 12.0, 32.0)
    .padding_all(24.0)
    .column()
    .gap(10.0)
    .show(|ui: &mut Ui| {
        ui.text("Glassmorphic Surface").size(18.0).font_weight(FontWeight::Bold).color(Color::WHITE).show();
        ui.text("Rendered with real-time GPU background blur and subtle borders.").show();
    });
```

---

### Recipe 5: Responsive Flex Card Row

```rust
ui.container()
    .row()
    .gap(16.0)
    .fill_x()
    .show(|ui: &mut Ui| {
        for i in 1..=3 {
            ui.card()
                .fill_x()
                .padding_all(16.0)
                .column()
                .gap(6.0)
                .show(|ui: &mut Ui| {
                    ui.text(&format!("Worker Node #{}", i)).size(14.0).font_weight(FontWeight::Bold).show();
                    ui.text("Memory: 16 GB").size(12.0).color(Color::rgb(0.6, 0.65, 0.75)).show();
                });
        }
    });
```

---

## 4. Anti-Patterns & Troubleshooting

| Anti-Pattern | Why it Fails | Recommended Fix |
|---|---|---|
| **Calling `.padding(f32)` with 1 argument** | Single-argument `padding(f32)` was an ad-hoc alias that contradicted standard `padding(t, r, b, l)`. | Use `.padding_all(16.0)` or `.padding(16.0, 16.0, 16.0, 16.0)`. |
| **Calling `.size(w, h)`** | Non-standard method conflicting with `TextStyle::size(f32)`. | Call `.width(w).height(h)` instead. |
| **Passing `.shadow((color, x, y, blur))` tuple** | `BoxStyle` method takes 4 parameters: `color, x, y, blur`. | Call `.shadow(color, x, y, blur)`. |
| **Ignoring `.clicked` Response** | Old `Card::show` returned `()`, but modern `Card::show` returns `Response`. | Check `if ui.card().show(...).clicked` for clickable cards. |

---

## 5. Architecture & BoxStyle Conformance

`CardBuilder` delegates internal rendering to `ContainerBuilder`, applying `self.style` as a unified whole:

```
┌────────────────────────────────────────────────────────┐
│                      CardBuilder                       │
│  - pub style: BoxStyle (impl_box_style_methods!)       │
│  - Layout: row(), column(), halign(), valign(), gap()  │
│  - Interactions: hover([ ... ]), active([ ... ])       │
└───────────────────────────┬────────────────────────────┘
                            │ Delegates directly via
                            │ container.style(self.style)
┌───────────────────────────▼────────────────────────────┐
│                   ContainerBuilder                     │
│  (Hardware clipping, background quad, border, shadow)  │
└────────────────────────────────────────────────────────┘
```
