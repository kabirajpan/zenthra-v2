# Zenthra UI Styling Unification Roadmap

This document tracks the progress, completed work, architecture decisions, and remaining roadmap for the **Unified Styling & Theming System** across Zenthra UI.

The canonical design specification and single source of truth for all widget styling methods, signatures, and macro contracts is **[`docs/api/macros.md`](./macros.md)**.

---

## 🎯 High-Level Goal

Unify all widgets and controls in `zenthra-widgets` to strictly conform to **[`docs/api/macros.md`](./macros.md)**:
1. **`Styleable` Trait**: Every widget builder implements `zenthra_core::Styleable` (`box_style_mut()` and `box_style()`).
2. **`BoxStyle`**: Unified struct for container geometry, backgrounds (solid & gradients), borders, corner radii, padding, margin, shadows, and backdrop blur.
3. **`TextStyle`**: Unified struct for typography, font family, font size, text colors, weights, line heights, and wrapping.
4. **`BoxStyleExt`**: GPU rendering primitives (`build_rect_styled`, `build_rect_instance`).
5. **Declarative Macros** in `macros.rs`:
   - `impl_box_style_methods!` (Bundles `impl_dimensions!`, `impl_padding!`, `impl_margin!`, `impl_radius!`, `impl_visuals!`).
   - `impl_text_style_methods!` (Bundles `impl_font_style_methods!`, `impl_text_wrap_methods!`, `impl_text_decorations!`).
   - `impl_interaction_state_methods!` (Zero-cost Sibling architecture for `.hover([ ... ])` and `.active([ ... ])`).

---

## 📊 Summary Status

| Category | Widget / Module | Status | Core Type | Notes |
| :--- | :--- | :---: | :--- | :--- |
| **Core Architecture** | `BoxStyle` (`zenthra-core`) | ✅ Complete | Struct | Unified box geometry, colors, borders, shadows |
| **Core Architecture** | `TextStyle` (`zenthra-text`) | ✅ Complete | Struct | Unified typography engine |
| **Core Architecture** | `BoxStyleExt` (`zenthra-widgets`) | ✅ Complete | Trait | GPU pipeline adapter for styled boxes |
| **Core Architecture** | `macros.rs` (`zenthra-widgets`) | ✅ Complete | Macros | Canonical single-source-of-truth macros (`macros.md`) |
| **Core Widgets** | `Container` | ✅ Complete | `BoxStyle` | Full box styling, gradients, shadows, flex |
| **Core Widgets** | `Button` | ✅ Complete | `BoxStyle` + `TextStyle` | Interactive states, typography, icons |
| **Core Widgets** | `Text` | ✅ Complete | `TextStyle` | Rich typography, wrapping, colors |
| **Form Inputs** | `Input` | ✅ Complete | `BoxStyle` + `TextStyle` | Focus border, text padding, selection highlight |
| **Form Inputs** | `TextArea` | ✅ Complete | `BoxStyle` + `TextStyle` | Multiline, scrollable, focus border, styling |
| **Form Controls** | `Checkbox` | ✅ Complete | `BoxStyle` + `TextStyle` | Box styling, animated checkmark, label typography |
| **Form Controls** | `Toggle` | ✅ Complete | `BoxStyle` + `TextStyle` | Pill track box styling, smooth knob, label typography |
| **Form Controls** | `Radio` | ✅ Complete | `BoxStyle` + `TextStyle` | Circle box styling, inner dot, label typography |
| **Numeric & Ranges** | `Slider` | ✅ Complete | `BoxStyle` | Track & thumb box styling, Sibling states |
| **Containers** | `Card` | ✅ Complete | `BoxStyle` | High-level card container, header/content/footer |
| **Containers** | `Panel` | ✅ Complete | `BoxStyle` | Dockable / sectional panel, header & collapsible body |
| **Numeric & Ranges** | `ProgressBar` | ✅ Complete | `BoxStyle` | Track & fill box styling, shimmer, and interaction states |
| **Form Controls** | `Dropdown` | ✅ Complete | `BoxStyle` + `TextStyle` | Header box, item list, selection styling, portal overlay |
| **Form Controls** | `DragValue` | ✅ Complete | `BoxStyle` + `TextStyle` | Interactive numeric drag field, modifier keys, in-place edit |
| **Overlays** | `Menu` | ✅ Complete | `BoxStyle` + `TextStyle` | Menu bar, cascading menus, flyout popup items |
| **Overlays** | `Popover` | ✅ Complete | `BoxStyle` | Popover floating box |
| **Layout & Media** | `Stack` | ✅ Complete | `BoxStyle` | Layered absolute / z-index stack |
| **Layout & Media** | `Image` | ✅ Complete | `BoxStyle` | Border, radius, shadow for image elements |

---

## ✅ Completed in Detail

### 1. Unified Core & Macro Architecture
* **`docs/api/macros.md`**: Master specification defining all generated method signatures, parameters, locks, and cross-endpoint availability matrices.
* **`crates/zenthra-widgets/src/style_ext.rs`**: Implements `BoxStyleExt` with GPU-accelerated drawing for backgrounds (solid + linear/radial gradients), borders, corner radii, and drop shadows.
* **`crates/zenthra-widgets/src/macros.rs`**: Clean declarative macro implementations with zero code bloat and full lifetime support (`'u, 'a` and `'u, 'a, 'b`).

