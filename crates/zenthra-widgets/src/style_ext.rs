use zenthra_core::{Background, BoxStyle, Color};
use zenthra_render::RectInstance;
use crate::ui::{DrawCommand, RectDraw};

/// Resolves background styling into GPU shader parameters.
pub fn resolve_background(bg: &Option<Background>) -> (
    [f32; 4], // color
    [f32; 4], // color2
    [f32; 4], // color3
    [f32; 4], // color4
    [f32; 4], // gradient_params
    [f32; 4], // gradient_stops
    bool,     // is_visible
) {
    match bg {
        Some(Background::Solid(c)) => (
            c.to_array(),
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 0.0, 0.5, 0.5],
            [0.0, 0.333, 0.667, 1.0],
            c.a > 0.0,
        ),
        Some(Background::Gradient(g)) => match g {
            zenthra_core::Gradient::Linear { direction, stops } => {
                let angle_rad = direction.to_radians();
                let count = stops.len().clamp(2, 4);
                let c1 = stops.get(0).map(|s| s.color.to_array()).unwrap_or([0.0; 4]);
                let c2 = stops.get(1).map(|s| s.color.to_array()).unwrap_or(c1);
                let c3 = stops.get(2).map(|s| s.color.to_array()).unwrap_or(c2);
                let c4 = stops.get(3).map(|s| s.color.to_array()).unwrap_or(c3);
                let s0 = stops.get(0).map(|s| s.position).unwrap_or(0.0);
                let s1 = stops.get(1).map(|s| s.position).unwrap_or(if count == 2 { 1.0 } else { 0.5 });
                let s2 = stops.get(2).map(|s| s.position).unwrap_or(if count == 3 { 1.0 } else { 0.667 });
                let s3 = stops.get(3).map(|s| s.position).unwrap_or(1.0);
                (
                    c1, c2, c3, c4,
                    [1.0, angle_rad, count as f32, 0.0],
                    [s0, s1, s2, s3],
                    true,
                )
            }
            zenthra_core::Gradient::Radial { anchor, radius, stops } => {
                let count = stops.len().clamp(2, 4);
                let c1 = stops.get(0).map(|s| s.color.to_array()).unwrap_or([0.0; 4]);
                let c2 = stops.get(1).map(|s| s.color.to_array()).unwrap_or(c1);
                let c3 = stops.get(2).map(|s| s.color.to_array()).unwrap_or(c2);
                let c4 = stops.get(3).map(|s| s.color.to_array()).unwrap_or(c3);
                let s0 = stops.get(0).map(|s| s.position).unwrap_or(0.0);
                let s1 = stops.get(1).map(|s| s.position).unwrap_or(0.333);
                let s2 = stops.get(2).map(|s| s.position).unwrap_or(0.667);
                let _s3 = stops.get(3).map(|s| s.position).unwrap_or(1.0);
                (
                    c1, c2, c3, c4,
                    [2.0, count as f32, anchor.0, anchor.1],
                    [s0, s1, s2, *radius],
                    true,
                )
            }
            zenthra_core::Gradient::Mesh { points } => {
                let c1 = points.get(0).map(|p| p.1.to_array()).unwrap_or([1.0; 4]);
                let c2 = points.get(1).map(|p| p.1.to_array()).unwrap_or(c1);
                let c3 = points.get(2).map(|p| p.1.to_array()).unwrap_or(c1);
                let c4 = points.get(3).map(|p| p.1.to_array()).unwrap_or(c2);
                (
                    c1, c2, c3, c4,
                    [3.0, points.len() as f32, 0.5, 0.5],
                    [0.0, 0.333, 0.667, 1.0],
                    true,
                )
            }
        },
        None | Some(Background::None) => (
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 0.0, 0.5, 0.5],
            [0.0, 0.333, 0.667, 1.0],
            false,
        ),
    }
}

/// Unified rendering extension trait for `BoxStyle` across all widgets.
pub trait BoxStyleExt {
    /// Builds a `RectInstance` unconditionally for GPU quad rendering with full styling.
    fn build_rect_instance(
        &self,
        ox: f32,
        oy: f32,
        w: f32,
        h: f32,
        effective_bg: Option<&Background>,
        effective_border_color: Option<Color>,
        effective_border_width: Option<f32>,
        scale: f32,
    ) -> RectInstance;

