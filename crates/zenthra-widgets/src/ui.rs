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

pub enum DrawCommand {
    Rect(RectDraw),
    Text(TextDraw),
    OverlayRect(OverlayRectDraw),
    Image(ImageDraw),
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

    pub fn is_occluded(&self, id: Id, x: f32, y: f32) -> bool {
        let our_win_id = self.widget_window_map.get(&id).copied().unwrap_or(id);
        let our_z = self.interaction_state
            .get(&Id::from_u64((our_win_id.raw() << 8) | 4))
            .copied()
            .unwrap_or(0.0);

        // Check active modal blocking
        for (&other_id, _) in self.screen_layout_cache {
            let other_win_id = self.widget_window_map.get(&other_id).copied().unwrap_or(other_id);
            let modal_key = Id::from_u64((other_win_id.raw() << 8) | 5);
            let is_modal = self.interaction_state
                .get(&modal_key)
                .map(|&v| v > 0.5)
                .unwrap_or(false);

            if is_modal && other_win_id != our_win_id {
                return true;
            }
        }

        // Check z-order occlusion
        for (&other_id, other_rect) in self.screen_layout_cache {
            let other_win_id = self.widget_window_map.get(&other_id).copied().unwrap_or(other_id);
            let other_z_key = Id::from_u64((other_win_id.raw() << 8) | 4);
            if let Some(&other_z) = self.interaction_state.get(&other_z_key) {
                if other_win_id != our_win_id && other_z > our_z {
                    if x >= other_rect.origin.x && x <= other_rect.origin.x + other_rect.size.width &&
                       y >= other_rect.origin.y && y <= other_rect.origin.y + other_rect.size.height {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn is_hovered(&self, id: Id, fallback_x: f32, fallback_y: f32, fallback_w: f32, fallback_h: f32) -> bool {
        let is_in = if let Some((rect, _)) = self.get_recorded_layout(id) {
            self.mouse_in_rect(rect.origin.x + self.offset_x, rect.origin.y + self.offset_y, rect.size.width, rect.size.height)
        } else {
            self.mouse_in_rect(fallback_x, fallback_y, fallback_w, fallback_h)
        };
        let mouse_in_viewport = self.current_viewport.contains(zenthra_core::Point::new(self.mouse_x, self.mouse_y));
        is_in && mouse_in_viewport && !self.is_occluded(id, self.mouse_x, self.mouse_y)
    }

    pub fn overlay<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Ui),
    {
        // Swap draws and overlays so that any pushes to self.draws go to self.overlays
        std::mem::swap(&mut self.draws, &mut self.overlays);

        self.skip_clip_stack.push(true);
        let prev_viewport = self.current_viewport;
        self.current_viewport = Rect::new(-100000.0, -100000.0, 2000000.0, 2000000.0);

        f(self);

        self.current_viewport = prev_viewport;
        self.skip_clip_stack.pop();

        // Swap back to restore self.draws and commit new draws to self.overlays
        std::mem::swap(&mut self.draws, &mut self.overlays);
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


    pub fn input<'b>(&mut self, buffer: &'b mut String, id: impl std::hash::Hash) -> crate::input::InputBuilder<'_, 'a, 'b> {
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

    pub fn text_area<'b>(&mut self, buffer: &'b mut String, id: impl std::hash::Hash) -> crate::text_area::TextAreaBuilder<'_, 'a, 'b> {
        crate::text_area::TextAreaBuilder::new(self, buffer).id(id)
    }

    pub fn window<'b>(&mut self, title: &str, is_open: &'b mut bool, pos: &'b mut [f32; 2]) -> crate::window::FloatingWindowBuilder<'_, 'a, 'b> {
        crate::window::FloatingWindowBuilder::new(self, title, is_open, pos)
    }

    pub fn menu_bar(&mut self) -> crate::controls::menu::MenuBarBuilder<'_, 'a> {
        crate::controls::menu::MenuBarBuilder::new(self)
    }

    pub fn menu(&mut self, label: &str) -> crate::controls::menu::MenuBuilder<'_, 'a> {
        crate::controls::menu::MenuBuilder::new(self, label)
    }

    pub fn sub_menu(&mut self, label: &str) -> crate::controls::menu::SubMenuBuilder<'_, 'a> {
        crate::controls::menu::SubMenuBuilder::new(self, label)
    }

    pub fn menu_item(&mut self, label: &str) -> crate::controls::menu::MenuItemBuilder<'_, 'a> {
        crate::controls::menu::MenuItemBuilder::new(self, label)
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
        self.text(content).size(40.0).bold()
    }

    pub fn h2(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(32.0).bold()
    }

    pub fn h3(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(24.0).bold()
    }

    pub fn h4(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(20.0).bold()
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
            .full_width()
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
                            .weight(crate::text::FontWeight::Bold)
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
            .full_width()
            .row()
            .show(f);
    }

    pub fn card_divider(&mut self) {
        self.spacing(8.0);
        self.container()
            .full_width()
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
            None, false, 0.0,
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
            None, false, 0.0,
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
}
