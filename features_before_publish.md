# Zenthra Framework: Roadmap to Public Release

This roadmap outlines the critical architectural features, rendering optimizations, and widget API improvements that Zenthra needs to implement before it is ready for public release. These requirements were identified during real-world testing with the `image-viewer` application.

---

## 1. Core Render & Texture Pipeline
* **Asynchronous Image Loading**: Add background-thread decoding and resizing (using `rayon` or worker threads) to prevent the main thread from stuttering when opening large images.
* **Texture Manager with LRU Cache**: Encapsulate texture caching inside a central manager that automatically evicts least-recently-used textures based on a configurable GPU memory limit.
* **Mipmapping Support**: Automatically generate and sample from mipmaps when scaling down textures to prevent visual aliasing and shimmering on small thumbnails.

## 2. Declarative Event Handling System
* **Widget Callbacks**: Replace low-level manual event loops with declarative, chainable callback methods on widgets:
  ```rust
  ui.button("Click Me")
      .on_click(|| println!("Clicked!"))
      .on_hover(|hovered| { /* ... */ })
      .on_scroll(|delta_x, delta_y| { /* ... */ })
  ```
* **Event Propagation (Bubbling)**: Implement standard event capture, target, and bubbling phases to cleanly route events through nested widgets and parent containers.

## 3. Text Layout & Typography Enhancements
* **Text Truncation (Ellipsis)**: Native support for truncating long text strings with trailing ellipsis (`...`) to fit their parents.
* **Auto-Wrapping**: Implement multi-line word wrapping based on available container bounds.
* **System Font Fallbacks**: Graceful fallback to default system fonts when custom fonts fail to load.

## 4. Component-Based Architecture
* **Stateful Isolated Components**: A trait-based component model (similar to React or Iced's `Component`) to allow developers to build self-contained widgets with local states:
  ```rust
  pub trait Component {
      type State;
      type Message;
      fn update(&mut self, state: &mut Self::State, message: Self::Message);
      fn view(&self, state: &Self::State, ui: &mut Ui);
  }
  ```

## 5. Layout Engine Flexibilities
* **Automatic Flex-Wrap**: Support for layout wrapping in rows/columns when items exceed available dimensions.
* **Grid Layouts**: A simplified CSS Grid-like structure for aligned rows and columns.
* **Aspect Ratio Locks**: Allow widgets to request aspect ratio locks (e.g., maintaining 16:9 or 1:1) during resize reflows.

## 6. Animations & Transitions Engine
* **Declarative Transitions**: Add transition properties to widgets to animate style changes smoothly:
  ```rust
  ui.button("Hover")
      .transition("scale", Duration::from_millis(150), Transition::EaseInOut)
      .transition("border-color", Duration::from_millis(100), Transition::Linear)
  ```
* **Spring Animations**: Native physics-based spring animations for a highly responsive, premium feel.

## 7. Accessibility & Keyboard Navigation
* **Focus Management**: A global focus engine that handles Tab key traversal, focus rings, and activation via spacebar/enter keys.
* **Keyboard Shortcuts**: Easy registration of local and global keyboard event shortcuts on containers and buttons.

## 8. Hierarchical Coordinate Space & Clipping Stack
* **Automatic Coordinate Transforms**: Manage coordinate mapping hierarchically so that parent scroll offsets are automatically subtracted during hit testing (resolving the issue where scroll containers cause incorrect hover highlights on children).
* **Unified Scissoring Stack**: Implement a stack-based scissor/clipping mask API in the renderer to automatically clip overflowing child widgets without requiring manual scissor calculations in container widgets.

## 9. Native Platform Services
* **System Dialogs**: Provide cross-platform wrappers for opening native file dialogs, directory choosers, and message boxes (crucial for loading custom image folders).
* **Clipboard & Drag-and-Drop**: Built-in support for reading/writing system clipboard text and native drag-and-drop file operations.

## 10. Hot Asset Reloading
* **Live File Watcher**: A built-in asset watcher that monitors local files (images, text, styles) and automatically updates loaded textures or themes in real-time when changed on disk, without requiring an application restart.

## 11. Event-Driven Loop & Idle Rendering (CPU Optimization)
* **Winit Wait Control Flow**: Transition the event loop from active polling (`ControlFlow::Poll`) to idle waiting (`ControlFlow::Wait` or `ControlFlow::WaitUntil`). This reduces application CPU/GPU usage to 0% when idle.
* **Timer API**: Provide a framework timer registry (e.g., `ui.request_redraw_after(duration)`) to support slideshows and blinking cursors without forcing continuous redrawing on every frame.

## 12. Native Overlay & Popover Layer
* **Floating/Modal Window API**: A native overlay layer where developers can declare popups and modals:
  ```rust
  ui.overlay(|ui| {
      ui.window("About Zenthra")
          .draggable(true)
          .modal(true)
          .close_on_outside_click(true)
          .show(|ui| { /* content */ });
  });
  ```
  The framework should handle drag state tracking, z-index ordering, and input event blocking (modals) automatically.

