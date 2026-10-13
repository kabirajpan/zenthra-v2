# Zenthra Macros Architecture & Method Reference (`macros.md`)

This document serves as the canonical specification for Zenthra's macro-driven styling and typography engine located in [`crates/zenthra-widgets/src/macros.rs`](file:///home/kabir/Desktop/projects/personal/framworks/zenthra-v2/crates/zenthra-widgets/src/macros.rs).

It details:
1. **The Architecture**: How macros guarantee 100% method parity across all widgets.
2. **The Macro Catalog**: All individual and composite macros.
3. **The Complete Generated Method List**: Signatures, parameters, and defaults.
4. **User Endpoints**: How application developers invoke these methods on widgets.
5. **Widget Author Guide**: How to implement these macros on new widgets.

---

## 1. Architecture & Design Philosophy

### The Single-Source Principle
In UI frameworks, widget styling methods (such as `.width()`, `.padding()`, `.radius()`, `.bg()`, `.border()`, `.size()`) frequently duplicate code across builders. Without centralization, widgets develop subtle inconsistencies—missing lifetimes, mismatched method names, diverging default values, or accidental aliases.

Zenthra solves this via declarative, single-source code-generation macros in `macros.rs`:
- Every widget that implements `zenthra_core::Styleable` and holds a `pub style: BoxStyle` field gains the **exact same** box-styling methods via `impl_box_style_methods!`.
- Every widget that displays text and holds a `pub text_style: TextStyle` field gains the **exact same** typography methods via `impl_font_style_methods!` or `impl_text_style_methods!`.

```
                    ┌─────────────────────────┐
                    │    docs/api/container.md │ (Canonical Blueprint)
                    └────────────┬────────────┘
                                 │
                     ┌───────────▼───────────┐
                     │       macros.rs       │
                     └─────┬───────────┬─────┘
                           │           │
       ┌───────────────────▼───┐   ┌───▼───────────────────┐
       │ impl_box_style_methods!│   │impl_text_style_methods!│
       └───────────┬───────────┘   └───┬───────────────────┘
                   │                   │
         ┌─────────┼───────────┐       ├───────────┬───────────┐
         │         │           │       │           │           │
         ▼         ▼           ▼       ▼           ▼           ▼
    Container   Button       Input    Text      Button     TextArea ...
```

---

## 2. Macro Hierarchy & Catalog

`macros.rs` exposes fine-grained modular macros and high-level composite macros:

### Box Styling Macros
- **`impl_dimensions!`**: Layout box sizing (`width`, `height`, `fill_x`, `fill_y`, `fill`, `min_width`, `max_width`, `min_height`, `max_height`).
- **`impl_padding!`**: Inner box padding edge setters (`padding`, `padding_all`, `padding_x`, `padding_y`, etc.).
- **`impl_margin!`**: Outer box margin edge setters (`margin`, `margin_all`, `margin_x`, `margin_y`, etc.).
- **`impl_radius!`**: Corner rounding setters (`radius`, `radius_all`, `radius_top`, `radius_left`, etc.).
- **`impl_visuals!`**: Backgrounds, borders, drop shadows, and opacity (`bg`, `border`, `shadow`, etc.).
- **`impl_box_style_methods!`** *(Composite)*: Bundles `impl_dimensions!`, `impl_padding!`, `impl_margin!`, `impl_radius!`, and `impl_visuals!`.

### Typography Styling Macros
- **`impl_font_style_methods!`**: Core typography (`size`, `color`, `font_family`, `font_weight`).
- **`impl_text_wrap_methods!`**: Text wrapping flow (`wrap`).
- **`impl_text_decorations!`**: Text styling and decorations (`decoration`, `ellipsis`).
- **`impl_text_style_methods!`** *(Composite)*: Bundles `impl_font_style_methods!`, `impl_text_wrap_methods!`, and `impl_text_decorations!`.

---

## 3. Complete Generated Method Reference

### 3.1 Dimensions (`impl_dimensions!`)

Target field: `self.style` (`BoxStyle`).