### 2. Migrated Widgets
* **`Container`**: Uses `self.style: BoxStyle` + `impl_box_style_methods!` + `impl_interaction_state_methods!`.
* **`Button`**: Uses `self.style: BoxStyle` + `self.text_style: TextStyle` + `impl_box_style_methods!` + `impl_text_style_methods!` + `impl_interaction_state_methods!`.
* **`Text`**: Uses `self.style: TextStyle` + `impl_text_style_methods!`.
* **`Input`**: Single-line text input with unified box styling, `focus_border`, placeholder styling, and cursor selection highlight.
* **`TextArea`**: Multi-line editor with unified box styling, scrolling, text padding, `focus_border`, and wrapping.
* **`Checkbox`**: Checkbox button and optional label migrated to `BoxStyle` and `TextStyle`. Full macro integration.
* **`Toggle`**: Switch track migrated to `BoxStyle`, switch label to `TextStyle`.
* **`Radio`**: Radio circle migrated to `BoxStyle`, label to `TextStyle`. Generic `<T: PartialEq + Clone>` builder methods implemented cleanly.
* **`Card`**: High-level card container with complete `BoxStyle` parity, hover/active interaction states, and backdrop blur.
* **`Panel`**: Collapsible header/content panel with unified `BoxStyle` parity and clean theme overrides.
* **`Slider`**: Track & thumb modernized with `BoxStyle` parity, granular radii, and hover/active states.
* **`ProgressBar`**: Modernized with `BoxStyle` parity, granular radii, solid/gradient track & fill, shimmer, and hover/active states.
* **`Dropdown`**: Header box migrated to `BoxStyle` + `TextStyle`, portal menu styling to `BoxStyle`, outside-click dismissal, and hover/active sibling state overrides.
* **`DragValue`**: Numeric box migrated to `BoxStyle` + `TextStyle`, `focus_border`, modifier keys (Shift/Ctrl), and hover/active sibling state overrides.
* **`Menu`**: `MenuBarBuilder`, `MenuBuilder`, and `MenuItemBuilder` unified with `BoxStyle` + `TextStyle` parity, popup styling, and Sibling hover/active state overrides.
* **`Popover`**: Floating overlay container unified with `BoxStyle` + `Styleable` parity, adaptive placement engine, auto-flipping, and backdrop blur.
* **`Stack`**: Multi-layer layout container unified with `BoxStyle` parity, child alignment (`halign`, `valign`), clipping, and backdrop blur.
* **`Image`**: Hardware-accelerated image widget unified with `BoxStyle` parity, object-fit scaling, 3D perspective tilts, UV manipulation, and Sibling hover/active states.

### 3. Documentation Updated
* [`docs/api/macros.md`](./macros.md) (Master Specification)
* [`docs/api/container.md`](./container.md)
* [`docs/api/button.md`](./button.md)
* [`docs/api/text.md`](./text.md)
* [`docs/api/input.md`](./input.md)
* [`docs/api/text_area.md`](./text_area.md)
* [`docs/api/checkbox.md`](./checkbox.md)
* [`docs/api/toggle.md`](./toggle.md)
* [`docs/api/radio.md`](./radio.md)
* [`docs/api/card.md`](./card.md)
* [`docs/api/panel.md`](./panel.md)
* [`docs/api/slider.md`](./slider.md)
* [`docs/api/progress_bar.md`](./progress_bar.md)
* [`docs/api/dropdown.md`](./dropdown.md)
* [`docs/api/drag_value.md`](./drag_value.md)
* [`docs/api/menu.md`](./menu.md)
* [`docs/api/popover.md`](./popover.md)
* [`docs/api/stack.md`](./stack.md)
* [`docs/api/image.md`](./image.md)

---

## 📌 Architectural Guidelines for Widget Migrations (from `macros.md`)

Whenever migrating an upcoming widget, follow these 4 mandatory rules from `macros.md`:

1. **Implement `Styleable`**:
   The builder struct must hold `pub style: BoxStyle` and implement `zenthra_core::Styleable`:
   ```rust
   impl<'u, 'a> Styleable for MyWidgetBuilder<'u, 'a> {
       #[inline]
       fn box_style_mut(&mut self) -> &mut BoxStyle { &mut self.style }
       #[inline]
       fn box_style(&self) -> &BoxStyle { &self.style }
   }
   ```

2. **Invoke Canonical Macros**:
   ```rust
   crate::impl_box_style_methods!(MyWidgetBuilder, 'u, 'a);
   crate::impl_interaction_state_methods!(MyWidgetBuilder, 'u, 'a); // if interactive
   ```

3. **Render via `BoxStyleExt`**:
   Outer visual enclosures must be drawn with `self.style.build_rect_styled(...)` with hover/active state blending.

4. **Preserve Sub-Component Domain Methods**:
   Widget-specific child components (such as dropdown list boxes, popup items, or menu arrows) use clean prefixes (e.g. `header_*`, `item_*`) and inherit defaults from `self.style`.

---

## 🚀 Next Priority Tasks

### Step 1: Dropdown Migration
* File: `crates/zenthra-widgets/src/controls/dropdown.rs`
* Implement `Styleable` and apply `impl_box_style_methods!` + `impl_text_style_methods!` to `DropdownBuilder`.
* Unify header box and dropdown item list styling with `BoxStyle` and `TextStyle`.

### Step 2: DragValue Migration
* File: `crates/zenthra-widgets/src/controls/drag_value.rs`
* Implement `Styleable` and apply `impl_box_style_methods!` + `impl_text_style_methods!` to `DragValueBuilder`.
* Unify interactive numeric drag field styling with `BoxStyle` and `TextStyle`.
