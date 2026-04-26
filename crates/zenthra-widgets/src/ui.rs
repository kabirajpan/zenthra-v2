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

pub enum DrawCommand {
    Rect(RectDraw),
    Text(TextDraw),
    OverlayRect(OverlayRectDraw),
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
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub base_x: f32,
    pub base_y: f32,
    pub direction: Direction,
    pub line_height: f32,
    pub child_sizes: Vec<(f32, f32)>,
    pub child_draw_ranges: Vec<(usize, usize)>,
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
    pub elapsed_time: f32,
    pub semantic_nodes: Vec<SemanticNode>,
    pub semantic_stack: Vec<Id>,
    pub render_mode_stack: Vec<zenthra_core::RenderMode>,
    pub needs_redraw: bool,
    pub layout_cache: &'a std::collections::HashMap<Id, (Rect, u64)>,
    pub next_layout_cache: &'a mut std::collections::HashMap<Id, (Rect, u64)>,
    pub available_width: f32,
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
        elapsed_time: f32,
        layout_cache: &'a std::collections::HashMap<Id, (Rect, u64)>,
        next_layout_cache: &'a mut std::collections::HashMap<Id, (Rect, u64)>,
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
            cursor_x: 0.0,
            cursor_y: 0.0,
            base_x: 0.0,
            base_y: 0.0,
            direction: Direction::Column,
            line_height: 0.0,
            child_sizes: Vec::new(),
            child_draw_ranges: Vec::new(),
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
            elapsed_time,
            semantic_nodes: Vec::new(),
            semantic_stack: Vec::new(),
            render_mode_stack: vec![zenthra_core::RenderMode::Static],
            needs_redraw: false,
            layout_cache,
            next_layout_cache,
            available_width: width as f32 / scale_factor as f32,
        }
    }

    pub fn id(&mut self) -> Id {
        self.id_counter += 1;
        let id = Id::from_u64(self.id_counter);
        self.id_log.push(id);
        id
    }

    pub fn record_layout(&mut self, id: Id, rect: Rect) {
        let id_count = self.id_counter.saturating_sub(id.raw());
        self.next_layout_cache.insert(id, (rect, id_count));
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

    pub fn continuous(&mut self) -> ContainerBuilder<'_, 'a> {
        ContainerBuilder::new(self).continuous()
    }

    pub fn static_mode(&mut self) -> ContainerBuilder<'_, 'a> {
        ContainerBuilder::new(self).static_mode()
    }

    pub fn button(&mut self, label: &str) -> crate::button::ButtonBuilder<'_, 'a> {
        crate::button::ButtonBuilder::new(self, label)
    }

    pub fn spacing(&mut self, size: f32) {
        let (w, h) = match self.direction {
            Direction::Column => (0.0, size),
            Direction::Row => (size, 0.0),
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
        let id = self.id();
        crate::input::InputBuilder::new(self, buffer, id)
    }

    pub fn text_area<'b>(&mut self, buffer: &'b mut String) -> crate::text_area::TextAreaBuilder<'_, 'a, 'b> {
        let id = self.id();
        crate::text_area::TextAreaBuilder::new(self, buffer, id)
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
}

/// A helper to get the default Zentype shaper for the current font system.
pub fn get_shaper(font_system: &Arc<Mutex<FontSystem>>) -> CosmicFontProvider {
    CosmicFontProvider::new_with_system(font_system.clone())
}
