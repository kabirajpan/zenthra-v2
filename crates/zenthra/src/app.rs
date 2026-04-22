use zenthra_platform::app::{App as PlatformApp, Frame};
use zenthra_render::{RectPipeline};
use zenthra_text::prelude::*;
use zenthra_widgets::ui::DrawCommand;
use zenthra_widgets::Ui;
use std::sync::{Arc, Mutex};

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
        let mut mouse_pos: (f32, f32) = (0.0, 0.0);

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
            for event in frame.events {
                if let zenthra_platform::event::PlatformEvent::MouseMoved { x, y } = event {
                    mouse_pos = (*x as f32 / sf, *y as f32 / sf);
                }
            }

            // Ensure the engine's projection matrix matches the current window size
            engine.resize(queue, width, height);

            let rp = rect_pipeline.get_or_insert_with(|| RectPipeline::new(&device, config.format));
            
            // We need the font_system from the engine for Ui and widgets to measure
            let font_system = engine.font_system(); // We'll add this accessor

            let mut ui = Ui::new(
                width,
                height,
                frame.scale_factor(),
                Some(font_system),
                frame.events.to_vec(),
                focused_id,
                mouse_pos,
            );
            
            f(&mut ui);
            focused_id = ui.focused_id;

            let mut rect_instances = Vec::new();

            // Process draw commands
            for cmd in &ui.draws {
                match cmd {
                    DrawCommand::Rect(r) => {
                        rect_instances.push(r.instance);
                    }
                    DrawCommand::Text(td) => {
                        // Zentype handles all shaping, measurement, and background generation internally!
                        engine.draw(queue, &td.text, td.pos, &td.options);
                    }
                    DrawCommand::Cursor(cd) => {
                        engine.draw_rect([cd.x, cd.y], [2.0, cd.height], cd.color);
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
