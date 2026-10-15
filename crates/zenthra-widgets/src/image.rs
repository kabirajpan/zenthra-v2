use crate::ui::{DrawCommand, ImageDraw, RectDraw, Ui};
use zenthra_core::{
    Background, Border, BoxStyle, EdgeInsets, Id, ImageSource, ObjectFit, Rect, Response,
    Role, SemanticNode, Styleable,
};
use zenthra_render::ImageInstance;
use crate::style_ext::BoxStyleExt;

/// An interactive, hardware-accelerated image widget supporting responsive scaling,
/// object-fit modes, 3D perspective tilts, UV manipulation, and single-source `BoxStyle` styling.
pub struct ImageBuilder<'u, 'a> {
    ui: &'u mut Ui<'a>,
    id: Id,
    source: ImageSource,

    // Canonical BoxStyle for geometry, spacing, borders, background, shadows, opacity
    pub style: BoxStyle,

    // Sizing & Positioning specific to Image
    pos: Option<(f32, f32)>,
    aspect_ratio: Option<f32>,
    original_size: bool,

    // Image-specific appearance & filters
    fit: ObjectFit,
    grayscale: f32,

    // Interaction Styling (Hover & Active States)
    pub hover_state: Option<crate::macros::WidgetStateOverrides>,
    pub active_state: Option<crate::macros::WidgetStateOverrides>,

    // Advanced Texture & Transform Controls
    internal_scale: [f32; 2],
    internal_offset: [f32; 2],
    rotation: [f32; 3], // (x, y, z) in radians
    flip_h: bool,
    flip_v: bool,

    // Framework Mechanics
    cursor: crate::text::CursorIcon,
    render_mode: Option<zenthra_core::RenderMode>,
}

impl<'u, 'a> Styleable for ImageBuilder<'u, 'a> {
    #[inline]
    fn box_style_mut(&mut self) -> &mut BoxStyle {
        &mut self.style
    }

    #[inline]
    fn box_style(&self) -> &BoxStyle {
        &self.style
    }
}

// Single-source macro generates all canonical BoxStyle and Sibling state methods:
// .width(), .height(), .fill_x(), .fill_y(), .fill(), .min/max_width(), .min/max_height(),
// .padding(), .padding_all(), .padding_x(), .padding_y(), .padding_top/bottom/left/right(),
// .margin(), .margin_all(), .margin_x(), .margin_y(), .margin_top/bottom/left/right(),
// .radius(), .radius_all(), .radius_top/bottom/left/right(), .bg(), .border(), .shadow(), .style(),
// .hover([ ... ]), .active([ ... ])
impl_box_style_methods!(ImageBuilder, 'u, 'a);
impl_interaction_state_methods!(ImageBuilder, 'u, 'a);

impl<'u, 'a> ImageBuilder<'u, 'a> {
    pub fn new(ui: &'u mut Ui<'a>, source: ImageSource) -> Self {
        let id = ui.id();
        Self {
            ui,
            id,
            source,
            style: BoxStyle::new(),
            pos: None,
            aspect_ratio: None,
            original_size: false,
            fit: ObjectFit::Contain,
            grayscale: 0.0,
            hover_state: None,
            active_state: None,
            internal_scale: [1.0, 1.0],
            internal_offset: [0.0; 2],
            rotation: [0.0; 3],
            flip_h: false,
            flip_v: false,
            cursor: crate::text::CursorIcon::Default,
            render_mode: None,
        }
    }

