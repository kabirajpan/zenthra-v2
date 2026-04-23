use winit::event::{MouseButton, ElementState};
use winit::keyboard::KeyCode;

#[derive(Debug, Clone)]
pub enum PlatformEvent {
    MouseMoved { x: f64, y: f64 },
    MouseButton { button: MouseButton, state: ElementState },
    MouseWheel { delta_x: f32, delta_y: f32 },
    KeyDown { key: KeyCode },
    CharTyped(char),
}
