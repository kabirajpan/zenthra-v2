# Slider Widget

The `Slider` is an interactive input component that allows users to select a value from a continuous or stepped range by dragging a thumb along a horizontal track.

## Features

- **Range Support**: Define custom `min` and `max` values.
- **Stepped Input**: Optionally snap to specific increments using `.step()`.
- **Smooth Dragging**: High-performance dragging logic with visual feedback on hover and active states.
- **Customizable Visuals**: Control track height, thumb radius, and colors.
- **Direct Binding**: Binds directly to a `&mut f32` value.

## API Reference

### Data & Configuration
| Method | Description |
|--------|-------------|
| `.new(ui, &mut value, id)` | Creates a new slider bound to the provided f32 value. |
| `.range(min, max)` | Sets the minimum and maximum bounds (Default is `0.0` to `1.0`). |
| `.step(f32)` | Snaps the value to the nearest multiple of the step. |

### Layout & Sizing
| Method | Description |
|--------|-------------|
| `.width(f32)` | Sets the total width of the slider track. |
| `.height(f32)` | Sets the touch/interaction height (Default is `32.0`). |

### Visual Styling
| Method | Description |
|--------|-------------|
| `.track_color(Color)` | The color of the horizontal bar. |
| `.thumb_color(Color)` | The color of the draggable circle. |

## Example: Volume Slider

```rust
let mut volume = 50.0;

ui.slider(&mut volume, "volume-control")
    .range(0.0, 100.0)
    .step(1.0)
    .width(200.0)
    .show();
```

## Interaction Rules

1. **Dragging**: Click and hold the thumb (or anywhere on the track) to begin dragging.
2. **Visual Feedback**: The thumb will glow/brighten when hovered and show a subtle shadow when active.
3. **Redraw**: The UI automatically requests a redraw when the value changes during dragging to ensure the thumb position remains fluid.

## Internal Mechanics

- **Mapping**: The widget maps the mouse position relative to the track width into the logical `[min, max]` range.
- **State Persistence**: Uses the provided `id` to track which slider is currently being dragged, allowing for stable interaction even if the mouse leaves the track bounds.
