# Text Widget

The `Text` widget is the primary tool for displaying typography in Zenthra. It uses the `cosmic-text` engine to provide high-quality shaping, layout, and font fallback support.

## Features

- **Subpixel Precision**: Smooth character spacing using fractional layout coordinates.
- **Rich Typography**: Support for custom weights, italics, and font families.
- **Word Wrapping**: Multiple wrapping strategies (Word, Character, or None).
- **Backgrounds & Highlights**: Built-in support for background colors and text highlighting.
- **Layout Aware**: Automatically respects parent container widths and alignment.

## API Reference

### Typography
| Method | Description |
|--------|-------------|
| `.size(f32)` | Sets the font size in logical pixels. |
| `.color(Color)` | Sets the text color. |
| `.font_family(impl Into<String>)` | Sets font family (e.g. "Inter", "monospace"). |
| `.font_weight(impl Into<FontWeight>)` | Sets weight (numeric `100`..`900` or `FontWeight` enum). |
| `.decoration(impl IntoDecoration)` | Applies text decorations (Underline, Strikethrough, Italic). |
| `.ellipsis(bool)` | Truncates overflowing text with trailing dots. |

### Layout & Sizing
| Method | Description |
|--------|-------------|
| `.wrap(TextWrap)` | `Word`, `Character`, or `None`. |
| `.align(Align)` | Horizontal alignment (`Left`, `Center`, `Right`). |
| `.max_width(f32)` | Constrains the text width for wrapping. |
| `.min_width(f32)` | Ensures the text block has a minimum width. |
| `.padding(t, r, b, l)` | Internal spacing within the text's background. |
| `.margin(t, r, b, l)` | External spacing added after the text block. |

### Visual Extras
| Method | Description |
|--------|-------------|
| `.bg(Color)` | Draws a background rectangle behind the text block. |
| `.cursor(CursorIcon)` | Changes the mouse cursor when hovering over the text. |

## Text Weight (Numeric)

Zenthra supports CSS-style numeric weights for maximum control:
- `100`: Thin
- `300`: Light
- `400`: Regular (Default)
- `500`: Medium
- `700`: Bold
- `900`: Black

```rust
ui.text("Heavy Text").font_weight(900).show();
```

## Example: Styled Paragraph

```rust
ui.text("Zenthra provides a modern, high-performance UI toolkit for Rust developers.")
    .size(16.0)
    .color(Color::rgb(0.8, 0.8, 0.8))
    .wrap(TextWrap::Word)
    .max_width(400.0)
    .show();
```

## Internal Mechanics

1. **Shaping**: Text is shaped using Rustybuzz to handle kerning and ligatures.
2. **Rasterization**: Glyphs are rasterized into a global atlas once and reused.
3. **Filtering**: The text pipeline uses **Linear Filtering** to ensure subpixel positions look smooth rather than jagged.
