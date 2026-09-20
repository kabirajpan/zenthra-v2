# Zenthra

[![crates.io](https://img.shields.io/crates/v/zenthra.svg)](https://crates.io/crates/zenthra)
[![docs.rs](https://img.shields.io/docsrs/zenthra)](https://docs.rs/zenthra)
[![license](https://img.shields.io/crates/l/zenthra.svg)](LICENSE)
[![downloads](https://img.shields.io/crates/d/zenthra.svg)](https://crates.io/crates/zenthra)

A high-performance, immediate-mode UI framework written in Rust. Zenthra is built from scratch with a custom GPU render pipeline, a flexible layout engine, and a widget system designed for speed and ergonomics.

🌐 **Website:** [zenthralabs.dev](https://zenthralabs.dev)
📖 **Documentation:** [zenthralabs.dev/products/zenthra/docs/](https://zenthralabs.dev/products/zenthra/docs/)

---

## Why Zenthra?

Most Rust GUI crates make you choose between ease-of-use (immediate-mode, like `egui`) and raw control (retained/reactive, like `iced`). Zenthra keeps the immediate-mode ergonomics but adds things you'd otherwise have to build yourself:

- **Virtualized lists out of the box** — `LazyContainer` renders only visible rows, so a list of a million items still runs at 60 FPS without you writing custom windowing logic.
- **A real GPU pipeline, not just a backend** — rect, text, and overlay draw commands go straight to WGPU, with built-in Kawase blur and glassmorphism effects rather than hand-rolled shaders.
- **Bring your own shaders** — register custom WGSL fragment shaders on any layout container.
- **You choose the render loop** — event-driven by default (sleeps until input, saves battery), or continuous when you're animating.

If you're building something visually heavy — editors, media tools, dashboards — that's the gap Zenthra is trying to fill.

---

## Showcase

Here are some native applications built entirely with the Zenthra framework:

### Zenthra View (Native Image Viewer)
![Zenthra View](https://raw.githubusercontent.com/kabirajpan/zenthra-v2/main/assets/zenthra_view.jpeg)

### ZenFile (Native File Manager)
<img src="https://raw.githubusercontent.com/kabirajpan/zenthra-v2/main/assets/zenfile.png" width="100%" />
<img src="https://raw.githubusercontent.com/kabirajpan/zenthra-v2/main/assets/zenfile_glassmorphis.png" width="49%" /> <img src="https://raw.githubusercontent.com/kabirajpan/zenthra-v2/main/assets/zenfile_glassmorphism2.png" width="49%" />

---

## Features

- **GPU-Accelerated Rendering** — Rect, text, and overlay draw commands sent directly to the GPU via WGPU.
- **Immediate-Mode API** — Rebuilds the UI each frame for instant state transitions and zero-synchronization overhead.
- **Flexible Layout Engine** — Row, Column, and Wrap layouts with alignment, padding, and gap support.
- **High-Performance Virtualization** — `LazyContainer` renders only visible items, handling millions of rows at a constant 60 FPS.
- **Premium Glassmorphism** — Built-in hardware-accelerated Dual-Pass Kawase blur, frosted grain overlays, and beveled specular edge highlights.
- **Extensible WGSL Shaders** — Dynamically load and run developer-registered fragment shaders on layout containers with ease.
- **Scrollable Containers** — Out-of-the-box scrollbar drag, mouse wheel, and trackpad gestures.
- **Text System** — Powered by [cosmic-text](https://github.com/pop-os/cosmic-text) with full shaping, bidirectional text, and font fallback.
- **Input Widgets** — Single-line `Input` and multi-line `TextArea` with cursor tracking, selection, and virtual scrolling.

---

## Understanding the Paradigm

### Immediate-Mode API
Zenthra uses an **Immediate-Mode** programming model (similar to `egui` or `Dear ImGui`). Instead of maintaining a persistent, memory-heavy tree of widgets (like the HTML DOM or traditional desktop toolkits), your application code describes the entire UI layout and behavior *every frame* in line with execution. This eliminates state-synchronization bugs and keeps your code simple and readable.

### Event-Driven vs. Continuous Rendering
While Zenthra's API is immediate-mode, its rendering loop is highly optimized:
- **Event-Driven (Default):** To save CPU/GPU resources and battery life, Zenthra sleeps when there is no activity. It only runs your UI functions and redraws the window when it receives a system event (e.g. mouse movement, clicks, keyboard input, or window resizing).
- **Continuous:** If you are running animations, transitions, or games, Zenthra can dynamically switch to continuous rendering to redraw the screen at your monitor's full refresh rate (60+ FPS).

---

## Quick Start

Add `zenthra` to your dependencies in `Cargo.toml`:

```toml
[dependencies]
zenthra = "0.2"
```

Create a minimal application in `src/main.rs`:

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

Run the built-in examples to see widgets and layouts in action:

```bash
# Basic window
cargo run --example hello

# Glassmorphism & layout tab panels
cargo run --example glassmorphism_demo

# Custom WGSL post-processing shaders
cargo run --example custom_shader_demo

# Standard containers
cargo run --example containers
```

---

## Widgets

| Widget | Description |
|---|---|
| `container()` | Layout box with padding, background, corner radius, scrolling, and custom shaders |
| `lazy_container()` | Virtualized container — only renders visible items |
| `text()` | Styled text with font size, color, and paragraph alignment |
| `button()` | Interactive button with hover/active states |
| `input()` | Single-line text input field |
| `text_area()` | Multi-line text editor |

Full API reference can be found in [`docs/widget-guide.md`](docs/widget-guide.md).

---

## Architecture

See [`docs/architecture.md`](docs/architecture.md) for a detailed overview of the layout lifecycle, rendering pipelines, and state management.

---

## Platforms

Built on `winit` + `wgpu`, so Zenthra targets Linux, Windows, and macOS out of the box, with Android support via `android-activity`. Linux is the primary development target — file an issue if you hit platform-specific problems elsewhere.

---

## Contributing

Issues and pull requests are welcome — whether it's a bug fix, a new widget, or an example app. Check open issues on [GitHub](https://github.com/kabirajpan/zenthra-v2) for a place to start.

If you find Zenthra useful, a ⭐ on the repo goes a long way toward other developers discovering it.

---

## License

MIT — see [LICENSE](LICENSE).
