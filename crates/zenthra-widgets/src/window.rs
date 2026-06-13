// crates/zenthra-widgets/src/window.rs

use crate::ui::{Ui};
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

    pub fn show<F>(self, content: F) -> Response 
    where F: FnOnce(&mut Ui)
    {
        let id = self.id;
        if !*self.is_open {
            let z_key = Id::from_u64((id.raw() << 8) | 4);
            let modal_key = Id::from_u64((id.raw() << 8) | 5);
            self.ui.interaction_state.remove(&z_key);
            self.ui.interaction_state.remove(&modal_key);
            return Response { clicked: false, hovered: false, pressed: false };
        }

        let drag_id = Id::from_u64((id.raw() << 8) | 1);
        let z_key = Id::from_u64((id.raw() << 8) | 4);
        let modal_key = Id::from_u64((id.raw() << 8) | 5);

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

        // Light dismiss logic
        if self.light_dismiss && self.ui.clicked && !is_hovered {
            *self.is_open = false;
            self.ui.interaction_state.remove(&z_key);
            self.ui.interaction_state.remove(&modal_key);
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

        // Draw modal backdrop if modal
        if self.modal {
            self.ui.overlays.push(crate::ui::DrawCommand::Rect(crate::ui::RectDraw {
                instance: zenthra_render::RectInstance {
                    pos: [0.0, 0.0],
                    size: [self.ui.width, self.ui.height],
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

        // Set current window ID context
        let prev_win_id = self.ui.current_window_id;
        self.ui.current_window_id = Some(id);

        self.ui.container()
            .absolute(win_x, win_y)
            .width(self.width)
            .height(self.height)
            .bg(self.bg)
            .border(self.border_color, self.border_width)
            .radius_all(self.radius)
            .shadow(self.shadow_color, self.shadow_offset[0], self.shadow_offset[1], self.shadow_blur)
            .shadow_opacity(self.shadow_opacity)
            .clip(true)
            .overlay()
            .show(|ui| {
                // --- Header ---
                let header_res = ui.container()
                    .full_width()
                    .height(self.header_height)
                    .bg(self.header_bg)
                    .padding_x(15.0)
                    .row()
                    .halign(Align::Center)
                    .valign(Align::Center)
                    .show(|ui| {
                        ui.text(&self.title)
                            .color(self.header_text_color)
                            .bold()
                            .show();
                        
                        if self.closable {
                            ui.spacing(ui.available_width - 20.0); // Push to right
                            if ui.button("×")
                                .bg(Color::TRANSPARENT)
                                .text_color(self.header_text_color)
                                .padding(0.0, 0.0, 0.0, 0.0)
                                .size(20.0)
                                .show()
                                .clicked 
                            {
                                *self.is_open = false;
                                ui.request_redraw();
                            }
                        }
                    });

                // --- Dragging Logic ---
                if header_res.pressed && ui.clicked {
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
                        
                        self.pos[0] = ui.mouse_x - ox;
                        self.pos[1] = ui.mouse_y - oy;
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

        // Restore window context
        self.ui.current_window_id = prev_win_id;

        // Drain window's draw commands from ui.overlays and save in window_overlays
        let window_cmds = self.ui.overlays.drain(start_len..).collect::<Vec<_>>();
        self.ui.window_overlays.push((id, window_cmds));

        Response {
            clicked: false,
            hovered: is_hovered,
            pressed: is_dragging,
        }
    }
}
