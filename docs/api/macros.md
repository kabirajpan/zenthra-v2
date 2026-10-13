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
In UI frameworks, widget styling methods (such as `.width()`, `.padding()`, `.radius()`, `.bg()`, `.border()`, `.font_size()`) frequently duplicate code across builders. Without centralization, widgets develop subtle inconsistencies—missing lifetimes, mismatched method names, diverging default values, or accidental aliases.

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
- **`impl_dimensions!`**: Layout box sizing (`width`, `height`, `fill_x`, `fill_y`, `fill`, `min_width`, `max_width`, etc.).
- **`impl_padding!`**: Inner box padding edge setters (`padding`, `padding_all`, `padding_x`, `padding_y`, etc.).
- **`impl_radius!`**: Corner rounding setters (`radius`, `radius_all`, `radius_top`, `radius_left`, etc.).
- **`impl_visuals!`**: Backgrounds, borders, drop shadows, and opacity (`bg`, `border`, `shadow`, etc.).
- **`impl_box_style_methods!`** *(Composite)*: Bundles `impl_dimensions!`, `impl_padding!`, `impl_radius!`, and `impl_visuals!`.

### Typography Styling Macros
- **`impl_font_style_methods!`**: Core typography (`font_size`, `color`, `text_color`, `family`, `weight`, `bold`, `italic`, `line_height`).
- **`impl_text_wrap_methods!`**: Text wrapping flow (`wrap`, `auto_wrap`, `no_wrap`).
- **`impl_text_decorations!`**: Text line decorations and highlights (`ellipsis`, `underline`, `strikethrough`, `highlight`).
- **`impl_text_style_methods!`** *(Composite)*: Bundles `impl_font_style_methods!`, `impl_text_wrap_methods!`, and `impl_text_decorations!`.

---

## 3. Complete Generated Method Reference

### 3.1 Dimensions (`impl_dimensions!`)

Target field: `self.style` (`BoxStyle`).

| Method Signature | Parameters | Default | Description |
| :--- | :--- | :--- | :--- |
| `pub fn width(mut self, w: f32) -> Self` | `w: f32` | `None` (auto fit) | Explicit box width in pixels. Disables `fill_x`. |
| `pub fn height(mut self, h: f32) -> Self` | `h: f32` | `None` (auto fit) | Explicit box height in pixels. Disables `fill_y`. |
| `pub fn fill_x(mut self) -> Self` | — | `false` | Stretches width to 100% of parent available space. |
| `pub fn fill_y(mut self) -> Self` | — | `false` | Stretches height to 100% of parent available space. |
| `pub fn fill(mut self) -> Self` | — | `false` | Shorthand to enable both `fill_x` and `fill_y`. |
| `pub fn min_width(mut self, w: f32) -> Self` | `w: f32` | `None` | Minimum width constraint. |
| `pub fn max_width(mut self, w: f32) -> Self` | `w: f32` | `None` | Maximum width constraint. |
| `pub fn min_height(mut self, h: f32) -> Self` | `h: f32` | `None` | Minimum height constraint. |
| `pub fn max_height(mut self, h: f32) -> Self` | `h: f32` | `None` | Maximum height constraint. |

---

### 3.2 Padding (`impl_padding!`)

Target field: `self.style.padding` (`EdgeInsets`).

| Method Signature | Parameters | Default | Description |
| :--- | :--- | :--- | :--- |
| `pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self` | `t, r, b, l: f32` | `EdgeInsets::ZERO` | Sets all 4 edges independently (CSS order: top, right, bottom, left). |
| `pub fn padding_all(mut self, p: f32) -> Self` | `p: f32` | `EdgeInsets::ZERO` | Sets uniform padding on all 4 edges. |
| `pub fn padding_x(mut self, p: f32) -> Self` | `p: f32` | `0.0` | Sets horizontal padding (`left` and `right`). |
| `pub fn padding_y(mut self, p: f32) -> Self` | `p: f32` | `0.0` | Sets vertical padding (`top` and `bottom`). |
| `pub fn padding_top(mut self, p: f32) -> Self` | `p: f32` | `0.0` | Sets top edge padding only. |
| `pub fn padding_bottom(mut self, p: f32) -> Self` | `p: f32` | `0.0` | Sets bottom edge padding only. |
| `pub fn padding_left(mut self, p: f32) -> Self` | `p: f32` | `0.0` | Sets left edge padding only. |
| `pub fn padding_right(mut self, p: f32) -> Self` | `p: f32` | `0.0` | Sets right edge padding only. |

