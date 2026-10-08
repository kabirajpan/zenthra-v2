// crates/zenthra-core/src/gradient.rs

use crate::color::Color;
use crate::style::Align;

/// Direction for linear gradients
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Direction {
    #[default]
    ToRight,
    ToLeft,
    ToBottom,
    ToTop,
    ToBottomRight,
    ToBottomLeft,
    ToTopRight,
    ToTopLeft,
    Angle(f32),
}

impl Direction {
    pub fn to_radians(&self) -> f32 {
        match self {
            Direction::ToRight => 0.0,
            Direction::ToBottom => std::f32::consts::FRAC_PI_2,
            Direction::ToLeft => std::f32::consts::PI,
            Direction::ToTop => 3.0 * std::f32::consts::FRAC_PI_2,
            Direction::ToBottomRight => std::f32::consts::FRAC_PI_4,
            Direction::ToBottomLeft => 3.0 * std::f32::consts::FRAC_PI_4,
            Direction::ToTopRight => 7.0 * std::f32::consts::FRAC_PI_4,
            Direction::ToTopLeft => 5.0 * std::f32::consts::FRAC_PI_4,
            Direction::Angle(deg) => deg.to_radians(),
        }
    }
}

impl From<f32> for Direction {
    fn from(deg: f32) -> Self {
        Direction::Angle(deg)
    }
}

impl From<crate::style::GradientDirection> for Direction {
    fn from(gd: crate::style::GradientDirection) -> Self {
        match gd {
            crate::style::GradientDirection::Horizontal => Direction::ToRight,
            crate::style::GradientDirection::Vertical => Direction::ToBottom,
            crate::style::GradientDirection::Diagonal => Direction::ToBottomRight,
            crate::style::GradientDirection::Angle(deg) => Direction::Angle(deg),
            _ => Direction::ToRight,
        }
    }
}

/// A color stop along a gradient ramp
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorStop {
    pub position: f32, // 0.0 to 1.0
    pub color: Color,
}

pub trait IntoStops {
    fn into_stops(self) -> Vec<ColorStop>;
}

impl<const N: usize> IntoStops for [Color; N] {
    fn into_stops(self) -> Vec<ColorStop> {
        if N <= 1 {
            return self.iter().map(|&c| ColorStop { position: 0.0, color: c }).collect();
        }
        self.iter().enumerate().map(|(i, &c)| {
            ColorStop {
                position: i as f32 / (N - 1) as f32,
                color: c,
            }
        }).collect()
    }
}

impl<const N: usize> IntoStops for [(f32, Color); N] {
    fn into_stops(self) -> Vec<ColorStop> {
        self.iter().map(|&(pos, c)| ColorStop { position: pos, color: c }).collect()
    }
}

impl<const N: usize> IntoStops for [(f32, f32, f32); N] {
    fn into_stops(self) -> Vec<ColorStop> {
        if N <= 1 {
            return self.iter().map(|&(r, g, b)| ColorStop { position: 0.0, color: Color::rgb(r, g, b) }).collect();
        }
        self.iter().enumerate().map(|(i, &(r, g, b))| {
            ColorStop {
                position: i as f32 / (N - 1) as f32,
                color: Color::rgb(r, g, b),
            }
        }).collect()
    }
}

impl<const N: usize> IntoStops for [[f32; 3]; N] {
    fn into_stops(self) -> Vec<ColorStop> {
        if N <= 1 {
            return self.iter().map(|&[r, g, b]| ColorStop { position: 0.0, color: Color::rgb(r, g, b) }).collect();
        }
        self.iter().enumerate().map(|(i, &[r, g, b])| {
            ColorStop {
                position: i as f32 / (N - 1) as f32,
                color: Color::rgb(r, g, b),
            }
        }).collect()
    }
}

impl<const N: usize> IntoStops for [(f32, (f32, f32, f32)); N] {
    fn into_stops(self) -> Vec<ColorStop> {
        self.iter().map(|&(pos, (r, g, b))| ColorStop { position: pos, color: Color::rgb(r, g, b) }).collect()
    }
}

impl<const N: usize> IntoStops for [(f32, f32, f32, f32); N] {
    fn into_stops(self) -> Vec<ColorStop> {
        if N <= 1 {
            return self.iter().map(|&(r, g, b, a)| ColorStop { position: 0.0, color: Color::rgba(r, g, b, a) }).collect();
        }
        self.iter().enumerate().map(|(i, &(r, g, b, a))| {
            ColorStop {
                position: i as f32 / (N - 1) as f32,
                color: Color::rgba(r, g, b, a),
            }
        }).collect()
    }
}

impl<const N: usize> IntoStops for [[f32; 4]; N] {
    fn into_stops(self) -> Vec<ColorStop> {
        if N <= 1 {
            return self.iter().map(|&[r, g, b, a]| ColorStop { position: 0.0, color: Color::rgba(r, g, b, a) }).collect();
        }
        self.iter().enumerate().map(|(i, &[r, g, b, a])| {
            ColorStop {
                position: i as f32 / (N - 1) as f32,
                color: Color::rgba(r, g, b, a),
            }
        }).collect()
    }
}

