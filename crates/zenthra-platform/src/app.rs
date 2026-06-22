use crate::window::Window;
use crate::event::PlatformEvent;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowAction {
    Drag,
    Minimize,
    Maximize,
    Close,
}

pub struct Frame<'a> {
    pub window: &'a mut Window,
    pub events: &'a [PlatformEvent],
    pub request_redraw_at: Option<std::time::Instant>,
    pub window_actions: Vec<WindowAction>,
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
    decorations: bool,
    draw_fn: Option<Box<dyn FnMut(&mut Frame) -> bool + 'static>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: "Zenthra".to_string(),
            width: 800,
            height: 600,
            decorations: true,
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

    pub fn decorations(mut self, dec: bool) -> Self {
        self.decorations = dec;
        self
    }

    pub fn with_ui<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut Frame) -> bool + 'static,
    {
        self.draw_fn = Some(Box::new(f));
        self
    }

    pub fn run(self) {
        let event_loop = EventLoop::new().unwrap();
        self.run_with_event_loop(event_loop);
    }

    pub fn run_with_event_loop(self, event_loop: EventLoop<()>) {
        let mut runner = AppRunner {
            title: self.title,
            width: self.width,
            height: self.height,
            decorations: self.decorations,
            draw_fn: self.draw_fn,
            window: None,
            pending_events: Vec::new(),
            next_wakeup: None,
        };
        event_loop.run_app(&mut runner).unwrap();
    }
}

struct AppRunner {
    title: String,
    width: u32,
    height: u32,
    decorations: bool,
    draw_fn: Option<Box<dyn FnMut(&mut Frame) -> bool + 'static>>,
    window: Option<Window>,
    pending_events: Vec<PlatformEvent>,
    next_wakeup: Option<std::time::Instant>,
}

impl AppRunner {
    fn render(&mut self, event_loop: &ActiveEventLoop) {
        let Some(window) = &mut self.window else { return };

        let mut needs_redraw = false;
        let mut request_redraw_at = None;
        let mut actions = Vec::new();
        if let Some(draw_fn) = &mut self.draw_fn {
            let mut frame = Frame { 
                window,
                events: &self.pending_events,
                request_redraw_at: None,
                window_actions: Vec::new(),
            };
            needs_redraw = draw_fn(&mut frame);
            request_redraw_at = frame.request_redraw_at;
            actions = frame.window_actions;
        }
        self.pending_events.clear();
        
        self.next_wakeup = request_redraw_at;
        
        for action in actions {
            match action {
                WindowAction::Drag => {
                    let _ = window.winit_window.drag_window();
                }
                WindowAction::Minimize => {
                    window.winit_window.set_minimized(true);
                }
                WindowAction::Maximize => {
                    let is_max = window.winit_window.is_maximized();
                    window.winit_window.set_maximized(!is_max);
                }
                WindowAction::Close => {
                    event_loop.exit();
                }
            }
        }
        
        if needs_redraw {
            window.request_redraw();
        }
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
                self.decorations,
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
                if let Some(w) = &mut self.window { w.request_redraw(); }
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.pending_events.push(PlatformEvent::MouseButton { button, state });
                if let Some(w) = &mut self.window { w.request_redraw(); }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (x, y) = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => (x, y),
                    winit::event::MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                self.pending_events.push(PlatformEvent::MouseWheel { delta_x: x, delta_y: y });
                if let Some(w) = &mut self.window { w.request_redraw(); }
            }
            WindowEvent::Touch(touch) => { 
                self.pending_events.push(PlatformEvent::Touch {
                    id: touch.id,
                    phase: touch.phase,
                    x: touch.location.x,
                    y: touch.location.y,
                });

                // Map first touch to mouse for basic interaction support
                if touch.id == 0 {
                    match touch.phase {
                        winit::event::TouchPhase::Started => {
                            self.pending_events.push(PlatformEvent::MouseMoved { x: touch.location.x, y: touch.location.y });
                            self.pending_events.push(PlatformEvent::MouseButton { 
                                button: winit::event::MouseButton::Left, 
                                state: winit::event::ElementState::Pressed 
                            });
                        }
                        winit::event::TouchPhase::Moved => {
                            self.pending_events.push(PlatformEvent::MouseMoved { x: touch.location.x, y: touch.location.y });
                        }
                        winit::event::TouchPhase::Ended | winit::event::TouchPhase::Cancelled => {
                            self.pending_events.push(PlatformEvent::MouseButton { 
                                button: winit::event::MouseButton::Left, 
                                state: winit::event::ElementState::Released 
                            });
                        }
                    }
                }
                if let Some(w) = &mut self.window { w.request_redraw(); }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(key) = event.physical_key {
                    if event.state == winit::event::ElementState::Pressed {
                        self.pending_events.push(PlatformEvent::KeyDown { key });
                    } else {
                        self.pending_events.push(PlatformEvent::KeyUp { key });
                    }
                }
                
                if event.state == winit::event::ElementState::Pressed {
                    if let Some(text) = event.text {
                        for c in text.chars() {
                            if !c.is_control() {
                                self.pending_events.push(PlatformEvent::CharTyped(c));
                            }
                        }
                    }
                }
                if let Some(w) = &mut self.window { w.request_redraw(); }
            }
            WindowEvent::RedrawRequested => {
                self.render(event_loop);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(wakeup) = self.next_wakeup {
            event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(wakeup));
        } else {
            event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        }
    }
}
