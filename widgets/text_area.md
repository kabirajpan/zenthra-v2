# TextArea Widget

The `TextArea` is a multi-line text editor designed for long-form content, code blocks, or chat messages. It supports complex multi-line navigation and vertical scrolling.

## Features

- **Multi-line Input**: Full support for the `Enter` key and multi-line buffer management.
- **Vertical Navigation**: Handles `Up` and `Down` arrow keys, intelligently moving the cursor to the nearest character in the next or previous line.
- **Flexible Wrapping**: Configurable word-wrapping strategies to fit any layout.
- **Built-in ScrollView**: Automatically handles vertical scrolling and scrollbar rendering for large bodies of text.
- **GPU Clipping**: Ensures text never bleeds outside the editor bounds.

## API Reference

### Data & Identity
| Method | Description |
|--------|-------------|
| `.new(ui, &mut string, id)` | Creates a new editor bound to a multi-line string buffer. |
| `.id(Id)` | **Required** for focus, cursor tracking, and scroll state. |

### Sizing & Layout
| Method | Description |
|--------|-------------|
| `.width(f32)` | Sets the fixed width (Default is `300.0`). |
| `.height(f32)` | Sets the fixed height. |
| `.max_height(f32)` | Auto-grows up to this maximum height before scrolling. |
| `.fill_x()` | Expands to fill available horizontal space. |
| `.fill_y()` | Expands to fill available vertical space. |
| `.padding(t, r, b, l)` | Spacing outside the editor (box padding). |
| `.padding_all(f32)`, `.padding_x(f32)`, `.padding_y(f32)` | Directional padding shortcuts. |
| `.text_padding(t, r, b, l)` | Spacing between text and the box edge. |
| `.wrap(TextWrap)` | `TextWrap::Word`, `TextWrap::Glyph`, or `TextWrap::None`. |
| `.scrollable(bool)` | Enables/disables vertical and horizontal scrolling. |

### Visuals & Styling
| Method | Description |
|--------|-------------|
| `.style(BoxStyle)` | Applies a complete, reusable `BoxStyle` theme. |
| `.bg(...)` | Solid color or gradient background surface (`None` for transparent). |
| `.border(...)` | Border color and width (`(Color, f32)` or `Border::Solid`). |
| `.focus_border(...)` | Custom border displayed when the text area has active focus. |
| `.radius(...)` / `.radius_all(f32)` | Corner radii. |
| `.shadow(...)` / `.shadow_opacity(f32)` | Elevation drop shadow. |
| `.size(f32)` | Font size of the text. |
| `.color(Color)` | Color of the text. |
| `.font_family(...)` | Custom font family name. |
| `.font_weight(...)` | Numeric or enum font weight. |
| `.placeholder(...)` | Placeholder hint text when buffer is empty. |
| `.submit_on_enter(bool)` | When true, pressing `Enter` submits rather than creating a newline. |

## Smart Navigation

The `TextArea` features an intelligent line-switching algorithm:
- When you press `Up` or `Down`, it measures the `x` position of your current cursor.
- It then calculates which character on the target line is closest to that `x` coordinate.
- This results in a "natural" feel, similar to modern IDEs or text editors.

## Example: Simple Note Editor

```rust
let mut notes = String::new();

ui.text_area(&mut notes, "notes-editor")
    .bg(Color::rgb(0.05, 0.05, 0.05))
    .width(500.0)
    .height(300.0)
    .scrollable(true)
    .padding(10.0, 10.0, 10.0, 10.0)
    .show();
```

## Internal Mechanics

- **Buffer Modification**: Directly modifies the bound `String` by inserting/removing UTF-8 characters.
- **Line Height Calculation**: Uses `font_size * line_height` to determine the virtual grid for cursor placement.
- **Activity Tracking**: Uses an `interaction_state` timestamp to reset the cursor blink cycle whenever you type or move.
