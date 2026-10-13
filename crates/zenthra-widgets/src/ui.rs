use std::sync::{Arc, Mutex};
use zenthra_text::prelude::*;
use crate::container::{ContainerBuilder, Direction};
use crate::lazy_container::LazyContainerBuilder;
use crate::text::{CursorIcon, TextBuilder};
use zenthra_core::{Color, Id, SemanticNode, Rect};
use zenthra_render::RectInstance;
use zenthra_platform::event::PlatformEvent;

pub struct TextDraw {
    pub text: String,
    pub pos: [f32; 2],
    pub options: TextOptions,
    pub clip: [f32; 4],
}

pub struct OverlayRectDraw {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub clip: [f32; 4],
}

pub struct RectDraw {
    pub instance: RectInstance,
}

pub struct ImageDraw {
    pub source: zenthra_core::ImageSource,
    pub instance: zenthra_render::ImageInstance,
    pub fit: zenthra_core::ObjectFit,
    pub internal_scale: [f32; 2],
    pub internal_offset: [f32; 2],
}

/// Requests the renderer to blur the scene behind this rect.
/// The renderer will copy the current offscreen framebuffer region,
/// run a Dual-Kawase blur pass, and composite it as the background
/// before drawing the tinted/bordered rect on top.
pub struct BackdropBlurDraw {
    /// Position and size in logical pixels
    pub x:      f32,
    pub y:      f32,
    pub width:  f32,
    pub height: f32,
    /// Per-corner radius: [top-left, top-right, bottom-right, bottom-left]
    pub radius: [f32; 4],
    /// Blur radius in logical pixels — maps to Kawase offset parameter
    pub blur_radius: f32,
    /// Clipping rect [x, y, w, h] in logical pixels
    pub clip_rect: [f32; 4],
    
    // CSS-like backdrop filters:
    pub brightness: f32,
    pub saturation: f32,
    pub contrast: f32,
    pub opacity: f32,
    pub blur_type: zenthra_core::style::blur::Type,
}

pub struct CustomPostProcessDraw {
    pub x:      f32,
    pub y:      f32,
    pub width:  f32,
    pub height: f32,
    pub radius: [f32; 4],
    pub blur_radius: f32,
    pub shader_id: &'static str,
    pub clip_rect: [f32; 4],
    pub params: [f32; 8],
}

pub enum DrawCommand {
    Rect(RectDraw),
    Text(TextDraw),
    OverlayRect(OverlayRectDraw),
    Image(ImageDraw),
    /// Glassmorphism backdrop blur
    BackdropBlur(BackdropBlurDraw),
    /// Custom post-process shader effect
    CustomPostProcess(CustomPostProcessDraw),
}

#[derive(Debug, Clone, Copy)]
pub struct ScrollDrag {
    pub id: Id,
    pub start_mouse: f32,
    pub start_scroll: f32,
}

pub struct Ui<'a> {
    pub width: f32,
    pub height: f32,
    pub scale_factor: f32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_down: bool,
    pub cursor_icon: CursorIcon,
    pub draws: Vec<DrawCommand>,
    pub overlays: Vec<DrawCommand>,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub base_x: f32,
    pub base_y: f32,
    pub direction: Direction,
    pub line_height: f32,
    pub child_sizes: Vec<(f32, f32)>,
    pub child_draw_ranges: Vec<(usize, usize)>,
    pub child_origins: Vec<(f32, f32)>,
    pub id_log: Vec<Id>,
    pub id_ranges: Vec<(usize, usize)>,
    pub last_w: f32,
    pub last_h: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub font_system: Option<Arc<Mutex<FontSystem>>>,
    pub input_events: Vec<PlatformEvent>,
    pub focused_id: Option<Id>,
    pub id_counter: u64,
    pub scroll_state: &'a mut std::collections::HashMap<Id, (f32, f32)>,
    pub cursor_state: &'a mut std::collections::HashMap<Id, usize>,
    pub interaction_state: &'a mut std::collections::HashMap<Id, f32>,
    pub active_drag: Option<ScrollDrag>,
    pub active_widget_id: Option<Id>,
    pub clicked: bool,
    pub right_clicked: bool,
    pub elapsed_time: f32,
    pub semantic_nodes: Vec<SemanticNode>,
    pub semantic_stack: Vec<Id>,
    pub render_mode_stack: Vec<zenthra_core::RenderMode>,
    pub needs_redraw: bool,
    pub layout_cache: &'a std::collections::HashMap<Id, (Rect, u64)>,
    pub next_layout_cache: &'a mut std::collections::HashMap<Id, (Rect, u64)>,
    pub screen_layout_cache: &'a std::collections::HashMap<Id, Rect>,
    pub next_screen_layout_cache: &'a mut std::collections::HashMap<Id, Rect>,
    pub image_sizes: &'a std::collections::HashMap<zenthra_core::ImageSource, (u32, u32)>,
    pub available_width: f32,
    pub skip_clip_stack: Vec<bool>,
    pub current_viewport: Rect,
    pub window_overlays: Vec<(Id, Vec<DrawCommand>)>,
    pub current_window_id: Option<Id>,
    pub widget_window_map: &'a std::collections::HashMap<Id, Id>,
    pub next_widget_window_map: &'a mut std::collections::HashMap<Id, Id>,
    pub requested_redraw_at: Option<std::time::Instant>,
    pub event_listeners: std::collections::HashMap<Id, Vec<EventHandler<'a>>>,
    pub window_actions: Vec<zenthra_platform::app::WindowAction>,
    pub active_overlay_stack: Vec<Id>,
    pub active_overlays: Vec<Id>,
    pub font_scale: f32,
    pub modal_active: bool,
    pub modal_window_id: Option<Id>,
    pub resize_active: bool,
}

pub fn copy_to_system_clipboard(text: &str) {
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("WAYLAND_DISPLAY").is_some() {
            use std::io::Write;
            if let Ok(mut child) = std::process::Command::new("wl-copy")
                .stdin(std::process::Stdio::piped())
                .spawn()
            {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(text.as_bytes());
                }
                let _ = child.wait();
            }
        }
        use std::io::Write;
        if let Ok(mut child) = std::process::Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(text.as_bytes());
            }
            let _ = child.wait();
        }
    }

    if let Ok(mut cb) = arboard::Clipboard::new() {
        let _ = cb.set_text(text.to_string());
    }
}

/// Generates a unique interaction_state key for an overlay widget id.
#[inline]
pub fn overlay_key(id: Id) -> Id {
    Id::from_u64(((id.raw() & 0x00FF_FFFF_FFFF_FFFF) << 8) | 99)
}

