use zenthra_platform::app::{App as PlatformApp, Frame};
use zenthra_render::{RectInstance, RectPipeline};
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
        let mut image_pipeline: Option<zenthra_render::ImagePipeline> = None;
        let mut texture_cache: std::collections::HashMap<
            zenthra_core::ImageSource,
            (std::sync::Arc<wgpu::BindGroup>, u32, u32),
        > = std::collections::HashMap::new();
        let mut image_sizes: std::collections::HashMap<zenthra_core::ImageSource, (u32, u32)> =
            std::collections::HashMap::new();
        let mut zentype: Option<Zentype> = None;
        let mut focused_id: Option<zenthra_core::Id> = None;
        let mut state: std::collections::HashMap<zenthra_core::Id, (f32, f32)> =
            std::collections::HashMap::new();
        let mut cursor_state: std::collections::HashMap<zenthra_core::Id, usize> =
            std::collections::HashMap::new();
        let mut interaction_state: std::collections::HashMap<zenthra_core::Id, f32> =
            std::collections::HashMap::new();
        let mut mouse_pos: (f32, f32) = (0.0, 0.0);
        let mut ui_mouse_down = false;
        let mut active_drag: Option<zenthra_widgets::ui::ScrollDrag> = None;
        let mut layout_cache: std::collections::HashMap<
            zenthra_core::Id,
            (zenthra_core::Rect, u64),
        > = std::collections::HashMap::new();
        let start_time = std::time::Instant::now();

        self.platform = self.platform.with_ui(move |frame: &mut Frame| {
            let elapsed = start_time.elapsed().as_secs_f32();
            let device = frame.window.gpu.device.clone();
            let queue = &frame.window.gpu.queue;
            let config = &frame.window.gpu.config;
            let width = frame.window.width();
            let height = frame.window.height();
            let sf = frame.scale_factor() as f32;
            let mut next_layout_cache = std::collections::HashMap::new();

            // Initialize or update the Zentype engine
            let engine = zentype.get_or_insert_with(|| Zentype::new(device.clone(), queue, config));

            let mut needs_redraw = false;

            // Update persistent mouse pos from current frame events
            let mut ui_clicked = false;
            for event in frame.events {
                match event {
                    zenthra_platform::event::PlatformEvent::MouseMoved { x, y } => {
                        mouse_pos = (*x as f32 / sf, *y as f32 / sf);
                        needs_redraw = true;
                    }
                    zenthra_platform::event::PlatformEvent::MouseButton { state, .. } => {
                        let was_down = ui_mouse_down;
                        ui_mouse_down = *state == winit::event::ElementState::Pressed;
                        if ui_mouse_down && !was_down {
                            ui_clicked = true;
                        }
                        // Only clear active_drag on a real release event,
                        // and let the UI handle the rest.
                        if !ui_mouse_down {
                            active_drag = None;
                        }
                        needs_redraw = true;
                    }
                    _ => {}
                }
            }

            // Ensure the engine's projection matrix matches the current window size
            engine.resize(queue, width, height);

            let rp = rect_pipeline.get_or_insert_with(|| RectPipeline::new(&device, config.format));
            let ip = image_pipeline
                .get_or_insert_with(|| zenthra_render::ImagePipeline::new(&device, config.format));
            ip.prepare(queue, width, height);

            let font_system = engine.font_system();
            let (logical_w, logical_h) = (width as f32 / sf, height as f32 / sf);


            {
                let (draws, overlays) = {
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
                        &mut interaction_state,
                        active_drag,
                        ui_clicked,
                        elapsed,
                        &layout_cache,
                        &mut next_layout_cache,
                        &image_sizes,
                    );

                    f(&mut ui);

                    focused_id = ui.focused_id;
                    active_drag = ui.active_drag;
                    needs_redraw |= ui.needs_redraw;
                    
                    (ui.draws, ui.overlays)
                };

                let surface_texture = match frame.window.gpu.surface.get_current_texture() {
                    wgpu::CurrentSurfaceTexture::Success(t) => t,
                    _ => return false,
                };
                let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Zenthra Frame") });

                // --- PASS 1: CLEAR ---
                {
                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Clear Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.05, b: 0.07, a: 1.0 }),
                                store: wgpu::StoreOp::Store,
                            },
                            depth_slice: None,
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                        multiview_mask: None,
                    });
                }

                // Prepare RectPipeline uniforms once for the frame
                rp.prepare(&device, queue, width, height, &[]);

                // Prepare RectPipeline uniforms once for the frame
                rp.prepare(&device, queue, width, height, &[]);

                // Temporary storage for buffers to keep them alive during the pass
                let mut temp_buffers: Vec<wgpu::Buffer> = Vec::new();

                // Helper to process a command list interleaved
                let mut process_interleaved = |encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, cmds: &[DrawCommand], temp_bufs: &mut Vec<wgpu::Buffer>| {
                    let mut current_rects = Vec::new();

                    let flush_batch = |encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, rects: &mut Vec<RectInstance>, temp_bufs: &mut Vec<wgpu::Buffer>| {
                        if rects.is_empty() { return; }
                        use wgpu::util::DeviceExt;
                        let buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Rect Instance Buffer"),
                            contents: bytemuck::cast_slice(rects),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                        {
                            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("Rect Batch Pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Load,
                                        store: wgpu::StoreOp::Store,
                                    },
                                    depth_slice: None,
                                })],
                                depth_stencil_attachment: None,
                                timestamp_writes: None,
                                occlusion_query_set: None,
                                multiview_mask: None,
                            });
                            
                            pass.set_pipeline(&rp.pipeline);
                            pass.set_bind_group(0, &rp.uniform_bg, &[]);
                            pass.set_vertex_buffer(0, buf.slice(..));
                            pass.draw(0..6, 0..rects.len() as u32);
                        }
                        
                        temp_bufs.push(buf);
                        rects.clear();
                    };

                    for cmd in cmds {
                        match cmd {
                            DrawCommand::Rect(rd) => {
                                let mut inst = rd.instance;
                                inst.pos[0] *= sf; inst.pos[1] *= sf;
                                inst.size[0] *= sf; inst.size[1] *= sf;
                                inst.shadow_offset[0] *= sf; inst.shadow_offset[1] *= sf;
                                inst.shadow_blur *= sf;
                                inst.clip_rect[0] *= sf; inst.clip_rect[1] *= sf;
                                inst.clip_rect[2] *= sf; inst.clip_rect[3] *= sf;
                                current_rects.push(inst);
                            }
                            DrawCommand::Text(td) => {
                                flush_batch(encoder, view, &mut current_rects, temp_bufs);
                                let mut opts = td.options.clone();
                                opts.scale_factor = sf;
                                opts.clip_rect = Some([td.clip[0] * sf, td.clip[1] * sf, td.clip[2] * sf, td.clip[3] * sf]);
                                engine.draw(queue, &td.text, td.pos, &opts);
                                
                                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: Some("Text Pass"),
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Load,
                                            store: wgpu::StoreOp::Store,
                                        },
                                        depth_slice: None,
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                    multiview_mask: None,
                                });
                                engine.render(&mut pass);
                            }
                            DrawCommand::Image(id_cmd) => {
                                flush_batch(encoder, view, &mut current_rects, temp_bufs);
                                if !texture_cache.contains_key(&id_cmd.source) {
                                    let bytes = match &id_cmd.source {
                                        zenthra_core::ImageSource::Path(p) => std::fs::read(p).unwrap_or_default(),
                                        zenthra_core::ImageSource::Bytes(b) => b.to_vec(),
                                    };
                                    if let Ok((bg, w, h)) = zenthra_render::texture::create_texture_bind_group(&device, queue, &ip.texture_bgl, &bytes) {
                                        texture_cache.insert(id_cmd.source.clone(), (std::sync::Arc::new(bg), w, h));
                                        image_sizes.insert(id_cmd.source.clone(), (w, h));
                                    }
                                }
                                if let Some((bg, _tw, _th)) = texture_cache.get(&id_cmd.source) {
                                    let mut inst = id_cmd.instance;
                                    inst.pos[0] *= sf; inst.pos[1] *= sf;
                                    inst.size[0] *= sf; inst.size[1] *= sf;
                                    inst.clip_rect[0] *= sf; inst.clip_rect[1] *= sf;
                                    inst.clip_rect[2] *= sf; inst.clip_rect[3] *= sf;
                                    
                                    use wgpu::util::DeviceExt;
                                    let buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                        label: Some("Image Instance"),
                                        contents: bytemuck::bytes_of(&inst),
                                        usage: wgpu::BufferUsages::VERTEX,
                                    });
                                    
                                    {
                                        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                            label: Some("Image Pass"),
                                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                                view,
                                                resolve_target: None,
                                                ops: wgpu::Operations {
                                                    load: wgpu::LoadOp::Load,
                                                    store: wgpu::StoreOp::Store,
                                                },
                                                depth_slice: None,
                                            })],
                                            depth_stencil_attachment: None,
                                            timestamp_writes: None,
                                            occlusion_query_set: None,
                                            multiview_mask: None,
                                        });
                                        ip.draw(&mut pass, bg, &buf, 1);
                                    }
                                    temp_bufs.push(buf);
                                }
                            }
                            DrawCommand::OverlayRect(od) => {
                                flush_batch(encoder, view, &mut current_rects, temp_bufs);
                                use wgpu::util::DeviceExt;
                                let inst = zenthra_render::RectInstance {
                                    pos: [od.x * sf, od.y * sf],
                                    size: [od.width * sf, od.height * sf],
                                    color: od.color.to_array(),
                                    clip_rect: [od.clip[0] * sf, od.clip[1] * sf, od.clip[2] * sf, od.clip[3] * sf],
                                    ..Default::default()
                                };
                                let buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                    label: Some("Overlay Rect Buffer"),
                                    contents: bytemuck::bytes_of(&inst),
                                    usage: wgpu::BufferUsages::VERTEX,
                                });
                                
                                {
                                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                        label: Some("Overlay Rect Pass"),
                                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                            view,
                                            resolve_target: None,
                                            ops: wgpu::Operations {
                                                load: wgpu::LoadOp::Load,
                                                store: wgpu::StoreOp::Store,
                                            },
                                            depth_slice: None,
                                        })],
                                        depth_stencil_attachment: None,
                                        timestamp_writes: None,
                                        occlusion_query_set: None,
                                        multiview_mask: None,
                                    });
                                    pass.set_pipeline(&rp.pipeline);
                                    pass.set_bind_group(0, &rp.uniform_bg, &[]);
                                    pass.set_vertex_buffer(0, buf.slice(..));
                                    pass.draw(0..6, 0..1);
                                }
                                temp_bufs.push(buf);
                            }
                        }
                    }
                    flush_batch(encoder, view, &mut current_rects, temp_bufs);
                };

                process_interleaved(&mut encoder, &view, &draws, &mut temp_buffers);
                process_interleaved(&mut encoder, &view, &overlays, &mut temp_buffers);

                queue.submit(std::iter::once(encoder.finish()));
                surface_texture.present();
            }

            // Swap layout caches for next frame
            layout_cache = next_layout_cache;
            needs_redraw
        });
        self
    }

    pub fn run(self) {
        self.platform.run();
    }
}
