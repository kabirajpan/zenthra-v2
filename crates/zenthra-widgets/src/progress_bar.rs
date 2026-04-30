use crate::ui::{Ui, DrawCommand, RectDraw};
use zenthra_core::{Color, EdgeInsets, Response};
use zenthra_render::RectInstance;

pub struct ProgressBarBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    value: f32, // 0.0 to 1.0
    
    // 1. Overall Widget Style
    width: Option<f32>,
    height: Option<f32>,
    padding: EdgeInsets,
    bg: Option<Color>,
    radius: [f32; 4],
    border_color: Option<Color>,
    border_width: f32,
    opacity: f32,

    // 2. Track Style (The background of the bar)
    track_h: f32,
    track_color: Color,

    // 3. Fill Style (The actual progress)
    fill_color: Color,
    
    // 4. Shadows
    shadow_color: Option<Color>,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,

    fill_shadow_color: Option<Color>,
    fill_shadow_offset: [f32; 2],
    fill_shadow_blur: f32,
    fill_shadow_opacity: f32,

    // 5. Effects
    shimmer: bool,
}

impl<'u, 'a> ProgressBarBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, value: f32) -> Self {
        Self {
            ui,
            value: value.clamp(0.0, 1.0),
            
            width: None,
            height: None,
            padding: EdgeInsets::symmetric(4.0, 4.0),
            bg: None,
            radius: [0.0; 4],
            border_color: None,
            border_width: 0.0,
            opacity: 1.0,

            track_h: 8.0,
            track_color: Color::rgb(0.15, 0.15, 0.15),

            fill_color: Color::rgb(0.4, 0.6, 1.0),

            shadow_color: None,
            shadow_offset: [0.0, 2.0],
            shadow_blur: 5.0,
            shadow_opacity: 0.3,

            fill_shadow_color: None,
            fill_shadow_offset: [0.0, 1.0],
            fill_shadow_blur: 3.0,
            fill_shadow_opacity: 0.4,
            
            shimmer: false,
        }
    }

    // --- 1. Overall Widget Methods ---
    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = Some(w);
        self.height = Some(h);
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

    pub fn track_size(mut self, w: f32, h: f32) -> Self {
        self.width = Some(w);
        self.track_h = h;
        self
    }

    pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.radius = [tl, tr, br, bl];
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self
    }

    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = o;
        self
    }

    pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
        self.padding = EdgeInsets { top: t, right: r, bottom: b, left: l };
        self
    }

    // --- 2. Track Methods ---
    pub fn track_height(mut self, h: f32) -> Self {
        self.track_h = h;
        self
    }

    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    // --- 3. Fill Methods ---
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    // --- 4. Shadow Methods ---
    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.shadow_color = Some(color);
        self.shadow_offset = [x, y];
        self.shadow_blur = blur;
        self
    }

    pub fn shadow_opacity(mut self, opacity: f32) -> Self {
        self.shadow_opacity = opacity;
        self
    }

    pub fn fill_shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.fill_shadow_color = Some(color);
        self.fill_shadow_offset = [x, y];
        self.fill_shadow_blur = blur;
        self
    }

    pub fn fill_shadow_opacity(mut self, opacity: f32) -> Self {
        self.fill_shadow_opacity = opacity;
        self
    }

    pub fn shimmer(mut self, enabled: bool) -> Self {
        self.shimmer = enabled;
        self
    }

    pub fn show(self) -> Response {
        let x = self.ui.cursor_x;
        let y = self.ui.cursor_y;
        let w = self.width.unwrap_or(self.ui.available_width.max(100.0));
        
        let h = self.height.unwrap_or_else(|| {
            self.track_h.max(12.0) + self.padding.vertical()
        });

        let is_hovered = self.ui.mouse_in_rect(x + self.ui.offset_x, y + self.ui.offset_y, w, h);
        let start_draw = self.ui.draws.len();

        // 0. Background
        if let Some(bg) = self.bg {
            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [x, y],
                    size: [w, h],
                    color: bg.to_array(),
                    radius: self.radius,
                    border_width: self.border_width,
                    border_color: self.border_color.map(|c| c.to_array()).unwrap_or([0.0; 4]),
                    opacity: self.opacity,
                    ..Default::default()
                }
            }));
        }

        // 1. Draw Track
        let track_w = w - self.padding.horizontal();
        let track_x = x + self.padding.left;
        let ty = y + (h - self.track_h) / 2.0;

        let mut track_shadow_color = [0.0; 4];
        if let Some(c) = self.shadow_color {
            track_shadow_color = c.to_array();
            track_shadow_color[3] *= self.shadow_opacity;
        }

        self.ui.draws.push(DrawCommand::Rect(RectDraw {
            instance: RectInstance {
                pos: [track_x, ty],
                size: [track_w, self.track_h],
                color: self.track_color.to_array(),
                radius: self.radius,
                opacity: self.opacity,
                shadow_color: track_shadow_color,
                shadow_offset: self.shadow_offset,
                shadow_blur: self.shadow_blur,
                ..Default::default()
            }
        }));

        // 2. Draw Fill
        let fill_w = track_w * self.value;
        if fill_w > 0.1 {
            let mut fill_shadow_color = [0.0; 4];
            if let Some(c) = self.fill_shadow_color {
                fill_shadow_color = c.to_array();
                fill_shadow_color[3] *= self.fill_shadow_opacity;
            }

            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: RectInstance {
                    pos: [track_x, ty],
                    size: [fill_w, self.track_h],
                    color: self.fill_color.to_array(),
                    radius: self.radius,
                    opacity: self.opacity,
                    shadow_color: fill_shadow_color,
                    shadow_offset: self.fill_shadow_offset,
                    shadow_blur: self.fill_shadow_blur,
                    ..Default::default()
                }
            }));

            // 3. Shimmer Effect
            if self.shimmer {
                let shimmer_t = (self.ui.elapsed_time * 1.5) % 2.0 - 0.5; // Moves from -0.5 to 1.5
                let shimmer_x = track_x + shimmer_t * track_w;
                let shimmer_w = track_w * 0.2;
                
                if shimmer_x + shimmer_w > track_x && shimmer_x < track_x + fill_w {
                    // Clip shimmer to the fill bar
                    let actual_shimmer_x = shimmer_x.max(track_x);
                    let actual_shimmer_w = (shimmer_x + shimmer_w).min(track_x + fill_w) - actual_shimmer_x;

                    if actual_shimmer_w > 0.0 {
                        self.ui.draws.push(DrawCommand::Rect(RectDraw {
                            instance: RectInstance {
                                pos: [actual_shimmer_x, ty],
                                size: [actual_shimmer_w, self.track_h],
                                color: [1.0, 1.0, 1.0, 0.2],
                                radius: self.radius,
                                brightness: 1.5,
                                ..Default::default()
                            }
                        }));
                        self.ui.request_redraw(); // Keep animating
                    }
                }
            }
        }

        self.ui.advance(w, h, start_draw);
        
        Response {
            clicked: self.ui.clicked && is_hovered,
            hovered: is_hovered,
            pressed: self.ui.mouse_down && is_hovered,
        }
    }
}
