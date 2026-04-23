use zenthra_platform::app::{App as PlatformApp, Frame};
use zenthra_render::{RectPipeline};
use zenthra_text::prelude::*;
use zenthra_widgets::ui::DrawCommand;
use zenthra_widgets::Ui;

pub struct App {
    platform: PlatformApp,
}

impl App {
    pub fn new() -> Self {
        Self {
            platform: PlatformApp::new(),
        }
    }

    pub fn title(mut self, t: &str) -> Self {
        self.platform = self.platform.title(t);
        self
    }
    pub fn size(mut self, w: u32, h: u32) -> Self {
        self.platform = self.platform.size(w, h);
        self
    }

    pub fn with_ui<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Ui) + 'static,
    {
        let mut rect_pipeline: Option<RectPipeline> = None;
        let mut zentype: Option<Zentype> = None;
        let mut focused_id: Option<u64> = None;
        let mut state: std::collections::HashMap<u64, f32> = std::collections::HashMap::new();
        let mut cursor_state: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
        let mut mouse_pos: (f32, f32) = (0.0, 0.0);
        let mut ui_mouse_down = false;
        let mut active_drag: Option<zenthra_widgets::ui::ScrollDrag> = None;

        self.platform = self.platform.with_ui(move |frame: &mut Frame| {
            let device = frame.window.gpu.device.clone();
            let queue = &frame.window.gpu.queue;
            let config = &frame.window.gpu.config;
            let width = frame.window.width();
            let height = frame.window.height();
            let sf = frame.scale_factor() as f32;

            // Initialize or update the Zentype engine
            let engine = zentype.get_or_insert_with(|| {
                Zentype::new(device.clone(), queue, config)
            });

            // Update persistent mouse pos from current frame events
            let mut ui_clicked = false;
            for event in frame.events {
                match event {
                    zenthra_platform::event::PlatformEvent::MouseMoved { x, y } => {
                        mouse_pos = (*x as f32 / sf, *y as f32 / sf);
                    }
                    zenthra_platform::event::PlatformEvent::MouseButton { state, .. } => {
                        let was_down = ui_mouse_down;
                        ui_mouse_down = *state == winit::event::ElementState::Pressed;
                        if ui_mouse_down && !was_down {
                            ui_clicked = true;
                        }
                        if !ui_mouse_down {
                            active_drag = None;
                        }
                    }
                    _ => {}
                }
            }

            // Ensure the engine's projection matrix matches the current window size
            engine.resize(queue, width, height);

            let rp = rect_pipeline.get_or_insert_with(|| RectPipeline::new(&device, config.format));
            
            // We need the font_system from the engine for Ui and widgets to measure
            let font_system = engine.font_system(); // We'll add this accessor

            let (logical_w, logical_h) = (width as f32 / sf, height as f32 / sf);

            let mut ui = Ui::new(
                logical_w as u32,
                logical_h as u32,
                frame.scale_factor(),
                Some(font_system),
                frame.events.to_vec(),
                focused_id,
                mouse_pos,
                ui_mouse_down,
                &mut state,
                &mut cursor_state,
                active_drag,
                ui_clicked,
            );
            
            f(&mut ui);
            focused_id = ui.focused_id;
            active_drag = ui.active_drag;

            let mut rect_instances = Vec::new();

            // Process draw commands
            for cmd in &ui.draws {
                match cmd {
                    DrawCommand::Rect(rd) => {
                        let mut inst = rd.instance;
                        inst.pos[0] *= sf;
                        inst.pos[1] *= sf;
                        inst.size[0] *= sf;
                        inst.size[1] *= sf;
                        inst.clip_rect[0] *= sf;
                        inst.clip_rect[1] *= sf;
                        inst.clip_rect[2] *= sf;
                        inst.clip_rect[3] *= sf;
                        rect_instances.push(inst);
                    }
                    DrawCommand::Text(td) => {
                        let mut scaled_options = td.options.clone();
                        scaled_options.scale_factor = sf;
                        engine.draw(queue, &td.text, td.pos, &scaled_options);
                    }
                    DrawCommand::OverlayRect(od) => {
                        // Keep manual scaling for OverlayRect as it's a raw call
                        engine.draw_rect(
                            [od.x * sf, od.y * sf], 
                            [od.width * sf, od.height * sf], 
                            od.color, 
                            [od.clip[0] * sf, od.clip[1] * sf, od.clip[2] * sf, od.clip[3] * sf]
                        );
                    }
                }
            }

            rp.prepare(&device, queue, width, height, &rect_instances);

            let surface_texture = match frame.window.gpu.surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(t) => t,
                wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
                _ => return,
            };

            let view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Zenthra Frame"),
            });

            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Main Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.05,
                                g: 0.05,
                                b: 0.07,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });

                rp.draw(&mut pass);
                engine.render(&mut pass); // Clean!
            }

            queue.submit(std::iter::once(encoder.finish()));
            surface_texture.present();
        });
        self
    }

    pub fn run(self) {
        self.platform.run();
    }
}
