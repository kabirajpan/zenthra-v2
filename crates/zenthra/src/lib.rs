pub mod app;

// Re-export everything the user needs
pub use app::App;
pub use zenthra_core::{Color, EdgeInsets, Event, Id, Point, Rect, RenderMode, Response, Size};
pub use zenthra_widgets::container::{Direction, HAlign, VAlign, Wrap};
pub use zenthra_widgets::{ButtonBuilder, ContainerBuilder, TextBuilder, Ui};

pub mod prelude {
    pub use crate::App;
    pub use zenthra_core::{Color, EdgeInsets, RenderMode, Response};
    pub use zenthra_widgets::container::{Direction, HAlign, VAlign, Wrap};
    pub use zenthra_widgets::Ui;
}