impl<'a> Ui<'a> {
    pub fn new(
        width: u32,
        height: u32,
        scale_factor: f64,
        font_system: Option<Arc<Mutex<FontSystem>>>,
        events: Vec<PlatformEvent>,
        initial_focused_id: Option<Id>,
        mouse_pos: (f32, f32),
        mouse_down: bool,
        scroll_state: &'a mut std::collections::HashMap<Id, (f32, f32)>,
        cursor_state: &'a mut std::collections::HashMap<Id, usize>,
        interaction_state: &'a mut std::collections::HashMap<Id, f32>,
        active_drag: Option<ScrollDrag>,
        active_widget_id: Option<Id>,
        clicked: bool,
        right_clicked: bool,
        elapsed_time: f32,
        layout_cache: &'a std::collections::HashMap<Id, (Rect, u64)>,
        next_layout_cache: &'a mut std::collections::HashMap<Id, (Rect, u64)>,
        screen_layout_cache: &'a std::collections::HashMap<Id, Rect>,
        next_screen_layout_cache: &'a mut std::collections::HashMap<Id, Rect>,
        widget_window_map: &'a std::collections::HashMap<Id, Id>,
        next_widget_window_map: &'a mut std::collections::HashMap<Id, Id>,
        image_sizes: &'a std::collections::HashMap<zenthra_core::ImageSource, (u32, u32)>,
    ) -> Self {
        let mouse_x = mouse_pos.0;
        let mouse_y = mouse_pos.1;

        Self {
            width: width as f32,
            height: height as f32,
            scale_factor: scale_factor as f32,
            mouse_x,
            mouse_y,
            mouse_down,
            input_events: events,
            focused_id: initial_focused_id,
            id_counter: 0,
            cursor_icon: CursorIcon::Default,
            draws: Vec::new(),
            overlays: Vec::new(),
            cursor_x: 0.0,
            cursor_y: 0.0,
            base_x: 0.0,
            base_y: 0.0,
            direction: Direction::Column,
            line_height: 0.0,
            child_sizes: Vec::new(),
            child_draw_ranges: Vec::new(),
            child_origins: Vec::new(),
            id_log: Vec::new(),
            id_ranges: Vec::new(),
            last_w: 0.0,
            last_h: 0.0,
            max_x: width as f32,
            max_y: height as f32,
            offset_x: 0.0,
            offset_y: 0.0,
            font_system,
            scroll_state,
            cursor_state,
            interaction_state,
            active_drag,
            active_widget_id: if mouse_down { active_widget_id } else { None },
            clicked,
            right_clicked,
            elapsed_time,
            semantic_nodes: Vec::new(),
            semantic_stack: Vec::new(),
            render_mode_stack: vec![zenthra_core::RenderMode::Static],
            needs_redraw: false,
            layout_cache,
            next_layout_cache,
            screen_layout_cache,
            next_screen_layout_cache,
            image_sizes,
            available_width: width as f32,
            skip_clip_stack: Vec::new(),
            current_viewport: Rect::new(0.0, 0.0, width as f32, height as f32),
            window_overlays: Vec::new(),
            current_window_id: None,
            widget_window_map,
            next_widget_window_map,
            requested_redraw_at: None,
            event_listeners: std::collections::HashMap::new(),
            window_actions: Vec::new(),
            active_overlay_stack: Vec::new(),
            active_overlays: Vec::new(),
            font_scale: 1.0,
            modal_active: false,
            modal_window_id: None,
            resize_active: false,
        }
    }

    pub fn request_redraw_at(&mut self, instant: std::time::Instant) {
        if let Some(current) = self.requested_redraw_at {
            if instant < current {
                self.requested_redraw_at = Some(instant);
            }
        } else {
            self.requested_redraw_at = Some(instant);
        }
    }

    pub fn add_listener<F>(&mut self, id: Id, phase: EventPhase, callback: F)
    where
        F: FnMut(&mut EventContext, &WidgetEvent) + 'a,
    {
        self.event_listeners.entry(id).or_default().push(EventHandler {
            phase,
            callback: Box::new(callback),
        });
    }

