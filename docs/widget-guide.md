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
| `.scroll_y(bool)` | Toggle vertical scrolling |
| `.scroll_x(bool)` | Toggle horizontal scrolling |
| `.scroll(bool)` | Toggle scrolling on both axes |

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

Single-line text input with auto-scrolling, cursor blink, and unified box/text styling.

```rust
ui.input(&mut my_string)
    .width(300.0)
    .padding(8.0, 12.0, 8.0, 12.0)
    .radius_all(6.0)
    .bg(Color::rgb(0.15, 0.15, 0.18))
    .border((Color::rgb(0.3, 0.3, 0.35), 1.0))
    .focus_border((Color::rgb(0.4, 0.7, 1.0), 1.5))
    .placeholder("Type here...")
    .show();
```

| Method | Description |
|---|---|
| `.style(BoxStyle)` | Reusable BoxStyle theme |
| `.text_style(TextStyle)` | Reusable TextStyle typography |
| `.width(f32)` / `.fill_x()` | Sizing |
| `.padding(...)` | Outer box padding |
| `.text_padding(...)` | Inner text padding |
| `.bg(Color)` | Surface color |
| `.border(Color, f32)` | Unfocused border |
| `.focus_border(Color, f32)` | Focused border ring |
| `.radius(f32)` / `.radius_all(f32)` | Corner radii |
| `.placeholder(str)` | Hint text when empty |
| `.bold()`, `.italic()`, `.monospace()` | Typography shortcuts |

---

## TextArea

Multi-line text editor with auto-wrap, vertical scrolling, and unified styling.

```rust
ui.text_area(&mut my_string, "editor_id")
    .width(400.0)
    .max_height(250.0) // Auto-grows up to 250px, then scrolls
    .padding(10.0, 12.0, 10.0, 12.0)
    .radius_all(8.0)
    .bg(Color::rgb(0.12, 0.12, 0.14))
    .border((Color::rgb(0.25, 0.25, 0.3), 1.0))
    .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
    .monospace()
    .placeholder("Enter code or notes...")
    .show();
```

| Method | Description |
|---|---|
| `.style(BoxStyle)` | Reusable BoxStyle theme |
| `.text_style(TextStyle)` | Reusable TextStyle typography |
| `.width(f32)` / `.fill_x()` | Width |
| `.height(f32)` / `.max_height(f32)` | Fixed height or auto-grow ceiling |
| `.scrollable(bool)` | 2D mouse wheel scrolling |
| `.wrap(TextWrap)` / `.auto_wrap()` | Word wrapping |
| `.monospace()`, `.bold()`, `.italic()` | Font typography |
| `.submit_on_enter(bool)` | Submit instead of newline |
| `.border(...)` / `.focus_border(...)` | Borders and focus ring |

---

## Checkbox

Boolean selection control with animated checkmark glyph, typography, and unified box styling.

```rust
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

| Method | Description |
|---|---|
| `.style(BoxStyle)` | Reusable BoxStyle for checkbox indicator box |
| `.text_style(TextStyle)` | Reusable TextStyle for label |
| `.size(f32)` | Width and height of checkbox box |
| `.gap(f32)` | Space between box and label |
| `.bg(Color)` | Unchecked box background |
| `.border(Color, f32)` | Unchecked box border |
| `.check_bg(Color)` | Checked box background color |
| `.check_color(Color)` | Checkmark glyph color |
| `.radius(f32)` / `.radius_all(f32)` | Corner radii |
| `.label_size(f32)` / `.font_size(f32)` | Label typography size |
| `.label_color(Color)` / `.text_color(Color)` | Label text color |
| `.active_label(Color)` | Label color transition when checked |
| `.glow(bool)` | Accent glow aura when checked |
| `.shadow(Color, x, y, blur)` | Box drop shadow |

---

## Toggle

Interactive sliding toggle switch with customizable track box, thumb, and optional external/inner labels.

```rust
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

| Method | Description |
|---|---|
| `.style(BoxStyle)` | Reusable BoxStyle for track box |
| `.text_style(TextStyle)` | Reusable TextStyle for external label |
| `.size(w, h)` / `.width(w)` / `.height(h)` | Track dimensions |
| `.padding(f32)` / `.padding_all(f32)` | Spacing between track and thumb |
| `.pill()` / `.square()` | Track shape presets |
| `.radius(f32)` / `.radius_all(f32)` | Custom corner radii |
| `.colors(on, off, thumb)` | Convenience setter for track & thumb colors |
| `.on_color(Color)` / `.off_color(Color)` | Individual track state colors |
| `.thumb_color(Color)` / `.thumb_radius(f32)` | Thumb knob styling |
| `.inner_labels(on, off)` | Inner track text ("ON" / "OFF") |
| `.label_left()` / `.label_right()` | External label placement side |
| `.label_size(f32)` / `.label_color(Color)` | External label typography |
| `.glow(bool)` | Track accent glow when active |
| `.shadow(Color, x, y, blur)` | Track drop shadow |
| `.disabled(bool)` | Interactive state disablement |

---

## Radio

Single-selection option control for mutually exclusive values with animated dot scaling, typography, and unified box styling.

```rust
ui.radio(&mut selected_theme, 0, "Dark Mode")
    .size(18.0)
    .gap(10.0)
    .colors(Color::rgb(0.4, 0.4, 0.4), Color::rgb(0.2, 0.5, 1.0))
    .show();
```

| Method | Description |
|---|---|
| `.style(BoxStyle)` | Reusable BoxStyle for the radio ring |
| `.text_style(TextStyle)` | Reusable TextStyle for label typography |
| `.size(f32)` | Width and height of radio ring |
| `.gap(f32)` | Spacing between ring and label |
| `.colors(ring, dot)` | Convenience setter for ring and dot colors |
| `.ring_color(Color)` | Inactive ring border color |
| `.dot_color(Color)` | Active dot fill and ring highlight color |
| `.bg(Color)` | Radio ring interior background |
| `.shadow(Color, x, y, blur)` | Ring drop shadow |
| `.label_size(f32)` / `.font_size(f32)` | Label font size |
| `.label_color(Color)` / `.text_color(Color)` | Label text color |
| `.active_label(Color)` | Label color transition when selected |
| `.glow(bool)` | Neon accent aura when selected |
| `.disabled(bool)` | Disable interactions |