| Method Signature | Parameters | Default | Lock | Description |
| :--- | :--- | :--- | :---: | :--- |
| `pub fn width(mut self, w: f32) -> Self` | `w: f32` | `None` (auto fit) | 🔒 Locked | Explicit box width in pixels. Disables `fill_x`. |
| `pub fn height(mut self, h: f32) -> Self` | `h: f32` | `None` (auto fit) | 🔒 Locked | Explicit box height in pixels. Disables `fill_y`. |
| `pub fn fill_x(mut self) -> Self` | — | `false` | 🔒 Locked | Stretches width to 100% of parent available space. |
| `pub fn fill_y(mut self) -> Self` | — | `false` | 🔒 Locked | Stretches height to 100% of parent available space. |
| `pub fn fill(mut self) -> Self` | — | `false` | 🔒 Locked | Shorthand to enable both `fill_x` and `fill_y`. |
| `pub fn min_width(mut self, w: f32) -> Self` | `w: f32` | `None` | 🔒 Locked | Minimum width constraint. |
| `pub fn max_width(mut self, w: f32) -> Self` | `w: f32` | `None` | 🔒 Locked | Maximum width constraint. |
| `pub fn min_height(mut self, h: f32) -> Self` | `h: f32` | `None` | 🔒 Locked | Minimum height constraint. |
| `pub fn max_height(mut self, h: f32) -> Self` | `h: f32` | `None` | 🔒 Locked | Maximum height constraint. |

---

### 3.2 Padding (`impl_padding!`)

Target field: `self.style.padding` (`EdgeInsets`).

| Method Signature | Parameters | Default | Lock | Description |
| :--- | :--- | :--- | :---: | :--- |
| `pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self` | `t, r, b, l: f32` | `EdgeInsets::ZERO` | 🔒 Locked | Sets all 4 edges independently (CSS order: top, right, bottom, left). |
| `pub fn padding_all(mut self, p: f32) -> Self` | `p: f32` | `EdgeInsets::ZERO` | 🔒 Locked | Sets uniform padding on all 4 edges. |
| `pub fn padding_x(mut self, p: f32) -> Self` | `p: f32` | `0.0` | 🔒 Locked | Sets horizontal padding (`left` and `right`). |
| `pub fn padding_y(mut self, p: f32) -> Self` | `p: f32` | `0.0` | 🔒 Locked | Sets vertical padding (`top` and `bottom`). |
| `pub fn padding_top(mut self, p: f32) -> Self` | `p: f32` | `0.0` | 🔒 Locked | Sets top edge padding only. |
| `pub fn padding_bottom(mut self, p: f32) -> Self` | `p: f32` | `0.0` | 🔒 Locked | Sets bottom edge padding only. |
| `pub fn padding_left(mut self, p: f32) -> Self` | `p: f32` | `0.0` | 🔒 Locked | Sets left edge padding only. |
| `pub fn padding_right(mut self, p: f32) -> Self` | `p: f32` | `0.0` | 🔒 Locked | Sets right edge padding only. |

---

### 3.3 Margin (`impl_margin!`)

Target field: `self.style.margin` (`EdgeInsets`).

| Method Signature | Parameters | Default | Lock | Description |
| :--- | :--- | :--- | :---: | :--- |
| `pub fn margin(mut self, t: f32, r: f32, b: f32, l: f32) -> Self` | `t, r, b, l: f32` | `EdgeInsets::ZERO` | 🔒 Locked | Sets all 4 outer margin edges independently (CSS order: top, right, bottom, left). |
| `pub fn margin_all(mut self, m: f32) -> Self` | `m: f32` | `EdgeInsets::ZERO` | 🔒 Locked | Sets uniform outer margin on all 4 edges. |
| `pub fn margin_x(mut self, m: f32) -> Self` | `m: f32` | `0.0` | 🔒 Locked | Sets horizontal outer margin (`left` and `right`). |
| `pub fn margin_y(mut self, m: f32) -> Self` | `m: f32` | `0.0` | 🔒 Locked | Sets vertical outer margin (`top` and `bottom`). |
| `pub fn margin_top(mut self, m: f32) -> Self` | `m: f32` | `0.0` | 🔒 Locked | Sets top edge outer margin only. |
| `pub fn margin_bottom(mut self, m: f32) -> Self` | `m: f32` | `0.0` | 🔒 Locked | Sets bottom edge outer margin only. |
| `pub fn margin_left(mut self, m: f32) -> Self` | `m: f32` | `0.0` | 🔒 Locked | Sets left edge outer margin only. |
| `pub fn margin_right(mut self, m: f32) -> Self` | `m: f32` | `0.0` | 🔒 Locked | Sets right edge outer margin only. |

