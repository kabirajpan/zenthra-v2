// crates/zenthra-core/src/rect.rs

use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const ZERO: Self = Self {
        width: 0.0,
        height: 0.0,
    };
    pub const INFINITY: Self = Self {
        width: f32::INFINITY,
        height: f32::INFINITY,
    };

    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn clamp(self, min: Size, max: Size) -> Self {
        Self {
            width: self.width.clamp(min.width, max.width),
            height: self.height.clamp(min.height, max.height),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub const ZERO: Self = Self {
        origin: Point::ZERO,
        size: Size::ZERO,
    };

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }

    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self::new(min.x, min.y, max.x - min.x, max.y - min.y)
    }

    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.origin.x
            && p.y >= self.origin.y
            && p.x <= self.origin.x + self.size.width
            && p.y <= self.origin.y + self.size.height
    }

    pub fn min(&self) -> Vec2 {
        Vec2::new(self.origin.x, self.origin.y)
    }
    pub fn max(&self) -> Vec2 {
        Vec2::new(
            self.origin.x + self.size.width,
            self.origin.y + self.size.height,
        )
    }
}
