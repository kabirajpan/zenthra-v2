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

## State Management & Reactivity

Zenthra combines immediate-mode UI rendering with a fine-grained reactive state engine (`zenthra-state`), bringing SolidJS/Leptos-style automatic dependency tracking and zero-overhead data binding to desktop Rust applications.

### 1. Reactive Signals (`Signal<T>`)
```rust
use zenthra::prelude::*;

let count = Signal::new(0);

// Reading (registers dependencies automatically)
let current = count.get();
count.with(|val| println!("Value: {val}"));

// Writing (notifies subscribers & triggers redraw hook)
count.set(10);
count.update(|prev| prev + 1);
count.with_mut(|val| *val += 5);

// Change detection (skips redraw and notifications if value == current)
count.set_if_changed(20);
```

### 2. Derived State (`Computed<T>`)
Derives reactive values from one or more signals, re-evaluating automatically when dependencies change:
```rust
let first = Signal::new("Ada".to_string());
let last = Signal::new("Lovelace".to_string());

let full_name = Computed::new({
    let first = first.clone();
    let last = last.clone();
    move || format!("{} {}", first.get(), last.get())
});
```

### 3. Reactive Effects (`Effect`)
Executes a closure immediately and re-executes whenever any accessed signals change. Subscriptions are automatically cleaned up when the `Effect` handle is dropped (RAII):
```rust
let _effect = Effect::run({
    let count = count.clone();
    move || log::info!("Count is: {}", count.get())
});

// Or detach for the application lifespan:
// _effect.forget();
```

### 4. Cross-Thread Signals (`ArcSignal<T>`)
Thread-safe signal (`Send + Sync`) for background worker threads, network polling, and asynchronous file I/O:
```rust
let progress = ArcSignal::new(0.0f32);
let worker_clone = progress.clone();

std::thread::spawn(move || {
    worker_clone.set(1.0); // Safely notifies UI and triggers redraw hook
});
```

### 5. Batched Updates & Redraw Hooks
```rust
// Hook state changes directly into window redraw requests
on_state_change(|| ui.request_redraw());

// Batch multiple mutations into a single subscriber notification
batch(|| {
    x.set(10.0);
    y.set(20.0);
});
```

### 6. Context Provider API (Dependency Injection)
Thread-local context container to share global data (themes, settings, sessions) across the widget tree without prop drilling:
```rust
provide_context(ThemeColors::default());

// Anywhere in the widget tree:
if let Some(theme) = use_context::<ThemeColors>() {
    ui.button("Submit").bg(theme.accent).show();
}

assert!(has_context::<ThemeColors>());
remove_context::<ThemeColors>();
```

### 7. Internal Immediate-Mode State (Widget `Id` Maps)
In addition to application reactivity, Zenthra persists transient widget interaction parameters across frames via deterministic hash maps keyed by widget `Id`:

| Map | Stores |
|---|---|
| `scroll_state` | `(scroll_x, scroll_y)` per scrollable container |
| `cursor_state` | Text cursor position and buffer per input widget |
| `interaction_state` | Hover/animation timers per interactive widget |
| `layout_cache` | Bounding rect per widget (for hit-testing) |

IDs are generated deterministically via `std::hash::DefaultHasher` from a user-supplied key combined with the parent container's ID.

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
