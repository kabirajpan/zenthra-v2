use std::sync::{Arc, Mutex};
use zenthra_text::prelude::*;
use crate::container::{ContainerBuilder, Direction, Wrap};
use crate::text::{CursorIcon, TextBuilder};
use zenthra_core::Color;
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
    pub id: u64,
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
    pub child_draw_ranges: Vec<(usize, usize)>, // (start, end) index into draws
    pub last_w: f32,
    pub last_h: f32,
    pub last_ascent: f32,
    pub last_v_slop: f32,
    pub last_box_h: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub font_system: Option<Arc<Mutex<FontSystem>>>,
    pub input_events: Vec<PlatformEvent>,
    pub focused_id: Option<u64>,
    pub id_counter: u64,
    pub scroll_state: &'a mut std::collections::HashMap<u64, f32>,
    pub cursor_state: &'a mut std::collections::HashMap<u64, usize>,
    pub active_drag: Option<ScrollDrag>,
    pub clicked: bool,
    pub elapsed_time: f32,
}

impl<'a> Ui<'a> {
    pub fn new(
        width: u32,
        height: u32,
        scale_factor: f64,
        font_system: Option<Arc<Mutex<FontSystem>>>,
        events: Vec<PlatformEvent>,
        initial_focused_id: Option<u64>,
        mouse_pos: (f32, f32),
        mouse_down: bool,
        scroll_state: &'a mut std::collections::HashMap<u64, f32>,
        cursor_state: &'a mut std::collections::HashMap<u64, usize>,
        active_drag: Option<ScrollDrag>,
        clicked: bool,
        elapsed_time: f32,
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
            last_w: 0.0,
            last_h: 0.0,
            last_ascent: 0.0,
            last_v_slop: 0.0,
            last_box_h: 0.0,
            max_x: width as f32,
            max_y: height as f32,
            font_system,
            scroll_state,
            cursor_state,
            active_drag,
            clicked,
            elapsed_time,
        }
    }

    pub fn id(&mut self) -> u64 {
        self.id_counter += 1;
        self.id_counter
    }

    pub fn input<'b>(&mut self, buffer: &'b mut String) -> crate::input::InputBuilder<'_, 'a, 'b> {
        let id = self.id();
        crate::input::InputBuilder::new(self, buffer, id)
    }

    pub fn text_area<'b>(&mut self, buffer: &'b mut String) -> crate::text_area::TextAreaBuilder<'_, 'a, 'b> {
        let id = self.id();
        crate::text_area::TextAreaBuilder::new(self, buffer, id)
    }

    pub fn set_mouse(&mut self, x: f32, y: f32, down: bool) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.mouse_down = down;
    }

    pub fn mouse_in_rect(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.mouse_x >= x && self.mouse_x <= x + w &&
        self.mouse_y >= y && self.mouse_y <= y + h
    }

    /// Called by widgets after pushing their draws.
    /// Records draw range and advances cursor.
    pub fn advance(&mut self, w: f32, h: f32, draw_start: usize) {
        let draw_end = self.draws.len();
        self.child_draw_ranges.push((draw_start, draw_end));
        self.child_sizes.push((w, h));
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
        TextBuilder::new(self, content).full_width_bg(true)
    }

    pub fn h1(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(40.0).bold().full_width_bg(true)
    }

    pub fn h2(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(32.0).bold().full_width_bg(true)
    }

    pub fn h3(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(24.0).bold().full_width_bg(true)
    }

    pub fn h4(&mut self, content: &str) -> TextBuilder<'_, 'a> {
        self.text(content).size(20.0).bold().full_width_bg(true)
    }

    pub fn row<F>(&mut self, f: F) -> ContainerBuilder<'_, 'a>
    where
        F: FnOnce(&mut Ui),
    {
        self.container(Direction::Row, Wrap::NoWrap, f)
    }

    pub fn column<F>(&mut self, f: F) -> ContainerBuilder<'_, 'a>
    where
        F: FnOnce(&mut Ui),
    {
        self.container(Direction::Column, Wrap::NoWrap, f)
    }

    pub fn container<F>(
        &mut self,
        direction: Direction,
        wrap: Wrap,
        f: F,
    ) -> ContainerBuilder<'_, 'a>
    where
        F: FnOnce(&mut Ui),
    {
        let start_x = self.cursor_x;
        let start_y = self.cursor_y;

        // save parent state
        let prev_dir = self.direction;
        let prev_line_h = self.line_height;
        let prev_child_sizes = std::mem::take(&mut self.child_sizes);
        let prev_child_ranges = std::mem::take(&mut self.child_draw_ranges);
        let prev_base_x = self.base_x;
        let prev_base_y = self.base_y;

        // set direction and base before children run
        self.direction = direction;
        self.line_height = 0.0;
        self.base_x = start_x;
        self.base_y = start_y;

        // swap draws to buffer children
        let parent_draws = std::mem::take(&mut self.draws);

        f(self);

        // collect children results
        let children_draws = std::mem::replace(&mut self.draws, parent_draws);
        let child_sizes = std::mem::replace(&mut self.child_sizes, prev_child_sizes);
        let child_ranges = std::mem::replace(&mut self.child_draw_ranges, prev_child_ranges);

        // restore parent state
        self.direction = prev_dir;
        self.line_height = prev_line_h;
        self.cursor_x = start_x;
        self.cursor_y = start_y;
        self.base_x = prev_base_x;
        self.base_y = prev_base_y;

        ContainerBuilder::new(
            self,
            direction,
            wrap,
            children_draws,
            child_sizes,
            child_ranges,
            start_x,
            start_y,
        )
    }
}

/// A helper to get the default Zentype shaper for the current font system.
pub fn get_shaper(font_system: &Arc<Mutex<FontSystem>>) -> CosmicFontProvider {
    CosmicFontProvider::new_with_system(font_system.clone())
}
