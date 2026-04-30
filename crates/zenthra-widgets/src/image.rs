use crate::ui::{DrawCommand, ImageDraw, Ui};
use zenthra_core::{Color, EdgeInsets, Id, Rect, Role, SemanticNode, ImageSource, ObjectFit};
use zenthra_render::ImageInstance;

pub struct ImageBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,
    source: ImageSource,

    // Sizing & Position
    pos: Option<(f32, f32)>,
    width: Option<f32>,
    height: Option<f32>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    aspect_ratio: Option<f32>,
    original_size: bool,

    // Spacing
    padding: EdgeInsets,
    margin: EdgeInsets,

    // Appearance (Idle State)
    fit: ObjectFit,
    radius: [f32; 4],
    border_color: Color,
    border_width: f32,
    bg: Color,
    opacity: f32,
    grayscale: f32,
    shadow_color: Color,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,

    // Interaction Styling (Hover & Active States)
    hover_opacity: Option<f32>,
    hover_grayscale: Option<f32>,
    hover_border: Option<Color>,
    active_opacity: Option<f32>,

    // Advanced Controls
    internal_scale: [f32; 2],
    internal_offset: [f32; 2],
    rotation: [f32; 3], // (x, y, z)
    flip_h: bool,
    flip_v: bool,

    // Framework Mechanics
    cursor: crate::text::CursorIcon,
    render_mode: Option<zenthra_core::RenderMode>,
}

