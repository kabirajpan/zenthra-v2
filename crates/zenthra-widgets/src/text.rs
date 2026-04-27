use zenthra_text::prelude::*;
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
    full_width_bg: bool,
    
    margin: EdgeInsets,
    bg_radius: f32, // Not currently in Zentype but used in Zenthra
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
        ui.id_log.push(id);
        
        
        Self {
            ui,
            id,
            content: content.to_string(),
            options: TextOptions::new()
                .at(0.0, 0.0) // Relative to pos
                .scale_factor(sf),
            padding: Padding::ZERO,
            bg_color: None,
            full_width_bg: false,
            margin: EdgeInsets::ZERO,
            bg_radius: 0.0,
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
    pub fn weight(mut self, w: FontWeight) -> Self {
        self.options = self.options.font_weight(w);
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

    pub fn highlight(mut self, c: Color) -> Self {
        self.options = self.options.highlight(c);
        self
    }

    pub fn full_width_bg(mut self, enabled: bool) -> Self {
        self.full_width_bg = enabled;
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

    pub fn show(mut self) -> Option<ShapedBuffer> {
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

        buffer
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
             if self.full_width_bg {
                 w = self.ui.width - self.start_x;
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
        let clip = self.options.clip_rect.unwrap_or([0.0, 0.0, self.ui.width, self.ui.height]);

        // Background
        if let Some(bg) = self.bg_color {
            use zenthra_render::RectInstance;
            use crate::ui::RectDraw;

            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [self.start_x, self.start_y],
                    size: [w, h],
                    color: bg.to_array(),
                    radius: [self.bg_radius; 4],
                    border_width: 0.0,
                    border_color: [0.0, 0.0, 0.0, 0.0],
                    shadow_color: [0.0, 0.0, 0.0, 0.0],
                    shadow_offset: [0.0, 0.0],
                    shadow_blur: 0.0,
                    clip_rect: clip,
                    grayscale: 0.0,
                    brightness: 1.0,
                    opacity: 1.0,
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