---

### 3.3 Corner Radius (`impl_radius!`)

Target field: `self.style.radius` (`[f32; 4]` representing `[top_left, top_right, bottom_right, bottom_left]`).

| Method Signature | Parameters | Default | Description |
| :--- | :--- | :--- | :--- |
| `pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self` | `tl, tr, br, bl: f32` | `[0.0; 4]` | Sets all 4 corners independently. |
| `pub fn radius_all(mut self, r: f32) -> Self` | `r: f32` | `[0.0; 4]` | Sets uniform radius for all corners. |
| `pub fn radius_top(mut self, r: f32) -> Self` | `r: f32` | `0.0` | Sets `top_left` and `top_right` radii. |
| `pub fn radius_bottom(mut self, r: f32) -> Self` | `r: f32` | `0.0` | Sets `bottom_left` and `bottom_right` radii. |
| `pub fn radius_left(mut self, r: f32) -> Self` | `r: f32` | `0.0` | Sets `top_left` and `bottom_left` radii. |
| `pub fn radius_right(mut self, r: f32) -> Self` | `r: f32` | `0.0` | Sets `top_right` and `bottom_right` radii. |
| `pub fn radius_top_left(mut self, r: f32) -> Self` | `r: f32` | `0.0` | Sets top-left corner radius only. |
| `pub fn radius_top_right(mut self, r: f32) -> Self` | `r: f32` | `0.0` | Sets top-right corner radius only. |
| `pub fn radius_bottom_right(mut self, r: f32) -> Self` | `r: f32` | `0.0` | Sets bottom-right corner radius only. |
| `pub fn radius_bottom_left(mut self, r: f32) -> Self` | `r: f32` | `0.0` | Sets bottom-left corner radius only. |

---

### 3.4 Visuals, Borders & Shadows (`impl_visuals!`)

Target field: `self.style` (`BoxStyle`).

| Method Signature | Parameters | Default | Description |
| :--- | :--- | :--- | :--- |
| `pub fn style(mut self, style: BoxStyle) -> Self` | `BoxStyle` | `BoxStyle::new()` | Replaces the entire underlying `BoxStyle` at once. |
| `pub fn bg(mut self, b: impl Into<Background>) -> Self` | `Color`, `Gradient`, or `Background` | `None` (transparent) | Sets background fill. Accepts `Color`, `Gradient`, `Background::None`, or `None`. |
| `pub fn bg_gradient(mut self, start: Color, end: Color, dir: GradientDirection) -> Self` | `Color, Color, GradientDirection` | `None` | Shorthand helper creating a 2-stop linear or radial gradient. |
| `pub fn bg_opacity(mut self, opacity: f32) -> Self` | `opacity: f32` | `1.0` | Multiplies the alpha channel of any solid or gradient background. |
| `pub fn border(mut self, border: impl Into<Border>) -> Self` | `(Color, f32)` or `Border` | `Border::None` | Sets border outline. Accepts `(Color, width)`, `Border::Solid(c, w)`, or `Border::None`. |
| `pub fn border_alignment(mut self, alignment: BorderAlignment) -> Self` | `BorderAlignment` | `BorderAlignment::Inside` | Border stroke placement (`Inside`, `Center`, or `Outside`). |
| `pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self` | `Color, f32, f32, f32` | `None` | Drop shadow specification: color, X offset, Y offset, and blur radius. |
| `pub fn shadow_opacity(mut self, opacity: f32) -> Self` | `opacity: f32` | `1.0` | Multiplies the shadow color alpha transparency. |
| `pub fn opacity(mut self, o: f32) -> Self` | `o: f32` | `1.0` | Overall container/widget composite opacity (0.0 = invisible, 1.0 = opaque). |

---

### 3.5 Typography & Font Styling (`impl_font_style_methods!`)

Target field: `self.text_style` (`TextStyle`).

