// crates/zenthra-widgets/src/window.rs

use crate::ui::{Ui, DrawCommand};
use zenthra_core::{Color, Id, Response, Align};

pub struct FloatingWindowBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    id: Id,
    title: String,
    is_open: &'b mut bool,
    pos: &'b mut [f32; 2],
    
    // Styling
    width: f32,
    height: f32,
    bg: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,
    shadow_color: Color,
    shadow_blur: f32,
    shadow_offset: [f32; 2],
    shadow_opacity: f32,

    // Header Styling
    header_bg: Color,
    header_text_color: Color,
    header_height: f32,
    
    closable: bool,
    modal: bool,
    light_dismiss: bool,
    backdrop_filter: Option<zenthra_core::BackdropFilter>,
}

impl<'u, 'a, 'b> FloatingWindowBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>, title: &str, is_open: &'b mut bool, pos: &'b mut [f32; 2]) -> Self {
        let id = ui.id();
        Self {
            ui,
            id,
            title: title.to_string(),
            is_open,
            pos,
            
            width: 320.0,
            height: 400.0,
            bg: Color::rgb(0.1, 0.1, 0.12),
            border_color: Color::rgb(0.3, 0.3, 0.35),
            border_width: 1.0,
            radius: 8.0,
            shadow_color: Color::BLACK,
            shadow_blur: 20.0,
            shadow_offset: [0.0, 10.0],
            shadow_opacity: 0.5,

            header_bg: Color::rgb(0.15, 0.15, 0.18),
            header_text_color: Color::WHITE,
            header_height: 40.0,
            
            closable: true,
            modal: false,
            light_dismiss: false,
            backdrop_filter: None,
        }
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn light_dismiss(mut self, dismiss: bool) -> Self {
        self.light_dismiss = dismiss;
        self
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = w;
        self.height = h;
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    pub fn backdrop_filter(mut self, filter: zenthra_core::BackdropFilter) -> Self {
        self.backdrop_filter = Some(filter);
        self
    }

    pub fn border(mut self, border_color: Color, border_width: f32) -> Self {
        self.border_color = border_color;
        self.border_width = border_width;
        self
    }

    pub fn radius_all(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn header_bg(mut self, bg: Color) -> Self {
        self.header_bg = bg;
        self
    }

    pub fn header_text_color(mut self, color: Color) -> Self {
        self.header_text_color = color;
        self
    }

    pub fn header_height(mut self, height: f32) -> Self {
        self.header_height = height;
        self
    }

    pub fn show<F>(self, content: F) -> Response 
    where F: FnOnce(&mut Ui)
    {
        let id = self.id;
        let z_key = Id::from_u64((id.raw() << 8) | 4);
        let modal_key = Id::from_u64((id.raw() << 8) | 5);
        let opened_key = Id::from_u64((id.raw() << 8) | 6);

        if !*self.is_open {
            self.ui.interaction_state.remove(&z_key);
            self.ui.interaction_state.remove(&modal_key);
            self.ui.interaction_state.remove(&opened_key);
            return Response { clicked: false, hovered: false, pressed: false };
        }

        let drag_id = Id::from_u64((id.raw() << 8) | 1);

        // If the window was just opened, promote it to the top z-index
        if !self.ui.interaction_state.contains_key(&z_key) {
            let max_z_key = Id::from_u64(999999999);
            let max_z = self.ui.interaction_state.get(&max_z_key).copied().unwrap_or(0.0);
            let new_z = max_z + 1.0;
            self.ui.interaction_state.insert(max_z_key, new_z);
            self.ui.interaction_state.insert(z_key, new_z);
            self.ui.needs_redraw = true;
        }

        // Store modal state
        self.ui.interaction_state.insert(modal_key, if self.modal { 1.0 } else { 0.0 });

        let mut is_dragging = self.ui.interaction_state.get(&drag_id).map(|&v| v > 0.5).unwrap_or(false);

        // Window position
        let win_x = self.pos[0];
        let win_y = self.pos[1];

        // Is hovered check (uses occlusion detection internally)
        let is_hovered = self.ui.is_hovered(id, win_x, win_y, self.width, self.height);

        // Check if the window was already open in the previous frame
        let was_already_open = self.ui.interaction_state.insert(opened_key, 1.0).is_some();

        {
            use std::fs::OpenOptions;
            use std::io::Write;
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("window_debug.log") {
                let _ = writeln!(
                    file,
                    "WINDOW ID: {:?} | is_open: {} | clicked: {} | is_hovered: {} | was_already_open: {}",
                    id, *self.is_open, self.ui.clicked, is_hovered, was_already_open
                );
            }
        }

        // Light dismiss logic
        if self.light_dismiss && self.ui.clicked && !is_hovered && was_already_open {
            *self.is_open = false;
            {
                use std::fs::OpenOptions;
                use std::io::Write;
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("window_debug.log") {
                    let _ = writeln!(file, "   --> LIGHT DISMISS TRIGGERED! Setting is_open to false.");
                }
            }
            self.ui.interaction_state.remove(&z_key);
            self.ui.interaction_state.remove(&modal_key);
            self.ui.interaction_state.remove(&opened_key);
            self.ui.needs_redraw = true;
            return Response { clicked: true, hovered: false, pressed: false };
        }

        // Active focus z-order promotion logic
        if self.ui.clicked && is_hovered {
            let max_z_key = Id::from_u64(999999999);
            let max_z = self.ui.interaction_state.get(&max_z_key).copied().unwrap_or(0.0);
            let new_z = max_z + 1.0;
            self.ui.interaction_state.insert(max_z_key, new_z);
            self.ui.interaction_state.insert(z_key, new_z);
            self.ui.needs_redraw = true;
        }

        // Capture start length of overlays
        let start_len = self.ui.overlays.len();

        let is_open = self.is_open;
        let pos = self.pos;
        let ui = self.ui;
        
        let title = self.title;
        let modal = self.modal;
        let closable = self.closable;
        let width = self.width;
        let height = self.height;
        let bg = self.bg;
        let border_color = self.border_color;
        let border_width = self.border_width;
        let radius = self.radius;
        let shadow_color = self.shadow_color;
        let shadow_offset = self.shadow_offset;
        let shadow_blur = self.shadow_blur;
        let shadow_opacity = self.shadow_opacity;
        let header_height = self.header_height;
        let header_bg = self.header_bg;
        let header_text_color = self.header_text_color;

        // Set current window ID context
        let prev_win_id = ui.current_window_id;
        ui.current_window_id = Some(id);

        ui.overlay(|ui| {
            // Draw modal backdrop if modal
            if modal {
                ui.draws.push(crate::ui::DrawCommand::Rect(crate::ui::RectDraw {
                    instance: zenthra_render::RectInstance {
                        pos: [0.0, 0.0],
                        size: [ui.width, ui.height],
                        color: Color::rgba(0.0, 0.0, 0.0, 0.4).to_array(),
                        radius: [0.0; 4],
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT.to_array(),
                        shadow_color: Color::TRANSPARENT.to_array(),
                        shadow_offset: [0.0, 0.0],
                        shadow_blur: 0.0,
                        clip_rect: [0.0, 0.0, 9999.0, 9999.0],
                        ..Default::default()
                    }
                }));
            }

            let mut container = ui.container()
                .absolute(win_x, win_y)
                .width(width)
                .height(height)
                .bg(bg)
                .border(border_color, border_width)
                .radius_all(radius)
                .shadow(shadow_color, shadow_offset[0], shadow_offset[1], shadow_blur)
                .shadow_opacity(shadow_opacity)
                .clip(true);
            if let Some(ref filter) = self.backdrop_filter {
                container = container.backdrop_filter(filter.clone());
            }
            container.show(|ui| {
                    let mut close_clicked = false;
                    let header_res = ui.container()
                        .full_width()
                        .height(header_height)
                        .bg(header_bg)
                        .padding_x(15.0)
                        .row()
                        .halign(Align::SpaceBetween)
                        .valign(Align::Center)
                        .show(|ui| {
                            // Left spacer to balance close button width for centering
                            ui.spacing(20.0);

                            ui.text(&title)
                                .color(header_text_color)
                                .bold()
                                .show();
                            
                            if closable {
                                if ui.button("×")
                                    .bg(Color::TRANSPARENT)
                                    .hover_bg(Color::rgba(1.0, 1.0, 1.0, 0.1))
                                    .text_color(header_text_color)
                                    .padding(4.0, 8.0, 4.0, 8.0)
                                    .radius_all(4.0)
                                    .size(16.0)
                                    .show()
                                    .clicked 
                                {
                                    close_clicked = true;
                                }
                            } else {
                                ui.spacing(20.0);
                            }
                        });

                    if close_clicked {
                        *is_open = false;
                        ui.request_redraw();
                    }

                    // --- Dragging Logic ---
                    if header_res.pressed && ui.clicked && !close_clicked {
                        if !is_dragging {
                            is_dragging = true;
                            ui.interaction_state.insert(drag_id, 1.0);
                            // Store drag offset in interaction state
                            let ox_id = Id::from_u64((id.raw() << 8) | 2);
                            let oy_id = Id::from_u64((id.raw() << 8) | 3);
                            ui.interaction_state.insert(ox_id, ui.mouse_x - win_x);
                            ui.interaction_state.insert(oy_id, ui.mouse_y - win_y);
                        }
                    }

                    if is_dragging {
                        if ui.mouse_down {
                            let ox_id = Id::from_u64((id.raw() << 8) | 2);
                            let oy_id = Id::from_u64((id.raw() << 8) | 3);
                            let ox = ui.interaction_state.get(&ox_id).cloned().unwrap_or(0.0);
                            let oy = ui.interaction_state.get(&oy_id).cloned().unwrap_or(0.0);
                            
                            pos[0] = ui.mouse_x - ox;
                            pos[1] = ui.mouse_y - oy;
                            ui.request_redraw();
                        } else {
                            is_dragging = false;
                            ui.interaction_state.insert(drag_id, 0.0);
                        }
                    }

                    // --- Content ---
                    ui.container()
                        .full_width()
                        .fill_y()
                        .padding_all(15.0)
                        .show(content);
                });
        });

        // Restore window context
        ui.current_window_id = prev_win_id;

        // Drain window's draw commands from ui.overlays and save in window_overlays
        let window_cmds = ui.overlays.drain(start_len..).collect::<Vec<_>>();
        {
            use std::fs::OpenOptions;
            use std::io::Write;
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("window_debug.log") {
                let _ = writeln!(
                    file,
                    "WINDOW {} DRAWS: count={}",
                    title, window_cmds.len()
                );
                for (i, cmd) in window_cmds.iter().enumerate() {
                    match cmd {
                        DrawCommand::Rect(r) => {
                            let _ = writeln!(file, "  [{}] Rect: pos={:?}, size={:?}, color={:?}, clip_rect={:?}", i, r.instance.pos, r.instance.size, r.instance.color, r.instance.clip_rect);
                        }
                        DrawCommand::Text(t) => {
                            let _ = writeln!(file, "  [{}] Text: text='{}', pos={:?}, clip={:?}", i, t.text, t.pos, t.clip);
                        }
                        DrawCommand::OverlayRect(o) => {
                            let _ = writeln!(file, "  [{}] OverlayRect: pos={:?}, size={:?}, clip={:?}", i, [o.x, o.y], [o.width, o.height], o.clip);
                        }
                        DrawCommand::Image(img) => {
                            let _ = writeln!(file, "  [{}] Image: pos={:?}, size={:?}, clip_rect={:?}", i, img.instance.pos, img.instance.size, img.instance.clip_rect);
                        }
                        DrawCommand::BackdropBlur(b) => {
                            let _ = writeln!(file, "  [{}] BackdropBlur: pos=[{},{}], size=[{},{}], blur_radius={}", i, b.x, b.y, b.width, b.height, b.blur_radius);
                        }
                        DrawCommand::CustomPostProcess(cp) => {
                            let _ = writeln!(file, "  [{}] CustomPostProcess: pos=[{},{}], size=[{},{}], shader_id={}, blur_radius={}", i, cp.x, cp.y, cp.width, cp.height, cp.shader_id, cp.blur_radius);
                        }
                    }
                }
            }
        }
        ui.window_overlays.push((id, window_cmds));

        Response {
            clicked: false,
            hovered: is_hovered,
            pressed: is_dragging,
        }
    }
}
