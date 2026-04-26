# Architecture

An overview of how Zenthra works internally.

---

## Overview

```
App::run()
  └── Platform window loop (winit)
        └── Each frame:
              ├── Collect platform events (keyboard, mouse, scroll)
              ├── Build Ui context
              ├── Run user's with_ui() closure → produces DrawCommands
              └── Render pipeline → flush to GPU (WGPU)
```

---

## Rendering Pipeline

Zenthra uses a two-phase render model:

### Phase 1 — UI Build (CPU)
The user's closure runs every frame. Widgets push `DrawCommand` values into `ui.draws`:

```rust
pub enum DrawCommand {
    Rect(RectDraw),         // colored/rounded rectangles, shadows, borders
    Text(TextDraw),         // shaped glyphs with clip rect
    OverlayRect(OverlayRectDraw), // post-layout overlays (scrollbars, etc.)
}
```

No GPU calls happen here. The UI tree is just a flat `Vec<DrawCommand>`.

### Phase 2 — GPU Flush (GPU)
The render pipeline batches all draw commands and uploads them as GPU instances:
- `RectInstance` → rectangle shader (SDF-based rounded corners, shadows)
- Text glyphs → glyph atlas texture + text shader
- Everything goes through a single clip rect per draw command

---

## Layout Engine

Containers collect their children's sizes and reposition them **after** all children have run:

```
container.show(|ui| {
    child_a.show();   // records size → child_sizes[0]
    child_b.show();   // records size → child_sizes[1]
})
// After closure: layout engine computes target positions
// Then translates all child draw commands to those positions
```

### Layout Modes
| Mode | Behavior |
|---|---|
| `Column` | Stack children vertically (default) |
| `Row` | Stack children horizontally |
| `Row + Wrap::Wrap` | Wrap into multiple rows (like CSS flexbox wrap) |

### Alignment
`halign` and `valign` accept `Align::Left/Center/Right/Top/Bottom/SpaceBetween/SpaceAround`.

---

## State Management

Zenthra persists state across frames via hash maps keyed by widget `Id`:

| Map | Stores |
|---|---|
| `scroll_state` | `(scroll_x, scroll_y)` per scrollable container |
| `cursor_state` | Text cursor position per input widget |
| `interaction_state` | Hover/animation timers per interactive widget |
| `layout_cache` | Bounding rect per widget (for hit-testing) |

IDs are generated deterministically via `std::hash::DefaultHasher` from a user-supplied key (string, integer, etc.) combined with the parent container's ID.

---

## LazyContainer — Virtualization

`LazyContainer` implements its own scroll and rendering pipeline, **bypassing the layout engine entirely**. This is the key to its performance.

```
LazyContainer.show()
  ├── Compute scroll_y from mouse wheel events
  ├── Calculate visible index range [start_idx, end_idx]
  ├── Save parent layout state
  ├── For each visible item i in [start_idx, end_idx]:
  │     ├── Compute absolute position: (ox + padding + col*row_w, oy + padding + row*row_h - scroll_y)
  │     ├── Run f(ui, i) — item renders itself at that position
  │     └── Clip all draw commands from this item to the viewport rect
  ├── Restore parent layout state
  └── Flush: background → items → scrollbar → ui.advance()
```

Because items are placed at absolute screen coordinates and the layout engine is never invoked, there is no re-positioning step that could corrupt the virtual scroll position.

**Complexity:** O(visible items) per frame, regardless of total count.

---

## Clip Rects

Every draw command carries its own `clip_rect: [f32; 4]`. The GPU shader discards any fragment outside this rect. This is how scrollable containers hide overflow — before flushing child draws to the parent, each draw command's clip rect is set to the container's viewport.

---

## Text System

Text rendering uses [cosmic-text](https://github.com/pop-os/cosmic-text) for full Unicode shaping, bidirectional text, and font fallback. Shaped glyph runs are uploaded to a glyph atlas texture and rendered via a dedicated text shader.