---

### 3.4 Corner Radius (`impl_radius!`)

Target field: `self.style.radius` (`[f32; 4]` representing `[top_left, top_right, bottom_right, bottom_left]`).

| Method Signature | Parameters | Default | Lock | Description |
| :--- | :--- | :--- | :---: | :--- |
| `pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self` | `tl, tr, br, bl: f32` | `[0.0; 4]` | 🔒 Locked | Sets all 4 corners independently. |
| `pub fn radius_all(mut self, r: f32) -> Self` | `r: f32` | `[0.0; 4]` | 🔒 Locked | Sets uniform radius for all corners. |
| `pub fn radius_top(mut self, r: f32) -> Self` | `r: f32` | `0.0` | 🔒 Locked | Sets `top_left` and `top_right` radii. |
| `pub fn radius_bottom(mut self, r: f32) -> Self` | `r: f32` | `0.0` | 🔒 Locked | Sets `bottom_left` and `bottom_right` radii. |
| `pub fn radius_left(mut self, r: f32) -> Self` | `r: f32` | `0.0` | 🔒 Locked | Sets `top_left` and `bottom_left` radii. |
| `pub fn radius_right(mut self, r: f32) -> Self` | `r: f32` | `0.0` | 🔒 Locked | Sets `top_right` and `bottom_right` radii. |
| `pub fn radius_top_left(mut self, r: f32) -> Self` | `r: f32` | `0.0` | 🔒 Locked | Sets top-left corner radius only. |
| `pub fn radius_top_right(mut self, r: f32) -> Self` | `r: f32` | `0.0` | 🔒 Locked | Sets top-right corner radius only. |
| `pub fn radius_bottom_right(mut self, r: f32) -> Self` | `r: f32` | `0.0` | 🔒 Locked | Sets bottom-right corner radius only. |
| `pub fn radius_bottom_left(mut self, r: f32) -> Self` | `r: f32` | `0.0` | 🔒 Locked | Sets bottom-left corner radius only. |

---

### 3.5 Visuals, Borders & Shadows (`impl_visuals!`)

Target field: `self.style` (`BoxStyle`).

| Method Signature | Parameters | Default | Lock | Description |
| :--- | :--- | :--- | :---: | :--- |
| `pub fn style(mut self, style: BoxStyle) -> Self` | `BoxStyle` | `BoxStyle::new()` | 🔒 Locked | Replaces the entire underlying `BoxStyle` at once. |
| `pub fn bg(mut self, b: impl Into<Background>) -> Self` | `Color`, `Gradient`, or `None` | `None` (transparent) | 🔒 **Locked** | Unified background setter. Accepts `Color`, `Gradient::linear(...)`, `Gradient::radial(...)`, or `None`. Transparency is configured directly via `Color::rgba(...)`. |
| `pub fn border(mut self, border: impl Into<BorderInput>) -> Self` | `(Color, f32)` or `(Color, f32, BorderAlignment)` or `Border` | `Border::None` | 🔒 **Locked** | Unified border setter. Accepts standard `(Color, width)`, aligned `(Color, width, BorderAlignment)`, or `Border::None`. |
| `pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self` | `Color, f32, f32, f32` | `None` | 🔒 Locked | Drop shadow specification: color (including alpha), X offset, Y offset, and blur radius. |

---

### 3.6 Typography & Decoration (`impl_text_style_methods!`)

Target field: `self.text_style` (`TextStyle`).