    #[inline]
    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.pos = Some((x, y));
        self
    }

    #[inline]
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = Some(ratio);
        self
    }

    #[inline]
    pub fn original_size(mut self) -> Self {
        self.original_size = true;
        self
    }

    #[inline]
    pub fn fit(mut self, fit: ObjectFit) -> Self {
        self.fit = fit;
        self
    }

    #[inline]
    pub fn opacity(mut self, alpha: f32) -> Self {
        self.style.opacity = alpha;
        self
    }

    #[inline]
    pub fn grayscale(mut self, amount: f32) -> Self {
        self.grayscale = amount;
        self
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // ADVANCED UV & 3D PERSPECTIVE CONTROLS
    // ─────────────────────────────────────────────────────────────────────────────

    #[inline]
    pub fn scale(mut self, s: f32) -> Self {
        self.internal_scale = [s, s];
        self
    }

    #[inline]
    pub fn scale_x(mut self, x: f32) -> Self {
        self.internal_scale[0] = x;
        self
    }

    #[inline]
    pub fn scale_y(mut self, y: f32) -> Self {
        self.internal_scale[1] = y;
        self
    }

    #[inline]
    pub fn zoom(mut self, z: f32) -> Self {
        self.internal_scale = [z, z];
        self
    }

    #[inline]
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.internal_offset = [x, y];
        self
    }

    #[inline]
    pub fn rotate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.rotation = [x.to_radians(), y.to_radians(), z.to_radians()];
        self
    }

    #[inline]
    pub fn rotate_x(mut self, x: f32) -> Self {
        self.rotation[0] = x.to_radians();
        self
    }

    #[inline]
    pub fn rotate_y(mut self, y: f32) -> Self {
        self.rotation[1] = y.to_radians();
        self
    }

    #[inline]
    pub fn rotate_z(mut self, z: f32) -> Self {
        self.rotation[2] = z.to_radians();
        self
    }

    #[inline]
    pub fn flip_h(mut self, flip: bool) -> Self {
        self.flip_h = flip;
        self
    }

    #[inline]
    pub fn flip_v(mut self, flip: bool) -> Self {
        self.flip_v = flip;
        self
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // FRAMEWORK MECHANICS
    // ─────────────────────────────────────────────────────────────────────────────

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = self.ui.id_from(id);
        self
    }

    #[inline]
    pub fn cursor(mut self, c: crate::text::CursorIcon) -> Self {
        self.cursor = c;
        self
    }

    #[inline]
    pub fn continuous(mut self) -> Self {
        self.render_mode = Some(zenthra_core::RenderMode::Continuous);
        self
    }

    #[inline]
    pub fn static_mode(mut self) -> Self {
        self.render_mode = Some(zenthra_core::RenderMode::Static);
        self
    }

    pub fn show(self) -> Response {
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
            match (self.style.width, self.style.height) {
                (Some(w), Some(h)) => (w, h),
                (Some(w), None) => (w, w / ratio),
                (None, Some(h)) => (h * ratio, h),
                (None, None) => {
                    if orig_w > self.ui.available_width && self.ui.available_width > 0.0 {
                        (self.ui.available_width, self.ui.available_width / ratio)
                    } else {
                        (orig_w, orig_h)
                    }
                }
            }
        };

        let mut w = if let Some(mw) = self.style.max_width { final_w.min(mw) } else { final_w };
        let mut h = if let Some(mh) = self.style.max_height { final_h.min(mh) } else { final_h };
        if let Some(min_w) = self.style.min_width { w = w.max(min_w); }
        if let Some(min_h) = self.style.min_height { h = h.max(min_h); }

        let draw_start = self.ui.draws.len();

        let local_x = start_x + self.style.margin.left;
        let local_y = start_y + self.style.margin.top;

        let is_hovered = self.ui.is_hovered(self.id, local_x + self.ui.offset_x, local_y + self.ui.offset_y, w, h);
        let is_pressed = is_hovered && self.ui.mouse_down;
        let clicked = self.ui.clicked && is_hovered;

        // Interaction state overrides blending
        let mut effective_style = self.style.clone();
        let mut scale_mult = 1.0;
        let mut cursor_override = None;
        let mut tooltip_override = None;

        if is_pressed {
            if let Some(ref active) = self.active_state {
                if active.has_overrides {
                    effective_style = active.style.clone();
                    scale_mult = active.scale;
                    if let Some(c) = active.cursor {
                        cursor_override = Some(c);
                    }
                    if let Some(ref t) = active.tooltip {
                        tooltip_override = Some(t.clone());
                    }
                }
            } else if let Some(ref hover) = self.hover_state {
                if hover.has_overrides {
                    effective_style = hover.style.clone();
                    scale_mult = hover.scale;
                    if let Some(c) = hover.cursor {
                        cursor_override = Some(c);
                    }
                    if let Some(ref t) = hover.tooltip {
                        tooltip_override = Some(t.clone());
                    }
                }
            }
        } else if is_hovered {
            if let Some(ref hover) = self.hover_state {
                if hover.has_overrides {
                    effective_style = hover.style.clone();
                    scale_mult = hover.scale;
                    if let Some(c) = hover.cursor {
                        cursor_override = Some(c);
                    }
                    if let Some(ref t) = hover.tooltip {
                        tooltip_override = Some(t.clone());
                    }
                }
            }
        }

        if is_hovered {
            if let Some(c) = cursor_override {
                self.ui.cursor_icon = c;
            } else if self.cursor != crate::text::CursorIcon::Default {
                self.ui.cursor_icon = self.cursor;
            }
            if let Some(ref t) = tooltip_override {
                self.ui.draw_tooltip(t);
            }
        }

        let pad = effective_style.padding;
        let img_x = local_x + pad.left;
        let img_y = local_y + pad.top;
        let img_w = (w - pad.horizontal()).max(0.0);
        let img_h = (h - pad.vertical()).max(0.0);

        // If padding is active, render the outer box frame via BoxStyleExt
        let has_padding = pad != EdgeInsets::ZERO;
        if has_padding {
            effective_style.draw(&mut self.ui.draws, local_x, local_y, w, h);
        } else if let Some(Background::Gradient(_)) = &effective_style.bg {
            // If gradient background is configured without padding, draw background rect underneath
            let mut bg_style = effective_style.clone();
            bg_style.border = Border::None;
            bg_style.shadow_color = None;
            self.ui.draws.push(DrawCommand::Rect(RectDraw {
                instance: bg_style.build_rect_instance(local_x, local_y, w, h, None, None, None, 1.0),
            }));
        }

        // Texture UV Mapping
        let uv_rect = if img_w <= 0.0 || img_h <= 0.0 || orig_w <= 0.0 || orig_h <= 0.0 {
            [0.0, 0.0, 1.0, 1.0]
        } else {
            let r_t = orig_w / orig_h;
            let r_w = img_w / img_h;
            let base_uv = match self.fit {
                ObjectFit::Fill => [0.0, 0.0, 1.0, 1.0],
                ObjectFit::Cover => {
                    if r_t > r_w {
                        let u_size = r_w / r_t;
                        let u_start = (1.0 - u_size) / 2.0;
                        [u_start, 0.0, u_size, 1.0]
                    } else {
                        let v_size = r_t / r_w;
                        let v_start = (1.0 - v_size) / 2.0;
                        [0.0, v_start, 1.0, v_size]
                    }
                }
                ObjectFit::Contain => {
                    if r_t > r_w {
                        let v_size = r_t / r_w;
                        let v_start = -0.5 * (v_size - 1.0);
                        [0.0, v_start, 1.0, v_size]
                    } else {
                        let u_size = r_w / r_t;
                        let u_start = -0.5 * (u_size - 1.0);
                        [u_start, 0.0, u_size, 1.0]
                    }
                }
                ObjectFit::None => {
                    let u_size = img_w / orig_w;
                    let u_start = -0.5 * (u_size - 1.0);
                    let v_size = img_h / orig_h;
                    let v_start = -0.5 * (v_size - 1.0);
                    [u_start, v_start, u_size, v_size]
                }
                ObjectFit::ScaleDown => {
                    if orig_w > img_w || orig_h > img_h {
                        if r_t > r_w {
                            let v_size = r_t / r_w;
                            let v_start = -0.5 * (v_size - 1.0);
                            [0.0, v_start, 1.0, v_size]
                        } else {
                            let u_size = r_w / r_t;
                            let u_start = -0.5 * (u_size - 1.0);
                            [u_start, 0.0, u_size, 1.0]
                        }
                    } else {
                        let u_size = img_w / orig_w;
                        let u_start = -0.5 * (u_size - 1.0);
                        let v_size = img_h / orig_h;
                        let v_start = -0.5 * (v_size - 1.0);
                        [u_start, v_start, u_size, v_size]
                    }
                }
            };

            let scale_x = (self.internal_scale[0] * scale_mult).max(0.001);
            let scale_y = (self.internal_scale[1] * scale_mult).max(0.001);

            let new_u_size = base_uv[2] / scale_x;
            let new_v_size = base_uv[3] / scale_y;

            let new_u_start = base_uv[0] + (base_uv[2] - new_u_size) / 2.0;
            let new_v_start = base_uv[1] + (base_uv[3] - new_v_size) / 2.0;

            let final_u_start = new_u_start - (self.internal_offset[0] / img_w) * new_u_size;
            let final_v_start = new_v_start - (self.internal_offset[1] / img_h) * new_v_size;

            [final_u_start, final_v_start, new_u_size, new_v_size]
        };

        // Extract border & shadow parameters for ImageInstance
        let (border_color, border_width) = if has_padding {
            ([0.0; 4], 0.0) // Outer frame already rendered
        } else {
            effective_style.border_params()
        };

        let (shadow_color_arr, shadow_offset, shadow_blur) = if has_padding {
            ([0.0; 4], [0.0, 0.0], 0.0) // Outer frame already rendered
        } else {
            (
                effective_style.gpu_shadow_color(),
                effective_style.shadow_offset,
                effective_style.shadow_blur,
            )
        };

        let bg_color_arr = if has_padding {
            [0.0; 4] // Outer frame already rendered
        } else {
            match &effective_style.bg {
                Some(Background::Solid(c)) => c.to_array(),
                _ => [0.0; 4],
            }
        };

        self.ui.draws.push(DrawCommand::Image(ImageDraw {
            source: self.source.clone(),
            fit: self.fit,
            internal_scale: [self.internal_scale[0] * scale_mult, self.internal_scale[1] * scale_mult],
            internal_offset: self.internal_offset,
            instance: ImageInstance {
                pos: [img_x, img_y],
                size: [img_w, img_h],
                radius: effective_style.radius,
                border_width,
                border_color,
                shadow_color: shadow_color_arr,
                shadow_offset,
                shadow_blur,
                clip_rect: [0.0, 0.0, 9999.0, 9999.0],
                grayscale: self.grayscale,
                brightness: 1.0,
                opacity: effective_style.opacity,
                uv_rect,
                bg_color: bg_color_arr,
                rotation: self.rotation,
                flip: [if self.flip_h { -1.0 } else { 1.0 }, if self.flip_v { -1.0 } else { 1.0 }],
            },
        }));

        let total_w = w + self.style.margin.horizontal();
        let total_h = h + self.style.margin.vertical();

        self.ui.register_semantic(
            SemanticNode::new(self.id, Role::Image, Rect::new(start_x, start_y, total_w, total_h))
        );

        self.ui.record_layout(self.id, Rect::new(start_x, start_y, total_w, total_h));

        if self.pos.is_none() {
            self.ui.advance(total_w, total_h, draw_start);
        }

        if self.render_mode.is_some() {
            self.ui.render_mode_stack.pop();
        }

        Response {
            clicked,
            hovered: is_hovered,
            pressed: is_pressed,
            submitted: false,
            changed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use zenthra_core::{Background, Border, Color, ImageSource};
    use crate::macros::{Style, Transform};

    #[test]
    fn test_image_box_style_and_sibling_parity() {
        let mut scroll = HashMap::new();
        let mut cursor = HashMap::new();
        let mut inter = HashMap::new();
        let l_cache = HashMap::new();
        let mut nl_cache = HashMap::new();
        let sl_cache = HashMap::new();
        let mut nsl_cache = HashMap::new();
        let ww_map = HashMap::new();
        let mut nww_map = HashMap::new();
        let img = HashMap::new();

        let mut ui = Ui::new(
            800, 600, 1.0, None, Vec::new(), None,
            (0.0, 0.0), false,
            &mut scroll, &mut cursor, &mut inter,
            None, None, false, false, 0.0,
            &l_cache, &mut nl_cache,
            &sl_cache, &mut nsl_cache,
            &ww_map, &mut nww_map,
            &img,
        );

        let image = ui.image(ImageSource::Bytes(std::sync::Arc::from(&[][..])))
            .width(200.0)
            .height(150.0)
            .padding_all(4.0)
            .margin_all(8.0)
            .radius_all(12.0)
            .bg(Color::rgb(0.05, 0.05, 0.08))
            .border((Color::rgb(0.3, 0.4, 0.5), 2.0))
            .shadow(Color::rgba(0.0, 0.0, 0.0, 0.5), 0.0, 4.0, 8.0)
            .opacity(0.95)
            .fit(ObjectFit::Cover)
            .grayscale(0.5)
            .hover([
                Style::bg(Color::rgb(0.1, 0.1, 0.15)),
                Style::border((Color::rgb(0.4, 0.6, 0.8), 2.0)),
                Transform::scale(1.05),
            ]);

        assert_eq!(image.style.width, Some(200.0));
        assert_eq!(image.style.height, Some(150.0));
        assert_eq!(image.style.padding.top, 4.0);
        assert_eq!(image.style.margin.top, 8.0);
        assert_eq!(image.style.radius, [12.0; 4]);
        assert_eq!(image.style.bg, Some(Background::Solid(Color::rgb(0.05, 0.05, 0.08))));
        assert_eq!(image.style.border, Border::Solid(Color::rgb(0.3, 0.4, 0.5), 2.0));
        assert_eq!(image.style.opacity, 0.95);
        assert_eq!(image.fit, ObjectFit::Cover);
        assert_eq!(image.grayscale, 0.5);
        assert!(image.hover_state.is_some());
    }
}