| Method Signature | Parameters | Default | Description |
| :--- | :--- | :--- | :--- |
| `pub fn text_style(mut self, style: TextStyle) -> Self` | `TextStyle` | `TextStyle::default()` | Replaces the entire underlying `TextStyle` at once. |
| `pub fn font_size(mut self, size: f32) -> Self` | `size: f32` | `14.0` / `16.0` | Font size in device-independent points. |
| `pub fn color(mut self, color: Color) -> Self` | `Color` | `Color::WHITE` | Text glyph color. |
| `pub fn text_color(self, color: Color) -> Self` | `Color` | `Color::WHITE` | Semantic alias for `.color(Color)`. |
| `pub fn family(mut self, family: impl Into<String>) -> Self` | `&str` / `String` | `None` (system) | Font family name (e.g. `"Inter"`, `"JetBrains Mono"`). |
| `pub fn font_family(self, family: impl Into<String>) -> Self` | `&str` / `String` | `None` | Semantic alias for `.family(...)`. |
| `pub fn monospace(mut self) -> Self` | — | `false` | Shorthand setting family to `"monospace"`. |
| `pub fn weight(mut self, weight: impl Into<FontWeight>) -> Self` | `FontWeight` | `FontWeight::Normal` | Font weight (`Thin`, `Normal`, `Medium`, `Bold`, etc.). |
| `pub fn font_weight(self, weight: impl Into<FontWeight>) -> Self` | `FontWeight` | `FontWeight::Normal` | Semantic alias for `.weight(...)`. |
| `pub fn bold(mut self) -> Self` | — | `false` | Shorthand setting weight to `FontWeight::Bold`. |
| `pub fn bold_when(mut self, bold: bool) -> Self` | `bool` | — | Conditionally sets weight to `FontWeight::Bold` if `true`. |
| `pub fn font_style(mut self, style: FontStyle) -> Self` | `FontStyle` | `FontStyle::Normal` | Font style variant (`Normal` or `Italic`). |
| `pub fn italic(mut self) -> Self` | — | `false` | Shorthand setting style to `FontStyle::Italic`. |
| `pub fn italic_when(mut self, italic: bool) -> Self` | `bool` | — | Conditionally sets style to `FontStyle::Italic` if `true`. |
| `pub fn line_height(mut self, lh: f32) -> Self` | `lh: f32` | `1.2` | Line height multiplier (relative to font size). |

---

### 3.6 Text Wrapping (`impl_text_wrap_methods!`)

Target field: `self.text_style.wrap` (`TextWrap`).

| Method Signature | Parameters | Default | Description |
| :--- | :--- | :--- | :--- |
| `pub fn wrap(mut self, wrap: impl Into<TextWrap>) -> Self` | `TextWrap` | `TextWrap::Word` | Explicit wrap strategy (`Word`, `Glyph`, or `None`). |
| `pub fn auto_wrap(mut self) -> Self` | — | `TextWrap::Word` | Shorthand setting wrap to `TextWrap::Word`. |
| `pub fn no_wrap(mut self) -> Self` | — | `TextWrap::None` | Shorthand disabling text wrapping (`TextWrap::None`). |

---

### 3.7 Text Decorations (`impl_text_decorations!`)

Target field: `self.text_style` (`TextStyle`).

| Method Signature | Parameters | Default | Description |
| :--- | :--- | :--- | :--- |
| `pub fn highlight(mut self, color: Color) -> Self` | `Color` | `None` | Background marker highlight behind text glyphs. |
| `pub fn ellipsis(mut self, ellipsis: bool) -> Self` | `bool` | `false` | Truncates overflowing text with trailing `...`. |
| `pub fn underline(mut self, underline: bool) -> Self` | `bool` | `false` | Draws an underline stroke beneath text. |
| `pub fn underline_color(mut self, color: Color) -> Self` | `Color` | Text color | Sets underline to `true` with a custom stroke color. |
| `pub fn strikethrough(mut self, strikethrough: bool) -> Self` | `bool` | `false` | Draws a horizontal strike line across the text center. |
| `pub fn strikethrough_color(mut self, color: Color) -> Self` | `Color` | Text color | Sets strikethrough to `true` with a custom stroke color. |

---

## 4. User Endpoints & How It Works

Application developers consume these macro-generated methods through fluent builder chains initiated from the `Ui` instance.

---

### 4.1 Master User Endpoint API Table

The table below catalogs every primary user-facing endpoint on `Ui`, which macros it implements, what methods become available, and how the endpoint terminates.

