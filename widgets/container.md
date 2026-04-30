# Container Widget

The `Container` is the core layout and styling component of Zenthra. It behaves similarly to a CSS Flexbox container with advanced rendering capabilities like rounded corners, shadows, and hardware-accelerated clipping.

## Features

- **Flexbox-like Layout**: Support for Rows, Columns, Wrapping, and Alignment.
- **Content-Adaptive Sizing**: Automatically shrinks to fit children if no dimensions are specified.
- **Advanced Styling**: 4-corner independent radius, outside borders, and soft shadows.
- **Built-in Scrolling**: Support for 2D scrolling with automatic clipping.
- **Interactive States**: Hover and Click state support.

## API Reference

### Sizing & Constraints
| Method | Description |
|--------|-------------|
| `.width(f32)` | Sets a fixed width. |
| `.height(f32)` | Sets a fixed height. |
| `.min_width(f32)` | Minimum width constraint. |
| `.max_width(f32)` | Maximum width constraint. |
| `.min_height(f32)` | Minimum height constraint. |
| `.max_height(f32)` | Maximum height constraint. |
| `.fill()` | Expand to fill all available parent space. |
| `.fill_x()` | Expand horizontally to fill available space. |
| `.fill_y()` | Expand vertically to fill available space. |

### Layout & Alignment
| Method | Description |
|--------|-------------|
| `.direction(Direction)` | `Direction::Row` or `Direction::Column`. |
| `.halign(Align)` | Horizontal alignment of children. |
| `.valign(Align)` | Vertical alignment of children. |
| `.center()` | Centers children in both directions. |
| `.gap(f32)` | Spacing between children. |
| `.wrap(bool)` | Enables line/column wrapping. |
| `.reverse(bool)` | Reverses the order of children. |

### Padding & Borders
| Method | Description |
|--------|-------------|
| `.padding(t, r, b, l)` | Sets internal padding on all sides. |
| `.padding_x(f32)` | Sets left and right padding. |
| `.padding_y(f32)` | Sets top and bottom padding. |
| `.border(Color, width)` | Draws a border **outside** the container bounds. |

### Visual Styling
| Method | Description |
|--------|-------------|
| `.bg(Color)` | Sets the background color. |
| `.radius(tl, tr, br, bl)` | Sets independent corner radii. |
| `.shadow(Color, x, y, blur)` | Adds a shadow with offset and blur. |
| `.shadow_opacity(f32)` | Controls shadow transparency (0.0 to 1.0). |

### Scrolling & State
| Method | Description |
|--------|-------------|
| `.id(Id)` | Required for scroll state and interactivity persistence. |
| `.scroll(bool)` | Enables both horizontal and vertical scrolling. |
| `.scroll_x()` | Enables horizontal scrolling. |
| `.scroll_y()` | Enables vertical scrolling. |
| `.scrollbar(bool)` | Shows/hides visual scrollbars. |
| `.on_click(F)` | Closure executed when the container is clicked. |
| `.hover_bg(Color)` | Background color when mouse is hovering. |
| `.hover_border(Color, w)` | Border color/width when mouse is hovering. |

## Layout Rules

1. **The Content Box**: Children are placed starting at `pos + border_width + padding`.
2. **Expansion**: `fill()` takes precedence over intrinsic content size.
3. **Clipping**: All children are automatically clipped to the container's radius and border boundaries.
4. **Border Model**: Zenthra uses an "outside" border model. If you have a 100px wide box with a 2px border, the total visual width on screen will be 104px.

## Examples

### Simple Centered Box
```rust
ui.container()
    .bg(Color::WHITE)
    .radius(8.0, 8.0, 8.0, 8.0)
    .padding(20.0, 20.0, 20.0, 20.0)
    .center()
    .show(|ui| {
        ui.text("Hello Zenthra").show();
    });
```

### Scrollable Sidebar
```rust
ui.container()
    .id("sidebar")
    .width(250.0)
    .fill_y()
    .scroll_y()
    .bg(Color::rgb(0.1, 0.1, 0.1))
    .show(|ui| {
        for i in 0..50 {
            ui.text(&format!("Item {}", i)).show();
        }
    });
```