| Method Signature | Parameters | Default | Lock | Description |
| :--- | :--- | :--- | :---: | :--- |
| `pub fn size(mut self, size: f32) -> Self` | `f32` | `14.0` / `16.0` | 🔒 **Locked** | Canonical font size in points. |
| `pub fn color(mut self, color: Color) -> Self` | `Color` | `Color::WHITE` | 🔒 **Locked** | Canonical text glyph color. |
| `pub fn font_family(mut self, family: impl Into<String>) -> Self` | `impl Into<String>` | `None` (system) | 🔒 **Locked** | Canonical font family name (e.g. `"Inter"`, `"monospace"`). |
| `pub fn font_weight(mut self, weight: impl Into<FontWeight>) -> Self` | `FontWeight` / `u16` | `FontWeight::Normal` | 🔒 **Locked** | Canonical font weight. Accepts numbers `400`..`900` or `FontWeight` enum. |
| `pub fn decoration(mut self, d: impl IntoDecoration) -> Self` | `TextDecoration`, `(Decoration, Color)`, or `[...]` | `None` | 🔒 **Locked** | Visual line decorations & styles (`Underline`, `Strikethrough`, `Italic`, `Highlight(Color)`, `LineHeight(f32)`). Inherits text color by default; overrides via tuple `(Decoration, Color)`. Accepts single item or array `[...]`. |
| `pub fn wrap(mut self, wrap: impl Into<TextWrap>) -> Self` | `impl Into<TextWrap>` | `TextWrap::Word` | 🔒 **Locked** | Text wrapping flow strategy (`Word`, `Character`, `None`, or `bool`). |
| `pub fn ellipsis(mut self, ellipsis: bool) -> Self` | `bool` | `false` | 🔒 **Locked** | Truncates overflowing text with trailing `...`. |

---

## 4. User Endpoints & How It Works

Application developers consume these macro-generated methods through fluent builder chains initiated from the `Ui` instance.

---

### 4.1 Master User Endpoint API Table

The table below catalogs every primary user-facing endpoint on `Ui`, which macros it implements, what methods become available, and how the endpoint terminates.

| User Endpoint (`Ui` Method) | Builder Struct | Macro Packages Applied | Macro & Builder Methods Exposed | Terminal Execution Method | Return Type |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **`ui.container()`** | `ContainerBuilder` | `impl_box_style_methods!` | **Box**: `.width()`, `.height()`, `.fill_x()`, `.fill_y()`, `.fill()`, `.min/max_width()`, `.min/max_height()`, `.padding()`, `.padding_all()`, `.padding_x/y()`, `.margin()`, `.margin_all()`, `.margin_x/y()`, `.radius()`, `.radius_all()`, `.radius_top/bottom/left/right()`, `.bg()`, `.border()`, `.shadow()`<br>**Layout**: `.align()`, `.halign()`, `.valign()`, `.gap()`, `.wrap(Wrap)` | `.show(\|ui\| { ... })` | `Response` |
| **`ui.row()`** | `ContainerBuilder` | `impl_box_style_methods!` | Same as `ui.container()` (pre-configured with `Direction::Row`, supports `.halign()`, `.valign()`, `.align()`) | `.show(\|ui\| { ... })` | `Response` |
| **`ui.column()`** | `ContainerBuilder` | `impl_box_style_methods!` | Same as `ui.container()` (pre-configured with `Direction::Column`, supports `.halign()`, `.valign()`, `.align()`) | `.show(\|ui\| { ... })` | `Response` |
| **`ui.stack()`** | `ContainerBuilder` | `impl_box_style_methods!` | Same as `ui.container()` (pre-configured with `Direction::Stack`, supports `.halign()`, `.valign()`, `.align()`) | `.show(\|ui\| { ... })` | `Response` |
| **`ui.button(label)`** | `ButtonBuilder` | `impl_box_style_methods!`<br>`impl_text_style_methods!` | **Box**: `.width()`, `.height()`, `.fill_x()`, `.padding()`, `.margin()`, `.radius()`, `.bg()`, `.border()`, `.shadow()`<br>**Text**: `.size()`, `.color()`, `.font_family()`, `.font_weight()`, `.decoration()`, `.wrap()`<br>**Layout**: `.align(Align)` (label alignment) | `.show()` | `Response` |
| **`ui.text(content)`** | `TextBuilder` | `impl_box_style_methods!`<br>`impl_text_style_methods!` | **Box (Badges)**: `.padding()`, `.margin()`, `.radius()`, `.bg()`, `.border()`, `.shadow()`<br>**Text**: `.size()`, `.color()`, `.font_family()`, `.font_weight()`, `.decoration()`, `.wrap()`, `.ellipsis()`<br>**Alignment**: `.align(Align)`, `.valign(Align)` | `.show()` | `(Response, Option<ShapedBuffer>)` |
| **`ui.h1(title)` .. `ui.h4`** | `TextBuilder` | `impl_box_style_methods!`<br>`impl_text_style_methods!` | Same as `ui.text()`, initialized with pre-scaled typographic sizing (h1: 28pt, h2: 22pt, h3: 18pt, h4: 16pt), supports `.align()`, `.valign()` | `.show()` | `(Response, Option<ShapedBuffer>)` |
| **`ui.input(&mut buf)`** | `InputBuilder` | `impl_box_style_methods!`<br>`impl_font_style_methods!` | **Box**: `.width()`, `.height()`, `.fill_x()`, `.padding()`, `.margin()`, `.radius()`, `.bg()`, `.border()`, `.shadow()`<br>**Font**: `.size()`, `.color()`, `.font_family()`, `.font_weight()` | `.show()` | `Response` |
| **`ui.text_area(&mut buf)`** | `TextAreaBuilder` | `impl_box_style_methods!`<br>`impl_font_style_methods!` | **Box**: `.width()`, `.height()`, `.fill_x()`, `.padding()`, `.margin()`, `.radius()`, `.bg()`, `.border()`, `.shadow()`<br>**Font**: `.size()`, `.color()`, `.font_family()`, `.font_weight()` | `.show()` | `Response` |

