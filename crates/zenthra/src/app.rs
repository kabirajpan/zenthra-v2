use zenthra_platform::app::{App as PlatformApp, Frame};
use zenthra_render::{RectPipeline, BlurPipeline, BlurScratch, BlitPipeline, run_kawase_blur};
use zenthra_text::prelude::*;
use zenthra_widgets::ui::DrawCommand;
use zenthra_widgets::Ui;

pub struct App {
    platform: PlatformApp,
    fonts: Vec<String>,
    font_data: Vec<Vec<u8>>,
    custom_shaders: Vec<(&'static str, &'static str)>,
    backdrop_tint: Option<zenthra_core::Color>,
    backdrop_opacity: Option<f32>,
    backdrop_filter: Option<zenthra_core::BackdropFilter>,
}

impl App {
    pub fn new() -> Self {
        Self {
            platform: PlatformApp::new(),
            fonts: Vec::new(),
            font_data: Vec::new(),
            custom_shaders: Vec::new(),
            backdrop_tint: None,
            backdrop_opacity: None,
            backdrop_filter: None,
        }
    }

    pub fn register_custom_shader(mut self, id: &'static str, source: &'static str) -> Self {
        self.custom_shaders.push((id, source));
        self
    }

    pub fn load_font_path(mut self, path: &str) -> Self {
        self.fonts.push(path.to_string());
        self
    }

    pub fn load_font_data(mut self, data: Vec<u8>) -> Self {
        self.font_data.push(data);
        self
    }

    pub fn title(mut self, t: &str) -> Self {
        self.platform = self.platform.title(t);
        self
    }
    pub fn size(mut self, w: u32, h: u32) -> Self {
        self.platform = self.platform.size(w, h);
        self
    }
    pub fn decorations(mut self, dec: bool) -> Self {
        self.platform = self.platform.decorations(dec);
        self
    }
    pub fn transparent(mut self, trans: bool) -> Self {
        self.platform = self.platform.transparent(trans);
        self
    }
    pub fn blur(mut self, blur_state: bool) -> Self {
        self.platform = self.platform.blur(blur_state);
        self
    }
    pub fn backdrop_tint(mut self, tint: zenthra_core::Color) -> Self {
        self.backdrop_tint = Some(tint);
        if tint.a < 1.0 {
            self.platform = self.platform.transparent(true);
        }
        self
    }
    pub fn bg(mut self, color: zenthra_core::Color) -> Self {
        self.backdrop_tint = Some(color);
        if color.a < 1.0 {
            self.platform = self.platform.transparent(true);
        }
        self
    }
    pub fn bg_opacity(mut self, opacity: f32) -> Self {
        self.backdrop_opacity = Some(opacity);
        if opacity < 1.0 {
            self.platform = self.platform.transparent(true);
        }
        self
    }
    pub fn backdrop_filter(mut self, filter: zenthra_core::BackdropFilter) -> Self {
        self.backdrop_filter = Some(filter.clone());
        let mut transparent = false;
        let mut blur = false;
        for f in &filter.filters {
            match f {
                zenthra_core::Filter::Blur(_, _) => {
                    blur = true;
                    transparent = true;
                }
                zenthra_core::Filter::Opacity(alpha) => {
                    if *alpha < 1.0 {
                        transparent = true;
                    }
                }
                _ => {}
            }
        }
        if transparent {
            self.platform = self.platform.transparent(true);
        }
        if blur {
            self.platform = self.platform.blur(true);
        }
        self
    }

