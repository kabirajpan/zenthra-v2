// crates/zenthra-core/src/lib.rs

pub mod color;
pub mod element;
pub mod event;
pub mod glyph;
pub mod id;
pub mod rect;
pub mod render_mode;
pub mod response;
pub mod style;
pub mod widget;

pub use color::Color;
pub use element::{Role, SemanticNode};
pub use event::{Event, EventResponse};
pub use glyph::GlyphInstance;
pub use id::Id;
pub use rect::{Point, Rect, Size};
pub use render_mode::RenderMode;
pub use response::Response;
pub use style::{BorderRadius, EdgeInsets, Align};
pub use widget::Widget;
