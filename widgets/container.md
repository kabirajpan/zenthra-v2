# Container Widget

The `Container` is the core layout and styling primitive of Zenthra. It provides a Flexbox-like layout system combined with advanced rendering capabilities such as multi-corner radii, shadows, and hardware-accelerated clipping.

## Features

- **Flexbox-like Layout**: Intelligent Row and Column positioning with support for wrapping and complex alignment.
- **Content-Adaptive Sizing**: Automatically shrinks to fit children or expands to fill available space using `fill()`.
- **Advanced Styling**: Individual corner radius control, customizable border alignment (Inside, Center, Outside), and soft shadows.
- **Built-in Scrolling**: Native 2D scrolling with automatic visual scrollbars and interaction handling.
- **Hardware Clipping**: High-performance clipping of child elements to the container's boundaries (including rounded corners).
- **Interactive States**: Built-in support for hover and active effects (background, border, and scaling).
- **Overlay Support**: Ability to render in an overlay pass, perfect for dropdowns, tooltips, and modals.

## API Reference

### Positioning & Sizing
| Method | Description |
|--------|-------------|
| `.width(f32)` | Sets a fixed width. |
| `.height(f32)` | Sets a fixed height. |
| `.min_width(f32)` | Minimum width constraint. |
| `.max_width(f32)` | Maximum width constraint. |
| `.min_height(f32)` | Minimum height constraint. |
| `.max_height(f32)` | Maximum height constraint. |
| `.fill()` | Expand to fill all available parent space (both X and Y). |
| `.fill_x()` | Expand horizontally to fill available space. |
| `.fill_y()` | Expand vertically to fill available space. |
| `.full_width()` | Alias for `.fill_x()`. |
| `.full_height()` | Alias for `.fill_y()`. |
| `.absolute(x, y)` | Places the container at an absolute position, ignoring flow. |
| `.pos(x, y)` | Sets the starting cursor position for this container. |

### Layout & Alignment
| Method | Description |
|--------|-------------|
| `.row()` | Sets layout direction to Horizontal. |
| `.column()` | Sets layout direction to Vertical (Default). |
| `.halign(Align)` | Sets horizontal alignment of children (`Left`, `Center`, `Right`). |
| `.valign(Align)` | Sets vertical alignment of children (`Top`, `Center`, `Bottom`). |
| `.align(Align)` | Sets both horizontal and vertical alignment. |
| `.gap(f32)` | Sets the spacing between child elements. |
| `.wrap(Wrap)` | Sets wrapping strategy (`NoWrap`, `Wrap`, `WrapReverse`, etc.). |
| `.no_wrap()` | Disables wrapping (Default). |

### Padding & Borders
| Method | Description |
|--------|-------------|
| `.padding(t, r, b, l)` | Sets internal padding for all sides. |
| `.padding_x(f32)` | Sets left and right padding. |
| `.padding_y(f32)` | Sets top and bottom padding. |
| `.padding_top(f32)` | Sets only top padding. |
| `.padding_bottom(f32)` | Sets only bottom padding. |
| `.padding_left(f32)` | Sets only left padding. |
| `.padding_right(f32)` | Sets only right padding. |
| `.border(Color, width)` | Sets the border color and width. |
| `.border_alignment(BorderAlignment)`| Sets border placement: `Inside`, `Center`, or `Outside`. |

### Visual Styling
| Method | Description |
|--------|-------------|
| `.bg(Color)` | Sets the background color. |
| `.radius(tl, tr, br, bl)` | Shorthand for all four corner radii. |
| `.radius_all(f32)` | Sets all corners to the same value. |
| `.radius_top_left(f32)` | Sets top-left corner radius only. |
| `.radius_top_right(f32)` | Sets top-right corner radius only. |
| `.radius_bottom_right(f32)` | Sets bottom-right corner radius only. |
| `.radius_bottom_left(f32)` | Sets bottom-left corner radius only. |
| `.radius_top(f32)` | Sets both top corner radii. |
| `.radius_bottom(f32)` | Sets both bottom corner radii. |
| `.radius_left(f32)` | Sets both left corner radii. |
| `.radius_right(f32)` | Sets both right corner radii. |
| `.radius_x(f32)` | Shorthand for uniform radius (alias for all). |
| `.radius_y(f32)` | Shorthand for uniform radius (alias for all). |
| `.shadow(Color, x, y, blur)` | Adds a shadow with specific offset and blur radius. |
| `.shadow_opacity(f32)` | Multiplier for the shadow color's alpha channel. |
| `.opacity(f32)` | Sets the overall opacity of the container and its children (0.0 to 1.0). |
| `.render_mode(RenderMode)` | Sets the render mode (Static by default). |
| `.overlay()` | Renders this container in the overlay pass (on top of everything). |

### Interaction & State
| Method | Description |
|--------|-------------|
| `.id(impl Hash)` | Sets a deterministic ID (Required for state persistence and scrolling). |
| `.hover_bg(Color)` | Background color when mouse is hovering. |
| `.hover_border(Color, w)` | Border color/width when mouse is hovering. |
| `.hover_scale(f32)` | Scales the container on hover. |
| `.active_bg(Color)` | Background color when the container is active/pressed. |
| `.active_border(Color, w)` | Border color/width when the container is active/pressed. |
| `.active_scale(f32)` | Scales the container when active. |

### Scrolling & Clipping
| Method | Description |
|--------|-------------|
| `.scrollable(x, y)` | Enables horizontal and/or vertical scrolling. |
| `.scroll_x(bool)` | Enables horizontal scrolling. |
| `.scroll_y(bool)` | Enables vertical scrolling. |
| `.clip(bool)` | Force clips content to container bounds even if not scrolling. |

## Layout Rules

1. **The Content Box**: Children are placed within the area defined by `pos + border + padding`.
2. **Expansion**: `fill()` methods take precedence over intrinsic content size.
3. **Border Model**: The `BorderAlignment` determines how the border affects size:
    - `Outside`: Border is added outside the width/height (Total size = width + 2 * border).
    - `Inside`: Border is drawn inside the width/height (Total size = width).
    - `Center`: Border is centered on the edge.
4. **Deterministic IDs**: Always provide a unique `.id()` if the container is scrollable or interactive to ensure state is preserved between frames.

## Execution

The `.show(|ui| { ... })` method executes the container logic and returns a `Response` object.

```rust
let response = ui.container()
    .bg(Color::GRAY)
    .halign(Align::Center)
    .valign(Align::Center)
    .show(|ui| {
        ui.text("Click Me").show();
    });

if response.clicked {
    println!("Container clicked!");
}
```

## Examples

### Modern Card with Interactive States
```rust
ui.container()
    .id("card")
    .width(300.0)
    .bg(Color::rgb(0.15, 0.15, 0.15))
    .radius(12.0, 12.0, 12.0, 12.0)
    .padding(16.0, 16.0, 16.0, 16.0)
    .hover_bg(Color::rgb(0.2, 0.2, 0.2))
    .active_scale(0.98)
    .shadow(Color::BLACK, 0.0, 4.0, 10.0)
    .show(|ui| {
        ui.h3("Zenthra UI").show();
        ui.text("Interactive and responsive.").show();
    });
```
