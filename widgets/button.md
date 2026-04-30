# Button Widget

The `Button` is an interactive component with built-in state management for hover and click behaviors. It automatically calculates its size based on its label and padding.

## Features

- **Automatic States**: Handles hover and active (pressed) states with visual feedback.
- **Intrinsic Sizing**: Automatically adjusts its width and height to fit the label text.
- **Customizable Styling**: Support for shadows, borders (strokes), and rounded corners.
- **Interaction Response**: Returns a `Response` object to handle click events in the application logic.

## API Reference

### Sizing & Layout
| Method | Description |
|--------|-------------|
| `.width(f32)` | Sets a fixed width. |
| `.height(f32)` | Sets a fixed height. |
| `.padding(t, r, b, l)` | Sets the internal padding around the label. |
| `.pos(x, y)` | Manually positions the button (overrides auto-layout). |

### Visual Styling
| Method | Description |
|--------|-------------|
| `.bg(Color)` | Sets the idle background color. |
| `.text_color(Color)` | Sets the label text color. |
| `.radius(tl, tr, br, bl)` | Sets independent corner radii. |
| `.stroke(Color, weight)` | Adds a border/stroke around the button. |
| `.shadow(Color, x, y, blur)` | Adds a drop shadow. |
| `.size(f32)` | Sets the font size of the label. |

### Interactive States
| Method | Description |
|--------|-------------|
| `.hover_bg(Color)` | The background color when the mouse is over the button. |
| `.active_bg(Color)` | The background color when the button is being pressed. |

### Execution
| Method | Description |
|--------|-------------|
| `.show()` | Renders the button and returns a `Response`. |

## The `Response` Object

When you call `.show()`, it returns a `Response` struct containing:
- `clicked`: `bool` (True only on the frame the button was released).
- `hovered`: `bool` (True while the mouse is over the button).
- `pressed`: `bool` (True while the mouse is held down on the button).

## Example: Interactive Button

```rust
let res = ui.button("Click Me")
    .bg(Color::rgb(0.2, 0.4, 0.8))
    .radius(6.0, 6.0, 6.0, 6.0)
    .padding(10.0, 20.0, 10.0, 20.0)
    .show();

if res.clicked {
    println!("Button was clicked!");
}
```

## Default Behavior

1. **Auto-Brightening**: By default, buttons will become slightly brighter on hover and darker when pressed if `hover_bg` or `active_bg` are not explicitly set.
2. **Text Centering**: The label text is always perfectly centered within the button's calculated or fixed area.
3. **Accessibility**: Buttons are registered with a `Role::Button` semantic node for screen readers.