    /// Builds a `RectInstance` for GPU quad rendering with optional interactive state overrides (hover/active),
    /// returning `None` if completely transparent and invisible.
    fn build_rect_styled(
        &self,
        ox: f32,
        oy: f32,
        w: f32,
        h: f32,
        effective_bg: Option<&Background>,
        effective_border_color: Option<Color>,
        effective_border_width: Option<f32>,
        scale: f32,
    ) -> Option<RectInstance>;

    /// Builds a `RectInstance` for GPU quad rendering using default/idle box styling.
    fn build_rect(&self, ox: f32, oy: f32, w: f32, h: f32) -> Option<RectInstance>;

    /// Draws the styled box quad directly into the target draw list if visible.
    fn draw(&self, target: &mut Vec<DrawCommand>, ox: f32, oy: f32, w: f32, h: f32) -> bool;
}

impl BoxStyleExt for BoxStyle {
    fn build_rect_instance(
        &self,
        ox: f32,
        oy: f32,
        w: f32,
        h: f32,
        effective_bg: Option<&Background>,
        effective_border_color: Option<Color>,
        effective_border_width: Option<f32>,
        scale: f32,
    ) -> RectInstance {
        let bg_opt = effective_bg.cloned().or_else(|| self.bg.clone());
        let (c1, c2, c3, c4, gradient_params, gradient_stops, _) =
            resolve_background(&bg_opt);

        let bw = effective_border_width.unwrap_or_else(|| self.border_width());
        let bc = effective_border_color.unwrap_or_else(|| self.border_color().unwrap_or(Color::TRANSPARENT));

        let visual_w = w * scale;
        let visual_h = h * scale;
        let visual_ox = ox - (visual_w - w) / 2.0;
        let visual_oy = oy - (visual_h - h) / 2.0;

        RectInstance {
            pos: [visual_ox, visual_oy],
            size: [visual_w, visual_h],
            color: c1,
            radius: self.gpu_radius(),
            border_width: bw,
            border_color: bc.to_array(),
            shadow_color: self.gpu_shadow_color(),
            shadow_offset: self.shadow_offset,
            shadow_blur: self.shadow_blur,
            clip_rect: [-100000.0, -100000.0, 2000000.0, 2000000.0],
            grayscale: 0.0,
            brightness: 1.0,
            opacity: self.opacity,
            border_alignment: self.border_alignment_f32(),
            color2: c2,
            color3: c3,
            color4: c4,
            gradient_params,
            gradient_stops,
        }
    }

    fn build_rect_styled(
        &self,
        ox: f32,
        oy: f32,
        w: f32,
        h: f32,
        effective_bg: Option<&Background>,
        effective_border_color: Option<Color>,
        effective_border_width: Option<f32>,
        scale: f32,
    ) -> Option<RectInstance> {
        let bg_opt = effective_bg.cloned().or_else(|| self.bg.clone());
        let (_, _, _, _, _, _, is_bg_visible) = resolve_background(&bg_opt);
        let bw = effective_border_width.unwrap_or_else(|| self.border_width());
        let bc = effective_border_color.unwrap_or_else(|| self.border_color().unwrap_or(Color::TRANSPARENT));

        let has_visible_draw = is_bg_visible || (bc.a > 0.0 && bw > 0.0) || self.shadow_color.is_some();
        if !has_visible_draw {
            return None;
        }

        Some(self.build_rect_instance(
            ox,
            oy,
            w,
            h,
            effective_bg,
            effective_border_color,
            effective_border_width,
            scale,
        ))
    }

    fn build_rect(&self, ox: f32, oy: f32, w: f32, h: f32) -> Option<RectInstance> {
        self.build_rect_styled(ox, oy, w, h, self.bg.as_ref(), self.border_color(), Some(self.border_width()), 1.0)
    }

    fn draw(&self, target: &mut Vec<DrawCommand>, ox: f32, oy: f32, w: f32, h: f32) -> bool {
        if let Some(instance) = self.build_rect(ox, oy, w, h) {
            target.push(DrawCommand::Rect(RectDraw { instance }));
            true
        } else {
            false
        }
    }
}
