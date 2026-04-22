// crates/zenthra-core/src/event.rs

use glam::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    // ── Pointer ──────────────────────────────────────────────
    PointerMoved { pos: Vec2 },
    PointerPressed { pos: Vec2, button: PointerButton },
    PointerReleased { pos: Vec2, button: PointerButton },
    PointerEntered,
    PointerLeft,
    Scroll { delta: Vec2 },

    // ── Keyboard ─────────────────────────────────────────────
    KeyPressed { key: KeyCode, modifiers: Modifiers },
    KeyReleased { key: KeyCode, modifiers: Modifiers },
    TextInput { ch: char },

    // ── Focus ─────────────────────────────────────────────────
    FocusGained,
    FocusLost,

    // ── Window ───────────────────────────────────────────────
    Resized { width: u32, height: u32 },
    CloseRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerButton {
    Primary,
    Secondary,
    Middle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Backspace,
    Delete,
    Tab,
    Return,
    Escape,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Other(u32),
}

/// What a widget returns after handling an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResponse {
    /// Stop propagation — this widget consumed the event.
    Consumed,
    /// Let it bubble up.
    Ignored,
}
