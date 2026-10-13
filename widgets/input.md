# Input Widget

The `Input` widget is a single-line text entry field designed for forms, search bars, and labels. It features high-precision text editing and smooth horizontal scrolling.

## Features

- **Direct Data Binding**: Binds directly to a `&mut String`, updating your data in real-time as the user types.
- **Smart Horizontal Scrolling**: Automatically follows the cursor as you type, ensuring the active character is always visible.
- **Precision Editing**: Pixel-perfect cursor positioning using `cosmic-text` glyph metrics.
- **Visual Feedback**: Built-in "Smart Blink" cursor and a focus-highlighted border.
- **Keyboard Support**: Full support for arrow keys and backspace.

## API Reference

### Data & Identity
| Method | Description |
|--------|-------------|
| `.new(ui, &mut string, id)` | Creates a new input bound to the provided string buffer. |
| `.id(Id)` | Required for focus tracking and scroll state. |

### Sizing & Layout
| Method | Description |
|--------|-------------|
| `.width(f32)` | Sets a fixed width. |
| `.min_width(f32)` | Sets a minimum width. |
| `.fill_x()` | Expands the input to fill all available horizontal space. |
| `.padding(t, r, b, l)` | Spacing outside the text area (the "box" padding). |
| `.padding_all(f32)`, `.padding_x(f32)`, `.padding_y(f32)` | Uniform or directional padding shortcuts. |
| `.text_padding(t, r, b, l)` | Spacing between the text and the box edge. |

### Visuals & Styling
| Method | Description |
|--------|-------------|
| `.style(BoxStyle)` | Applies a complete, reusable `BoxStyle` theme. |
| `.bg(...)` | Solid color or gradient background surface (`None` for transparent). |
| `.border(...)` | Border color and width (`(Color, f32)` or `Border::Solid`). |
| `.focus_border(...)` | Custom border displayed when the input has active keyboard focus. |
| `.radius(...)` / `.radius_all(f32)` | Corner radii. |
| `.shadow(...)` / `.shadow_opacity(f32)` | Elevation drop shadow. |
| `.size(f32)` | Font size of the text. |
| `.color(Color)` | Color of the text. |
| `.font_family(...)` | Custom font family name. |
| `.font_weight(...)` | Numeric or enum font weight. |
| `.placeholder(...)` | Placeholder hint text when buffer is empty. |

## Interaction Rules

1. **Focus**: Click the input to focus it. A focused input will show a subtle border highlight.
2. **Scrolling**: If the text exceeds the width, a horizontal scrollbar appears. You can drag this bar or use the mouse wheel to scroll.
3. **Cursor**: The cursor automatically "warms up" (stops blinking) while you are actively typing to prevent visual distraction.

## Example: Login Form Input

```rust
let mut username = String::new();

ui.input(&mut username, "username-field")
    .bg(Color::rgb(0.1, 0.1, 0.1))
    .padding(8.0, 12.0, 8.0, 12.0)
    .fill_x()
    .show();
```

## Internal Mechanics

- **Event Consumption**: When focused, the `Input` widget consumes keyboard events from the `Ui` context.
- **Auto-Scroll**: The widget calculates the `x` position of the cursor glyph every frame and adjusts `scroll_x` to keep it within the viewport.
- **Hardware Clipping**: The text is strictly clipped to the input's bounding box using a GPU clip rectangle.
