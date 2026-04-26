# Zenthra

A high-performance, immediate-mode UI framework written in Rust. Zenthra is built from scratch with a custom GPU render pipeline, a flexible layout engine, and a widget system designed for speed and ergonomics.

---

## Features

- **GPU-accelerated rendering** — Rect, text, and overlay draw commands sent directly to the GPU via WGPU
- **Immediate-mode API** — no retained widget trees, rebuild UI each frame
- **Flexible layout engine** — Row, Column, and Wrap layouts with alignment, padding, and gap support
- **High-performance virtualization** — `LazyContainer` renders only visible items, handling millions of rows smoothly
- **Scrollable containers** — built-in scroll state, drag scrollbars, and wheel events
- **Text system** — powered by [cosmic-text](https://github.com/pop-os/cosmic-text) with full shaping support
- **Input widgets** — single-line `Input` and multi-line `TextArea` with cursor, selection, and scrolling
- **Theming** — composable styling via builder methods on every widget

---

## Quick Start

```rust
use zenthra::prelude::*;

fn main() {
    App::new()
        .title("My App")
        .size(800, 600)
        .with_ui(|ui| {
            ui.text("Hello, Zenthra!")
                .size(32.0)
                .color(Color::WHITE)
                .show();
        })
        .run();
}
```

---

## Examples

```bash
cargo run --example hello
cargo run --example containers
cargo run --example text
cargo run --example edit
```

---

## Widgets

| Widget | Description |
|---|---|
| `container()` | Layout box with padding, bg, radius, scroll, row/column/wrap |
| `lazy_container()` | Virtualized container — only renders visible items |
| `text()` | Styled text with font size, color, alignment |
| `button()` | Interactive button with hover/active states |
| `input()` | Single-line text input |
| `text_area()` | Multi-line text editor |

See [`docs/widget-guide.md`](docs/widget-guide.md) for full API reference.

---

## Architecture

See [`docs/architecture.md`](docs/architecture.md) for an overview of the rendering pipeline, layout engine, and state management.

---

## License

Apache 2.0 — see [LICENSE](LICENSE).
