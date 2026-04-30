# LazyContainer Widget

The `LazyContainer` is a high-performance virtualization component designed to render thousands (or millions) of items with minimal CPU and GPU overhead. It achieves this by only executing the rendering logic for items that are currently visible within the viewport.

## Features

- **Virtual Rendering**: Skips processing for any item outside the scroll view.
- **Fixed-Size Grid/List**: Optimized for uniform item sizes.
- **Built-in Scrolling**: Handles mouse wheel scrolling and scrollbar rendering internally.
- **Wrap Support**: Automatically calculates row/column counts and wrap positions.
- **Infinite Scalability**: Performance remains constant regardless of the total item count.

## API Reference

### Configuration
| Method | Description |
|--------|-------------|
| `.item_size(w, h)` | Sets the uniform width and height for every item in the list. |
| `.count(usize)` | The total number of items in the collection. |
| `.gap(f32)` | Spacing between items. |
| `.padding(t, r, b, l)` | Sets internal padding (Currently uniform based on the first argument). |
| `.id(Id)` | **Required**. Used to persist scroll position across frames. |

### Layout & Direction
| Method | Description |
|--------|-------------|
| `.row()` | Arrange items horizontally. If wrapping is enabled, it forms a grid. |
| `.column()` | Arrange items vertically (Standard list view). |
| `.wrap(Wrap)` | `Wrap::Wrap` enables grid-like behavior for Row direction. |

### Visual Styling
| Method | Description |
|--------|-------------|
| `.bg(Color)` | Sets the background color for the viewport area. |
| `.radius(r, r, r, r)` | Sets the corner radius for the viewport (Currently uniform). |

### Rendering
| Method | Description |
|--------|-------------|
| `.show(|ui, index|)` | The core render loop. The closure is called once for each visible item. |

## Performance Tips

1. **Fixed Item Size**: `LazyContainer` requires all items to have the same size to calculate scroll offsets efficiently without measuring every item.
2. **Deterministic IDs**: Always provide a stable `.id()` so that the scroll position doesn't reset when other UI elements change.
3. **Heavy Logic**: Try to keep the logic inside the `.show()` closure lightweight, as it runs every frame for the visible set.

## Example: A Grid of 10,000 Items

```rust
ui.lazy_container()
    .id("photo-grid")
    .item_size(150.0, 150.0)
    .count(10000)
    .gap(10.0)
    .row()
    .wrap(Wrap::Wrap)
    .fill()
    .show(|ui, i| {
        // This code only runs for the ~20 items visible on screen
        ui.container()
            .bg(Color::rgb(0.2, 0.2, 0.2))
            .center()
            .show(|ui| {
                ui.text(&format!("Item #{}", i)).show();
            });
    });
```

## Internal Mechanics

1. **Viewport Calculation**: It determines the visible range by checking the current `scroll_y` against `item_height + gap`.
2. **Layout Isolation**: Each item's `Ui` context is isolated, so `ui.advance()` calls inside an item don't affect the global layout.
3. **Clip Injection**: The container automatically injects its own viewport bounds into the `clip_rect` of every child draw command.