| User Endpoint (`Ui` Method) | Builder Struct | Macro Packages Applied | Macro Methods Exposed | Terminal Execution Method | Return Type |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **`ui.container()`** | `ContainerBuilder` | `impl_box_style_methods!` | `.width()`, `.height()`, `.fill_x()`, `.fill_y()`, `.fill()`, `.min/max_width()`, `.min/max_height()`, `.padding()`, `.padding_all()`, `.padding_x/y()`, `.radius()`, `.radius_all()`, `.radius_top/bottom/left/right()`, `.bg()`, `.border()`, `.shadow()`, `.opacity()` | `.show(\|ui\| { ... })` | `Response` |
| **`ui.row()`** | `ContainerBuilder` | `impl_box_style_methods!` | Same as `ui.container()` (pre-configured with `Direction::Row`) | `.show(\|ui\| { ... })` | `Response` |
| **`ui.column()`** | `ContainerBuilder` | `impl_box_style_methods!` | Same as `ui.container()` (pre-configured with `Direction::Column`) | `.show(\|ui\| { ... })` | `Response` |
| **`ui.stack()`** | `ContainerBuilder` | `impl_box_style_methods!` | Same as `ui.container()` (pre-configured with `Direction::Stack`) | `.show(\|ui\| { ... })` | `Response` |
| **`ui.button(label)`** | `ButtonBuilder` | `impl_box_style_methods!`<br>`impl_text_style_methods!` | **Box**: `.width()`, `.height()`, `.fill_x()`, `.padding()`, `.radius()`, `.bg()`, `.border()`, `.shadow()`, `.opacity()`<br>**Text**: `.font_size()`, `.text_color()`, `.family()`, `.weight()`, `.bold()`, `.italic()`, `.wrap()` | `.show()` | `Response` |
| **`ui.text(content)`** | `TextBuilder` | `impl_box_style_methods!`<br>`impl_text_style_methods!` | **Box (Badges)**: `.padding()`, `.radius()`, `.bg()`, `.border()`, `.shadow()`<br>**Text**: `.font_size()`, `.color()`, `.family()`, `.monospace()`, `.weight()`, `.bold()`, `.italic()`, `.line_height()`, `.wrap()`, `.ellipsis()`, `.underline()`, `.strikethrough()` | `.show()` | `(Response, Option<ShapedBuffer>)` |
| **`ui.h1(title)` .. `ui.h4`** | `TextBuilder` | `impl_box_style_methods!`<br>`impl_text_style_methods!` | Same as `ui.text()`, initialized with pre-scaled typographic sizing (h1: 28pt, h2: 22pt, h3: 18pt, h4: 16pt) | `.show()` | `(Response, Option<ShapedBuffer>)` |
| **`ui.input(&mut buf)`** | `InputBuilder` | `impl_box_style_methods!`<br>`impl_font_style_methods!` | **Box**: `.width()`, `.height()`, `.fill_x()`, `.padding()`, `.radius()`, `.bg()`, `.border()`, `.shadow()`<br>**Font**: `.font_size()`, `.text_color()`, `.font_family()`, `.line_height()` | `.show()` | `Response` |
| **`ui.text_area(&mut buf)`** | `TextAreaBuilder` | `impl_box_style_methods!`<br>`impl_font_style_methods!` | **Box**: `.width()`, `.height()`, `.fill_x()`, `.padding()`, `.radius()`, `.bg()`, `.border()`, `.shadow()`<br>**Font**: `.font_size()`, `.text_color()`, `.font_family()`, `.monospace()`, `.line_height()` | `.show()` | `Response` |

---

### 4.2 Cross-Endpoint Method Availability Matrix

This table maps every user-callable macro method to each user endpoint:

