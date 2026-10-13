# Toggle Widget

An interactive sliding toggle switch with customizable track box, thumb knob, and optional external/inner labels.

## Basic Usage

```rust
let mut dark_mode = false;

ui.toggle(&mut dark_mode, "Dark Mode")
    .width(48.0)
    .height(24.0)
    .padding(3.0)
    .pill()
    .colors(
        Color::rgb(0.0, 0.5, 1.0), // Active track
        Color::rgb(0.2, 0.2, 0.2), // Inactive track
        Color::WHITE,              // Thumb knob
    )
    .show();
```

## Styling Options

| Method | Description |
|---|---|
| `.style(BoxStyle)` | Reusable BoxStyle for track box |
| `.text_style(TextStyle)` | Reusable TextStyle for external label |
| `.size(w, h)` / `.width(w)` / `.height(h)` | Track dimensions |
| `.padding(f32)` / `.padding_all(f32)` | Spacing between track and thumb |
| `.pill()` / `.square()` | Track shape presets |
| `.radius(f32)` / `.radius_all(f32)` | Corner radii |
| `.colors(on, off, thumb)` | Convenience setter for track & thumb colors |
| `.on_color(Color)` / `.off_color(Color)` | Track state colors |
| `.thumb_color(Color)` / `.thumb_radius(f32)` | Thumb knob styling |
| `.inner_labels(on, off)` | Inner track status ("ON" / "OFF") |
| `.label_left()` / `.label_right()` | External label placement side |
| `.label_size(f32)` / `.label_color(Color)` | External label typography |
| `.glow(bool)` | Track neon aura when active |
| `.shadow(Color, x, y, blur)` | Track drop shadow |
| `.disabled(bool)` | Disable interactions |
