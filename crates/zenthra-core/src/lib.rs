// crates/zenthra-core/src/lib.rs

pub mod color;
pub mod event;
pub mod id;
pub mod rect;
pub mod style;
pub mod widget;

pub use color::Color;
pub use event::{Event, EventResponse};
pub use id::Id;
pub use rect::{Point, Rect, Size};
pub use style::{BorderRadius, EdgeInsets};
pub use widget::Widget;