impl<const N: usize> IntoStops for [(f32, (f32, f32, f32, f32)); N] {
    fn into_stops(self) -> Vec<ColorStop> {
        self.iter().map(|&(pos, (r, g, b, a))| ColorStop { position: pos, color: Color::rgba(r, g, b, a) }).collect()
    }
}

impl<const N: usize> IntoStops for [(f32, [f32; 4]); N] {
    fn into_stops(self) -> Vec<ColorStop> {
        self.iter().map(|&(pos, [r, g, b, a])| ColorStop { position: pos, color: Color::rgba(r, g, b, a) }).collect()
    }
}

impl IntoStops for Vec<Color> {
    fn into_stops(self) -> Vec<ColorStop> {
        let n = self.len();
        if n <= 1 {
            return self.into_iter().map(|c| ColorStop { position: 0.0, color: c }).collect();
        }
        self.into_iter().enumerate().map(|(i, c)| {
            ColorStop {
                position: i as f32 / (n - 1) as f32,
                color: c,
            }
        }).collect()
    }
}

impl IntoStops for Vec<(f32, Color)> {
    fn into_stops(self) -> Vec<ColorStop> {
        self.into_iter().map(|(pos, c)| ColorStop { position: pos, color: c }).collect()
    }
}

pub trait IntoAnchor {
    fn into_anchor(self) -> (f32, f32);
}

impl IntoAnchor for (f32, f32) {
    fn into_anchor(self) -> (f32, f32) { self }
}

impl IntoAnchor for [f32; 2] {
    fn into_anchor(self) -> (f32, f32) { (self[0], self[1]) }
}

impl IntoAnchor for Align {
    fn into_anchor(self) -> (f32, f32) {
        match self {
            Align::Center => (0.5, 0.5),
            Align::Left => (0.0, 0.5),
            Align::Right => (1.0, 0.5),
            Align::Top => (0.5, 0.0),
            Align::Bottom => (0.5, 1.0),
            Align::TopLeft => (0.0, 0.0),
            Align::TopRight => (1.0, 0.0),
            Align::BottomLeft => (0.0, 1.0),
            Align::BottomRight => (1.0, 1.0),
            _ => (0.5, 0.5),
        }
    }
}

pub trait IntoMeshPoints {
    fn into_mesh_points(self) -> Vec<((f32, f32), Color)>;
}

impl<const N: usize> IntoMeshPoints for [((f32, f32), Color); N] {
    fn into_mesh_points(self) -> Vec<((f32, f32), Color)> {
        self.to_vec()
    }
}

impl<const N: usize> IntoMeshPoints for [(f32, f32, Color); N] {
    fn into_mesh_points(self) -> Vec<((f32, f32), Color)> {
        self.iter().map(|&(x, y, c)| ((x, y), c)).collect()
    }
}

impl<const N: usize> IntoMeshPoints for [(Align, Color); N] {
    fn into_mesh_points(self) -> Vec<((f32, f32), Color)> {
        self.iter().map(|&(a, c)| (a.into_anchor(), c)).collect()
    }
}

impl IntoMeshPoints for Vec<((f32, f32), Color)> {
    fn into_mesh_points(self) -> Vec<((f32, f32), Color)> {
        self
    }
}

/// A gradient definition supporting Linear, Radial, and Freeform Mesh types
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear {
        direction: Direction,
        stops: Vec<ColorStop>,
    },
    Radial {
        anchor: (f32, f32),
        radius: f32,
        stops: Vec<ColorStop>,
    },
    Mesh {
        points: Vec<((f32, f32), Color)>,
    },
}

impl Gradient {
    /// Creates a linear gradient
    pub fn linear<D: Into<Direction>, S: IntoStops>(direction: D, stops: S) -> Self {
        Gradient::Linear {
            direction: direction.into(),
            stops: stops.into_stops(),
        }
    }

    /// Creates a radial gradient
    pub fn radial<A: IntoAnchor, S: IntoStops>(anchor: A, stops: S) -> Self {
        Gradient::Radial {
            anchor: anchor.into_anchor(),
            radius: 1.0,
            stops: stops.into_stops(),
        }
    }

    /// Creates a freeform multi-point mesh gradient
    pub fn mesh<P: IntoMeshPoints>(points: P) -> Self {
        Gradient::Mesh {
            points: points.into_mesh_points(),
        }
    }

    /// Shorthand for 4-corner mesh gradient (TopLeft, TopRight, BottomLeft, BottomRight)
    pub fn corners(tl: Color, tr: Color, bl: Color, br: Color) -> Self {
        Gradient::Mesh {
            points: vec![
                ((0.0, 0.0), tl),
                ((1.0, 0.0), tr),
                ((0.0, 1.0), bl),
                ((1.0, 1.0), br),
            ],
        }
    }
}

/// Unified Background model for widgets: Solid Color or Gradient
#[derive(Debug, Clone, PartialEq)]
pub enum Background {
    Solid(Color),
    Gradient(Gradient),
}

impl From<Color> for Background {
    fn from(c: Color) -> Self {
        Background::Solid(c)
    }
}

impl From<Gradient> for Background {
    fn from(g: Gradient) -> Self {
        Background::Gradient(g)
    }
}