---

### 4.2 Cross-Endpoint Method Availability Matrix

This table maps every user-callable styling and layout method to each user endpoint:

| User-Callable Method | `ui.container()` | `ui.button()` | `ui.text()` | `ui.input()` | `ui.text_area()` | Lock | Category |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :--- |
| `.width(w: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Dimensions (Macro) |
| `.height(h: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Dimensions (Macro) |
| `.fill_x()` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Dimensions (Macro) |
| `.fill_y()` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Dimensions (Macro) |
| `.fill()` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Dimensions (Macro) |
| `.min_width(w)` / `.max_width(w)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Dimensions (Macro) |
| `.min_height(h)` / `.max_height(h)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Dimensions (Macro) |
| `.padding(t, r, b, l)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Padding (Macro) |
| `.padding_all(p: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Padding (Macro) |
| `.padding_x(p)` / `.padding_y(p)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Padding (Macro) |
| `.padding_top/bottom/left/right(p)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Padding (Macro) |
| `.margin(t, r, b, l)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Margin (Macro) |
| `.margin_all(m: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Margin (Macro) |
| `.margin_x(m)` / `.margin_y(m)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Margin (Macro) |
| `.margin_top/bottom/left/right(m)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Margin (Macro) |
| `.radius(tl, tr, br, bl)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Radius (Macro) |
| `.radius_all(r: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Radius (Macro) |
| `.radius_top/bottom/left/right(r)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Radius (Macro) |
| `.radius_top_left/right/etc.(r)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Radius (Macro) |
| `.style(BoxStyle)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Visuals (Macro) |
| `.bg(impl Into<Background>)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Visuals (Macro) |
| **`.border(...)`** | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 **Locked** | Visuals (Color, width, optional `BorderAlignment`) (Macro) |
| `.shadow(color, x, y, blur)` | ✅ | ✅ | ✅ | ✅ | ✅ | 🔒 Locked | Visuals (Macro) |
| **`.size(size: f32)`** | — | ✅ | ✅ | ✅ | ✅ | 🔒 **Locked** | Typography (Macro) |
| **`.color(Color)`** | — | ✅ | ✅ | ✅ | ✅ | 🔒 **Locked** | Typography (Macro) |
| **`.font_family(...)`** | — | ✅ | ✅ | ✅ | ✅ | 🔒 **Locked** | Typography (Macro) |
| **`.font_weight(...)`** | — | ✅ | ✅ | ✅ | ✅ | 🔒 **Locked** | Typography (Macro) |
| **`.decoration(...)`** | — | ✅ | ✅ | — | — | 🔒 **Locked** | Typography (Underline, Strikethrough, Italic, Highlight, LineHeight) (Macro) |
| **`.wrap(TextWrap)`** | — | ✅ | ✅ | — | — | 🔒 **Locked** | Wrap Flow (Macro) |
| **`.ellipsis(bool)`** | — | — | ✅ | — | — | 🔒 **Locked** | Text Decoration (Macro) |
| **`.align(Align)`** | ✅ | ✅ | ✅ | — | — | 🔒 **Locked** | Layout / Text Alignment (Builder) |
| **`.halign(Align)`** | ✅ | — | — | — | — | 🔒 **Locked** | Horizontal Layout Alignment (Builder) |
| **`.valign(Align)`** | ✅ | — | ✅ | — | — | 🔒 **Locked** | Vertical Layout / Text Alignment (Builder) |

> [!NOTE]
> **Border Alignment vs Layout Alignment**:
> - **Border Alignment** (`BorderAlignment::Inside`, `Center`, `Outside`) is part of box geometry and configured via `.border((color, width, alignment))`.
> - **Layout Alignment** (`Align::Left`, `Center`, `Right`, `Top`, `Bottom`, `SpaceBetween`, `SpaceAround`) is an orchestration property on builders: `ContainerBuilder` uses it for positioning children, `ButtonBuilder` for positioning its inner label, and `TextBuilder` for aligning glyph lines.

---

### 4.3 Production Endpoint Code Recipes

#### 1. Container & Row Endpoints (`ui.container()`, `ui.row()`)
```rust
// A. Centered card container
ui.container()
    .fill_x()
    .height(180.0)
    .padding_all(20.0)
    .align(Align::Center) // Centers child widgets both horizontally & vertically
    .bg(Color::rgb(0.10, 0.12, 0.16))
    .radius_all(12.0)
    .border((Color::rgb(0.25, 0.30, 0.45), 1.0, BorderAlignment::Inside))
    .shadow(Color::rgba(0.0, 0.0, 0.0, 0.35), 0.0, 6.0, 16.0)
    .show(|ui| {
        ui.text("Centered Card Content").show();
    });

// B. Justified navigation toolbar row
ui.row()
    .fill_x()
    .height(48.0)
    .halign(Align::SpaceBetween) // Distributes title on left, actions on right
    .valign(Align::Center)       // Centers items vertically
    .padding_x(16.0)
    .bg(Color::rgb(0.14, 0.16, 0.20))
    .show(|ui| {
        ui.text("Dashboard").show();
        ui.button("Settings").show();
    });
```

#### 2. Button Endpoint (`ui.button(label)`)
```rust
if ui.button("Save Changes")
    .fill_x()
    .height(42.0)
    .padding_x(18.0)
    .align(Align::Center) // Centers text inside button
    .bg(Gradient::linear(90.0, [Color::rgb(0.15, 0.45, 0.90), Color::rgb(0.25, 0.55, 1.00)]))
    .radius_all(6.0)
    .size(14.0)
    .font_weight(FontWeight::SemiBold)
    .color(Color::WHITE)
    .show()
    .clicked 
{
    save_changes();
}
```

#### 3. Text Endpoint (`ui.text(content)`)
```rust
// A. Badge with padding, background, and radius:
ui.text("v0.2.4")
    .size(12.0)
    .color(Color::rgb(0.40, 0.80, 1.0))
    .padding(3.0, 8.0, 3.0, 8.0)
    .bg(Color::rgb(0.12, 0.20, 0.32))
    .radius_all(4.0)
    .show();

// B. Decorated link with array of styles:
ui.text("Read the developer documentation")
    .size(14.0)
    .color(Color::WHITE)
    .font_weight(600)
    .decoration([
        TextDecoration::Underline,
        TextDecoration::Italic,
        TextDecoration::LineHeight(1.5),
    ])
    .show();

// C. Centered paragraph:
ui.text("Welcome back to your workspace")
    .fill_x()
    .size(16.0)
    .align(Align::Center)
    .show();
```

#### 4. Input Endpoint (`ui.input(&mut buffer)`)
```rust
ui.input(&mut search_query)
    .width(280.0)
    .height(36.0)
    .padding_x(10.0)
    .bg(Color::rgb(0.15, 0.17, 0.22))
    .border((Color::rgb(0.30, 0.35, 0.45), 1.0, BorderAlignment::Inside))
    .radius_all(6.0)
    .size(14.0)
    .color(Color::WHITE)
    .placeholder("Search documentation...")
    .show();
```

#### 5. TextArea Endpoint (`ui.text_area(&mut buffer)`)
```rust
ui.text_area(&mut code_buffer)
    .fill_x()
    .height(240.0)
    .padding_all(12.0)
    .bg(Color::rgb(0.08, 0.09, 0.12))
    .border((Color::rgb(0.20, 0.22, 0.30), 1.0))
    .radius_all(8.0)
    .size(13.0)
    .color(Color::rgb(0.85, 0.90, 0.95))
    .scrollable(true)
    .show();
```

---

## 5. Widget Author Guide: Implementing on Custom Widgets

To add complete, canonical styling to any custom widget, follow this 3-step pattern:

### Step 1: Implement `Styleable` on the Builder Struct
The struct must hold a `pub style: BoxStyle` field:

```rust
use zenthra_core::{BoxStyle, Styleable};

pub struct MyCardBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    pub style: BoxStyle,
    pub text_style: zenthra_text::prelude::TextStyle,
    // widget-specific fields...
}

impl<'u, 'a> Styleable for MyCardBuilder<'u, 'a> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}
```

### Step 2: Invoke the Macros
For standard 2-lifetime builders (`'u, 'a`):

```rust
// Automatically injects all dimensions, padding, radius, and visuals:
impl_box_style_methods!(MyCardBuilder);

// Automatically injects all typography methods:
impl_text_style_methods!(MyCardBuilder);
```

For builders with 3 lifetimes (e.g. holding a buffer reference `'b`):

```rust
impl_box_style_methods!(MyEditorBuilder, 'u, 'a, 'b);
impl_font_style_methods!(MyEditorBuilder, 'u, 'a, 'b);
```

### Step 3: Consume in `show()`
In the widget's `show()` method:
1. Extract box layout constraints:
   ```rust
   let width = self.style.width.unwrap_or(computed_content_w + self.style.padding.horizontal());
   let height = self.style.height.unwrap_or(computed_content_h + self.style.padding.vertical());
   ```
2. Emit background and border geometry via `RectDraw`:
   ```rust
   self.ui.draw_command(DrawCommand::Rect(RectDraw {
       bounds: [x, y, width, height],
       style: self.style,
       // ...
   }));
   ```

---

## 6. Summary Matrix

| Macro | Dimensions | Padding | Radius | Visuals / Borders / Shadows | Font & Color | Wrap | Decorations |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `impl_dimensions!` | ✅ | — | — | — | — | — | — |
| `impl_padding!` | — | ✅ | — | — | — | — | — |
| `impl_radius!` | — | — | ✅ | — | — | — | — |
| `impl_visuals!` | — | — | — | ✅ | — | — | — |
| `impl_box_style_methods!` | ✅ | ✅ | ✅ | ✅ | — | — | — |
| `impl_font_style_methods!` | — | — | — | — | ✅ | — | — |
| `impl_text_wrap_methods!` | — | — | — | — | — | ✅ | — |
| `impl_text_decorations!` | — | — | — | — | — | — | ✅ |
| `impl_text_style_methods!` | — | — | — | — | ✅ | ✅ | ✅ |

With this architecture, whenever a new style capability is added to `macros.rs`, every single widget in Zenthra inherits it automatically with zero boilerplate.
