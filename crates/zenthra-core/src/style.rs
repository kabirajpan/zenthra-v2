// crates/zenthra-core/src/style.rs

use crate::Color;

/// Spacing on all four sides.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EdgeInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeInsets {
    pub const ZERO: Self = Self {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    };

    pub fn all(v: f32) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }

    pub fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }

    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl From<f32> for EdgeInsets {
    fn from(v: f32) -> Self {
        Self::all(v)
    }
}

impl From<f64> for EdgeInsets {
    fn from(v: f64) -> Self {
        Self::all(v as f32)
    }
}

impl From<i32> for EdgeInsets {
    fn from(v: i32) -> Self {
        Self::all(v as f32)
    }
}

/// Per-corner border radius.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl BorderRadius {
    pub const ZERO: Self = Self {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    };

    pub fn all(r: f32) -> Self {
        Self {
            top_left: r,
            top_right: r,
            bottom_right: r,
            bottom_left: r,
        }
    }

    pub fn to_array(self) -> [f32; 4] {
        [
            self.top_left,
            self.top_right,
            self.bottom_right,
            self.bottom_left,
        ]
    }
}

/// A unified alignment enum for both horizontal and vertical axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    SpaceBetween,
    SpaceAround,
}

/// Placement direction for popovers, menus, tooltips, and floating panels relative to an anchor.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Placement {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
    Custom(f32, f32),
}

/// Alignment along the cross-axis of a placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverAlign {
    #[default]
    Start,
    Center,
    End,
}

/// Unified border representation for styling widgets and containers.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Border {
    #[default]
    None,
    Solid(Color, f32),
}

impl Border {
    pub const NONE: Self = Self::None;

    pub fn none() -> Self {
        Self::None
    }

    pub fn solid(color: Color, width: f32) -> Self {
        if width <= 0.0 || color.a == 0.0 {
            Self::None
        } else {
            Self::Solid(color, width)
        }
    }

    pub fn to_option(self) -> Option<(Color, f32)> {
        match self {
            Self::None => None,
            Self::Solid(c, w) => Some((c, w)),
        }
    }
}

impl From<(Color, f32)> for Border {
    fn from((color, width): (Color, f32)) -> Self {
        Border::solid(color, width)
    }
}

impl From<(Color, f64)> for Border {
    fn from((color, width): (Color, f64)) -> Self {
        Border::solid(color, width as f32)
    }
}

impl From<Color> for Border {
    fn from(color: Color) -> Self {
        Border::solid(color, 1.0)
    }
}

