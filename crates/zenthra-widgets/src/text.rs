use zenthra_text::prelude::*;
pub use zenthra_text::prelude::FontWeight;
// use zenthra_text::traits::FontProvider;
use crate::ui::{DrawCommand, TextDraw, Ui};
use zenthra_core::{Color, EdgeInsets, Role, SemanticNode, Rect, Align, Id};

pub struct TextBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,
    content: String,
    options: TextOptions,
    
    // Container/Widget-level styling
    padding: Padding,
    bg_color: Option<Color>,
    fill_x: bool,
    
    margin: EdgeInsets,
    radius: [f32; 4],
    border_color: Option<Color>,
    border_width: f32,
    shadow_color: Option<Color>,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,
    opacity: f32,
    cursor: CursorIcon,
    render_mode: Option<zenthra_core::RenderMode>,
    start_x: f32,
    start_y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorIcon {
    Default,
    Text,
    Pointer,
    Crosshair,
}

impl<'u, 'a> TextBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, content: &str) -> Self {
        let x = ui.cursor_x;
        let y = ui.cursor_y;
        let sf = ui.scale_factor;
        
        // --- STABLE DETERMINISTIC ID ---
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        content.hash(&mut hasher);
        // Include parent ID in hash to differentiate same text in different containers
        if let Some(parent) = ui.semantic_stack.last() {
            parent.hash(&mut hasher);
        }
        let id_raw = hasher.finish();
        let id = Id::from_u64(id_raw);
        
        
        Self {
            ui,
            id,
            content: content.to_string(),
            options: TextOptions::new()
                .at(0.0, 0.0) // Relative to pos
                .scale_factor(sf),
            padding: Padding::ZERO,
            bg_color: None,
            fill_x: false,
            margin: EdgeInsets::ZERO,
            radius: [0.0; 4],
            border_color: None,
            border_width: 0.0,
            shadow_color: None,
            shadow_offset: [0.0; 2],
            shadow_blur: 0.0,
            shadow_opacity: 1.0,
            opacity: 1.0,
            cursor: CursorIcon::Default,
            render_mode: None,
            start_x: x,
            start_y: y,
        }
    }

    pub fn min_width(mut self, w: f32) -> Self {
        self.options = self.options.min_width(w);
        self
    }

    pub fn size(mut self, s: f32) -> Self {
        self.options = self.options.font_size(s);
        self
    }
    pub fn color(mut self, c: Color) -> Self {
        self.options = self.options.color(c);
        self
    }
    pub fn weight(mut self, w: impl Into<FontWeight>) -> Self {
        self.options = self.options.font_weight(w.into());
        self
    }
    pub fn bold(mut self) -> Self {
        self.options = self.options.font_weight(FontWeight::Bold);
        self
    }
    pub fn italic(mut self) -> Self {
        self.options = self.options.font_style(FontStyle::Italic);
        self
    }
    pub fn family(mut self, f: impl Into<String>) -> Self {
        self.options = self.options.font_family(f);
        self
    }
    pub fn monospace(mut self) -> Self {
        self.options = self.options.font_family("monospace");
        self
    }
    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.start_x = x;
        self.start_y = y;
        self.options = self.options.at(0.0, 0.0);
        self
    }
    pub fn max_width(mut self, w: f32) -> Self {
        self.options = self.options.max_width(w);
        self
    }

    pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.padding = Padding { top: t, right: r, bottom: b, left: l };
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

    pub fn line_height(mut self, lh: f32) -> Self {
        self.options = self.options.line_height(lh);
        self
    }

    pub fn bg(mut self, c: Color) -> Self {
        self.bg_color = Some(c);
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self
    }

    pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.radius = [tl, tr, br, bl];
        self
    }

    pub fn radius_all(mut self, r: f32) -> Self {
        self.radius = [r; 4];
        self
    }

    pub fn radius_top(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self.radius[1] = r;
        self
    }

    pub fn radius_bottom(mut self, r: f32) -> Self {
        self.radius[2] = r;
        self.radius[3] = r;
        self
    }

    pub fn radius_top_left(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self
    }

    pub fn radius_top_right(mut self, r: f32) -> Self {
        self.radius[1] = r;
        self
    }

    pub fn radius_bottom_right(mut self, r: f32) -> Self {
        self.radius[2] = r;
        self
    }

    pub fn radius_bottom_left(mut self, r: f32) -> Self {
        self.radius[3] = r;
        self
    }

    pub fn radius_left(mut self, r: f32) -> Self {
        self.radius[0] = r;
        self.radius[3] = r;
        self
    }

    pub fn radius_right(mut self, r: f32) -> Self {
        self.radius[1] = r;
        self.radius[2] = r;
        self
    }

    pub fn shadow(mut self, color: Color, ox: f32, oy: f32, blur: f32) -> Self {
        self.shadow_color = Some(color);
        self.shadow_offset = [ox, oy];
        self.shadow_blur = blur;
        self
    }

    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = o;
        self
    }

    pub fn highlight(mut self, c: Color) -> Self {
        self.options = self.options.highlight(c);
        self
    }

    pub fn fill_x(mut self, enabled: bool) -> Self {
        self.fill_x = enabled;
        self
    }

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
    
    pub fn wrap(mut self, strategy: TextWrap) -> Self {
        self.options = self.options.wrap(strategy);
        self
    }

    pub fn align(mut self, alignment: Align) -> Self {
        let halign = match alignment {
            Align::Left => HorizontalAlignment::Left,
            Align::Center => HorizontalAlignment::Center,
            Align::Right => HorizontalAlignment::Right,
            _ => HorizontalAlignment::Left,
        };
        self.options = self.options.align(halign);
        self
    }

    pub fn align_left(mut self) -> Self {
        self.options = self.options.align(HorizontalAlignment::Left);
        self
    }

    pub fn align_center(mut self) -> Self {
        self.options = self.options.align(HorizontalAlignment::Center);
        self
    }

    pub fn align_right(mut self) -> Self {
        self.options = self.options.align(HorizontalAlignment::Right);
        self
    }

    pub fn halign(self, alignment: Align) -> Self {
        self.align(alignment)
    }

    pub fn valign(mut self, alignment: Align) -> Self {
        let valign = match alignment {
            Align::Top => VerticalAlignment::Top,
            Align::Center => VerticalAlignment::Center,
            Align::Bottom => VerticalAlignment::Bottom,
            _ => VerticalAlignment::Top,
        };
        self.options = self.options.valign(valign);
        self
    }
    
    pub fn cursor(mut self, c: CursorIcon) -> Self {
        self.cursor = c;
        self
    }
    pub fn cursor_text(mut self) -> Self {
        self.cursor = CursorIcon::Text;
        self
    }
    pub fn cursor_pointer(mut self) -> Self {
        self.cursor = CursorIcon::Pointer;
        self
    }

    pub fn clip_rect(mut self, x: f32, y: f32, w: f32, h: f32) -> Self {
        self.options = self.options.clip_rect(x, y, w, h);
        self
    }

    pub fn render_mode(mut self, mode: zenthra_core::RenderMode) -> Self {
        self.render_mode = Some(mode);
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

    pub fn show(mut self) -> (zenthra_core::Response, Option<ShapedBuffer>) {
        if let Some(mode) = self.render_mode {
            self.ui.render_mode_stack.push(mode);
        }

        let horiz = self.margin.horizontal();
        let vert = self.margin.vertical();
        let (w, h, buffer, start) = self.draw_and_measure();
        
        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Label, Rect::new(self.start_x, self.start_y, w, h))
                .with_label(self.content.clone())
        );

        self.ui.advance(w + horiz, h + vert, start);
        
        if self.render_mode.is_some() {
            self.ui.render_mode_stack.pop();
        }

        let is_hovered = self.ui.mouse_in_rect(self.start_x + self.ui.offset_x, self.start_y + self.ui.offset_y, w, h);
        let response = zenthra_core::Response {
            clicked: self.ui.clicked && is_hovered,
            hovered: is_hovered,
            pressed: is_hovered && self.ui.mouse_down,
        };

        (response, buffer)
    }

    pub fn draw_and_measure(&mut self) -> (f32, f32, Option<ShapedBuffer>, usize) {
        let (w, h, buffer) = if let Some(fs) = self.ui.font_system.as_ref() {
             let mut adapter = CosmicFontProvider::new_with_system(fs.clone());
             
             // Use explicitly set max_width if available, otherwise fallback to container width
             let layout_width = self.options.max_width.unwrap_or_else(|| {
                 (self.ui.available_width - self.padding.horizontal()).max(0.0)
             });
             
             self.options.max_width = Some(layout_width);
             
             adapter.set_layout_size(layout_width, self.ui.height);
             
             let buffer = adapter.shape(&self.content, &self.options);
             let (cw, ch) = buffer.size();

             // Important: record layout for next frame culling
             let mut w = cw + self.padding.horizontal();
             if self.fill_x {
                 w = self.ui.max_x - self.start_x;
             } else if let Some(min_w) = self.options.min_width {
                 if w < min_w { w = min_w; }
             }
             let h = ch + self.padding.vertical();
             self.ui.record_layout(self.id, Rect::new(self.start_x, self.start_y, w, h));

             (w, h, Some(buffer))
        } else {
            (100.0, 20.0, None)
        };

        let start_draw = self.ui.draws.len();
        let clip = self.options.clip_rect.unwrap_or([-100000.0, -100000.0, 2000000.0, 2000000.0]);

        // Background
        if let Some(bg) = self.bg_color {
            use zenthra_render::RectInstance;
            use crate::ui::RectDraw;

            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [self.start_x, self.start_y],
                    size: [w, h],
                    color: bg.to_array(),
                    radius: [
                        self.radius[3],
                        self.radius[2],
                        self.radius[1],
                        self.radius[0],
                    ],
                    border_width: self.border_width,
                    border_color: self.border_color.unwrap_or(Color::TRANSPARENT).to_array(),
                    shadow_color: self.shadow_color.map(|mut c| { c.a *= self.shadow_opacity; c.to_array() }).unwrap_or([0.0; 4]),
                    shadow_offset: self.shadow_offset,
                    shadow_blur: self.shadow_blur,
                    clip_rect: clip,
                    grayscale: 0.0,
                    brightness: 1.0,
                    opacity: self.opacity,
                    ..Default::default()
                }
            }));
        }

        // --- 2. Draw Text (Padded) ---
        self.ui.draws.push(DrawCommand::Text(TextDraw {
            text: self.content.clone().into(),
            pos: [self.start_x + self.padding.left, self.start_y + self.padding.top],
            options: self.options.clone(),
            clip,
        }));

        (w, h, buffer, start_draw)
    }
}
