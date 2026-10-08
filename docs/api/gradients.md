# Gradient System API

Zenthra provides a hardware-accelerated WebGPU gradient system designed for simplicity, consistency, and complete creative freedom.

---

## 1. Core Philosophy

1. **Single Method Entry Point**: Every background in Zenthra (solid color, linear gradient, radial spotlight, or freeform mesh) is applied via the exact same method:
   ```rust
   ui.container().bg(...)
   ```
2. **Zero Method Chaining Confusion**: Gradients are constructed in a single call with two arguments: `(Where, What Colors)`. You never have to chain confusing helper methods:
   - `Gradient::linear(direction_or_angle, colors)`
   - `Gradient::radial(anchor_position, colors)`
   - `Gradient::mesh(points)`
3. **Hardware Accelerated**: Computed natively on the GPU in the fragment shader at full display refresh rate (144+ FPS) with zero texture memory overhead.

---

## 2. API Signature Reference

### Linear Gradients
```rust
Gradient::linear(direction_or_angle, colors_or_stops)
```
- **`direction_or_angle`**:
  - Named direction: `Direction::ToRight`, `Direction::ToLeft`, `Direction::ToBottom`, `Direction::ToTop`, `Direction::ToBottomRight`, `Direction::ToTopRight`
  - Degree angle (f32): `45.0`, `135.0`, `68.5`, etc.
- **`colors_or_stops`**:
  - Array of colors: `[Color::BLUE, Color::RED]` or `[Color::rgb(0.1, 0.5, 0.3), ...]`
  - Array of raw RGB tuples: `[(0.08, 0.48, 0.32), (0.12, 0.68, 0.45), (0.08, 0.48, 0.32)]`
  - Array of custom stops: `[(0.0, Color::RED), (0.15, Color::BLUE), (1.0, Color::RED)]` or `[(0.0, (r, g, b)), ...]`

### Radial Gradients
```rust
Gradient::radial(anchor_position, colors_or_stops)
```
- **`anchor_position`**:
  - Named alignment: `Align::Center`, `Align::TopLeft`, `Align::Left`, `Align::Bottom`, etc.
  - Normalized coordinate: `(0.25, 0.80)` (X, Y percentage across the card)
  - Off-screen coordinate: `(-0.30, 0.50)` for dramatic side lighting
- **`colors_or_stops`**:
  - Array of colors: `[Color::RED, Color::GREEN]`
  - Array of custom stops: `[(0.0, Color::RED), (0.5, Color::PURPLE), (1.0, Color::BLUE)]`

### Freeform Mesh Gradients
```rust
Gradient::mesh(points)
```
- **`points`**: Array of `(position, color)` pairs placed anywhere on the card:
  - `[((0.0, 0.0), Color::YELLOW), ((1.0, 0.0), Color::GREEN), ...]`

---

## 3. Complete Examples Collection

### Family 1: Linear Directional (Standard Edges & Flows)

#### 1. Left to Right (Horizontal Flow)
```rust
ui.container().bg(
    Gradient::linear(Direction::ToRight, [Color::BLUE, Color::RED])
);
```

#### 2. Right to Left (Reverse Flow)
```rust
ui.container().bg(
    Gradient::linear(Direction::ToLeft, [Color::BLUE, Color::RED])
);
```

#### 3. Top to Bottom (Vertical Drop)
```rust
ui.container().bg(
    Gradient::linear(Direction::ToBottom, [
        Color::rgb(0.12, 0.14, 0.20),
        Color::rgb(0.04, 0.05, 0.07),
    ])
);
```

#### 4. Bottom to Top (Ground Up / Fog)
```rust
ui.container().bg(
    Gradient::linear(Direction::ToTop, [Color::BLACK, Color::TRANSPARENT])
);
```

#### 5. Top-Left to Bottom-Right (Diagonal Sheen)
```rust
ui.container().bg(
    Gradient::linear(Direction::ToBottomRight, [
        Color::rgb(0.20, 0.50, 1.00),
        Color::rgb(0.80, 0.20, 0.80),
    ])
);
```