impl From<Option<Border>> for Border {
    fn from(opt: Option<Border>) -> Self {
        opt.unwrap_or(Border::None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderAlignment {
    #[default]
    Inside,
    Center,
    Outside,
}

pub mod blur {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum Type {
        #[default]
        Normal,
        Frosted,
        Glassmorphism,
        OpaqueGlass,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filter {
    Blur(f32, blur::Type),
    Opacity(f32),
    Saturate(f32),
    Brightness(f32),
    Contrast(f32),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BackdropFilter {
    pub filters: Vec<Filter>,
}

impl BackdropFilter {
    pub fn new() -> Self {
        Self { filters: Vec::new() }
    }

    pub fn blur(mut self, radius: f32, kind: blur::Type) -> Self {
        self.filters.push(Filter::Blur(radius, kind));
        self
    }

    pub fn opacity(mut self, factor: f32) -> Self {
        self.filters.push(Filter::Opacity(factor));
        self
    }

    pub fn saturate(mut self, factor: f32) -> Self {
        self.filters.push(Filter::Saturate(factor));
        self
    }

    pub fn brightness(mut self, factor: f32) -> Self {
        self.filters.push(Filter::Brightness(factor));
        self
    }

    pub fn contrast(mut self, factor: f32) -> Self {
        self.filters.push(Filter::Contrast(factor));
        self
    }
}

/// Direction / mode for rendering GPU gradients
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GradientDirection {
    #[default]
    Horizontal,
    Vertical,
    Diagonal,
    Angle(f32),
    Radial,
    HorizontalFade(f32),
}

impl GradientDirection {
    pub fn to_type_and_angle(&self) -> (f32, f32) {
        match self {
            GradientDirection::Horizontal => (1.0, 0.0),
            GradientDirection::Vertical => (1.0, std::f32::consts::FRAC_PI_2),
            GradientDirection::Diagonal => (1.0, std::f32::consts::FRAC_PI_4),
            GradientDirection::Angle(deg) => (1.0, deg.to_radians()),
            GradientDirection::Radial => (2.0, 0.0),
            GradientDirection::HorizontalFade(px) => (3.0, *px),
        }
    }
}

/// Comprehensive, unified box styling model for any widget or container.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BoxStyle {
    pub bg: Option<crate::gradient::Background>,
    pub border: Border,
    pub border_alignment: BorderAlignment,
    pub radius: [f32; 4], // [top_left, top_right, bottom_right, bottom_left]
    pub shadow_color: Option<Color>,
    pub shadow_offset: [f32; 2],
    pub shadow_blur: f32,
    pub shadow_opacity: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub fill_x: bool,
    pub fill_y: bool,
    pub padding: EdgeInsets,
    pub opacity: f32,
}

impl BoxStyle {
    pub fn new() -> Self {
        Self {
            bg: None,
            border: Border::None,
            border_alignment: BorderAlignment::Inside,
            radius: [0.0; 4],
            shadow_color: None,
            shadow_offset: [0.0, 0.0],
            shadow_blur: 0.0,
            shadow_opacity: 1.0,
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            fill_x: false,
            fill_y: false,
            padding: EdgeInsets::ZERO,
            opacity: 1.0,
        }
    }

    /// Returns corner radii correctly mapped to GPU RectInstance order: [bl, br, tr, tl].
    #[inline]
    pub fn gpu_radius(&self) -> [f32; 4] {
        [self.radius[3], self.radius[2], self.radius[1], self.radius[0]]
    }

    /// Returns shadow color array with shadow_opacity multiplier applied: [r, g, b, a * opacity].
    #[inline]
    pub fn gpu_shadow_color(&self) -> [f32; 4] {
        if let Some(c) = self.shadow_color {
            let mut a = c.to_array();
            a[3] *= self.shadow_opacity;
            a
        } else {
            [0.0, 0.0, 0.0, 0.0]
        }
    }

    /// Returns border color array and stroke width.
    #[inline]
    pub fn border_params(&self) -> ([f32; 4], f32) {
        match self.border {
            Border::None => ([0.0, 0.0, 0.0, 0.0], 0.0),
            Border::Solid(c, w) => (c.to_array(), w),
        }
    }

    /// Returns the border width, or 0.0 if none.
    #[inline]
    pub fn border_width(&self) -> f32 {
        match self.border {
            Border::None => 0.0,
            Border::Solid(_, w) => w,
        }
    }

    /// Returns the border color, or None if no border is set.
    #[inline]
    pub fn border_color(&self) -> Option<Color> {
        match self.border {
            Border::None => None,
            Border::Solid(c, _) => Some(c),
        }
    }

    /// Returns border alignment as shader float: Inside = 0.0, Center = 0.5, Outside = 1.0.
    #[inline]
    pub fn border_alignment_f32(&self) -> f32 {
        match self.border_alignment {
            BorderAlignment::Inside => 0.0,
            BorderAlignment::Center => 0.5,
            BorderAlignment::Outside => 1.0,
        }
    }

    /// Returns the inner content origin (x, y) taking padding and border stroke into account.
    #[inline]
    pub fn content_origin(&self, ox: f32, oy: f32) -> (f32, f32) {
        let bw = self.border_width();
        (ox + self.padding.left + bw, oy + self.padding.top + bw)
    }

    /// Returns the available inner content dimensions (width, height) taking padding and border stroke into account.
    #[inline]
    pub fn content_size(&self, w: f32, h: f32) -> (f32, f32) {
        let bw = self.border_width();
        (
            (w - self.padding.left - self.padding.right - 2.0 * bw).max(0.0),
            (h - self.padding.top - self.padding.bottom - 2.0 * bw).max(0.0),
        )
    }

    /// Returns the inner content rectangle (x, y, w, h) taking padding and border stroke into account.
    #[inline]
    pub fn inner_bounds(&self, ox: f32, oy: f32, w: f32, h: f32) -> (f32, f32, f32, f32) {
        let (cx, cy) = self.content_origin(ox, oy);
        let (cw, ch) = self.content_size(w, h);
        (cx, cy, cw, ch)
    }
}

/// Universal styling trait providing standardized, chainable builder methods
/// across all containers, controls, and widgets in Zenthra.
pub trait Styleable: Sized {
    fn box_style_mut(&mut self) -> &mut BoxStyle;
    fn box_style(&self) -> &BoxStyle;

    /// Replaces the entire box styling with the provided `BoxStyle`.
    fn style(mut self, s: BoxStyle) -> Self {
        *self.box_style_mut() = s;
        self
    }

    fn bg(mut self, bg: impl Into<crate::gradient::Background>) -> Self {
        match bg.into() {
            crate::gradient::Background::None => self.box_style_mut().bg = None,
            other => self.box_style_mut().bg = Some(other),
        }
        self
    }

    fn bg_gradient(mut self, start: Color, end: Color, direction: GradientDirection) -> Self {
        let (dir_type, angle) = direction.to_type_and_angle();
        if dir_type == 2.0 {
            self.box_style_mut().bg = Some(crate::gradient::Background::Gradient(crate::gradient::Gradient::radial(
                Align::Center,
                [start, end],
            )));
        } else {
            self.box_style_mut().bg = Some(crate::gradient::Background::Gradient(crate::gradient::Gradient::linear(
                angle.to_degrees(),
                [start, end],
            )));
        }
        self
    }

    fn bg_opacity(mut self, opacity: f32) -> Self {
        if let Some(ref mut bg) = self.box_style_mut().bg {
            match bg {
                crate::gradient::Background::Solid(ref mut c) => c.a = opacity,
                crate::gradient::Background::Gradient(ref mut g) => match g {
                    crate::gradient::Gradient::Linear { ref mut stops, .. } => {
                        for s in stops.iter_mut() { s.color.a *= opacity; }
                    }
                    crate::gradient::Gradient::Radial { ref mut stops, .. } => {
                        for s in stops.iter_mut() { s.color.a *= opacity; }
                    }
                    crate::gradient::Gradient::Mesh { ref mut points } => {
                        for p in points.iter_mut() { p.1.a *= opacity; }
                    }
                },
                crate::gradient::Background::None => {}
            }
        }
        self
    }

    fn border(mut self, border: impl Into<Border>) -> Self {
        self.box_style_mut().border = border.into();
        self
    }

    fn border_alignment(mut self, alignment: BorderAlignment) -> Self {
        self.box_style_mut().border_alignment = alignment;
        self
    }

    fn radius_all(mut self, r: f32) -> Self {
        self.box_style_mut().radius = [r; 4];
        self
    }

    fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        self.box_style_mut().radius = [tl, tr, br, bl];
        self
    }

    fn radius_top(mut self, r: f32) -> Self {
        self.box_style_mut().radius[0] = r;
        self.box_style_mut().radius[1] = r;
        self
    }

    fn radius_bottom(mut self, r: f32) -> Self {
        self.box_style_mut().radius[2] = r;
        self.box_style_mut().radius[3] = r;
        self
    }

    fn radius_left(mut self, r: f32) -> Self {
        self.box_style_mut().radius[0] = r;
        self.box_style_mut().radius[3] = r;
        self
    }

    fn radius_right(mut self, r: f32) -> Self {
        self.box_style_mut().radius[1] = r;
        self.box_style_mut().radius[2] = r;
        self
    }

    fn radius_top_left(mut self, r: f32) -> Self {
        self.box_style_mut().radius[0] = r;
        self
    }

    fn radius_top_right(mut self, r: f32) -> Self {
        self.box_style_mut().radius[1] = r;
        self
    }

    fn radius_bottom_right(mut self, r: f32) -> Self {
        self.box_style_mut().radius[2] = r;
        self
    }

    fn radius_bottom_left(mut self, r: f32) -> Self {
        self.box_style_mut().radius[3] = r;
        self
    }

    fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.box_style_mut().shadow_color = Some(color);
        self.box_style_mut().shadow_offset = [x, y];
        self.box_style_mut().shadow_blur = blur;
        self
    }

    fn shadow_opacity(mut self, opacity: f32) -> Self {
        self.box_style_mut().shadow_opacity = opacity;
        self
    }

    fn opacity(mut self, opacity: f32) -> Self {
        self.box_style_mut().opacity = opacity;
        self
    }

    fn width(mut self, w: f32) -> Self {
        self.box_style_mut().width = Some(w);
        self.box_style_mut().fill_x = false;
        self
    }

    fn height(mut self, h: f32) -> Self {
        self.box_style_mut().height = Some(h);
        self.box_style_mut().fill_y = false;
        self
    }

    fn fill_x(mut self) -> Self {
        self.box_style_mut().fill_x = true;
        self
    }

    fn fill_y(mut self) -> Self {
        self.box_style_mut().fill_y = true;
        self
    }

    fn fill(mut self) -> Self {
        self.box_style_mut().fill_x = true;
        self.box_style_mut().fill_y = true;
        self
    }

    fn full_width(self) -> Self {
        self.fill_x()
    }

    fn full_height(self) -> Self {
        self.fill_y()
    }

    fn max_width(mut self, w: f32) -> Self {
        self.box_style_mut().max_width = Some(w);
        self
    }

    fn max_height(mut self, h: f32) -> Self {
        self.box_style_mut().max_height = Some(h);
        self
    }

    fn min_width(mut self, w: f32) -> Self {
        self.box_style_mut().min_width = Some(w);
        self
    }

    fn min_height(mut self, h: f32) -> Self {
        self.box_style_mut().min_height = Some(h);
        self
    }

    fn padding(mut self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        self.box_style_mut().padding = EdgeInsets { top, right, bottom, left };
        self
    }

    fn padding_all(mut self, p: f32) -> Self {
        self.box_style_mut().padding = EdgeInsets::all(p);
        self
    }

    fn padding_x(mut self, p: f32) -> Self {
        self.box_style_mut().padding.left = p;
        self.box_style_mut().padding.right = p;
        self
    }

    fn padding_y(mut self, p: f32) -> Self {
        self.box_style_mut().padding.top = p;
        self.box_style_mut().padding.bottom = p;
        self
    }

    fn padding_top(mut self, p: f32) -> Self {
        self.box_style_mut().padding.top = p;
        self
    }

    fn padding_bottom(mut self, p: f32) -> Self {
        self.box_style_mut().padding.bottom = p;
        self
    }

    fn padding_left(mut self, p: f32) -> Self {
        self.box_style_mut().padding.left = p;
        self
    }

    fn padding_right(mut self, p: f32) -> Self {
        self.box_style_mut().padding.right = p;
        self
    }
}