| User-Callable Method | `ui.container()` | `ui.button()` | `ui.text()` | `ui.input()` | `ui.text_area()` | Category |
| :--- | :---: | :---: | :---: | :---: | :---: | :--- |
| `.width(w: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | Dimensions |
| `.height(h: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | Dimensions |
| `.fill_x()` | ✅ | ✅ | ✅ | ✅ | ✅ | Dimensions |
| `.fill_y()` | ✅ | ✅ | ✅ | ✅ | ✅ | Dimensions |
| `.fill()` | ✅ | ✅ | ✅ | ✅ | ✅ | Dimensions |
| `.min_width(w)` / `.max_width(w)` | ✅ | ✅ | ✅ | ✅ | ✅ | Dimensions |
| `.min_height(h)` / `.max_height(h)` | ✅ | ✅ | ✅ | ✅ | ✅ | Dimensions |
| `.padding(t, r, b, l)` | ✅ | ✅ | ✅ | ✅ | ✅ | Padding |
| `.padding_all(p: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | Padding |
| `.padding_x(p)` / `.padding_y(p)` | ✅ | ✅ | ✅ | ✅ | ✅ | Padding |
| `.padding_top/bottom/left/right(p)` | ✅ | ✅ | ✅ | ✅ | ✅ | Padding |
| `.radius(tl, tr, br, bl)` | ✅ | ✅ | ✅ | ✅ | ✅ | Radius |
| `.radius_all(r: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | Radius |
| `.radius_top/bottom/left/right(r)` | ✅ | ✅ | ✅ | ✅ | ✅ | Radius |
| `.radius_top_left/right/etc.(r)` | ✅ | ✅ | ✅ | ✅ | ✅ | Radius |
| `.style(BoxStyle)` | ✅ | ✅ | ✅ | ✅ | ✅ | Visuals |
| `.bg(impl Into<Background>)` | ✅ | ✅ | ✅ | ✅ | ✅ | Visuals |
| `.bg_opacity(opacity: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | Visuals |
| `.border(impl Into<Border>)` | ✅ | ✅ | ✅ | ✅ | ✅ | Visuals |
| `.border_alignment(BorderAlignment)` | ✅ | ✅ | ✅ | ✅ | ✅ | Visuals |
| `.shadow(color, x, y, blur)` | ✅ | ✅ | ✅ | ✅ | ✅ | Visuals |
| `.shadow_opacity(opacity: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | Visuals |
| `.opacity(o: f32)` | ✅ | ✅ | ✅ | ✅ | ✅ | Visuals |
| `.font_size(size: f32)` | — | ✅ | ✅ | ✅ | ✅ | Typography |
| `.color(Color)` / `.text_color(Color)` | — | ✅ | ✅ | ✅ | ✅ | Typography |
| `.family(&str)` / `.font_family(&str)` | — | ✅ | ✅ | ✅ | ✅ | Typography |
| `.monospace()` | — | ✅ | ✅ | ✅ | ✅ | Typography |
| `.weight(FontWeight)` | — | ✅ | ✅ | ✅ | ✅ | Typography |
| `.bold()` / `.bold_when(bool)` | — | ✅ | ✅ | ✅ | ✅ | Typography |
| `.font_style(FontStyle)` | — | ✅ | ✅ | ✅ | ✅ | Typography |
| `.italic()` / `.italic_when(bool)` | — | ✅ | ✅ | ✅ | ✅ | Typography |
| `.line_height(lh: f32)` | — | ✅ | ✅ | ✅ | ✅ | Typography |
| `.wrap(TextWrap)` | — | ✅ | ✅ | — | — | Wrap Flow |
| `.auto_wrap()` / `.no_wrap()` | — | ✅ | ✅ | — | — | Wrap Flow |
| `.highlight(Color)` | — | — | ✅ | — | — | Text Decoration |
| `.ellipsis(bool)` | — | — | ✅ | — | — | Text Decoration |
| `.underline(bool)` / `.underline_color(Color)` | — | — | ✅ | — | — | Text Decoration |
| `.strikethrough(bool)` / `.strikethrough_color(Color)` | — | — | ✅ | — | — | Text Decoration |

---

### 4.3 Production Endpoint Code Recipes

#### 1. Container Endpoint (`ui.container()`)
```rust
ui.container()
    .fill_x()
    .height(180.0)
    .padding_all(20.0)
    .bg(Color::rgb(0.10, 0.12, 0.16))
    .radius_all(12.0)
    .border((Color::rgb(0.25, 0.30, 0.45), 1.0))
    .shadow(Color::BLACK, 0.0, 6.0, 16.0)
    .shadow_opacity(0.35)
    .show(|ui| {
        // Child widgets...
    });
```

#### 2. Button Endpoint (`ui.button(label)`)
```rust
if ui.button("Save Changes")
    .fill_x()
    .height(42.0)
    .padding_x(18.0)
    .bg(Color::rgb(0.15, 0.45, 0.90))
    .radius_all(6.0)
    .font_size(14.0)
    .weight(FontWeight::SemiBold)
    .text_color(Color::WHITE)
    .show()
    .clicked 
{
    save_changes();
}
```

#### 3. Text Endpoint (`ui.text(content)`)
```rust
// Badge with padding, background, and radius:
ui.text("v0.2.4")
    .font_size(12.0)
    .monospace()
    .color(Color::rgb(0.40, 0.80, 1.0))
    .padding(3.0, 8.0, 3.0, 8.0)
    .bg(Color::rgb(0.12, 0.20, 0.32))
    .radius_all(4.0)
    .show();
```

#### 4. Input Endpoint (`ui.input(&mut buffer)`)
```rust
ui.input(&mut search_query)
    .width(280.0)
    .height(36.0)
    .padding_x(10.0)
    .bg(Color::rgb(0.15, 0.17, 0.22))
    .border((Color::rgb(0.30, 0.35, 0.45), 1.0))
    .radius_all(6.0)
    .font_size(14.0)
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
    .font_size(13.0)
    .monospace()
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