    pub fn with_ui<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Ui) + 'static,
    {
        let mut rect_pipeline: Option<RectPipeline> = None;
        let mut image_pipeline: Option<zenthra_render::ImagePipeline> = None;
        let mut blur_pipeline:  Option<BlurPipeline> = None;
        let mut blit_pipeline:  Option<BlitPipeline> = None;
        let mut blur_scratch:   Option<BlurScratch> = None;
        let mut texture_cache: std::collections::HashMap<
            zenthra_core::ImageSource,
            (std::sync::Arc<wgpu::BindGroup>, u32, u32),
        > = std::collections::HashMap::new();
        let mut image_sizes: std::collections::HashMap<zenthra_core::ImageSource, (u32, u32)> =
            std::collections::HashMap::new();
        let mut texture_lru: std::collections::VecDeque<zenthra_core::ImageSource> =
            std::collections::VecDeque::new();
        let mut loading_textures: std::collections::HashSet<zenthra_core::ImageSource> =
            std::collections::HashSet::new();
        let (tx, rx) = std::sync::mpsc::channel::<(
            zenthra_core::ImageSource,
            Result<(wgpu::BindGroup, u32, u32), String>,
        )>();
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
        let mut screen_layout_cache: std::collections::HashMap<
            zenthra_core::Id,
            zenthra_core::Rect,
        > = std::collections::HashMap::new();
        let mut widget_window_map: std::collections::HashMap<
            zenthra_core::Id,
            zenthra_core::Id,
        > = std::collections::HashMap::new();
        let start_time = std::time::Instant::now();

        let fonts = std::mem::take(&mut self.fonts);
        let font_data_list = std::mem::take(&mut self.font_data);
        let custom_shaders_list = std::mem::take(&mut self.custom_shaders);
        let custom_shaders_map: std::collections::HashMap<&'static str, &'static str> =
            custom_shaders_list.into_iter().collect();
        let mut custom_pipelines: std::collections::HashMap<
            &'static str,
            wgpu::RenderPipeline,
        > = std::collections::HashMap::new();

        let is_transparent = self.platform.is_transparent();
        let backdrop_tint = self.backdrop_tint;
        let backdrop_opacity = self.backdrop_opacity;
        let backdrop_filter = self.backdrop_filter.clone();
        self.platform = self.platform.with_ui(move |frame: &mut Frame| {
            let elapsed = start_time.elapsed().as_secs_f32();
            let device = frame.window.gpu.device.clone();
            let queue = &frame.window.gpu.queue;
            let config = &frame.window.gpu.config;
            let width = frame.window.width();
            let height = frame.window.height();
            let sf = frame.scale_factor() as f32;
            let mut next_layout_cache = std::collections::HashMap::new();
            let mut next_screen_layout_cache = std::collections::HashMap::new();
            let mut next_widget_window_map = std::collections::HashMap::new();

            // Initialize or update the Zentype engine
            let engine = zentype.get_or_insert_with(|| {
                let engine = Zentype::new(device.clone(), queue, config);
                for font_path in &fonts {
                    if let Err(e) = engine.font_system().lock().unwrap().db_mut().load_font_file(font_path) {
                        eprintln!("Failed to load custom font {}: {:?}", font_path, e);
                    }
                }
                for data in &font_data_list {
                    engine.font_system().lock().unwrap().db_mut().load_font_data(data.clone());
                }
                engine
            });

            let mut needs_redraw = false;

            // Process async loaded textures
            while let Ok((source, result)) = rx.try_recv() {
                loading_textures.remove(&source);
                match result {
                    Ok((bg, w, h)) => {
                        let is_thumbnail = matches!(source, zenthra_core::ImageSource::Thumbnail(_));
                        if is_thumbnail {
                            let thumb_count = texture_cache.keys().filter(|k| matches!(k, zenthra_core::ImageSource::Thumbnail(_))).count();
                            if thumb_count >= 1000 {
                                if let Some(idx) = texture_lru.iter().position(|k| matches!(k, zenthra_core::ImageSource::Thumbnail(_))) {
                                    if let Some(lru_key) = texture_lru.remove(idx) {
                                        texture_cache.remove(&lru_key);
                                        image_sizes.remove(&lru_key);
                                    }
                                }
                            }
                        } else {
                            let full_count = texture_cache.keys().filter(|k| !matches!(k, zenthra_core::ImageSource::Thumbnail(_))).count();
                            if full_count >= 256 {
                                if let Some(idx) = texture_lru.iter().position(|k| !matches!(k, zenthra_core::ImageSource::Thumbnail(_))) {
                                    if let Some(lru_key) = texture_lru.remove(idx) {
                                        texture_cache.remove(&lru_key);
                                        image_sizes.remove(&lru_key);
                                    }
                                }
                            }
                        }
                        texture_cache.insert(source.clone(), (std::sync::Arc::new(bg), w, h));
                        image_sizes.insert(source.clone(), (w, h));
                        
                        if let Some(pos) = texture_lru.iter().position(|x| *x == source) {
                            texture_lru.remove(pos);
                        }
                        texture_lru.push_back(source);
                        needs_redraw = true;
                    }
                    Err(e) => {
                        eprintln!("Background texture load failed: {}", e);
                    }
                }
            }

            // Update persistent mouse pos from current frame events
            let mut ui_clicked = false;
            let mut ui_right_clicked = false;
            for event in frame.events {
                match event {
                    zenthra_platform::event::PlatformEvent::MouseMoved { x, y } => {
                        mouse_pos = (*x as f32 / sf, *y as f32 / sf);
                        needs_redraw = true;
                    }
                    zenthra_platform::event::PlatformEvent::MouseButton { button, state } => {
                        let was_down = ui_mouse_down;
                        ui_mouse_down = *state == winit::event::ElementState::Pressed;
                        if ui_mouse_down && !was_down {
                            ui_clicked = true;
                            if *button == winit::event::MouseButton::Right {
                                ui_right_clicked = true;
                            }
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

            // Lazily create / resize blur & blit pipelines + scratch textures
            let bp = blur_pipeline.get_or_insert_with(|| BlurPipeline::new(&device, config.format));
            let blitp = blit_pipeline.get_or_insert_with(|| BlitPipeline::new(&device, config.format));
            let scratch = {
                let needs_new = blur_scratch.as_ref().map_or(true, |s| s.needs_resize(width, height));
                if needs_new {
                    blur_scratch = Some(BlurScratch::new(&device, width, height, config.format));
                }
                blur_scratch.as_ref().unwrap()
            };

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
                        ui_right_clicked,
                        elapsed,
                        &layout_cache,
                        &mut next_layout_cache,
                        &screen_layout_cache,
                        &mut next_screen_layout_cache,
                        &widget_window_map,
                        &mut next_widget_window_map,
                        &image_sizes,
                    );

                    f(&mut ui);

                    // Clear modal flags for any window that was not shown in this frame
                    let shown_windows: std::collections::HashSet<zenthra_core::Id> = ui.window_overlays.iter().map(|(id, _)| *id).collect();
                    let mut keys_to_clear = Vec::new();
                    for (&key, &val) in ui.interaction_state.iter() {
                        let raw = key.raw();
                        if (raw & 0xFF) == 5 && val > 0.5 {
                            let win_id = zenthra_core::Id::from_u64(raw >> 8);
                            if !shown_windows.contains(&win_id) {
                                keys_to_clear.push(key);
                            }
                        }
                    }
                    for key in keys_to_clear {
                        ui.interaction_state.insert(key, 0.0);
                    }

                    // Sort window overlays by z-index and append them to ui.overlays
                    ui.window_overlays.sort_by(|(id_a, _), (id_b, _)| {
                        let z_a = ui.interaction_state.get(&zenthra_core::Id::from_u64((id_a.raw() << 8) | 4)).copied().unwrap_or(0.0);
                        let z_b = ui.interaction_state.get(&zenthra_core::Id::from_u64((id_b.raw() << 8) | 4)).copied().unwrap_or(0.0);
                        z_a.partial_cmp(&z_b).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    for (_, cmds) in ui.window_overlays {
                        ui.overlays.extend(cmds);
                    }

                    focused_id = ui.focused_id;
                    active_drag = ui.active_drag;
                    needs_redraw |= ui.needs_redraw;
                    frame.request_redraw_at = ui.requested_redraw_at;
                    frame.window_actions = ui.window_actions.clone();

                    let winit_cursor = match ui.cursor_icon {
                        zenthra_widgets::text::CursorIcon::Default => winit::window::CursorIcon::Default,
                        zenthra_widgets::text::CursorIcon::Text => winit::window::CursorIcon::Text,
                        zenthra_widgets::text::CursorIcon::Pointer => winit::window::CursorIcon::Pointer,
                        zenthra_widgets::text::CursorIcon::Crosshair => winit::window::CursorIcon::Crosshair,
                        zenthra_widgets::text::CursorIcon::ColResize => winit::window::CursorIcon::ColResize,
                    };
                    frame.window.winit_window.set_cursor(winit_cursor);
                    
                    (ui.draws, ui.overlays)
                };

                // ── Acquire surface + encoder ─────────────────────────────────
                let surface_texture = match frame.window.gpu.surface.get_current_texture() {
                    wgpu::CurrentSurfaceTexture::Success(t) => t,
                    _ => return false,
                };
                let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Zenthra Frame") });

                // ── OFFSCREEN TEXTURE ──────────────────────────────────────────
                // All rendering targets scratch.full_a (RENDER_ATTACHMENT | COPY_SRC).
                // This lets us snapshot the scene for backdrop blur without touching the surface.
                let offscreen_view = &scratch.full_a_v;

                // ── PASS 1: CLEAR offscreen ────────────────────────────────────
                let clear_color = if let Some(tint) = backdrop_tint {
                    let mut alpha = tint.a;
                    if let Some(op) = backdrop_opacity {
                        alpha = op;
                    } else if let Some(ref filter) = backdrop_filter {
                        for f in &filter.filters {
                            if let zenthra_core::Filter::Opacity(a) = f {
                                alpha = tint.a * a;
                            }
                        }
                    }
                    wgpu::Color {
                        r: tint.r as f64,
                        g: tint.g as f64,
                        b: tint.b as f64,
                        a: alpha as f64,
                    }
                } else if is_transparent {
                    wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    }
                } else {
                    wgpu::Color {
                        r: 0.06,
                        g: 0.06,
                        b: 0.08,
                        a: 1.0,
                    }
                };
                {
                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Clear Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: offscreen_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(clear_color),
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

                rp.prepare(&device, queue, width, height, &[]);

                let mut temp_buffers: Vec<wgpu::Buffer> = Vec::new();

                // ── Ordered draw loop (handles BackdropBlur in-order) ──────────
                // Batches rect/image/text commands as before, but flushes them before
                // any BackdropBlur and runs the Kawase pass in between.
                let flush_rects = |encoder: &mut wgpu::CommandEncoder,
                                   dst: &wgpu::TextureView,
                                   instances: &[zenthra_render::RectInstance],
                                   temp_bufs: &mut Vec<wgpu::Buffer>| {
                    if instances.is_empty() { return; }
                    use wgpu::util::DeviceExt;
                    let buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Rect Batch"),
                        contents: bytemuck::cast_slice(instances),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                    {
                        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Rect Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: dst,
                                resolve_target: None,
                                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
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
                        pass.draw(0..6, 0..instances.len() as u32);
                    }
                    temp_bufs.push(buf);
                };

                let mut process_commands = |encoder: &mut wgpu::CommandEncoder,
                                        dst: &wgpu::TextureView,
                                        cmds: &[DrawCommand],
                                        temp_bufs: &mut Vec<wgpu::Buffer>| {
                    let mut pending_rects: Vec<zenthra_render::RectInstance> = Vec::new();
                    let mut pending_images: Vec<(&zenthra_widgets::ui::ImageDraw, wgpu::Buffer)> = Vec::new();
                    let mut pending_texts: Vec<&zenthra_widgets::ui::TextDraw> = Vec::new();
                    let mut pending_overlay_rects: Vec<zenthra_render::RectInstance> = Vec::new();

                    // Flush all accumulated batches in correct painter's order:
                    // rects → images → text → overlay rects
                    let mut flush_all = |encoder: &mut wgpu::CommandEncoder,
                                     pending_rects: &mut Vec<zenthra_render::RectInstance>,
                                     pending_images: &mut Vec<(&zenthra_widgets::ui::ImageDraw, wgpu::Buffer)>,
                                     pending_texts: &mut Vec<&zenthra_widgets::ui::TextDraw>,
                                     pending_overlay_rects: &mut Vec<zenthra_render::RectInstance>,
                                     temp_bufs: &mut Vec<wgpu::Buffer>| {
                        flush_rects(encoder, dst, pending_rects, temp_bufs);
                        pending_rects.clear();

                        // Images
                        if !pending_images.is_empty() {
                            {
                                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: Some("Image Pass"),
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: dst,
                                        resolve_target: None,
                                        ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                                        depth_slice: None,
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                    multiview_mask: None,
                                });
                                for (id_cmd, buf) in pending_images.iter() {
                                    if let Some((bg, _, _)) = texture_cache.get(&id_cmd.source) {
                                        ip.draw(&mut pass, bg, buf, 1);
                                    }
                                }
                            }
                            for (_, buf) in pending_images.drain(..) {
                                temp_bufs.push(buf);
                            }
                        }

                        // Text
                        if !pending_texts.is_empty() {
                            for td in pending_texts.drain(..) {
                                let mut opts = td.options.clone();
                                opts.scale_factor = sf;
                                opts.clip_rect = Some([td.clip[0]*sf, td.clip[1]*sf, td.clip[2]*sf, td.clip[3]*sf]);
                                engine.draw(queue, &td.text, td.pos, &opts);
                            }
                            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("Text Pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: dst,
                                    resolve_target: None,
                                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                                    depth_slice: None,
                                })],
                                depth_stencil_attachment: None,
                                timestamp_writes: None,
                                occlusion_query_set: None,
                                multiview_mask: None,
                            });
                            engine.render(&mut pass);
                        }

                        // Overlay rects
                        flush_rects(encoder, dst, pending_overlay_rects, temp_bufs);
                        pending_overlay_rects.clear();
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
                                pending_rects.push(inst);
                            }
                            DrawCommand::Image(id_cmd) => {
                                // Kick off async load if needed
                                if !texture_cache.contains_key(&id_cmd.source) {
                                    if !loading_textures.contains(&id_cmd.source) {
                                        loading_textures.insert(id_cmd.source.clone());
                                        let tx2 = tx.clone();
                                        let device2 = device.clone();
                                        let queue2 = frame.window.gpu.queue.clone();
                                        let bgl2 = ip.texture_bgl.clone();
                                        let source2 = id_cmd.source.clone();
                                        let winit2 = frame.window.winit_window.clone();
                                        std::thread::spawn(move || {
                                            let result = match &source2 {
                                                zenthra_core::ImageSource::Path(p) => {
                                                    std::fs::read(p).map_err(|e| format!("{e:?}")).and_then(|bytes|
                                                        zenthra_render::texture::create_texture_bind_group(&device2, &queue2, &bgl2, &bytes).map_err(|e| format!("{e:?}")))
                                                }
                                                zenthra_core::ImageSource::Thumbnail(p) => {
                                                    std::fs::read(p).map_err(|e| format!("{e:?}")).and_then(|bytes|
                                                        zenthra_render::texture::create_texture_bind_group_thumbnail(&device2, &queue2, &bgl2, &bytes, 200).map_err(|e| format!("{e:?}")))
                                                }
                                                zenthra_core::ImageSource::Bytes(b) => {
                                                    zenthra_render::texture::create_texture_bind_group(&device2, &queue2, &bgl2, b).map_err(|e| format!("{e:?}"))
                                                }
                                            };
                                            let _ = tx2.send((source2, result));
                                            winit2.request_redraw();
                                        });
                                    }
                                } else {
                                    if let Some(pos) = texture_lru.iter().position(|x| *x == id_cmd.source) {
                                        texture_lru.remove(pos);
                                    }
                                    texture_lru.push_back(id_cmd.source.clone());
                                    if texture_cache.contains_key(&id_cmd.source) {
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
                                        pending_images.push((id_cmd, buf));
                                    }
                                }
                            }
                            DrawCommand::Text(td) => {
                                pending_texts.push(td);
                            }
                            DrawCommand::OverlayRect(od) => {
                                let inst = zenthra_render::RectInstance {
                                    pos: [od.x * sf, od.y * sf],
                                    size: [od.width * sf, od.height * sf],
                                    color: od.color.to_array(),
                                    clip_rect: [od.clip[0]*sf, od.clip[1]*sf, od.clip[2]*sf, od.clip[3]*sf],
                                    ..Default::default()
                                };
                                pending_overlay_rects.push(inst);
                            }
                            DrawCommand::BackdropBlur(bd) => {
                                // ── Flush everything drawn so far ─────────────
                                flush_all(encoder,
                                    &mut pending_rects, &mut pending_images,
                                    &mut pending_texts, &mut pending_overlay_rects,
                                    temp_bufs);

                                // ── Execute requested blur pass: full_a → mips → full_b ──
                                run_kawase_blur(
                                    bp, scratch, &device, encoder,
                                    &scratch.full_a_v,   // read scene
                                    &scratch.full_b_v,   // write blurred
                                    bd.blur_radius * sf,
                                );

                                // ── Composite blurred region onto full_a ──────
                                let rect_pos = [bd.x * sf, bd.y * sf];
                                let rect_size = [bd.width * sf, bd.height * sf];
                                let radius = [
                                    bd.radius[0] * sf,
                                    bd.radius[1] * sf,
                                    bd.radius[2] * sf,
                                    bd.radius[3] * sf,
                                ];
                                let btype_val = match bd.blur_type {
                                    zenthra_core::style::blur::Type::Normal => 0.0,
                                    zenthra_core::style::blur::Type::Frosted => 1.0,
                                    zenthra_core::style::blur::Type::Glassmorphism => 2.0,
                                    zenthra_core::style::blur::Type::OpaqueGlass => 3.0,
                                };
                                blitp.blit_to_clipped(
                                    &device, encoder,
                                    &scratch.full_b_v,
                                    dst,
                                    rect_pos,
                                    rect_size,
                                    radius,
                                    scratch.width,
                                    scratch.height,
                                    bd.brightness,
                                    bd.saturation,
                                    bd.contrast,
                                    btype_val,
                                    bd.opacity,
                                );
                            }
                            DrawCommand::CustomPostProcess(cp) => {
                                // ── Flush everything drawn so far ─────────────
                                flush_all(encoder,
                                    &mut pending_rects, &mut pending_images,
                                    &mut pending_texts, &mut pending_overlay_rects,
                                    temp_bufs);

                                // ── If blur_radius > 0.0, run Kawase blur ────
                                let src_view = if cp.blur_radius > 0.0 {
                                    run_kawase_blur(
                                        bp, scratch, &device, encoder,
                                        &scratch.full_a_v,
                                        &scratch.full_b_v,
                                        cp.blur_radius * sf,
                                    );
                                    &scratch.full_b_v
                                } else {
                                     // Copy full_a to full_b so we sample from full_b and write to full_a (avoiding conflicting usages)
                                     encoder.copy_texture_to_texture(
                                         wgpu::TexelCopyTextureInfo {
                                             texture: &scratch.full_a,
                                             mip_level: 0,
                                             origin: wgpu::Origin3d::ZERO,
                                             aspect: wgpu::TextureAspect::All,
                                         },
                                         wgpu::TexelCopyTextureInfo {
                                             texture: &scratch.full_b,
                                             mip_level: 0,
                                             origin: wgpu::Origin3d::ZERO,
                                             aspect: wgpu::TextureAspect::All,
                                         },
                                         wgpu::Extent3d {
                                             width: scratch.width,
                                             height: scratch.height,
                                             depth_or_array_layers: 1,
                                         },
                                     );
                                     &scratch.full_b_v
                                 };

                                // ── Compile custom shader if needed ───────────
                                let shader_src = custom_shaders_map.get(cp.shader_id).copied();
                                if let Some(src) = shader_src {
                                    let pipeline = custom_pipelines.entry(cp.shader_id).or_insert_with(|| {
                                        let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                                             label: Some("Custom PostProcess VS"),
                                             source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                                                 r#"
                                                 struct VsOut {
                                                     @builtin(position) pos: vec4<f32>,
                                                     @location(0)       uv:  vec2<f32>,
                                                 }
                                                 @vertex
                                                 fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
                                                     var positions = array<vec2<f32>, 3>(
                                                         vec2<f32>(-1.0, -1.0),
                                                         vec2<f32>( 3.0, -1.0),
                                                         vec2<f32>(-1.0,  3.0),
                                                     );
                                                     var uvs = array<vec2<f32>, 3>(
                                                         vec2<f32>(0.0, 1.0),
                                                         vec2<f32>(2.0, 1.0),
                                                         vec2<f32>(0.0, -1.0),
                                                     );
                                                     var out: VsOut;
                                                     out.pos = vec4<f32>(positions[vi], 0.0, 1.0);
                                                     out.uv  = uvs[vi];
                                                     return out;
                                                 }
                                                 "#
                                             )),
                                         });
                                        let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                                            label: Some(cp.shader_id),
                                            source: wgpu::ShaderSource::Wgsl(src.into()),
                                        });

                                        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                                            label: Some("Custom PostProcess Pipeline Layout"),
                                            bind_group_layouts: &[Some(&blitp.bgl), Some(&blitp.backdrop_bgl)],
                                            immediate_size: 0,
                                        });

                                        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                                            label: Some(cp.shader_id),
                                            layout: Some(&layout),
                                            vertex: wgpu::VertexState {
                                                module: &vs_module,
                                                entry_point: Some("vs_backdrop"),
                                                buffers: &[],
                                                compilation_options: Default::default(),
                                            },
                                            fragment: Some(wgpu::FragmentState {
                                                module: &fs_module,
                                                entry_point: Some("fs_main"),
                                                targets: &[Some(wgpu::ColorTargetState {
                                                    format: config.format,
                                                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                                                    write_mask: wgpu::ColorWrites::ALL,
                                                })],
                                                compilation_options: Default::default(),
                                            }),
                                            primitive: wgpu::PrimitiveState::default(),
                                            depth_stencil: None,
                                            multisample: wgpu::MultisampleState::default(),
                                            multiview_mask: None,
                                            cache: None,
                                        })
                                    });

                                    // ── Composite with custom shader ─────────────
                                    let rect_pos = [cp.x * sf, cp.y * sf];
                                    let rect_size = [cp.width * sf, cp.height * sf];
                                    let radius = [
                                        cp.radius[0] * sf,
                                        cp.radius[1] * sf,
                                        cp.radius[2] * sf,
                                        cp.radius[3] * sf,
                                    ];

                                    // Render pass setup (same as blit_to_clipped but using custom pipeline)
                                    use wgpu::util::DeviceExt;
                                    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
                                        label: Some("Custom Blit Clipped BG"),
                                        layout: &blitp.bgl,
                                        entries: &[
                                            wgpu::BindGroupEntry {
                                                binding: 0,
                                                resource: wgpu::BindingResource::TextureView(src_view),
                                            },
                                            wgpu::BindGroupEntry {
                                                binding: 1,
                                                resource: wgpu::BindingResource::Sampler(&blitp.sampler),
                                            },
                                        ],
                                    });

                                    let uniforms = zenthra_render::BackdropUniforms {
                                         radius,
                                         rect_pos,
                                         rect_size,
                                         screen_size: [scratch.width as f32, scratch.height as f32],
                                         time: elapsed,
                                         brightness: 1.0,
                                         saturation: 1.0,
                                         contrast: 1.0,
                                         blur_type: 0.0,
                                         opacity: 1.0, padding: [cp.blur_radius, 0.0], _end_padding: [0.0; 2],
                                     };

                                    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                        label: Some("Custom Blit Uniform Buffer"),
                                        contents: bytemuck::bytes_of(&uniforms),
                                        usage: wgpu::BufferUsages::UNIFORM,
                                    });

                                    let uniform_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
                                        label: Some("Custom Blit Uniform BG"),
                                        layout: &blitp.backdrop_bgl,
                                        entries: &[
                                            wgpu::BindGroupEntry {
                                                binding: 0,
                                                resource: uniform_buf.as_entire_binding(),
                                            },
                                        ],
                                    });

                                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                        label: Some("Custom Blit Clipped Pass"),
                                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                            view: dst,
                                            resolve_target: None,
                                            ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                                            depth_slice: None,
                                        })],
                                        depth_stencil_attachment: None,
                                        timestamp_writes: None,
                                        occlusion_query_set: None,
                                        multiview_mask: None,
                                    });

                                    pass.set_pipeline(pipeline);
                                    pass.set_bind_group(0, &bg, &[]);
                                    pass.set_bind_group(1, &uniform_bg, &[]);
                                    pass.draw(0..6, 0..1);
                                }
                            }
                        }
                    }

                    // Final flush of any remaining batches
                    flush_all(encoder,
                        &mut pending_rects, &mut pending_images,
                        &mut pending_texts, &mut pending_overlay_rects,
                        temp_bufs);
                };

                process_commands(&mut encoder, offscreen_view, &draws, &mut temp_buffers);
                process_commands(&mut encoder, offscreen_view, &overlays, &mut temp_buffers);

                // ── FINAL: Blit offscreen → surface ────────────────────────────
                blitp.blit_to(
                    &device, &mut encoder,
                    offscreen_view,
                    &view,
                    wgpu::LoadOp::Clear(clear_color),
                );

                queue.submit(std::iter::once(encoder.finish()));
                surface_texture.present();
            }

            // Compare layout caches to see if layout changed
            let mut layout_changed = false;
            if layout_cache.len() != next_layout_cache.len() {
                layout_changed = true;
            } else {
                for (id, (rect, _)) in &next_layout_cache {
                    if let Some((old_rect, _)) = layout_cache.get(id) {
                        if (old_rect.origin.x - rect.origin.x).abs() > 0.1 ||
                           (old_rect.origin.y - rect.origin.y).abs() > 0.1 ||
                           (old_rect.size.width - rect.size.width).abs() > 0.1 ||
                           (old_rect.size.height - rect.size.height).abs() > 0.1 {
                            layout_changed = true;
                            break;
                        }
                    } else {
                        layout_changed = true;
                        break;
                    }
                }
            }

            if layout_changed {
                needs_redraw = true;
            }

            // Swap layout caches for next frame
            layout_cache = next_layout_cache;
            screen_layout_cache = next_screen_layout_cache;
            widget_window_map = next_widget_window_map;
            needs_redraw
        });
        self
    }

    pub fn run(self) {
        self.platform.run();
    }

    pub fn run_with_event_loop(self, event_loop: winit::event_loop::EventLoop<()>) {
        self.platform.run_with_event_loop(event_loop);
    }
}
