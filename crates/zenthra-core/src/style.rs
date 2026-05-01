// crates/zenthra-core/src/style.rs

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
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderAlignment {
    #[default]
    Inside,
    Center,
    Outside,
}
