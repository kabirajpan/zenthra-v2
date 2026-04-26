# Widget Guide

Complete API reference for all Zenthra widgets.

---

## Container

A general-purpose layout box. Children stack in a `Column` by default.

```rust
ui.container()
    .width(400.0)
    .height(300.0)
    .bg(Color::rgb(0.1, 0.1, 0.15))
    .radius(12.0)
    .padding(16.0)
    .gap(8.0)
    .show(|ui| {
        // children go here
    });
```

### Layout methods
| Method | Description |
|---|---|
| `.row()` | Lay children left-to-right |
| `.column()` | Lay children top-to-bottom (default) |
| `.wrap(Wrap::Wrap)` | Wrap items to next line when row is full |
| `.halign(Align::Center)` | Horizontal alignment of children |
| `.valign(Align::Center)` | Vertical alignment of children |
| `.gap(f32)` | Space between children |

### Sizing methods
| Method | Description |
|---|---|
| `.width(f32)` | Fixed width |
| `.height(f32)` | Fixed height |
| `.fill_x()` | Fill available width |
| `.fill_y()` | Fill available height |
| `.fill()` | Fill both axes |
| `.max_width(f32)` | Maximum width |
| `.min_width(f32)` | Minimum width |

### Styling methods
| Method | Description |
|---|---|
| `.bg(Color)` | Background color |
| `.radius(f32)` | Corner radius |
| `.border(Color, f32)` | Border color and width |
| `.shadow(f32)` | Shadow blur amount |
| `.padding(f32)` | Uniform padding |
| `.padding_x(f32)` | Horizontal padding |
| `.padding_y(f32)` | Vertical padding |
| `.opacity(f32)` | Transparency (0.0–1.0) |

### Scroll methods
| Method | Description |
|---|---|
| `.scroll_y(true)` | Enable vertical scrolling |
| `.scroll_x(true)` | Enable horizontal scrolling |
| `.scrollable(x, y)` | Enable both axes at once |

---

## LazyContainer

A high-performance virtualized container. Renders **only visible items** regardless of total count. Ideal for lists, grids, and feeds with large datasets.

```rust
ui.lazy_container()
    .id("my_list")
    .bg(Color::rgb(0.05, 0.05, 0.07))
    .padding(10.0)
    .item_size(150.0, 100.0)
    .gap(15.0)
    .row()
    .wrap(Wrap::Wrap)
    .count(10_000)
    .show(|ui, index| {
        ui.container()
            .id(index)
            .width(150.0)
            .height(100.0)
            .bg(Color::rgb(0.2, 0.2, 0.3))
            .radius(4.0)
            .show(|ui| {
                ui.text(&format!("Item {}", index + 1))
                    .color(Color::WHITE)
                    .show();
            });
    });
```

### Methods
| Method | Description |
|---|---|
| `.id(key)` | Stable scroll-state identity |
| `.count(usize)` | Total number of items |
| `.item_size(w, h)` | Fixed width and height per item |
| `.gap(f32)` | Space between items |
| `.padding(f32)` | Padding inside the container |
| `.bg(Color)` | Background color |
| `.radius(f32)` | Corner radius |
| `.row()` | Horizontal item flow |
| `.column()` | Vertical item flow (default) |
| `.wrap(Wrap::Wrap)` | Wrap into a grid (must combine with `.row()`) |

> **Note:** All items must have the same fixed size (set via `.item_size()`). Variable-height virtualization is not yet supported.

---

## Text

Styled text rendering.

```rust
ui.text("Hello World")
    .size(24.0)
    .color(Color::WHITE)
    .bold()
    .align_center()
    .show();
```

| Method | Description |
|---|---|
| `.size(f32)` | Font size in logical pixels |
| `.color(Color)` | Text color |
| `.bold()` | Bold weight |
| `.italic()` | Italic style |
| `.align_center()` | Center align text |
| `.align_right()` | Right align text |
| `.width(f32)` | Max width for wrapping |

---

## Button

Interactive button with hover and click states.

```rust
let clicked = ui.button("Click Me")
    .bg(Color::rgb(0.2, 0.4, 0.9))
    .radius(8.0)
    .padding(12.0)
    .show();

if clicked {
    // handle click
}
```

---

## Input

Single-line text input.

```rust
ui.input(&mut my_string)
    .width(300.0)
    .placeholder("Type here...")
    .show();
```

---

## TextArea

Multi-line text editor.

```rust
ui.text_area(&mut my_string)
    .width(400.0)
    .height(200.0)
    .show();
```
