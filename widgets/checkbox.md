# Checkbox Widget

A boolean toggle control that directly binds to `&mut bool` with animated checkmark glyph, typography, and unified box styling.

## Basic Usage

```rust
let mut auto_save = true;

ui.checkbox(&mut auto_save, "Auto-save changes")
    .size(18.0)
    .gap(8.0)
    .radius_all(4.0)
    .bg(Color::rgb(0.15, 0.15, 0.15))
    .border((Color::rgb(0.3, 0.3, 0.3), 1.0))
    .check_bg(Color::rgb(0.2, 0.5, 1.0))
    .check_color(Color::WHITE)
    .show();
```

## Styling Options

| Method | Description |
|---|---|
| `.style(BoxStyle)` | Reusable BoxStyle for checkbox box |
| `.size(f32)` | Width & height of checkbox box |
| `.gap(f32)` | Spacing between box and label |
| `.bg(Color)` | Unchecked box background |
| `.border(Color, f32)` | Unchecked box border |
| `.check_bg(Color)` | Checked box background |
| `.check_color(Color)` | Checkmark glyph color |
| `.radius(f32)` / `.radius_all(f32)` | Corner radii |
| `.color(Color)` | Label typography color |
| `.active_label(Color)` | Label color transition when checked |
| `.glow(bool)` | Neon accent aura when checked |
| `.shadow(Color, x, y, blur)` | Box drop shadow |