    pub fn dispatch_event(&mut self, target_id: Id, event: WidgetEvent) {
        let ancestors = self.semantic_stack.clone();

        let mut ctx = EventContext {
            target: target_id,
            current_target: target_id,
            propagation_stopped: false,
        };

        // 1. Capture Phase
        for ancestor_id in &ancestors {
            ctx.current_target = *ancestor_id;
            if let Some(handlers) = self.event_listeners.get_mut(ancestor_id) {
                for handler in handlers {
                    if handler.phase == EventPhase::Capture {
                        (handler.callback)(&mut ctx, &event);
                        if ctx.propagation_stopped {
                            return;
                        }
                    }
                }
            }
        }

        // 2. Target Phase
        ctx.current_target = target_id;
        if let Some(handlers) = self.event_listeners.get_mut(&target_id) {
            for handler in handlers {
                if handler.phase == EventPhase::Target || handler.phase == EventPhase::Bubble {
                    (handler.callback)(&mut ctx, &event);
                    if ctx.propagation_stopped {
                        return;
                    }
                }
            }
        }

        // 3. Bubble Phase
        for ancestor_id in ancestors.iter().rev() {
            ctx.current_target = *ancestor_id;
            if let Some(handlers) = self.event_listeners.get_mut(ancestor_id) {
                for handler in handlers {
                    if handler.phase == EventPhase::Bubble {
                        (handler.callback)(&mut ctx, &event);
                        if ctx.propagation_stopped {
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn request_redraw_after(&mut self, duration: std::time::Duration) {
        let target = std::time::Instant::now() + duration;
        self.request_redraw_at(target);
    }

    pub fn id(&mut self) -> Id {
        self.id_counter += 1;
        Id::from_u64(self.id_counter)
    }

    pub fn id_from(&self, id: impl std::hash::Hash) -> Id {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        Id::from_u64(hasher.finish() & 0x00FF_FFFF_FFFF_FFFF)
    }

    pub fn overlay_key(&self, id: Id) -> Id {
        overlay_key(id)
    }

    pub fn record_layout(&mut self, id: Id, rect: Rect) {
        let id_count = self.id_counter.saturating_sub(id.raw());

        self.next_layout_cache.insert(id, (rect, id_count));
        let screen_rect = Rect::new(rect.origin.x + self.offset_x, rect.origin.y + self.offset_y, rect.size.width, rect.size.height);
        self.next_screen_layout_cache.insert(id, screen_rect);
        if let Some(win_id) = self.current_window_id {
            self.next_widget_window_map.insert(id, win_id);
        }
        self.id_log.push(id);
    }

    /// Consumes the primary click event for this frame so subsequent/underlying widgets do not trigger.
    pub fn consume_click(&mut self) {
        self.clicked = false;
        self.input_events.retain(|event| {
            !matches!(
                event,
                zenthra_platform::event::PlatformEvent::MouseButton {
                    button: winit::event::MouseButton::Left,
                    ..
                }
            )
        });
    }

    /// Consumes the secondary (right) click event for this frame.
    pub fn consume_right_click(&mut self) {
        self.right_clicked = false;
        self.input_events.retain(|event| {
            !matches!(
                event,
                zenthra_platform::event::PlatformEvent::MouseButton {
                    button: winit::event::MouseButton::Right,
                    ..
                }
            )
        });
    }

    /// Consumes all pointer events (click, right-click, mouse-down) for this frame.
    pub fn consume_pointer(&mut self) {
        self.clicked = false;
        self.right_clicked = false;
        self.mouse_down = false;
        self.input_events.retain(|event| {
            !matches!(
                event,
                zenthra_platform::event::PlatformEvent::MouseButton { .. }
            )
        });
    }

    /// Checks if any modal dialog or window is currently active.
    pub fn has_active_modal(&self) -> bool {
        for (&key, &val) in self.interaction_state.iter() {
            if (key.raw() & 0xFF) == 5 && val > 0.5 {
                return true;
            }
        }
        false
    }

    pub fn is_occluded(&self, id: Id, x: f32, y: f32) -> bool {
        let our_win_id = self.widget_window_map.get(&id)
            .or_else(|| self.next_widget_window_map.get(&id))
            .copied()
            .or(self.current_window_id)
            .unwrap_or(id);
        let our_z = self.interaction_state
            .get(&Id::from_u64((our_win_id.raw() << 8) | 4))
            .copied()
            .unwrap_or(0.0);

        // 1. Check active modal blocking: if ANY modal is active, any widget not belonging
        // to that modal is 100% occluded (non-interactive).
        // Overlays and widgets inside an active overlay are on top of modals and are not blocked.
        let is_in_active_overlay = !self.active_overlay_stack.is_empty() || self.active_overlays.contains(&id);
        if !is_in_active_overlay {
            if self.modal_active {
                if let Some(modal_id) = self.modal_window_id {
                    if our_win_id != modal_id {
                        return true;
                    }
                } else {
                    return true;
                }
            }
            for (&key, &val) in self.interaction_state.iter() {
                if (key.raw() & 0xFF) == 5 && val > 0.5 {
                    let modal_win_id = Id::from_u64(key.raw() >> 8);
                    if our_win_id != modal_win_id {
                        return true;
                    }
                }
            }
        }

        // 2. Check active overlays & floating windows occlusion:
        // If mouse (x, y) is inside any active overlay or floating window rect,
        // and current widget is NOT inside that overlay/window, it is occluded!
        let effective_our_overlay = self.active_overlay_stack.last().copied()
            .or_else(|| if self.active_overlays.contains(&id) { Some(id) } else { None });

        let check_overlay_rect = |other_id: Id, other_rect: &Rect| -> bool {
            if other_id == id || other_id == our_win_id {
                return false;
            }
            // If current widget is inside other_id's active overlay stack, it is NOT occluded by it
            if self.active_overlay_stack.contains(&other_id) {
                return false;
            }

            let other_win_id = self.widget_window_map.get(&other_id)
                .or_else(|| self.next_widget_window_map.get(&other_id))
                .copied()
                .unwrap_or(other_id);
            let overlay_k = overlay_key(other_id);
            let win_overlay_k = overlay_key(other_win_id);
            let is_overlay = self.active_overlays.contains(&other_id)
                || self.active_overlays.contains(&other_win_id)
                || self.interaction_state.get(&overlay_k).map(|&v| v > 0.5).unwrap_or(false)
                || self.interaction_state.get(&win_overlay_k).map(|&v| v > 0.5).unwrap_or(false);

            if is_overlay {
                // If current widget belongs to an active overlay, check relative stack order:
                // An overlay is only occluded by overlays that appear AFTER it in active_overlays
                if let Some(our_overlay_id) = effective_our_overlay {
                    if our_overlay_id == other_id || our_overlay_id == other_win_id {
                        return false;
                    }
                    let our_pos = self.active_overlays.iter().rposition(|&oid| oid == our_overlay_id);
                    let other_pos = self.active_overlays.iter().rposition(|&oid| oid == other_id || oid == other_win_id);
                    if let (Some(our_idx), Some(other_idx)) = (our_pos, other_pos) {
                        if our_idx >= other_idx {
                            return false;
                        }
                    } else if other_pos.is_none() {
                        // Current widget is inside an active overlay being drawn.
                        // Stale overlay rects from previous frames that are not active in this frame cannot occlude it.
                        return false;
                    }
                }

                if x >= other_rect.origin.x && x <= other_rect.origin.x + other_rect.size.width &&
                   y >= other_rect.origin.y && y <= other_rect.origin.y + other_rect.size.height {
                    return true;
                }
            } else if other_win_id == our_win_id {
                return false;
            }
            false
        };

        for (&other_id, other_rect) in self.screen_layout_cache {
            if check_overlay_rect(other_id, other_rect) {
                return true;
            }
        }
        for (&other_id, other_rect) in self.next_screen_layout_cache.iter() {
            if check_overlay_rect(other_id, other_rect) {
                return true;
            }
        }

        // 3. Check z-order occlusion (only applies if current widget is not in an overlay)
        if !is_in_active_overlay && effective_our_overlay.is_none() {
            let check_z_rect = |other_id: Id, other_rect: &Rect| -> bool {
                if other_id == id || other_id == our_win_id {
                    return false;
                }
                let other_win_id = self.widget_window_map.get(&other_id)
                    .or_else(|| self.next_widget_window_map.get(&other_id))
                    .copied()
                    .unwrap_or(other_id);
                if other_win_id == our_win_id {
                    return false;
                }
                let other_z_key = Id::from_u64(((other_win_id.raw() & 0x00FF_FFFF_FFFF_FFFF) << 8) | 4);
                if let Some(&other_z) = self.interaction_state.get(&other_z_key) {
                    if other_z > our_z {
                        if x >= other_rect.origin.x && x <= other_rect.origin.x + other_rect.size.width &&
                           y >= other_rect.origin.y && y <= other_rect.origin.y + other_rect.size.height {
                            return true;
                        }
                    }
                }
                false
            };

            for (&other_id, other_rect) in self.screen_layout_cache {
                if check_z_rect(other_id, other_rect) {
                    return true;
                }
            }
            for (&other_id, other_rect) in self.next_screen_layout_cache.iter() {
                if check_z_rect(other_id, other_rect) {
                    return true;
                }
            }
        }

        false
    }

    /// Returns true if this widget is allowed to receive pointer interactions.
    /// Returns false if another widget currently has exclusive pointer capture.
    pub fn can_interact(&self, id: Id) -> bool {
        if self.active_widget_id.is_none() || self.active_widget_id == Some(id) {
            return true;
        }

        // Overlay Priority: If current widget is inside an active overlay, but
        // active_widget_id was captured by an underlying non-overlay background element,
        // the active overlay takes precedence.
        let is_in_overlay = !self.active_overlay_stack.is_empty() || self.active_overlays.contains(&id);
        if is_in_overlay {
            if let Some(active_id) = self.active_widget_id {
                let active_k = overlay_key(active_id);
                let active_is_overlay = self.active_overlays.contains(&active_id)
                    || self.active_overlay_stack.contains(&active_id)
                    || self.interaction_state.get(&active_k).map(|&v| v > 0.5).unwrap_or(false);
                if !active_is_overlay {
                    return true;
                }
            }
        }

        false
    }

    /// Returns true if this widget currently holds exclusive pointer capture.
    pub fn is_active(&self, id: Id) -> bool {
        self.active_widget_id == Some(id)
    }

    /// Claims pointer capture for the given widget if no other widget has it.
    /// Returns true if capture was granted or already held by this widget.
    pub fn claim_active(&mut self, id: Id) -> bool {
        if self.can_interact(id) {
            self.active_widget_id = Some(id);
            true
        } else {
            false
        }
    }

    /// Releases pointer capture.
    pub fn release_active(&mut self) {
        self.active_widget_id = None;
    }

    pub fn is_hovered(&self, id: Id, fallback_x: f32, fallback_y: f32, fallback_w: f32, fallback_h: f32) -> bool {
        // Pointer capture check: if another widget holds capture, this widget is NOT hovered.
        if !self.can_interact(id) {
            return false;
        }

        let is_in = if let Some(screen_rect) = self.screen_layout_cache.get(&id) {
            self.mouse_in_rect(screen_rect.origin.x, screen_rect.origin.y, screen_rect.size.width, screen_rect.size.height)
        } else if let Some((rect, _)) = self.get_recorded_layout(id) {
            self.mouse_in_rect(rect.origin.x + self.offset_x, rect.origin.y + self.offset_y, rect.size.width, rect.size.height)
        } else if fallback_w > 0.0 && fallback_h > 0.0 {
            self.mouse_in_rect(fallback_x, fallback_y, fallback_w, fallback_h)
        } else {
            false
        };
        let mouse_in_viewport = self.current_viewport.contains(zenthra_core::Point::new(self.mouse_x, self.mouse_y));
        is_in && mouse_in_viewport && !self.is_occluded(id, self.mouse_x, self.mouse_y)
    }

    pub fn overlay<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Ui),
    {
        let already_swapped = self.skip_clip_stack.last().cloned().unwrap_or(false);

        if !already_swapped {
            // Swap draws and overlays so that any pushes to self.draws go to self.overlays
            std::mem::swap(&mut self.draws, &mut self.overlays);
        }

        self.skip_clip_stack.push(true);
        let prev_viewport = self.current_viewport;
        self.current_viewport = Rect::new(-100000.0, -100000.0, 2000000.0, 2000000.0);

        f(self);

        self.current_viewport = prev_viewport;
        self.skip_clip_stack.pop();

        if !already_swapped {
            // Swap back to restore self.draws and commit new draws to self.overlays
            std::mem::swap(&mut self.draws, &mut self.overlays);
        }
    }

    pub fn get_recorded_layout(&self, id: Id) -> Option<(Rect, u64)> {
        self.layout_cache.get(&id).cloned()
    }

    pub fn current_render_mode(&self) -> zenthra_core::RenderMode {
        *self.render_mode_stack.last().unwrap_or(&zenthra_core::RenderMode::Static)
    }

    pub fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub fn minimize(&mut self) {
        self.window_actions.push(zenthra_platform::app::WindowAction::Minimize);
    }

    pub fn maximize(&mut self) {
        self.window_actions.push(zenthra_platform::app::WindowAction::Maximize);
    }

    pub fn close(&mut self) {
        self.window_actions.push(zenthra_platform::app::WindowAction::Close);
    }

    pub fn drag(&mut self) {
        self.window_actions.push(zenthra_platform::app::WindowAction::Drag);
    }

    pub fn set_blur(&mut self, blur: bool) {
        self.window_actions.push(zenthra_platform::app::WindowAction::SetBlur(blur));
    }

    pub fn set_always_on_top(&mut self, on_top: bool) {
        self.window_actions.push(zenthra_platform::app::WindowAction::SetAlwaysOnTop(on_top));
    }

    pub fn set_window_size(&mut self, w: u32, h: u32) {
        self.window_actions.push(zenthra_platform::app::WindowAction::SetSize(w, h));
    }

    pub fn set_window_position(&mut self, x: i32, y: i32) {
        self.window_actions.push(zenthra_platform::app::WindowAction::SetPosition(x, y));
    }

    pub fn set_cursor_hittest(&mut self, hittest: bool) {
        self.window_actions.push(zenthra_platform::app::WindowAction::SetCursorHitTest(hittest));
    }

    pub fn get_max_bounds(&self) -> (f32, f32) {
        (self.max_x, self.max_y)
    }

    pub fn row(&mut self) -> ContainerBuilder<'_, 'a> {
        ContainerBuilder::new(self).row()
    }

    pub fn column(&mut self) -> ContainerBuilder<'_, 'a> {
        ContainerBuilder::new(self).column()
    }

    pub fn container(&mut self) -> ContainerBuilder<'_, 'a> {
        ContainerBuilder::new(self)
    }

    pub fn lazy_container(&mut self) -> LazyContainerBuilder<'_, 'a> {
        LazyContainerBuilder::new(self)
    }

    pub fn button(&mut self, label: &str) -> crate::button::ButtonBuilder<'_, 'a> {
        crate::button::ButtonBuilder::new(self, label)
    }

    pub fn image(&mut self, source: zenthra_core::ImageSource) -> crate::image::ImageBuilder<'_, 'a> {
        crate::image::ImageBuilder::new(self, source)
    }

    pub fn spacing(&mut self, size: f32) {
        let (w, h) = match self.direction {
            Direction::Column => (0.0, size),
            Direction::Row => (size, 0.0),
            Direction::Stack => (0.0, 0.0),
        };
        let draw_start = self.draws.len();
        self.advance(w, h, draw_start);
    }

    /// Moves the cursor down by `h` pixels WITHOUT registering a layout child.
    /// Use this inside a lazy/virtual list to offset visible items below invisible ones.
    pub fn add_space(&mut self, h: f32) {
        match self.direction {
            Direction::Column => self.cursor_y += h,
            Direction::Row => self.cursor_x += h,
            Direction::Stack => {}
        }
    }

    /// Registers an invisible zero-draw child with the given size.
    /// This forces the parent container's layout engine to account for this size
    /// when calculating total content height for scroll area sizing.
    pub fn set_min_size(&mut self, w: f32, h: f32) {
        let draw_start = self.draws.len();
        self.advance(w, h, draw_start);
    }


    pub fn input<'b>(&mut self, buffer: &'b mut String) -> crate::input::InputBuilder<'_, 'a, 'b> {
        crate::input::InputBuilder::new(self, buffer)
    }

    pub fn input_with_id<'b>(&mut self, buffer: &'b mut String, id: impl std::hash::Hash) -> crate::input::InputBuilder<'_, 'a, 'b> {
        crate::input::InputBuilder::new(self, buffer).id(id)
    }

    pub fn slider<'b>(&mut self, value: &'b mut f32, id: impl std::hash::Hash) -> crate::slider::SliderBuilder<'_, 'a, 'b> {
        crate::slider::SliderBuilder::new(self, value).id(id)
    }

    pub fn progress_bar(&mut self, value: f32) -> crate::progress_bar::ProgressBarBuilder<'_, 'a> {
        crate::progress_bar::ProgressBarBuilder::new(self, value)
    }

    pub fn checkbox<'b>(&mut self, value: &'b mut bool, label: &str) -> crate::controls::checkbox::CheckboxBuilder<'_, 'a, 'b> {
        crate::controls::checkbox::CheckboxBuilder::new(self, value, label)
    }

    pub fn toggle<'b>(&mut self, value: &'b mut bool, label: impl Into<Option<&'b str>>) -> crate::controls::toggle::ToggleBuilder<'_, 'a, 'b> {
        let l: Option<&str> = label.into();
        crate::controls::toggle::ToggleBuilder::new(self, value, l)
    }

    pub fn radio<'b, T: PartialEq + Clone>(&mut self, state: &'b mut T, value: T, label: &str) -> crate::controls::radio::RadioBuilder<'_, 'a, 'b, T> {
        crate::controls::radio::RadioBuilder::new(self, state, value, label)
    }

    pub fn dropdown<'b, T: PartialEq + Clone + ToString>(&mut self, selected: &'b mut T, options: Vec<T>) -> crate::controls::dropdown::DropdownBuilder<'_, 'a, 'b, T> {
        crate::controls::dropdown::DropdownBuilder::new(self, selected, options)
    }

    pub fn text_area<'b>(&mut self, buffer: &'b mut String) -> crate::text_area::TextAreaBuilder<'_, 'a, 'b> {
        crate::text_area::TextAreaBuilder::new(self, buffer)
    }

    pub fn text_area_with_id<'b>(&mut self, buffer: &'b mut String, id: impl std::hash::Hash) -> crate::text_area::TextAreaBuilder<'_, 'a, 'b> {
        crate::text_area::TextAreaBuilder::new(self, buffer).id(id)
    }

    pub fn window<'b>(&mut self, title: &str, is_open: &'b mut bool, pos: &'b mut [f32; 2]) -> crate::window::FloatingWindowBuilder<'_, 'a, 'b> {
        crate::window::FloatingWindowBuilder::new(self, title, is_open, pos)
    }

    pub fn menu_bar(&mut self) -> crate::controls::menu::MenuBarBuilder<'_, 'a> {
        crate::controls::menu::MenuBarBuilder::new(self)
    }

    pub fn drag_value<'b, N: crate::controls::drag_value::NumericValue>(&mut self, value: &'b mut N, id: impl std::hash::Hash) -> crate::controls::drag_value::DragValueBuilder<'_, 'a, 'b, N> {
        crate::controls::drag_value::DragValueBuilder::new(self, value, id)
    }

    pub fn number_box<'b, N: crate::controls::drag_value::NumericValue>(&mut self, value: &'b mut N, id: impl std::hash::Hash) -> crate::controls::drag_value::DragValueBuilder<'_, 'a, 'b, N> {
        crate::controls::drag_value::DragValueBuilder::new(self, value, id)
    }

    pub fn menu(&mut self, label: &str) -> crate::controls::menu::MenuBuilder<'_, 'a> {
        crate::controls::menu::MenuBuilder::new(self, label)
    }

    pub fn menu_item(&mut self, label: &str) -> crate::controls::menu::MenuItemBuilder<'_, 'a> {
        crate::controls::menu::MenuItemBuilder::new(self, label)
    }

    pub fn popover(&mut self, anchor_id: zenthra_core::Id) -> crate::controls::popover::PopoverBuilder<'_, 'a> {
        crate::controls::popover::PopoverBuilder::new(self, anchor_id)
    }

    /// Registers a widget in the semantic tree.
    pub fn register_semantic(&mut self, node: zenthra_core::SemanticNode) {
        // If we are inside a container, add this node as a child of the current parent
        if let Some(parent_id) = self.semantic_stack.last() {
            if let Some(parent) = self.semantic_nodes.iter_mut().find(|n| n.id == *parent_id) {
                parent.children.push(node.id);
            }
        }
        self.semantic_nodes.push(node);
    }

    pub fn set_mouse(&mut self, x: f32, y: f32, down: bool) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.mouse_down = down;
    }

    pub fn mouse_in_rect(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let mx = self.mouse_x;
        let my = self.mouse_y;
        mx >= x && mx <= x + w &&
        my >= y && my <= y + h
    }

    pub fn is_rect_visible(&self, rect: Rect) -> bool {
        let rw = rect.size.width;
        let rh = rect.size.height;
        let rx = rect.origin.x + self.offset_x;
        let ry = rect.origin.y + self.offset_y;
        
        // Bleed allows items slightly off-screen to remain drawn (prevents flickering)
        let bleed = 150.0;
        rx + rw + bleed >= 0.0 && rx - bleed <= self.width &&
        ry + rh + bleed >= 0.0 && ry - bleed <= self.height
    }

    /// Converts a logical [x, y, w, h] rect to physical pixels for GPU clipping.
    pub fn physical_clip(&self, x: f32, y: f32, w: f32, h: f32) -> [f32; 4] {
        let sf = self.scale_factor;
        [x * sf, y * sf, w * sf, h * sf]
    }

    /// Called by widgets after pushing their draws.
    /// Records draw range and advances cursor.
    pub fn advance(&mut self, w: f32, h: f32, draw_start: usize) {
        let draw_end = self.draws.len();
        self.child_draw_ranges.push((draw_start, draw_end));
        self.child_sizes.push((w, h));
        self.child_origins.push((self.cursor_x, self.cursor_y));
        
        let id_start = self.id_ranges.last().map(|(_, end)| *end).unwrap_or(0);
        let id_end = self.id_log.len();
        self.id_ranges.push((id_start, id_end));

        self.last_w = w;
        self.last_h = h;

        match self.direction {
            Direction::Column => {
                self.line_height = self.line_height.max(h);
                self.cursor_y += h;
                self.cursor_x = self.base_x;
            }
            Direction::Row => {
                self.cursor_x += w;
                self.cursor_y = self.base_y;
                self.line_height = self.line_height.max(h);
            }
            Direction::Stack => {
                self.cursor_x = self.base_x;
                self.cursor_y = self.base_y;
            }
        }
    }

    pub fn text(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        TextBuilder::new(self, content)
    }

    pub fn h1(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(40.0).font_weight(crate::text::FontWeight::Bold)
    }

    pub fn h2(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(32.0).font_weight(crate::text::FontWeight::Bold)
    }

    pub fn h3(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(24.0).font_weight(crate::text::FontWeight::Bold)
    }

    pub fn h4(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(20.0).font_weight(crate::text::FontWeight::Bold)
    }

    pub fn card(&mut self) -> crate::containers::card::CardBuilder<'_, 'a> {
        crate::containers::card::CardBuilder::new(self)
    }

    pub fn panel<'b>(&mut self) -> crate::containers::panel::PanelBuilder<'_, 'a, 'b> {
        crate::containers::panel::PanelBuilder::new(self)
    }

    pub fn card_header<F>(&mut self, title: &str, subtitle: &str, actions: F)
    where F: FnOnce(&mut Ui) {
        self.container()
            .fill_x()
            .row()
            .halign(zenthra_core::Align::SpaceBetween)
            .valign(zenthra_core::Align::Center)
            .padding_bottom(8.0)
            .show(|ui| {
                ui.container()
                    .column()
                    .show(|ui| {
                        ui.text(title)
                            .size(16.0)
                            .font_weight(crate::text::FontWeight::Bold)
                            .color(Color::WHITE)
                            .show();
                        if !subtitle.is_empty() {
                            ui.spacing(2.0);
                            ui.text(subtitle)
                                .size(12.0)
                                .color(Color::rgb(0.6, 0.6, 0.6))
                                .show();
                        }
                    });

                ui.container()
                    .row()
                    .valign(zenthra_core::Align::Center)
                    .show(actions);
            });

        self.card_divider();
    }

    pub fn card_footer<F>(&mut self, f: F)
    where F: FnOnce(&mut Ui) {
        self.card_divider();
        self.container()
            .fill_x()
            .row()
            .show(f);
    }

    pub fn card_divider(&mut self) {
        self.spacing(8.0);
        self.container()
            .fill_x()
            .height(1.0)
            .bg(Color::rgb(0.2, 0.2, 0.25))
            .show(|_| {});
        self.spacing(8.0);
    }

    pub fn stack(&mut self) -> crate::layout::StackBuilder<'_, 'a> {
        crate::layout::StackBuilder::new(self)
    }
}