#### 6. Bottom-Left to Top-Right
```rust
ui.container().bg(
    Gradient::linear(Direction::ToTopRight, [
        Color::rgb(0.05, 0.05, 0.08),
        Color::rgb(0.18, 0.22, 0.32),
    ])
);
```

---

### Family 2: Linear Angles (Exact Degrees)

#### 7. 45° Standard Card Angle
```rust
ui.container().bg(
    Gradient::linear(45.0, [Color::BLUE, Color::PURPLE])
);
```

#### 8. 135° Modern Dark SaaS Card
```rust
ui.container().bg(
    Gradient::linear(135.0, [
        Color::rgb(0.14, 0.15, 0.18),
        Color::rgb(0.06, 0.07, 0.09),
    ])
);
```

#### 9. Precision Angle (68.5° Sunset Angle)
```rust
ui.container().bg(
    Gradient::linear(68.5, [
        Color::rgb(0.95, 0.35, 0.20),
        Color::rgb(0.40, 0.10, 0.50),
    ])
);
```

---

### Family 3: Multi-Color & Custom Stop Positions

#### 10. 3 Colors Evenly Spaced
```rust
ui.container().bg(
    Gradient::linear(Direction::ToRight, [
        Color::BLUE,
        Color::WHITE,
        Color::RED,
    ])
);
```

#### 11. 4 Colors (Rainbow / Spectrum)
```rust
ui.container().bg(
    Gradient::linear(Direction::ToRight, [
        Color::rgb(0.20, 0.50, 1.00), // Blue
        Color::rgb(0.20, 0.80, 0.60), // Teal
        Color::rgb(0.90, 0.80, 0.20), // Yellow
        Color::rgb(0.90, 0.30, 0.30), // Red
    ])
);
```

#### 12. 3 Colors: Subtle Symmetrical Edge Darkening
Left edge little bit dark, center vibrant green, right edge little bit dark (same as left), linear from left to right:
```rust
// Directly with raw RGB tuples in the array (no hidden methods needed):
ui.container().bg(
    Gradient::linear(Direction::ToRight, [
        (0.08, 0.48, 0.32), // Left: green a little bit dark
        (0.12, 0.68, 0.45), // Middle: base green
        (0.08, 0.48, 0.32), // Right: green a little bit dark
    ])
);

// Or with RGBA tuples (with alpha):
ui.container().bg(
    Gradient::linear(Direction::ToRight, [
        (0.08, 0.48, 0.32, 0.85),
        (0.12, 0.68, 0.45, 1.00),
        (0.08, 0.48, 0.32, 0.85),
    ])
);
```

#### 13. Custom Positions: Audio VU Meter (Green → Yellow → Red)
```rust
ui.container().bg(
    Gradient::linear(Direction::ToRight, [
        (0.00, Color::rgb(0.10, 0.80, 0.35)), // Green (Safe)
        (0.70, Color::rgb(0.10, 0.80, 0.35)),
        (0.85, Color::rgb(0.95, 0.80, 0.10)), // Yellow (Warning)
        (1.00, Color::rgb(0.95, 0.20, 0.20)), // Red (Peaking)
    ])
);
```

#### 14. Metallic Chrome / Silver Glint
```rust
ui.container().bg(
    Gradient::linear(45.0, [
        (0.00, Color::rgb(0.50, 0.50, 0.55)),
        (0.40, Color::rgb(0.95, 0.95, 1.00)), // Bright reflection
        (0.60, Color::rgb(0.30, 0.30, 0.35)), // Deep chrome shadow
        (1.00, Color::rgb(0.70, 0.70, 0.75)),
    ])
);
```

#### 15. Opacity Fade-Out (Solid Red to 0% Transparent)
```rust
ui.container().bg(
    Gradient::linear(Direction::ToRight, [
        (0.0, Color::RED),
        (1.0, Color::TRANSPARENT),
    ])
);
```

---

### Family 4: Radial Gradients (Spotlights, Vignettes & Halos)

#### 16. Centered Radial (Middle Red, Outer Green)
```rust
ui.container().bg(
    Gradient::radial(Align::Center, [Color::RED, Color::GREEN])
);
```

#### 17. Center Dark Vignette (Bright Core to Pitch Black Edge)
```rust
ui.container().bg(
    Gradient::radial(Align::Center, [
        Color::rgb(0.15, 0.16, 0.20),
        Color::BLACK,
    ])
);
```

