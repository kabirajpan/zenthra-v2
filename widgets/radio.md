# Radio Widget

Single-selection option control for enumerated or mutually exclusive values with smooth animated dot scaling, typography, and unified box styling.

## Basic Usage

```rust
let mut selected = 1;

ui.radio(&mut selected, 1, "Standard Option")
    .size(18.0)
    .gap(10.0)
    .colors(Color::rgb(0.4, 0.4, 0.4), Color::rgb(0.2, 0.5, 1.0))
    .show();
```

## Styling Options

| Method | Description |
|---|---|
| `.style(BoxStyle)` | Reusable BoxStyle for the radio ring |
| `.size(f32)` | Width and height of radio ring |
| `.gap(f32)` | Spacing between ring and label |
| `.colors(ring, dot)` | Convenience setter for ring and dot colors |
| `.ring_color(Color)` | Inactive ring border color |
| `.dot_color(Color)` | Active dot fill and ring highlight color |
| `.bg(Color)` | Radio ring interior background |
| `.shadow(Color, x, y, blur)` | Ring drop shadow |
| `.color(Color)` | Label text color |
| `.active_label(Color)` | Label color transition when selected |
| `.glow(bool)` | Neon accent aura when selected |
| `.disabled(bool)` | Disable interactions |
