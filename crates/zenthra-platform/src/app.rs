use crate::window::Window;
use crate::event::PlatformEvent;
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, MouseButton, ElementState},
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

pub struct Frame<'a> {
    pub window: &'a mut Window,
    pub events: &'a [PlatformEvent],
}

impl<'a> Frame<'a> {
    pub fn width(&self) -> u32 {
        self.window.width()
    }
    pub fn height(&self) -> u32 {
        self.window.height()
    }
    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }
}

pub struct App {
    title: String,
    width: u32,
    height: u32,
    draw_fn: Option<Box<dyn FnMut(&mut Frame) + 'static>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: "Zenthra".to_string(),
            width: 800,
            height: 600,
            draw_fn: None,
        }
    }

    pub fn title(mut self, t: &str) -> Self {
        self.title = t.to_string();
        self
    }

    pub fn size(mut self, w: u32, h: u32) -> Self {
        self.width = w;
        self.height = h;
        self
    }

    pub fn with_ui<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut Frame) + 'static,
    {
        self.draw_fn = Some(Box::new(f));
        self
    }

    pub fn run(self) {
        let event_loop = EventLoop::new().unwrap();
        let mut runner = AppRunner {
            title: self.title,
            width: self.width,
            height: self.height,
            draw_fn: self.draw_fn,
            window: None,
            pending_events: Vec::new(),
        };
        event_loop.run_app(&mut runner).unwrap();
    }
}

struct AppRunner {
    title: String,
    width: u32,
    height: u32,
    draw_fn: Option<Box<dyn FnMut(&mut Frame) + 'static>>,
    window: Option<Window>,
    pending_events: Vec<PlatformEvent>,
}

impl AppRunner {
    fn render(&mut self) {
        let Some(window) = &mut self.window else { return };

        if let Some(draw_fn) = &mut self.draw_fn {
            let mut frame = Frame { 
                window,
                events: &self.pending_events,
            };
            draw_fn(&mut frame);
        }
        self.pending_events.clear();
    }
}

impl ApplicationHandler for AppRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = pollster::block_on(Window::new(
                event_loop,
                &self.title,
                self.width,
                self.height,
            ));
            window.request_redraw();
            self.window = Some(window);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(w) = &mut self.window {
                    w.resize(size);
                    w.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.pending_events.push(PlatformEvent::MouseMoved {
                    x: position.x,
                    y: position.y,
                });
            }
            WindowEvent::MouseInput { button, state, .. } => {
                println!("MouseButton: {:?} {:?}", button, state); self.pending_events.push(PlatformEvent::MouseButton { button, state });
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    if let winit::keyboard::PhysicalKey::Code(key) = event.physical_key {
                         println!("KeyDown: {:?}", key); self.pending_events.push(PlatformEvent::KeyDown { key });
                    }
                    
                    if let Some(text) = event.text {
                        for c in text.chars() {
                            if !c.is_control() {
                                println!("CharTyped: {}", c); self.pending_events.push(PlatformEvent::CharTyped(c));
                            }
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.render();
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            _ => {}
        }
    }
}