#### 18. Left-Edge Glow (Spotlight Anchored at Left Side of Card)
```rust
ui.container().bg(
    Gradient::radial(Align::Left, [Color::RED, Color::BLUE])
);
```

#### 19. Top-Left Corner Glow (Hero Section Backlight)
```rust
ui.container().bg(
    Gradient::radial(Align::TopLeft, [
        Color::rgba(0.20, 0.50, 1.00, 0.40),
        Color::TRANSPARENT,
    ])
);
```

#### 20. Bottom-Right Corner Spotlight
```rust
ui.container().bg(
    Gradient::radial(Align::BottomRight, [
        Color::rgb(0.90, 0.30, 0.50),
        Color::BLACK,
    ])
);
```

#### 21. Arbitrary Percentage Position `(x, y)` Anywhere in the Box
```rust
// Anchored at 25% from left, 80% from top
ui.container().bg(
    Gradient::radial((0.25, 0.80), [
        Color::rgb(0.90, 0.50, 0.10),
        Color::TRANSPARENT,
    ])
);
```

#### 22. Off-Screen Light Source (Anchored Outside the Card)
```rust
// Light source placed 30% beyond the left boundary
ui.container().bg(
    Gradient::radial((-0.30, 0.50), [
        Color::rgba(0.90, 0.20, 0.40, 0.70),
        Color::TRANSPARENT,
    ])
);
```

#### 23. Radial Concentric Pulse Ring (Multi-Stop)
```rust
ui.container().bg(
    Gradient::radial(Align::Center, [
        (0.0, Color::CYAN),
        (0.3, Color::rgba(0.0, 0.8, 1.0, 0.20)),
        (0.7, Color::CYAN),                      // Concentric glowing ring
        (1.0, Color::TRANSPARENT),
    ])
);
```

---

### Family 5: Freeform Mesh Gradients (Organic Apple/Stripe Cards)

#### 24. 4-Corner Mesh Card (Apple & Stripe Wallpaper Style)
```rust
ui.container().bg(
    Gradient::mesh([
        ((0.0, 0.0), Color::rgb(0.98, 0.88, 0.15)), // Top-Left: Yellow
        ((1.0, 0.0), Color::rgb(0.15, 0.78, 0.42)), // Top-Right: Green
        ((0.0, 1.0), Color::rgb(0.92, 0.22, 0.22)), // Bottom-Left: Red
        ((1.0, 1.0), Color::rgb(0.15, 0.45, 0.95)), // Bottom-Right: Blue
    ])
);
```

#### 25. 5-Point Organic Mesh (4 Corners + Warm Amber Center)
```rust
ui.container().bg(
    Gradient::mesh([
        ((0.10, 0.10), Color::YELLOW),
        ((0.90, 0.10), Color::GREEN),
        ((0.50, 0.50), Color::rgb(0.95, 0.50, 0.15)), // Center amber warmth
        ((0.10, 0.90), Color::RED),
        ((0.90, 0.90), Color::BLUE),
    ])
);
```

#### 26. 3-Point Pastel Mesh (Minimalist Mood Gradient)
```rust
ui.container().bg(
    Gradient::mesh([
        ((0.20, 0.20), Color::rgb(0.95, 0.75, 0.85)), // Soft pink
        ((0.80, 0.30), Color::rgb(0.75, 0.85, 0.95)), // Soft sky blue
        ((0.50, 0.90), Color::rgb(0.85, 0.95, 0.80)), // Soft mint
    ])
);
```

---

## 4. Interactive State Support

Gradients work out-of-the-box on hover and active states using the exact same syntax:

```rust
ui.container()
    // Default state: dark subtle diagonal
    .bg(Gradient::linear(135.0, [
        Color::rgb(0.10, 0.10, 0.14),
        Color::rgb(0.06, 0.06, 0.08),
    ]))
    // Hover state: vibrant glow
    .hover_bg(Gradient::linear(135.0, [
        Color::rgb(0.20, 0.40, 0.90),
        Color::rgb(0.10, 0.20, 0.60),
    ]))
```
