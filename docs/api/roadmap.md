# Zenthra UI Styling Unification Roadmap

This document tracks the progress, completed work, architecture decisions, and remaining roadmap for the **Unified Styling & Theming System** across Zenthra UI (`BoxStyle`, `TextStyle`, `BoxStyleExt`, and builder macros).

---

## 🎯 High-Level Goal

Unify all widgets and controls in `zenthra-widgets` to use:
1. **`BoxStyle`** for container geometry, backgrounds (solid & gradients), borders, corner radii, padding, shadows, and backdrop blur.
2. **`TextStyle`** for typography, font family, font size, text colors, weights, line heights, and wrapping.
3. **`BoxStyleExt`** GPU rendering primitives (`render_box_styled`, `render_box_styled_interactive`).
4. **Declarative Macros** in `macros.rs` (`impl_dimensions!`, `impl_padding!`, `impl_radius!`, `impl_visuals!`, `impl_box_style_methods!`, `impl_text_style_methods!`).

---

## 📊 Summary Status

| Category | Widget / Module | Status | Core Type | Notes |
| :--- | :--- | :---: | :--- | :--- |
| **Core Architecture** | `BoxStyle` (`zenthra-core`) | ✅ Complete | Struct | Unified box geometry, colors, borders, shadows |
| **Core Architecture** | `TextStyle` (`zenthra-text`) | ✅ Complete | Struct | Unified typography engine |
| **Core Architecture** | `BoxStyleExt` (`zenthra-widgets`) | ✅ Complete | Trait | GPU pipeline adapter for styled boxes |
| **Core Architecture** | `macros.rs` (`zenthra-widgets`) | ✅ Complete | Macros | Lean single-source-of-truth builder macros |
| **Core Widgets** | `Container` | ✅ Complete | `BoxStyle` | Full box styling, gradients, shadows, flex |
| **Core Widgets** | `Button` | ✅ Complete | `BoxStyle` + `TextStyle` | Interactive states, typography, icons |
| **Core Widgets** | `Text` | ✅ Complete | `TextStyle` | Rich typography, wrapping, colors |
| **Form Inputs** | `Input` | ✅ Complete | `BoxStyle` + `TextStyle` | Focus border, text padding, selection highlight |
| **Form Inputs** | `TextArea` | ✅ Complete | `BoxStyle` + `TextStyle` | Multiline, scrollable, focus border, styling |
| **Form Controls** | `Checkbox` | ✅ Complete | `BoxStyle` + `TextStyle` | Box styling, animated checkmark, label typography |
| **Form Controls** | `Toggle` | ✅ Complete | `BoxStyle` + `TextStyle` | Pill track box styling, smooth knob, label typography |
| **Form Controls** | `Radio` | ✅ Complete | `BoxStyle` + `TextStyle` | Circle box styling, inner dot, label typography |
| **Numeric & Ranges** | `Slider` | ⏳ **Next Up** | `BoxStyle` | Track & thumb box styling unification |
| **Numeric & Ranges** | `ProgressBar` | ⏳ **Next Up** | `BoxStyle` | Track & fill box styling unification |
| **Form Controls** | `Dropdown` | 📋 Scheduled | `BoxStyle` + `TextStyle` | Header box, item list, selection styling |
| **Form Controls** | `DragValue` | 📋 Scheduled | `BoxStyle` + `TextStyle` | Interactive numeric drag field |
| **Containers** | `Card` | 📋 Scheduled | `BoxStyle` | High-level card container |
| **Containers** | `Panel` | 📋 Scheduled | `BoxStyle` | Dockable / sectional panel |
| **Overlays** | `Menu` | 📋 Scheduled | `BoxStyle` + `TextStyle` | Menu bar, popup items |
| **Overlays** | `Popover` | 📋 Scheduled | `BoxStyle` | Popover floating box |
| **Layout & Media** | `Stack` | 📋 Scheduled | `BoxStyle` | Layered absolute / z-index stack |
| **Layout & Media** | `Image` | 📋 Scheduled | `BoxStyle` | Border, radius, shadow for image elements |

---

## ✅ Completed in Detail

### 1. Unified Core & Macro Architecture
* **`crates/zenthra-widgets/src/style_ext.rs`**:
  * Implements `BoxStyleExt` with GPU-accelerated drawing for backgrounds (solid + linear gradients), borders, corner radii, and drop shadows.
