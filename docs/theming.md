# Theming & Unified Styling Architecture

Zenthra provides a single-source, zero-overhead styling system. All visual and layout properties are centralized into two core data structures:
1. **`BoxStyle`**: Controls geometry, dimensions, padding, corner radii, background colors/gradients, borders, and drop shadows.
2. **`TextStyle`**: Controls typography, font sizes, colors, font families, font weights, styles (italic), line heights, wrap modes, and text highlights.

Widgets implement the `Styleable` trait and call the unified styling macros, giving every widget the same consistent, fluent styling API.

---

## 1. Core Mental Model

Instead of every widget inventing its own loose fields (`bg`, `padding`, `radius`, `border`, `font_size`, etc.), all widgets delegate to:

```text
WidgetBuilder
 ├── pub style: BoxStyle             <-- Layout, Box Geometry & Visuals
 └── pub text_style: TextStyle       <-- Typography & Text Appearance
```

This unlocks two powerful capabilities:
1. **Identical Fluent API**: Methods like `.bg()`, `.border()`, `.padding()`, `.radius_all()`, `.bold()`, and `.monospace()` work identically on `Button`, `Container`, `Input`, `TextArea`, and `Text`.
2. **Theme Sharing**: You can pass pre-defined `BoxStyle` or `TextStyle` objects directly to any widget via `.style(...)` and `.text_style(...)`.

---

## 2. Reusable Themes

Define your visual styles once, then apply them across multiple widgets:

```rust
use zenthra::prelude::*;

// Define a shared card/input surface theme:
let mut dark_surface = BoxStyle::new();
dark_surface.bg = Some(Background::Solid(Color::rgb(0.14, 0.15, 0.18)));
dark_surface.radius = [6.0; 4];
dark_surface.border = Border::Solid(Color::rgb(0.28, 0.30, 0.36), 1.0);
dark_surface.padding = EdgeInsets::symmetric(8.0, 12.0);

// Define a shared typography style:
let mut code_font = TextStyle::default();
code_font.font_family = Some("Fira Code".to_string());
code_font.font_size = 14.0;
code_font.color = Color::rgb(0.9, 0.9, 0.95);
code_font.line_height = 1.35;
```

### Applying Themes to Widgets

```rust
// 1. Single-line Input
ui.input(&mut user_command)
    .style(dark_surface.clone())
    .text_style(code_font.clone())
    .placeholder("$ type command here...")
    .show();

// 2. Multi-line TextArea (inherits same styling, overrides height)
ui.text_area(&mut script_buffer, "script_editor")
    .style(dark_surface.clone())
    .text_style(code_font.clone())
    .max_height(250.0) // On-the-fly override
    .show();

// 3. Container Card
ui.container()
    .style(dark_surface)
    .show(|ui| {
        ui.text("Inside styled card").text_style(code_font).show();
    });
```

---

## 3. On-The-Fly Overrides

When using `.style(theme)` or `.text_style(theme)`, you can always override specific properties by chaining standard methods after:

```rust
ui.input(&mut search)
    .style(dark_surface)        // Base theme (width: 320.0, bg: dark)
    .width(480.0)               // Custom override: wider for search bar
    .placeholder("Search...")
    .show();
```

---

## 4. Complete Styling Reference

### Box Styling (`BoxStyle` / `impl_box_style_methods!`)

| Category | Method | Description |
| :--- | :--- | :--- |
| **Dimensions** | `.width(f32)` | Fixed width |
| | `.height(f32)` | Fixed height |
| | `.min_width(f32)`, `.max_width(f32)` | Width constraints |
| | `.min_height(f32)`, `.max_height(f32)` | Height constraints |
| | `.fill_x()`, `.full_width()` | Expand to fill container width |
| | `.fill_y()`, `.full_height()` | Expand to fill container height |
| | `.fill()` | Expand both axes |
| **Padding** | `.padding(t, r, b, l)` | 4-sided box padding |
| | `.padding_all(p)` | Uniform padding |
| | `.padding_x(x)`, `.padding_y(y)` | Symmetric directional padding |
| | `.padding_top(t)`, `.padding_bottom(b)`, etc. | Individual side padding |
| **Corners** | `.radius(tl, tr, br, bl)` | 4 corner radii |
| | `.radius_all(r)` | Uniform radius for all corners |
| | `.radius_top(r)`, `.radius_bottom(r)` | Top or bottom pair radius |
| | `.radius_left(r)`, `.radius_right(r)` | Left or right pair radius |
| **Surfaces** | `.bg(Background)` | Solid color, gradient, or `None` |
| | `.bg_gradient(start, end, dir)` | Linear or radial gradient |
| | `.bg_opacity(alpha)` | Background opacity multiplier |
| **Borders** | `.border((Color, width))` | Solid border or `None` |
| | `.border_alignment(align)` | `Inside`, `Center`, or `Outside` |
| **Shadows** | `.shadow(color, ox, oy, blur)` | Elevation shadow |
| | `.shadow_opacity(alpha)` | Shadow opacity multiplier |
| | `.opacity(alpha)` | Overall widget opacity |

### Typography Styling (`TextStyle` / `impl_text_style_methods!`)

| Method | Description |
| :--- | :--- |
| `.font_size(size)` / `.size(size)` | Font size in logical pixels |
| `.text_color(color)` / `.color(color)` | Text fill color |
| `.font_family(name)` / `.family(name)` | Custom font family name |
| `.monospace()` | Shortcut for `font_family("monospace")` |
| `.font_weight(weight)` / `.weight(weight)` | `Thin`, `Light`, `Regular`, `Medium`, `SemiBold`, `Bold`, `Black` |
| `.bold()` | Shortcut for `FontWeight::Bold` |
| `.font_style(style)` | Font style (`Normal`, `Italic`) |
| `.italic()` | Shortcut for `FontStyle::Italic` |
| `.line_height(lh)` | Line height multiplier (e.g. `1.2`, `1.4`) |
| `.wrap(strategy)` | `TextWrap::Word`, `TextWrap::Glyph`, `TextWrap::None` |
| `.auto_wrap()` | Shortcut for `TextWrap::Word` |
| `.no_wrap()` | Shortcut for `TextWrap::None` |
| `.highlight(color)` | Selection highlight background color |
| `.ellipsis(bool)` | Truncate overflowing single-line text with `...` |

---

## 5. Supported Widgets

The following widgets are currently unified under this styling architecture:
- [`Container`](file:///home/kabir/Desktop/projects/personal/framworks/zenthra-v2/docs/api/container.md)
- [`Button`](file:///home/kabir/Desktop/projects/personal/framworks/zenthra-v2/docs/api/button.md)
- [`Text`](file:///home/kabir/Desktop/projects/personal/framworks/zenthra-v2/docs/api/text.md)
- [`Input`](file:///home/kabir/Desktop/projects/personal/framworks/zenthra-v2/docs/api/input.md)
- [`TextArea`](file:///home/kabir/Desktop/projects/personal/framworks/zenthra-v2/docs/api/text_area.md)
