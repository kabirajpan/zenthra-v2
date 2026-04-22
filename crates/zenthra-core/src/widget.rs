// crates/zenthra-core/src/widget.rs

use crate::event::{Event, EventResponse};
use crate::rect::{Rect, Size};

/// Layout constraints passed down the widget tree.
#[derive(Debug, Clone, Copy)]
pub struct Constraints {
    pub min: Size,
    pub max: Size,
}

impl Constraints {
    pub fn loose(max: Size) -> Self {
        Self {
            min: Size::ZERO,
            max,
        }
    }
    pub fn tight(size: Size) -> Self {
        Self {
            min: size,
            max: size,
        }
    }

    pub fn constrain(&self, size: Size) -> Size {
        size.clamp(self.min, self.max)
    }
}

/// Every UI element implements this.
pub trait Widget: std::fmt::Debug {
    /// Measure and return the widget's size given constraints.
    fn layout(&mut self, constraints: Constraints) -> Size;

    /// Called after layout — widget knows its final rect.
    fn paint(&self, rect: Rect);

    /// Handle an event. Default: ignore.
    fn on_event(&mut self, event: &Event, rect: Rect) -> EventResponse {
        let _ = (event, rect);
        EventResponse::Ignored
    }

    /// Human-readable name for debugging.
    fn name(&self) -> &str {
        "Widget"
    }
}