* **`crates/zenthra-widgets/src/macros.rs`**:
  * Deduplicated and kept lean (~470 lines).
  * Exposes:
    * `impl_dimensions!($builder)`
    * `impl_padding!($builder)`
    * `impl_radius!($builder)`
    * `impl_visuals!($builder)`
    * `impl_box_style_methods!($builder)`
    * `impl_text_style_methods!($builder)`
  * All macros cleanly support 2-lifetime (`'u, 'a`) and 3-lifetime (`'u, 'a, 'b`) builders without redundant declarations.

### 2. Migrated Widgets
* **`Container`**: Uses `self.style: BoxStyle` + `impl_box_style_methods!`.
* **`Button`**: Uses `self.style: BoxStyle` + `self.text_style: TextStyle` + both macros.
* **`Text`**: Uses `self.style: TextStyle` + `impl_text_style_methods!`.
* **`Input`**: Single-line text input with unified box styling, `focus_border`, placeholder styling, and cursor selection highlight.
* **`TextArea`**: Multi-line editor with unified box styling, scrolling, text padding, `focus_border`, and wrapping.
* **`Checkbox`**: Checkbox button and optional label migrated to `BoxStyle` and `TextStyle`. Full macro integration.
* **`Toggle`**: Switch track migrated to `BoxStyle`, switch label to `TextStyle`.
* **`Radio`**: Radio circle migrated to `BoxStyle`, label to `TextStyle`. Generic `<T: PartialEq + Clone>` builder methods implemented cleanly without polluting macro arms.

### 3. Examples & Showcases
* **`examples/controls_showcase.rs`**: Rich interactive demonstration showcasing styled Checkboxes, Toggles, and custom theme overrides.
* **`examples/wrapper_test/main.rs`**: Demonstration of styled containers and buttons.

### 4. Documentation Updated
* `docs/api/button.md`
* `docs/api/checkbox.md`
* `docs/api/container.md`
* `docs/api/input.md`
* `docs/api/radio.md`
* `docs/api/slider.md`
* `docs/api/text.md`
* `docs/api/text_area.md`
* `docs/api/toggle.md`
* `widgets/button.md`
* `widgets/checkbox.md`
* `widgets/container.md`
* `widgets/input.md`
* `widgets/radio.md`
* `widgets/text.md`
* `widgets/text_area.md`
* `widgets/toggle.md`
* `docs/widget-guide.md`

---

## 📌 Architectural Guidelines for Upcoming Sessions

1. **Keep `macros.rs` Clean**:
   * Do NOT add duplicate arms or redundant method definitions to `macros.rs`.
   * Standard lifetime builders (`'u, 'a` or `'u, 'a, 'b`) invoke `impl_box_style_methods!(BuilderName, 'u, 'a)`.
   * Generic builders with type parameters (such as `RadioBuilder<'u, 'a, 'b, T>` or `DropdownBuilder<'u, 'a, 'b, T>`) should define their forwarding methods directly in their respective files to avoid macro expansion complexities.

2. **Builder `.style()` Overrides**:
   * Every widget builder must expose `.style(BoxStyle)` so themes can be passed in one call:
     ```rust
     ui.button("Save").style(theme.primary_button)
     ```
   * Text-bearing widgets must expose `.text_style(TextStyle)`.

3. **No Unintentional Git Commands**:
   * Do not run `git commit`, `git push`, or modifying git operations unless explicitly requested.

---

## 🚀 Next Priority Tasks

### Step 1: Slider Migration
* File: `crates/zenthra-widgets/src/slider.rs`
* Add `pub track_style: BoxStyle` and `pub thumb_style: BoxStyle` (or unified `style: BoxStyle` for the track).
* Expose `.track_style(...)`, `.thumb_style(...)`, and standard dimension/visual shortcuts.

### Step 2: ProgressBar Migration
* File: `crates/zenthra-widgets/src/progress_bar.rs`
* Unify track and fill bars using `BoxStyle` with full support for solid and gradient fills.

### Step 3: Dropdown Migration
* File: `crates/zenthra-widgets/src/controls/dropdown.rs`
* Unify header box and dropdown item list with `BoxStyle` and `TextStyle`.