impl<'u, 'a> ImageBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, source: ImageSource) -> Self {
        let id = ui.id();
        Self {
            ui,
            id,
            source,
            pos: None,
            width: None,
            height: None,
            max_width: None,
            max_height: None,
            padding: EdgeInsets::ZERO,
            margin: EdgeInsets::ZERO,
            fit: ObjectFit::Contain,
            radius: [0.0; 4],
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            bg: Color::TRANSPARENT,
            opacity: 1.0,
            grayscale: 0.0,
            shadow_color: Color::TRANSPARENT,
            shadow_offset: [0.0, 0.0],
            shadow_blur: 0.0,
            shadow_opacity: 1.0,
            hover_opacity: None,
            hover_grayscale: None,
            hover_border: None,
            active_opacity: None,
            aspect_ratio: None,
            original_size: false,
            internal_scale: [1.0, 1.0],
            internal_offset: [0.0; 2],
            rotation: [0.0; 3],
            flip_h: false,
            flip_v: false,
            cursor: crate::text::CursorIcon::Default,
            render_mode: None,
        }
    }

    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.pos = Some((x, y));
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }
    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = Some(w);
        self.height = Some(h);
        self
    }
    pub fn max_width(mut self, w: f32) -> Self {
        self.max_width = Some(w);
        self
    }
    pub fn max_height(mut self, h: f32) -> Self {
        self.max_height = Some(h);
        self
    }
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = Some(ratio);
        self
    }
    pub fn original_size(mut self) -> Self {
        self.original_size = true;
        self
    }

    // Padding
    pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.padding = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }
    pub fn padding_x(mut self, p: f32) -> Self {
        self.padding.left = p;
        self.padding.right = p;
        self
    }
    pub fn padding_y(mut self, p: f32) -> Self {
        self.padding.top = p;
        self.padding.bottom = p;
        self
    }
    pub fn padding_top(mut self, p: f32) -> Self {
        self.padding.top = p;
        self
    }
    pub fn padding_bottom(mut self, p: f32) -> Self {
        self.padding.bottom = p;
        self
    }
    pub fn padding_left(mut self, p: f32) -> Self {
        self.padding.left = p;
        self
    }
    pub fn padding_right(mut self, p: f32) -> Self {
        self.padding.right = p;
        self
    }

    // Margin
    pub fn margin(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.margin = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }
    pub fn margin_x(mut self, m: f32) -> Self {
        self.margin.left = m;
        self.margin.right = m;
        self
    }
    pub fn margin_y(mut self, m: f32) -> Self {
        self.margin.top = m;
        self.margin.bottom = m;
        self
    }
    pub fn margin_top(mut self, m: f32) -> Self {
        self.margin.top = m;
        self
    }
    pub fn margin_bottom(mut self, m: f32) -> Self {
        self.margin.bottom = m;
        self
    }
    pub fn margin_left(mut self, m: f32) -> Self {
        self.margin.left = m;
        self
    }
    pub fn margin_right(mut self, m: f32) -> Self {
        self.margin.right = m;
        self
    }

    // Appearance
    pub fn fit(mut self, fit: ObjectFit) -> Self {
        self.fit = fit;
        self
    }
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.radius = [radius; 4];
        self
    }
    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = color;
        self.border_width = width;
        self
    }
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
    pub fn opacity(mut self, alpha: f32) -> Self {
        self.opacity = alpha;
        self
    }
    pub fn grayscale(mut self, amount: f32) -> Self {
        self.grayscale = amount;
        self
    }
    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.shadow_color = color;
        self.shadow_offset = [x, y];
        self.shadow_blur = blur;
        self
    }
    pub fn shadow_opacity(mut self, opacity: f32) -> Self {
        self.shadow_opacity = opacity;
        self
    }

    // Advanced Controls
    pub fn scale(mut self, s: f32) -> Self {
        self.internal_scale = [s, s];
        self
    }
    pub fn scale_x(mut self, x: f32) -> Self {
        self.internal_scale[0] = x;
        self
    }
    pub fn scale_y(mut self, y: f32) -> Self {
        self.internal_scale[1] = y;
        self
    }
    pub fn zoom(mut self, z: f32) -> Self {
        self.internal_scale = [z, z];
        self
    }
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.internal_offset = [x, y];
        self
    }
    pub fn rotate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.rotation = [x.to_radians(), y.to_radians(), z.to_radians()];
        self
    }
    pub fn rotate_x(mut self, x: f32) -> Self {
        self.rotation[0] = x.to_radians();
        self
    }
    pub fn rotate_y(mut self, y: f32) -> Self {
        self.rotation[1] = y.to_radians();
        self
    }
    pub fn rotate_z(mut self, z: f32) -> Self {
        self.rotation[2] = z.to_radians();
        self
    }
    pub fn flip_h(mut self, flip: bool) -> Self {
        self.flip_h = flip;
        self
    }
    pub fn flip_v(mut self, flip: bool) -> Self {
        self.flip_v = flip;
        self
    }

    // Interaction Styling
    pub fn hover_opacity(mut self, alpha: f32) -> Self {
        self.hover_opacity = Some(alpha);
        self
    }
    pub fn hover_grayscale(mut self, amount: f32) -> Self {
        self.hover_grayscale = Some(amount);
        self
    }
    pub fn hover_border(mut self, color: Color) -> Self {
        self.hover_border = Some(color);
        self
    }
    pub fn active_opacity(mut self, alpha: f32) -> Self {
        self.active_opacity = Some(alpha);
        self
    }

    // Framework
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        id.hash(&mut hasher);
        if let Some(parent) = self.ui.semantic_stack.last() {
            parent.hash(&mut hasher);
        }
        self.id = zenthra_core::Id::from_u64(hasher.finish());
        self
    }
    pub fn cursor(mut self, c: crate::text::CursorIcon) -> Self {
        self.cursor = c;
        self
    }
    pub fn continuous(mut self) -> Self {
        self.render_mode = Some(zenthra_core::RenderMode::Continuous);
        self
    }
    pub fn static_mode(mut self) -> Self {
        self.render_mode = Some(zenthra_core::RenderMode::Static);
        self
    }

    pub fn show(self) -> zenthra_core::Response {
        if let Some(mode) = self.render_mode {
            self.ui.render_mode_stack.push(mode);
        }

        let start_x = self.pos.map(|p| p.0).unwrap_or(self.ui.cursor_x);
        let start_y = self.pos.map(|p| p.1).unwrap_or(self.ui.cursor_y);

        let (orig_w, orig_h) = self.ui.image_sizes.get(&self.source)
            .map(|(w, h)| (*w as f32, *h as f32))
            .unwrap_or((100.0, 100.0));

        let ratio = self.aspect_ratio.unwrap_or(orig_w / orig_h);

        let (final_w, final_h) = if self.original_size {
            (orig_w, orig_h)
        } else {
            match (self.width, self.height) {
                (Some(w), Some(h)) => (w, h),
                (Some(w), None) => (w, w / ratio),
                (None, Some(h)) => (h * ratio, h),
                (None, None) => (orig_w, orig_h),
            }
        };

        let w = if let Some(mw) = self.max_width { final_w.min(mw) } else { final_w };
        let h = if let Some(mh) = self.max_height { final_h.min(mh) } else { final_h };

        let draw_start = self.ui.draws.len();

        let ox = start_x + self.ui.offset_x + self.margin.left;
        let oy = start_y + self.ui.offset_y + self.margin.top;

        let is_hovered = if let Some((rect, _)) = self.ui.get_recorded_layout(self.id) {
            self.ui.mouse_in_rect(
                rect.origin.x + self.ui.offset_x,
                rect.origin.y + self.ui.offset_y,
                rect.size.width,
                rect.size.height,
            )
        } else {
            self.ui.mouse_in_rect(ox, oy, w, h)
        };

        let is_pressed = is_hovered && self.ui.mouse_down;
        let clicked = self.ui.clicked && is_hovered;

        // Interaction state overrides
        let current_opacity = if is_pressed {
            self.active_opacity.unwrap_or(self.opacity)
        } else if is_hovered {
            self.hover_opacity.unwrap_or(self.opacity)
        } else {
            self.opacity
        };

        let current_grayscale = if is_hovered && !is_pressed {
            self.hover_grayscale.unwrap_or(self.grayscale)
        } else {
            self.grayscale
        };

        let current_border_color = if is_hovered && !is_pressed {
            self.hover_border.unwrap_or(self.border_color)
        } else {
            self.border_color
        };

        let shadow_color_arr = {
            let mut c = self.shadow_color;
            c.a *= self.shadow_opacity;
            c.to_array()
        };

        self.ui.draws.push(DrawCommand::Image(ImageDraw {
            source: self.source.clone(),
            fit: self.fit,
            internal_scale: self.internal_scale,
            internal_offset: self.internal_offset,
            instance: ImageInstance {
                pos: [ox, oy],
                size: [w, h],
                radius: self.radius,
                border_width: self.border_width,
                border_color: current_border_color.to_array(),
                shadow_color: shadow_color_arr,
                shadow_offset: self.shadow_offset,
                shadow_blur: self.shadow_blur,
                clip_rect: [0.0, 0.0, 9999.0, 9999.0], // Managed by containers
                grayscale: current_grayscale,
                brightness: 1.0,
                opacity: current_opacity,
                uv_rect: [0.0, 0.0, 1.0, 1.0], // App texture manager will compute this based on ObjectFit
                bg_color: self.bg.to_array(),
                rotation: self.rotation,
                flip: [if self.flip_h { -1.0 } else { 1.0 }, if self.flip_v { -1.0 } else { 1.0 }],
            },
        }));

        let total_w = w + self.margin.horizontal();
        let total_h = h + self.margin.vertical();

        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Image, Rect::new(start_x, start_y, total_w, total_h))
        );

        self.ui.record_layout(self.id, Rect::new(start_x, start_y, total_w, total_h));
        
        // Only advance cursor if not absolutely positioned
        if self.pos.is_none() {
            self.ui.advance(total_w, total_h, draw_start);
        }

        if self.render_mode.is_some() {
            self.ui.render_mode_stack.pop();
        }

        zenthra_core::Response {
            clicked,
            hovered: is_hovered,
            pressed: is_pressed,
        }
    }
}
