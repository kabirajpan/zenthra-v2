# Panel API Specification & Unified Styling Architecture

`Panel` is Zenthra's structured surface container widget. It provides an edge-to-edge flush header (with optional title, subtitle, and collapsible chevron toggle), subtle divider, and an indented content body. It supports single-source two-way boolean state binding (`.collapsed(&mut bool)`), collision-free header keys, and full conformance to the unified styling contract established in [`docs/api/macros.md`](macros.md).

---

## Table of Contents

1. [Design Philosophy & Mental Model](#1-design-philosophy--mental-model)
2. [Complete Panel API Reference](#2-complete-panel-api-reference)
   - [2.1 Construction & Deterministic Identification](#21-construction--deterministic-identification)
   - [2.2 Header Configuration (.title, .subtitle, .header_bg, .header_padding)](#22-header-configuration-title-subtitle-header_bg-header_padding)
   - [2.3 Collapsible State & Two-Way Binding](#23-collapsible-state--two-way-binding)
   - [2.4 Box Dimensions & Sizing (Unified BoxStyle)](#24-box-dimensions--sizing-unified-boxstyle)
   - [2.5 Body Padding & Margin (Unified BoxStyle)](#25-body-padding--margin-unified-boxstyle)
   - [2.6 Backgrounds, Borders, Radii & Shadows (Unified BoxStyle)](#26-backgrounds-borders-radii--shadows-unified-boxstyle)
   - [2.7 Execution & Response Handling](#27-execution--response-handling)
3. [Production Recipes & Patterns](#3-production-recipes--patterns)
   - [Recipe 1: Standard Metric / Information Panel](#recipe-1-standard-metric--information-panel)
   - [Recipe 2: Collapsible Settings Panel](#recipe-2-collapsible-settings-panel)
   - [Recipe 3: Programmatically Controlled Panel](#recipe-3-programmatically-controlled-panel)
   - [Recipe 4: Premium Dark Velvet Panel with Accent Borders](#recipe-4-premium-dark-velvet-panel-with-accent-borders)
   - [Recipe 5: Mutually Exclusive Accordion Group](#recipe-5-mutually-exclusive-accordion-group)
4. [Anti-Patterns & Troubleshooting](#4-anti-patterns--troubleshooting)
5. [Architecture & Key Collisions Specification](#5-architecture--key-collisions-specification)

---

## 1. Design Philosophy & Mental Model

### The Core Mental Model

In Zenthra, a `Panel`:
- **Flush Edge-to-Edge Header**: Unlike nesting a header inside a padded container, `Panel` removes outer container padding so the header spans cleanly from left edge to right edge, perfectly inheriting container corner radii.
- **Independent Body Padding**: Inner content passed to `.show(|ui| ...)` is wrapped in an inner container applying `self.style.padding`.
- **Single Canonical Collapse Method**: A single method `.collapsed(&mut bool)` controls collapsibility. It directly binds to caller-owned boolean state—identically to `ui.checkbox()`, `ui.toggle()`, and `ui.slider()`. If omitted, the panel behaves as a static structured card.
- **Immediate-Mode Body Elimination**: When collapsed, the body closure is completely skipped, eliminating layout passes, draw command emission, and child widget processing.
- **Single-Source Styling**: Conforms 100% to [`macros.md`](macros.md) via `impl_box_style_methods!(PanelBuilder, 'u, 'a, 'b)`. No magical or ad-hoc aliases.

```rust
// ✅ Clean, canonical Zenthra Panel:
let mut is_collapsed = false;

ui.panel()
    .id("system_panel")
    .title("System Telemetry")
    .subtitle("Live cluster metrics and engine health")
    .collapsed(&mut is_collapsed)
    .fill_x()
    .show(|ui: &mut Ui| {
        ui.text("All systems operational.").show();
    });
```

---

## 2. Complete Panel API Reference

### 2.1 Construction & Deterministic Identification

```rust
// On Ui:
pub fn panel<'b>(&mut self) -> PanelBuilder<'_, 'a, 'b>

// On PanelBuilder:
pub fn id(mut self, id: impl std::hash::Hash) -> Self
```

- `ui.panel()`: Instantiates a `PanelBuilder` with dark theme defaults (dark slate background, subtle border, `8.0` corner radius, `14.0` padding).
- `.id(impl Hash)`: Hashes a deterministic key into an `Id` for state persistence and interaction tracking.

---

### 2.2 Header Configuration (.title, .subtitle, .header_bg, .header_padding)

The header section is generated automatically when a title, subtitle, or collapsible chevron is configured.

```rust
pub fn title(mut self, title: impl Into<String>) -> Self
pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self
pub fn header_bg(mut self, bg: impl Into<Background>) -> Self
pub fn header_padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self
pub fn header_padding_all(mut self, p: f32) -> Self
pub fn header_padding_x(mut self, p: f32) -> Self
pub fn header_padding_y(mut self, p: f32) -> Self
```

#### Defaults:
- `header_padding`: `EdgeInsets::symmetric(10.0, 14.0)`
- `header_bg`: None (inherits outer panel background)
- Corner Radii: Top corners match outer panel radius. Bottom corners are `0.0` when expanded, and seamlessly match the container radius when collapsed.

---

### 2.3 Collapsible State & State Binding

```rust
pub fn collapsed(mut self, state: &'b mut bool) -> Self
```

- `.collapsed(&mut bool)`: Makes the panel collapsible and binds its open/close state directly to a caller-owned boolean variable.
  - When `*state == true`, the body is hidden and the chevron icon points right (`▶`).
  - When `*state == false`, the body is rendered and the chevron icon points down (`▼`).
  - Clicking anywhere on the header bar toggles `*state = !*state`, consumes the click (`ui.consume_click()`), and requests a redraw.
  - Allows external buttons, icons, or keyboard shortcuts anywhere in the application to programmatically toggle or query the panel's open/close status.

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

### 2.5 Body Padding & Margin (Unified BoxStyle)

Methods generated via `impl_padding!` and `impl_margin!`:

```rust
// Body Padding (internal spacing around body contents)
pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self
pub fn padding_all(mut self, p: f32) -> Self
pub fn padding_x(mut self, p: f32) -> Self
pub fn padding_y(mut self, p: f32) -> Self
pub fn padding_top(mut self, p: f32) -> Self
pub fn padding_bottom(mut self, p: f32) -> Self
pub fn padding_left(mut self, p: f32) -> Self
pub fn padding_right(mut self, p: f32) -> Self

// Outer Margin (spacing around the panel container)
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

### 2.6 Backgrounds, Borders, Radii & Shadows (Unified BoxStyle)

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

### 2.7 Execution & Response Handling

```rust
pub fn show<F>(self, f: F) -> Response
where
    F: FnOnce(&mut Ui)
```

#### Returns:
`Response` indicating whether the header or outer panel was:
- `response.clicked`: `true` on the frame the header was clicked.
- `response.hovered`: `true` if mouse is over the panel or header.
- `response.changed`: `true` on the exact frame the collapsed state toggled.

---

## 3. Production Recipes & Patterns

### Recipe 1: Standard Metric / Information Panel

```rust
ui.panel()
    .id("cluster_metrics")
    .title("Cluster Metrics")
    .subtitle("Real-time GPU nodes and throughput")
    .fill_x()
    .show(|ui: &mut Ui| {
        ui.container()
            .row()
            .gap(24.0)
            .show(|ui: &mut Ui| {
                ui.container().column().gap(4.0).show(|ui| {
                    ui.text("VRAM Usage").size(11.0).color(Color::rgb(0.5, 0.55, 0.65)).show();
                    ui.text("7.2 / 16 GB").size(14.0).color(Color::WHITE).show();
                });
                ui.container().column().gap(4.0).show(|ui| {
                    ui.text("Temperature").size(11.0).color(Color::rgb(0.5, 0.55, 0.65)).show();
                    ui.text("54°C").size(14.0).color(Color::rgb(0.3, 0.85, 0.5)).show();
                });
            });
    });
```

---

### Recipe 2: Collapsible Settings Panel

```rust
let mut preferences_collapsed = false;

ui.panel()
    .id("notifications_panel")
    .title("Notification Preferences")
    .subtitle("Configure email and desktop alerts")
    .collapsed(&mut preferences_collapsed)
    .fill_x()
    .show(|ui: &mut Ui| {
        ui.checkbox(&mut email_alerts, "Send Email Digest").show();
        ui.checkbox(&mut desktop_notifications, "Push Desktop Alerts").show();
    });
```

---

### Recipe 3: Programmatically Controlled Panel

```rust
let mut is_collapsed = false;

// Header button controlling panel externally
if ui.button("Toggle Logs").show().clicked {
    is_collapsed = !is_collapsed;
}

ui.panel()
    .id("logs_panel")
    .title("Runtime Logs")
    .subtitle("Diagnostic telemetry stream")
    .collapsed(&mut is_collapsed)
    .fill_x()
    .show(|ui: &mut Ui| {
        ui.text("Waiting for logs...").color(Color::rgb(0.6, 0.65, 0.75)).show();
    });
```

---

### Recipe 4: Premium Dark Velvet Panel with Accent Borders

```rust
let mut security_collapsed = false;

ui.panel()
    .id("premium_panel")
    .title("Enterprise Security")
    .subtitle("TLS termination and token signing")
    .bg(Color::rgb(0.08, 0.10, 0.16))
    .border((Color::rgb(0.25, 0.45, 0.85), 1.5))
    .radius_all(14.0)
    .shadow(Color::rgba(0.10, 0.25, 0.65, 0.25), 0.0, 8.0, 24.0)
    .header_bg(Color::rgb(0.12, 0.15, 0.24))
    .header_padding(14.0, 18.0, 14.0, 18.0)
    .padding_all(20.0)
    .collapsed(&mut security_collapsed)
    .fill_x()
    .show(|ui: &mut Ui| {
        ui.text("Cryptographic modules active.").show();
    });
```

---

### Recipe 5: Mutually Exclusive Accordion Group

```rust
let mut active_tab = 0; // 0 = General, 1 = Security, 2 = Billing

// Section 1: General
let mut gen_collapsed = active_tab != 0;
let resp1 = ui.panel()
    .id("acc_general")
    .title("General Settings")
    .collapsed(&mut gen_collapsed)
    .fill_x()
    .show(|ui| { ui.text("General preferences content.").show(); });
if resp1.clicked {
    active_tab = if active_tab == 0 { 99 } else { 0 };
}

// Section 2: Security
let mut sec_collapsed = active_tab != 1;
let resp2 = ui.panel()
    .id("acc_security")
    .title("Security & MFA")
    .collapsed(&mut sec_collapsed)
    .fill_x()
    .show(|ui| { ui.text("MFA setup and tokens.").show(); });
if resp2.clicked {
    active_tab = if active_tab == 1 { 99 } else { 1 };
}
```

---

## 4. Anti-Patterns & Troubleshooting

| Anti-Pattern | Why it Fails | Recommended Fix |
|---|---|---|
| **Calling `.padding(f32)` with 1 argument** | Single-argument `padding(f32)` was an ad-hoc alias that contradicted standard `padding(t, r, b, l)`. | Use `.padding_all(14.0)` or `.padding(14.0, 14.0, 14.0, 14.0)`. |
| **Omitting `.id(...)` in dynamic loops** | Unstable generated IDs cause container layouts to flicker or reset across frames. | Provide `.id("unique_key")` or `.id(item_index)`. |
| **Passing `.shadow((color, x, y, blur))` tuple** | `BoxStyle` method takes 4 parameters: `color, x, y, blur`. | Call `.shadow(color, x, y, blur)`. |
| **Adding header inside body closure** | Double headers with broken corner radius clipping. | Use `.title(...)` and `.subtitle(...)` directly on `PanelBuilder`. |

---

## 5. Architecture & Key Collisions Specification

State persistence in immediate-mode frameworks requires collision-free ID tagging. `Panel` allocates deterministic 64-bit sub-keys derived from the parent ID:

```
64-bit ID Structure:
┌──────────────────────────────────────────────┬───────────┐
│        Upper 56 bits (Hashed Component)      │ 8-bit Tag │
└──────────────────────────────────────────────┴───────────┘
```

1. **Header Container Key (`0x23`)**:
   ```rust
   Id::from_u64(((id.raw() & 0x00FF_FFFF_FFFF_FFFF) << 8) | 0x23)
   ```
   Guarantees header hit-testing, cursor feedback, and click event bubbling remain decoupled from the outer container. State for collapse is bound directly to the caller's `&mut bool` reference without requiring internal storage keys.