/// A helper to get the default Zentype shaper for the current font system.
pub fn get_shaper(font_system: &Arc<Mutex<FontSystem>>) -> CosmicFontProvider {
    CosmicFontProvider::new_with_system(font_system.clone())
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidgetEvent {
    Click,
    Hover(bool),
    Scroll(f32, f32),
    Change(EventValue),
    Submit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventValue {
    Bool(bool),
    Float(f32),
    String(String),
}

#[derive(Debug, Clone)]
pub struct EventContext {
    pub target: Id,
    pub current_target: Id,
    pub propagation_stopped: bool,
}

impl EventContext {
    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPhase {
    Capture,
    Target,
    Bubble,
}

pub struct EventHandler<'a> {
    pub phase: EventPhase,
    pub callback: Box<dyn FnMut(&mut EventContext, &WidgetEvent) + 'a>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_hover_viewport_bounds() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        // 1. Create a UI with mouse position at (50.0, 50.0)
        let mut ui = Ui::new(
            100, 100, 1.0, None, Vec::new(), None,
            (50.0, 50.0), false,
            &mut scroll_state,
            &mut cursor_state,
            &mut interaction_state,
            None, None, false, false, 0.0,
            &layout_cache,
            &mut next_layout_cache,
            &screen_layout_cache,
            &mut next_screen_layout_cache,
            &widget_window_map,
            &mut next_widget_window_map,
            &image_sizes,
        );

        let test_id = Id::from_u64(12345);

        // Standard bounds: (40.0, 40.0, 20.0, 20.0). Mouse at (50.0, 50.0) is inside.
        // Initially, current_viewport is (0.0, 0.0, 100.0, 100.0) which covers the mouse.
        assert!(ui.is_hovered(test_id, 40.0, 40.0, 20.0, 20.0));

        // 2. Set current_viewport to not cover the mouse (e.g. scrolled out of view)
        ui.current_viewport = Rect::new(0.0, 0.0, 30.0, 30.0);
        // The mouse (50, 50) is now outside the visible viewport.
        // Even though the fallback bounds (40, 40, 20, 20) technically contain the mouse,
        // the viewport check should restrict the hover.
        assert!(!ui.is_hovered(test_id, 40.0, 40.0, 20.0, 20.0));
    }

    #[test]
    fn test_event_propagation() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        let mut ui = Ui::new(
            100, 100, 1.0, None, Vec::new(), None,
            (50.0, 50.0), false,
            &mut scroll_state,
            &mut cursor_state,
            &mut interaction_state,
            None, None, false, false, 0.0,
            &layout_cache,
            &mut next_layout_cache,
            &screen_layout_cache,
            &mut next_screen_layout_cache,
            &widget_window_map,
            &mut next_widget_window_map,
            &image_sizes,
        );

        let parent_id = Id::from_u64(1);
        let middle_id = Id::from_u64(2);
        let child_id = Id::from_u64(3);

        // Simulate hierarchy
        ui.semantic_stack = vec![parent_id, middle_id];

        use std::cell::RefCell;
        use std::rc::Rc;
        let trace = Rc::new(RefCell::new(Vec::new()));

        {
            let trace_capture = trace.clone();
            ui.add_listener(parent_id, EventPhase::Capture, move |_, _| {
                trace_capture.borrow_mut().push("parent_capture".to_string());
            });
        }
        {
            let trace_bubble = trace.clone();
            ui.add_listener(parent_id, EventPhase::Bubble, move |_, _| {
                trace_bubble.borrow_mut().push("parent_bubble".to_string());
            });
        }
        {
            let trace_capture = trace.clone();
            ui.add_listener(middle_id, EventPhase::Capture, move |_, _| {
                trace_capture.borrow_mut().push("middle_capture".to_string());
            });
        }
        {
            let trace_bubble = trace.clone();
            ui.add_listener(middle_id, EventPhase::Bubble, move |_, _| {
                trace_bubble.borrow_mut().push("middle_bubble".to_string());
            });
        }
        {
            let trace_target = trace.clone();
            ui.add_listener(child_id, EventPhase::Target, move |_, _| {
                trace_target.borrow_mut().push("child_target".to_string());
            });
        }

        ui.dispatch_event(child_id, WidgetEvent::Click);

        assert_eq!(
            *trace.borrow(),
            vec![
                "parent_capture".to_string(),
                "middle_capture".to_string(),
                "child_target".to_string(),
                "middle_bubble".to_string(),
                "parent_bubble".to_string(),
            ]
        );

        // Test stopping propagation in middle_capture phase
        trace.borrow_mut().clear();
        ui.event_listeners.clear();

        {
            let trace_capture = trace.clone();
            ui.add_listener(parent_id, EventPhase::Capture, move |_, _| {
                trace_capture.borrow_mut().push("parent_capture".to_string());
            });
        }
        {
            let trace_capture = trace.clone();
            ui.add_listener(middle_id, EventPhase::Capture, move |ctx, _| {
                trace_capture.borrow_mut().push("middle_capture".to_string());
                ctx.stop_propagation();
            });
        }
        {
            let trace_target = trace.clone();
            ui.add_listener(child_id, EventPhase::Target, move |_, _| {
                trace_target.borrow_mut().push("child_target".to_string());
            });
        }

        ui.dispatch_event(child_id, WidgetEvent::Click);

        assert_eq!(
            *trace.borrow(),
            vec![
                "parent_capture".to_string(),
                "middle_capture".to_string(),
            ]
        );
    }

    #[test]
    fn test_item_container_with_image_hover() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let mut layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let mut screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        for frame in 1..=3 {
            let hovered_item_0 = {
                let mut ui = Ui::new(
                    800, 600, 1.0, None, Vec::new(), None,
                    (70.0, 50.0), false, // Mouse is over item 0's image
                    &mut scroll_state,
                    &mut cursor_state,
                    &mut interaction_state,
                    None, None, false, false, 0.0,
                    &layout_cache,
                    &mut next_layout_cache,
                    &screen_layout_cache,
                    &mut next_screen_layout_cache,
                    &widget_window_map,
                    &mut next_widget_window_map,
                    &image_sizes,
                );

                let mut hov = false;

                ui.container().id("list_container").width(800.0).height(600.0).show(|ui| {
                    ui.lazy_container()
                        .item_size(112.0, 96.0)
                        .count(4)
                        .id("grid")
                        .show(|ui, idx| {
                            let item_id = Id::from_u64(1000 + idx as u64);
                            let (cur_x, cur_y) = (ui.cursor_x, ui.cursor_y);
                            let resp = ui.container()
                                .id(item_id)
                                .pos(cur_x, cur_y)
                                .width(112.0)
                                .height(96.0)
                                .column()
                                .align(zenthra_core::Align::Center)
                                .valign(zenthra_core::Align::Center)
                                .gap(2.0)
                                .radius_all(6.0)
                                .padding(2.0, 2.0, 2.0, 2.0)
                                .clip(true)
                                .show(|ui| {
                                    ui.image(zenthra_core::ImageSource::Bytes(std::sync::Arc::from(&[][..])))
                                        .size(64.0, 64.0)
                                        .fit(zenthra_core::ObjectFit::Contain)
                                        .show();
                                    ui.container().row().gap(4.0).valign(zenthra_core::Align::Center).show(|ui| {
                                        ui.text("test").show();
                                    });
                                });

                            if idx == 0 {
                                hov = resp.hovered;
                            }
                        });
                });
                hov
            };

            println!("FRAME {}: hovered_item_0 = {}", frame, hovered_item_0);
            for (id, rect) in &next_screen_layout_cache {
                println!("  next_screen_layout_cache: id={:?} rect={:?}", id, rect);
            }

            layout_cache = next_layout_cache.clone();
            screen_layout_cache = next_screen_layout_cache.clone();
            next_layout_cache.clear();
            next_screen_layout_cache.clear();

            assert!(hovered_item_0, "Item 0 must be hovered on Frame {}", frame);
        }
    }

    #[test]
    fn test_submenu_click() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let mut layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let mut screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        let active_menu_key = Id::from_u64(999999900);
        let active_submenu_key = Id::from_u64(999999901);
        let hover_flag_key = Id::from_u64(999999902);

        // Frame 1: Click "Theme" menu at (10.0, 10.0)
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (10.0, 10.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, true, false, 0.0, // clicked = true
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.container().row().show(|ui| {
                ui.menu("Theme").show(|ui| {
                    ui.menu_item("Dark Theme").show();
                    ui.menu_item("Light Theme").show();
                    ui.menu_item("Glassmorphism").show();
                    ui.spacing(4.0);
                    ui.menu_item("Flat Folder Icons").show();
                    ui.spacing(4.0);
                    ui.menu("Accent Color").show(|ui| {
                        ui.menu_item("Yellow").show();
                    });
                });
            });
        }
        layout_cache = next_layout_cache.clone();
        screen_layout_cache = next_screen_layout_cache.clone();
        next_layout_cache.clear();
        next_screen_layout_cache.clear();

        let menu_id = interaction_state.get(&active_menu_key).copied().unwrap_or(0.0);
        assert_ne!(menu_id, 0.0, "Theme menu should be active after clicking it");

        // Frame 2: Hover over Accent Color
        for (id, rect) in &screen_layout_cache {
            // Find Accent Color
            eprintln!("Frame 2 cache: id={:?}, rect={:?}", id, rect);
        }

        // Run Frame 2 with mouse at Accent Color (cursor_y should be around 30 + 26*4 + 8)
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (50.0, 150.0), false, // hovering at Accent Color
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            ui.container().row().show(|ui| {
                ui.menu("Theme").show(|ui| {
                    ui.menu_item("Dark Theme").show();
                    ui.menu_item("Light Theme").show();
                    ui.menu_item("Glassmorphism").show();
                    ui.spacing(4.0);
                    ui.menu_item("Flat Folder Icons").show();
                    ui.spacing(4.0);
                    ui.menu("Accent Color").show(|ui| {
                        ui.menu_item("Yellow").show();
                    });
                });
            });
        }
        layout_cache = next_layout_cache.clone();
        screen_layout_cache = next_screen_layout_cache.clone();
        next_layout_cache.clear();
        next_screen_layout_cache.clear();

        // Print all rects from Frame 2
        for (id, rect) in &screen_layout_cache {
            eprintln!("After Frame 2 cache: id={:?}, rect={:?}", id, rect);
        }

        // Frame 3: Click "Accent Color" with clicked = true at (50.0, 150.0)
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (50.0, 150.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, true, false, 0.0, // clicked = true
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            // Title bar resets hover flag at start
            ui.interaction_state.insert(hover_flag_key, 0.0);

            ui.container().row().show(|ui| {
                ui.menu("Theme").show(|ui| {
                    ui.menu_item("Dark Theme").show();
                    ui.menu_item("Light Theme").show();
                    ui.menu_item("Glassmorphism").show();
                    ui.spacing(4.0);
                    ui.menu_item("Flat Folder Icons").show();
                    ui.spacing(4.0);
                    ui.menu("Accent Color").show(|ui| {
                        ui.menu_item("Yellow").show();
                    });
                });
            });

            // Title bar light dismiss logic at end
            let clicked = ui.clicked;
            let active_menu_id = ui.interaction_state.get(&active_menu_key).copied().map(|v| v as u64).unwrap_or(0);
            let hover_flag = ui.interaction_state.get(&hover_flag_key).copied().unwrap_or(0.0) > 0.5;

            eprintln!("Frame 3 end: clicked={}, active_menu_id={}, hover_flag={}", clicked, active_menu_id, hover_flag);
            if active_menu_id != 0 && clicked && !hover_flag {
                eprintln!("LIGHT DISMISS TRIGGERED IN FRAME 3!");
                ui.interaction_state.insert(active_menu_key, 0.0);
                ui.interaction_state.insert(active_submenu_key, 0.0);
            }
        }

        let sub_id = interaction_state.get(&active_submenu_key).copied().unwrap_or(0.0);
        eprintln!("After Frame 3, active_submenu_key = {}", sub_id);
        assert_ne!(sub_id, 0.0, "Accent Color submenu should be active after clicking it!");

        layout_cache = next_layout_cache.clone();
        screen_layout_cache = next_screen_layout_cache.clone();
        next_layout_cache.clear();
        next_screen_layout_cache.clear();

        // Frame 4: Accent Color is active, render sub-menu
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (50.0, 150.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );

            ui.container().row().show(|ui| {
                ui.menu("Theme").show(|ui| {
                    ui.menu_item("Dark Theme").show();
                    ui.menu_item("Light Theme").show();
                    ui.menu_item("Glassmorphism").show();
                    ui.spacing(4.0);
                    ui.menu_item("Flat Folder Icons").show();
                    ui.spacing(4.0);
                    ui.menu("Accent Color").show(|ui| {
                        ui.menu_item("Yellow").show();
                    });
                });
            });
        }

        // Verify that Yellow exists in next_screen_layout_cache
        let yellow_found = next_screen_layout_cache.iter().any(|(id, rect)| {
            eprintln!("Frame 4 cache: id={:?}, rect={:?}", id, rect);
            rect.origin.x > 200.0 // Submenu is to the right
        });
        assert!(yellow_found, "Submenu popup item (Yellow) should be rendered in screen layout cache");
    }

    #[test]
    fn test_popover_click() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let mut layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let mut screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        let ai_more_btn_id = Id::from_u64(999999887);
        let msgs_scroll_id = Id::from_u64(999999802);
        let bubble_id = Id::from_u64(999_100_000);
        let copy_btn_id = Id::from_u64(999_100_001);

        let render_frame = |ui: &mut Ui,
                            clicked_zene: &mut bool,
                            clicked_settings: &mut bool| {
            // Main background column
            ui.container().width(800.0).height(600.0).column().show(|ui| {
                // Title bar (height 36)
                ui.container().width(800.0).height(36.0).show(|_| {});

                // Middle area (height 540)
                ui.container().width(800.0).height(540.0).row().show(|ui| {
                    // Buffer area (width 500)
                    ui.container().width(500.0).height(540.0).show(|_| {});

                    // AI panel (width 300)
                    let ai_w = 300.0;
                    let ai_h = 540.0;
                    ui.container().id(Id::from_u64(8888)).width(ai_w).height(ai_h).clip(true).column().show(|ui| {
                        // Header (height 32)
                        ui.container().width(ai_w).height(32.0).row().show(|ui| {
                            ui.container().width(200.0).height(32.0).show(|_| {});
                            ui.container().raw_id(ai_more_btn_id).width(22.0).height(22.0).interactive(true).role(zenthra_core::Role::Button).show(|_| {});
                        });

                        // Chat content (height 507)
                        let content_h = ai_h - 33.0;
                        ui.container().width(ai_w).height(content_h).column().show(|ui| {
                            // Messages scroll area
                            ui.container().id(msgs_scroll_id).width(ai_w).height(350.0).scroll_y(true).column().show(|ui| {
                                // Message bubble
                                ui.container().id(bubble_id).width(200.0).height(40.0).hover_bg(Color::RED).row().show(|ui| {
                                    ui.container().id(copy_btn_id).width(20.0).height(20.0).hover_bg(Color::GREEN).interactive(true).role(zenthra_core::Role::Button).show(|_| {});
                                });
                            });

                            // Prompt dock
                            ui.container().width(ai_w).height(100.0).show(|_| {});
                        });
                    });

                    // 3-dot menu popover
                    let menu_w = 180.0;
                    let menu_h = 122.0;
                    ui.popover(ai_more_btn_id)
                        .placement(zenthra_core::Placement::Bottom)
                        .align(zenthra_core::PopoverAlign::End)
                        .gap(4.0)
                        .auto_flip(false)
                        .width(menu_w)
                        .height(menu_h)
                        .show(|ui| {
                            ui.container().fill_x().column().gap(1.0).show(|ui| {
                                // Section 1 header
                                ui.container().padding(3.0, 6.0, 2.0, 6.0).show(|ui| {
                                    ui.text("NATIVE AGENT").show();
                                });

                                // ZENE btn
                                let zene_btn = ui.container()
                                    .fill_x()
                                    .padding(4.0, 6.0, 4.0, 6.0)
                                    .radius_all(3.5)
                                    .hover_bg(Color::BLUE)
                                    .interactive(true).role(zenthra_core::Role::Button)
                                    .show(|ui| {
                                        ui.text("ZENE").show();
                                    });
                                if zene_btn.clicked { *clicked_zene = true; }

                                // Divider
                                ui.container().fill_x().height(1.0).show(|_| {});

                                // Section 2 header
                                ui.container().padding(2.0, 6.0, 2.0, 6.0).show(|ui| {
                                    ui.text("EXTERNAL AGENTS").show();
                                });

                                // AG btn
                                let _ag_btn = ui.container()
                                    .fill_x()
                                    .padding(4.0, 6.0, 4.0, 6.0)
                                    .radius_all(3.5)
                                    .hover_bg(Color::BLUE)
                                    .interactive(true).role(zenthra_core::Role::Button)
                                    .show(|ui| {
                                        ui.text("Google Antigravity").show();
                                    });

                                // Divider
                                ui.container().fill_x().height(1.0).show(|_| {});

                                // Settings btn
                                let set_btn = ui.container()
                                    .fill_x()
                                    .padding(4.0, 6.0, 4.0, 6.0)
                                    .radius_all(3.5)
                                    .hover_bg(Color::BLUE)
                                    .interactive(true).role(zenthra_core::Role::Button)
                                    .show(|ui| {
                                        ui.text("AI & Agent Settings...").show();
                                    });
                                if set_btn.clicked { *clicked_settings = true; }
                            });
                        });
                });
            });
        };

        // Frame 1: Initial render
        {
            let mut cz = false;
            let mut cs = false;
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            render_frame(&mut ui, &mut cz, &mut cs);
        }

        layout_cache = next_layout_cache.clone();
        screen_layout_cache = next_screen_layout_cache.clone();
        next_layout_cache.clear();
        next_screen_layout_cache.clear();

        // Find zene_btn rect and set_btn rect from screen_layout_cache
        // Let's test clicking on ZENE button in Frame 2!
        let mut clicked_zene = false;
        let mut clicked_settings = false;

        // Where is ai_more_btn?
        // ui_w = 800. Buffer = 500. AI panel starts at x = 500.
        // Header height = 32. y starts at 36 (title bar).
        // ai_more_btn is at x = 500 + 200 = 700, y = 36.
        // Popover is anchored at (700, 36).
        // Placement = Bottom: y = 36 + 22 + 4 = 62.
        // Align = End: x = 700 + 22 - 180 = 542.
        // Popover is at (542, 62, 180, 122).
        // zene_btn is near the top of the popover: y ~= 62 + 4 + 18 ~= 84.
        // set_btn is near the bottom of the popover: y ~= 62 + 95 ~= 157.

        // Test 1: Click on ZENE (x = 600.0, y = 100.0)
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (600.0, 100.0), true,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, true, false, 0.0, // clicked = true
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            render_frame(&mut ui, &mut clicked_zene, &mut clicked_settings);
        }

        // Test 2: Click on Settings (x = 600.0, y = 190.0)
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (600.0, 190.0), true,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, true, false, 0.0, // clicked = true
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            render_frame(&mut ui, &mut clicked_zene, &mut clicked_settings);
        }

        assert!(clicked_zene, "ZENE button must be clicked when clicking inside it!");
        assert!(clicked_settings, "Settings button must be clicked when clicking inside it!");
    }

    #[test]
    fn test_model_picker_popover() {
        let mut scroll_state = HashMap::new();
        let mut cursor_state = HashMap::new();
        let mut interaction_state = HashMap::new();
        let mut layout_cache = HashMap::new();
        let mut next_layout_cache = HashMap::new();
        let mut screen_layout_cache = HashMap::new();
        let mut next_screen_layout_cache = HashMap::new();
        let widget_window_map = HashMap::new();
        let mut next_widget_window_map = HashMap::new();
        let image_sizes = HashMap::new();

        let model_btn_id = Id::from_u64(999999886);
        let msgs_scroll_id = Id::from_u64(999999802);

        let render_model_picker = |ui: &mut Ui, clicked_model: &mut Option<usize>| {
            // Main layout
            ui.container().width(800.0).height(600.0).column().show(|ui| {
                ui.container().width(800.0).height(36.0).show(|_| {});
                ui.container().width(800.0).height(540.0).row().show(|ui| {
                    ui.container().width(500.0).height(540.0).show(|_| {});

                    let ai_w = 300.0;
                    let ai_h = 540.0;
                    ui.container().id(Id::from_u64(8888)).width(ai_w).height(ai_h).clip(true).column().show(|ui| {
                        ui.container().width(ai_w).height(32.0).show(|_| {});

                        let content_h = ai_h - 33.0;
                        ui.container().width(ai_w).height(content_h).column().show(|ui| {
                            // Messages scroll area
                            ui.container().id(msgs_scroll_id).width(ai_w).height(350.0).scroll_y(true).column().show(|ui| {
                                ui.container().width(200.0).height(200.0).hover_bg(Color::RED).show(|_| {});
                            });

                            // Prompt dock (height 120, at y = 36 + 32 + 350 = 418)
                            ui.container().width(ai_w).height(120.0).column().show(|ui| {
                                ui.container().width(ai_w).height(80.0).show(|_| {});
                                // Bottom bar with model button
                                ui.container().width(ai_w).height(26.0).row().show(|ui| {
                                    ui.container().raw_id(model_btn_id).width(60.0).height(24.0).interactive(true).role(zenthra_core::Role::Button).show(|_| {});
                                });
                            });
                        });
                    });

                    // Popover for model picker: Placement::Top anchored to model_btn_id
                    let menu_w = 220.0;
                    let menu_h = 240.0;
                    ui.popover(model_btn_id)
                        .placement(zenthra_core::Placement::Top)
                        .align(zenthra_core::PopoverAlign::Start)
                        .gap(3.0)
                        .auto_flip(false)
                        .width(menu_w)
                        .height(menu_h)
                        .show(|ui| {
                            ui.container()
                                .raw_id(Id::from_u64(0xF00D_1101))
                                .fill_x()
                                .height(menu_h - 6.0)
                                .column()
                                .gap(1.2)
                                .scroll_y(true)
                                .show(|ui| {
                                    for i in 0..6 {
                                        let row_id = Id::from_u64(0xBEEF_0000 + i as u64);
                                        let row = ui.container()
                                            .raw_id(row_id)
                                            .interactive(true).role(zenthra_core::Role::Button)
                                            .fill_x()
                                            .height(26.0)
                                            .hover_bg(Color::BLUE)
                                            .row()
                                            .show(|ui| {
                                                ui.text(&format!("Model {}", i)).show();
                                            });
                                        if row.clicked || (row.hovered && ui.clicked) {
                                            *clicked_model = Some(i);
                                        }
                                    }
                                });
                        });
                });
            });
        };

        // Frame 1
        {
            let mut cm = None;
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (0.0, 0.0), false,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, false, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            render_model_picker(&mut ui, &mut cm);
        }

        layout_cache = next_layout_cache.clone();
        screen_layout_cache = next_screen_layout_cache.clone();
        next_layout_cache.clear();
        next_screen_layout_cache.clear();

        // Test clicking Model 0 (top model) vs Model 5 (bottom model)
        let m0_rect = screen_layout_cache.get(&Id::from_u64(0xBEEF_0000)).copied().unwrap();
        let m5_rect = screen_layout_cache.get(&Id::from_u64(0xBEEF_0005)).copied().unwrap();

        let mut clicked_m0 = None;
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (m0_rect.origin.x + 10.0, m0_rect.origin.y + 10.0), true,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, true, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            render_model_picker(&mut ui, &mut clicked_m0);
        }

        let mut clicked_m5 = None;
        {
            let mut ui = Ui::new(
                800, 600, 1.0, None, Vec::new(), None,
                (m5_rect.origin.x + 10.0, m5_rect.origin.y + 10.0), true,
                &mut scroll_state, &mut cursor_state, &mut interaction_state,
                None, None, true, false, 0.0,
                &layout_cache, &mut next_layout_cache,
                &screen_layout_cache, &mut next_screen_layout_cache,
                &widget_window_map, &mut next_widget_window_map,
                &image_sizes,
            );
            render_model_picker(&mut ui, &mut clicked_m5);
        }

        assert_eq!(clicked_m0, Some(0), "Model 0 (top of popover) must be clickable!");
        assert_eq!(clicked_m5, Some(5), "Model 5 (bottom of popover) must be clickable!");
    }
}




