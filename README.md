# Zenthra

[![crates.io](https://img.shields.io/crates/v/zenthra.svg)](https://crates.io/crates/zenthra)
[![docs.rs](https://img.shields.io/docsrs/zenthra)](https://docs.rs/zenthra)
[![CI](https://github.com/kabirajpan/zenthra-v2/actions/workflows/ci.yml/badge.svg)](https://github.com/kabirajpan/zenthra-v2/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/zenthra.svg)](LICENSE)
[![downloads](https://img.shields.io/crates/d/zenthra.svg)](https://crates.io/crates/zenthra)

Immediate-mode UI for native Rust apps. Zenthra owns the GPU pipeline (wgpu), layout, widgets, and a fine-grained reactive state layer so you can ship editors, dashboards, and media tools without assembling those pieces yourself.

- **Website:** [zenthralabs.dev](https://zenthralabs.dev)
- **Docs:** [zenthralabs.dev/products/zenthra/docs](https://zenthralabs.dev/products/zenthra/docs/)
- **API reference:** [docs.rs/zenthra](https://docs.rs/zenthra)
- **Crate:** [`zenthra` 0.2.x](https://crates.io/crates/zenthra)

Linux, Windows, and macOS are supported through `winit` + `wgpu`. Linux is the primary development target. Android is available via `android-activity` and is still early.

---

## Why Zenthra?

Most Rust GUI crates ask you to pick either immediate-mode ergonomics (`egui`) or a retained/reactive tree (`iced`). Zenthra keeps an immediate-mode `with_ui` loop and still gives you:

- **Virtualized lists** — `lazy_container()` draws only visible rows, so large lists stay at a stable frame cost.
- **A first-class GPU pipeline** — rects, text, images, and overlays go to wgpu. Dual-pass Kawase blur and glassmorphism are built in.
- **Custom WGSL** — register fragment shaders and attach them to layout containers.
- **Reactive state when you need it** — `Signal`, `Computed`, `Effect`, and context, without leaving the immediate-mode API.
- **A render loop you control** — event-driven by default (idle until input), `RenderMode::Continuous` when you are animating.

---

## Showcase

Native apps built with Zenthra:

### Zenthra View (image viewer)

![Zenthra View](https://raw.githubusercontent.com/kabirajpan/zenthra-v2/main/assets/zenthra_view.jpeg)

### ZenFile (file manager)

<img src="https://raw.githubusercontent.com/kabirajpan/zenthra-v2/main/assets/zenfile.png" width="100%" />
<img src="https://raw.githubusercontent.com/kabirajpan/zenthra-v2/main/assets/zenfile_glassmorphis.png" width="49%" /> <img src="https://raw.githubusercontent.com/kabirajpan/zenthra-v2/main/assets/zenfile_glassmorphism2.png" width="49%" />

---

## Features

- **GPU rendering** — batched rect, text, image, and overlay commands through wgpu (SDF rounded rects, shadows, clip rects).
- **Immediate-mode UI** — describe the frame in a closure; no persistent widget tree to keep in sync.
- **Layout** — row, column, wrap, alignment, padding, gap, fill, and stacking (`stack()`, `card()`, `panel()`).
- **Virtualization** — `LazyContainer` is O(visible items) per frame.
- **Glassmorphism** — Kawase blur, frosted grain, specular edges; optional window transparency and backdrop filters.
- **Custom shaders** — `App::register_custom_shader` plus per-container shader binding.
- **Scrolling** — scrollbar drag, mouse wheel, and trackpad on containers.
- **Text** — [cosmic-text](https://github.com/pop-os/cosmic-text) shaping, bidi, and font fallback; `Input` and `TextArea` with selection and cursor.
- **Reactive state** — signals, computed values, effects, batched updates, thread-safe `ArcSignal`, and `provide_context` / `use_context`.

---

## Quick start

Requires a recent **stable Rust** toolchain (`rustup update stable`).

```toml
[dependencies]
zenthra = "0.2"
```

```rust
use zenthra::prelude::*;

fn main() {
    App::new()
        .title("My App")
        .size(800, 600)
        .with_ui(|ui| {
            ui.container()
                .fill()
                .bg(Color::rgb(0.05, 0.05, 0.08))
                .valign(Align::Center)
                .halign(Align::Center)
                .show(|ui| {
                    ui.text("Hello, Zenthra!")
                        .size(32.0)
                        .color(Color::WHITE)
                        .show();
                });
        })
        .run();
}
```

Useful `App` builders: `.title()`, `.size()`, `.decorations()`, `.transparent()`, `.blur()`, `.bg()`, `.backdrop_filter()`, `.load_font_path()`, `.register_custom_shader()`.

---

## Immediate mode and the render loop

Zenthra is immediate-mode: each frame your `with_ui` closure describes layout and behavior. Transient interaction (scroll, cursor, hover) is stored in id-keyed maps, not in a retained widget tree.

The **loop** is separate from the **API**:

| Mode | When it runs |
|---|---|
| **Static / event-driven (default)** | Sleeps until mouse, keyboard, resize, or a state-change redraw. Saves CPU and battery. |
| **Continuous** | Set `RenderMode::Continuous` on a container (or inherit it) for animations and live previews at display refresh. |

See `cargo run -p zenthra --example render_modes`.

---

## Reactive state

Use signals for app data; the UI still rebuilds every frame.

```rust
use zenthra::prelude::*;

fn main() {
    let count = Signal::new(0);
    let doubled = Computed::new({
        let count = count.clone();
        move || count.get() * 2
    });

    App::new()
        .title("Counter")
        .size(400, 240)
        .with_ui(move |ui| {
            ui.column().padding_all(24.0).gap(12.0).show(|ui| {
                ui.text(&format!("count = {}, doubled = {}", count.get(), doubled.get()))
                    .show();
                if ui.button("Increment").show().clicked {
                    count.update(|n| n + 1);
                }
            });
        })
        .run();
}
```

Also available: `Effect`, `ArcSignal` (for worker threads), `batch()`, `on_state_change()`, and context (`provide_context` / `use_context`). Full write-up: [`docs/architecture.md`](docs/architecture.md). Demo: `reactive_state_demo`.

---

## Widgets

| Widget | Role |
|---|---|
| `container()` / `row()` / `column()` | Layout box: padding, fill, scroll, shaders, render mode |
| `lazy_container()` | Virtualized list/grid (fixed item size) |
| `stack()` | Overlay children |
| `card()` / `panel()` | Structured surfaces |
| `text()` / `h1()` | Styled text |
| `button()` | Click / hover |
| `input()` | Single-line field |
| `text_area()` | Multi-line editor |
| `slider()` | Numeric drag |
| `progress_bar()` | Determinate progress |
| `checkbox()` / `toggle()` / `radio()` | Selection |
| `dropdown()` | Options list |
| `image()` | GPU image (`ImageSource`) |
| `window()` | Floating window |
| `menu_bar()` / `menu()` / `menu_item()` | Menus |

Widget API notes: [`docs/widget-guide.md`](docs/widget-guide.md) (core widgets). Per-widget drafts live under [`widgets/`](widgets/).

---

## Examples

Examples are registered on the `zenthra` crate. From the **workspace root**:

```bash
cargo run -p zenthra --example hello
cargo run -p zenthra --example containers
cargo run -p zenthra --example glassmorphism_demo
cargo run -p zenthra --example custom_shader_demo
cargo run -p zenthra --example reactive_state_demo
cargo run -p zenthra --example render_modes
```

| Example | What it shows |
|---|---|
| `hello` | Minimal window and text |
| `containers` / `container_test` | Layout, padding, nesting |
| `nested_wrap` | Flex wrap |
| `text` / `edit` | Typography and text editing |
| `button` / `button_default` | Buttons |
| `slider_test` / `progress_test` | Slider and progress |
| `checkbox_test` / `toggle_list` / `radio_list` | Controls |
| `dropdown_test` / `menu_test` | Dropdowns and menus |
| `image_simple` / `image_test` | Images |
| `scroll_test` | Scrolling |
| `card_test` / `panel_test` / `stack_test` / `sidebar_test` | Composite layout |
| `window_test` / `floating_window` / `simple_window` | Windows |
| `glassmorphism_demo` | Blur / glass |
| `custom_shader_demo` | Custom WGSL |
| `reactive_state_demo` | Signals and context |
| `render_modes` | Static vs continuous |
| `color_test` / `declarative_event_test` | Color and events |

---

## Build from source

```bash
git clone https://github.com/kabirajpan/zenthra-v2.git
cd zenthra-v2
cargo test --workspace --exclude window_test_mobile
cargo run -p zenthra --example hello
```

**Linux** (Debian/Ubuntu) packages used in CI:

```bash
sudo apt-get install -y pkg-config libx11-dev libx11-xcb-dev \
  libxkbcommon-dev libwayland-dev libegl1-mesa-dev libgbm-dev
```

The workspace is a set of crates behind a single public API (`zenthra`):

| Crate | Responsibility |
|---|---|
| `zenthra` | Facade: `App`, `prelude`, examples |
| `zenthra-core` | Colors, geometry, ids, render modes |
| `zenthra-layout` | Layout (Taffy) |
| `zenthra-render` | wgpu pipelines, blur, images |
| `zenthra-widgets` | Widget builders |
| `zenthra-state` | Signals and context |
| `zenthra-text` | Shaping and glyph atlas |
| `zenthra-input` | Pointer and keyboard |
| `zenthra-platform` | winit event loop / window |
| `zenthra-theme` | Theme types |
| `zenthra-animation` | Animation helpers |

How a frame is built and flushed: [`docs/architecture.md`](docs/architecture.md).

---

## Status

Zenthra is **0.2** — usable for real apps (see the showcase) but the API can still change. File platform bugs on [GitHub Issues](https://github.com/kabirajpan/zenthra-v2/issues).

---

## Name

**Zenthra** is the name of this project (crates.io `zenthra`, this repo, [zenthralabs.dev](https://zenthralabs.dev)). You can use the framework in your own apps and mention that they are built with Zenthra. Do not name a fork, competing crate, or company after Zenthra, and do not present a copy of this engine as a product you originated. Keep the MIT copyright notice in any copy of the source, as the license requires.

---

## Contributing

Bug reports, widgets, and examples are welcome. Open an issue or PR on [github.com/kabirajpan/zenthra-v2](https://github.com/kabirajpan/zenthra-v2).

---

## License

[MIT](LICENSE) — Copyright (c) 2026 kabirajpan.

You may use, modify, and ship Zenthra in your applications. Copies of the source must keep the copyright and permission notice.
