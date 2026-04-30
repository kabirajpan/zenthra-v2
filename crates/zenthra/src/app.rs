use zenthra_platform::app::{App as PlatformApp, Frame};
use zenthra_render::RectPipeline;
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

            let mut image_buffers = Vec::new();

            {
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

                let mut rect_instances = Vec::new();
                let mut image_draw_calls: Vec<(
                    std::sync::Arc<wgpu::BindGroup>,
                    zenthra_render::ImageInstance,
                )> = Vec::new();

                // Process draw commands
                for cmd in &ui.draws {
                    match cmd {
                        DrawCommand::Rect(rd) => {
                            let mut inst = rd.instance;
                            inst.pos[0] *= sf;
                            inst.pos[1] *= sf;
                            inst.size[0] *= sf;
                            inst.size[1] *= sf;
                            inst.shadow_offset[0] *= sf;
                            inst.shadow_offset[1] *= sf;
                            inst.shadow_blur *= sf;
                            inst.clip_rect[0] *= sf;
                            inst.clip_rect[1] *= sf;
                            inst.clip_rect[2] *= sf;
                            inst.clip_rect[3] *= sf;
                            rect_instances.push(inst);
                        }
                        DrawCommand::Image(id_cmd) => {
                            let mut inst = id_cmd.instance;
                            inst.pos[0] *= sf;
                            inst.pos[1] *= sf;
                            inst.size[0] *= sf;
                            inst.size[1] *= sf;
                            inst.shadow_offset[0] *= sf;
                            inst.shadow_offset[1] *= sf;
                            inst.shadow_blur *= sf;
                            inst.clip_rect[0] *= sf;
                            inst.clip_rect[1] *= sf;
                            inst.clip_rect[2] *= sf;
                            inst.clip_rect[3] *= sf;

                            // Load texture if needed
                            if !texture_cache.contains_key(&id_cmd.source) {
                                let bytes = match &id_cmd.source {
                                    zenthra_core::ImageSource::Path(p) => {
                                        std::fs::read(p).unwrap_or_default()
                                    }
                                    zenthra_core::ImageSource::Bytes(b) => b.to_vec(),
                                };
                                if let Ok((bg, w, h)) =
                                    zenthra_render::texture::create_texture_bind_group(
                                        &device,
                                        queue,
                                        &ip.texture_bgl,
                                        &bytes,
                                    )
                                {
                                    texture_cache.insert(
                                        id_cmd.source.clone(),
                                        (std::sync::Arc::new(bg), w, h),
                                    );
                                    image_sizes.insert(id_cmd.source.clone(), (w, h));
                                    needs_redraw = true; // Redraw now that we have the real size
                                }
                            }

                            if let Some((bg, tw, th)) = texture_cache.get(&id_cmd.source) {
                                let img_w = *tw as f32;
                                let img_h = *th as f32;
                                let img_aspect = img_w / img_h;
                                let box_aspect = inst.size[0] / inst.size[1];

                                // uv_rect layout: [u_start, v_start, u_size, v_size]
                                // i.e. the shader samples from u_start..(u_start+u_size)
                                //                              v_start..(v_start+v_size)
                                match id_cmd.fit {
                                    zenthra_core::ObjectFit::Fill => {
                                        // Stretch to fill the box exactly — no aspect correction
                                        inst.uv_rect = [0.0, 0.0, 1.0, 1.0];
                                    }

                                    zenthra_core::ObjectFit::Contain => {
                                        let internal_scale = 0.9;
                                        let uv_zoom = 1.0 / internal_scale;
                                        let uv_shift = (1.0 - uv_zoom) / 2.0;

                                        let final_u_size = uv_zoom * 0.93; // 👈 YOUR WIDE SETTING
                                        let final_u_start = (1.0 - final_u_size) / 2.0;

                                        if img_aspect > box_aspect {
                                            let v_ratio = box_aspect / img_aspect;
                                            let v_size = uv_zoom / v_ratio;
                                            let v_start = (1.0 - v_size) / 2.0;
                                            inst.uv_rect =
                                                [final_u_start, v_start, final_u_size, v_size];
                                        } else {
                                            let u_ratio = img_aspect / box_aspect;
                                            let u_size = final_u_size / u_ratio; // 👈 Apply wide setting here too
                                            let u_start = (1.0 - u_size) / 2.0;
                                            inst.uv_rect = [u_start, uv_shift, u_size, uv_zoom];
                                        }
                                    }

                                    zenthra_core::ObjectFit::Cover => {
                                        // Crop the image so it fills the box with no letterboxing.
                                        // Draw box stays the same size; UV window is narrowed.
                                        if img_aspect > box_aspect {
                                            // Image is wider than the box → crop left/right
                                            let scale = inst.size[1] / img_h; // scale to fit height
                                            let drawn_w = img_w * scale; // how wide it would be at that scale
                                            let u_size = inst.size[0] / drawn_w; // fraction of image width visible
                                            let u_start = (1.0 - u_size) / 2.0; // centre the crop
                                            inst.uv_rect = [u_start, 0.0, u_size, 1.0];
                                        } else if img_aspect < box_aspect {
                                            // Image is taller than the box → crop top/bottom
                                            let scale = inst.size[0] / img_w; // scale to fit width
                                            let drawn_h = img_h * scale; // how tall it would be at that scale
                                            let v_size = inst.size[1] / drawn_h; // fraction of image height visible
                                            let v_start = (1.0 - v_size) / 2.0; // centre the crop
                                            inst.uv_rect = [0.0, v_start, 1.0, v_size];
                                        } else {
                                            // Perfect match — no cropping required
                                            inst.uv_rect = [0.0, 0.0, 1.0, 1.0];
                                        }
                                    }

                                    zenthra_core::ObjectFit::None => {
                                        // Display the image at its natural pixel size (no scaling).
                                        // If the image is larger than the box, crop it (centred).
                                        // If smaller, shrink the draw box to match (centred).

                                        // --- Horizontal axis ---
                                        let (u_start, u_size) = if img_w > inst.size[0] {
                                            // Image wider than box → crop: show only the centre strip
                                            let u_size = inst.size[0] / img_w;
                                            let u_start = (1.0 - u_size) / 2.0;
                                            (u_start, u_size)
                                        } else {
                                            // Image narrower than box → shrink the draw box
                                            let dw = (inst.size[0] - img_w) / 2.0;
                                            inst.pos[0] += dw;
                                            inst.size[0] = img_w;
                                            (0.0, 1.0)
                                        };

                                        // --- Vertical axis ---
                                        let (v_start, v_size) = if img_h > inst.size[1] {
                                            // Image taller than box → crop: show only the centre strip
                                            let v_size = inst.size[1] / img_h;
                                            let v_start = (1.0 - v_size) / 2.0;
                                            (v_start, v_size)
                                        } else {
                                            // Image shorter than box → shrink the draw box
                                            let dh = (inst.size[1] - img_h) / 2.0;
                                            inst.pos[1] += dh;
                                            inst.size[1] = img_h;
                                            (0.0, 1.0)
                                        };

                                        inst.uv_rect = [u_start, v_start, u_size, v_size];
                                    }

                                    _ => {
                                        inst.uv_rect = [0.0, 0.0, 1.0, 1.0];
                                    }
                                }

                                image_draw_calls.push((bg.clone(), inst));
                            }
                        }
                        DrawCommand::Text(td) => {
                            let mut scaled_options = td.options.clone();
                            scaled_options.scale_factor = sf;

                            // Scale the current command's clip rect from logical to physical
                            let clip = td.clip;
                            scaled_options.clip_rect =
                                Some([clip[0] * sf, clip[1] * sf, clip[2] * sf, clip[3] * sf]);

                            engine.draw(queue, &td.text, td.pos, &scaled_options);
                        }
                        DrawCommand::OverlayRect(od) => {
                            engine.draw_rect(
                                [od.x * sf, od.y * sf],
                                [od.width * sf, od.height * sf],
                                od.color,
                                [
                                    od.clip[0] * sf,
                                    od.clip[1] * sf,
                                    od.clip[2] * sf,
                                    od.clip[3] * sf,
                                ],
                            );
                        }
                    }
                }

                rp.prepare(&device, queue, width, height, &rect_instances);

                // Upload image instances to buffers
                for (bg, inst) in image_draw_calls {
                    use wgpu::util::DeviceExt;
                    let buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Image Instance"),
                        contents: bytemuck::bytes_of(&inst),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                    image_buffers.push((bg, buf));
                }
            }

            // Swap layout caches for next frame
            layout_cache = next_layout_cache;

            let surface_texture = match frame.window.gpu.surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(t) => t,
                wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
                _ => return false,
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

                for (bg, buf) in &image_buffers {
                    ip.draw(&mut pass, bg, buf, 1);
                }

                engine.render(&mut pass);
            }

            queue.submit(std::iter::once(encoder.finish()));
            surface_texture.present();
            needs_redraw
        });
        self
    }

    pub fn run(self) {
        self.platform.run();
    }
}
