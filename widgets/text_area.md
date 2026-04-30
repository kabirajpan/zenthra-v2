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
| `.width(f32)` | Sets the width (Default is `300.0`). |
| `.height(f32)` | Sets the fixed height (Enables scrolling if text is longer). |
| `.full_width()` | Expands to fill available horizontal space. |
| `.wrap(TextWrap)` | `Word`, `Character`, or `None`. |
| `.scrollable(bool)` | Enables/disables vertical scrolling. |

### Visuals
| Method | Description |
|--------|-------------|
| `.bg(Color)` | The background color of the text area. |
| `.size(f32)` | The font size of the text. |
| `.line_height(f32)` | The spacing between lines (Default is `1.2`). |
| `.padding(...)` / `.text_padding(...)` | Control spacing around the editor and text. |

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
